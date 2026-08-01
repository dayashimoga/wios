//! # WIOS Storage
//!
//! Offline-first encrypted storage engine with CRDT-based sync.
//!
//! Provides:
//! - SQLite for structured/relational data
//! - Schema migration system
//! - Encrypted-at-rest storage
//! - CRDT-based conflict-free sync engine
//! - File chunking and deduplication
//! - Storage quota management
//! - Multi-node replication
//! - Content versioning with rollback
//! - Snapshot backup/restore
//! - System metrics collection

pub mod backend;
pub mod backup;
pub mod chunking;
pub mod migration;
pub mod quota;
pub mod replication;
pub mod sqlite;
pub mod sync_engine;
pub mod versioning;

pub use backend::{RocksDbBackend, SqliteBackend, StorageBackend};
pub use backup::{BackupManager, MetricsCollector};
pub use chunking::ChunkEngine;
pub use migration::MigrationRunner;
pub use quota::QuotaManager;
pub use replication::ReplicationManager;
pub use sqlite::SqliteStore;
pub use sync_engine::SyncEngine;
pub use versioning::VersionManager;
