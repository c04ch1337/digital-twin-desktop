//! Infrastructure layer implementation.
//!
//! This module contains all infrastructure-level implementations including:
//! - Database access and repositories
//! - LLM client implementations
//! - Tool executors
//! - Configuration management
//! - Logging infrastructure
//! - Security utilities

pub mod config;
pub mod db;
pub mod llm;
pub mod tools;
pub mod logging;
pub mod security;

// Re-export commonly used types
pub use config::AppConfig;
pub use db::{
    SqliteManager,
    repositories::{
        SqliteAgentRepository,
        SqliteConversationRepository,
        SqliteTwinRepository,
        SqliteSensorDataRepository,
        SqliteToolRepository,
        SqliteRepositoryFactory,
    },
};
pub use llm::{
    AnthropicClient,
    OpenAIClient,
    DefaultLLMClientFactory,
};
pub use tools::{
    FileToolExecutor,
    WebToolExecutor,
    ModbusToolExecutor,
    MqttToolExecutor,
    TwinToolExecutor,
    DefaultToolExecutorFactory,
    DefaultToolExecutorRegistry,
};
pub use logging::{
    init_logging,
    scope_guard,
    log_metrics,
    request_logger,
};
pub use security::{
    TokenManager,
    PasswordHasher,
    RateLimiter,
    PermissionChecker,
};

/// Initialize infrastructure layer
pub async fn init(config: &AppConfig) -> anyhow::Result<()> {
    // Initialize logging
    logging::init_logging(&config.logging)?;

    // Log startup
    tracing::info!("Initializing infrastructure layer...");

    // Initialize database
    let db = SqliteManager::new(config.database.clone()).await?;
    db.run_migrations().await?;

    tracing::info!("Infrastructure layer initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_infrastructure_init() {
        let temp_dir = tempdir().unwrap();
        let config = AppConfig {
            database: config::DatabaseConfig {
                path: temp_dir.path().join("test.db"),
                max_connections: 5,
                foreign_keys: true,
                wal_mode: true,
            },
            logging: config::LoggingConfig {
                level: "debug".to_string(),
                file_path: None,
                json_format: false,
            },
            ..Default::default()
        };

        assert!(init(&config).await.is_ok());
    }
}