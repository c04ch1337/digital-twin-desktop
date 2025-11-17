//! Tauri commands for tool execution
//!
//! This module provides Tauri commands for registering, configuring,
//! and executing tools within the Digital Twin Desktop.

use tauri::{State, Window};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;

use crate::core::application::services::{
    ToolService, ToolRegistration, ToolExecutionRequest,
    ToolValidationResult
};
use crate::core::domain::models::{
    Tool, ToolType, ToolParameter, ParameterType, 
    SecurityConfig, Permission, ExecutionConfig
};
use crate::api::dto::{
    ApiResponse, ToolSummary, ToolExecutionRequest as ToolExecutionRequestDto,
    ToolExecutionResult
};
use crate::api::error::{ApiResult, map_result};

/// Register a new tool
#[tauri::command]
pub async fn register_tool(
    name: String,
    description: String,
    tool_type: String,
    parameters: Vec<Value>,
    permissions: Vec<String>,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<ToolSummary> {
    // Parse tool type
    let tool_type = match tool_type.as_str() {
        "file" => ToolType::File,
        "web" => ToolType::Web,
        "modbus" => ToolType::Modbus,
        "mqtt" => ToolType::MQTT,
        "twin" => ToolType::Twin,
        custom => ToolType::Custom(custom.to_string()),
    };
    
    // Parse parameters
    let tool_parameters = parameters.iter()
        .map(|param| {
            let name = param.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unnamed")
                .to_string();
            
            let description = param.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let param_type_str = param.get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("string");
            
            let param_type = match param_type_str {
                "string" => ParameterType::String,
                "number" => ParameterType::Number,
                "boolean" => ParameterType::Boolean,
                "object" => ParameterType::Object,
                "array" => ParameterType::Array,
                _ => ParameterType::String,
            };
            
            let required = param.get("required")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            let default_value = param.get("default");
            
            ToolParameter {
                name,
                description,
                parameter_type: param_type,
                required,
                default_value: default_value.cloned(),
                validation_rules: None,
            }
        })
        .collect();
    
    // Parse permissions
    let tool_permissions = permissions.iter()
        .map(|perm_str| {
            match perm_str.as_str() {
                "file_read" => Permission::FileRead,
                "file_write" => Permission::FileWrite,
                "network" => Permission::Network,
                "twin_read" => Permission::TwinRead,
                "twin_write" => Permission::TwinWrite,
                "system" => Permission::System,
                custom => Permission::Custom(custom.to_string()),
            }
        })
        .collect();
    
    // Create security config
    let security_config = SecurityConfig {
        permissions: tool_permissions,
        auth_requirement: None,
        sandbox_config: None,
        audit_config: None,
    };
    
    // Create execution config
    let execution_config = ExecutionConfig {
        timeout_ms: 30000,
        retry_config: None,
        resource_limits: None,
        concurrency_config: None,
    };
    
    // Create tool registration
    let registration = ToolRegistration {
        name,
        description,
        tool_type,
        parameters: tool_parameters,
        security_config,
        execution_config,
    };
    
    // Register the tool
    let result = tool_service.register_tool(registration).await;
    let tool = map_result(result)?;
    
    // Convert to DTO
    Ok(crate::api::dto::converters::tool_to_summary(&tool))
}

/// Get tool by ID
#[tauri::command]
pub async fn get_tool(
    tool_id: String,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&tool_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.get_tool(id).await;
    let tool = map_result(result)?;
    
    // Convert to detailed JSON response
    let parameters = tool.parameters.iter()
        .map(|param| {
            serde_json::json!({
                "name": param.name,
                "description": param.description,
                "type": format!("{:?}", param.parameter_type),
                "required": param.required,
                "default_value": param.default_value,
            })
        })
        .collect::<Vec<_>>();
    
    let permissions = tool.security_config.permissions.iter()
        .map(|perm| format!("{:?}", perm))
        .collect::<Vec<_>>();
    
    let response = serde_json::json!({
        "id": tool.id,
        "name": tool.metadata.name,
        "description": tool.metadata.description,
        "tool_type": format!("{:?}", tool.tool_type),
        "created_at": tool.metadata.created_at,
        "updated_at": tool.metadata.updated_at,
        "status": format!("{:?}", tool.status),
        "parameters": parameters,
        "permissions": permissions,
        "execution_config": {
            "timeout_ms": tool.execution_config.timeout_ms,
        },
        "usage": {
            "total_executions": tool.usage.total_executions,
            "successful_executions": tool.usage.successful_executions,
            "failed_executions": tool.usage.failed_executions,
            "average_execution_time_ms": tool.usage.average_execution_time_ms,
        },
    });
    
    Ok(response)
}

/// List all tools
#[tauri::command]
pub async fn list_tools(
    tool_type: Option<String>,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<Vec<ToolSummary>> {
    // Filter by tool type if provided
    let type_filter = tool_type.map(|t| match t.as_str() {
        "file" => ToolType::File,
        "web" => ToolType::Web,
        "modbus" => ToolType::Modbus,
        "mqtt" => ToolType::MQTT,
        "twin" => ToolType::Twin,
        custom => ToolType::Custom(custom.to_string()),
    });
    
    // Get tools
    let result = match type_filter {
        Some(tool_type) => tool_service.list_tools_by_type(tool_type).await,
        None => tool_service.list_all_tools().await,
    };
    
    let tools = map_result(result)?;
    
    // Convert to DTOs
    let summaries = tools.iter()
        .map(|tool| crate::api::dto::converters::tool_to_summary(tool))
        .collect();
    
    Ok(summaries)
}

/// Execute a tool
#[tauri::command]
pub async fn execute_tool(
    request: ToolExecutionRequestDto,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<ToolExecutionResult> {
    // Create execution request
    let execution_request = ToolExecutionRequest {
        tool_id: request.tool_id,
        parameters: request.parameters,
        context: request.context,
    };
    
    // Execute the tool
    let result = tool_service.execute_tool(execution_request).await;
    let execution_result = map_result(result)?;
    
    // Convert to DTO
    Ok(ToolExecutionResult {
        execution_id: execution_result.execution_id,
        tool_id: execution_result.tool_id,
        status: format!("{:?}", execution_result.status),
        result: execution_result.result,
        error: execution_result.error.map(|e| e.to_string()),
        duration_ms: execution_result.duration_ms,
    })
}

/// Stream tool execution
///
/// This command supports streaming responses for long-running tool executions
#[tauri::command]
pub async fn stream_tool_execution(
    request: ToolExecutionRequestDto,
    window: Window,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<String> {
    // Create execution request
    let execution_request = ToolExecutionRequest {
        tool_id: request.tool_id,
        parameters: request.parameters,
        context: request.context,
    };
    
    // Start streaming execution
    let (execution_id, mut receiver) = tool_service.execute_tool_streaming(execution_request).await?;
    
    // Spawn a task to handle the streaming
    tauri::async_runtime::spawn(async move {
        while let Some(update) = receiver.recv().await {
            // Convert to DTO
            let update_dto = ToolExecutionResult {
                execution_id: update.execution_id,
                tool_id: update.tool_id,
                status: format!("{:?}", update.status),
                result: update.result,
                error: update.error.map(|e| e.to_string()),
                duration_ms: update.duration_ms,
            };
            
            // Emit event to the frontend
            let _ = window.emit("tool:execution_update", update_dto);
        }
    });
    
    Ok(execution_id.to_string())
}

/// Cancel a tool execution
#[tauri::command]
pub async fn cancel_tool_execution(
    execution_id: String,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&execution_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.cancel_execution(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Get tool execution status
#[tauri::command]
pub async fn get_execution_status(
    execution_id: String,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<ToolExecutionResult> {
    let id = Uuid::parse_str(&execution_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.get_execution_status(id).await;
    let status = map_result(result)?;
    
    // Convert to DTO
    Ok(ToolExecutionResult {
        execution_id: status.execution_id,
        tool_id: status.tool_id,
        status: format!("{:?}", status.status),
        result: status.result,
        error: status.error.map(|e| e.to_string()),
        duration_ms: status.duration_ms,
    })
}

/// Validate tool parameters
#[tauri::command]
pub async fn validate_tool_parameters(
    tool_id: String,
    parameters: Value,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&tool_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.validate_parameters(id, parameters).await;
    let validation = map_result(result)?;
    
    // Convert to JSON response
    let response = match validation {
        ToolValidationResult::Valid => {
            serde_json::json!({
                "valid": true,
            })
        },
        ToolValidationResult::Invalid(errors) => {
            let error_map = errors.iter()
                .map(|(param, error)| (param.clone(), error.clone()))
                .collect::<std::collections::HashMap<_, _>>();
            
            serde_json::json!({
                "valid": false,
                "errors": error_map,
            })
        },
    };
    
    Ok(response)
}

/// Update tool configuration
#[tauri::command]
pub async fn update_tool_configuration(
    tool_id: String,
    timeout_ms: Option<u64>,
    enabled: Option<bool>,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&tool_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.update_tool_configuration(id, timeout_ms, enabled).await;
    map_result(result)?;
    
    Ok(true)
}

/// Delete a tool
#[tauri::command]
pub async fn delete_tool(
    tool_id: String,
    tool_service: State<'_, Arc<ToolService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&tool_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = tool_service.delete_tool(id).await;
    map_result(result)?;
    
    Ok(true)
}