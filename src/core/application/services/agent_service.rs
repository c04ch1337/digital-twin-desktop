use crate::core::{
    domain::{
        errors::DomainError,
        models::agent::{Agent, AgentId, ModelConfig},
        models::tool::ToolId,
        traits::repository::{AgentRepository, ToolRepository},
    },
};
use std::sync::Arc;
use chrono::Utc;

/// Service to manage agent lifecycle and operations
pub struct AgentService {
    agent_repo: Arc<dyn AgentRepository>,
    tool_repo: Arc<dyn ToolRepository>,
}

impl AgentService {
    pub fn new(
        agent_repo: Arc<dyn AgentRepository>,
        tool_repo: Arc<dyn ToolRepository>,
    ) -> Self {
        Self {
            agent_repo,
            tool_repo,
        }
    }

    /// Create a new agent
    pub async fn create_agent(
        &self,
        name: String,
        instructions: String,
        model_config: ModelConfig,
        tool_ids: Vec<ToolId>,
    ) -> Result<Agent, DomainError> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Agent name cannot be empty".to_string(),
            ));
        }

        if instructions.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Agent instructions cannot be empty".to_string(),
            ));
        }

        // Validate tools exist
        for tool_id in &tool_ids {
            self.tool_repo
                .find_by_id(tool_id)
                .await
                .map_err(|e| DomainError::RepositoryError(e.to_string()))?
                .ok_or_else(|| DomainError::NotFound(
                    format!("Tool not found: {:?}", tool_id)
                ))?;
        }

        // Create agent
        let agent = Agent::new(
            AgentId::new(),
            name,
            instructions,
            model_config,
            tool_ids,
            Utc::now(),
            Utc::now(),
        );

        // Save to repository
        self.agent_repo
            .save(&agent)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(agent)
    }

    /// Get agent by ID
    pub async fn get_agent(&self, agent_id: &AgentId) -> Result<Option<Agent>, DomainError> {
        self.agent_repo
            .find_by_id(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Get all agents
    pub async fn list_agents(&self) -> Result<Vec<Agent>, DomainError> {
        self.agent_repo
            .find_all()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Update agent configuration
    pub async fn update_agent(
        &self,
        agent_id: AgentId,
        name: Option<String>,
        instructions: Option<String>,
        model_config: Option<ModelConfig>,
        tool_ids: Option<Vec<ToolId>>,
    ) -> Result<Agent, DomainError> {
        // Retrieve existing agent
        let mut agent = self
            .agent_repo
            .find_by_id(&agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        // Update fields if provided
        if let Some(name) = name {
            if name.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Agent name cannot be empty".to_string(),
                ));
            }
            agent.name = name;
        }

        if let Some(instructions) = instructions {
            if instructions.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Agent instructions cannot be empty".to_string(),
                ));
            }
            agent.instructions = instructions;
        }

        if let Some(model_config) = model_config {
            agent.model = model_config;
        }

        if let Some(tool_ids) = tool_ids {
            // Validate tools exist
            for tool_id in &tool_ids {
                self.tool_repo
                    .find_by_id(tool_id)
                    .await
                    .map_err(|e| DomainError::RepositoryError(e.to_string()))?
                    .ok_or_else(|| DomainError::NotFound(
                        format!("Tool not found: {:?}", tool_id)
                    ))?;
            }
            agent.tools = tool_ids;
        }

        agent.updated_at = Utc::now();

        // Update in repository
        self.agent_repo
            .update(&agent)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(agent)
    }

    /// Delete an agent
    pub async fn delete_agent(&self, agent_id: &AgentId) -> Result<(), DomainError> {
        // Verify agent exists
        self.agent_repo
            .find_by_id(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        // Delete from repository
        self.agent_repo
            .delete(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Add a tool to an agent
    pub async fn add_tool_to_agent(
        &self,
        agent_id: &AgentId,
        tool_id: ToolId,
    ) -> Result<Agent, DomainError> {
        // Retrieve agent
        let mut agent = self
            .agent_repo
            .find_by_id(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        // Verify tool exists
        self.tool_repo
            .find_by_id(&tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Tool not found".to_string()))?;

        // Check if agent already has the tool
        if agent.tools.contains(&tool_id) {
            return Err(DomainError::ValidationError(
                "Agent already has this tool".to_string(),
            ));
        }

        // Add tool
        agent.tools.push(tool_id);
        agent.updated_at = Utc::now();

        // Update in repository
        self.agent_repo
            .update(&agent)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(agent)
    }

    /// Remove a tool from an agent
    pub async fn remove_tool_from_agent(
        &self,
        agent_id: &AgentId,
        tool_id: &ToolId,
    ) -> Result<Agent, DomainError> {
        // Retrieve agent
        let mut agent = self
            .agent_repo
            .find_by_id(agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Agent not found".to_string()))?;

        // Remove tool if present
        let initial_count = agent.tools.len();
        agent.tools.retain(|id| id != tool_id);

        if agent.tools.len() == initial_count {
            return Err(DomainError::ValidationError(
                "Agent does not have this tool".to_string(),
            ));
        }

        agent.updated_at = Utc::now();

        // Update in repository
        self.agent_repo
            .update(&agent)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(agent)
    }

    /// Clone an existing agent with a new name
    pub async fn clone_agent(
        &self,
        source_agent_id: &AgentId,
        new_name: String,
    ) -> Result<Agent, DomainError> {
        // Retrieve source agent
        let source_agent = self
            .agent_repo
            .find_by_id(source_agent_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Source agent not found".to_string()))?;

        // Create new agent with copied configuration
        let new_agent = Agent::new(
            AgentId::new(),
            new_name,
            source_agent.instructions.clone(),
            source_agent.model.clone(),
            source_agent.tools.clone(),
            Utc::now(),
            Utc::now(),
        );

        // Save to repository
        self.agent_repo
            .save(&new_agent)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(new_agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::{
        models::tool::{Tool, ToolCategory},
    };
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockAgentRepository {
        agents: Arc<Mutex<Vec<Agent>>>,
    }

    #[async_trait]
    impl AgentRepository for MockAgentRepository {
        async fn save(&self, agent: &Agent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.agents.lock().unwrap().push(agent.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &AgentId) -> Result<Option<Agent>, Box<dyn std::error::Error + Send + Sync>> {
            let agents = self.agents.lock().unwrap();
            Ok(agents.iter().find(|a| a.id == *id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<Agent>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.agents.lock().unwrap().clone())
        }

        async fn update(&self, agent: &Agent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut agents = self.agents.lock().unwrap();
            if let Some(index) = agents.iter().position(|a| a.id == agent.id) {
                agents[index] = agent.clone();
            }
            Ok(())
        }

        async fn delete(&self, id: &AgentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut agents = self.agents.lock().unwrap();
            agents.retain(|a| a.id != *id);
            Ok(())
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

        async fn find_by_id(&self, id: &ToolId) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            let tools = self.tools.lock().unwrap();
            Ok(tools.iter().find(|t| t.id == *id).cloned())
        }

        async fn find_by_name(&self, _name: &str) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
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

    #[tokio::test]
    async fn test_create_agent() {
        let agents = Arc::new(Mutex::new(Vec::new()));
        let tools = Arc::new(Mutex::new(Vec::new()));
        
        let agent_repo = Arc::new(MockAgentRepository { agents: agents.clone() });
        let tool_repo = Arc::new(MockToolRepository { tools });
        
        let service = AgentService::new(agent_repo, tool_repo);
        
        let model_config = ModelConfig {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(1000),
        };
        
        let result = service.create_agent(
            "Test Agent".to_string(),
            "You are a helpful assistant".to_string(),
            model_config,
            vec![],
        ).await;
        
        assert!(result.is_ok());
        let agent = result.unwrap();
        assert_eq!(agent.name, "Test Agent");
        assert_eq!(agent.instructions, "You are a helpful assistant");
        
        let saved_agents = agents.lock().unwrap();
        assert_eq!(saved_agents.len(), 1);
    }

    #[tokio::test]
    async fn test_update_agent() {
        let agent_id = AgentId::new();
        let agent = Agent::new(
            agent_id.clone(),
            "Original Name".to_string(),
            "Original instructions".to_string(),
            ModelConfig {
                provider: "openai".to_string(),
                model: "gpt-3.5".to_string(),
                temperature: None,
                max_tokens: None,
            },
            vec![],
            Utc::now(),
            Utc::now(),
        );
        
        let agents = Arc::new(Mutex::new(vec![agent]));
        let tools = Arc::new(Mutex::new(Vec::new()));
        
        let agent_repo = Arc::new(MockAgentRepository { agents: agents.clone() });
        let tool_repo = Arc::new(MockToolRepository { tools });
        
        let service = AgentService::new(agent_repo, tool_repo);
        
        let result = service.update_agent(
            agent_id.clone(),
            Some("Updated Name".to_string()),
            None,
            None,
            None,
        ).await;
        
        assert!(result.is_ok());
        let updated_agent = result.unwrap();
        assert_eq!(updated_agent.name, "Updated Name");
        assert_eq!(updated_agent.instructions, "Original instructions");
    }
}