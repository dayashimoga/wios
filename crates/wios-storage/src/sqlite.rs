//! SQLite storage backend.

use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use wios_core::error::{WiosError, WiosResult};

/// SQLite-backed key-value and relational storage.
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStore {
    /// Open or create a SQLite database.
    pub fn open(path: impl AsRef<Path>) -> WiosResult<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)
            .map_err(|e| WiosError::Storage(format!("Failed to open SQLite: {}", e)))?;

        // Enable WAL mode for better concurrent read performance
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;")
            .map_err(|e| WiosError::Storage(e.to_string()))?;

        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.initialize_schema_sync()?;

        info!("SQLite store opened at {:?}", path);
        Ok(store)
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> WiosResult<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")
            .map_err(|e| WiosError::Storage(e.to_string()))?;

        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.initialize_schema_sync()?;
        Ok(store)
    }

    /// Initialize the base schema.
    fn initialize_schema_sync(&self) -> WiosResult<()> {
        // Use try_lock since no contention exists during initialization
        let conn = self.conn.try_lock()
            .map_err(|_| WiosError::Storage("Failed to acquire lock during initialization".into()))?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS kv_store (
                namespace TEXT NOT NULL,
                key TEXT NOT NULL,
                value BLOB NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                version INTEGER NOT NULL DEFAULT 1,
                deleted INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (namespace, key)
            );

            CREATE TABLE IF NOT EXISTS sync_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                namespace TEXT NOT NULL,
                key TEXT NOT NULL,
                operation TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                node_id TEXT NOT NULL,
                version INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_kv_namespace ON kv_store(namespace);
            CREATE INDEX IF NOT EXISTS idx_sync_timestamp ON sync_log(timestamp);
            ",
        )
        .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Store a key-value pair.
    pub async fn put(&self, namespace: &str, key: &str, value: &[u8]) -> WiosResult<()> {
        let conn = self.conn.lock().await;
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO kv_store (namespace, key, value, created_at, updated_at, version, deleted)
             VALUES (?1, ?2, ?3, ?4, ?4, 1, 0)
             ON CONFLICT(namespace, key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at,
                version = kv_store.version + 1,
                deleted = 0",
            params![namespace, key, value, now],
        )
        .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Retrieve a value by key.
    pub async fn get(&self, namespace: &str, key: &str) -> WiosResult<Option<Vec<u8>>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT value FROM kv_store WHERE namespace = ?1 AND key = ?2 AND deleted = 0")
            .map_err(|e| WiosError::Storage(e.to_string()))?;

        let result = stmt
            .query_row(params![namespace, key], |row| row.get(0))
            .optional()
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(result)
    }

    /// Delete a key (soft delete for sync).
    pub async fn delete(&self, namespace: &str, key: &str) -> WiosResult<bool> {
        let conn = self.conn.lock().await;
        let now = Utc::now().to_rfc3339();
        let rows = conn
            .execute(
                "UPDATE kv_store SET deleted = 1, updated_at = ?3, version = version + 1
                 WHERE namespace = ?1 AND key = ?2 AND deleted = 0",
                params![namespace, key, now],
            )
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(rows > 0)
    }

    /// List keys in a namespace.
    pub async fn list_keys(&self, namespace: &str, prefix: Option<&str>) -> WiosResult<Vec<String>> {
        let conn = self.conn.lock().await;
        let mut keys = Vec::new();

        match prefix {
            Some(p) => {
                let pattern = format!("{}%", p);
                let mut stmt = conn
                    .prepare(
                        "SELECT key FROM kv_store WHERE namespace = ?1 AND key LIKE ?2 AND deleted = 0",
                    )
                    .map_err(|e| WiosError::Storage(e.to_string()))?;
                let rows = stmt
                    .query_map(params![namespace, pattern], |row| row.get(0))
                    .map_err(|e| WiosError::Storage(e.to_string()))?;
                for row in rows {
                    keys.push(row.map_err(|e| WiosError::Storage(e.to_string()))?);
                }
            }
            None => {
                let mut stmt = conn
                    .prepare("SELECT key FROM kv_store WHERE namespace = ?1 AND deleted = 0")
                    .map_err(|e| WiosError::Storage(e.to_string()))?;
                let rows = stmt
                    .query_map(params![namespace], |row| row.get(0))
                    .map_err(|e| WiosError::Storage(e.to_string()))?;
                for row in rows {
                    keys.push(row.map_err(|e| WiosError::Storage(e.to_string()))?);
                }
            }
        }
        Ok(keys)
    }

    /// Check if a key exists.
    pub async fn exists(&self, namespace: &str, key: &str) -> WiosResult<bool> {
        let conn = self.conn.lock().await;
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM kv_store WHERE namespace = ?1 AND key = ?2 AND deleted = 0",
                params![namespace, key],
                |row| row.get(0),
            )
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(count > 0)
    }

    /// Get storage statistics.
    pub async fn stats(&self) -> WiosResult<StorageStats> {
        let conn = self.conn.lock().await;
        let total_keys: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM kv_store WHERE deleted = 0",
                [],
                |row| row.get(0),
            )
            .map_err(|e| WiosError::Storage(e.to_string()))?;

        let total_bytes: u64 = conn
            .query_row(
                "SELECT COALESCE(SUM(LENGTH(value)), 0) FROM kv_store WHERE deleted = 0",
                [],
                |row| row.get(0),
            )
            .map_err(|e| WiosError::Storage(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT DISTINCT namespace FROM kv_store WHERE deleted = 0")
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        let namespaces: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| WiosError::Storage(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(StorageStats {
            total_keys,
            total_bytes,
            namespaces,
        })
    }
}

/// Storage statistics.
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_keys: u64,
    pub total_bytes: u64,
    pub namespaces: Vec<String>,
}

// Make optional() available on rusqlite results
use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_put_get() {
        let store = SqliteStore::open_in_memory().unwrap();
        store.put("test", "key1", b"value1").await.unwrap();
        let value = store.get("test", "key1").await.unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));
    }

    #[tokio::test]
    async fn test_get_nonexistent() {
        let store = SqliteStore::open_in_memory().unwrap();
        let value = store.get("test", "missing").await.unwrap();
        assert!(value.is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let store = SqliteStore::open_in_memory().unwrap();
        store.put("test", "key1", b"value1").await.unwrap();
        assert!(store.delete("test", "key1").await.unwrap());
        assert!(store.get("test", "key1").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let store = SqliteStore::open_in_memory().unwrap();
        store.put("ns", "foo_1", b"v").await.unwrap();
        store.put("ns", "foo_2", b"v").await.unwrap();
        store.put("ns", "bar_1", b"v").await.unwrap();

        let all = store.list_keys("ns", None).await.unwrap();
        assert_eq!(all.len(), 3);

        let foo = store.list_keys("ns", Some("foo_")).await.unwrap();
        assert_eq!(foo.len(), 2);
    }

    #[tokio::test]
    async fn test_upsert() {
        let store = SqliteStore::open_in_memory().unwrap();
        store.put("test", "key1", b"v1").await.unwrap();
        store.put("test", "key1", b"v2").await.unwrap();
        let value = store.get("test", "key1").await.unwrap();
        assert_eq!(value, Some(b"v2".to_vec()));
    }

    #[tokio::test]
    async fn test_stats() {
        let store = SqliteStore::open_in_memory().unwrap();
        store.put("ns1", "k1", b"hello").await.unwrap();
        store.put("ns2", "k2", b"world").await.unwrap();

        let stats = store.stats().await.unwrap();
        assert_eq!(stats.total_keys, 2);
        assert_eq!(stats.namespaces.len(), 2);
    }
}
