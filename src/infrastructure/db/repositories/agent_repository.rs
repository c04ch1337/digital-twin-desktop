use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use anyhow::Result;
use serde_json::Value;

use crate::core::domain::{
    models::{Agent, AgentId, AgentState},
    traits::repository::{
        AgentRepository, RepositoryResult, RepositoryError,
        FilterCriteria, SortCriteria, Pagination, PaginatedResult,
    },
};

pub struct SqliteAgentRepository {
    pool: Pool<Sqlite>,
}

impl SqliteAgentRepository {
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
impl AgentRepository for SqliteAgentRepository {
    async fn create(&self, agent: Agent) -> RepositoryResult<Agent> {
        sqlx::query(
            "INSERT INTO agents (id, name, description, state, capabilities, system_prompt, instructions, prompt_version, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(agent.id.to_string())
        .bind(&agent.name)
        .bind(&agent.description)
        .bind(agent.state.to_string())
        .bind(serde_json::to_string(&agent.capabilities).unwrap_or("[]".to_string()))
        .bind(&agent.system_prompt)
        .bind(&agent.system_prompt) // instructions field
        .bind("1.0.0") // prompt_version
        .bind(serde_json::to_string(&agent.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(agent)
    }

    async fn get_by_id(&self, id: AgentId) -> RepositoryResult<Agent> {
        let agent = sqlx::query_as!(
            AgentRow,
            "SELECT id, name, description, state, capabilities, system_prompt, instructions, prompt_version, created_at, updated_at, metadata
             FROM agents WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "Agent".to_string(),
            id: id.to_string(),
        })?;

        Ok(Agent {
            id,
            name: agent.name,
            description: agent.description,
            state: agent.state.parse().unwrap_or(AgentState::Inactive),
            capabilities: serde_json::from_str(&agent.capabilities).unwrap_or_default(),
            created_at: agent.created_at,
            updated_at: agent.updated_at,
            metadata: serde_json::from_str(&agent.metadata).unwrap_or_default(),
        })
    }

    async fn update(&self, agent: Agent) -> RepositoryResult<Agent> {
        sqlx::query(
            "UPDATE agents
             SET name = ?, description = ?, state = ?, capabilities = ?,
                 system_prompt = ?, instructions = ?, prompt_version = ?,
                 updated_at = CURRENT_TIMESTAMP, metadata = ?
             WHERE id = ?"
        )
        .bind(&agent.name)
        .bind(&agent.description)
        .bind(agent.state.to_string())
        .bind(serde_json::to_string(&agent.capabilities).unwrap_or("[]".to_string()))
        .bind(&agent.system_prompt)
        .bind(&agent.system_prompt) // instructions field
        .bind("1.0.0") // prompt_version
        .bind(serde_json::to_string(&agent.metadata).unwrap_or("{}".to_string()))
        .bind(agent.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(agent)
    }

    async fn delete(&self, id: AgentId) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM agents WHERE id = ?")
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
    ) -> RepositoryResult<PaginatedResult<Agent>> {
        let mut query = String::from(
            "SELECT id, name, description, state, capabilities, system_prompt, instructions, prompt_version, created_at, updated_at, metadata
             FROM agents"
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

        let agents = sqlx::query_as::<_, AgentRow>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agents")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: agents
                .into_iter()
                .map(|a| Agent {
                    id: Uuid::parse_str(&a.id).unwrap(),
                    name: a.name,
                    description: a.description,
                    state: a.state.parse().unwrap_or(AgentState::Inactive),
                    capabilities: serde_json::from_str(&a.capabilities).unwrap_or_default(),
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                    metadata: serde_json::from_str(&a.metadata).unwrap_or_default(),
                })
                .collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_state(
        &self,
        state: AgentState,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>> {
        let agents = sqlx::query_as!(
            AgentRow,
            "SELECT id, name, description, state, capabilities, system_prompt, instructions, prompt_version, created_at, updated_at, metadata
             FROM agents
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
            "SELECT COUNT(*) FROM agents WHERE state = ?",
            state.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: agents
                .into_iter()
                .map(|a| Agent {
                    id: Uuid::parse_str(&a.id).unwrap(),
                    name: a.name,
                    description: a.description,
                    state: a.state.parse().unwrap_or(AgentState::Inactive),
                    capabilities: serde_json::from_str(&a.capabilities).unwrap_or_default(),
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                    metadata: serde_json::from_str(&a.metadata).unwrap_or_default(),
                })
                .collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_capability(
        &self,
        capability_type: &str,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>> {
        let agents = sqlx::query_as!(
            AgentRow,
            "SELECT id, name, description, state, capabilities, system_prompt, instructions, prompt_version, created_at, updated_at, metadata
             FROM agents
             WHERE json_array_contains(capabilities, ?)
             LIMIT ? OFFSET ?",
            capability_type,
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM agents WHERE json_array_contains(capabilities, ?)",
            capability_type
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: agents
                .into_iter()
                .map(|a| Agent {
                    id: Uuid::parse_str(&a.id).unwrap(),
                    name: a.name,
                    description: a.description,
                    state: a.state.parse().unwrap_or(AgentState::Inactive),
                    capabilities: serde_json::from_str(&a.capabilities).unwrap_or_default(),
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                    metadata: serde_json::from_str(&a.metadata).unwrap_or_default(),
                })
                .collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn update_state(&self, id: AgentId, state: AgentState) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE agents SET state = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(state.to_string())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

// Database row structure
#[derive(sqlx::FromRow)]
struct AgentRow {
    id: String,
    name: String,
    description: String,
    state: String,
    capabilities: String,
    system_prompt: Option<String>,
    instructions: Option<String>,
    prompt_version: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    metadata: String,
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
    async fn test_agent_crud() {
        let pool = create_test_db().await;
        let repo = SqliteAgentRepository::new(pool);

        // Create test agent
        let agent = Agent {
            id: Uuid::new_v4(),
            name: "Test Agent".to_string(),
            description: "Test Description".to_string(),
            state: AgentState::Active,
            capabilities: vec!["test".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: Default::default(),
        };

        // Test create
        let created = repo.create(agent.clone()).await.unwrap();
        assert_eq!(created.id, agent.id);

        // Test get
        let retrieved = repo.get_by_id(agent.id).await.unwrap();
        assert_eq!(retrieved.name, agent.name);

        // Test update state
        repo.update_state(agent.id, AgentState::Inactive).await.unwrap();
        let updated = repo.get_by_id(agent.id).await.unwrap();
        assert_eq!(updated.state, AgentState::Inactive);

        // Test delete
        repo.delete(agent.id).await.unwrap();
        assert!(repo.get_by_id(agent.id).await.is_err());
    }
}