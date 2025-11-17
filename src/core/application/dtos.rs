//! Data Transfer Objects for boundary crossing
//! 
//! DTOs are used to transfer data between layers without exposing
//! domain models directly to the presentation layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Agent DTO for external representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDto {
    pub id: String,
    pub name: String,
    pub instructions: String,
    pub model_provider: String,
    pub model_name: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub tool_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Conversation DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationDto {
    pub id: String,
    pub agent_id: String,
    pub messages: Vec<MessageDto>,
    pub context: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Message DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDto {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

/// Digital Twin DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub twin_type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub sensor_data: Vec<SensorDataDto>,
    pub status: String,
    pub simulation_results: Vec<SimulationResultDto>,
    pub metadata: TwinMetadataDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Twin Metadata DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinMetadataDto {
    pub version: String,
    pub schema_version: String,
    pub tags: Vec<String>,
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Sensor Data DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDataDto {
    pub sensor_name: String,
    pub sensor_type: String,
    pub unit: String,
    pub latest_reading: Option<SensorReadingDto>,
    pub reading_count: usize,
}

/// Sensor Reading DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReadingDto {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Simulation Result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultDto {
    pub id: String,
    pub simulation_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub metrics: HashMap<String, f64>,
    pub recommendations: Vec<String>,
}

/// Tool DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub required_parameters: Vec<String>,
    pub parameter_types: HashMap<String, String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tool Execution Result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultDto {
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Create Agent Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequestDto {
    pub name: String,
    pub instructions: String,
    pub model_provider: String,
    pub model_name: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub tool_ids: Vec<String>,
}

/// Create Digital Twin Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTwinRequestDto {
    pub name: String,
    pub description: Option<String>,
    pub twin_type: String,
    pub initial_properties: HashMap<String, serde_json::Value>,
    pub tags: Vec<String>,
}

/// Send Message Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequestDto {
    pub conversation_id: String,
    pub content: String,
}

/// Execute Tool Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteToolRequestDto {
    pub tool_name: String,
    pub conversation_id: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Run Simulation Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSimulationRequestDto {
    pub twin_id: String,
    pub simulation_type: String,
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<SimulationScenarioDto>,
    pub variables: HashMap<String, f64>,
}

/// Simulation Scenario DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioDto {
    pub name: String,
    pub conditions: HashMap<String, f64>,
    pub start_hour: u32,
    pub end_hour: u32,
}

/// Sync Twin Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTwinRequestDto {
    pub twin_id: String,
    pub force_sync: bool,
}

/// Conversation List Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationListDto {
    pub conversations: Vec<ConversationSummaryDto>,
    pub total: usize,
}

/// Conversation Summary DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummaryDto {
    pub id: String,
    pub agent_id: String,
    pub agent_name: String,
    pub message_count: usize,
    pub last_message: Option<MessageDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Twin List Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinListDto {
    pub twins: Vec<TwinSummaryDto>,
    pub total: usize,
}

/// Twin Summary DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinSummaryDto {
    pub id: String,
    pub name: String,
    pub twin_type: String,
    pub status: String,
    pub last_sync: Option<DateTime<Utc>>,
    pub sensor_count: usize,
    pub created_at: DateTime<Utc>,
}

/// Error Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseDto {
    pub error_type: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

/// Pagination Request DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequestDto {
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" or "desc"
}

/// Paginated Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponseDto<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

// Conversion implementations would go here
// These would convert between domain models and DTOs

impl From<crate::core::domain::models::conversation::Message> for MessageDto {
    fn from(message: crate::core::domain::models::conversation::Message) -> Self {
        MessageDto {
            role: message.role,
            content: message.content,
            timestamp: message.timestamp,
            metadata: message.metadata,
        }
    }
}

// Helper functions for DTO conversions

/// Convert page parameters to offset and limit
pub fn pagination_to_offset_limit(pagination: &PaginationRequestDto) -> (usize, usize) {
    let offset = ((pagination.page - 1) * pagination.page_size) as usize;
    let limit = pagination.page_size as usize;
    (offset, limit)
}

/// Create paginated response
pub fn create_paginated_response<T>(
    items: Vec<T>,
    total: usize,
    page: u32,
    page_size: u32,
) -> PaginatedResponseDto<T> {
    let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;
    
    PaginatedResponseDto {
        items,
        total,
        page,
        page_size,
        total_pages,
    }
}