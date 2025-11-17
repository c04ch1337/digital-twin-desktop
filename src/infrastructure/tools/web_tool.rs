use async_trait::async_trait;
use reqwest::{Client, Method, header};
use serde_json::Value;
use std::time::Duration;
use chrono::Utc;
use tracing::{debug, error, info};

use crate::core::domain::{
    models::{Tool, ToolResult, ExecutionStatus, ExecutionMetrics},
    traits::tool_executor::{
        ToolExecutor, ExecutorResult, ExecutorError, ExecutionRequest,
        ExecutionContext, ValidationResult, ValidationError,
    },
};

/// Web API tool executor
pub struct WebToolExecutor {
    client: Client,
    max_response_size: usize,
    allowed_domains: Vec<String>,
    timeout: Duration,
}

impl WebToolExecutor {
    /// Create a new web tool executor
    pub fn new(
        max_response_size: usize,
        allowed_domains: Vec<String>,
        timeout: Duration,
    ) -> ExecutorResult<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| ExecutorError::Other(e.to_string()))?;

        Ok(Self {
            client,
            max_response_size,
            allowed_domains,
            timeout,
        })
    }

    /// Validate URL against allowed domains
    fn validate_url(&self, url: &str) -> ExecutorResult<()> {
        let url = url::Url::parse(url)
            .map_err(|e| ExecutorError::ValidationError {
                parameter: "url".to_string(),
                reason: e.to_string(),
            })?;

        let host = url.host_str().ok_or_else(|| ExecutorError::ValidationError {
            parameter: "url".to_string(),
            reason: "URL must have a host".to_string(),
        })?;

        if !self.allowed_domains.is_empty() && 
           !self.allowed_domains.iter().any(|domain| host.ends_with(domain)) {
            return Err(ExecutorError::PermissionDenied(
                format!("Domain not allowed: {}", host)
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl ToolExecutor for WebToolExecutor {
    async fn execute(&self, request: ExecutionRequest) -> ExecutorResult<ToolResult> {
        let start_time = Utc::now();
        let mut metrics = ExecutionMetrics::default();

        // Get URL
        let url = request.parameters.get("url")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "web".to_string(),
                parameter: "url".to_string(),
            })?;

        // Validate URL
        self.validate_url(url)?;

        // Get method
        let method = request.parameters.get("method")
            .and_then(Value::as_str)
            .unwrap_or("GET");

        let method = match method.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => return Err(ExecutorError::ValidationError {
                parameter: "method".to_string(),
                reason: format!("Invalid HTTP method: {}", method),
            }),
        };

        // Build request
        let mut req_builder = self.client.request(method, url);

        // Add headers
        if let Some(headers) = request.parameters.get("headers").and_then(Value::as_object) {
            for (key, value) in headers {
                if let Some(value) = value.as_str() {
                    req_builder = req_builder.header(key, value);
                }
            }
        }

        // Add body
        if let Some(body) = request.parameters.get("body") {
            req_builder = req_builder.json(body);
        }

        // Execute request
        let response = req_builder
            .send()
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        // Check response size
        let content_length = response.content_length().unwrap_or(0);
        if content_length > self.max_response_size as u64 {
            return Err(ExecutorError::ResourceLimitExceeded {
                resource: "response_size".to_string(),
                message: format!(
                    "Response size {} exceeds limit of {} bytes",
                    content_length, self.max_response_size
                ),
            });
        }

        // Get response data
        let status = response.status();
        let headers: HashMap<String, String> = response.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.text()
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        metrics.bytes_read = body.len() as u64;

        // Try to parse body as JSON
        let body_value = serde_json::from_str::<Value>(&body).unwrap_or(Value::String(body));

        let end_time = Utc::now();
        metrics.execution_time_ms = (end_time - start_time).num_milliseconds() as u64;

        Ok(ToolResult {
            execution_id: request.execution_id,
            tool_id: request.tool_id,
            status: ExecutionStatus::Completed,
            parameters: request.parameters,
            output: Some(serde_json::json!({
                "status": status.as_u16(),
                "headers": headers,
                "body": body_value,
            })),
            started_at: start_time,
            completed_at: Some(end_time),
            error: None,
            metrics,
        })
    }

    async fn validate_parameters(
        &self,
        tool_id: ToolId,
        parameters: &HashMap<String, Value>,
    ) -> ExecutorResult<ValidationResult> {
        let mut errors = Vec::new();

        // Validate URL
        match parameters.get("url").and_then(Value::as_str) {
            Some(url) => {
                if let Err(e) = self.validate_url(url) {
                    errors.push(ValidationError {
                        parameter: "url".to_string(),
                        code: "INVALID_URL".to_string(),
                        message: e.to_string(),
                        expected: None,
                        actual: Some(url.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "url".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "URL parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate method
        if let Some(method) = parameters.get("method").and_then(Value::as_str) {
            if !["GET", "POST", "PUT", "DELETE"].contains(&method.to_uppercase().as_str()) {
                errors.push(ValidationError {
                    parameter: "method".to_string(),
                    code: "INVALID_METHOD".to_string(),
                    message: format!("Invalid HTTP method: {}", method),
                    expected: Some("GET, POST, PUT, or DELETE".to_string()),
                    actual: Some(method.to_string()),
                });
            }
        }

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
            normalized_parameters: None,
        })
    }

    async fn can_execute(
        &self,
        tool_id: ToolId,
        context: &ExecutionContext,
    ) -> ExecutorResult<bool> {
        // Check if context has required permissions
        if !context.security.permissions.contains(&"web:request".to_string()) {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_web_tool() {
        let mut server = Server::new();

        // Mock endpoint
        let mock = server.mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Hello, world!"}"#)
            .create();

        let executor = WebToolExecutor::new(
            1024 * 1024, // 1MB
            vec!["localhost".to_string()],
            Duration::from_secs(30),
        ).unwrap();

        let request = ExecutionRequest {
            execution_id: Uuid::new_v4(),
            tool_id: Uuid::new_v4(),
            parameters: serde_json::json!({
                "url": server.url().join("/test").unwrap().to_string(),
                "method": "GET",
            })
            .as_object()
            .unwrap()
            .clone(),
            context: ExecutionContext {
                agent_id: Uuid::new_v4(),
                user_id: None,
                conversation_id: None,
                session_id: "test".to_string(),
                security: SecurityContext {
                    auth_token: None,
                    permissions: vec!["web:request".to_string()],
                    ip_address: None,
                    labels: Default::default(),
                },
                environment: Default::default(),
                working_directory: None,
                metadata: Default::default(),
            },
            options: Default::default(),
            callback_url: None,
        };

        let result = executor.execute(request).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Completed);

        let output = result.output.unwrap();
        assert_eq!(output.get("status").unwrap().as_u64().unwrap(), 200);
        assert_eq!(
            output.get("body").unwrap().get("message").unwrap().as_str().unwrap(),
            "Hello, world!"
        );

        mock.assert();
    }
}