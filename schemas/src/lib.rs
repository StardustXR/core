//! Standard schemas + object structs, (de)serialization of flexbuffers, and re-exporting standard flat/flexbuffer versions for Stardust XR.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::derive_partial_eq_without_eq)]
/// Flatbuffers schemas and exports.
pub mod flat;

/// To get the files compiling correctly
use flat::*;

/// Flexbuffers schemas and exports.
pub mod flex;

pub mod dbus;
pub mod protocol;
