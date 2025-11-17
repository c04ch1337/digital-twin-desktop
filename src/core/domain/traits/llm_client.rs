//! LLM (Large Language Model) client trait definitions.
//!
//! This module defines the interface for interacting with AI language models,
//! abstracting the specific implementation details of different providers
//! (OpenAI, Anthropic, local models, etc.).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::domain::models::{AgentId, ConversationId, MessageId};

/// Result type for LLM operations
pub type LLMResult<T> = Result<T, LLMError>;

/// Errors that can occur during LLM operations
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    /// Authentication or authorization error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded: {message}")]
    RateLimitError { 
        message: String,
        retry_after: Option<u64>, // seconds
    },
    
    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Model not found or not available
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    
    /// Token limit exceeded
    #[error("Token limit exceeded: used {used}, limit {limit}")]
    TokenLimitExceeded { used: u32, limit: u32 },
    
    /// Network or connection error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Timeout error
    #[error("Request timeout after {0} seconds")]
    Timeout(u64),
    
    /// Response parsing error
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    
    /// Content filter triggered
    #[error("Content filtered: {0}")]
    ContentFiltered(String),
    
    /// Provider-specific error
    #[error("Provider error: {provider} - {message}")]
    ProviderError { provider: String, message: String },
    
    /// Other errors
    #[error("LLM error: {0}")]
    Other(String),
}

/// Main trait for LLM client implementations
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Generate a completion for a single prompt
    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> LLMResult<CompletionResponse>;
    
    /// Generate a chat completion
    async fn chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<ChatCompletionResponse>;
    
    /// Stream a completion response
    async fn stream_complete(
        &self,
        request: CompletionRequest,
    ) -> LLMResult<Box<dyn CompletionStream>>;
    
    /// Stream a chat completion response
    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>>;
    
    /// Generate embeddings for text
    async fn embed(
        &self,
        request: EmbeddingRequest,
    ) -> LLMResult<EmbeddingResponse>;
    
    /// List available models
    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>>;
    
    /// Get information about a specific model
    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo>;
    
    /// Validate API credentials
    async fn validate_credentials(&self) -> LLMResult<bool>;
    
    /// Get usage statistics
    async fn get_usage(&self, period: UsagePeriod) -> LLMResult<UsageStats>;
    
    /// Cancel an ongoing request
    async fn cancel_request(&self, request_id: &str) -> LLMResult<()>;
}

/// Request for text completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// Unique request ID for tracking
    pub request_id: String,
    
    /// The prompt to complete
    pub prompt: String,
    
    /// Model identifier
    pub model: String,
    
    /// Sampling temperature (0.0 to 2.0)
    pub temperature: Option<f32>,
    
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    
    /// Frequency penalty (-2.0 to 2.0)
    pub frequency_penalty: Option<f32>,
    
    /// Presence penalty (-2.0 to 2.0)
    pub presence_penalty: Option<f32>,
    
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    
    /// Number of completions to generate
    pub n: Option<u32>,
    
    /// User identifier for tracking
    pub user_id: Option<String>,
    
    /// Additional provider-specific parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

/// Response from text completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Request ID this response corresponds to
    pub request_id: String,
    
    /// Generated completions
    pub choices: Vec<CompletionChoice>,
    
    /// Token usage information
    pub usage: TokenUsage,
    
    /// Model used
    pub model: String,
    
    /// Response timestamp
    pub created_at: DateTime<Utc>,
    
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// A single completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    /// The generated text
    pub text: String,
    
    /// Choice index
    pub index: u32,
    
    /// Log probabilities (if requested)
    pub logprobs: Option<LogProbs>,
    
    /// Reason the completion stopped
    pub finish_reason: FinishReason,
}

/// Request for chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// Unique request ID
    pub request_id: String,
    
    /// Chat messages
    pub messages: Vec<ChatMessage>,
    
    /// Model identifier
    pub model: String,
    
    /// Sampling parameters (same as CompletionRequest)
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
    pub n: Option<u32>,
    
    /// System message override
    pub system: Option<String>,
    
    /// Function/tool definitions
    pub tools: Option<Vec<ToolDefinition>>,
    
    /// Force specific tool use
    pub tool_choice: Option<ToolChoice>,
    
    /// Response format
    pub response_format: Option<ResponseFormat>,
    
    /// User identifier
    pub user_id: Option<String>,
    
    /// Context from conversation
    pub conversation_context: Option<ConversationContext>,
    
    /// Additional parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

/// Response from chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// Request ID
    pub request_id: String,
    
    /// Generated choices
    pub choices: Vec<ChatChoice>,
    
    /// Token usage
    pub usage: TokenUsage,
    
    /// Model used
    pub model: String,
    
    /// Response timestamp
    pub created_at: DateTime<Utc>,
    
    /// Processing time
    pub processing_time_ms: u64,
}

/// A single chat completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    /// The message generated
    pub message: ChatMessage,
    
    /// Choice index
    pub index: u32,
    
    /// Finish reason
    pub finish_reason: FinishReason,
    
    /// Tool calls made
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Chat message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role
    pub role: MessageRole,
    
    /// Message content
    pub content: Option<String>,
    
    /// Name of the speaker (for multi-party conversations)
    pub name: Option<String>,
    
    /// Tool call ID (for tool responses)
    pub tool_call_id: Option<String>,
    
    /// Tool calls (for assistant messages)
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Message roles in chat
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
    Function, // Legacy, replaced by Tool
}

/// Tool/function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool type (usually "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    
    /// Function definition
    pub function: FunctionDefinition,
}

/// Function definition for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// Function name
    pub name: String,
    
    /// Function description
    pub description: String,
    
    /// Parameters as JSON Schema
    pub parameters: serde_json::Value,
    
    /// Whether the function is required
    pub required: Option<bool>,
}

/// Tool choice options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// No tool use
    None,
    
    /// Automatic tool selection
    Auto,
    
    /// Require tool use
    Required,
    
    /// Force specific tool
    Specific { 
        #[serde(rename = "type")]
        tool_type: String,
        function: ToolChoiceFunction,
    },
}

/// Specific tool choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    pub name: String,
}

/// Tool call made by the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique ID for this tool call
    pub id: String,
    
    /// Tool type
    #[serde(rename = "type")]
    pub tool_type: String,
    
    /// Function call details
    pub function: FunctionCall,
}

/// Function call details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name
    pub name: String,
    
    /// Arguments as JSON string
    pub arguments: String,
}

/// Response format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Format type
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
    
    /// JSON schema (if type is json_schema)
    pub json_schema: Option<serde_json::Value>,
}

/// Response format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Text,
    JsonObject,
    JsonSchema,
}

/// Reason why generation stopped
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    Null,
}

/// Token usage information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input/prompt tokens
    pub prompt_tokens: u32,
    
    /// Output/completion tokens
    pub completion_tokens: u32,
    
    /// Total tokens
    pub total_tokens: u32,
    
    /// Cached tokens (if applicable)
    pub cached_tokens: Option<u32>,
}

/// Log probability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbs {
    /// Token log probabilities
    pub tokens: Vec<String>,
    
    /// Log probabilities for each token
    pub token_logprobs: Vec<Option<f32>>,
    
    /// Top log probabilities
    pub top_logprobs: Option<Vec<HashMap<String, f32>>>,
    
    /// Text offsets
    pub text_offset: Vec<usize>,
}

/// Embedding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Texts to embed
    pub input: Vec<String>,
    
    /// Model to use
    pub model: String,
    
    /// Encoding format
    pub encoding_format: Option<EncodingFormat>,
    
    /// User identifier
    pub user_id: Option<String>,
}

/// Embedding response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Generated embeddings
    pub data: Vec<Embedding>,
    
    /// Model used
    pub model: String,
    
    /// Token usage
    pub usage: EmbeddingUsage,
}

/// Single embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// Embedding vector
    pub embedding: Vec<f32>,
    
    /// Index in input array
    pub index: usize,
}

/// Embedding usage information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    /// Input tokens
    pub prompt_tokens: u32,
    
    /// Total tokens
    pub total_tokens: u32,
}

/// Encoding format for embeddings
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    Float,
    Base64,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,
    
    /// Model display name
    pub name: String,
    
    /// Model provider
    pub provider: String,
    
    /// Model type
    pub model_type: ModelType,
    
    /// Maximum context length
    pub max_tokens: u32,
    
    /// Maximum output tokens
    pub max_output_tokens: Option<u32>,
    
    /// Supports streaming
    pub supports_streaming: bool,
    
    /// Supports function calling
    pub supports_functions: bool,
    
    /// Supports vision/images
    pub supports_vision: bool,
    
    /// Cost per token (if available)
    pub pricing: Option<ModelPricing>,
    
    /// Model capabilities
    pub capabilities: Vec<String>,
    
    /// Deprecation date
    pub deprecated_at: Option<DateTime<Utc>>,
}

/// Model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Chat,
    Completion,
    Embedding,
    Image,
    Audio,
    MultiModal,
}

/// Model pricing information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModelPricing {
    /// Cost per 1K input tokens
    pub input_cost_per_1k: f64,
    
    /// Cost per 1K output tokens
    pub output_cost_per_1k: f64,
    
    /// Currency (USD, EUR, etc.)
    pub currency: &'static str,
}

/// Conversation context for maintaining state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Conversation ID
    pub conversation_id: ConversationId,
    
    /// Agent ID
    pub agent_id: AgentId,
    
    /// Parent message ID
    pub parent_message_id: Option<MessageId>,
    
    /// Additional context data
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Usage period for statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UsagePeriod {
    Today,
    ThisWeek,
    ThisMonth,
    Last30Days,
    Custom { 
        start: DateTime<Utc>, 
        end: DateTime<Utc> 
    },
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// Period these stats cover
    pub period: UsagePeriod,
    
    /// Total requests made
    pub total_requests: u64,
    
    /// Successful requests
    pub successful_requests: u64,
    
    /// Failed requests
    pub failed_requests: u64,
    
    /// Total tokens used
    pub total_tokens: TokenUsage,
    
    /// Total cost
    pub total_cost: f64,
    
    /// Cost currency
    pub currency: String,
    
    /// Usage by model
    pub usage_by_model: HashMap<String, ModelUsage>,
    
    /// Average response time
    pub avg_response_time_ms: f64,
}

/// Usage for a specific model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsage {
    /// Number of requests
    pub requests: u64,
    
    /// Tokens used
    pub tokens: TokenUsage,
    
    /// Cost for this model
    pub cost: f64,
}

/// Stream for completion responses
#[async_trait]
pub trait CompletionStream: Send {
    /// Get the next chunk
    async fn next_chunk(&mut self) -> Option<LLMResult<CompletionChunk>>;
    
    /// Cancel the stream
    async fn cancel(&mut self) -> LLMResult<()>;
}

/// Stream for chat completion responses
#[async_trait]
pub trait ChatCompletionStream: Send {
    /// Get the next chunk
    async fn next_chunk(&mut self) -> Option<LLMResult<ChatCompletionChunk>>;
    
    /// Cancel the stream
    async fn cancel(&mut self) -> LLMResult<()>;
}

/// Streamed completion chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChunk {
    /// Chunk text
    pub text: String,
    
    /// Finish reason (if this is the last chunk)
    pub finish_reason: Option<FinishReason>,
    
    /// Chunk index
    pub index: u32,
}

/// Streamed chat completion chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// Delta content
    pub delta: ChatMessageDelta,
    
    /// Finish reason (if last chunk)
    pub finish_reason: Option<FinishReason>,
    
    /// Choice index
    pub index: u32,
}

/// Delta for streaming chat messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageDelta {
    /// Role (only in first chunk)
    pub role: Option<MessageRole>,
    
    /// Content delta
    pub content: Option<String>,
    
    /// Tool calls delta
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

/// Delta for streaming tool calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Tool call index
    pub index: usize,
    
    /// Tool call ID (first chunk only)
    pub id: Option<String>,
    
    /// Tool type (first chunk only)
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    
    /// Function delta
    pub function: Option<FunctionCallDelta>,
}

/// Delta for streaming function calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallDelta {
    /// Function name (first chunk only)
    pub name: Option<String>,
    
    /// Arguments delta
    pub arguments: Option<String>,
}

/// Factory for creating LLM clients
#[async_trait]
pub trait LLMClientFactory: Send + Sync {
    /// Create an LLM client for the specified provider
    async fn create_client(
        &self,
        provider: &str,
        config: LLMClientConfig,
    ) -> LLMResult<Box<dyn LLMClient>>;
    
    /// List available providers
    fn available_providers(&self) -> Vec<String>;
}

/// Configuration for LLM clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMClientConfig {
    /// API key or credentials
    pub api_key: Option<String>,
    
    /// Base URL override
    pub base_url: Option<String>,
    
    /// Organization ID (for OpenAI)
    pub organization_id: Option<String>,
    
    /// Default model to use
    pub default_model: Option<String>,
    
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
    
    /// Maximum retries
    pub max_retries: Option<u32>,
    
    /// Custom headers
    pub headers: HashMap<String, String>,
    
    /// Proxy configuration
    pub proxy: Option<String>,
    
    /// Provider-specific configuration
    pub extra_config: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_completion_request_creation() {
        let request = CompletionRequest {
            request_id: "test-123".to_string(),
            prompt: "Hello, world!".to_string(),
            model: "gpt-4".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(100),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: None,
            n: None,
            user_id: None,
            extra_params: HashMap::new(),
        };
        
        assert_eq!(request.prompt, "Hello, world!");
        assert_eq!(request.temperature, Some(0.7));
    }
    
    #[test]
    fn test_message_role_serialization() {
        let role = MessageRole::Assistant;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"assistant\"");
        
        let deserialized: MessageRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, MessageRole::Assistant);
    }
    
    #[test]
    fn test_token_usage() {
        let usage = TokenUsage {
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
            cached_tokens: Some(5),
        };
        
        assert_eq!(usage.prompt_tokens + usage.completion_tokens, usage.total_tokens);
    }
}