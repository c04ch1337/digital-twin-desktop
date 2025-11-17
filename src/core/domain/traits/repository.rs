//! Repository trait definitions for domain entities.
//!
//! This module defines repository interfaces that abstract data persistence
//! operations, following the repository pattern to keep domain logic
//! independent of infrastructure concerns.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::domain::models::{
    Agent, AgentId, AgentState,
    Conversation, ConversationId, ConversationState, Message, MessageId,
    DigitalTwin, TwinId, TwinState, TwinType,
    SensorData, SensorDataId, SensorReading,
    Tool, ToolId, ToolResult, ExecutionId, ToolType,
};

/// Common result type for repository operations
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Errors that can occur during repository operations.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /// Entity not found
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },
    
    /// Entity already exists
    #[error("Entity already exists: {entity_type} with id {id}")]
    AlreadyExists { entity_type: String, id: String },
    
    /// Invalid query parameters
    #[error("Invalid query: {message}")]
    InvalidQuery { message: String },
    
    /// Connection or database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    /// Other errors
    #[error("Repository error: {0}")]
    Other(String),
}

/// Query filter operators
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
}

/// Query filter criteria
#[derive(Debug, Clone)]
pub struct FilterCriteria {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Sort criteria
#[derive(Debug, Clone)]
pub struct SortCriteria {
    pub field: String,
    pub order: SortOrder,
}

/// Pagination parameters
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub offset: usize,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 50,
        }
    }
}

/// Paginated result
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
}

/// Repository for Conversation entities
#[async_trait]
pub trait ConversationRepository: Send + Sync {
    /// Create a new conversation
    async fn create(&self, conversation: Conversation) -> RepositoryResult<Conversation>;
    
    /// Get a conversation by ID
    async fn get_by_id(&self, id: ConversationId) -> RepositoryResult<Conversation>;
    
    /// Update an existing conversation
    async fn update(&self, conversation: Conversation) -> RepositoryResult<Conversation>;
    
    /// Delete a conversation
    async fn delete(&self, id: ConversationId) -> RepositoryResult<()>;
    
    /// Find conversations with filters
    async fn find(
        &self,
        filters: Vec<FilterCriteria>,
        sort: Vec<SortCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Conversation>>;
    
    /// Get conversations by participant agent ID
    async fn get_by_agent_id(
        &self,
        agent_id: AgentId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Conversation>>;
    
    /// Get conversations by state
    async fn get_by_state(
        &self,
        state: ConversationState,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Conversation>>;
    
    /// Add a message to a conversation
    async fn add_message(
        &self,
        conversation_id: ConversationId,
        message: Message,
    ) -> RepositoryResult<Message>;
    
    /// Get messages for a conversation
    async fn get_messages(
        &self,
        conversation_id: ConversationId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Message>>;
    
    /// Search conversations by content
    async fn search(
        &self,
        query: &str,
        filters: Vec<FilterCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Conversation>>;
}

/// Repository for Agent entities
#[async_trait]
pub trait AgentRepository: Send + Sync {
    /// Create a new agent
    async fn create(&self, agent: Agent) -> RepositoryResult<Agent>;
    
    /// Get an agent by ID
    async fn get_by_id(&self, id: AgentId) -> RepositoryResult<Agent>;
    
    /// Update an existing agent
    async fn update(&self, agent: Agent) -> RepositoryResult<Agent>;
    
    /// Delete an agent
    async fn delete(&self, id: AgentId) -> RepositoryResult<()>;
    
    /// Find agents with filters
    async fn find(
        &self,
        filters: Vec<FilterCriteria>,
        sort: Vec<SortCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>>;
    
    /// Get agents by state
    async fn get_by_state(
        &self,
        state: AgentState,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>>;
    
    /// Get agents by capability type
    async fn get_by_capability(
        &self,
        capability_type: &str,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>>;
    
    /// Search agents by name or description
    async fn search(
        &self,
        query: &str,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Agent>>;
    
    /// Update agent state
    async fn update_state(
        &self,
        id: AgentId,
        state: AgentState,
    ) -> RepositoryResult<()>;
}

/// Repository for DigitalTwin entities
#[async_trait]
pub trait TwinRepository: Send + Sync {
    /// Create a new digital twin
    async fn create(&self, twin: DigitalTwin) -> RepositoryResult<DigitalTwin>;
    
    /// Get a digital twin by ID
    async fn get_by_id(&self, id: TwinId) -> RepositoryResult<DigitalTwin>;
    
    /// Update an existing digital twin
    async fn update(&self, twin: DigitalTwin) -> RepositoryResult<DigitalTwin>;
    
    /// Delete a digital twin
    async fn delete(&self, id: TwinId) -> RepositoryResult<()>;
    
    /// Find twins with filters
    async fn find(
        &self,
        filters: Vec<FilterCriteria>,
        sort: Vec<SortCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>>;
    
    /// Get twins by type
    async fn get_by_type(
        &self,
        twin_type: &TwinType,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>>;
    
    /// Get twins by state
    async fn get_by_state(
        &self,
        state: TwinState,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>>;
    
    /// Get twins by agent ID
    async fn get_by_agent_id(
        &self,
        agent_id: AgentId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<DigitalTwin>>;
    
    /// Update twin state
    async fn update_state(
        &self,
        id: TwinId,
        state: TwinState,
    ) -> RepositoryResult<()>;
    
    /// Update twin properties
    async fn update_properties(
        &self,
        id: TwinId,
        properties: HashMap<String, serde_json::Value>,
    ) -> RepositoryResult<()>;
    
    /// Mark twin as synchronized
    async fn mark_synchronized(
        &self,
        id: TwinId,
        timestamp: DateTime<Utc>,
    ) -> RepositoryResult<()>;
    
    /// Get twins needing synchronization
    async fn get_twins_needing_sync(
        &self,
        limit: usize,
    ) -> RepositoryResult<Vec<DigitalTwin>>;
}

/// Repository for SensorData entities
#[async_trait]
pub trait SensorDataRepository: Send + Sync {
    /// Create new sensor data
    async fn create(&self, sensor_data: SensorData) -> RepositoryResult<SensorData>;
    
    /// Get sensor data by ID
    async fn get_by_id(&self, id: SensorDataId) -> RepositoryResult<SensorData>;
    
    /// Update sensor data
    async fn update(&self, sensor_data: SensorData) -> RepositoryResult<SensorData>;
    
    /// Delete sensor data
    async fn delete(&self, id: SensorDataId) -> RepositoryResult<()>;
    
    /// Get sensor data by twin ID
    async fn get_by_twin_id(
        &self,
        twin_id: TwinId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<SensorData>>;
    
    /// Add sensor reading
    async fn add_reading(
        &self,
        sensor_data_id: SensorDataId,
        reading: SensorReading,
    ) -> RepositoryResult<()>;
    
    /// Get readings within time range
    async fn get_readings_in_range(
        &self,
        sensor_data_id: SensorDataId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<SensorReading>>;
    
    /// Get latest reading for sensor
    async fn get_latest_reading(
        &self,
        sensor_data_id: SensorDataId,
    ) -> RepositoryResult<Option<SensorReading>>;
    
    /// Get aggregated data
    async fn get_aggregated_data(
        &self,
        sensor_data_id: SensorDataId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        interval: &str, // e.g., "1h", "1d"
        aggregation: &str, // e.g., "avg", "sum", "max", "min"
    ) -> RepositoryResult<Vec<(DateTime<Utc>, f64)>>;
    
    /// Delete old readings based on retention policy
    async fn cleanup_old_readings(
        &self,
        retention_days: u32,
    ) -> RepositoryResult<usize>;
}

/// Repository for Tool entities
#[async_trait]
pub trait ToolRepository: Send + Sync {
    /// Create a new tool
    async fn create(&self, tool: Tool) -> RepositoryResult<Tool>;
    
    /// Get a tool by ID
    async fn get_by_id(&self, id: ToolId) -> RepositoryResult<Tool>;
    
    /// Update an existing tool
    async fn update(&self, tool: Tool) -> RepositoryResult<Tool>;
    
    /// Delete a tool
    async fn delete(&self, id: ToolId) -> RepositoryResult<()>;
    
    /// Find tools with filters
    async fn find(
        &self,
        filters: Vec<FilterCriteria>,
        sort: Vec<SortCriteria>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>>;
    
    /// Get tools by type
    async fn get_by_type(
        &self,
        tool_type: &ToolType,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>>;
    
    /// Get available tools
    async fn get_available(
        &self,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>>;
    
    /// Search tools by name or description
    async fn search(
        &self,
        query: &str,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<Tool>>;
    
    /// Save tool execution result
    async fn save_execution_result(
        &self,
        result: ToolResult,
    ) -> RepositoryResult<ToolResult>;
    
    /// Get execution result by ID
    async fn get_execution_by_id(
        &self,
        execution_id: ExecutionId,
    ) -> RepositoryResult<ToolResult>;
    
    /// Get execution results for a tool
    async fn get_executions_by_tool_id(
        &self,
        tool_id: ToolId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<ToolResult>>;
    
    /// Update tool usage statistics
    async fn update_usage_stats(
        &self,
        tool_id: ToolId,
        execution_time_ms: u64,
        success: bool,
    ) -> RepositoryResult<()>;
}

/// Unit of Work trait for transactional operations
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Begin a new transaction
    async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>>;
}

/// Transaction trait for atomic operations
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Commit the transaction
    async fn commit(self: Box<Self>) -> RepositoryResult<()>;
    
    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;
    
    /// Get conversation repository for this transaction
    fn conversations(&self) -> &dyn ConversationRepository;
    
    /// Get agent repository for this transaction
    fn agents(&self) -> &dyn AgentRepository;
    
    /// Get twin repository for this transaction
    fn twins(&self) -> &dyn TwinRepository;
    
    /// Get sensor data repository for this transaction
    fn sensor_data(&self) -> &dyn SensorDataRepository;
    
    /// Get tool repository for this transaction
    fn tools(&self) -> &dyn ToolRepository;
}

/// Repository factory trait
#[async_trait]
pub trait RepositoryFactory: Send + Sync {
    /// Create conversation repository
    async fn create_conversation_repository(&self) -> Box<dyn ConversationRepository>;
    
    /// Create agent repository
    async fn create_agent_repository(&self) -> Box<dyn AgentRepository>;
    
    /// Create twin repository
    async fn create_twin_repository(&self) -> Box<dyn TwinRepository>;
    
    /// Create sensor data repository
    async fn create_sensor_data_repository(&self) -> Box<dyn SensorDataRepository>;
    
    /// Create tool repository
    async fn create_tool_repository(&self) -> Box<dyn ToolRepository>;
    
    /// Create unit of work
    async fn create_unit_of_work(&self) -> Box<dyn UnitOfWork>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_criteria_creation() {
        let filter = FilterCriteria {
            field: "name".to_string(),
            operator: FilterOperator::Contains,
            value: serde_json::Value::String("test".to_string()),
        };
        
        assert_eq!(filter.field, "name");
        assert_eq!(filter.operator, FilterOperator::Contains);
    }
    
    #[test]
    fn test_pagination_default() {
        let pagination = Pagination::default();
        assert_eq!(pagination.offset, 0);
        assert_eq!(pagination.limit, 50);
    }
    
    #[test]
    fn test_repository_error_display() {
        let error = RepositoryError::NotFound {
            entity_type: "Agent".to_string(),
            id: "123".to_string(),
        };
        
        assert_eq!(error.to_string(), "Entity not found: Agent with id 123");
    }
}