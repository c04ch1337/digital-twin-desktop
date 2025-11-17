//! Tauri commands for agent operations
//!
//! This module provides Tauri commands for creating, configuring,
//! and interacting with AI agents.

use tauri::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;

use crate::core::application::services::AgentService;
use crate::core::domain::models::{
    Agent, AgentCapability, AgentConfiguration, CapabilityType,
    MemoryConfiguration, MemoryType, LongTermMemoryConfig
};
use crate::api::dto::{
    ApiResponse, AgentSummary, CreateAgentRequest
};
use crate::api::error::{ApiResult, map_result};

/// Create a new agent
#[tauri::command]
pub async fn create_agent(
    request: CreateAgentRequest,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<AgentSummary> {
    // Parse capabilities from strings
    let capabilities = request.capabilities.iter()
        .map(|cap_str| {
            let capability_type = match cap_str.as_str() {
                "conversation" => CapabilityType::Conversation,
                "tool_use" => CapabilityType::ToolUse,
                "twin_management" => CapabilityType::TwinManagement,
                "simulation" => CapabilityType::Simulation,
                "data_analysis" => CapabilityType::DataAnalysis,
                custom => CapabilityType::Custom(custom.to_string()),
            };
            
            AgentCapability {
                capability_type,
                enabled: true,
                configuration: serde_json::json!({}),
            }
        })
        .collect();
    
    // Create agent configuration
    let config = AgentConfiguration {
        model: request.configuration.get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("gpt-4")
            .to_string(),
        memory: MemoryConfiguration {
            memory_type: MemoryType::Hybrid,
            long_term: LongTermMemoryConfig {
                enabled: true,
                max_tokens: 100000,
            },
        },
        parameters: request.configuration.clone(),
    };
    
    // Create the agent
    let result = agent_service.create_agent(
        request.name,
        request.description,
        capabilities,
        config,
    ).await;
    
    let agent = map_result(result)?;
    
    // Convert to DTO
    Ok(crate::api::dto::converters::agent_to_summary(&agent))
}

/// Get agent by ID
#[tauri::command]
pub async fn get_agent(
    agent_id: String,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = agent_service.get_agent(id).await;
    let agent = map_result(result)?;
    
    // Convert to detailed JSON response
    let capabilities = agent.capabilities.iter()
        .map(|cap| {
            serde_json::json!({
                "type": format!("{:?}", cap.capability_type),
                "enabled": cap.enabled,
                "configuration": cap.configuration,
            })
        })
        .collect::<Vec<_>>();
    
    let response = serde_json::json!({
        "id": agent.id,
        "name": agent.metadata.name,
        "description": agent.metadata.description,
        "created_at": agent.metadata.created_at,
        "updated_at": agent.metadata.updated_at,
        "state": format!("{:?}", agent.state),
        "capabilities": capabilities,
        "configuration": {
            "model": agent.configuration.model,
            "memory_type": format!("{:?}", agent.configuration.memory.memory_type),
            "long_term_memory": {
                "enabled": agent.configuration.memory.long_term.enabled,
                "max_tokens": agent.configuration.memory.long_term.max_tokens,
            },
            "parameters": agent.configuration.parameters,
        },
        "metrics": {
            "total_conversations": agent.metrics.total_conversations,
            "total_messages": agent.metrics.total_messages,
            "average_response_time_ms": agent.metrics.average_response_time_ms,
        }
    });
    
    Ok(response)
}

/// List all agents
#[tauri::command]
pub async fn list_agents(
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<Vec<AgentSummary>> {
    let result = agent_service.list_agents().await;
    let agents = map_result(result)?;
    
    // Convert to DTOs
    let summaries = agents.iter()
        .map(|agent| crate::api::dto::converters::agent_to_summary(agent))
        .collect();
    
    Ok(summaries)
}

/// Update agent configuration
#[tauri::command]
pub async fn update_agent_configuration(
    agent_id: String,
    configuration: Value,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Extract configuration parameters
    let model = configuration.get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let memory_enabled = configuration.get("memory_enabled")
        .and_then(|v| v.as_bool());
    
    let max_tokens = configuration.get("max_tokens")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);
    
    // Update the agent configuration
    let result = agent_service.update_agent_configuration(
        id, model, memory_enabled, max_tokens, Some(configuration)
    ).await;
    
    map_result(result)?;
    
    Ok(true)
}

/// Enable or disable agent capability
#[tauri::command]
pub async fn set_agent_capability(
    agent_id: String,
    capability: String,
    enabled: bool,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Parse capability type
    let capability_type = match capability.as_str() {
        "conversation" => CapabilityType::Conversation,
        "tool_use" => CapabilityType::ToolUse,
        "twin_management" => CapabilityType::TwinManagement,
        "simulation" => CapabilityType::Simulation,
        "data_analysis" => CapabilityType::DataAnalysis,
        custom => CapabilityType::Custom(custom.to_string()),
    };
    
    // Update the capability
    let result = agent_service.set_capability(id, capability_type, enabled).await;
    map_result(result)?;
    
    Ok(true)
}

/// Delete an agent
#[tauri::command]
pub async fn delete_agent(
    agent_id: String,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = agent_service.delete_agent(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Assign agent to a conversation
#[tauri::command]
pub async fn assign_agent_to_conversation(
    agent_id: String,
    conversation_id: String,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<bool> {
    let agent_id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let conversation_id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = agent_service.assign_to_conversation(agent_id, conversation_id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Get agent metrics
#[tauri::command]
pub async fn get_agent_metrics(
    agent_id: String,
    agent_service: State<'_, Arc<AgentService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&agent_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = agent_service.get_agent_metrics(id).await;
    let metrics = map_result(result)?;
    
    let response = serde_json::json!({
        "total_conversations": metrics.total_conversations,
        "total_messages": metrics.total_messages,
        "average_response_time_ms": metrics.average_response_time_ms,
        "token_usage": {
            "prompt_tokens": metrics.prompt_tokens,
            "completion_tokens": metrics.completion_tokens,
            "total_tokens": metrics.prompt_tokens + metrics.completion_tokens,
        },
        "success_rate": metrics.success_rate,
    });
    
    Ok(response)
}