use crate::core::domain::{
    errors::DomainError,
    models::conversation::{Conversation, ConversationId, Message},
    models::agent::Agent,
    models::tool::{Tool, ToolResult, ToolParameters},
    traits::repository::{ConversationRepository, AgentRepository, ToolRepository},
    traits::tool_executor::ToolExecutor,
};
use std::sync::Arc;
use chrono::Utc;
use serde_json::Value;

/// Command to execute a tool within a conversation
#[derive(Debug, Clone)]
pub struct ExecuteToolCommand {
    pub conversation_id: ConversationId,
    pub tool_name: String,
    pub parameters: ToolParameters,
}

/// Response from tool execution
#[derive(Debug, Clone)]
pub struct ExecuteToolResponse {
    pub conversation: Conversation,
    pub tool_result: ToolResult,
    pub execution_message: Message,
}

/// Use case for executing tools through agents
pub struct ExecuteToolUseCase {
    conversation_repo: Arc<dyn ConversationRepository>,
    agent_repo: Arc<dyn AgentRepository>,
    tool_repo: Arc<dyn ToolRepository>,
    tool_executor: Arc<dyn ToolExecutor>,
}

impl ExecuteToolUseCase {
    pub fn new(
        conversation_repo: Arc<dyn ConversationRepository>,
        agent_repo: Arc<dyn AgentRepository>,
        tool_repo: Arc<dyn ToolRepository>,
        tool_executor: Arc<dyn ToolExecutor>,
    ) -> Self {
        Self {
            conversation_repo,
            agent_repo,
            tool_repo,
            tool_executor,
        }
    }

    pub async fn execute(
        &self,
        command: ExecuteToolCommand,
    ) -> Result<ExecuteToolResponse, DomainError> {
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

        // Check if agent has the tool
        if !agent.tools.iter().any(|tool_id| tool_id.value == command.tool_name) {
            return Err(DomainError::ValidationError(
                format!("Agent does not have access to tool: {}", command.tool_name)
            ));
        }

        // Retrieve tool definition
        let tool = self
            .tool_repo
            .find_by_name(&command.tool_name)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(
                format!("Tool not found: {}", command.tool_name)
            ))?;

        // Validate parameters
        self.validate_tool_parameters(&tool, &command.parameters)?;

        // Add tool invocation message to conversation
        let invocation_message = Message {
            role: "system".to_string(),
            content: format!(
                "Executing tool '{}' with parameters: {}",
                command.tool_name,
                serde_json::to_string(&command.parameters).unwrap_or_default()
            ),
            timestamp: Utc::now(),
            metadata: Some(serde_json::json!({
                "tool_name": command.tool_name,
                "tool_type": "invocation"
            })),
        };
        conversation.messages.push(invocation_message);

        // Execute the tool
        let tool_result = self
            .tool_executor
            .execute(&tool, &command.parameters)
            .await
            .map_err(|e| DomainError::ExternalServiceError(
                format!("Tool execution failed: {}", e)
            ))?;

        // Create execution result message
        let execution_message = Message {
            role: "tool".to_string(),
            content: match &tool_result.output {
                Value::String(s) => s.clone(),
                _ => serde_json::to_string_pretty(&tool_result.output).unwrap_or_default(),
            },
            timestamp: Utc::now(),
            metadata: Some(serde_json::json!({
                "tool_name": command.tool_name,
                "tool_type": "result",
                "success": tool_result.success,
                "execution_time_ms": tool_result.execution_time.as_millis()
            })),
        };

        // Add result message to conversation
        conversation.messages.push(execution_message.clone());
        conversation.updated_at = Utc::now();

        // Save updated conversation
        self.conversation_repo
            .update(&conversation)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(ExecuteToolResponse {
            conversation,
            tool_result,
            execution_message,
        })
    }

    fn validate_tool_parameters(
        &self,
        tool: &Tool,
        parameters: &ToolParameters,
    ) -> Result<(), DomainError> {
        // Check required parameters
        for required_param in &tool.required_parameters {
            if !parameters.contains_key(required_param) {
                return Err(DomainError::ValidationError(
                    format!("Missing required parameter: {}", required_param)
                ));
            }
        }

        // Validate parameter types
        for (param_name, param_value) in parameters {
            if let Some(expected_type) = tool.parameter_types.get(param_name) {
                match (expected_type.as_str(), param_value) {
                    ("string", Value::String(_)) => {},
                    ("number", Value::Number(_)) => {},
                    ("boolean", Value::Bool(_)) => {},
                    ("object", Value::Object(_)) => {},
                    ("array", Value::Array(_)) => {},
                    _ => {
                        return Err(DomainError::ValidationError(
                            format!(
                                "Invalid type for parameter '{}'. Expected: {}, Got: {}",
                                param_name,
                                expected_type,
                                match param_value {
                                    Value::String(_) => "string",
                                    Value::Number(_) => "number",
                                    Value::Bool(_) => "boolean",
                                    Value::Object(_) => "object",
                                    Value::Array(_) => "array",
                                    Value::Null => "null",
                                }
                            )
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::models::agent::{AgentId, ModelConfig};
    use crate::core::domain::models::tool::{ToolId, ToolCategory};
    use async_trait::async_trait;
    use std::sync::Mutex;
    use std::time::Duration;

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

    struct MockToolRepository {
        tools: Arc<Mutex<Vec<Tool>>>,
    }

    #[async_trait]
    impl ToolRepository for MockToolRepository {
        async fn save(&self, _tool: &Tool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_id(&self, _id: &ToolId) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_name(&self, name: &str) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            let tools = self.tools.lock().unwrap();
            Ok(tools.iter().find(|t| t.name == name).cloned())
        }

        async fn find_by_category(&self, _category: &ToolCategory) -> Result<Vec<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_all(&self) -> Result<Vec<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, _tool: &Tool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn delete(&self, _id: &ToolId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    struct MockToolExecutor {}

    #[async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(
            &self,
            tool: &Tool,
            parameters: &ToolParameters,
        ) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>> {
            // Simple mock execution
            let output = match tool.name.as_str() {
                "calculator" => {
                    let a = parameters.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let b = parameters.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let op = parameters.get("operation").and_then(|v| v.as_str()).unwrap_or("add");
                    
                    let result = match op {
                        "add" => a + b,
                        "subtract" => a - b,
                        "multiply" => a * b,
                        "divide" => if b != 0.0 { a / b } else { 0.0 },
                        _ => 0.0,
                    };
                    
                    serde_json::json!({ "result": result })
                }
                _ => serde_json::json!({ "message": "Tool executed successfully" }),
            };

            Ok(ToolResult {
                success: true,
                output,
                error: None,
                execution_time: Duration::from_millis(100),
            })
        }

        async fn validate(
            &self,
            _tool: &Tool,
        ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_execute_tool_success() {
        let conversation_id = ConversationId::new();
        let agent_id = AgentId::new();
        let tool_id = ToolId::new();
        
        let conversation = Conversation::new(
            conversation_id.clone(),
            agent_id.clone(),
            vec![],
            None,
            Utc::now(),
            Utc::now(),
        );
        
        let agent = Agent::new(
            agent_id,
            "Test Agent".to_string(),
            "Test instructions".to_string(),
            ModelConfig {
                provider: "test".to_string(),
                model: "test-model".to_string(),
                temperature: None,
                max_tokens: None,
            },
            vec![tool_id.clone()],
            Utc::now(),
            Utc::now(),
        );
        
        let mut parameter_types = std::collections::HashMap::new();
        parameter_types.insert("a".to_string(), "number".to_string());
        parameter_types.insert("b".to_string(), "number".to_string());
        parameter_types.insert("operation".to_string(), "string".to_string());
        
        let tool = Tool::new(
            tool_id,
            "calculator".to_string(),
            "A simple calculator tool".to_string(),
            ToolCategory::Utility,
            vec!["a".to_string(), "b".to_string(), "operation".to_string()],
            parameter_types,
            serde_json::json!({
                "endpoints": {
                    "execute": "/tools/calculator/execute"
                }
            }),
            true,
            Utc::now(),
            Utc::now(),
        );
        
        let conversations = Arc::new(Mutex::new(vec![conversation]));
        let agents = Arc::new(Mutex::new(vec![agent]));
        let tools = Arc::new(Mutex::new(vec![tool]));
        
        let conversation_repo = Arc::new(MockConversationRepository {
            conversations: conversations.clone(),
        });
        let agent_repo = Arc::new(MockAgentRepository {
            agents: agents.clone(),
        });
        let tool_repo = Arc::new(MockToolRepository {
            tools: tools.clone(),
        });
        let tool_executor = Arc::new(MockToolExecutor {});
        
        let use_case = ExecuteToolUseCase::new(
            conversation_repo,
            agent_repo,
            tool_repo,
            tool_executor,
        );
        
        let mut parameters = ToolParameters::new();
        parameters.insert("a".to_string(), serde_json::json!(10.0));
        parameters.insert("b".to_string(), serde_json::json!(5.0));
        parameters.insert("operation".to_string(), serde_json::json!("add"));
        
        let command = ExecuteToolCommand {
            conversation_id,
            tool_name: "calculator".to_string(),
            parameters,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.tool_result.success);
        assert_eq!(
            response.tool_result.output.get("result").and_then(|v| v.as_f64()),
            Some(15.0)
        );
        assert_eq!(response.conversation.messages.len(), 2); // invocation + result
    }

    #[tokio::test]
    async fn test_execute_tool_missing_parameter() {
        let conversation_id = ConversationId::new();
        let agent_id = AgentId::new();
        let tool_id = ToolId::new();
        
        let conversation = Conversation::new(
            conversation_id.clone(),
            agent_id.clone(),
            vec![],
            None,
            Utc::now(),
            Utc::now(),
        );
        
        let agent = Agent::new(
            agent_id,
            "Test Agent".to_string(),
            "Test instructions".to_string(),
            ModelConfig {
                provider: "test".to_string(),
                model: "test-model".to_string(),
                temperature: None,
                max_tokens: None,
            },
            vec![tool_id.clone()],
            Utc::now(),
            Utc::now(),
        );
        
        let tool = Tool::new(
            tool_id,
            "test_tool".to_string(),
            "Test tool".to_string(),
            ToolCategory::Utility,
            vec!["required_param".to_string()],
            std::collections::HashMap::new(),
            serde_json::json!({}),
            true,
            Utc::now(),
            Utc::now(),
        );
        
        let conversations = Arc::new(Mutex::new(vec![conversation]));
        let agents = Arc::new(Mutex::new(vec![agent]));
        let tools = Arc::new(Mutex::new(vec![tool]));
        
        let conversation_repo = Arc::new(MockConversationRepository {
            conversations: conversations.clone(),
        });
        let agent_repo = Arc::new(MockAgentRepository {
            agents: agents.clone(),
        });
        let tool_repo = Arc::new(MockToolRepository {
            tools: tools.clone(),
        });
        let tool_executor = Arc::new(MockToolExecutor {});
        
        let use_case = ExecuteToolUseCase::new(
            conversation_repo,
            agent_repo,
            tool_repo,
            tool_executor,
        );
        
        let parameters = ToolParameters::new(); // Missing required parameter
        
        let command = ExecuteToolCommand {
            conversation_id,
            tool_name: "test_tool".to_string(),
            parameters,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::ValidationError(_)
        ));
    }
}