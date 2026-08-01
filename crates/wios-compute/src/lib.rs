//! # WIOS Compute
//!
//! Distributed compute task management and device capability sharing.

pub mod scheduler;
pub mod resource;
pub mod distributed;
pub mod device_bus;

pub use scheduler::TaskScheduler;
pub use resource::ResourceManager;
pub use distributed::TaskDistributor;
pub use device_bus::DeviceBus;
