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

const API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Google Gemini client implementation
pub struct GeminiClient {
    client: Client,
    api_key: String,
    default_model: String,
}

impl GeminiClient {
    /// Create a new Gemini client
    pub fn new(config: LLMClientConfig) -> LLMResult<Self> {
        let api_key = config.api_key.ok_or_else(|| LLMError::AuthError(
            "Gemini API key is required".to_string()
        ))?;

        let client = Client::builder()
            .timeout(config.timeout_seconds.map(Duration::from_secs).unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(|e| LLMError::Other(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            default_model: config.default_model.unwrap_or_else(|| "gemini-pro".to_string()),
        })
    }

    fn get_api_url(&self, endpoint: &str) -> String {
        format!("{}{}?key={}", API_URL, endpoint, self.api_key)
    }
}

#[async_trait]
impl LLMClient for GeminiClient {
    async fn complete(&self, request: CompletionRequest) -> LLMResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let url = self.get_api_url(&format!("/{}:generateContent", model));

        let request_body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": request.prompt
                }]
            }],
            "generationConfig": {
                "maxOutputTokens": request.max_tokens.unwrap_or(100),
                "temperature": request.temperature.unwrap_or(0.7),
                "topP": request.top_p.unwrap_or(1.0),
                "topK": 40,
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
                provider: "Gemini".to_string(),
                message: error,
            });
        }

        let completion: GeminiGenerateResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        let text = completion.candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        Ok(CompletionResponse {
            request_id: request.request_id,
            choices: vec![CompletionChoice {
                text,
                index: 0,
                logprobs: None,
                finish_reason: completion.candidates
                    .first()
                    .and_then(|c| c.finish_reason.clone())
                    .into(),
            }],
            usage: completion.usage_metadata.map(|u| u.into()).unwrap_or_default(),
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

        let url = self.get_api_url(&format!("/{}:generateContent", model));

        let contents: Vec<GeminiContent> = request.messages.iter().map(|m| GeminiContent {
            role: match m.role.as_str() {
                "user" => "user".to_string(),
                "assistant" => "model".to_string(),
                _ => "user".to_string(),
            },
            parts: vec![GeminiPart {
                text: m.content.clone(),
            }],
        }).collect();

        let request_body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "maxOutputTokens": request.max_tokens.unwrap_or(100),
                "temperature": request.temperature.unwrap_or(0.7),
                "topP": request.top_p.unwrap_or(1.0),
                "topK": 40,
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
                provider: "Gemini".to_string(),
                message: error,
            });
        }

        let chat_response: GeminiGenerateResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        let text = chat_response.candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
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
                finish_reason: chat_response.candidates
                    .first()
                    .and_then(|c| c.finish_reason.clone())
                    .into(),
                tool_calls: None,
            }],
            usage: chat_response.usage_metadata.map(|u| u.into()).unwrap_or_default(),
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
            provider: "Gemini".to_string(),
            message: "Streaming is not yet implemented for Gemini".to_string(),
        })
    }

    async fn stream_chat_complete(
        &self,
        request: ChatCompletionRequest,
    ) -> LLMResult<Box<dyn ChatCompletionStream>> {
        Err(LLMError::ProviderError {
            provider: "Gemini".to_string(),
            message: "Streaming is not yet implemented for Gemini".to_string(),
        })
    }

    async fn embed(&self, request: EmbeddingRequest) -> LLMResult<EmbeddingResponse> {
        let url = self.get_api_url("/embedding-001:embedContent");

        let request_body = serde_json::json!({
            "model": "models/embedding-001",
            "content": {
                "parts": [{
                    "text": request.input.join(" ")
                }]
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
                provider: "Gemini".to_string(),
                message: error,
            });
        }

        let embedding: GeminiEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(EmbeddingResponse {
            data: vec![Embedding {
                embedding: embedding.embedding.values,
                index: 0,
            }],
            model: "embedding-001".to_string(),
            usage: TokenUsage::default(),
        })
    }

    async fn list_models(&self) -> LLMResult<Vec<ModelInfo>> {
        let url = format!("{}?key={}", API_URL, self.api_key);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ProviderError {
                provider: "Gemini".to_string(),
                message: error,
            });
        }

        let models: GeminiModelList = response
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
                provider: "Gemini".to_string(),
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
            provider: "Gemini".to_string(),
            message: "Usage statistics are not supported by Gemini".to_string(),
        })
    }

    async fn cancel_request(&self, _request_id: &str) -> LLMResult<()> {
        Err(LLMError::ProviderError {
            provider: "Gemini".to_string(),
            message: "Request cancellation is not supported by Gemini".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct GeminiGenerateResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
    finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiUsageMetadata {
    prompt_token_count: u32,
    candidates_token_count: u32,
    total_token_count: u32,
}

#[derive(Debug, Deserialize)]
struct GeminiModelList {
    models: Vec<GeminiModel>,
}

#[derive(Debug, Deserialize)]
struct GeminiModel {
    name: String,
    display_name: Option<String>,
    description: Option<String>,
    input_token_limit: Option<u32>,
    output_token_limit: Option<u32>,
    supported_generation_methods: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiEmbeddingResponse {
    embedding: GeminiEmbeddingValue,
}

#[derive(Debug, Deserialize)]
struct GeminiEmbeddingValue {
    values: Vec<f32>,
}

impl From<GeminiUsageMetadata> for TokenUsage {
    fn from(usage: GeminiUsageMetadata) -> Self {
        Self {
            prompt_tokens: usage.prompt_token_count,
            completion_tokens: usage.candidates_token_count,
            total_tokens: usage.total_token_count,
            cached_tokens: None,
        }
    }
}

impl From<GeminiModel> for ModelInfo {
    fn from(model: GeminiModel) -> Self {
        Self {
            id: model.name.clone(),
            name: model.display_name.unwrap_or_else(|| model.name.clone()),
            provider: "Gemini".to_string(),
            model_type: if model.supported_generation_methods.contains(&"generateContent".to_string()) {
                ModelType::Chat
            } else {
                ModelType::Completion
            },
            max_tokens: model.input_token_limit.unwrap_or(32000),
            max_output_tokens: model.output_token_limit,
            supports_streaming: true,
            supports_functions: false,
            supports_vision: true,
            pricing: None,
            capabilities: model.supported_generation_methods,
            deprecated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_gemini_client() {
        if let Ok(api_key) = env::var("GEMINI_API_KEY") {
            let config = LLMClientConfig {
                api_key: Some(api_key),
                ..Default::default()
            };

            let client = GeminiClient::new(config).unwrap();
            assert!(client.validate_credentials().await.unwrap());
        }
    }
}
