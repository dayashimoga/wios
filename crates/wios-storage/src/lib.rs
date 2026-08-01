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

pub mod sqlite;
pub mod migration;
pub mod sync_engine;
pub mod chunking;
pub mod quota;
pub mod backend;
pub mod replication;
pub mod versioning;
pub mod backup;

pub use sqlite::SqliteStore;
pub use migration::MigrationRunner;
pub use sync_engine::SyncEngine;
pub use chunking::ChunkEngine;
pub use quota::QuotaManager;
pub use backend::{StorageBackend, SqliteBackend, RocksDbBackend};
pub use replication::ReplicationManager;
pub use versioning::VersionManager;
pub use backup::{BackupManager, MetricsCollector};
