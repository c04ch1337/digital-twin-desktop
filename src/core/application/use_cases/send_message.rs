use crate::core::domain::{
    errors::DomainError,
    models::conversation::{Conversation, ConversationId, Message},
    models::agent::Agent,
    traits::repository::{ConversationRepository, AgentRepository},
    traits::llm_client::LLMClient,
};
use std::sync::Arc;
use chrono::Utc;

/// Command to send a message in a conversation
#[derive(Debug, Clone)]
pub struct SendMessageCommand {
    pub conversation_id: ConversationId,
    pub content: String,
}

/// Response from sending a message
#[derive(Debug, Clone)]
pub struct SendMessageResponse {
    pub conversation: Conversation,
    pub agent_response: Message,
}

/// Use case for sending a message to an agent
pub struct SendMessageUseCase {
    conversation_repo: Arc<dyn ConversationRepository>,
    agent_repo: Arc<dyn AgentRepository>,
    llm_client: Arc<dyn LLMClient>,
}

impl SendMessageUseCase {
    pub fn new(
        conversation_repo: Arc<dyn ConversationRepository>,
        agent_repo: Arc<dyn AgentRepository>,
        llm_client: Arc<dyn LLMClient>,
    ) -> Self {
        Self {
            conversation_repo,
            agent_repo,
            llm_client,
        }
    }

    pub async fn execute(
        &self,
        command: SendMessageCommand,
    ) -> Result<SendMessageResponse, DomainError> {
        // Retrieve conversation
        let mut conversation = self
            .conversation_repo
            .find_by_id(&command.conversation_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Conversation not found".to_string()))?;

        // Retrieve agent
        let agent = self
            .agent_repo
            .find_by_id(&conversation.agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        // Create user message
        let user_message = Message {
            role: "user".to_string(),
            content: command.content.clone(),
            timestamp: Utc::now(),
            metadata: None,
        };

        // Add user message to conversation
        conversation.messages.push(user_message.clone());

        // Prepare messages for LLM
        let mut llm_messages = vec![];
        
        // Add system message from agent instructions
        if !agent.instructions.is_empty() {
            llm_messages.push(Message {
                role: "system".to_string(),
                content: agent.instructions.clone(),
                timestamp: Utc::now(),
                metadata: None,
            });
        }

        // Add context if available
        if let Some(context) = &conversation.context {
            llm_messages.push(Message {
                role: "system".to_string(),
                content: format!("Context: {}", context),
                timestamp: Utc::now(),
                metadata: None,
            });
        }

        // Add conversation history
        llm_messages.extend(conversation.messages.clone());

        // Get response from LLM
        let llm_response = self
            .llm_client
            .complete(&llm_messages, &agent.model)
            .await
            .map_err(|e| DomainError::ExternalServiceError(e.to_string()))?;

        // Create assistant message
        let assistant_message = Message {
            role: "assistant".to_string(),
            content: llm_response,
            timestamp: Utc::now(),
            metadata: None,
        };

        // Add assistant message to conversation
        conversation.messages.push(assistant_message.clone());
        conversation.updated_at = Utc::now();

        // Save updated conversation
        self.conversation_repo
            .update(&conversation)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(SendMessageResponse {
            conversation,
            agent_response: assistant_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::models::agent::{AgentId, ModelConfig};
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockConversationRepository {
        conversations: Arc<Mutex<Vec<Conversation>>>,
    }

    #[async_trait]
    impl ConversationRepository for MockConversationRepository {
        async fn save(&self, _conversation: &Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_id(&self, id: &ConversationId) -> Result<Option<Conversation>, Box<dyn std::error::Error + Send + Sync>> {
            let conversations = self.conversations.lock().unwrap();
            Ok(conversations.iter().find(|c| c.id == *id).cloned())
        }

        async fn find_by_agent(&self, _agent_id: &AgentId) -> Result<Vec<Conversation>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, conversation: &Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut conversations = self.conversations.lock().unwrap();
            if let Some(index) = conversations.iter().position(|c| c.id == conversation.id) {
                conversations[index] = conversation.clone();
            }
            Ok(())
        }

        async fn delete(&self, _id: &ConversationId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    struct MockAgentRepository {
        agents: Arc<Mutex<Vec<Agent>>>,
    }

    #[async_trait]
    impl AgentRepository for MockAgentRepository {
        async fn save(&self, _agent: &Agent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_id(&self, id: &AgentId) -> Result<Option<Agent>, Box<dyn std::error::Error + Send + Sync>> {
            let agents = self.agents.lock().unwrap();
            Ok(agents.iter().find(|a| a.id == *id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<Agent>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, _agent: &Agent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn delete(&self, _id: &AgentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    struct MockLLMClient {}

    #[async_trait]
    impl LLMClient for MockLLMClient {
        async fn complete(
            &self,
            _messages: &[Message],
            _model: &ModelConfig,
        ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok("Test response from LLM".to_string())
        }
    }

    #[tokio::test]
    async fn test_send_message() {
        let conversation_id = ConversationId::new();
        let agent_id = AgentId::new();
        
        let conversation = Conversation::new(
            conversation_id.clone(),
            agent_id.clone(),
            vec![],
            Some("Test conversation context".to_string()),
            Utc::now(),
            Utc::now(),
        );
        
        let agent = Agent::new(
            agent_id,
            "Test Agent".to_string(),
            "You are a helpful assistant".to_string(),
            ModelConfig {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                temperature: Some(0.7),
                max_tokens: Some(1000),
            },
            vec![],
            Utc::now(),
            Utc::now(),
        );
        
        let conversations = Arc::new(Mutex::new(vec![conversation]));
        let agents = Arc::new(Mutex::new(vec![agent]));
        
        let conversation_repo = Arc::new(MockConversationRepository {
            conversations: conversations.clone(),
        });
        let agent_repo = Arc::new(MockAgentRepository {
            agents: agents.clone(),
        });
        let llm_client = Arc::new(MockLLMClient {});
        
        let use_case = SendMessageUseCase::new(conversation_repo, agent_repo, llm_client);
        
        let command = SendMessageCommand {
            conversation_id,
            content: "Hello, assistant!".to_string(),
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.conversation.messages.len(), 2);
        assert_eq!(response.conversation.messages[0].content, "Hello, assistant!");
        assert_eq!(response.agent_response.content, "Test response from LLM");
    }
}