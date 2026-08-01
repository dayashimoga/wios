//! # WIOS Core
//!
//! Core types, error handling, configuration, traits, and event system
//! for the Wireless Intelligence Operating System.
//!
//! This crate provides the foundational abstractions that all other WIOS
//! crates depend on. It defines:
//!
//! - **Configuration** — TOML/YAML-based hierarchical configuration
//! - **Error Types** — Comprehensive error hierarchy for all subsystems
//! - **Domain Types** — Core identifiers (DeviceId, PeerId, NodeInfo, etc.)
//! - **Service Traits** — Interface definitions for all services
//! - **Event System** — Pub/sub event bus for inter-component communication

pub mod config;
pub mod error;
pub mod event;
pub mod traits;
pub mod types;

pub use config::WiosConfig;
pub use error::{WiosError, WiosResult};
pub use types::{DeviceCapabilities, DeviceId, NodeId, NodeInfo, PeerId, Timestamp};
