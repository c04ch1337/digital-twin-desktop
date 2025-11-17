//! Domain models for the Digital Twin Desktop application.
//!
//! This module exports all core domain entities that form the heart of
//! the business logic, including agents, conversations, digital twins,
//! sensor data, and tools.

pub mod agent;
pub mod conversation;
pub mod digital_twin;
pub mod sensor_data;
pub mod tool;

// Re-export commonly used types for convenience
pub use agent::{
    Agent, AgentCapability, AgentConfiguration, AgentContext, AgentMetadata,
    AgentMetrics, AgentState, CapabilityType, LongTermMemoryConfig, MemoryConfiguration,
    MemoryType, RateLimitConfig, ResponseFormat,
};

pub use conversation::{
    Attachment, AttachmentType, ContentType, Conversation, ConversationMetadata,
    ConversationPriority, ConversationState, Message, MessageMetadata, MessageSender,
};

pub use digital_twin::{
    Anomaly, ChartConfig, ConflictStrategy, ConnectionConfig, DashboardLayout,
    DataMapping, DataSource, DataSourceType, DataType, DigitalTwin, Measurement,
    Model3DConfig, RetentionPolicy, RetryConfig as TwinRetryConfig, SyncConfiguration,
    SyncMode, TransformRule, TwinAnalytics, TwinMetadata, TwinProperties, TwinState,
    TwinType, ViewType, VisualizationConfig, WidgetConfig, WidgetLayout,
};

pub use sensor_data::{
    AggregationConfig, AggregationMethod, AlertSeverity, AlertType, AnomalyAlgorithm,
    AnomalyDetectionConfig, CalibrationInfo, DataQualityMetrics, Diagnostic,
    DiagnosticLevel, EnergyMeasurementType, EnvironmentalContext, FilterConfig,
    FilterType, FlowUnit, FrequencyRange, IssueSeverity, LightSpectrum, ManufacturerInfo,
    Measurement as SensorMeasurement, NoiseLevel, PressureUnit, ProcessingConfig,
    QualityIndicators, QualityIssue, QualityIssueType, ReadingContext, ReadingQuality,
    SensorAlert, SensorData, SensorDataMetadata, SensorInfo, SensorLocation,
    SensorReading, SensorSpecifications, SensorStatistics, SensorStatus, SensorType,
    SensorValue, TemperatureUnit, ThresholdDirection, ThresholdInfo, ThresholdType,
    TimeStatistics, TransformationRule, TransformationType,
};

pub use tool::{
    AuditConfig, AuthRequirement, BackoffStrategy, ConcurrencyConfig, DataPolicies,
    DatabaseOperation, Diagnostic as ToolDiagnostic, DiagnosticLevel as ToolDiagnosticLevel,
    ExecutionConfig, ExecutionEnvironment, ExecutionMetrics, ExecutionStatus, FileOperation,
    HttpMethod, IsolationLevel, OutputSchema, OutputType, ParameterType, Permission,
    PiiHandling, RateLimits, ResourceLimits, RetentionPolicy as ToolRetentionPolicy,
    RetryConfig as ToolRetryConfig, SandboxConfig, SecurityConfig, Tool, ToolMetadata,
    ToolOutput, ToolParameter, ToolResult, ToolStatus, ToolType, ToolUsage, ValidationRules,
};

// Type aliases for clarity and future flexibility
pub type ConversationId = uuid::Uuid;
pub type MessageId = uuid::Uuid;
pub type AgentId = uuid::Uuid;
pub type TwinId = uuid::Uuid;
pub type SensorDataId = uuid::Uuid;
pub type ToolId = uuid::Uuid;
pub type ExecutionId = uuid::Uuid;

/// Common result type for domain operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;