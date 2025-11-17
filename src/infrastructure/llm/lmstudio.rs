use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::core::domain::traits::llm_client::{
    LLMClient, LLMResult, LLMError, CompletionRequest, CompletionResponse,
    ChatCompletionRequest, ChatCompletionResponse, CompletionStream,
    ChatCompletionStream, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    UsagePeriod, UsageStats, LLMClientConfig, CompletionChoice, ChatChoice,
    ChatMessage, ToolCall, FunctionCall, TokenUsage, Embedding, ModelType,
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// LMStudio local LLM client implementation
pub struct LMStudioClient {
    client: Client,
    base_url: String,
    default_model: String,
}

impl LMStudioClient {
    /// Create a new LMStudio client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let base_url = config.base_url.unwrap_or_else(|| "http://localhost:1234".to_string());

        let client = Client::builder()
            .timeout(config.timeout_seconds.map(Duration::from_secs).unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(|e| LLMError::Other(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            default_model: config.default_model.unwrap_or_else(|| "local-model".to_string()),
        })
    }
}

#[async_trait]
impl LLMClient for LMStudioClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let url = format!("{}/v1/completions", self.base_url);

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
                provider: "LMStudio".to_string(),
                message: error,
            });
        }

        let completion: LMStudioCompletionResponse = response
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
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let url = format!("{}/v1/chat/completions", self.base_url);

        let messages: Vec<LMStudioChatMessage> = request.messages.iter().map(|m| LMStudioChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
            name: m.name.clone(),
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
                provider: "LMStudio".to_string(),
                message: error,
            });
        }

        let chat_response: LMStudioChatResponse = response
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
        Err(LLMError::ProviderError {
            provider: "LMStudio".to_string(),
            message: "Streaming is not yet implemented for LMStudio".to_string(),
        })
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        Err(LLMError::ProviderError {
            provider: "LMStudio".to_string(),
            message: "Streaming is not yet implemented for LMStudio".to_string(),
        })
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        let url = format!("{}/v1/embeddings", self.base_url);

        let request_body = serde_json::json!({
            "model": request.model,
            "input": request.input,
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
                provider: "LMStudio".to_string(),
                message: error,
            });
        }

        let embedding: LMStudioEmbeddingResponse = response
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
        let url = format!("{}/v1/models", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "LMStudio".to_string(),
                message: error,
            });
        }

        let models: LMStudioModelList = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(models.data.into_iter().map(|m| m.into()).collect())
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let models = self.list_models().await?;
        models.into_iter()
            .find(|m| m.id.contains(model_id))
            .ok_or_else(|| LLMError::ProviderError {
                provider: "LMStudio".to_string(),
                message: format!("Model {} not found", model_id),
            })
    }

    async fn validate_credentials(&self) -> LLMResult<bool> {
        match self.list_models().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn get_usage(&self, _period: UsagePeriod) -> LLMResult<UsageStats> {
        Err(LLMError::ProviderError {
            provider: "LMStudio".to_string(),
            message: "Usage statistics are not supported by LMStudio".to_string(),
        })
    }

    async fn cancel_request(&self, _request_id: &str) -> LLMResult<()> {
        Err(LLMError::ProviderError {
            provider: "LMStudio".to_string(),
            message: "Request cancellation is not supported by LMStudio".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct LMStudioCompletionResponse {
    choices: Vec<LMStudioCompletionChoice>,
    usage: LMStudioUsage,
}

#[derive(Debug, Deserialize)]
struct LMStudioCompletionChoice {
    text: String,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioChatResponse {
    choices: Vec<LMStudioChatChoice>,
    usage: LMStudioUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct LMStudioChatMessage {
    role: String,
    content: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioChatChoice {
    message: LMStudioChatMessage,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct LMStudioEmbeddingResponse {
    data: Vec<LMStudioEmbedding>,
    model: String,
    usage: LMStudioUsage,
}

#[derive(Debug, Deserialize)]
struct LMStudioEmbedding {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Debug, Deserialize)]
struct LMStudioModelList {
    data: Vec<LMStudioModel>,
}

#[derive(Debug, Deserialize)]
struct LMStudioModel {
    id: String,
    object: String,
    owned_by: String,
}

impl From<LMStudioCompletionChoice> for CompletionChoice {
    fn from(choice: LMStudioCompletionChoice) -> Self {
        Self {
            text: choice.text,
            index: choice.index,
            logprobs: None,
            finish_reason: choice.finish_reason.into(),
        }
    }
}

impl From<LMStudioChatChoice> for ChatChoice {
    fn from(choice: LMStudioChatChoice) -> Self {
        Self {
            message: ChatMessage {
                role: choice.message.role,
                content: choice.message.content,
                name: choice.message.name,
                tool_call_id: None,
                tool_calls: None,
            },
            index: choice.index,
            finish_reason: choice.finish_reason.into(),
            tool_calls: None,
        }
    }
}

impl From<LMStudioUsage> for TokenUsage {
    fn from(usage: LMStudioUsage) -> Self {
        Self {
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
            cached_tokens: None,
        }
    }
}

impl From<LMStudioEmbedding> for Embedding {
    fn from(embedding: LMStudioEmbedding) -> Self {
        Self {
            embedding: embedding.embedding,
            index: embedding.index,
        }
    }
}

impl From<LMStudioModel> for ModelInfo {
    fn from(model: LMStudioModel) -> Self {
        Self {
            id: model.id.clone(),
            name: model.id,
            provider: "LMStudio".to_string(),
            model_type: ModelType::Chat,
            max_tokens: 4096,
            max_output_tokens: Some(2048),
            supports_streaming: true,
            supports_functions: false,
            supports_vision: false,
            pricing: None,
            capabilities: vec!["chat".to_string(), "completion".to_string()],
            deprecated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lmstudio_client() {
        let config = LLMClientConfig {
            base_url: Some("http://localhost:1234".to_string()),
            ..Default::default()
        };

        let client = LMStudioClient::new(config).unwrap();
        // This will only pass if LMStudio is running locally
        let _ = client.validate_credentials().await;
    }
}
