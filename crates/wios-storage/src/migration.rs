//! Schema migration system.

use chrono::Utc;
use rusqlite::{params, Connection};

use tracing::info;
use wios_core::error::{WiosError, WiosResult};

/// A database migration.
pub struct Migration {
    pub name: &'static str,
    pub sql: &'static str,
}

/// Runs schema migrations in order.
pub struct MigrationRunner {
    migrations: Vec<Migration>,
}

impl MigrationRunner {
    /// Create a new migration runner with the given migrations.
    pub fn new(migrations: Vec<Migration>) -> Self {
        Self { migrations }
    }

    /// Run all pending migrations.
    pub fn run(&self, conn: &Connection) -> WiosResult<usize> {
        let mut applied = 0;
        for migration in &self.migrations {
            if !self.is_applied(conn, migration.name)? {
                conn.execute_batch(migration.sql)
                    .map_err(|e| WiosError::Migration(format!("{}: {}", migration.name, e)))?;
                self.mark_applied(conn, migration.name)?;
                info!("Applied migration: {}", migration.name);
                applied += 1;
            }
        }
        Ok(applied)
    }

    fn is_applied(&self, conn: &Connection, name: &str) -> WiosResult<bool> {
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM migrations WHERE name = ?1",
                params![name],
                |row| row.get(0),
            )
            .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(count > 0)
    }

    fn mark_applied(&self, conn: &Connection, name: &str) -> WiosResult<()> {
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO migrations (name, applied_at) VALUES (?1, ?2)",
            params![name, now],
        )
        .map_err(|e| WiosError::Storage(e.to_string()))?;
        Ok(())
    }
}

/// Get the default WIOS migrations.
pub fn default_migrations() -> Vec<Migration> {
    vec![
        Migration {
            name: "001_create_nodes",
            sql: "
                CREATE TABLE IF NOT EXISTS nodes (
                    node_id TEXT PRIMARY KEY,
                    device_id TEXT NOT NULL,
                    peer_id TEXT,
                    name TEXT NOT NULL,
                    platform TEXT NOT NULL,
                    capabilities TEXT NOT NULL DEFAULT '{}',
                    version TEXT NOT NULL,
                    first_seen TEXT NOT NULL,
                    last_seen TEXT NOT NULL,
                    is_online INTEGER NOT NULL DEFAULT 0
                );
                CREATE INDEX IF NOT EXISTS idx_nodes_online ON nodes(is_online);
            ",
        },
        Migration {
            name: "002_create_messages",
            sql: "
                CREATE TABLE IF NOT EXISTS messages (
                    id TEXT PRIMARY KEY,
                    sender TEXT NOT NULL,
                    recipient TEXT,
                    topic TEXT NOT NULL,
                    payload BLOB NOT NULL,
                    priority INTEGER NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL,
                    delivered_at TEXT,
                    ttl_secs INTEGER NOT NULL DEFAULT 300,
                    hop_count INTEGER NOT NULL DEFAULT 0,
                    FOREIGN KEY (sender) REFERENCES nodes(node_id)
                );
                CREATE INDEX IF NOT EXISTS idx_messages_topic ON messages(topic);
                CREATE INDEX IF NOT EXISTS idx_messages_created ON messages(created_at);
            ",
        },
        Migration {
            name: "003_create_files",
            sql: "
                CREATE TABLE IF NOT EXISTS files (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    mime_type TEXT NOT NULL,
                    size_bytes INTEGER NOT NULL,
                    hash TEXT NOT NULL,
                    encrypted INTEGER NOT NULL DEFAULT 1,
                    owner TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (owner) REFERENCES nodes(node_id)
                );
                CREATE TABLE IF NOT EXISTS file_chunks (
                    id TEXT PRIMARY KEY,
                    file_id TEXT NOT NULL,
                    chunk_index INTEGER NOT NULL,
                    data BLOB NOT NULL,
                    hash TEXT NOT NULL,
                    FOREIGN KEY (file_id) REFERENCES files(id)
                );
                CREATE INDEX IF NOT EXISTS idx_chunks_file ON file_chunks(file_id, chunk_index);
            ",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            );",
        )
        .unwrap();

        let runner = MigrationRunner::new(default_migrations());
        let applied = runner.run(&conn).unwrap();
        assert_eq!(applied, 3);

        // Running again should apply nothing
        let applied = runner.run(&conn).unwrap();
        assert_eq!(applied, 0);
    }
}
