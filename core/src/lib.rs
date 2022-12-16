//! Standard crate all Stardust clients/servers should use.

/// Standard connection to servers as a client
pub mod client;
/// Standard creation of sockets for servers
pub mod server;

/// Symmetrical messenger for client/server
pub mod messenger;
/// Scenegraph trait and error for messenger
pub mod scenegraph;

/// Common values for client/server
#[macro_use]
pub mod values;

pub use stardust_xr_schemas as schemas;
