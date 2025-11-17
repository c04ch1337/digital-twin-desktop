use crate::core::domain::{
    errors::DomainError,
    models::conversation::{Conversation, ConversationId},
    models::agent::AgentId,
    traits::repository::ConversationRepository,
};
use std::sync::Arc;
use chrono::Utc;

/// Command to create a new conversation
#[derive(Debug, Clone)]
pub struct CreateConversationCommand {
    pub agent_id: AgentId,
    pub initial_context: Option<String>,
}

/// Use case for creating a new conversation with an agent
pub struct CreateConversationUseCase {
    conversation_repo: Arc<dyn ConversationRepository>,
}

impl CreateConversationUseCase {
    pub fn new(conversation_repo: Arc<dyn ConversationRepository>) -> Self {
        Self { conversation_repo }
    }

    pub async fn execute(
        &self,
        command: CreateConversationCommand,
    ) -> Result<Conversation, DomainError> {
        // Create new conversation
        let conversation = Conversation::new(
            ConversationId::new(),
            command.agent_id,
            vec![],
            command.initial_context,
            Utc::now(),
            Utc::now(),
        );

        // Save to repository
        self.conversation_repo
            .save(&conversation)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(conversation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockConversationRepository {
        saved_conversations: Arc<Mutex<Vec<Conversation>>>,
    }

    #[async_trait]
    impl ConversationRepository for MockConversationRepository {
        async fn save(&self, conversation: &Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.saved_conversations
                .lock()
                .unwrap()
                .push(conversation.clone());
            Ok(())
        }

        async fn find_by_id(&self, _id: &ConversationId) -> Result<Option<Conversation>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_agent(&self, _agent_id: &AgentId) -> Result<Vec<Conversation>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, _conversation: &Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn delete(&self, _id: &ConversationId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_create_conversation() {
        let saved_conversations = Arc::new(Mutex::new(Vec::new()));
        let repo = Arc::new(MockConversationRepository {
            saved_conversations: saved_conversations.clone(),
        });
        let use_case = CreateConversationUseCase::new(repo);

        let command = CreateConversationCommand {
            agent_id: AgentId::new(),
            initial_context: Some("Test context".to_string()),
        };

        let result = use_case.execute(command.clone()).await;
        assert!(result.is_ok());

        let conversation = result.unwrap();
        assert_eq!(conversation.agent_id, command.agent_id);
        assert_eq!(conversation.context, command.initial_context);
        assert!(conversation.messages.is_empty());

        let saved = saved_conversations.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].id, conversation.id);
    }
}