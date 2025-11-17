use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use anyhow::Result;

use crate::core::domain::{
    models::{Tool, ToolId, ToolType, ToolResult, ExecutionId},
    traits::repository::{
        ToolRepository, RepositoryResult, RepositoryError,
        FilterCriteria, SortCriteria, Pagination, PaginatedResult,
    },
};

pub struct SqliteToolRepository {
    pool: Pool<Sqlite>,
}

impl SqliteToolRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ToolRepository for SqliteToolRepository {
    async fn create(&self, tool: Tool) -> RepositoryResult<Tool> {
        sqlx::query(
            "INSERT INTO tools (id, name, description, tool_type, config, enabled, metadata) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(tool.id.to_string())
        .bind(&tool.name)
        .bind(&tool.description)
        .bind(tool.tool_type.to_string())
        .bind(serde_json::to_string(&tool.config).unwrap_or("{}".to_string()))
        .bind(tool.enabled)
        .bind(serde_json::to_string(&tool.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(tool)
    }

    async fn get_by_id(&self, id: ToolId) -> RepositoryResult<Tool> {
        let tool = sqlx::query_as!(
            ToolRow,
            "SELECT id, name, description, tool_type, config, enabled, 
                    created_at, updated_at, metadata 
             FROM tools 
             WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "Tool".to_string(),
            id: id.to_string(),
        })?;

        Ok(tool.into())
    }

    async fn update(&self, tool: Tool) -> RepositoryResult<Tool> {
        sqlx::query(
            "UPDATE tools 
             SET name = ?, description = ?, tool_type = ?, config = ?, 
                 enabled = ?, updated_at = CURRENT_TIMESTAMP, metadata = ? 
             WHERE id = ?"
        )
        .bind(&tool.name)
        .bind(&tool.description)
        .bind(tool.tool_type.to_string())
        .bind(serde_json::to_string(&tool.config).unwrap_or("{}".to_string()))
        .bind(tool.enabled)
        .bind(serde_json::to_string(&tool.metadata).unwrap_or("{}".to_string()))
        .bind(tool.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(tool)
    }

    async fn delete(&self, id: ToolId) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM tools WHERE id = ?")
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
    ) -> RepositoryResult<PaginatedResult<Tool>> {
        let mut query = String::from(
            "SELECT id, name, description, tool_type, config, enabled, 
                    created_at, updated_at, metadata 
             FROM tools"
        );

        // Add filters
        if !filters.is_empty() {
            query.push_str(" WHERE ");
            for (i, filter) in filters.iter().enumerate() {
                if i > 0 {
                    query.push_str(" AND ");
                }
                match filter.operator {
                    FilterOperator::Equals => {
                        query.push_str(&format!("{} = ?", filter.field));
                    },
                    FilterOperator::Contains => {
                        query.push_str(&format!("{} LIKE ?", filter.field));
                    },
                    // Add other operators as needed
                    _ => {}
                }
            }
        }

        // Add sorting
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

        let tools = sqlx::query_as::<_, ToolRow>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tools")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: tools.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_by_type(
        &self,
        tool_type: &ToolType,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>> {
        let tools = sqlx::query_as!(
            ToolRow,
            "SELECT id, name, description, tool_type, config, enabled, 
                    created_at, updated_at, metadata 
             FROM tools 
             WHERE tool_type = ? 
             LIMIT ? OFFSET ?",
            tool_type.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tools WHERE tool_type = ?",
            tool_type.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: tools.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_available(
        &self,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>> {
        let tools = sqlx::query_as!(
            ToolRow,
            "SELECT id, name, description, tool_type, config, enabled, 
                    created_at, updated_at, metadata 
             FROM tools 
             WHERE enabled = true 
             LIMIT ? OFFSET ?",
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tools WHERE enabled = true"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: tools.into_iter().map(|t| t.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn save_execution_result(&self, result: ToolResult) -> RepositoryResult<ToolResult> {
        sqlx::query(
            "INSERT INTO tool_executions 
             (id, tool_id, status, parameters, result, started_at, completed_at, error, metrics) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(result.execution_id.to_string())
        .bind(result.tool_id.to_string())
        .bind(result.status.to_string())
        .bind(serde_json::to_string(&result.parameters).unwrap_or("{}".to_string()))
        .bind(serde_json::to_string(&result.output).ok())
        .bind(result.started_at)
        .bind(result.completed_at)
        .bind(result.error.as_ref().map(|e| e.to_string()))
        .bind(serde_json::to_string(&result.metrics).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_execution_by_id(&self, execution_id: ExecutionId) -> RepositoryResult<ToolResult> {
        let execution = sqlx::query_as!(
            ExecutionRow,
            "SELECT id, tool_id, status, parameters, result, started_at, 
                    completed_at, error, metrics 
             FROM tool_executions 
             WHERE id = ?",
            execution_id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "ToolExecution".to_string(),
            id: execution_id.to_string(),
        })?;

        Ok(execution.into())
    }

    async fn get_executions_by_tool_id(
        &self,
        tool_id: ToolId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<ToolResult>> {
        let executions = sqlx::query_as!(
            ExecutionRow,
            "SELECT id, tool_id, status, parameters, result, started_at, 
                    completed_at, error, metrics 
             FROM tool_executions 
             WHERE tool_id = ? 
             ORDER BY started_at DESC 
             LIMIT ? OFFSET ?",
            tool_id.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tool_executions WHERE tool_id = ?",
            tool_id.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: executions.into_iter().map(|e| e.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn update_usage_stats(
        &self,
        tool_id: ToolId,
        execution_time_ms: u64,
        success: bool,
    ) -> RepositoryResult<()> {
        // This could be enhanced to store more detailed usage statistics
        sqlx::query(
            "UPDATE tools 
             SET metadata = json_set(
                 COALESCE(metadata, '{}'),
                 '$.usage_stats',
                 json_object(
                     'last_execution', datetime('now'),
                     'total_executions', COALESCE(
                         json_extract(metadata, '$.usage_stats.total_executions'), 0
                     ) + 1,
                     'total_execution_time_ms', COALESCE(
                         json_extract(metadata, '$.usage_stats.total_execution_time_ms'), 0
                     ) + ?,
                     'successful_executions', COALESCE(
                         json_extract(metadata, '$.usage_stats.successful_executions'), 0
                     ) + ?
                 )
             )
             WHERE id = ?"
        )
        .bind(execution_time_ms as i64)
        .bind(if success { 1i64 } else { 0i64 })
        .bind(tool_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

// Database row structures
#[derive(sqlx::FromRow)]
struct ToolRow {
    id: String,
    name: String,
    description: String,
    tool_type: String,
    config: String,
    enabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    metadata: String,
}

#[derive(sqlx::FromRow)]
struct ExecutionRow {
    id: String,
    tool_id: String,
    status: String,
    parameters: String,
    result: Option<String>,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    error: Option<String>,
    metrics: String,
}

impl From<ToolRow> for Tool {
    fn from(row: ToolRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: row.name,
            description: row.description,
            tool_type: row.tool_type.parse().unwrap_or_default(),
            config: serde_json::from_str(&row.config).unwrap_or_default(),
            enabled: row.enabled,
            created_at: row.created_at,
            updated_at: row.updated_at,
            metadata: serde_json::from_str(&row.metadata).unwrap_or_default(),
        }
    }
}

impl From<ExecutionRow> for ToolResult {
    fn from(row: ExecutionRow) -> Self {
        Self {
            execution_id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            tool_id: Uuid::parse_str(&row.tool_id).unwrap(),
            status: row.status.parse().unwrap_or_default(),
            parameters: serde_json::from_str(&row.parameters).unwrap_or_default(),
            output: row.result.and_then(|r| serde_json::from_str(&r).ok()),
            started_at: row.started_at,
            completed_at: row.completed_at,
            error: row.error,
            metrics: serde_json::from_str(&row.metrics).unwrap_or_default(),
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
    async fn test_tool_crud() {
        let pool = create_test_db().await;
        let repo = SqliteToolRepository::new(pool);

        // Create test tool
        let tool = Tool {
            id: Uuid::new_v4(),
            name: "Test Tool".to_string(),
            description: "Test Description".to_string(),
            tool_type: ToolType::FileSystem,
            config: Default::default(),
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: Default::default(),
        };

        // Test create
        let created = repo.create(tool.clone()).await.unwrap();
        assert_eq!(created.id, tool.id);

        // Test get
        let retrieved = repo.get_by_id(tool.id).await.unwrap();
        assert_eq!(retrieved.name, tool.name);

        // Test update
        let mut updated = tool.clone();
        updated.enabled = false;
        repo.update(updated.clone()).await.unwrap();
        let retrieved = repo.get_by_id(tool.id).await.unwrap();
        assert_eq!(retrieved.enabled, false);

        // Test delete
        repo.delete(tool.id).await.unwrap();
        assert!(repo.get_by_id(tool.id).await.is_err());
    }
}