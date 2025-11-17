use crate::core::{
    domain::{
        errors::DomainError,
        models::tool::{Tool, ToolId, ToolCategory, ToolResult, ToolParameters},
        traits::repository::ToolRepository,
        traits::tool_executor::ToolExecutor,
    },
};
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;
use serde_json::Value;

/// Service for tool registry and execution
pub struct ToolService {
    tool_repo: Arc<dyn ToolRepository>,
    tool_executors: HashMap<String, Arc<dyn ToolExecutor>>,
}

/// Tool registration request
#[derive(Debug, Clone)]
pub struct ToolRegistration {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub required_parameters: Vec<String>,
    pub parameter_types: HashMap<String, String>,
    pub config: Value,
}

/// Tool execution request
#[derive(Debug, Clone)]
pub struct ToolExecutionRequest {
    pub tool_name: String,
    pub parameters: ToolParameters,
}

/// Tool validation result
#[derive(Debug)]
pub struct ToolValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ToolService {
    pub fn new(
        tool_repo: Arc<dyn ToolRepository>,
        tool_executors: HashMap<String, Arc<dyn ToolExecutor>>,
    ) -> Self {
        Self {
            tool_repo,
            tool_executors,
        }
    }

    /// Register a new tool
    pub async fn register_tool(
        &self,
        registration: ToolRegistration,
    ) -> Result<Tool, DomainError> {
        // Validate registration
        if registration.name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Tool name cannot be empty".to_string(),
            ));
        }

        if registration.description.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Tool description cannot be empty".to_string(),
            ));
        }

        // Check if tool already exists
        if let Some(_) = self.tool_repo
            .find_by_name(&registration.name)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
        {
            return Err(DomainError::ValidationError(
                format!("Tool with name '{}' already exists", registration.name)
            ));
        }

        // Validate parameter types
        for param in &registration.required_parameters {
            if !registration.parameter_types.contains_key(param) {
                return Err(DomainError::ValidationError(
                    format!("Type not specified for required parameter '{}'", param)
                ));
            }
        }

        // Create tool
        let tool = Tool::new(
            ToolId::new(),
            registration.name,
            registration.description,
            registration.category,
            registration.required_parameters,
            registration.parameter_types,
            registration.config,
            true, // Enable by default
            Utc::now(),
            Utc::now(),
        );

        // Save to repository
        self.tool_repo
            .save(&tool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(tool)
    }

    /// Get tool by ID
    pub async fn get_tool(&self, tool_id: &ToolId) -> Result<Option<Tool>, DomainError> {
        self.tool_repo
            .find_by_id(tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Get tool by name
    pub async fn get_tool_by_name(&self, name: &str) -> Result<Option<Tool>, DomainError> {
        self.tool_repo
            .find_by_name(name)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// List all tools
    pub async fn list_tools(&self) -> Result<Vec<Tool>, DomainError> {
        self.tool_repo
            .find_all()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// List tools by category
    pub async fn list_tools_by_category(
        &self,
        category: &ToolCategory,
    ) -> Result<Vec<Tool>, DomainError> {
        self.tool_repo
            .find_by_category(category)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Execute a tool
    pub async fn execute_tool(
        &self,
        request: ToolExecutionRequest,
    ) -> Result<ToolResult, DomainError> {
        // Get tool definition
        let tool = self
            .tool_repo
            .find_by_name(&request.tool_name)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(
                format!("Tool not found: {}", request.tool_name)
            ))?;

        // Check if tool is enabled
        if !tool.enabled {
            return Err(DomainError::ValidationError(
                format!("Tool '{}' is disabled", tool.name)
            ));
        }

        // Get executor for this tool
        let executor = self.tool_executors
            .get(&tool.name)
            .ok_or_else(|| DomainError::InvalidState(
                format!("No executor registered for tool '{}'", tool.name)
            ))?;

        // Validate parameters
        self.validate_parameters(&tool, &request.parameters)?;

        // Execute the tool
        executor
            .execute(&tool, &request.parameters)
            .await
            .map_err(|e| DomainError::ExternalServiceError(
                format!("Tool execution failed: {}", e)
            ))
    }

    /// Validate tool parameters
    pub async fn validate_tool(
        &self,
        tool_name: &str,
        parameters: &ToolParameters,
    ) -> Result<ToolValidationResult, DomainError> {
        let tool = self
            .tool_repo
            .find_by_name(tool_name)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(
                format!("Tool not found: {}", tool_name)
            ))?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check required parameters
        for required_param in &tool.required_parameters {
            if !parameters.contains_key(required_param) {
                errors.push(format!("Missing required parameter: {}", required_param));
            }
        }

        // Check parameter types
        for (param_name, param_value) in parameters {
            if let Some(expected_type) = tool.parameter_types.get(param_name) {
                if !self.is_valid_type(param_value, expected_type) {
                    errors.push(format!(
                        "Invalid type for parameter '{}'. Expected: {}",
                        param_name, expected_type
                    ));
                }
            } else {
                warnings.push(format!("Unknown parameter: {}", param_name));
            }
        }

        Ok(ToolValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        })
    }

    /// Enable or disable a tool
    pub async fn set_tool_enabled(
        &self,
        tool_id: &ToolId,
        enabled: bool,
    ) -> Result<Tool, DomainError> {
        let mut tool = self
            .tool_repo
            .find_by_id(tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Tool not found".to_string()))?;

        tool.enabled = enabled;
        tool.updated_at = Utc::now();

        self.tool_repo
            .update(&tool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(tool)
    }

    /// Update tool configuration
    pub async fn update_tool_config(
        &self,
        tool_id: &ToolId,
        config: Value,
    ) -> Result<Tool, DomainError> {
        let mut tool = self
            .tool_repo
            .find_by_id(tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Tool not found".to_string()))?;

        tool.config = config;
        tool.updated_at = Utc::now();

        self.tool_repo
            .update(&tool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(tool)
    }

    /// Delete a tool
    pub async fn delete_tool(&self, tool_id: &ToolId) -> Result<(), DomainError> {
        // Verify tool exists
        self.tool_repo
            .find_by_id(tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Tool not found".to_string()))?;

        self.tool_repo
            .delete(tool_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Register a tool executor
    pub fn register_executor(&mut self, tool_name: String, executor: Arc<dyn ToolExecutor>) {
        self.tool_executors.insert(tool_name, executor);
    }

    /// Get available executors
    pub fn get_available_executors(&self) -> Vec<String> {
        self.tool_executors.keys().cloned().collect()
    }

    // Helper methods

    fn validate_parameters(
        &self,
        tool: &Tool,
        parameters: &ToolParameters,
    ) -> Result<(), DomainError> {
        // Check required parameters
        for required_param in &tool.required_parameters {
            if !parameters.contains_key(required_param) {
                return Err(DomainError::ValidationError(
                    format!("Missing required parameter: {}", required_param)
                ));
            }
        }

        // Validate parameter types
        for (param_name, param_value) in parameters {
            if let Some(expected_type) = tool.parameter_types.get(param_name) {
                if !self.is_valid_type(param_value, expected_type) {
                    return Err(DomainError::ValidationError(
                        format!(
                            "Invalid type for parameter '{}'. Expected: {}, Got: {}",
                            param_name,
                            expected_type,
                            self.get_value_type(param_value)
                        )
                    ));
                }
            }
        }

        Ok(())
    }

    fn is_valid_type(&self, value: &Value, expected_type: &str) -> bool {
        match (expected_type, value) {
            ("string", Value::String(_)) => true,
            ("number", Value::Number(_)) => true,
            ("boolean", Value::Bool(_)) => true,
            ("object", Value::Object(_)) => true,
            ("array", Value::Array(_)) => true,
            _ => false,
        }
    }

    fn get_value_type(&self, value: &Value) -> &'static str {
        match value {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Bool(_) => "boolean",
            Value::Object(_) => "object",
            Value::Array(_) => "array",
            Value::Null => "null",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use std::time::Duration;

    struct MockToolRepository {
        tools: Arc<Mutex<Vec<Tool>>>,
    }

    #[async_trait]
    impl ToolRepository for MockToolRepository {
        async fn save(&self, tool: &Tool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.tools.lock().unwrap().push(tool.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &ToolId) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            let tools = self.tools.lock().unwrap();
            Ok(tools.iter().find(|t| t.id == *id).cloned())
        }

        async fn find_by_name(&self, name: &str) -> Result<Option<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            let tools = self.tools.lock().unwrap();
            Ok(tools.iter().find(|t| t.name == name).cloned())
        }

        async fn find_by_category(&self, category: &ToolCategory) -> Result<Vec<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            let tools = self.tools.lock().unwrap();
            Ok(tools.iter()
                .filter(|t| t.category == *category)
                .cloned()
                .collect())
        }

        async fn find_all(&self) -> Result<Vec<Tool>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.tools.lock().unwrap().clone())
        }

        async fn update(&self, tool: &Tool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut tools = self.tools.lock().unwrap();
            if let Some(index) = tools.iter().position(|t| t.id == tool.id) {
                tools[index] = tool.clone();
            }
            Ok(())
        }

        async fn delete(&self, id: &ToolId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut tools = self.tools.lock().unwrap();
            tools.retain(|t| t.id != *id);
            Ok(())
        }
    }

    struct MockToolExecutor {}

    #[async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(
            &self,
            _tool: &Tool,
            _parameters: &ToolParameters,
        ) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ToolResult {
                success: true,
                output: serde_json::json!({ "message": "Tool executed successfully" }),
                error: None,
                execution_time: Duration::from_millis(100),
            })
        }

        async fn validate(
            &self,
            _tool: &Tool,
        ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_register_and_execute_tool() {
        let tools = Arc::new(Mutex::new(Vec::new()));
        let tool_repo = Arc::new(MockToolRepository { tools: tools.clone() });
        
        let mut tool_executors = HashMap::new();
        tool_executors.insert("test_tool".to_string(), Arc::new(MockToolExecutor {}) as Arc<dyn ToolExecutor>);
        
        let service = ToolService::new(tool_repo, tool_executors);
        
        // Register tool
        let mut parameter_types = HashMap::new();
        parameter_types.insert("param1".to_string(), "string".to_string());
        
        let registration = ToolRegistration {
            name: "test_tool".to_string(),
            description: "Test tool".to_string(),
            category: ToolCategory::Utility,
            required_parameters: vec!["param1".to_string()],
            parameter_types,
            config: serde_json::json!({}),
        };
        
        let result = service.register_tool(registration).await;
        assert!(result.is_ok());
        
        // Execute tool
        let mut parameters = ToolParameters::new();
        parameters.insert("param1".to_string(), serde_json::json!("test value"));
        
        let exec_request = ToolExecutionRequest {
            tool_name: "test_tool".to_string(),
            parameters,
        };
        
        let exec_result = service.execute_tool(exec_request).await;
        assert!(exec_result.is_ok());
        assert!(exec_result.unwrap().success);
    }

    #[tokio::test]
    async fn test_validate_tool_parameters() {
        let tool_id = ToolId::new();
        
        let mut parameter_types = HashMap::new();
        parameter_types.insert("required_param".to_string(), "string".to_string());
        parameter_types.insert("optional_param".to_string(), "number".to_string());
        
        let tool = Tool::new(
            tool_id,
            "validation_test".to_string(),
            "Validation test tool".to_string(),
            ToolCategory::Utility,
            vec!["required_param".to_string()],
            parameter_types,
            serde_json::json!({}),
            true,
            Utc::now(),
            Utc::now(),
        );
        
        let tools = Arc::new(Mutex::new(vec![tool]));
        let tool_repo = Arc::new(MockToolRepository { tools });
        
        let service = ToolService::new(tool_repo, HashMap::new());
        
        // Test missing required parameter
        let mut parameters = ToolParameters::new();
        parameters.insert("optional_param".to_string(), serde_json::json!(42));
        
        let validation = service.validate_tool("validation_test", &parameters).await;
        assert!(validation.is_ok());
        let result = validation.unwrap();
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        
        // Test valid parameters
        parameters.insert("required_param".to_string(), serde_json::json!("test"));
        
        let validation = service.validate_tool("validation_test", &parameters).await;
        assert!(validation.is_ok());
        let result = validation.unwrap();
        assert!(result.is_valid);
        assert_eq!(result.errors.len(), 0);
    }
}