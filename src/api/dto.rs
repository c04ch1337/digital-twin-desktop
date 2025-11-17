//! Data Transfer Objects for the API layer
//!
//! This module contains DTOs that are specifically designed for
//! frontend communication, optimized for JSON serialization and
//! frontend consumption patterns.

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Base response wrapper for all API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data (present only if success is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error information (present only if success is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::api::error::ApiError>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(error: crate::api::error::ApiError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

//
// Conversation DTOs
//

/// DTO for conversation creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    /// Conversation title
    pub title: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional initial message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message: Option<String>,
    /// Optional agent ID to assign to conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Uuid>,
}

/// DTO for conversation summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// Conversation ID
    pub id: Uuid,
    /// Conversation title
    pub title: String,
    /// Last message timestamp
    pub last_activity: DateTime<Utc>,
    /// Message count
    pub message_count: usize,
    /// Conversation state
    pub state: String,
}

/// DTO for sending a message
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// Conversation ID
    pub conversation_id: Uuid,
    /// Message content
    pub content: String,
    /// Optional attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentDto>>,
}

/// DTO for message response
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDto {
    /// Message ID
    pub id: Uuid,
    /// Conversation ID
    pub conversation_id: Uuid,
    /// Sender type (user, agent, system)
    pub sender: String,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Optional attachments
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attachments: Vec<AttachmentDto>,
}

/// DTO for message attachments
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDto {
    /// Attachment ID
    pub id: Uuid,
    /// Attachment name
    pub name: String,
    /// MIME type
    pub mime_type: String,
    /// Size in bytes
    pub size: usize,
    /// URL or data URI
    pub url: String,
}

//
// Agent DTOs
//

/// DTO for agent creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Agent capabilities
    pub capabilities: Vec<String>,
    /// Agent configuration
    pub configuration: serde_json::Value,
}

/// DTO for agent summary
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSummary {
    /// Agent ID
    pub id: Uuid,
    /// Agent name
    pub name: String,
    /// Agent status
    pub status: String,
    /// Agent capabilities
    pub capabilities: Vec<String>,
}

//
// Digital Twin DTOs
//

/// DTO for digital twin creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTwinRequest {
    /// Twin name
    pub name: String,
    /// Twin type
    pub twin_type: String,
    /// Optional configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}

/// DTO for digital twin summary
#[derive(Debug, Serialize, Deserialize)]
pub struct TwinSummary {
    /// Twin ID
    pub id: Uuid,
    /// Twin name
    pub name: String,
    /// Twin type
    pub twin_type: String,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Last update date
    pub updated_at: DateTime<Utc>,
}

//
// Simulation DTOs
//

/// DTO for simulation creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSimulationRequest {
    /// Digital twin ID
    pub twin_id: Uuid,
    /// Simulation name
    pub name: String,
    /// Simulation parameters
    pub parameters: serde_json::Value,
    /// Optional duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u64>,
}

/// DTO for simulation status
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationStatusDto {
    /// Simulation ID
    pub id: Uuid,
    /// Digital twin ID
    pub twin_id: Uuid,
    /// Simulation name
    pub name: String,
    /// Current status
    pub status: String,
    /// Progress percentage (0-100)
    pub progress: f32,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// End time (if completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

//
// Tool DTOs
//

/// DTO for tool execution request
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolExecutionRequest {
    /// Tool ID
    pub tool_id: Uuid,
    /// Tool parameters
    pub parameters: serde_json::Value,
    /// Optional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

/// DTO for tool execution result
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    /// Execution ID
    pub execution_id: Uuid,
    /// Tool ID
    pub tool_id: Uuid,
    /// Execution status
    pub status: String,
    /// Result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
}

/// DTO for tool summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolSummary {
    /// Tool ID
    pub id: Uuid,
    /// Tool name
    pub name: String,
    /// Tool type
    pub tool_type: String,
    /// Tool description
    pub description: String,
    /// Required permissions
    pub permissions: Vec<String>,
}

/// Helper function to convert domain models to DTOs
pub mod converters {
    use super::*;
    use crate::core::domain::models::*;

    /// Convert domain Message to MessageDto
    pub fn message_to_dto(message: &Message) -> MessageDto {
        MessageDto {
            id: message.id,
            conversation_id: message.conversation_id,
            sender: format!("{:?}", message.sender),
            content: message.content.clone(),
            timestamp: message.metadata.timestamp,
            attachments: message.metadata.attachments
                .iter()
                .map(attachment_to_dto)
                .collect(),
        }
    }

    /// Convert domain Attachment to AttachmentDto
    pub fn attachment_to_dto(attachment: &Attachment) -> AttachmentDto {
        AttachmentDto {
            id: Uuid::new_v4(), // Assuming domain model doesn't have ID
            name: attachment.name.clone(),
            mime_type: attachment.content_type.to_string(),
            size: attachment.size,
            url: attachment.url.clone(),
        }
    }

    /// Convert domain Agent to AgentSummary
    pub fn agent_to_summary(agent: &Agent) -> AgentSummary {
        AgentSummary {
            id: agent.id,
            name: agent.metadata.name.clone(),
            status: format!("{:?}", agent.state),
            capabilities: agent.capabilities
                .iter()
                .map(|cap| format!("{:?}", cap.capability_type))
                .collect(),
        }
    }

    /// Convert domain DigitalTwin to TwinSummary
    pub fn twin_to_summary(twin: &DigitalTwin) -> TwinSummary {
        TwinSummary {
            id: twin.id,
            name: twin.metadata.name.clone(),
            twin_type: format!("{:?}", twin.twin_type),
            created_at: twin.metadata.created_at,
            updated_at: twin.metadata.updated_at,
        }
    }

    /// Convert domain Tool to ToolSummary
    pub fn tool_to_summary(tool: &Tool) -> ToolSummary {
        ToolSummary {
            id: tool.id,
            name: tool.metadata.name.clone(),
            tool_type: format!("{:?}", tool.tool_type),
            description: tool.metadata.description.clone(),
            permissions: tool.security_config.permissions
                .iter()
                .map(|perm| format!("{:?}", perm))
                .collect(),
        }
    }
}