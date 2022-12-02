use crate::scenegraph;
// use anyhow::anyhow;
use rustc_hash::FxHashMap;
use stardust_xr_schemas::flat::flatbuffers;
use stardust_xr_schemas::flat::message::{root_as_message, Message as FlatMessage, MessageArgs};
use stardust_xr_schemas::flex::flexbuffers;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;
use tokio::sync::{mpsc, oneshot};
use tracing::debug;

#[derive(Error, Debug)]
pub enum MessengerError {
	#[error("Receiver has been dropped")]
	ReceiverDropped,
	#[error("Message type is out of bounds")]
	MessageTypeOutOfBounds,
	#[error("IO Error: {e}")]
	IOError { e: std::io::Error },
	#[error("Invalid flatbuffer")]
	InvalidFlatbuffer,
}

pub struct Message {
	data: Vec<u8>,
}
impl Message {
	pub fn into_data(self) -> Vec<u8> {
		self.data
	}
}

type PendingFuture = oneshot::Sender<Result<Vec<u8>, String>>;
type PendingFutureSender = mpsc::UnboundedSender<(u64, PendingFuture)>;
type PendingFutureReceiver = mpsc::UnboundedReceiver<(u64, PendingFuture)>;
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
	pub fn update_pending_futures(&mut self) {
		while let Ok((id, future)) = self.pending_future_rx.try_recv() {
			let _ = self.pending_futures.insert(id, future);
		}
	}
	pub async fn dispatch<S: scenegraph::Scenegraph>(
		&mut self,
		scenegraph: &S,
	) -> Result<(), MessengerError> {
		let mut message_length_buffer: [u8; 4] = [0; 4];
		self.read
			.read_exact(&mut message_length_buffer)
			.await
			.map_err(|e| MessengerError::IOError { e })?;
		let message_length: u32 = u32::from_ne_bytes(message_length_buffer);

		let mut message_buffer: Vec<u8> = std::vec::from_elem(0_u8, message_length as usize);
		self.read
			.read_exact(message_buffer.as_mut_slice())
			.await
			.map_err(|e| MessengerError::IOError { e })?;

		self.update_pending_futures();
		self.handle_message(message_buffer, scenegraph)
	}

	fn handle_message<S: scenegraph::Scenegraph>(
		&mut self,
		message: Vec<u8>,
		scenegraph: &S,
	) -> Result<(), MessengerError> {
		let message = root_as_message(&message).map_err(|_| MessengerError::InvalidFlatbuffer)?;
		let message_type = message.type_();
		match message_type {
			// Errors
			0 => {
				let future_opt = self.pending_futures.remove(&message.id());
				match future_opt {
					None => {
						eprintln!(
							"[Stardust XR][{}:{}] {}",
							message.object().unwrap_or("unknown"),
							message.method().unwrap_or("unknown"),
							message.error().unwrap_or("unknown"),
						);
					}
					Some(future) => {
						let _ = future.send(Err(message.error().unwrap_or("unknown").to_string()));
					}
				}
			}
			// Signals
			1 => {
				let signal_status = scenegraph.send_signal(
					message.object().unwrap(),
					message.method().unwrap(),
					message.data().unwrap(),
				);
				if let Err(e) = signal_status {
					self.send_handle.error(
						message.object().unwrap(),
						message.method().unwrap(),
						e,
					)?;
				}
			}
			// Method called
			2 => {
				let method_result = scenegraph.execute_method(
					message.object().unwrap(),
					message.method().unwrap(),
					message.data().unwrap(),
				);
				match method_result {
					Ok(return_value) => self.send_handle.send(serialize_call(
						3,
						Some(message.id()),
						message.object().unwrap(),
						message.method().unwrap(),
						None,
						Some(&return_value),
					))?,
					Err(error) => self.send_handle.error(
						message.object().unwrap(),
						message.method().unwrap(),
						error,
					)?,
				};
			}
			// Method return
			3 => {
				let future_opt = self.pending_futures.remove(&message.id());
				match future_opt {
					None => {
						self.send_handle.error(
							message.object().unwrap(),
							message.method().unwrap(),
							"Method return without method call".to_string(),
						)?;
					}
					Some(future) => {
						let _ = future.send(Ok(message.data().unwrap().to_vec()));
					}
				}
			}
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}
}

pub fn serialize_error<T: std::fmt::Display>(object: &str, method: &str, err: T) -> Message {
	let error = format!("{}", err);
	serialize_call(0, None, object, method, Some(error.as_str()), None)
}
pub fn serialize_signal_call(object: &str, method: &str, data: &[u8]) -> Message {
	serialize_call(1, None, object, method, None, Some(data))
}
pub fn serialize_method_call(id: u64, object: &str, method: &str, data: &[u8]) -> Message {
	serialize_call(2, Some(id), object, method, None, Some(data))
}
fn serialize_call(
	call_type: u8,
	id: Option<u64>,
	path: &str,
	method: &str,
	err: Option<&str>,
	data: Option<&[u8]>,
) -> Message {
	debug!(
		call_type = match call_type {
			0 => "error",
			1 => "signal",
			2 => "method call",
			3 => "method return",
			_ => "unknown",
		},
		id,
		path,
		method,
		err,
		content = data
			.map(|data| match flexbuffers::Reader::get_root(data) {
				Ok(root) => format!("{}", root),
				Err(_) => String::from_utf8_lossy(data).into_owned(),
			})
			.unwrap_or_else(|| {
				err.map(|err| err.to_string())
					.unwrap_or_else(|| "Unknown".to_string())
			})
	);

	let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(1024);
	let flex_path = fbb.create_string(path);
	let flex_method = fbb.create_string(method);
	let flex_err = err.map(|s| fbb.create_string(s));
	let flex_data = data.map(|s| fbb.create_vector(s));

	let message_constructed = FlatMessage::create(
		&mut fbb,
		&MessageArgs {
			type_: call_type,
			id: id.unwrap_or(0),
			object: Some(flex_path),
			method: Some(flex_method),
			error: flex_err,
			data: flex_data,
		},
	);
	fbb.finish(message_constructed, None);
	Message {
		data: fbb.finished_data().to_vec(),
	}
}

pub struct MessageSender {
	write: OwnedWriteHalf,
	handle: MessageSenderHandle,
	message_rx: mpsc::UnboundedReceiver<Message>,
	pending_future_tx: PendingFutureSender,
	max_message_id: Arc<AtomicU64>,
}
impl MessageSender {
	fn new(write: OwnedWriteHalf, pending_future_tx: PendingFutureSender) -> Self {
		let (message_tx, message_rx) = mpsc::unbounded_channel();
		let max_message_id = Arc::new(AtomicU64::new(0));
		MessageSender {
			write,
			handle: MessageSenderHandle {
				message_tx,
				pending_future_tx: pending_future_tx.clone(),
				max_message_id: max_message_id.clone(),
			},
			message_rx,
			pending_future_tx,
			max_message_id,
		}
	}
	pub async fn flush(&mut self) -> Result<(), MessengerError> {
		while let Some(message) = self.message_rx.recv().await {
			self.send(&message.into_data()).await?;
		}
		Ok(())
	}
	pub async fn send(&mut self, message: &[u8]) -> Result<(), MessengerError> {
		let message_length = message.len() as u32;
		self.write
			.write_all(&message_length.to_ne_bytes())
			.await
			.map_err(|e| MessengerError::IOError { e })?;
		self.write
			.write_all(message)
			.await
			.map_err(|e| MessengerError::IOError { e })
	}
	pub fn handle(&self) -> MessageSenderHandle {
		self.handle.clone()
	}

	pub async fn error<E: std::fmt::Display>(
		&mut self,
		node_path: &str,
		method_name: &str,
		err: E,
	) -> Result<(), MessengerError> {
		self.send(&serialize_error(node_path, method_name, err).into_data())
			.await
	}
	pub async fn signal(
		&mut self,
		node_path: &str,
		signal_name: &str,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(&serialize_signal_call(node_path, signal_name, data).into_data())
			.await
	}
	pub async fn method(
		&mut self,
		node_path: &str,
		method: &str,
		data: &[u8],
	) -> Result<Result<Vec<u8>, String>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.max_message_id.load(Ordering::Relaxed);
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(&serialize_method_call(id, node_path, method, data).into_data())
			.await?;
		self.max_message_id.store(id + 1, Ordering::Relaxed);
		rx.await.map_err(|_| MessengerError::ReceiverDropped)
	}
}

#[derive(Clone)]
pub struct MessageSenderHandle {
	message_tx: mpsc::UnboundedSender<Message>,
	pending_future_tx: PendingFutureSender,
	max_message_id: Arc<AtomicU64>,
}
impl MessageSenderHandle {
	pub fn error<E: std::fmt::Display>(
		&self,
		node_path: &str,
		method_name: &str,
		err: E,
	) -> Result<(), MessengerError> {
		self.send(serialize_error(node_path, method_name, err))
	}
	pub fn signal(
		&self,
		node_path: &str,
		signal_name: &str,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_signal_call(node_path, signal_name, data))
	}
	pub fn method(
		&self,
		node_path: &str,
		method: &str,
		data: &[u8],
	) -> Result<impl Future<Output = Result<Vec<u8>, String>>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.max_message_id.load(Ordering::Relaxed);
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(serialize_method_call(id, node_path, method, data))?;
		self.max_message_id.store(id + 1, Ordering::Relaxed);
		Ok(async move { rx.await.map_err(|e| e.to_string())? })
	}

	fn send(&self, message: Message) -> Result<(), MessengerError> {
		self.message_tx
			.send(message)
			.map_err(|_| MessengerError::ReceiverDropped)
	}
}

pub fn create(connection: UnixStream) -> (MessageSender, MessageReceiver) {
	let (read, write) = connection.into_split();
	let (pending_future_tx, pending_future_rx) = mpsc::unbounded_channel();
	let sender = MessageSender::new(write, pending_future_tx);
	let receiver = MessageReceiver::new(read, pending_future_rx, sender.handle());
	(sender, receiver)
}
