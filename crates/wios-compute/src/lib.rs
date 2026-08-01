//! # WIOS Compute
//!
//! Distributed compute task management and device capability sharing.

pub mod device_bus;
pub mod distributed;
pub mod resource;
pub mod scheduler;

pub use device_bus::DeviceBus;
pub use distributed::TaskDistributor;
pub use resource::ResourceManager;
pub use scheduler::TaskScheduler;
