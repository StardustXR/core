//! A library for Stardust XR clients to use with abstractions over the client and event loop.

#![allow(dead_code)]

pub use stardust_xr_protocol::protocol::*;

pub mod client;
pub mod spatial;
pub mod fields;
pub mod input;
pub mod drawable;
pub mod audio;
pub mod camera;
