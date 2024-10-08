//! Symmetrical messenger for both client and server.

use crate::scenegraph::{self, ScenegraphError};
use global_counter::primitive::exact::CounterU64;
use nix::cmsg_space;
use nix::fcntl::{fcntl, FcntlArg};
use nix::sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags};
use rustc_hash::FxHashMap;
use stardust_xr_schemas::flat::flatbuffers::{self, InvalidFlatbuffer};
use stardust_xr_schemas::flat::message::{root_as_message, Message as FlatMessage, MessageArgs};
use stardust_xr_schemas::flex::flexbuffers;
use std::future::Future;
use std::io::{IoSlice, IoSliceMut};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

fn trace_call(
	incoming: bool,
	call_type: u8,
	message_id: u64,
	node: u64,
	method: u64,
	err: Option<&str>,
	data: &[u8],
) {
	let level = match call_type {
		0 => tracing::Level::WARN,
		_ => tracing::Level::TRACE,
	};

	if tracing::level_enabled!(level) {
		let call_type = match call_type {
			0 => "error",
			1 => "signal",
			2 => "method call",
			3 => "method return",
			_ => "unknown",
		};
		let data = match flexbuffers::Reader::get_root(data) {
			Ok(root) => root.to_string(),
			Err(_) => String::from_utf8_lossy(data).into_owned(),
		};

		match level {
			tracing::Level::WARN => {
				tracing::warn!(
					source = match incoming {
						true => "remote",
						false => "local",
					},
					message_id,
					node,
					method,
					err,
					data,
					"Stardust error",
				)
			}
			_ => {
				tracing::trace!(
					direction = match incoming {
						true => "incoming",
						false => "outgoing",
					},
					call_type,
					message_id,
					node,
					method,
					err,
					data,
					"Stardust message",
				)
			}
		}
	}
}

/// Error for all messenger-related failures.
#[derive(Error, Debug)]
pub enum MessengerError {
	/// The MessageReceiver has been dropped with pending futures
	#[error("Receiver has been dropped")]
	ReceiverDropped,
	#[error("IO Error: {0}")]
	IOError(std::io::Error),
	/// The incoming message is corrupted
	#[error("Invalid flatbuffer {0}")]
	InvalidFlatbuffer(InvalidFlatbuffer),
	/// The message type u8 is greater than method return (3)
	#[error("Message type is out of bounds")]
	MessageTypeOutOfBounds,
}
impl From<std::io::Error> for MessengerError {
	fn from(e: std::io::Error) -> Self {
		MessengerError::IOError(e)
	}
}
impl From<InvalidFlatbuffer> for MessengerError {
	fn from(e: InvalidFlatbuffer) -> Self {
		MessengerError::InvalidFlatbuffer(e)
	}
}

/// Wrapper for messages after being serialized, for type safety.
pub struct Message {
	data: Vec<u8>,
	fds: Vec<OwnedFd>,
}
impl Message {
	pub fn into_message(self) -> Vec<u8> {
		self.data
	}
	pub fn into_components(self) -> (Vec<u8>, Vec<OwnedFd>) {
		(self.data, self.fds)
	}
}

/// Header for sending messages over the socket.
#[derive(Clone, Copy)]
pub struct Header {
	pub body_length: u32,
}
impl Header {
	pub const SIZE: usize = 4;
	pub fn into_bytes(self) -> [u8; Self::SIZE] {
		self.body_length.to_ne_bytes()
	}
	pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
		let body_length = u32::from_ne_bytes(bytes);
		Header { body_length }
	}
}

type PendingFuture = oneshot::Sender<Result<Message, String>>;
type PendingFutureSender = mpsc::UnboundedSender<(u64, PendingFuture)>;
type PendingFutureReceiver = mpsc::UnboundedReceiver<(u64, PendingFuture)>;

/// Receiving half of the messenger.
pub struct MessageReceiver {
	read: OwnedReadHalf,
	pending_futures: FxHashMap<u64, PendingFuture>,
	pending_future_rx: PendingFutureReceiver,
	send_handle: MessageSenderHandle,
}
impl MessageReceiver {
	fn new(
		read: OwnedReadHalf,
		pending_future_rx: PendingFutureReceiver,
		send_handle: MessageSenderHandle,
	) -> Self {
		MessageReceiver {
			read,
			pending_futures: Default::default(),
			pending_future_rx,
			send_handle,
		}
	}
	/// Take all the pending futures in the queue from method calls and store them for when the other side sends a method return.
	pub fn update_pending_futures(&mut self) {
		while let Ok((id, future)) = self.pending_future_rx.try_recv() {
			let _ = self.pending_futures.insert(id, future);
		}
	}
	/// Await a message from the socket, parse it, and handle it.
	pub async fn dispatch<S: scenegraph::Scenegraph>(
		&mut self,
		scenegraph: &S,
	) -> Result<(), MessengerError> {
		let mut header_buffer = [0_u8; Header::SIZE];
		self.read.read_exact(&mut header_buffer).await?;
		let header = Header::from_bytes(header_buffer);

		let mut body: Vec<u8> = std::vec::from_elem(0_u8, header.body_length as usize);
		
		let iov = &mut [IoSliceMut::new(body.as_mut_slice())];

		// 253 is the Linux value for SCM_MAX_FD (max FDs in a cmsg)
		let mut cmsgs = cmsg_space!([RawFd; 253]);

		let stream = self.read.as_ref();
		let fds: Vec<OwnedFd> = stream
			.async_io(Interest::READABLE, || {
				match recvmsg::<()>(stream.as_raw_fd(), iov, Some(&mut cmsgs), MsgFlags::empty()) {
					Ok(recv_msg) => {
						let fds = recv_msg
							.cmsgs()
							.flat_map(|cmsg| {
								if let ControlMessageOwned::ScmRights(fds) = cmsg {
									fds
								} else {
									Vec::new()
								}
							})
							.filter_map(|fd| match fcntl(fd, FcntlArg::F_GETFD) {
								Err(nix::errno::Errno::EBADF) => None,
								_ => unsafe { Some(OwnedFd::from_raw_fd(fd)) }, // Consider non-EBADF errors as valid
							})
							.collect();
						Ok(fds)
					}
					Err(nix::Error::EWOULDBLOCK) => Err(std::io::ErrorKind::WouldBlock.into()),
					Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
				}
			})
			.await?;

		self.update_pending_futures();
		self.handle_message(body, scenegraph, fds)
	}

	#[instrument(level = "trace", skip_all)]
	fn handle_message<S: scenegraph::Scenegraph>(
		&mut self,
		raw_message: Vec<u8>,
		scenegraph: &S,
		fds: Vec<OwnedFd>,
	) -> Result<(), MessengerError> {
		let message = root_as_message(&raw_message)?;
		let message_type = message.type_();

		trace_call(
			true,
			message_type,
			message.id(),
			message.node(),
			message.method(),
			message.error(),
			message.data().map(|d| d.bytes()).unwrap_or(&[]),
		);
		let id = message.id();
		let node = message.node();
		let method = message.method();
		let data = message.data().unwrap_or_default().bytes();
		match message_type {
			// Errors
			0 => {
				let future_opt = self.pending_futures.remove(&message.id());
				if let Some(future) = future_opt {
					let _ = future.send(Err(message.error().unwrap_or("unknown").to_string()));
				}
			}
			// Signals
			1 => {
				let signal_status = scenegraph.send_signal(node, method, data, fds);
				if let Err(e) = signal_status {
					self.send_handle
						.error(message.id(), node, method, e, data)?;
				}
			}
			// Method called
			2 => {
				let (response_tx, response_rx) =
					oneshot::channel::<Result<(Vec<u8>, Vec<OwnedFd>), ScenegraphError>>();
				let send_handle = self.send_handle.clone();
				scenegraph.execute_method(node, method, data, fds, response_tx);
				tokio::task::spawn(async move {
					let Ok(message) = root_as_message(&raw_message) else {
						return;
					};
					let data = message.data().unwrap_or_default().bytes();
					if let Ok(result) = response_rx.await {
						let _ = match result {
							Ok((data, fds)) => send_handle.send(serialize_call(
								3,
								message.id(),
								node,
								method,
								None,
								&data,
								fds,
							)),
							Err(error) => {
								send_handle.error(message.id(), node, method, error, data)
							}
						};
					} else {
						let _ = send_handle.error(
							message.id(),
							node,
							method,
							"Internal: method did not return a response",
							data,
						);
					}
				});
			}
			// Method return
			3 => {
				let future_opt = self.pending_futures.remove(&id);
				match future_opt {
					None => {
						self.send_handle.error(
							message.id(),
							node,
							method,
							"Method return without method call".to_string(),
							data,
						)?;
					}
					Some(future) => {
						let _ = future.send(Ok(Message {
							data: data.to_vec(),
							fds,
						}));
					}
				}
			}
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}
}

/// Generate an error message from arguments.
pub fn serialize_error<T: std::fmt::Display>(
	message_id: u64,
	node: u64,
	method: u64,
	err: T,
	data: &[u8],
) -> Message {
	let error = format!("{}", err);
	serialize_call(
		0,
		message_id,
		node,
		method,
		Some(error.as_str()),
		data,
		Vec::new(),
	)
}
/// Generate a signal message from arguments.
pub fn serialize_signal_call(
	id: u64,
	node: u64,
	method: u64,
	data: &[u8],
	fds: Vec<OwnedFd>,
) -> Message {
	serialize_call(1, id, node, method, None, data, fds)
}
/// Generate a method message from arguments.
pub fn serialize_method_call(
	id: u64,
	node: u64,
	method: u64,
	data: &[u8],
	fds: Vec<OwnedFd>,
) -> Message {
	serialize_call(2, id, node, method, None, data, fds)
}
#[instrument(level = "trace", skip_all)]
fn serialize_call(
	call_type: u8,
	id: u64,
	node: u64,
	method: u64,
	err: Option<&str>,
	data: &[u8],
	fds: Vec<OwnedFd>,
) -> Message {
	trace_call(false, call_type, id, node, method, err, data);

	let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(1024);
	let flex_err = err.map(|s| fbb.create_string(s));
	let flex_data = fbb.create_vector(data);

	let message_constructed = FlatMessage::create(
		&mut fbb,
		&MessageArgs {
			type_: call_type,
			id,
			node,
			method,
			error: flex_err,
			data: Some(flex_data),
		},
	);
	fbb.finish(message_constructed, None);
	Message {
		data: fbb.finished_data().to_vec(),
		fds,
	}
}

/// Sender half of the messenger
pub struct MessageSender {
	write: OwnedWriteHalf,
	handle: MessageSenderHandle,
	message_rx: mpsc::UnboundedReceiver<Message>,
	pending_future_tx: PendingFutureSender,
	message_counter: Arc<CounterU64>,
}
impl MessageSender {
	fn new(write: OwnedWriteHalf, pending_future_tx: PendingFutureSender) -> Self {
		let (message_tx, message_rx) = mpsc::unbounded_channel();
		let max_message_id = Arc::new(CounterU64::new(0));
		MessageSender {
			write,
			handle: MessageSenderHandle {
				message_tx,
				pending_future_tx: pending_future_tx.clone(),
				message_counter: max_message_id.clone(),
			},
			message_rx,
			pending_future_tx,
			message_counter: max_message_id,
		}
	}
	/// Send all the queued messages from the handles
	pub async fn flush(&mut self) -> Result<(), MessengerError> {
		while let Some(message) = self.message_rx.recv().await {
			self.send(message).await?;
		}
		Ok(())
	}
	/// Send a message and await until sent.
	pub async fn send(&mut self, message: Message) -> Result<(), MessengerError> {
		let body = &message.data;
		let header = Header {
			body_length: body.len() as u32,
		};

		self.write.write_all(&header.into_bytes()).await?;

		if message.fds.is_empty() {
			self.write.write_all(body).await?;
		} else {
			let iov = &[IoSlice::new(body)];
			let fds = message
				.fds
				.into_iter()
				.map(IntoRawFd::into_raw_fd)
				.collect::<Vec<_>>();
			let cmsgs = &[ControlMessage::ScmRights(&fds)];

			let stream = self.write.as_ref();
			stream
				.async_io(Interest::WRITABLE, || {
					match sendmsg::<()>(stream.as_raw_fd(), iov, cmsgs, MsgFlags::empty(), None) {
						Ok(_) => Ok(()),
						Err(nix::Error::EWOULDBLOCK) => Err(std::io::ErrorKind::WouldBlock.into()),
						Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
					}
				})
				.await?;
		}
		Ok(())
	}
	/// Get a handle to send messages from anywhere.
	pub fn handle(&self) -> MessageSenderHandle {
		self.handle.clone()
	}

	/// Send a signal immediately, await until sent.
	pub async fn signal(
		&mut self,
		node: u64,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), MessengerError> {
		let id = self.message_counter.inc();
		self.send(serialize_signal_call(id, node, signal, data, fds))
			.await
	}
	/// Call a method immediately, await until other side sends back a response or the message fails to send.
	pub async fn method(
		&mut self,
		node: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<Result<Message, String>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.message_counter.inc();
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(serialize_method_call(id, node, method, data, fds))
			.await?;
		rx.await.map_err(|_| MessengerError::ReceiverDropped)
	}
}

/// Handle to the message sender, so you can synchronously send messages from anywhere without blocking.
#[derive(Clone)]
pub struct MessageSenderHandle {
	message_tx: mpsc::UnboundedSender<Message>,
	pending_future_tx: PendingFutureSender,
	message_counter: Arc<CounterU64>,
}
impl MessageSenderHandle {
	/// Queue up an error to be sent.
	pub fn error<E: std::fmt::Display>(
		&self,
		id: u64,
		node: u64,
		method: u64,
		err: E,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_error(id, node, method, err, data))
	}
	/// Queue up a signal to be sent.
	pub fn signal(
		&self,
		node: u64,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), MessengerError> {
		let id = self.message_counter.inc();
		self.send(serialize_signal_call(id, node, signal, data, fds))
	}
	/// Queue up a method to be sent and get back a future for when a response is returned.
	pub fn method(
		&self,
		node: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<impl Future<Output = Result<Message, String>>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.message_counter.inc();
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(serialize_method_call(id, node, method, data, fds))?;
		Ok(async move { rx.await.map_err(|e| e.to_string())? })
	}

	#[instrument(level = "trace", skip_all)]
	fn send(&self, message: Message) -> Result<(), MessengerError> {
		self.message_tx
			.send(message)
			.map_err(|_| MessengerError::ReceiverDropped)
	}
}

/// Create 2 messenger halves from a connection to a stardust client or server.
pub fn create(connection: UnixStream) -> (MessageSender, MessageReceiver) {
	let (read, write) = connection.into_split();
	let (pending_future_tx, pending_future_rx) = mpsc::unbounded_channel();
	let sender = MessageSender::new(write, pending_future_tx);
	let receiver = MessageReceiver::new(read, pending_future_rx, sender.handle());
	(sender, receiver)
}
