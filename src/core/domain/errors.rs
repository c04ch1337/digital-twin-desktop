//! Domain-specific error types for the Digital Twin Desktop application.
//!
//! This module defines comprehensive error types that represent various
//! failure scenarios in the domain layer, using thiserror for ergonomic
//! error handling and clear error messages.

use thiserror::Error;
use uuid::Uuid;
use std::time::Duration;

use crate::core::domain::models::{
    AgentId, ConversationId, MessageId, TwinId, SensorDataId, ToolId, ExecutionId,
    AgentState, TwinState, ConversationState, SensorStatus, ToolStatus, ExecutionStatus,
};

/// Main domain error type that encompasses all domain-specific errors
#[derive(Debug, Error)]
pub enum DomainError {
    /// Entity not found errors
    #[error("Entity not found: {0}")]
    NotFound(#[from] NotFoundError),
    
    /// Validation errors
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    
    /// State transition errors
    #[error("Invalid state transition: {0}")]
    StateTransition(#[from] StateTransitionError),
    
    /// Authorization errors
    #[error("Authorization failed: {0}")]
    Authorization(#[from] AuthorizationError),
    
    /// Business rule violations
    #[error("Business rule violation: {0}")]
    BusinessRule(#[from] BusinessRuleError),
    
    /// Resource constraint errors
    #[error("Resource constraint: {0}")]
    ResourceConstraint(#[from] ResourceConstraintError),
    
    /// Conflict errors
    #[error("Conflict detected: {0}")]
    Conflict(#[from] ConflictError),
    
    /// Operation timeout
    #[error("Operation timed out after {duration:?}")]
    Timeout { duration: Duration },
    
    /// External service errors
    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
    
    /// Data integrity errors
    #[error("Data integrity violation: {0}")]
    DataIntegrity(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Other domain errors
    #[error("Domain error: {0}")]
    Other(String),
}

/// Entity not found errors
#[derive(Debug, Error)]
pub enum NotFoundError {
    /// Agent not found
    #[error("Agent not found with ID: {0}")]
    Agent(AgentId),
    
    /// Conversation not found
    #[error("Conversation not found with ID: {0}")]
    Conversation(ConversationId),
    
    /// Message not found
    #[error("Message not found with ID: {0}")]
    Message(MessageId),
    
    /// Digital Twin not found
    #[error("Digital Twin not found with ID: {0}")]
    DigitalTwin(TwinId),
    
    /// Sensor data not found
    #[error("Sensor data not found with ID: {0}")]
    SensorData(SensorDataId),
    
    /// Tool not found
    #[error("Tool not found with ID: {0}")]
    Tool(ToolId),
    
    /// Tool execution not found
    #[error("Tool execution not found with ID: {0}")]
    ToolExecution(ExecutionId),
    
    /// Generic entity not found
    #[error("{entity_type} not found with identifier: {identifier}")]
    Generic { entity_type: String, identifier: String },
}

/// Validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Empty or blank value
    #[error("Field '{field}' cannot be empty")]
    EmptyField { field: String },
    
    /// Invalid length
    #[error("Field '{field}' must be between {min} and {max} characters, got {actual}")]
    InvalidLength { 
        field: String, 
        min: usize, 
        max: usize, 
        actual: usize 
    },
    
    /// Invalid format
    #[error("Field '{field}' has invalid format: {reason}")]
    InvalidFormat { field: String, reason: String },
    
    /// Invalid value range
    #[error("Field '{field}' must be between {min} and {max}, got {actual}")]
    OutOfRange { 
        field: String, 
        min: String, 
        max: String, 
        actual: String 
    },
    
    /// Invalid URL
    #[error("Invalid URL in field '{field}': {url}")]
    InvalidUrl { field: String, url: String },
    
    /// Invalid email
    #[error("Invalid email in field '{field}': {email}")]
    InvalidEmail { field: String, email: String },
    
    /// Pattern mismatch
    #[error("Field '{field}' does not match required pattern: {pattern}")]
    PatternMismatch { field: String, pattern: String },
    
    /// Missing required field
    #[error("Required field '{field}' is missing")]
    MissingRequired { field: String },
    
    /// Invalid enum value
    #[error("Invalid value '{value}' for field '{field}'. Valid values are: {valid_values:?}")]
    InvalidEnumValue { 
        field: String, 
        value: String, 
        valid_values: Vec<String> 
    },
    
    /// Duplicate value
    #[error("Duplicate value '{value}' for unique field '{field}'")]
    DuplicateValue { field: String, value: String },
    
    /// Invalid reference
    #[error("Invalid reference in field '{field}': referenced entity does not exist")]
    InvalidReference { field: String },
    
    /// Multiple validation errors
    #[error("Multiple validation errors: {0:?}")]
    Multiple(Vec<ValidationError>),
}

/// State transition errors
#[derive(Debug, Error)]
pub enum StateTransitionError {
    /// Invalid agent state transition
    #[error("Cannot transition agent from {from:?} to {to:?}")]
    InvalidAgentTransition { from: AgentState, to: AgentState },
    
    /// Invalid twin state transition
    #[error("Cannot transition digital twin from {from:?} to {to:?}")]
    InvalidTwinTransition { from: TwinState, to: TwinState },
    
    /// Invalid conversation state transition
    #[error("Cannot transition conversation from {from:?} to {to:?}")]
    InvalidConversationTransition { 
        from: ConversationState, 
        to: ConversationState 
    },
    
    /// Invalid sensor status transition
    #[error("Cannot transition sensor from {from:?} to {to:?}")]
    InvalidSensorTransition { from: SensorStatus, to: SensorStatus },
    
    /// Invalid tool status transition
    #[error("Cannot transition tool from {from:?} to {to:?}")]
    InvalidToolTransition { from: ToolStatus, to: ToolStatus },
    
    /// Invalid execution status transition
    #[error("Cannot transition execution from {from:?} to {to:?}")]
    InvalidExecutionTransition { 
        from: ExecutionStatus, 
        to: ExecutionStatus 
    },
    
    /// State precondition not met
    #[error("State precondition not met: {reason}")]
    PreconditionNotMet { reason: String },
}

/// Authorization errors
#[derive(Debug, Error)]
pub enum AuthorizationError {
    /// Insufficient permissions
    #[error("Insufficient permissions: requires {required:?}, has {actual:?}")]
    InsufficientPermissions { 
        required: Vec<String>, 
        actual: Vec<String> 
    },
    
    /// Access denied to entity
    #[error("Access denied to {entity_type} with ID {entity_id}")]
    AccessDenied { entity_type: String, entity_id: String },
    
    /// Operation not allowed
    #[error("Operation '{operation}' not allowed for user '{user}'")]
    OperationNotAllowed { operation: String, user: String },
    
    /// Invalid credentials
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    
    /// Token expired
    #[error("Authentication token has expired")]
    TokenExpired,
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded: {limit} requests per {period}")]
    RateLimitExceeded { limit: u32, period: String },
}

/// Business rule errors
#[derive(Debug, Error)]
pub enum BusinessRuleError {
    /// Agent capacity exceeded
    #[error("Agent {agent_id} has reached maximum capacity of {max} concurrent conversations")]
    AgentCapacityExceeded { agent_id: AgentId, max: u32 },
    
    /// Conversation participant limit
    #[error("Conversation cannot have more than {max} participants")]
    ConversationParticipantLimit { max: u32 },
    
    /// Tool execution limit
    #[error("Tool {tool_id} execution limit reached: {limit} per {period}")]
    ToolExecutionLimitReached { 
        tool_id: ToolId, 
        limit: u32, 
        period: String 
    },
    
    /// Sensor reading out of bounds
    #[error("Sensor reading {value} is out of acceptable bounds [{min}, {max}]")]
    SensorReadingOutOfBounds { 
        value: f64, 
        min: f64, 
        max: f64 
    },
    
    /// Twin sync interval violation
    #[error("Twin sync interval {requested}s is below minimum allowed {minimum}s")]
    TwinSyncIntervalTooShort { requested: u32, minimum: u32 },
    
    /// Message size limit
    #[error("Message size {size} bytes exceeds limit of {limit} bytes")]
    MessageSizeExceeded { size: usize, limit: usize },
    
    /// Circular dependency detected
    #[error("Circular dependency detected: {context}")]
    CircularDependency { context: String },
    
    /// Invalid tool chain
    #[error("Invalid tool chain: {reason}")]
    InvalidToolChain { reason: String },
    
    /// Data retention policy violation
    #[error("Cannot delete data: retention policy requires keeping data for {days} days")]
    RetentionPolicyViolation { days: u32 },
    
    /// Custom business rule violation
    #[error("Business rule violation: {rule} - {message}")]
    Custom { rule: String, message: String },
}

/// Resource constraint errors
#[derive(Debug, Error)]
pub enum ResourceConstraintError {
    /// Memory limit exceeded
    #[error("Memory limit exceeded: requested {requested} bytes, available {available} bytes")]
    MemoryLimitExceeded { requested: u64, available: u64 },
    
    /// Storage limit exceeded
    #[error("Storage limit exceeded: {used} of {limit} bytes used")]
    StorageLimitExceeded { used: u64, limit: u64 },
    
    /// CPU usage limit
    #[error("CPU usage limit exceeded: {usage}% (limit: {limit}%)")]
    CpuLimitExceeded { usage: f32, limit: f32 },
    
    /// Connection pool exhausted
    #[error("Connection pool exhausted: all {size} connections in use")]
    ConnectionPoolExhausted { size: u32 },
    
    /// Queue full
    #[error("Queue '{queue}' is full: {size} items (capacity: {capacity})")]
    QueueFull { queue: String, size: u32, capacity: u32 },
    
    /// Token limit exceeded
    #[error("Token limit exceeded: {used} tokens (limit: {limit})")]
    TokenLimitExceeded { used: u32, limit: u32 },
    
    /// Bandwidth limit exceeded
    #[error("Bandwidth limit exceeded: {used} bytes/s (limit: {limit} bytes/s)")]
    BandwidthLimitExceeded { used: u64, limit: u64 },
}

/// Conflict errors
#[derive(Debug, Error)]
pub enum ConflictError {
    /// Concurrent modification
    #[error("Concurrent modification detected for {entity_type} {entity_id}")]
    ConcurrentModification { 
        entity_type: String, 
        entity_id: String 
    },
    
    /// Version mismatch
    #[error("Version mismatch: expected {expected}, actual {actual}")]
    VersionMismatch { expected: String, actual: String },
    
    /// Duplicate entity
    #[error("Duplicate {entity_type} with identifier {identifier}")]
    DuplicateEntity { 
        entity_type: String, 
        identifier: String 
    },
    
    /// Resource locked
    #[error("Resource {resource_type} {resource_id} is locked by another process")]
    ResourceLocked { 
        resource_type: String, 
        resource_id: String 
    },
    
    /// State conflict
    #[error("State conflict: {message}")]
    StateConflict { message: String },
}

/// Result type alias for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

/// Extension trait for converting between error types
pub trait ErrorExt {
    /// Convert to a domain error with additional context
    fn context(self, context: &str) -> DomainError;
}

impl<E: Into<DomainError>> ErrorExt for E {
    fn context(self, context: &str) -> DomainError {
        let base_error = self.into();
        match base_error {
            DomainError::Other(msg) => DomainError::Other(format!("{}: {}", context, msg)),
            err => DomainError::Other(format!("{}: {}", context, err)),
        }
    }
}

/// Helper functions for creating common validation errors
impl ValidationError {
    /// Create a new empty field error
    pub fn empty_field(field: &str) -> Self {
        Self::EmptyField { 
            field: field.to_string() 
        }
    }
    
    /// Create a new invalid length error
    pub fn invalid_length(field: &str, min: usize, max: usize, actual: usize) -> Self {
        Self::InvalidLength { 
            field: field.to_string(), 
            min, 
            max, 
            actual 
        }
    }
    
    /// Create a new out of range error
    pub fn out_of_range<T: ToString>(field: &str, min: T, max: T, actual: T) -> Self {
        Self::OutOfRange {
            field: field.to_string(),
            min: min.to_string(),
            max: max.to_string(),
            actual: actual.to_string(),
        }
    }
    
    /// Combine multiple validation errors
    pub fn combine(errors: Vec<ValidationError>) -> Self {
        match errors.len() {
            0 => unreachable!("Cannot combine empty error list"),
            1 => errors.into_iter().next().unwrap(),
            _ => Self::Multiple(errors),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_not_found_error() {
        let agent_id = Uuid::new_v4();
        let error = NotFoundError::Agent(agent_id);
        assert_eq!(
            error.to_string(), 
            format!("Agent not found with ID: {}", agent_id)
        );
    }
    
    #[test]
    fn test_validation_error_helpers() {
        let error = ValidationError::empty_field("name");
        assert_eq!(error.to_string(), "Field 'name' cannot be empty");
        
        let error = ValidationError::invalid_length("description", 10, 100, 5);
        assert_eq!(
            error.to_string(), 
            "Field 'description' must be between 10 and 100 characters, got 5"
        );
        
        let error = ValidationError::out_of_range("temperature", -50, 150, 200);
        assert_eq!(
            error.to_string(),
            "Field 'temperature' must be between -50 and 150, got 200"
        );
    }
    
    #[test]
    fn test_domain_error_from_variants() {
        let not_found = NotFoundError::Agent(Uuid::new_v4());
        let domain_error: DomainError = not_found.into();
        assert!(matches!(domain_error, DomainError::NotFound(_)));
        
        let validation = ValidationError::empty_field("test");
        let domain_error: DomainError = validation.into();
        assert!(matches!(domain_error, DomainError::Validation(_)));
    }
    
    #[test]
    fn test_error_context() {
        let error = ValidationError::empty_field("name");
        let contextualized = error.context("Failed to create agent");
        match contextualized {
            DomainError::Other(msg) => {
                assert!(msg.contains("Failed to create agent"));
            }
            _ => panic!("Expected Other variant"),
        }
    }
    
    #[test]
    fn test_multiple_validation_errors() {
        let errors = vec![
            ValidationError::empty_field("name"),
            ValidationError::invalid_length("description", 10, 100, 5),
        ];
        
        let combined = ValidationError::combine(errors);
        assert!(matches!(combined, ValidationError::Multiple(_)));
    }
}