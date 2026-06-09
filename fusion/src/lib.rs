//! A library for Stardust XR clients to use with abstractions over the client and event loop.

#![allow(dead_code)]

mod error;
pub mod tracked;
pub mod keymap;
pub use error::{Error, Result};

pub mod audio;
pub mod camera;
pub mod client;
pub mod dmatex;
pub mod drawable;
pub mod fields;
pub mod query;
pub mod spatial;
pub mod spatial_query;
pub mod suis;
pub mod types;
