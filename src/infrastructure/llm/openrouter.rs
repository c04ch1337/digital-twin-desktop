use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info};
use uuid::Uuid;
use std::collections::HashMap;

use crate::core::domain::traits::llm_client::{
    LLMClient, LLMResult, LLMError, CompletionRequest, CompletionResponse,
    ChatCompletionRequest, ChatCompletionResponse, CompletionStream,
    ChatCompletionStream, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    UsagePeriod, UsageStats, LLMClientConfig, CompletionChoice, ChatChoice,
    ChatMessage, ToolCall, FunctionCall, TokenUsage, Embedding, ModelType,
};

const API_URL: &str = "https://openrouter.io/api/v1";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// OpenRouter client implementation
pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    default_model: String,
    site_url: Option<String>,
    site_name: Option<String>,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let api_key = config.api_key.ok_or_else(|| LLMError::AuthError(
            "OpenRouter API key is required".to_string()
        ))?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|e| LLMError::AuthError(e.to_string()))?
        );

        let client = Client::builder()
            .timeout(config.timeout_seconds.map(Duration::from_secs).unwrap_or(DEFAULT_TIMEOUT))
            .default_headers(headers)
            .build()
            .map_err(|e| LLMError::Other(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            default_model: config.default_model.unwrap_or_else(|| "openrouter/auto".to_string()),
            site_url: config.site_url,
            site_name: config.site_name,
        })
    }
}

#[async_trait]
impl LLMClient for OpenRouterClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let url = format!("{}/completions", API_URL);
        
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let mut request_body = serde_json::json!({
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

        if let Some(site_url) = &self.site_url {
            request_body["site_url"] = serde_json::json!(site_url);
        }
        if let Some(site_name) = &self.site_name {
            request_body["site_name"] = serde_json::json!(site_name);
        }

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
                provider: "OpenRouter".to_string(),
                message: error,
            });
        }

        let completion: OpenRouterCompletionResponse = response
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

        let messages: Vec<OpenRouterChatMessage> = request.messages.iter().map(|m| OpenRouterChatMessage {
            role: m.role.into(),
            content: m.content.clone(),
            name: m.name.clone(),
        }).collect();

        let mut request_body = serde_json::json!({
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

        if let Some(site_url) = &self.site_url {
            request_body["site_url"] = serde_json::json!(site_url);
        }
        if let Some(site_name) = &self.site_name {
            request_body["site_name"] = serde_json::json!(site_name);
        }

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
                provider: "OpenRouter".to_string(),
                message: error,
            });
        }

        let chat_response: OpenRouterChatResponse = response
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
            provider: "OpenRouter".to_string(),
            message: "Streaming is not yet implemented for OpenRouter".to_string(),
        })
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        Err(LLMError::ProviderError {
            provider: "OpenRouter".to_string(),
            message: "Streaming is not yet implemented for OpenRouter".to_string(),
        })
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        Err(LLMError::ProviderError {
            provider: "OpenRouter".to_string(),
            message: "Embeddings are not supported by OpenRouter".to_string(),
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
                provider: "OpenRouter".to_string(),
                message: error,
            });
        }

        let models: OpenRouterModelList = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(models.data.into_iter().map(|m| m.into()).collect())
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let models = self.list_models().await?;
        models.into_iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| LLMError::ProviderError {
                provider: "OpenRouter".to_string(),
                message: format!("Model {} not found", model_id),
            })
    }

    async fn validate_credentials(&self) -> LLMResult<bool> {
        match self.list_models().await {
            Ok(_) => Ok(true),
            Err(LLMError::AuthError(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_usage(&self, _period: UsagePeriod) -> LLMResult<UsageStats> {
        Err(LLMError::ProviderError {
            provider: "OpenRouter".to_string(),
            message: "Usage statistics are not supported by OpenRouter".to_string(),
        })
    }

    async fn cancel_request(&self, _request_id: &str) -> LLMResult<()> {
        Err(LLMError::ProviderError {
            provider: "OpenRouter".to_string(),
            message: "Request cancellation is not supported by OpenRouter".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct OpenRouterCompletionResponse {
    choices: Vec<OpenRouterCompletionChoice>,
    usage: OpenRouterUsage,
}

#[derive(Debug, Deserialize)]
struct OpenRouterCompletionChoice {
    text: String,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChatResponse {
    choices: Vec<OpenRouterChatChoice>,
    usage: OpenRouterUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenRouterChatMessage {
    role: String,
    content: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChatChoice {
    message: OpenRouterChatMessage,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenRouterModelList {
    data: Vec<OpenRouterModel>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterModel {
    id: String,
    name: Option<String>,
    description: Option<String>,
    pricing: Option<OpenRouterPricing>,
    context_length: Option<u32>,
    architecture: Option<OpenRouterArchitecture>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterPricing {
    prompt: Option<String>,
    completion: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterArchitecture {
    modality: Option<String>,
    tokenizer: Option<String>,
    instruct_type: Option<String>,
}

impl From<OpenRouterCompletionChoice> for CompletionChoice {
    fn from(choice: OpenRouterCompletionChoice) -> Self {
        Self {
            text: choice.text,
            index: choice.index,
            logprobs: None,
            finish_reason: choice.finish_reason.into(),
        }
    }
}

impl From<OpenRouterChatChoice> for ChatChoice {
    fn from(choice: OpenRouterChatChoice) -> Self {
        Self {
            message: ChatMessage {
                role: choice.message.role.into(),
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

impl From<OpenRouterUsage> for TokenUsage {
    fn from(usage: OpenRouterUsage) -> Self {
        Self {
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
            cached_tokens: None,
        }
    }
}

impl From<OpenRouterModel> for ModelInfo {
    fn from(model: OpenRouterModel) -> Self {
        Self {
            id: model.id.clone(),
            name: model.name.unwrap_or_else(|| model.id.clone()),
            provider: "OpenRouter".to_string(),
            model_type: ModelType::Chat,
            max_tokens: model.context_length.unwrap_or(4096),
            max_output_tokens: Some(2048),
            supports_streaming: true,
            supports_functions: false,
            supports_vision: false,
            pricing: None,
            capabilities: vec!["chat".to_string()],
            deprecated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_openrouter_client() {
        if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
            let config = LLMClientConfig {
                api_key: Some(api_key),
                ..Default::default()
            };

            let client = OpenRouterClient::new(config).unwrap();
            assert!(client.validate_credentials().await.unwrap());
        }
    }
}
