use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use anyhow::Result;
use serde_json::Value;

use crate::core::domain::{
    models::{Conversation, ConversationId, ConversationState, Message, MessageId},
    traits::repository::{
        ConversationRepository, RepositoryResult, RepositoryError,
        FilterCriteria, SortCriteria, Pagination, PaginatedResult,
    },
};

pub struct SqliteConversationRepository {
    pool: Pool<Sqlite>,
}

impl SqliteConversationRepository {
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
impl ConversationRepository for SqliteConversationRepository {
    async fn create(&self, conversation: Conversation) -> RepositoryResult<Conversation> {
        let mut tx = self.pool.begin().await.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Insert conversation with agent_id
        sqlx::query(
            "INSERT INTO conversations (id, title, state, agent_id, metadata) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(conversation.id.to_string())
        .bind(&conversation.title)
        .bind(conversation.state.to_string())
        .bind(conversation.participant_agent_ids.first().map(|id| id.to_string()))
        .bind(serde_json::to_string(&conversation.metadata).unwrap_or("{}".to_string()))
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Insert messages if any
        for message in &conversation.messages {
            let agent_id = match &message.sender {
                crate::core::domain::models::conversation::MessageSender::Agent { agent_id } => Some(agent_id.to_string()),
                _ => None,
            };

            sqlx::query(
                "INSERT INTO messages (id, conversation_id, role, content, agent_id, token_count, model, content_type, metadata)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(message.id.to_string())
            .bind(conversation.id.to_string())
            .bind(&message.role)
            .bind(&message.content)
            .bind(agent_id)
            .bind(message.metadata.token_count)
            .bind(&message.metadata.model)
            .bind(format!("{:?}", message.content_type))
            .bind(serde_json::to_string(&message.metadata).unwrap_or("{}".to_string()))
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(conversation)
    }

    async fn get_by_id(&self, id: ConversationId) -> RepositoryResult<Conversation> {
        let conversation = sqlx::query_as!(
            ConversationRow,
            "SELECT id, title, state, agent_id, created_at, updated_at, metadata FROM conversations WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "Conversation".to_string(),
            id: id.to_string(),
        })?;

        let messages = sqlx::query_as!(
            MessageRow,
            "SELECT id, conversation_id, role, content, agent_id, token_count, model, content_type, created_at, metadata
             FROM messages WHERE conversation_id = ? ORDER BY created_at",
            id.to_string()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(Conversation {
            id,
            title: conversation.title,
            state: conversation.state.parse().unwrap_or(ConversationState::Active),
            messages: messages.into_iter().map(|m| m.into()).collect(),
            created_at: conversation.created_at,
            updated_at: conversation.updated_at,
            metadata: serde_json::from_str(&conversation.metadata).unwrap_or_default(),
        })
    }

    async fn update(&self, conversation: Conversation) -> RepositoryResult<Conversation> {
        let mut tx = self.pool.begin().await.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query(
            "UPDATE conversations SET title = ?, state = ?, updated_at = CURRENT_TIMESTAMP, metadata = ? 
             WHERE id = ?"
        )
        .bind(&conversation.title)
        .bind(conversation.state.to_string())
        .bind(serde_json::to_string(&conversation.metadata).unwrap_or("{}".to_string()))
        .bind(conversation.id.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        tx.commit().await.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(conversation)
    }

    async fn delete(&self, id: ConversationId) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM conversations WHERE id = ?")
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
    ) -> RepositoryResult<PaginatedResult<Conversation>> {
        let mut query = String::from("SELECT id, title, state, created_at, updated_at, metadata FROM conversations");
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

        let conversations = sqlx::query_as::<_, ConversationRow>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM conversations")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: conversations
                .into_iter()
                .map(|c| Conversation {
                    id: Uuid::parse_str(&c.id).unwrap(),
                    title: c.title,
                    state: c.state.parse().unwrap_or(ConversationState::Active),
                    messages: Vec::new(), // Messages loaded on demand
                    created_at: c.created_at,
                    updated_at: c.updated_at,
                    metadata: serde_json::from_str(&c.metadata).unwrap_or_default(),
                })
                .collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn add_message(
        &self,
        conversation_id: ConversationId,
        message: Message,
    ) -> RepositoryResult<Message> {
        let agent_id = match &message.sender {
            crate::core::domain::models::conversation::MessageSender::Agent { agent_id } => Some(agent_id.to_string()),
            _ => None,
        };

        sqlx::query(
            "INSERT INTO messages (id, conversation_id, role, content, agent_id, token_count, model, content_type, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(message.id.to_string())
        .bind(conversation_id.to_string())
        .bind(&message.role)
        .bind(&message.content)
        .bind(agent_id)
        .bind(message.metadata.token_count)
        .bind(&message.metadata.model)
        .bind(format!("{:?}", message.content_type))
        .bind(serde_json::to_string(&message.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn get_messages(
        &self,
        conversation_id: ConversationId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Message>> {
        let messages = sqlx::query_as!(
            MessageRow,
            "SELECT id, conversation_id, role, content, agent_id, token_count, model, content_type, created_at, metadata
             FROM messages
             WHERE conversation_id = ?
             ORDER BY created_at
             LIMIT ? OFFSET ?",
            conversation_id.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM messages WHERE conversation_id = ?",
            conversation_id.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: messages.into_iter().map(|m| m.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }
}

// Database row structures
#[derive(sqlx::FromRow)]
struct ConversationRow {
    id: String,
    title: String,
    state: String,
    agent_id: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    metadata: String,
}

#[derive(sqlx::FromRow)]
struct MessageRow {
    id: String,
    conversation_id: String,
    role: String,
    content: String,
    agent_id: Option<String>,
    token_count: Option<i32>,
    model: Option<String>,
    content_type: Option<String>,
    created_at: DateTime<Utc>,
    metadata: String,
}

impl From<MessageRow> for Message {
    fn from(row: MessageRow) -> Self {
        Message {
            id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            role: row.role,
            content: row.content,
            created_at: row.created_at,
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
    async fn test_conversation_crud() {
        let pool = create_test_db().await;
        let repo = SqliteConversationRepository::new(pool);

        // Create test conversation
        let conv = Conversation {
            id: Uuid::new_v4(),
            title: "Test Conversation".to_string(),
            state: ConversationState::Active,
            messages: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: Default::default(),
        };

        // Test create
        let created = repo.create(conv.clone()).await.unwrap();
        assert_eq!(created.id, conv.id);

        // Test get
        let retrieved = repo.get_by_id(conv.id).await.unwrap();
        assert_eq!(retrieved.title, conv.title);

        // Test delete
        repo.delete(conv.id).await.unwrap();
        assert!(repo.get_by_id(conv.id).await.is_err());
    }
}