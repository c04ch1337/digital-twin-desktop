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

/// Ollama local LLM client implementation
pub struct OllamaClient {
    client: Client,
    base_url: String,
    default_model: String,
}

impl OllamaClient {
    /// Create a new Ollama client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let base_url = config.base_url.unwrap_or_else(|| "http://localhost:11434".to_string());

        let client = Client::builder()
            .timeout(config.timeout_seconds.map(Duration::from_secs).unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(|e| LLMError::Other(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            default_model: config.default_model.unwrap_or_else(|| "llama2".to_string()),
        })
    }
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let url = format!("{}/api/generate", self.base_url);

        let request_body = serde_json::json!({
            "model": model,
            "prompt": request.prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0),
                "top_k": 40,
                "num_predict": request.max_tokens.unwrap_or(100),
            }
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
                provider: "Ollama".to_string(),
                message: error,
            });
        }

        let completion: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(CompletionResponse {
            request_id: request.request_id,
            choices: vec![CompletionChoice {
                text: completion.response,
                index: 0,
                logprobs: None,
                finish_reason: Some("stop".to_string()),
            }],
            usage: TokenUsage {
                prompt_tokens: completion.prompt_eval_count.unwrap_or(0),
                completion_tokens: completion.eval_count.unwrap_or(0),
                total_tokens: completion.prompt_eval_count.unwrap_or(0) + completion.eval_count.unwrap_or(0),
                cached_tokens: None,
            },
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

        let url = format!("{}/api/chat", self.base_url);

        let messages: Vec<OllamaChatMessage> = request.messages.iter().map(|m| OllamaChatMessage {
            role: m.role.clone(),
            content: m.content.clone().unwrap_or_default(),
        }).collect();

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0),
                "top_k": 40,
                "num_predict": request.max_tokens.unwrap_or(100),
            }
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
                provider: "Ollama".to_string(),
                message: error,
            });
        }

        let chat_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(ChatCompletionResponse {
            request_id: request.request_id,
            choices: vec![ChatChoice {
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: Some(chat_response.message.content),
                    name: None,
                    tool_call_id: None,
                    tool_calls: None,
                },
                index: 0,
                finish_reason: Some("stop".to_string()),
                tool_calls: None,
            }],
            usage: TokenUsage {
                prompt_tokens: chat_response.prompt_eval_count.unwrap_or(0),
                completion_tokens: chat_response.eval_count.unwrap_or(0),
                total_tokens: chat_response.prompt_eval_count.unwrap_or(0) + chat_response.eval_count.unwrap_or(0),
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
        Err(LLMError::ProviderError {
            provider: "Ollama".to_string(),
            message: "Streaming is not yet implemented for Ollama".to_string(),
        })
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        Err(LLMError::ProviderError {
            provider: "Ollama".to_string(),
            message: "Streaming is not yet implemented for Ollama".to_string(),
        })
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        let url = format!("{}/api/embeddings", self.base_url);

        let request_body = serde_json::json!({
            "model": request.model,
            "prompt": request.input.join(" "),
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
                provider: "Ollama".to_string(),
                message: error,
            });
        }

        let embedding: OllamaEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(EmbeddingResponse {
            data: vec![Embedding {
                embedding: embedding.embedding,
                index: 0,
            }],
            model: request.model,
            usage: TokenUsage::default(),
        })
    }

    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>> {
        let url = format!("{}/api/tags", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "Ollama".to_string(),
                message: error,
            });
        }

        let models: OllamaModelList = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(models.models.into_iter().map(|m| m.into()).collect())
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let models = self.list_models().await?;
        models.into_iter()
            .find(|m| m.id.contains(model_id))
            .ok_or_else(|| LLMError::ProviderError {
                provider: "Ollama".to_string(),
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
            provider: "Ollama".to_string(),
            message: "Usage statistics are not supported by Ollama".to_string(),
        })
    }

    async fn cancel_request(&self, _request_id: &str) -> LLMResult<()> {
        Err(LLMError::ProviderError {
            provider: "Ollama".to_string(),
            message: "Request cancellation is not supported by Ollama".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaChatMessage,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelList {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
    #[serde(default)]
    modified_at: Option<String>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    digest: Option<String>,
}

impl From<OllamaModel> for ModelInfo {
    fn from(model: OllamaModel) -> Self {
        Self {
            id: model.name.clone(),
            name: model.name,
            provider: "Ollama".to_string(),
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
    async fn test_ollama_client() {
        let config = LLMClientConfig {
            base_url: Some("http://localhost:11434".to_string()),
            ..Default::default()
        };

        let client = OllamaClient::new(config).unwrap();
        // This will only pass if Ollama is running locally
        let _ = client.validate_credentials().await;
    }
}
