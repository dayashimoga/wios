//! Pluggable storage backend trait — SQLite default, RocksDB optional.

use async_trait::async_trait;
use wios_core::error::WiosResult;

/// Storage backend trait — implementations for SQLite and RocksDB.
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store a key-value pair in a namespace.
    async fn put(&self, namespace: &str, key: &str, value: &[u8]) -> WiosResult<()>;

    /// Retrieve a value by key.
    async fn get(&self, namespace: &str, key: &str) -> WiosResult<Option<Vec<u8>>>;

    /// Delete a key.
    async fn delete(&self, namespace: &str, key: &str) -> WiosResult<()>;

    /// List all keys in a namespace.
    async fn list_keys(&self, namespace: &str) -> WiosResult<Vec<String>>;

    /// Check if a key exists.
    async fn exists(&self, namespace: &str, key: &str) -> WiosResult<bool> {
        Ok(self.get(namespace, key).await?.is_some())
    }

    /// Get backend name.
    fn backend_name(&self) -> &str;

    /// Flush writes to disk.
    async fn flush(&self) -> WiosResult<()>;
}

/// SQLite implementation of StorageBackend (wraps existing SqliteStore).
pub struct SqliteBackend {
    store: crate::SqliteStore,
}

impl SqliteBackend {
    pub fn new(store: crate::SqliteStore) -> Self {
        Self { store }
    }
}

#[async_trait]
impl StorageBackend for SqliteBackend {
    async fn put(&self, namespace: &str, key: &str, value: &[u8]) -> WiosResult<()> {
        self.store.put(namespace, key, value).await
    }

    async fn get(&self, namespace: &str, key: &str) -> WiosResult<Option<Vec<u8>>> {
        self.store.get(namespace, key).await
    }

    async fn delete(&self, namespace: &str, key: &str) -> WiosResult<()> {
        self.store.delete(namespace, key).await.map(|_| ())
    }

    async fn list_keys(&self, namespace: &str) -> WiosResult<Vec<String>> {
        self.store.list_keys(namespace, None).await
    }

    fn backend_name(&self) -> &str {
        "sqlite"
    }

    async fn flush(&self) -> WiosResult<()> {
        Ok(()) // SQLite auto-flushes with WAL
    }
}

/// RocksDB stub backend — ready for `rocksdb` crate integration.
/// Enable by adding `rocksdb = "0.22"` to Cargo.toml dependencies.
pub struct RocksDbBackend {
    // When rocksdb crate is added: db: rocksdb::DB,
    _path: String,
}

impl RocksDbBackend {
    /// Create a new RocksDB backend (stub — returns error until rocksdb crate is linked).
    pub fn new(path: &str) -> WiosResult<Self> {
        Ok(Self { _path: path.to_string() })
    }
}

#[async_trait]
impl StorageBackend for RocksDbBackend {
    async fn put(&self, _namespace: &str, _key: &str, _value: &[u8]) -> WiosResult<()> {
        Err(wios_core::error::WiosError::NotImplemented(
            "RocksDB backend requires 'rocksdb' crate — add to Cargo.toml".into(),
        ))
    }

    async fn get(&self, _namespace: &str, _key: &str) -> WiosResult<Option<Vec<u8>>> {
        Err(wios_core::error::WiosError::NotImplemented(
            "RocksDB backend requires 'rocksdb' crate".into(),
        ))
    }

    async fn delete(&self, _namespace: &str, _key: &str) -> WiosResult<()> {
        Err(wios_core::error::WiosError::NotImplemented(
            "RocksDB backend requires 'rocksdb' crate".into(),
        ))
    }

    async fn list_keys(&self, _namespace: &str) -> WiosResult<Vec<String>> {
        Err(wios_core::error::WiosError::NotImplemented(
            "RocksDB backend requires 'rocksdb' crate".into(),
        ))
    }

    fn backend_name(&self) -> &str {
        "rocksdb"
    }

    async fn flush(&self) -> WiosResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_backend_via_trait() {
        let store = crate::SqliteStore::open_in_memory().unwrap();
        let backend = SqliteBackend::new(store);
        assert_eq!(backend.backend_name(), "sqlite");

        backend.put("test", "k1", b"v1").await.unwrap();
        let val = backend.get("test", "k1").await.unwrap();
        assert_eq!(val, Some(b"v1".to_vec()));
        assert!(backend.exists("test", "k1").await.unwrap());

        backend.delete("test", "k1").await.unwrap();
        assert!(!backend.exists("test", "k1").await.unwrap());
    }

    #[tokio::test]
    async fn test_rocksdb_stub() {
        let backend = RocksDbBackend::new("/tmp/test-db").unwrap();
        assert_eq!(backend.backend_name(), "rocksdb");
        assert!(backend.put("ns", "k", b"v").await.is_err());
    }
}
