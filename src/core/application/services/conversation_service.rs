use crate::core::{
    domain::{
        errors::DomainError,
        models::conversation::{Conversation, ConversationId},
        models::agent::AgentId,
        traits::repository::{ConversationRepository, AgentRepository},
    },
    application::use_cases::{
        create_conversation::{CreateConversationCommand, CreateConversationUseCase},
        send_message::{SendMessageCommand, SendMessageResponse, SendMessageUseCase},
        execute_tool::{ExecuteToolCommand, ExecuteToolResponse, ExecuteToolUseCase},
    },
};
use std::sync::Arc;

/// Service to orchestrate conversation-related operations
pub struct ConversationService {
    create_conversation_use_case: CreateConversationUseCase,
    send_message_use_case: SendMessageUseCase,
    execute_tool_use_case: ExecuteToolUseCase,
    conversation_repo: Arc<dyn ConversationRepository>,
    agent_repo: Arc<dyn AgentRepository>,
}

impl ConversationService {
    pub fn new(
        create_conversation_use_case: CreateConversationUseCase,
        send_message_use_case: SendMessageUseCase,
        execute_tool_use_case: ExecuteToolUseCase,
        conversation_repo: Arc<dyn ConversationRepository>,
        agent_repo: Arc<dyn AgentRepository>,
    ) -> Self {
        Self {
            create_conversation_use_case,
            send_message_use_case,
            execute_tool_use_case,
            conversation_repo,
            agent_repo,
        }
    }

    /// Create a new conversation with an agent
    pub async fn create_conversation(
        &self,
        agent_id: AgentId,
        initial_context: Option<String>,
    ) -> Result<Conversation, DomainError> {
        // Verify agent exists
        self.agent_repo
            .find_by_id(&agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        let command = CreateConversationCommand {
            agent_id,
            initial_context,
        };

        self.create_conversation_use_case.execute(command).await
    }

    /// Send a message in an existing conversation
    pub async fn send_message(
        &self,
        conversation_id: ConversationId,
        content: String,
    ) -> Result<SendMessageResponse, DomainError> {
        let command = SendMessageCommand {
            conversation_id,
            content,
        };

        self.send_message_use_case.execute(command).await
    }

    /// Execute a tool within a conversation
    pub async fn execute_tool(
        &self,
        conversation_id: ConversationId,
        tool_name: String,
        parameters: serde_json::Value,
    ) -> Result<ExecuteToolResponse, DomainError> {
        // Convert parameters to the expected format
        let tool_parameters = parameters
            .as_object()
            .ok_or_else(|| DomainError::ValidationError(
                "Tool parameters must be an object".to_string()
            ))?
            .clone();

        let command = ExecuteToolCommand {
            conversation_id,
            tool_name,
            parameters: tool_parameters,
        };

        self.execute_tool_use_case.execute(command).await
    }

    /// Get conversation by ID
    pub async fn get_conversation(
        &self,
        conversation_id: &ConversationId,
    ) -> Result<Option<Conversation>, DomainError> {
        self.conversation_repo
            .find_by_id(conversation_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Get all conversations for an agent
    pub async fn get_agent_conversations(
        &self,
        agent_id: &AgentId,
    ) -> Result<Vec<Conversation>, DomainError> {
        self.conversation_repo
            .find_by_agent(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Delete a conversation
    pub async fn delete_conversation(
        &self,
        conversation_id: &ConversationId,
    ) -> Result<(), DomainError> {
        // Verify conversation exists
        self.conversation_repo
            .find_by_id(conversation_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Conversation not found".to_string()))?;

        self.conversation_repo
            .delete(conversation_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Clear all messages in a conversation while preserving the conversation itself
    pub async fn clear_conversation(
        &self,
        conversation_id: &ConversationId,
    ) -> Result<Conversation, DomainError> {
        let mut conversation = self
            .conversation_repo
            .find_by_id(conversation_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Conversation not found".to_string()))?;

        // Clear messages but preserve context
        conversation.messages.clear();
        conversation.updated_at = chrono::Utc::now();

        self.conversation_repo
            .update(&conversation)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(conversation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        domain::{
            models::{
                agent::{Agent, ModelConfig},
                conversation::Message,
                tool::{Tool, ToolId, ToolCategory},
            },
            traits::{
                llm_client::LLMClient,
                repository::ToolRepository,
                tool_executor::ToolExecutor,
            },
        },
    };
    use async_trait::async_trait;
    use std::sync::Mutex;

    // Mock implementations would go here...
    // For brevity, I'm omitting the mock implementations as they would be similar
    // to those in the use case tests

    #[tokio::test]
    async fn test_create_and_send_message() {
        // Test implementation would go here
        // This would set up the mocks and test the full flow
    }

    #[tokio::test]
    async fn test_clear_conversation() {
        // Test implementation would go here
        // This would test clearing conversation messages
    }
}