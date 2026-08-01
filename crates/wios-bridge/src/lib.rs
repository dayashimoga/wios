//! # WIOS Bridge
//!
//! API surface for flutter_rust_bridge integration.
//!
//! This crate exposes the Rust backend functionality to the Flutter
//! frontend through a clean, typed API. Functions in this crate are
//! designed to be consumed by flutter_rust_bridge's code generator.

pub mod api;

pub use api::*;
