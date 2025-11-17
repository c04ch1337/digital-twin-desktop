//! Repository implementations for SQLite database.

mod agent_repository;
mod conversation_repository;
mod sensor_data_repository;
mod tool_repository;
mod twin_repository;

pub use agent_repository::SqliteAgentRepository;
pub use conversation_repository::SqliteConversationRepository;
pub use sensor_data_repository::SqliteSensorDataRepository;
pub use tool_repository::SqliteToolRepository;
pub use twin_repository::SqliteTwinRepository;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::core::domain::traits::repository::{
    RepositoryFactory, RepositoryResult, ConversationRepository,
    AgentRepository, TwinRepository, SensorDataRepository, ToolRepository,
    UnitOfWork, Transaction,
};

/// SQLite implementation of the repository factory
pub struct SqliteRepositoryFactory {
    pool: Pool<Sqlite>,
}

impl SqliteRepositoryFactory {
    /// Create a new SQLite repository factory
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepositoryFactory for SqliteRepositoryFactory {
    async fn create_conversation_repository(&self) -> Box<dyn ConversationRepository> {
        Box::new(SqliteConversationRepository::new(self.pool.clone()))
    }

    async fn create_agent_repository(&self) -> Box<dyn AgentRepository> {
        Box::new(SqliteAgentRepository::new(self.pool.clone()))
    }

    async fn create_twin_repository(&self) -> Box<dyn TwinRepository> {
        Box::new(SqliteTwinRepository::new(self.pool.clone()))
    }

    async fn create_sensor_data_repository(&self) -> Box<dyn SensorDataRepository> {
        Box::new(SqliteSensorDataRepository::new(self.pool.clone()))
    }

    async fn create_tool_repository(&self) -> Box<dyn ToolRepository> {
        Box::new(SqliteToolRepository::new(self.pool.clone()))
    }

    async fn create_unit_of_work(&self) -> Box<dyn UnitOfWork> {
        Box::new(SqliteUnitOfWork::new(self.pool.clone()))
    }
}

/// SQLite implementation of the unit of work pattern
pub struct SqliteUnitOfWork {
    pool: Pool<Sqlite>,
}

impl SqliteUnitOfWork {
    /// Create a new SQLite unit of work
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for SqliteUnitOfWork {
    async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>> {
        let transaction = self.pool
            .begin()
            .await
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;

        Ok(Box::new(SqliteTransaction {
            transaction: Some(transaction),
            conversation_repo: SqliteConversationRepository::new(self.pool.clone()),
            agent_repo: SqliteAgentRepository::new(self.pool.clone()),
            twin_repo: SqliteTwinRepository::new(self.pool.clone()),
            sensor_data_repo: SqliteSensorDataRepository::new(self.pool.clone()),
            tool_repo: SqliteToolRepository::new(self.pool.clone()),
        }))
    }
}

/// SQLite implementation of a transaction
pub struct SqliteTransaction {
    transaction: Option<sqlx::Transaction<'static, Sqlite>>,
    conversation_repo: SqliteConversationRepository,
    agent_repo: SqliteAgentRepository,
    twin_repo: SqliteTwinRepository,
    sensor_data_repo: SqliteSensorDataRepository,
    tool_repo: SqliteToolRepository,
}

#[async_trait]
impl Transaction for SqliteTransaction {
    async fn commit(mut self: Box<Self>) -> RepositoryResult<()> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .commit()
                .await
                .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;
        }
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> RepositoryResult<()> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .rollback()
                .await
                .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;
        }
        Ok(())
    }

    fn conversations(&self) -> &dyn ConversationRepository {
        &self.conversation_repo
    }

    fn agents(&self) -> &dyn AgentRepository {
        &self.agent_repo
    }

    fn twins(&self) -> &dyn TwinRepository {
        &self.twin_repo
    }

    fn sensor_data(&self) -> &dyn SensorDataRepository {
        &self.sensor_data_repo
    }

    fn tools(&self) -> &dyn ToolRepository {
        &self.tool_repo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use tempfile::tempdir;

    async fn create_test_db() -> Pool<Sqlite> {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(db_path)
                    .create_if_missing(true)
            )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_repository_factory() {
        let pool = create_test_db().await;
        let factory = SqliteRepositoryFactory::new(pool);

        let conversation_repo = factory.create_conversation_repository().await;
        let agent_repo = factory.create_agent_repository().await;
        let twin_repo = factory.create_twin_repository().await;
        let sensor_repo = factory.create_sensor_data_repository().await;
        let tool_repo = factory.create_tool_repository().await;

        assert!(conversation_repo.get_by_id(Uuid::new_v4()).await.is_err());
        assert!(agent_repo.get_by_id(Uuid::new_v4()).await.is_err());
        assert!(twin_repo.get_by_id(Uuid::new_v4()).await.is_err());
        assert!(sensor_repo.get_by_id(Uuid::new_v4()).await.is_err());
        assert!(tool_repo.get_by_id(Uuid::new_v4()).await.is_err());
    }

    #[tokio::test]
    async fn test_transaction() {
        let pool = create_test_db().await;
        let factory = SqliteRepositoryFactory::new(pool);
        let uow = factory.create_unit_of_work().await;

        let tx = uow.begin().await.unwrap();
        tx.commit().await.unwrap();

        let tx = uow.begin().await.unwrap();
        tx.rollback().await.unwrap();
    }
}