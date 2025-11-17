//! Tool executor trait definitions for agent tool execution.
//!
//! This module defines the interface for executing tools on behalf of agents,
//! providing a safe and controlled environment for tool operations.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::core::domain::models::{
    AgentId, Tool, ToolId, ToolResult, ToolOutput, ExecutionStatus,
    ToolType, ParameterType, ExecutionMetrics, Diagnostic,
};

/// Result type for tool executor operations
pub type ExecutorResult<T> = Result<T, ExecutorError>;

/// Errors that can occur during tool execution
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    /// Tool not found
    #[error("Tool not found: {0}")]
    ToolNotFound(ToolId),
    
    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    
    /// Missing required parameter
    #[error("Missing required parameter: {parameter} for tool {tool}")]
    MissingParameter { tool: String, parameter: String },
    
    /// Parameter validation failed
    #[error("Parameter validation failed: {parameter} - {reason}")]
    ValidationError { parameter: String, reason: String },
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Execution timeout
    #[error("Execution timeout after {0} seconds")]
    Timeout(u64),
    
    /// Resource limit exceeded
    #[error("Resource limit exceeded: {resource} - {message}")]
    ResourceLimitExceeded { resource: String, message: String },
    
    /// Execution failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    /// Sandbox violation
    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded for tool {tool}: {message}")]
    RateLimitExceeded { tool: String, message: String },
    
    /// Concurrent execution limit reached
    #[error("Concurrent execution limit reached: {limit}")]
    ConcurrencyLimitReached { limit: u32 },
    
    /// Tool is unavailable
    #[error("Tool unavailable: {tool} - {reason}")]
    ToolUnavailable { tool: String, reason: String },
    
    /// Network error during execution
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Other errors
    #[error("Executor error: {0}")]
    Other(String),
}

/// Main trait for tool execution
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Execute a tool with the given parameters
    async fn execute(
        &self,
        request: ExecutionRequest,
    ) -> ExecutorResult<ToolResult>;
    
    /// Execute a tool with streaming output
    async fn execute_streaming(
        &self,
        request: ExecutionRequest,
    ) -> ExecutorResult<Box<dyn ExecutionStream>>;
    
    /// Validate parameters before execution
    async fn validate_parameters(
        &self,
        tool_id: ToolId,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> ExecutorResult<ValidationResult>;
    
    /// Check if the executor can execute a specific tool
    async fn can_execute(
        &self,
        tool_id: ToolId,
        context: &ExecutionContext,
    ) -> ExecutorResult<bool>;
    
    /// Get execution status for a running execution
    async fn get_execution_status(
        &self,
        execution_id: Uuid,
    ) -> ExecutorResult<ExecutionStatusInfo>;
    
    /// Cancel a running execution
    async fn cancel_execution(
        &self,
        execution_id: Uuid,
    ) -> ExecutorResult<()>;
    
    /// Get available tools for a given context
    async fn list_available_tools(
        &self,
        context: &ExecutionContext,
    ) -> ExecutorResult<Vec<ToolInfo>>;
    
    /// Preload/prepare a tool for faster execution
    async fn prepare_tool(
        &self,
        tool_id: ToolId,
    ) -> ExecutorResult<()>;
    
    /// Clean up resources after tool execution
    async fn cleanup(
        &self,
        execution_id: Uuid,
    ) -> ExecutorResult<()>;
}

/// Request to execute a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// Unique execution ID
    pub execution_id: Uuid,
    
    /// Tool to execute
    pub tool_id: ToolId,
    
    /// Parameters for the tool
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Execution context
    pub context: ExecutionContext,
    
    /// Execution options
    pub options: ExecutionOptions,
    
    /// Callback URL for async execution
    pub callback_url: Option<String>,
}

/// Context for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Agent requesting the execution
    pub agent_id: AgentId,
    
    /// User on whose behalf the agent is acting
    pub user_id: Option<String>,
    
    /// Conversation context
    pub conversation_id: Option<Uuid>,
    
    /// Session ID for tracking
    pub session_id: String,
    
    /// Security context
    pub security: SecurityContext,
    
    /// Environment variables
    pub environment: HashMap<String, String>,
    
    /// Working directory
    pub working_directory: Option<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Security context for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,
    
    /// Permissions/scopes
    pub permissions: Vec<String>,
    
    /// IP address of the requester
    pub ip_address: Option<String>,
    
    /// Security labels
    pub labels: HashMap<String, String>,
}

/// Execution options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOptions {
    /// Timeout override
    pub timeout: Option<Duration>,
    
    /// Priority level
    pub priority: ExecutionPriority,
    
    /// Whether to wait for completion
    pub wait_for_completion: bool,
    
    /// Enable detailed logging
    pub detailed_logging: bool,
    
    /// Resource limit overrides
    pub resource_limits: Option<ResourceLimits>,
    
    /// Retry configuration override
    pub retry_config: Option<RetryConfig>,
    
    /// Output format preferences
    pub output_preferences: OutputPreferences,
}

/// Execution priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Resource limits for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in bytes
    pub max_memory_bytes: Option<u64>,
    
    /// Maximum CPU milliseconds
    pub max_cpu_ms: Option<u64>,
    
    /// Maximum disk I/O in bytes
    pub max_disk_io_bytes: Option<u64>,
    
    /// Maximum network I/O in bytes
    pub max_network_io_bytes: Option<u64>,
    
    /// Maximum execution time
    pub max_execution_time: Option<Duration>,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Initial delay between retries
    pub initial_delay: Duration,
    
    /// Maximum delay between retries
    pub max_delay: Duration,
    
    /// Backoff multiplier
    pub backoff_multiplier: f32,
    
    /// Jitter to add to delays
    pub jitter: bool,
}

/// Output format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPreferences {
    /// Preferred output format
    pub format: PreferredFormat,
    
    /// Include metadata in output
    pub include_metadata: bool,
    
    /// Pretty print JSON output
    pub pretty_json: bool,
    
    /// Maximum output size
    pub max_size_bytes: Option<u64>,
}

/// Preferred output formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PreferredFormat {
    Text,
    Json,
    Yaml,
    Xml,
    Binary,
    Auto,
}

/// Validation result for parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub valid: bool,
    
    /// Validation errors
    pub errors: Vec<ValidationError>,
    
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    
    /// Normalized parameters
    pub normalized_parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Validation error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Parameter that failed validation
    pub parameter: String,
    
    /// Error code
    pub code: String,
    
    /// Error message
    pub message: String,
    
    /// Expected value/format
    pub expected: Option<String>,
    
    /// Actual value provided
    pub actual: Option<String>,
}

/// Validation warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Parameter that triggered warning
    pub parameter: String,
    
    /// Warning code
    pub code: String,
    
    /// Warning message
    pub message: String,
    
    /// Suggestion for fixing
    pub suggestion: Option<String>,
}

/// Information about execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatusInfo {
    /// Execution ID
    pub execution_id: Uuid,
    
    /// Current status
    pub status: ExecutionStatus,
    
    /// Progress percentage (0-100)
    pub progress: Option<u8>,
    
    /// Status message
    pub message: Option<String>,
    
    /// Start time
    pub started_at: DateTime<Utc>,
    
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    
    /// Current resource usage
    pub resource_usage: ResourceUsage,
    
    /// Partial results if available
    pub partial_results: Option<Vec<PartialResult>>,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory used in bytes
    pub memory_bytes: u64,
    
    /// CPU time used in milliseconds
    pub cpu_ms: u64,
    
    /// Disk I/O in bytes
    pub disk_io_bytes: u64,
    
    /// Network I/O in bytes
    pub network_io_bytes: u64,
}

/// Partial result from streaming execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialResult {
    /// Sequence number
    pub sequence: u32,
    
    /// Partial output
    pub output: ToolOutput,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Tool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    /// Tool ID
    pub id: ToolId,
    
    /// Tool name
    pub name: String,
    
    /// Tool description
    pub description: String,
    
    /// Tool type
    pub tool_type: ToolType,
    
    /// Whether the tool is available
    pub available: bool,
    
    /// Estimated execution time
    pub estimated_duration_ms: Option<u64>,
    
    /// Required permissions
    pub required_permissions: Vec<String>,
    
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
}

/// Resource requirements for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Minimum memory needed
    pub min_memory_bytes: Option<u64>,
    
    /// Estimated CPU usage
    pub estimated_cpu_ms: Option<u64>,
    
    /// Requires network access
    pub requires_network: bool,
    
    /// Requires filesystem access
    pub requires_filesystem: bool,
    
    /// Other requirements
    pub other: HashMap<String, String>,
}

/// Stream for tool execution output
#[async_trait]
pub trait ExecutionStream: Send {
    /// Get the next output chunk
    async fn next_chunk(&mut self) -> Option<ExecutorResult<ExecutionChunk>>;
    
    /// Get current progress
    fn progress(&self) -> Option<u8>;
    
    /// Cancel the stream
    async fn cancel(&mut self) -> ExecutorResult<()>;
}

/// Chunk of execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionChunk {
    /// Chunk sequence number
    pub sequence: u32,
    
    /// Chunk data
    pub data: ChunkData,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Whether this is the final chunk
    pub is_final: bool,
}

/// Types of chunk data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkData {
    /// Text output
    Text(String),
    
    /// Binary data
    Binary(Vec<u8>),
    
    /// Progress update
    Progress {
        percentage: u8,
        message: Option<String>,
    },
    
    /// Log message
    Log {
        level: LogLevel,
        message: String,
    },
    
    /// Metric update
    Metric {
        name: String,
        value: f64,
        unit: Option<String>,
    },
    
    /// Error occurred
    Error {
        code: String,
        message: String,
    },
}

/// Log levels for execution logs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

/// Factory for creating tool executors
#[async_trait]
pub trait ToolExecutorFactory: Send + Sync {
    /// Create an executor for a specific tool type
    async fn create_executor(
        &self,
        tool_type: &ToolType,
    ) -> ExecutorResult<Box<dyn ToolExecutor>>;
    
    /// Get all available executor types
    fn available_executors(&self) -> Vec<String>;
}

/// Registry for tool executors
#[async_trait]
pub trait ToolExecutorRegistry: Send + Sync {
    /// Register a new executor
    async fn register_executor(
        &self,
        name: &str,
        executor: Box<dyn ToolExecutor>,
    ) -> ExecutorResult<()>;
    
    /// Get an executor by name
    async fn get_executor(
        &self,
        name: &str,
    ) -> ExecutorResult<Box<dyn ToolExecutor>>;
    
    /// List all registered executors
    async fn list_executors(&self) -> Vec<String>;
    
    /// Unregister an executor
    async fn unregister_executor(
        &self,
        name: &str,
    ) -> ExecutorResult<()>;
}

impl Default for ExecutionPriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            priority: ExecutionPriority::Normal,
            wait_for_completion: true,
            detailed_logging: false,
            resource_limits: None,
            retry_config: None,
            output_preferences: OutputPreferences::default(),
        }
    }
}

impl Default for OutputPreferences {
    fn default() -> Self {
        Self {
            format: PreferredFormat::Auto,
            include_metadata: false,
            pretty_json: false,
            max_size_bytes: None,
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_bytes: 0,
            cpu_ms: 0,
            disk_io_bytes: 0,
            network_io_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_execution_request_creation() {
        let mut params = HashMap::new();
        params.insert("file".to_string(), serde_json::Value::String("/tmp/test.txt".to_string()));
        
        let request = ExecutionRequest {
            execution_id: Uuid::new_v4(),
            tool_id: Uuid::new_v4(),
            parameters: params,
            context: ExecutionContext {
                agent_id: Uuid::new_v4(),
                user_id: Some("user123".to_string()),
                conversation_id: None,
                session_id: "session-456".to_string(),
                security: SecurityContext {
                    auth_token: None,
                    permissions: vec!["file:read".to_string()],
                    ip_address: None,
                    labels: HashMap::new(),
                },
                environment: HashMap::new(),
                working_directory: None,
                metadata: HashMap::new(),
            },
            options: ExecutionOptions::default(),
            callback_url: None,
        };
        
        assert_eq!(request.parameters.len(), 1);
        assert!(request.parameters.contains_key("file"));
    }
    
    #[test]
    fn test_validation_result() {
        let result = ValidationResult {
            valid: false,
            errors: vec![
                ValidationError {
                    parameter: "timeout".to_string(),
                    code: "INVALID_TYPE".to_string(),
                    message: "Expected number, got string".to_string(),
                    expected: Some("number".to_string()),
                    actual: Some("string".to_string()),
                }
            ],
            warnings: vec![],
            normalized_parameters: None,
        };
        
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].parameter, "timeout");
    }
    
    #[test]
    fn test_execution_priority_ordering() {
        assert!(ExecutionPriority::Low < ExecutionPriority::Normal);
        assert!(ExecutionPriority::Normal < ExecutionPriority::High);
        assert!(ExecutionPriority::High < ExecutionPriority::Critical);
    }
}