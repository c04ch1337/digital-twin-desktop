use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::core::domain::traits::llm_client::{
    LLMClient, LLMResult, LLMError, CompletionRequest, CompletionResponse,
    ChatCompletionRequest, ChatCompletionResponse, CompletionStream,
    ChatCompletionStream, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    UsagePeriod, UsageStats, LLMClientConfig,
};

const API_URL: &str = "https://api.openai.com/v1";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// OpenAI client implementation
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    organization_id: Option<String>,
    default_model: String,
}

impl OpenAIClient {
    /// Create a new OpenAI client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let api_key = config.api_key.ok_or_else(|| LLMError::AuthError(
            "OpenAI API key is required".to_string()
        ))?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|e| LLMError::AuthError(e.to_string()))?
        );

        if let Some(org_id) = &config.organization_id {
            headers.insert(
                "OpenAI-Organization",
                header::HeaderValue::from_str(org_id)
                    .map_err(|e| LLMError::AuthError(e.to_string()))?
            );
        }

        let client = Client::builder()
            .timeout(config.timeout_seconds.map(Duration::from_secs).unwrap_or(DEFAULT_TIMEOUT))
            .default_headers(headers)
            .build()
            .map_err(|e| LLMError::Other(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            organization_id: config.organization_id,
            default_model: config.default_model.unwrap_or_else(|| "gpt-4".to_string()),
        })
    }
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let url = format!("{}/completions", API_URL);
        
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let request_body = serde_json::json!({
            "model": model,
            "prompt": request.prompt,
            "max_tokens": request.max_tokens.unwrap_or(100),
            "temperature": request.temperature.unwrap_or(0.7),
            "top_p": request.top_p.unwrap_or(1.0),
            "frequency_penalty": request.frequency_penalty.unwrap_or(0.0),
            "presence_penalty": request.presence_penalty.unwrap_or(0.0),
            "stop": request.stop_sequences,
            "n": request.n.unwrap_or(1),
            "user": request.user_id,
        });

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "OpenAI".to_string(),
                message: error,
            });
        }

        let completion: OpenAICompletionResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(CompletionResponse {
            request_id: request.request_id,
            choices: completion.choices.into_iter().map(|c| c.into()).collect(),
            usage: completion.usage.into(),
            model: model.to_string(),
            created_at: chrono::Utc::now(),
            processing_time_ms: 0,
        })
    }

    async fn chat_complete(&self, request: ChatCompletionRequest) -> LLMResult<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", API_URL);

        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let messages: Vec<OpenAIChatMessage> = request.messages.iter().map(|m| OpenAIChatMessage {
            role: m.role.into(),
            content: m.content.clone(),
            name: m.name.clone(),
            function_call: None,
        }).collect();

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(100),
            "temperature": request.temperature.unwrap_or(0.7),
            "top_p": request.top_p.unwrap_or(1.0),
            "frequency_penalty": request.frequency_penalty.unwrap_or(0.0),
            "presence_penalty": request.presence_penalty.unwrap_or(0.0),
            "stop": request.stop_sequences,
            "n": request.n.unwrap_or(1),
            "user": request.user_id,
            "functions": request.tools.map(|tools| {
                tools.into_iter().map(|t| t.function).collect::<Vec<_>>()
            }),
            "function_call": request.tool_choice,
        });

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "OpenAI".to_string(),
                message: error,
            });
        }

        let chat_response: OpenAIChatResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(ChatCompletionResponse {
            request_id: request.request_id,
            choices: chat_response.choices.into_iter().map(|c| c.into()).collect(),
            usage: chat_response.usage.into(),
            model: model.to_string(),
            created_at: chrono::Utc::now(),
            processing_time_ms: 0,
        })
    }

    async fn stream_complete(
        &self,
        request: CompletionRequest,
    ) -> LLMResult<Box<dyn CompletionStream>> {
        // Implement streaming completion
        todo!("Implement streaming completion for OpenAI")
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        // Implement streaming chat completion
        todo!("Implement streaming chat completion for OpenAI")
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        let url = format!("{}/embeddings", API_URL);

        let request_body = serde_json::json!({
            "model": request.model,
            "input": request.input,
            "user": request.user_id,
        });

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "OpenAI".to_string(),
                message: error,
            });
        }

        let embedding: OpenAIEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(EmbeddingResponse {
            data: embedding.data.into_iter().map(|e| e.into()).collect(),
            model: embedding.model,
            usage: embedding.usage.into(),
        })
    }

    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>> {
        let url = format!("{}/models", API_URL);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "OpenAI".to_string(),
                message: error,
            });
        }

        let models: OpenAIModelList = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(models.data.into_iter().map(|m| m.into()).collect())
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let url = format!("{}/models/{}", API_URL, model_id);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "OpenAI".to_string(),
                message: error,
            });
        }

        let model: OpenAIModel = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(model.into())
    }

    async fn validate_credentials(&self) -> LLMResult<bool> {
        match self.list_models().await {
            Ok(_) => Ok(true),
            Err(LLMError::AuthError(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_usage(&self, period: UsagePeriod) -> LLMResult<UsageStats> {
        // OpenAI doesn't provide usage stats API
        Err(LLMError::ProviderError {
            provider: "OpenAI".to_string(),
            message: "Usage statistics are not supported by OpenAI".to_string(),
        })
    }

    async fn cancel_request(&self, request_id: &str) -> LLMResult<()> {
        // OpenAI doesn't support canceling requests
        Err(LLMError::ProviderError {
            provider: "OpenAI".to_string(),
            message: "Request cancellation is not supported by OpenAI".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionResponse {
    id: String,
    choices: Vec<OpenAICompletionChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionChoice {
    text: String,
    index: u32,
    logprobs: Option<OpenAILogProbs>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    choices: Vec<OpenAIChatChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIChatMessage {
    role: String,
    content: Option<String>,
    name: Option<String>,
    function_call: Option<OpenAIFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatChoice {
    message: OpenAIChatMessage,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenAILogProbs {
    tokens: Vec<String>,
    token_logprobs: Vec<Option<f32>>,
    top_logprobs: Option<Vec<HashMap<String, f32>>>,
    text_offset: Vec<usize>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbedding>,
    model: String,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbedding {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Debug, Deserialize)]
struct OpenAIModelList {
    data: Vec<OpenAIModel>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModel {
    id: String,
    owned_by: String,
    permission: Vec<OpenAIPermission>,
}

#[derive(Debug, Deserialize)]
struct OpenAIPermission {
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: String,
    group: Option<String>,
    is_blocking: bool,
}

impl From<OpenAICompletionChoice> for CompletionChoice {
    fn from(choice: OpenAICompletionChoice) -> Self {
        Self {
            text: choice.text,
            index: choice.index,
            logprobs: choice.logprobs.map(|l| LogProbs {
                tokens: l.tokens,
                token_logprobs: l.token_logprobs,
                top_logprobs: l.top_logprobs,
                text_offset: l.text_offset,
            }),
            finish_reason: choice.finish_reason.into(),
        }
    }
}

impl From<OpenAIChatChoice> for ChatChoice {
    fn from(choice: OpenAIChatChoice) -> Self {
        Self {
            message: ChatMessage {
                role: choice.message.role.into(),
                content: choice.message.content,
                name: choice.message.name,
                tool_call_id: None,
                tool_calls: choice.message.function_call.map(|f| vec![ToolCall {
                    id: Uuid::new_v4().to_string(),
                    tool_type: "function".to_string(),
                    function: FunctionCall {
                        name: f.name,
                        arguments: f.arguments,
                    },
                }]),
            },
            index: choice.index,
            finish_reason: choice.finish_reason.into(),
            tool_calls: None,
        }
    }
}

impl From<OpenAIUsage> for TokenUsage {
    fn from(usage: OpenAIUsage) -> Self {
        Self {
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
            cached_tokens: None,
        }
    }
}

impl From<OpenAIEmbedding> for Embedding {
    fn from(embedding: OpenAIEmbedding) -> Self {
        Self {
            embedding: embedding.embedding,
            index: embedding.index,
        }
    }
}

impl From<OpenAIModel> for ModelInfo {
    fn from(model: OpenAIModel) -> Self {
        Self {
            id: model.id.clone(),
            name: model.id,
            provider: "OpenAI".to_string(),
            model_type: if model.id.starts_with("gpt-") {
                ModelType::Chat
            } else {
                ModelType::Completion
            },
            max_tokens: 4096, // Default, should be updated per model
            max_output_tokens: Some(2048),
            supports_streaming: true,
            supports_functions: true,
            supports_vision: false,
            pricing: None, // Should be updated per model
            capabilities: vec![
                "chat".to_string(),
                "completion".to_string(),
            ],
            deprecated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_openai_client() {
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY environment variable is required for tests");

        let config = LLMClientConfig {
            api_key: Some(api_key),
            ..Default::default()
        };

        let client = OpenAIClient::new(config).unwrap();
        assert!(client.validate_credentials().await.unwrap());
    }
}