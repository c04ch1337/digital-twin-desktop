//! Application services for the Digital Twin Desktop
//!
//! This module contains services that orchestrate domain logic
//! and coordinate between multiple use cases.

pub mod conversation_service;
pub mod agent_service;
pub mod twin_service;
pub mod simulation_service;
pub mod tool_service;
pub mod memory_manager;
pub mod prompt_manager;

// Re-export services for convenient access
pub use conversation_service::ConversationService;
pub use agent_service::AgentService;
pub use twin_service::TwinService;
pub use simulation_service::{
    SimulationService, SimulationConfig, ScenarioConfig,
    BatchSimulationRequest, BatchSimulationResult,
    SimulationAnalysis, MetricsSummary
};
pub use tool_service::{
    ToolService, ToolRegistration, ToolExecutionRequest,
    ToolValidationResult
};
pub use memory_manager::{
    MemoryManager, MemoryConfig, MemoryStrategy, TokenModel,
    ContextWindow, TokenStats
};
pub use prompt_manager::{
    PromptManager, VersionedPrompt, PromptChange, PromptDiff
};