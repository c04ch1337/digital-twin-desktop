//! Digital Twin Agent domain models.
//!
//! This module defines the core agent entities that represent AI-powered
//! digital twins capable of interacting with users and performing tasks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Represents a Digital Twin Agent with specific capabilities and personality.
///
/// An Agent is an AI-powered entity that can engage in conversations,
/// execute tools, and maintain state across interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier for the agent
    pub id: Uuid,
    
    /// Human-readable name of the agent
    pub name: String,
    
    /// Description of the agent's purpose and capabilities
    pub description: String,
    
    /// The agent's personality and behavioral instructions
    pub system_prompt: String,
    
    /// Current state of the agent
    pub state: AgentState,
    
    /// Capabilities this agent possesses
    pub capabilities: Vec<AgentCapability>,
    
    /// Configuration parameters for the agent
    pub configuration: AgentConfiguration,
    
    /// Memory and context management settings
    pub memory_config: MemoryConfiguration,
    
    /// Metadata for additional agent properties
    pub metadata: AgentMetadata,
    
    /// Timestamp when the agent was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the agent was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents the current state of an agent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
    /// Agent is active and ready to interact
    Active,
    
    /// Agent is temporarily inactive
    Inactive,
    
    /// Agent is processing a request
    Processing,
    
    /// Agent encountered an error
    Error,
    
    /// Agent is undergoing maintenance or updates
    Maintenance,
}

/// Defines a specific capability that an agent possesses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentCapability {
    /// Unique identifier for the capability
    pub id: Uuid,
    
    /// Name of the capability
    pub name: String,
    
    /// Type of capability
    pub capability_type: CapabilityType,
    
    /// Whether this capability is currently enabled
    pub enabled: bool,
    
    /// Configuration specific to this capability
    pub config: HashMap<String, serde_json::Value>,
}

/// Types of capabilities an agent can have.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityType {
    /// Natural language conversation
    Conversation,
    
    /// Code generation and analysis
    CodeGeneration,
    
    /// Tool execution
    ToolExecution { tool_ids: Vec<Uuid> },
    
    /// File system operations
    FileSystem,
    
    /// Web browsing and research
    WebBrowsing,
    
    /// Data analysis and visualization
    DataAnalysis,
    
    /// Image generation
    ImageGeneration,
    
    /// Custom capability with specific identifier
    Custom { identifier: String },
}

/// Configuration parameters for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfiguration {
    /// The LLM model to use for this agent
    pub model: String,
    
    /// Temperature setting for response generation
    pub temperature: f32,
    
    /// Maximum tokens for responses
    pub max_tokens: u32,
    
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    
    /// Response format preference
    pub response_format: ResponseFormat,
    
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
    
    /// Custom model parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

/// Preferred response format for the agent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseFormat {
    /// Plain text responses
    Text,
    
    /// Markdown formatted responses
    Markdown,
    
    /// JSON structured responses
    Json,
    
    /// Mixed format based on context
    Auto,
}

/// Memory configuration for context management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfiguration {
    /// Type of memory system to use
    pub memory_type: MemoryType,
    
    /// Maximum number of messages to retain in context
    pub max_context_messages: usize,
    
    /// Maximum token count for context window
    pub max_context_tokens: u32,
    
    /// Whether to use semantic memory retrieval
    pub use_semantic_memory: bool,
    
    /// Configuration for long-term memory storage
    pub long_term_memory: Option<LongTermMemoryConfig>,
}

/// Types of memory systems available.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    /// Simple sliding window of recent messages
    SlidingWindow,
    
    /// Summary-based memory compression
    Summarization,
    
    /// Retrieval-augmented generation
    RAG,
    
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Configuration for long-term memory storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMemoryConfig {
    /// Whether to persist memories
    pub persist_memories: bool,
    
    /// Retention policy for memories
    pub retention_days: Option<u32>,
    
    /// Categories of information to remember
    pub memory_categories: HashSet<String>,
}

/// Rate limiting configuration for agent operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    
    /// Maximum tokens per minute
    pub tokens_per_minute: Option<u32>,
    
    /// Maximum concurrent requests
    pub max_concurrent: u8,
}

/// Metadata associated with an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Version of the agent configuration
    pub version: String,
    
    /// Creator or owner of the agent
    pub created_by: String,
    
    /// Tags for categorizing agents
    pub tags: Vec<String>,
    
    /// Icon or avatar URL for the agent
    pub avatar_url: Option<String>,
    
    /// Supported languages
    pub languages: Vec<String>,
    
    /// Custom metadata fields
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Represents the runtime context for an agent during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    /// Current conversation ID
    pub conversation_id: Option<Uuid>,
    
    /// Active tool executions
    pub active_tools: Vec<Uuid>,
    
    /// Temporary state for the current interaction
    pub session_state: HashMap<String, serde_json::Value>,
    
    /// Performance metrics for the current session
    pub metrics: AgentMetrics,
}

/// Performance metrics for agent operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentMetrics {
    /// Total tokens consumed
    pub total_tokens: u64,
    
    /// Number of tool executions
    pub tool_executions: u32,
    
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    
    /// Error count
    pub error_count: u32,
    
    /// Success rate percentage
    pub success_rate: f32,
}

impl Agent {
    /// Creates a new agent with basic configuration.
    pub fn new(name: String, description: String, system_prompt: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            system_prompt,
            state: AgentState::Active,
            capabilities: Vec::new(),
            configuration: AgentConfiguration::default(),
            memory_config: MemoryConfiguration::default(),
            metadata: AgentMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Adds a capability to the agent.
    pub fn add_capability(&mut self, capability: AgentCapability) {
        self.capabilities.push(capability);
        self.updated_at = Utc::now();
    }
    
    /// Checks if the agent has a specific capability type.
    pub fn has_capability(&self, capability_type: &CapabilityType) -> bool {
        self.capabilities
            .iter()
            .any(|cap| cap.enabled && &cap.capability_type == capability_type)
    }
    
    /// Gets all enabled capabilities.
    pub fn enabled_capabilities(&self) -> Vec<&AgentCapability> {
        self.capabilities
            .iter()
            .filter(|cap| cap.enabled)
            .collect()
    }
    
    /// Updates the agent's state.
    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
        self.updated_at = Utc::now();
    }
    
    /// Checks if the agent is available for interactions.
    pub fn is_available(&self) -> bool {
        matches!(self.state, AgentState::Active)
    }
}

impl Default for AgentConfiguration {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            top_p: None,
            response_format: ResponseFormat::Auto,
            rate_limit: RateLimitConfig::default(),
            custom_parameters: HashMap::new(),
        }
    }
}

impl Default for MemoryConfiguration {
    fn default() -> Self {
        Self {
            memory_type: MemoryType::SlidingWindow,
            max_context_messages: 50,
            max_context_tokens: 4096,
            use_semantic_memory: false,
            long_term_memory: None,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            tokens_per_minute: Some(90000),
            max_concurrent: 5,
        }
    }
}

impl Default for AgentMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            created_by: "system".to_string(),
            tags: Vec::new(),
            avatar_url: None,
            languages: vec!["en".to_string()],
            custom_fields: HashMap::new(),
        }
    }
}

impl AgentCapability {
    /// Creates a new capability.
    pub fn new(name: String, capability_type: CapabilityType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            capability_type,
            enabled: true,
            config: HashMap::new(),
        }
    }
    
    /// Creates a tool execution capability for specific tools.
    pub fn tool_execution(name: String, tool_ids: Vec<Uuid>) -> Self {
        Self::new(name, CapabilityType::ToolExecution { tool_ids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_creation() {
        let agent = Agent::new(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            "You are a helpful assistant.".to_string(),
        );
        
        assert_eq!(agent.name, "Test Agent");
        assert_eq!(agent.state, AgentState::Active);
        assert!(agent.capabilities.is_empty());
        assert!(agent.is_available());
    }
    
    #[test]
    fn test_agent_capabilities() {
        let mut agent = Agent::new(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            "You are a helpful assistant.".to_string(),
        );
        
        let capability = AgentCapability::new(
            "Code Generation".to_string(),
            CapabilityType::CodeGeneration,
        );
        
        agent.add_capability(capability);
        
        assert_eq!(agent.capabilities.len(), 1);
        assert!(agent.has_capability(&CapabilityType::CodeGeneration));
        assert!(!agent.has_capability(&CapabilityType::WebBrowsing));
    }
    
    #[test]
    fn test_agent_state_changes() {
        let mut agent = Agent::new(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            "You are a helpful assistant.".to_string(),
        );
        
        assert!(agent.is_available());
        
        agent.set_state(AgentState::Processing);
        assert!(!agent.is_available());
        assert_eq!(agent.state, AgentState::Processing);
        
        agent.set_state(AgentState::Active);
        assert!(agent.is_available());
    }
}