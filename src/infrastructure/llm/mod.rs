//! LLM client implementations.

mod anthropic;
mod openai;
mod openrouter;
mod gemini;
mod huggingface;
mod ollama;
mod lmstudio;

pub use anthropic::AnthropicClient;
pub use openai::OpenAIClient;
pub use openrouter::OpenRouterClient;
pub use gemini::GeminiClient;
pub use huggingface::HuggingFaceClient;
pub use ollama::OllamaClient;
pub use lmstudio::LMStudioClient;

use async_trait::async_trait;
use anyhow::Result;

use crate::core::domain::traits::llm_client::{
    LLMClient, LLMResult, LLMError, LLMClientConfig, LLMClientFactory,
};

/// Factory for creating LLM clients
pub struct DefaultLLMClientFactory;

impl DefaultLLMClientFactory {
    /// Create a new LLM client factory
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LLMClientFactory for DefaultLLMClientFactory {
    async fn create_client(
        &self,
        provider: &str,
        config: LLMClientConfig,
    ) -> LLMResult<Box<dyn LLMClient>> {
        match provider.to_lowercase().as_str() {
            "anthropic" => Ok(Box::new(AnthropicClient::new(config)?)),
            "openai" => Ok(Box::new(OpenAIClient::new(config)?)),
            "openrouter" => Ok(Box::new(OpenRouterClient::new(config)?)),
            "gemini" => Ok(Box::new(GeminiClient::new(config)?)),
            "huggingface" => Ok(Box::new(HuggingFaceClient::new(config)?)),
            "ollama" => Ok(Box::new(OllamaClient::new(config)?)),
            "lmstudio" => Ok(Box::new(LMStudioClient::new(config)?)),
            _ => Err(LLMError::ProviderError {
                provider: provider.to_string(),
                message: "Unsupported LLM provider".to_string(),
            }),
        }
    }

    fn available_providers(&self) -> Vec<String> {
        vec![
            "anthropic".to_string(),
            "openai".to_string(),
            "openrouter".to_string(),
            "gemini".to_string(),
            "huggingface".to_string(),
            "ollama".to_string(),
            "lmstudio".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_llm_factory() {
        let factory = DefaultLLMClientFactory::new();
        assert_eq!(factory.available_providers().len(), 2);

        // Test OpenAI client creation
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            let config = LLMClientConfig {
                api_key: Some(api_key),
                ..Default::default()
            };
            let client = factory.create_client("openai", config).await.unwrap();
            assert!(client.validate_credentials().await.unwrap());
        }

        // Test Anthropic client creation
        if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
            let config = LLMClientConfig {
                api_key: Some(api_key),
                ..Default::default()
            };
            let client = factory.create_client("anthropic", config).await.unwrap();
            assert!(client.validate_credentials().await.unwrap());
        }
    }
}