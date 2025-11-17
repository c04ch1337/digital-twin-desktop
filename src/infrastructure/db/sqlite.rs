use std::path::Path;
use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, Pool, Sqlite};
use anyhow::Result;
use tokio::fs;
use tracing::{info, error};

/// SQLite database configuration
pub struct SqliteConfig {
    /// Database file path
    pub database_path: String,
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Enable foreign key constraints
    pub foreign_keys: bool,
    /// Enable WAL mode
    pub wal_mode: bool,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            database_path: "digital_twin.db".to_string(),
            max_connections: 5,
            foreign_keys: true,
            wal_mode: true,
        }
    }
}

/// SQLite connection manager
pub struct SqliteManager {
    pool: Pool<Sqlite>,
    config: SqliteConfig,
}

impl SqliteManager {
    /// Create a new SQLite manager instance
    pub async fn new(config: SqliteConfig) -> Result<Self> {
        // Ensure database directory exists
        if let Some(parent) = Path::new(&config.database_path).parent() {
            fs::create_dir_all(parent).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(&config.database_path)
                    .create_if_missing(true)
                    .foreign_keys(config.foreign_keys)
                    .journal_mode(if config.wal_mode {
                        sqlx::sqlite::SqliteJournalMode::Wal
                    } else {
                        sqlx::sqlite::SqliteJournalMode::Delete
                    })
            )
            .await?;

        info!("Connected to SQLite database at {}", config.database_path);

        Ok(Self { pool, config })
    }

    /// Get a connection pool reference
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations...");
        
        sqlx::migrate!("./src/infrastructure/db/migrations")
            .run(self.pool())
            .await?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Check database health
    pub async fn check_health(&self) -> Result<bool> {
        match sqlx::query("SELECT 1").execute(self.pool()).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Database health check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Get database configuration
    pub fn config(&self) -> &SqliteConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_sqlite_manager() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = SqliteConfig {
            database_path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        let manager = SqliteManager::new(config).await.unwrap();
        assert!(manager.check_health().await.unwrap());
    }
}