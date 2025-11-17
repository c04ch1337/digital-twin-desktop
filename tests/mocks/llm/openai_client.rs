//! Mock implementation of the OpenAI client.

use digital_twin_desktop::core::domain::traits::llm_client::LlmClient;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{self, BoxStream};

/// Mock OpenAI client for testing.
pub struct MockOpenAiClient {
    responses: Arc<Mutex<HashMap<String, String>>>,
    default_response: String,
}

impl MockOpenAiClient {
    /// Create a new mock OpenAI client.
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            default_response: "This is a mock response from the OpenAI client.".to_string(),
        }
    }
    
    /// Create a new mock OpenAI client with a default response.
    pub fn with_default_response(default_response: &str) -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            default_response: default_response.to_string(),
        }
    }
    
    /// Set a response for a specific prompt.
    pub fn set_response(&self, prompt: &str, response: &str) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(prompt.to_string(), response.to_string());
    }
    
    /// Set multiple responses at once.
    pub fn set_responses(&self, responses: HashMap<String, String>) {
        let mut current_responses = self.responses.lock().unwrap();
        for (prompt, response) in responses {
            current_responses.insert(prompt, response);
        }
    }
}

#[async_trait]
impl LlmClient for MockOpenAiClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let responses = self.responses.lock().unwrap();
        if let Some(response) = responses.get(prompt) {
            Ok(response.clone())
        } else {
            Ok(self.default_response.clone())
        }
    }
    
    async fn generate_stream(&self, prompt: &str) -> Result<BoxStream<'static, Result<String>>> {
        let response = self.generate_text(prompt).await?;
        
        // Split the response into words to simulate streaming
        let words: Vec<String> = response
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        // Create a stream of the words
        let stream = stream::iter(words)
            .map(|word| Ok(word))
            .boxed();
        
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_openai_client() {
        // Create a mock client
        let client = MockOpenAiClient::new();
        
        // Set a response
        client.set_response("Hello", "Hi there!");
        
        // Test the response
        let response = client.generate_text("Hello").await.unwrap();
        assert_eq!(response, "Hi there!");
        
        // Test the default response
        let default_response = client.generate_text("Unknown prompt").await.unwrap();
        assert_eq!(default_response, "This is a mock response from the OpenAI client.");
        
        // Test with a custom default response
        let custom_client = MockOpenAiClient::with_default_response("Custom default response");
        let custom_default = custom_client.generate_text("Unknown prompt").await.unwrap();
        assert_eq!(custom_default, "Custom default response");
    }
    
    #[tokio::test]
    async fn test_mock_openai_client_stream() {
        // Create a mock client
        let client = MockOpenAiClient::new();
        
        // Set a response
        client.set_response("Tell me a story", "Once upon a time there was a digital twin.");
        
        // Test the streaming response
        let stream = client.generate_stream("Tell me a story").await.unwrap();
        let words: Vec<String> = stream
            .map(|result| result.unwrap())
            .collect::<Vec<String>>()
            .await;
        
        assert_eq!(words.len(), 9); // 9 words in the response
        assert_eq!(words.join(" "), "Once upon a time there was a digital twin.");
    }
}