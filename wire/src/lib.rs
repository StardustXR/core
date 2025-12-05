//! Standard crate all Stardust clients/servers should use.

/// Standard connection to servers as a client
pub mod client;
/// Standard creation of sockets for servers
pub mod server;

/// Message format flatbuffer
pub mod message {
	#![allow(unused)]
	#![allow(unsafe_op_in_unsafe_fn)]
	#![allow(mismatched_lifetime_syntaxes)]
	#![allow(clippy::all)]
	pub use stardust_xr::*;
	include!("message.rs");
}
/// Symmetrical messenger for client/server
pub mod messenger;
/// Scenegraph trait and error for messenger
pub mod scenegraph;

/// Flexbuffer format serialize/deserialize
pub mod flex;
/// Common values for client/server
#[macro_use]
pub mod values;
