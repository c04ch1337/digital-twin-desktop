//! API module for the Digital Twin Desktop application
//!
//! This module provides the interface between the frontend and the
//! application core, including Tauri commands, DTOs, and error handling.

// Export command modules
pub mod commands;

// Export error handling
pub mod error;

// Export DTOs
pub mod dto;

// Re-export API items for convenient access
pub use commands::*;
pub use error::{ApiError, ApiResult, ErrorCode, map_domain_error, map_result, to_api_error};
pub use dto::{
    ApiResponse, ConversationSummary, CreateConversationRequest, MessageDto, SendMessageRequest,
    AgentSummary, CreateAgentRequest, TwinSummary, CreateTwinRequest, SimulationStatusDto,
    CreateSimulationRequest, ToolSummary, ToolExecutionRequest, ToolExecutionResult,
    AttachmentDto
};

/// Initialize API services
///
/// This function is called during application startup to initialize
/// all API-related services and dependencies.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing API services");
    
    // Initialize any API-specific services here
    
    Ok(())
}