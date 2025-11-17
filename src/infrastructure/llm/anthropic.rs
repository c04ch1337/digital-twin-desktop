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

const API_URL: &str = "https://api.anthropic.com/v1";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Anthropic Claude client implementation
pub struct AnthropicClient {
    client: Client,
    api_key: String,
    organization_id: Option<String>,
    default_model: String,
}

impl AnthropicClient {
    /// Create a new Anthropic client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let api_key = config.api_key.ok_or_else(|| LLMError::AuthError(
            "Anthropic API key is required".to_string()
        ))?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&api_key)
                .map_err(|e| LLMError::AuthError(e.to_string()))?
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01")
        );

        if let Some(org_id) = &config.organization_id {
            headers.insert(
                "x-organization",
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
            default_model: config.default_model.unwrap_or_else(|| "claude-2".to_string()),
        })
    }
}

#[async_trait]
impl LLMClient for AnthropicClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let url = format!("{}/complete", API_URL);
        
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let request_body = serde_json::json!({
            "model": model,
            "prompt": request.prompt,
            "max_tokens_to_sample": request.max_tokens.unwrap_or(1000),
            "temperature": request.temperature.unwrap_or(0.7),
            "top_p": request.top_p.unwrap_or(1.0),
            "stop_sequences": request.stop_sequences,
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
                provider: "Anthropic".to_string(),
                message: error,
            });
        }

        let completion: AnthropicCompletionResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(CompletionResponse {
            request_id: request.request_id,
            choices: vec![CompletionChoice {
                text: completion.completion,
                index: 0,
                logprobs: None,
                finish_reason: completion.stop_reason.into(),
            }],
            usage: TokenUsage {
                prompt_tokens: completion.usage.input_tokens,
                completion_tokens: completion.usage.output_tokens,
                total_tokens: completion.usage.total_tokens,
                cached_tokens: None,
            },
            model: model.to_string(),
            created_at: chrono::Utc::now(),
            processing_time_ms: 0, // Not provided by Anthropic
        })
    }

    async fn chat_complete(&self, request: ChatCompletionRequest) -> LLMResult<ChatCompletionResponse> {
        let url = format!("{}/messages", API_URL);

        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let messages: Vec<AnthropicMessage> = request.messages.iter().map(|m| AnthropicMessage {
            role: match m.role {
                MessageRole::System => "system",
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                _ => "user", // Default to user for other roles
            }.to_string(),
            content: m.content.clone().unwrap_or_default(),
        }).collect();

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(1000),
            "temperature": request.temperature.unwrap_or(0.7),
            "top_p": request.top_p.unwrap_or(1.0),
            "stop_sequences": request.stop_sequences,
            "system": request.system,
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
                provider: "Anthropic".to_string(),
                message: error,
            });
        }

        let chat_response: AnthropicChatResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(ChatCompletionResponse {
            request_id: request.request_id,
            choices: vec![ChatChoice {
                message: ChatMessage {
                    role: MessageRole::Assistant,
                    content: Some(chat_response.content),
                    name: None,
                    tool_call_id: None,
                    tool_calls: None,
                },
                index: 0,
                finish_reason: chat_response.stop_reason.into(),
                tool_calls: None,
            }],
            usage: TokenUsage {
                prompt_tokens: chat_response.usage.input_tokens,
                completion_tokens: chat_response.usage.output_tokens,
                total_tokens: chat_response.usage.total_tokens,
                cached_tokens: None,
            },
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
        todo!("Implement streaming completion for Anthropic")
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        // Implement streaming chat completion
        todo!("Implement streaming chat completion for Anthropic")
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        // Anthropic doesn't provide embeddings yet
        Err(LLMError::ModelNotAvailable(
            "Embeddings are not supported by Anthropic".to_string()
        ))
    }

    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "claude-2".to_string(),
                name: "Claude 2".to_string(),
                provider: "Anthropic".to_string(),
                model_type: ModelType::Chat,
                max_tokens: 100000,
                max_output_tokens: Some(4096),
                supports_streaming: true,
                supports_functions: false,
                supports_vision: false,
                pricing: Some(ModelPricing {
                    input_cost_per_1k: 0.008,
                    output_cost_per_1k: 0.024,
                    currency: "USD",
                }),
                capabilities: vec![
                    "chat".to_string(),
                    "completion".to_string(),
                ],
                deprecated_at: None,
            },
            // Add other Claude models
        ])
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let models = self.list_models().await?;
        models.into_iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| LLMError::ModelNotAvailable(
                format!("Model {} not found", model_id)
            ))
    }

    async fn validate_credentials(&self) -> LLMResult<bool> {
        // Try a simple completion to validate credentials
        let request = CompletionRequest {
            request_id: Uuid::new_v4().to_string(),
            prompt: "Test".to_string(),
            model: self.default_model.clone(),
            max_tokens: Some(1),
            ..Default::default()
        };

        match self.complete(request).await {
            Ok(_) => Ok(true),
            Err(LLMError::AuthError(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_usage(&self, period: UsagePeriod) -> LLMResult<UsageStats> {
        // Anthropic doesn't provide usage stats API yet
        Err(LLMError::ProviderError {
            provider: "Anthropic".to_string(),
            message: "Usage statistics are not supported by Anthropic".to_string(),
        })
    }

    async fn cancel_request(&self, request_id: &str) -> LLMResult<()> {
        // Anthropic doesn't support canceling requests
        Err(LLMError::ProviderError {
            provider: "Anthropic".to_string(),
            message: "Request cancellation is not supported by Anthropic".to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicCompletionResponse {
    completion: String,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicChatResponse {
    content: String,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
    total_tokens: u32,
}

impl From<Option<String>> for FinishReason {
    fn from(reason: Option<String>) -> Self {
        match reason.as_deref() {
            Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            None => FinishReason::Null,
            _ => FinishReason::Stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_anthropic_client() {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY environment variable is required for tests");

        let config = LLMClientConfig {
            api_key: Some(api_key),
            ..Default::default()
        };

        let client = AnthropicClient::new(config).unwrap();
        assert!(client.validate_credentials().await.unwrap());
    }
}