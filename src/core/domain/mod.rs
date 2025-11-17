//! Core domain layer for the Digital Twin Desktop application.
//!
//! This module contains all domain entities, value objects, traits,
//! and business logic that form the heart of the application.
//! The domain layer is independent of external concerns and frameworks.

pub mod errors;
pub mod models;
pub mod traits;
pub mod value_objects;

// Re-export commonly used types for convenience
pub use errors::{
    AuthorizationError, BusinessRuleError, ConflictError, DomainError, DomainResult,
    ErrorExt, NotFoundError, ResourceConstraintError, StateTransitionError, ValidationError,
};

pub use models::{
    // Agent types
    Agent, AgentCapability, AgentConfiguration, AgentContext, AgentMetadata,
    AgentMetrics, AgentState, CapabilityType, LongTermMemoryConfig, MemoryConfiguration,
    MemoryType, RateLimitConfig, ResponseFormat,
    
    // Conversation types
    Attachment, AttachmentType, ContentType, Conversation, ConversationMetadata,
    ConversationPriority, ConversationState, Message, MessageMetadata, MessageSender,
    
    // Digital Twin types
    Anomaly, ChartConfig, ConflictStrategy, ConnectionConfig, DashboardLayout,
    DataMapping, DataSource, DataSourceType, DataType, DigitalTwin, Measurement,
    Model3DConfig, RetentionPolicy, RetryConfig as TwinRetryConfig, SyncConfiguration,
    SyncMode, TransformRule, TwinAnalytics, TwinMetadata, TwinProperties, TwinState,
    TwinType, ViewType, VisualizationConfig, WidgetConfig, WidgetLayout,
    
    // Sensor Data types
    AggregationConfig, AggregationMethod, AlertSeverity, AlertType, AnomalyAlgorithm,
    AnomalyDetectionConfig, CalibrationInfo, DataQualityMetrics, Diagnostic,
    DiagnosticLevel, EnergyMeasurementType, EnvironmentalContext, FilterConfig,
    FilterType, FlowUnit, FrequencyRange, IssueSeverity, LightSpectrum, ManufacturerInfo,
    NoiseLevel, PressureUnit, ProcessingConfig, QualityIndicators, QualityIssue,
    QualityIssueType, ReadingContext, ReadingQuality, SensorAlert, SensorData,
    SensorDataMetadata, SensorInfo, SensorLocation, SensorReading, SensorSpecifications,
    SensorStatistics, SensorStatus, SensorType, SensorValue, TemperatureUnit,
    ThresholdDirection, ThresholdInfo, ThresholdType, TimeStatistics, TransformationRule,
    TransformationType,
    
    // Tool types
    AuditConfig, AuthRequirement, BackoffStrategy, ConcurrencyConfig, DataPolicies,
    DatabaseOperation, ExecutionConfig, ExecutionEnvironment, ExecutionMetrics,
    ExecutionStatus, FileOperation, HttpMethod, IsolationLevel, OutputSchema, OutputType,
    ParameterType, Permission, PiiHandling, RateLimits, ResourceLimits,
    RetentionPolicy as ToolRetentionPolicy, RetryConfig as ToolRetryConfig, SandboxConfig,
    SecurityConfig, Tool, ToolMetadata, ToolOutput, ToolParameter, ToolResult, ToolStatus,
    ToolType, ToolUsage, ValidationRules,
    
    // ID type aliases
    AgentId, ConversationId, ExecutionId, MessageId, SensorDataId, ToolId, TwinId,
};

pub use traits::{
    // Repository traits
    AgentRepository, ConversationRepository, FilterCriteria, FilterOperator,
    PaginatedResult, Pagination, RepositoryError, RepositoryFactory,
    RepositoryResult, SensorDataRepository, SortCriteria, SortOrder,
    ToolRepository, Transaction, TwinRepository, UnitOfWork,
    
    // LLM Client traits
    ChatChoice, ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse,
    ChatCompletionStream, ChatMessage, ChatMessageDelta, CompletionChunk,
    CompletionChoice, CompletionRequest, CompletionResponse, CompletionStream,
    ConversationContext, Embedding, EmbeddingRequest, EmbeddingResponse,
    EmbeddingUsage, EncodingFormat, FinishReason, FunctionCall, FunctionCallDelta,
    FunctionDefinition, LLMClient, LLMClientConfig, LLMClientFactory, LLMError,
    LLMResult, LogProbs, MessageRole, ModelInfo, ModelPricing, ModelType, ModelUsage,
    ResponseFormat as LLMResponseFormat, ResponseFormatType, ToolCall, ToolCallDelta,
    ToolChoice, ToolChoiceFunction, ToolDefinition, TokenUsage, UsagePeriod, UsageStats,
    
    // Tool Executor traits
    ChunkData, ExecutionChunk, ExecutionContext, ExecutionOptions, ExecutionPriority,
    ExecutionRequest, ExecutionStatusInfo, ExecutionStream, ExecutorError,
    ExecutorResult, LogLevel, OutputPreferences, PartialResult, PreferredFormat,
    ResourceLimits as ExecutorResourceLimits, ResourceRequirements, ResourceUsage,
    RetryConfig as ExecutorRetryConfig, SecurityContext, ToolExecutor,
    ToolExecutorFactory, ToolExecutorRegistry, ToolInfo, ValidationError as ExecutorValidationError,
    ValidationResult, ValidationWarning,
};

pub use value_objects::{
    DataSize, Email, Percentage, Temperature, TimeWindow, Url, Version,
};

// Re-export the ID value objects from value_objects (they were defined there)
pub use value_objects::{
    AgentId as ValueAgentId, ConversationId as ValueConversationId,
    ExecutionId as ValueExecutionId, MessageId as ValueMessageId,
    SensorDataId as ValueSensorDataId, ToolId as ValueToolId, TwinId as ValueTwinId,
};