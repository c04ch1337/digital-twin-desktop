//! Test helpers for the Digital Twin Desktop application.
//!
//! This module provides helper functions and utilities to simplify test writing.

use digital_twin_desktop::core::domain::traits::{
    llm_client::LlmClient,
    repository::Repository,
    tool_executor::ToolExecutor,
};
use digital_twin_desktop::core::domain::models::{
    agent::Agent,
    conversation::{Conversation, Message},
    digital_twin::DigitalTwin,
    sensor_data::SensorData,
    tool::{Tool, ToolExecution},
};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use std::collections::HashMap;

/// Helper for creating temporary files
pub mod temp_files {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    /// Create a temporary directory that will be cleaned up when dropped
    pub struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        /// Create a new temporary directory
        pub fn new() -> Self {
            let path = std::env::temp_dir().join(format!("dtd-test-{}", Uuid::new_v4()));
            fs::create_dir_all(&path).expect("Failed to create temporary directory");
            Self { path }
        }

        /// Get the path to the temporary directory
        pub fn path(&self) -> &Path {
            &self.path
        }

        /// Create a file in the temporary directory
        pub fn create_file(&self, name: &str, contents: &str) -> PathBuf {
            let file_path = self.path.join(name);
            let mut file = File::create(&file_path).expect("Failed to create temporary file");
            file.write_all(contents.as_bytes()).expect("Failed to write to temporary file");
            file_path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

/// Mock LLM client for testing
pub struct MockLlmClient {
    responses: Arc<Mutex<HashMap<String, String>>>,
}

impl MockLlmClient {
    /// Create a new mock LLM client
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a response for a specific prompt
    pub fn set_response(&self, prompt: &str, response: &str) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(prompt.to_string(), response.to_string());
    }
}

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let responses = self.responses.lock().unwrap();
        if let Some(response) = responses.get(prompt) {
            Ok(response.clone())
        } else {
            Ok("Default mock response".to_string())
        }
    }

    async fn generate_stream(&self, prompt: &str) -> Result<futures::stream::BoxStream<'static, Result<String>>> {
        let response = self.generate_text(prompt).await?;
        let stream = futures::stream::once(async move { Ok(response) }).boxed();
        Ok(stream)
    }
}

/// In-memory repository for testing
pub struct InMemoryRepository<T> {
    items: Arc<Mutex<HashMap<Uuid, T>>>,
}

impl<T: Clone> InMemoryRepository<T> {
    /// Create a new in-memory repository
    pub fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get all items in the repository
    pub fn get_all(&self) -> Vec<T> {
        let items = self.items.lock().unwrap();
        items.values().cloned().collect()
    }
}

/// Mock tool executor for testing
pub struct MockToolExecutor {
    executions: Arc<Mutex<Vec<ToolExecution>>>,
    results: Arc<Mutex<HashMap<String, serde_json::Value>>>,
}

impl MockToolExecutor {
    /// Create a new mock tool executor
    pub fn new() -> Self {
        Self {
            executions: Arc::new(Mutex::new(Vec::new())),
            results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a result for a specific command
    pub fn set_result(&self, command: &str, result: serde_json::Value) {
        let mut results = self.results.lock().unwrap();
        results.insert(command.to_string(), result);
    }

    /// Get all recorded executions
    pub fn get_executions(&self) -> Vec<ToolExecution> {
        let executions = self.executions.lock().unwrap();
        executions.clone()
    }
}

#[async_trait]
impl ToolExecutor for MockToolExecutor {
    async fn execute(&self, tool: &Tool, input: serde_json::Value) -> Result<serde_json::Value> {
        let command = input["command"].as_str().unwrap_or("unknown");
        
        // Record the execution
        let execution = ToolExecution {
            id: Uuid::new_v4(),
            tool_id: tool.id,
            agent_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            status: digital_twin_desktop::core::domain::models::tool::ToolExecutionStatus::Completed,
            input: input.clone(),
            output: serde_json::json!({}),
            started_at: chrono::Utc::now(),
            completed_at: Some(chrono::Utc::now()),
            error: None,
        };
        
        let mut executions = self.executions.lock().unwrap();
        executions.push(execution);
        
        // Return the configured result or a default
        let results = self.results.lock().unwrap();
        if let Some(result) = results.get(command) {
            Ok(result.clone())
        } else {
            Ok(serde_json::json!({ "status": "success", "result": "Mock execution" }))
        }
    }
}

/// HTTP test client helper
pub mod http {
    use reqwest::{Client, Response, StatusCode};
    use serde::Serialize;
    use std::time::Duration;

    /// Create a test HTTP client
    pub fn create_test_client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create test HTTP client")
    }

    /// Helper to make a test POST request
    pub async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<Response, reqwest::Error> {
        let client = create_test_client();
        client.post(url).json(body).send().await
    }

    /// Helper to make a test GET request
    pub async fn get(url: &str) -> Result<Response, reqwest::Error> {
        let client = create_test_client();
        client.get(url).send().await
    }

    /// Assert that a response has the expected status code
    pub fn assert_status(response: &Response, expected: StatusCode) {
        assert_eq!(
            response.status(),
            expected,
            "Expected status code {:?}, got {:?}",
            expected,
            response.status()
        );
    }
}