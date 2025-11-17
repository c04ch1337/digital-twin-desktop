use async_trait::async_trait;
use reqwest::{Client, header};
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

const API_URL: &str = "https://api-inference.huggingface.co";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// HuggingFace Inference API client implementation
pub struct HuggingFaceClient {
    client: Client,
    api_key: String,
    default_model: String,
}

impl HuggingFaceClient {
    /// Create a new HuggingFace client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let api_key = config.api_key.ok_or_else(|| LLMError::AuthError(
            "HuggingFace API key is required".to_string()
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
            default_model: config.default_model.unwrap_or_else(|| "gpt2".to_string()),
        })
    }
}

#[async_trait]
impl LLMClient for HuggingFaceClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let url = format!("{}/models/{}", API_URL, model);

        let request_body = serde_json::json!({
            "inputs": request.prompt,
            "parameters": {
                "max_new_tokens": request.max_tokens.unwrap_or(100),
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0),
                "top_k": 50,
                "repetition_penalty": 1.0 + request.frequency_penalty.unwrap_or(0.0),
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
                provider: "HuggingFace".to_string(),
                message: error,
            });
        }

        let completions: Vec<HuggingFaceCompletion> = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        let text = completions
            .first()
            .map(|c| c.generated_text.clone())
            .unwrap_or_default();

        Ok(CompletionResponse {
            request_id: request.request_id,
            choices: vec![CompletionChoice {
                text,
                index: 0,
                logprobs: None,
                finish_reason: Some("length".to_string()),
            }],
            usage: TokenUsage::default(),
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

        let url = format!("{}/models/{}", API_URL, model);

        // Convert messages to a prompt format
        let prompt = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content.as_deref().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("\n");

        let request_body = serde_json::json!({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": request.max_tokens.unwrap_or(100),
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0),
                "top_k": 50,
                "repetition_penalty": 1.0 + request.frequency_penalty.unwrap_or(0.0),
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
                provider: "HuggingFace".to_string(),
                message: error,
            });
        }

        let completions: Vec<HuggingFaceCompletion> = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        let text = completions
            .first()
            .map(|c| c.generated_text.clone())
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            request_id: request.request_id,
            choices: vec![ChatChoice {
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: Some(text),
                    name: None,
                    tool_call_id: None,
                    tool_calls: None,
                },
                index: 0,
                finish_reason: Some("length".to_string()),
                tool_calls: None,
            }],
            usage: TokenUsage::default(),
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
            provider: "HuggingFace".to_string(),
            message: "Streaming is not yet implemented for HuggingFace".to_string(),
        })
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        Err(LLMError::ProviderError {
            provider: "HuggingFace".to_string(),
            message: "Streaming is not yet implemented for HuggingFace".to_string(),
        })
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        let model = request.model.as_str();
        let url = format!("{}/models/{}", API_URL, model);

        let request_body = serde_json::json!({
            "inputs": request.input,
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
                provider: "HuggingFace".to_string(),
                message: error,
            });
        }

        let embeddings: Vec<Vec<f32>> = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(EmbeddingResponse {
            data: embeddings.into_iter().enumerate().map(|(i, embedding)| Embedding {
                embedding,
                index: i,
            }).collect(),
            model: model.to_string(),
            usage: TokenUsage::default(),
        })
    }

    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>> {
        let url = format!("{}/api/models", API_URL);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "HuggingFace".to_string(),
                message: error,
            });
        }

        let models: Vec<HuggingFaceModel> = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    async fn get_model_info(&self, model_id: &str) -> LLMResult<ModelInfo> {
        let models = self.list_models().await?;
        models.into_iter()
            .find(|m| m.id.contains(model_id))
            .ok_or_else(|| LLMError::ProviderError {
                provider: "HuggingFace".to_string(),
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
            provider: "HuggingFace".to_string(),
            message: "Usage statistics are not supported by HuggingFace".to_string(),
        })
    }

    async fn cancel_request(&self, _request_id: &str) -> LLMResult<()> {
        Err(LLMError::ProviderError {
            provider: "HuggingFace".to_string(),
            message: "Request cancellation is not supported by HuggingFace".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct HuggingFaceCompletion {
    generated_text: String,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceModel {
    id: String,
    #[serde(default)]
    modelId: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    downloads: u32,
    #[serde(default)]
    likes: u32,
}

impl From<HuggingFaceModel> for ModelInfo {
    fn from(model: HuggingFaceModel) -> Self {
        let model_type = if model.tags.contains(&"text-generation".to_string()) {
            ModelType::Chat
        } else if model.tags.contains(&"text2text-generation".to_string()) {
            ModelType::Chat
        } else {
            ModelType::Completion
        };

        Self {
            id: model.id.clone(),
            name: model.id,
            provider: "HuggingFace".to_string(),
            model_type,
            max_tokens: 2048,
            max_output_tokens: Some(1024),
            supports_streaming: false,
            supports_functions: false,
            supports_vision: false,
            pricing: None,
            capabilities: model.tags,
            deprecated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_huggingface_client() {
        if let Ok(api_key) = env::var("HUGGINGFACE_API_KEY") {
            let config = LLMClientConfig {
                api_key: Some(api_key),
                ..Default::default()
            };

            let client = HuggingFaceClient::new(config).unwrap();
            assert!(client.validate_credentials().await.unwrap());
        }
    }
}
