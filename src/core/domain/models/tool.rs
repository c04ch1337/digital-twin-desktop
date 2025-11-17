//! Tool domain models for agent capabilities.
//!
//! This module defines tools that digital twin agents can use to perform
//! actions, gather information, and interact with external systems.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a tool that agents can use to perform actions.
///
/// Tools extend agent capabilities by providing specific functions like
/// file operations, API calls, data processing, or system interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Unique identifier for the tool
    pub id: Uuid,
    
    /// Human-readable name of the tool
    pub name: String,
    
    /// Description of what the tool does
    pub description: String,
    
    /// Category/type of tool
    pub tool_type: ToolType,
    
    /// Input parameters the tool accepts
    pub parameters: Vec<ToolParameter>,
    
    /// Expected output format
    pub output_schema: OutputSchema,
    
    /// Execution configuration
    pub execution_config: ExecutionConfig,
    
    /// Security and permission settings
    pub security: SecurityConfig,
    
    /// Tool availability and status
    pub status: ToolStatus,
    
    /// Usage statistics and limits
    pub usage: ToolUsage,
    
    /// Metadata for the tool
    pub metadata: ToolMetadata,
    
    /// When the tool was created
    pub created_at: DateTime<Utc>,
    
    /// When the tool was last updated
    pub updated_at: DateTime<Utc>,
}

/// Categories of tools available to agents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolType {
    /// File system operations
    FileSystem {
        allowed_operations: Vec<FileOperation>,
        allowed_paths: Vec<String>,
    },
    
    /// HTTP/API requests
    HttpRequest {
        allowed_methods: Vec<HttpMethod>,
        allowed_domains: Vec<String>,
    },
    
    /// Database operations
    Database {
        db_type: String,
        allowed_operations: Vec<DatabaseOperation>,
    },
    
    /// Code execution
    CodeExecution {
        language: String,
        sandbox_config: SandboxConfig,
    },
    
    /// Data transformation
    DataTransformation {
        transform_type: String,
        supported_formats: Vec<String>,
    },
    
    /// System commands
    SystemCommand {
        allowed_commands: Vec<String>,
        environment: HashMap<String, String>,
    },
    
    /// AI/ML model interaction
    ModelInference {
        model_type: String,
        model_id: String,
    },
    
    /// Communication tools
    Communication {
        channel_type: String,
        protocols: Vec<String>,
    },
    
    /// Custom tool type
    Custom {
        category: String,
        capabilities: HashMap<String, serde_json::Value>,
    },
}

/// Represents the result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Unique identifier for this execution
    pub execution_id: Uuid,
    
    /// The tool that was executed
    pub tool_id: Uuid,
    
    /// Execution status
    pub status: ExecutionStatus,
    
    /// The actual result data
    pub output: ToolOutput,
    
    /// Execution metrics
    pub metrics: ExecutionMetrics,
    
    /// Any errors or warnings
    pub diagnostics: Vec<Diagnostic>,
    
    /// When execution started
    pub started_at: DateTime<Utc>,
    
    /// When execution completed
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status of a tool execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Execution pending
    Pending,
    
    /// Currently running
    Running,
    
    /// Completed successfully
    Success,
    
    /// Failed with error
    Failed,
    
    /// Execution was cancelled
    Cancelled,
    
    /// Timed out
    Timeout,
    
    /// Partial success with warnings
    PartialSuccess,
}

/// Output from a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolOutput {
    /// Text output
    Text(String),
    
    /// JSON structured data
    Json(serde_json::Value),
    
    /// Binary data (base64 encoded)
    Binary {
        data: String,
        mime_type: String,
    },
    
    /// File reference
    File {
        path: String,
        size: u64,
        checksum: Option<String>,
    },
    
    /// Multiple outputs
    Multiple(Vec<ToolOutput>),
    
    /// Streaming output handle
    Stream {
        stream_id: String,
        content_type: String,
    },
    
    /// Empty/void output
    Void,
    
    /// Error output
    Error {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },
}

/// Parameter definition for a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    /// Parameter name
    pub name: String,
    
    /// Description of the parameter
    pub description: String,
    
    /// Parameter type
    pub param_type: ParameterType,
    
    /// Whether the parameter is required
    pub required: bool,
    
    /// Default value if not provided
    pub default_value: Option<serde_json::Value>,
    
    /// Validation rules
    pub validation: Option<ValidationRules>,
}

/// Types of parameters tools can accept.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    /// String parameter
    String,
    
    /// Integer parameter
    Integer,
    
    /// Floating point number
    Float,
    
    /// Boolean parameter
    Boolean,
    
    /// Array/list parameter
    Array {
        item_type: Box<ParameterType>,
    },
    
    /// Object/map parameter
    Object {
        properties: HashMap<String, ParameterType>,
    },
    
    /// File path parameter
    FilePath,
    
    /// URL parameter
    Url,
    
    /// JSON parameter
    Json,
    
    /// Enum with specific values
    Enum {
        values: Vec<String>,
    },
}

/// Validation rules for parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Minimum value (for numbers)
    pub min: Option<f64>,
    
    /// Maximum value (for numbers)
    pub max: Option<f64>,
    
    /// Minimum length (for strings/arrays)
    pub min_length: Option<usize>,
    
    /// Maximum length (for strings/arrays)
    pub max_length: Option<usize>,
    
    /// Regular expression pattern
    pub pattern: Option<String>,
    
    /// Custom validation function name
    pub custom_validator: Option<String>,
}

/// Schema definition for tool output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSchema {
    /// Expected output type
    pub output_type: OutputType,
    
    /// Schema definition (JSON Schema format)
    pub schema: Option<serde_json::Value>,
    
    /// Example outputs
    pub examples: Vec<serde_json::Value>,
}

/// Types of output a tool can produce.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputType {
    Text,
    Json,
    Binary,
    File,
    Stream,
    Void,
    Multiple,
}

/// Execution configuration for tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// Maximum execution time
    pub timeout: Duration,
    
    /// Retry configuration
    pub retry: RetryConfig,
    
    /// Resource limits
    pub resource_limits: ResourceLimits,
    
    /// Execution environment
    pub environment: ExecutionEnvironment,
    
    /// Concurrency settings
    pub concurrency: ConcurrencyConfig,
}

/// Retry configuration for failed executions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Backoff strategy
    pub backoff_strategy: BackoffStrategy,
    
    /// Which errors to retry
    pub retryable_errors: Vec<String>,
}

/// Backoff strategies for retries.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed { delay_ms: u64 },
    
    /// Exponential backoff
    Exponential { 
        initial_ms: u64,
        multiplier: f32,
        max_ms: u64,
    },
    
    /// Linear increase
    Linear { 
        initial_ms: u64,
        increment_ms: u64,
    },
}

/// Resource limits for tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    
    /// Maximum CPU time in milliseconds
    pub max_cpu_ms: Option<u64>,
    
    /// Maximum output size in bytes
    pub max_output_size: Option<u64>,
    
    /// Maximum number of files created
    pub max_files: Option<u32>,
    
    /// Network bandwidth limit
    pub network_bandwidth: Option<u64>,
}

/// Execution environment settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEnvironment {
    /// Environment variables
    pub variables: HashMap<String, String>,
    
    /// Working directory
    pub working_dir: Option<String>,
    
    /// User context
    pub user: Option<String>,
    
    /// Isolation level
    pub isolation: IsolationLevel,
}

/// Isolation levels for tool execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IsolationLevel {
    /// No isolation
    None,
    
    /// Process isolation
    Process,
    
    /// Container isolation
    Container,
    
    /// Full VM isolation
    VM,
}

/// Concurrency configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Maximum concurrent executions
    pub max_concurrent: u32,
    
    /// Queue size for pending executions
    pub queue_size: u32,
    
    /// Priority handling
    pub priority_enabled: bool,
}

/// Security configuration for tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Required permissions
    pub required_permissions: Vec<Permission>,
    
    /// Authentication requirements
    pub authentication: AuthRequirement,
    
    /// Audit logging settings
    pub audit: AuditConfig,
    
    /// Rate limiting
    pub rate_limits: RateLimits,
    
    /// Data handling policies
    pub data_policies: DataPolicies,
}

/// Permission types for tool access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Permission {
    /// Permission scope
    pub scope: String,
    
    /// Permission action
    pub action: String,
    
    /// Resource identifier
    pub resource: Option<String>,
}

/// Authentication requirements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthRequirement {
    /// No authentication required
    None,
    
    /// API key required
    ApiKey,
    
    /// OAuth token required
    OAuth,
    
    /// User session required
    UserSession,
    
    /// Custom authentication
    Custom,
}

/// Audit configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Log all executions
    pub log_executions: bool,
    
    /// Log input parameters
    pub log_inputs: bool,
    
    /// Log outputs
    pub log_outputs: bool,
    
    /// Retention period for logs
    pub retention_days: u32,
}

/// Rate limiting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Requests per minute
    pub per_minute: Option<u32>,
    
    /// Requests per hour
    pub per_hour: Option<u32>,
    
    /// Requests per day
    pub per_day: Option<u32>,
    
    /// Burst allowance
    pub burst_size: Option<u32>,
}

/// Data handling policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPolicies {
    /// Encrypt data in transit
    pub encrypt_transit: bool,
    
    /// Encrypt data at rest
    pub encrypt_rest: bool,
    
    /// Data retention policy
    pub retention: RetentionPolicy,
    
    /// PII handling
    pub pii_handling: PiiHandling,
}

/// Data retention policies.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RetentionPolicy {
    /// Keep forever
    Indefinite,
    
    /// Delete after duration
    Duration { days: u32 },
    
    /// Delete immediately after use
    Immediate,
}

/// PII handling strategies.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PiiHandling {
    /// Allow PII
    Allow,
    
    /// Redact PII
    Redact,
    
    /// Block operations with PII
    Block,
}

/// Tool availability status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolStatus {
    /// Tool is available
    Available,
    
    /// Tool is temporarily unavailable
    Unavailable,
    
    /// Tool is deprecated
    Deprecated,
    
    /// Tool is in maintenance
    Maintenance,
    
    /// Tool is disabled
    Disabled,
}

/// Usage tracking for tools.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolUsage {
    /// Total execution count
    pub total_executions: u64,
    
    /// Successful executions
    pub successful_executions: u64,
    
    /// Failed executions
    pub failed_executions: u64,
    
    /// Average execution time
    pub avg_execution_time_ms: f64,
    
    /// Last execution time
    pub last_execution: Option<DateTime<Utc>>,
    
    /// Usage by time period
    pub usage_by_period: HashMap<String, u64>,
}

/// Metadata for tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Tool version
    pub version: String,
    
    /// Author/creator
    pub author: String,
    
    /// Category tags
    pub tags: Vec<String>,
    
    /// Documentation URL
    pub documentation_url: Option<String>,
    
    /// Source code URL
    pub source_url: Option<String>,
    
    /// License information
    pub license: Option<String>,
    
    /// Custom metadata
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Execution metrics for tool runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionMetrics {
    /// Execution duration
    pub duration_ms: u64,
    
    /// Memory used in bytes
    pub memory_bytes: Option<u64>,
    
    /// CPU time used
    pub cpu_ms: Option<u64>,
    
    /// Network bytes transferred
    pub network_bytes: Option<u64>,
    
    /// Number of retries
    pub retry_count: u32,
    
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Diagnostic information from execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Diagnostic level
    pub level: DiagnosticLevel,
    
    /// Diagnostic code
    pub code: String,
    
    /// Message
    pub message: String,
    
    /// Source location
    pub location: Option<String>,
    
    /// Additional context
    pub context: HashMap<String, serde_json::Value>,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// File operations for filesystem tools.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileOperation {
    Read,
    Write,
    Delete,
    Create,
    Rename,
    Copy,
    Move,
    List,
}

/// HTTP methods for API tools.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// Database operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatabaseOperation {
    Select,
    Insert,
    Update,
    Delete,
    CreateTable,
    DropTable,
    Execute,
}

/// Sandbox configuration for code execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Allowed imports/libraries
    pub allowed_imports: Vec<String>,
    
    /// Blocked functions
    pub blocked_functions: Vec<String>,
    
    /// Filesystem access
    pub filesystem_access: bool,
    
    /// Network access
    pub network_access: bool,
    
    /// Maximum execution time
    pub max_execution_time_ms: u64,
}

impl Tool {
    /// Creates a new tool with basic configuration.
    pub fn new(name: String, description: String, tool_type: ToolType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            tool_type,
            parameters: Vec::new(),
            output_schema: OutputSchema {
                output_type: OutputType::Json,
                schema: None,
                examples: Vec::new(),
            },
            execution_config: ExecutionConfig::default(),
            security: SecurityConfig::default(),
            status: ToolStatus::Available,
            usage: ToolUsage::default(),
            metadata: ToolMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Adds a parameter to the tool.
    pub fn add_parameter(&mut self, parameter: ToolParameter) {
        self.parameters.push(parameter);
        self.updated_at = Utc::now();
    }
    
    /// Checks if the tool is available for use.
    pub fn is_available(&self) -> bool {
        self.status == ToolStatus::Available
    }
    
    /// Records a tool execution.
    pub fn record_execution(&mut self, result: &ToolResult) {
        self.usage.total_executions += 1;
        
        match result.status {
            ExecutionStatus::Success => self.usage.successful_executions += 1,
            ExecutionStatus::Failed => self.usage.failed_executions += 1,
            _ => {}
        }
        
        self.usage.last_execution = Some(Utc::now());
        
        // Update average execution time
        if let Some(duration) = result.metrics.duration_ms.checked_sub(0) {
            let total = self.usage.avg_execution_time_ms * (self.usage.total_executions - 1) as f64;
            self.usage.avg_execution_time_ms = (total + duration as f64) / self.usage.total_executions as f64;
        }
        
        self.updated_at = Utc::now();
    }
}

impl ToolResult {
    /// Creates a new successful tool result.
    pub fn success(tool_id: Uuid, output: ToolOutput) -> Self {
        let now = Utc::now();
        Self {
            execution_id: Uuid::new_v4(),
            tool_id,
            status: ExecutionStatus::Success,
            output,
            metrics: ExecutionMetrics::default(),
            diagnostics: Vec::new(),
            started_at: now,
            completed_at: Some(now),
        }
    }
    
    /// Creates a new failed tool result.
    pub fn failure(tool_id: Uuid, error: String) -> Self {
        let now = Utc::now();
        Self {
            execution_id: Uuid::new_v4(),
            tool_id,
            status: ExecutionStatus::Failed,
            output: ToolOutput::Error {
                code: "EXECUTION_FAILED".to_string(),
                message: error,
                details: None,
            },
            metrics: ExecutionMetrics::default(),
            diagnostics: vec![
                Diagnostic {
                    level: DiagnosticLevel::Error,
                    code: "EXECUTION_FAILED".to_string(),
                    message: error,
                    location: None,
                    context: HashMap::new(),
                }
            ],
            started_at: now,
            completed_at: Some(now),
        }
    }
    
    /// Checks if the execution was successful.
    pub fn is_success(&self) -> bool {
        matches!(self.status, ExecutionStatus::Success | ExecutionStatus::PartialSuccess)
    }
    
    /// Gets the execution duration if completed.
    pub fn duration(&self) -> Option<Duration> {
        self.completed_at.map(|end| end.signed_duration_since(self.started_at))
    }
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::minutes(5),
            retry: RetryConfig {
                max_attempts: 3,
                backoff_strategy: BackoffStrategy::Exponential {
                    initial_ms: 1000,
                    multiplier: 2.0,
                    max_ms: 30000,
                },
                retryable_errors: vec![
                    "TIMEOUT".to_string(),
                    "NETWORK_ERROR".to_string(),
                ],
            },
            resource_limits: ResourceLimits {
                max_memory: Some(512 * 1024 * 1024), // 512MB
                max_cpu_ms: Some(60000), // 1 minute
                max_output_size: Some(10 * 1024 * 1024), // 10MB
                max_files: Some(100),
                network_bandwidth: None,
            },
            environment: ExecutionEnvironment {
                variables: HashMap::new(),
                working_dir: None,
                user: None,
                isolation: IsolationLevel::Process,
            },
            concurrency: ConcurrencyConfig {
                max_concurrent: 10,
                queue_size: 100,
                priority_enabled: false,
            },
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            required_permissions: Vec::new(),
            authentication: AuthRequirement::None,
            audit: AuditConfig {
                log_executions: true,
                log_inputs: false,
                log_outputs: false,
                retention_days: 30,
            },
            rate_limits: RateLimits {
                per_minute: Some(60),
                per_hour: Some(1000),
                per_day: Some(10000),
                burst_size: Some(10),
            },
            data_policies: DataPolicies {
                encrypt_transit: true,
                encrypt_rest: false,
                retention: RetentionPolicy::Duration { days: 30 },
                pii_handling: PiiHandling::Allow,
            },
        }
    }
}

impl Default for ToolMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            author: "system".to_string(),
            tags: Vec::new(),
            documentation_url: None,
            source_url: None,
            license: None,
            custom_fields: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_creation() {
        let tool = Tool::new(
            "File Reader".to_string(),
            "Reads content from files".to_string(),
            ToolType::FileSystem {
                allowed_operations: vec![FileOperation::Read],
                allowed_paths: vec!["/tmp".to_string()],
            },
        );
        
        assert_eq!(tool.name, "File Reader");
        assert!(tool.is_available());
        assert_eq!(tool.parameters.len(), 0);
        assert_eq!(tool.usage.total_executions, 0);
    }
    
    #[test]
    fn test_tool_parameter() {
        let mut tool = Tool::new(
            "Test Tool".to_string(),
            "Description".to_string(),
            ToolType::Custom {
                category: "test".to_string(),
                capabilities: HashMap::new(),
            },
        );
        
        let param = ToolParameter {
            name: "input_file".to_string(),
            description: "Path to input file".to_string(),
            param_type: ParameterType::FilePath,
            required: true,
            default_value: None,
            validation: Some(ValidationRules {
                min: None,
                max: None,
                min_length: Some(1),
                max_length: Some(255),
                pattern: Some(r"^[a-zA-Z0-9./\-_]+$".to_string()),
                custom_validator: None,
            }),
        };
        
        tool.add_parameter(param);
        
        assert_eq!(tool.parameters.len(), 1);
        assert_eq!(tool.parameters[0].name, "input_file");
        assert!(tool.parameters[0].required);
    }
    
    #[test]
    fn test_tool_result_success() {
        let tool_id = Uuid::new_v4();
        let result = ToolResult::success(
            tool_id,
            ToolOutput::Text("Operation completed".to_string()),
        );
        
        assert!(result.is_success());
        assert_eq!(result.status, ExecutionStatus::Success);
        assert!(matches!(result.output, ToolOutput::Text(_)));
        assert!(result.completed_at.is_some());
    }
    
    #[test]
    fn test_tool_result_failure() {
        let tool_id = Uuid::new_v4();
        let result = ToolResult::failure(
            tool_id,
            "Network timeout".to_string(),
        );
        
        assert!(!result.is_success());
        assert_eq!(result.status, ExecutionStatus::Failed);
        assert!(matches!(result.output, ToolOutput::Error { .. }));
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].level, DiagnosticLevel::Error);
    }
    
    #[test]
    fn test_record_execution() {
        let mut tool = Tool::new(
            "Test".to_string(),
            "Test tool".to_string(),
            ToolType::Custom {
                category: "test".to_string(),
                capabilities: HashMap::new(),
            },
        );
        
        let result = ToolResult::success(
            tool.id,
            ToolOutput::Void,
        );
        
        tool.record_execution(&result);
        
        assert_eq!(tool.usage.total_executions, 1);
        assert_eq!(tool.usage.successful_executions, 1);
        assert_eq!(tool.usage.failed_executions, 0);
        assert!(tool.usage.last_execution.is_some());
    }
}