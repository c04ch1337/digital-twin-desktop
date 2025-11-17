//! Domain trait definitions for the Digital Twin Desktop application.
//!
//! This module exports all core domain traits that define contracts
//! for infrastructure implementations, ensuring clean separation
//! between domain logic and external concerns.

pub mod llm_client;
pub mod repository;
pub mod tool_executor;

// Re-export all repository traits
pub use repository::{
    AgentRepository, ConversationRepository, FilterCriteria, FilterOperator,
    PaginatedResult, Pagination, RepositoryError, RepositoryFactory,
    RepositoryResult, SensorDataRepository, SortCriteria, SortOrder,
    ToolRepository, Transaction, TwinRepository, UnitOfWork,
};

// Re-export LLM client traits and types
pub use llm_client::{
    ChatChoice, ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse,
    ChatCompletionStream, ChatMessage, ChatMessageDelta, CompletionChunk,
    CompletionChoice, CompletionRequest, CompletionResponse, CompletionStream,
    ConversationContext, Embedding, EmbeddingRequest, EmbeddingResponse,
    EmbeddingUsage, EncodingFormat, FinishReason, FunctionCall, FunctionCallDelta,
    FunctionDefinition, LLMClient, LLMClientConfig, LLMClientFactory, LLMError,
    LLMResult, LogProbs, MessageRole, ModelInfo, ModelPricing, ModelType, ModelUsage,
    ResponseFormat, ResponseFormatType, ToolCall, ToolCallDelta, ToolChoice,
    ToolChoiceFunction, ToolDefinition, TokenUsage, UsagePeriod, UsageStats,
};

// Re-export tool executor traits and types
pub use tool_executor::{
    ChunkData, ExecutionChunk, ExecutionContext, ExecutionOptions, ExecutionPriority,
    ExecutionRequest, ExecutionStatusInfo, ExecutionStream, ExecutorError,
    ExecutorResult, LogLevel, OutputPreferences, PartialResult, PreferredFormat,
    ResourceLimits, ResourceRequirements, ResourceUsage, RetryConfig as ExecutorRetryConfig,
    SecurityContext, ToolExecutor, ToolExecutorFactory, ToolExecutorRegistry,
    ToolInfo, ValidationError, ValidationResult, ValidationWarning,
};

/// Type alias for async trait results
pub type AsyncResult<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;