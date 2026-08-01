//! # WIOS API
//!
//! REST, gRPC, and plugin API server for external access to WIOS services.

pub mod rest;
pub mod middleware;
pub mod grpc;
pub mod plugin;

pub use rest::create_router;
pub use plugin::PluginRegistry;
