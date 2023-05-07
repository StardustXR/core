//! Contains From/Into impls for flatbuffers types to mint types.

pub use flatbuffers;

pub(crate) mod generated;

mod common;
pub use common::*;
mod hand;
pub use hand::*;
mod input_data;
pub use input_data::*;
mod pointer;
pub use pointer::*;
mod tip;
pub use tip::*;

pub use generated::message;
