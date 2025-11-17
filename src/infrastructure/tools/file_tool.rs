use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;
use serde_json::Value;
use anyhow::Result;
use chrono::Utc;
use tracing::{debug, error, info};

use crate::core::domain::{
    models::{Tool, ToolResult, ExecutionStatus, ExecutionMetrics},
    traits::tool_executor::{
        ToolExecutor, ExecutorResult, ExecutorError, ExecutionRequest,
        ExecutionContext, ValidationResult, ValidationError,
    },
};

/// File system tool executor
pub struct FileToolExecutor {
    base_path: PathBuf,
    max_file_size: u64,
    allowed_extensions: Vec<String>,
}

impl FileToolExecutor {
    /// Create a new file tool executor
    pub fn new(base_path: PathBuf, max_file_size: u64, allowed_extensions: Vec<String>) -> Self {
        Self {
            base_path,
            max_file_size,
            allowed_extensions,
        }
    }

    /// Validate file path
    fn validate_path(&self, path: &Path) -> ExecutorResult<PathBuf> {
        let full_path = self.base_path.join(path);
        
        // Ensure path is within base directory
        if !full_path.starts_with(&self.base_path) {
            return Err(ExecutorError::PermissionDenied(
                "Path must be within allowed directory".to_string()
            ));
        }

        // Validate extension if specified
        if let Some(ext) = path.extension() {
            if !self.allowed_extensions.is_empty() && 
               !self.allowed_extensions.contains(&ext.to_string_lossy().to_string()) {
                return Err(ExecutorError::ValidationError {
                    parameter: "path".to_string(),
                    reason: format!("File extension not allowed: {}", ext.to_string_lossy()),
                });
            }
        }

        Ok(full_path)
    }

    /// Read file contents
    async fn read_file(&self, path: &Path) -> ExecutorResult<String> {
        let full_path = self.validate_path(path)?;

        // Check file size
        let metadata = fs::metadata(&full_path)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        if metadata.len() > self.max_file_size {
            return Err(ExecutorError::ResourceLimitExceeded {
                resource: "file_size".to_string(),
                message: format!("File size exceeds limit of {} bytes", self.max_file_size),
            });
        }

        fs::read_to_string(&full_path)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))
    }

    /// Write file contents
    async fn write_file(&self, path: &Path, content: &str) -> ExecutorResult<()> {
        let full_path = self.validate_path(path)?;

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;
        }

        // Check content size
        if content.len() as u64 > self.max_file_size {
            return Err(ExecutorError::ResourceLimitExceeded {
                resource: "content_size".to_string(),
                message: format!("Content size exceeds limit of {} bytes", self.max_file_size),
            });
        }

        fs::write(&full_path, content)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))
    }

    /// Delete file
    async fn delete_file(&self, path: &Path) -> ExecutorResult<()> {
        let full_path = self.validate_path(path)?;

        if full_path.exists() {
            fs::remove_file(&full_path)
                .await
                .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;
        }

        Ok(())
    }
}

#[async_trait]
impl ToolExecutor for FileToolExecutor {
    async fn execute(&self, request: ExecutionRequest) -> ExecutorResult<ToolResult> {
        let start_time = Utc::now();
        let mut metrics = ExecutionMetrics::default();

        let result = match request.parameters.get("action").and_then(Value::as_str) {
            Some("read") => {
                let path = PathBuf::from(
                    request.parameters.get("path")
                        .and_then(Value::as_str)
                        .ok_or_else(|| ExecutorError::MissingParameter {
                            tool: "file".to_string(),
                            parameter: "path".to_string(),
                        })?
                );

                let content = self.read_file(&path).await?;
                metrics.bytes_read = content.len() as u64;
                
                serde_json::json!({
                    "content": content,
                })
            },

            Some("write") => {
                let path = PathBuf::from(
                    request.parameters.get("path")
                        .and_then(Value::as_str)
                        .ok_or_else(|| ExecutorError::MissingParameter {
                            tool: "file".to_string(),
                            parameter: "path".to_string(),
                        })?
                );

                let content = request.parameters.get("content")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "file".to_string(),
                        parameter: "content".to_string(),
                    })?;

                self.write_file(&path, content).await?;
                metrics.bytes_written = content.len() as u64;

                serde_json::json!({
                    "success": true,
                })
            },

            Some("delete") => {
                let path = PathBuf::from(
                    request.parameters.get("path")
                        .and_then(Value::as_str)
                        .ok_or_else(|| ExecutorError::MissingParameter {
                            tool: "file".to_string(),
                            parameter: "path".to_string(),
                        })?
                );

                self.delete_file(&path).await?;

                serde_json::json!({
                    "success": true,
                })
            },

            Some(action) => {
                return Err(ExecutorError::ValidationError {
                    parameter: "action".to_string(),
                    reason: format!("Invalid action: {}", action),
                });
            },

            None => {
                return Err(ExecutorError::MissingParameter {
                    tool: "file".to_string(),
                    parameter: "action".to_string(),
                });
            }
        };

        let end_time = Utc::now();
        metrics.execution_time_ms = (end_time - start_time).num_milliseconds() as u64;

        Ok(ToolResult {
            execution_id: request.execution_id,
            tool_id: request.tool_id,
            status: ExecutionStatus::Completed,
            parameters: request.parameters,
            output: Some(result),
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

        // Validate action
        match parameters.get("action").and_then(Value::as_str) {
            Some(action) => {
                if !["read", "write", "delete"].contains(&action) {
                    errors.push(ValidationError {
                        parameter: "action".to_string(),
                        code: "INVALID_VALUE".to_string(),
                        message: format!("Invalid action: {}", action),
                        expected: Some("read, write, or delete".to_string()),
                        actual: Some(action.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "action".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Action parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate path
        match parameters.get("path").and_then(Value::as_str) {
            Some(path) => {
                if let Err(e) = self.validate_path(Path::new(path)) {
                    errors.push(ValidationError {
                        parameter: "path".to_string(),
                        code: "INVALID_PATH".to_string(),
                        message: e.to_string(),
                        expected: None,
                        actual: Some(path.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "path".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Path parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate content for write action
        if parameters.get("action").and_then(Value::as_str) == Some("write") {
            if parameters.get("content").is_none() {
                errors.push(ValidationError {
                    parameter: "content".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Content parameter is required for write action".to_string(),
                    expected: None,
                    actual: None,
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
        if !context.security.permissions.contains(&"file:read".to_string()) &&
           !context.security.permissions.contains(&"file:write".to_string()) {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_file_tool() {
        let temp_dir = tempdir().unwrap();
        let executor = FileToolExecutor::new(
            temp_dir.path().to_path_buf(),
            1024 * 1024, // 1MB
            vec!["txt".to_string()],
        );

        // Test write
        let write_request = ExecutionRequest {
            execution_id: Uuid::new_v4(),
            tool_id: Uuid::new_v4(),
            parameters: serde_json::json!({
                "action": "write",
                "path": "test.txt",
                "content": "Hello, world!",
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
                    permissions: vec!["file:write".to_string()],
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

        let result = executor.execute(write_request).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Completed);

        // Test read
        let read_request = ExecutionRequest {
            execution_id: Uuid::new_v4(),
            tool_id: Uuid::new_v4(),
            parameters: serde_json::json!({
                "action": "read",
                "path": "test.txt",
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
                    permissions: vec!["file:read".to_string()],
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

        let result = executor.execute(read_request).await.unwrap();
        assert_eq!(
            result.output.unwrap().get("content").unwrap().as_str().unwrap(),
            "Hello, world!"
        );
    }
}