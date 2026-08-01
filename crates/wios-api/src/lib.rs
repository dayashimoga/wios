//! # WIOS API
//!
//! REST, gRPC, and plugin API server for external access to WIOS services.

pub mod grpc;
pub mod middleware;
pub mod plugin;
pub mod rest;

pub use plugin::PluginRegistry;
pub use rest::create_router;
