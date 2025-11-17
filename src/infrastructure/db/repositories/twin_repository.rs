use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;
use serde_json::Value;

use crate::core::domain::{
    models::{DigitalTwin, TwinId, TwinState, TwinType, AgentId},
    traits::repository::{
        TwinRepository, RepositoryResult, RepositoryError,
        FilterCriteria, SortCriteria, Pagination, PaginatedResult,
    },
};

pub struct SqliteTwinRepository {
    pool: Pool<Sqlite>,
}

impl SqliteTwinRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn build_filters(query: &mut String, filters: &[FilterCriteria]) -> Vec<Value> {
        let mut params = Vec::new();
        if !filters.is_empty() {
            query.push_str(" WHERE ");
            for (i, filter) in filters.iter().enumerate() {
                if i > 0 {
                    query.push_str(" AND ");
                }
                match filter.operator {
                    FilterOperator::Equals => {
                        query.push_str(&format!("{} = ?", filter.field));
                        params.push(filter.value.clone());
                    },
                    FilterOperator::Contains => {
                        query.push_str(&format!("{} LIKE ?", filter.field));
                        params.push(Value::String(format!("%{}%", filter.value.as_str().unwrap_or(""))));
                    },
                    // Add other operators as needed
                    _ => {}
                }
            }
        }
        params
    }
}

#[async_trait]
impl TwinRepository for SqliteTwinRepository {
    async fn create(&self, twin: DigitalTwin) -> RepositoryResult<DigitalTwin> {
        sqlx::query(
            "INSERT INTO digital_twins 
             (id, name, description, twin_type, state, properties, agent_id, metadata) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(twin.id.to_string())
        .bind(&twin.name)
        .bind(&twin.description)
        .bind(twin.twin_type.to_string())
        .bind(twin.state.to_string())
        .bind(serde_json::to_string(&twin.properties).unwrap_or("{}".to_string()))
        .bind(twin.agent_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&twin.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(twin)
    }

    async fn get_by_id(&self, id: TwinId) -> RepositoryResult<DigitalTwin> {
        let twin = sqlx::query_as!(
            TwinRow,
            "SELECT id, name, description, twin_type, state, properties, agent_id, 
                    last_sync, created_at, updated_at, metadata 
             FROM digital_twins 
             WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "DigitalTwin".to_string(),
            id: id.to_string(),
        })?;

        Ok(twin.into())
    }

    async fn update(&self, twin: DigitalTwin) -> RepositoryResult<DigitalTwin> {
        sqlx::query(
            "UPDATE digital_twins 
             SET name = ?, description = ?, twin_type = ?, state = ?, 
                 properties = ?, agent_id = ?, updated_at = CURRENT_TIMESTAMP, metadata = ?
             WHERE id = ?"
        )
        .bind(&twin.name)
        .bind(&twin.description)
        .bind(twin.twin_type.to_string())
        .bind(twin.state.to_string())
        .bind(serde_json::to_string(&twin.properties).unwrap_or("{}".to_string()))
        .bind(twin.agent_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&twin.metadata).unwrap_or("{}".to_string()))
        .bind(twin.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(twin)
    }

    async fn delete(&self, id: TwinId) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM digital_twins WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find(
        &self,
        filters: Vec<FilterCriteria>,
        sort: Vec<SortCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>> {
        let mut query = String::from(
            "SELECT id, name, description, twin_type, state, properties, 
                    agent_id, last_sync, created_at, updated_at, metadata 
             FROM digital_twins"
        );
        let params = Self::build_filters(&mut query, &filters).await;

        if !sort.is_empty() {
            query.push_str(" ORDER BY ");
            for (i, sort_criteria) in sort.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                query.push_str(&format!(
                    "{} {}",
                    sort_criteria.field,
                    if sort_criteria.order == SortOrder::Ascending { "ASC" } else { "DESC" }
                ));
            }
        }

        query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.limit, pagination.offset));

        let twins = sqlx::query_as::<_, TwinRow>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM digital_twins")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: twins.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_type(
        &self,
        twin_type: &TwinType,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>> {
        let twins = sqlx::query_as!(
            TwinRow,
            "SELECT id, name, description, twin_type, state, properties, 
                    agent_id, last_sync, created_at, updated_at, metadata 
             FROM digital_twins 
             WHERE twin_type = ? 
             LIMIT ? OFFSET ?",
            twin_type.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM digital_twins WHERE twin_type = ?",
            twin_type.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: twins.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_state(
        &self,
        state: TwinState,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>> {
        let twins = sqlx::query_as!(
            TwinRow,
            "SELECT id, name, description, twin_type, state, properties, 
                    agent_id, last_sync, created_at, updated_at, metadata 
             FROM digital_twins 
             WHERE state = ? 
             LIMIT ? OFFSET ?",
            state.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM digital_twins WHERE state = ?",
            state.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: twins.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_agent_id(
        &self,
        agent_id: AgentId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>> {
        let twins = sqlx::query_as!(
            TwinRow,
            "SELECT id, name, description, twin_type, state, properties, 
                    agent_id, last_sync, created_at, updated_at, metadata 
             FROM digital_twins 
             WHERE agent_id = ? 
             LIMIT ? OFFSET ?",
            agent_id.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM digital_twins WHERE agent_id = ?",
            agent_id.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: twins.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn update_state(&self, id: TwinId, state: TwinState) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE digital_twins 
             SET state = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ?"
        )
        .bind(state.to_string())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_properties(
        &self,
        id: TwinId,
        properties: HashMap<String, serde_json::Value>,
    ) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE digital_twins 
             SET properties = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ?"
        )
        .bind(serde_json::to_string(&properties).unwrap_or("{}".to_string()))
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_synchronized(
        &self,
        id: TwinId,
        timestamp: DateTime<Utc>,
    ) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE digital_twins 
             SET last_sync = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ?"
        )
        .bind(timestamp)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_twins_needing_sync(&self, limit: usize) -> RepositoryResult<Vec<DigitalTwin>> {
        let twins = sqlx::query_as!(
            TwinRow,
            "SELECT id, name, description, twin_type, state, properties, 
                    agent_id, last_sync, created_at, updated_at, metadata 
             FROM digital_twins 
             WHERE last_sync IS NULL OR 
                   (state = ? AND datetime(last_sync) < datetime('now', '-1 hour')) 
             ORDER BY last_sync ASC NULLS FIRST 
             LIMIT ?",
            TwinState::Active.to_string(),
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(twins.into_iter().map(|t| t.into()).collect())
    }
}

// Database row structure
#[derive(sqlx::FromRow)]
struct TwinRow {
    id: String,
    name: String,
    description: String,
    twin_type: String,
    state: String,
    properties: String,
    agent_id: Option<String>,
    last_sync: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    metadata: String,
}

impl From<TwinRow> for DigitalTwin {
    fn from(row: TwinRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: row.name,
            description: row.description,
            twin_type: row.twin_type.parse().unwrap_or_default(),
            state: row.state.parse().unwrap_or_default(),
            properties: serde_json::from_str(&row.properties).unwrap_or_default(),
            agent_id: row.agent_id.map(|id| Uuid::parse_str(&id).unwrap()),
            last_sync: row.last_sync,
            created_at: row.created_at,
            updated_at: row.updated_at,
            metadata: serde_json::from_str(&row.metadata).unwrap_or_default(),
        }
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
    async fn test_twin_crud() {
        let pool = create_test_db().await;
        let repo = SqliteTwinRepository::new(pool);

        // Create test twin
        let twin = DigitalTwin {
            id: Uuid::new_v4(),
            name: "Test Twin".to_string(),
            description: "Test Description".to_string(),
            twin_type: TwinType::Device,
            state: TwinState::Active,
            properties: HashMap::new(),
            agent_id: None,
            last_sync: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: Default::default(),
        };

        // Test create
        let created = repo.create(twin.clone()).await.unwrap();
        assert_eq!(created.id, twin.id);

        // Test get
        let retrieved = repo.get_by_id(twin.id).await.unwrap();
        assert_eq!(retrieved.name, twin.name);

        // Test update state
        repo.update_state(twin.id, TwinState::Inactive).await.unwrap();
        let updated = repo.get_by_id(twin.id).await.unwrap();
        assert_eq!(updated.state, TwinState::Inactive);

        // Test delete
        repo.delete(twin.id).await.unwrap();
        assert!(repo.get_by_id(twin.id).await.is_err());
    }
}