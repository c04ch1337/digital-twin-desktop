//! Use cases for the Digital Twin Desktop application
//! 
//! This module contains all the use cases which orchestrate
//! domain logic to accomplish specific business operations.

pub mod create_conversation;
pub mod send_message;
pub mod create_twin;
pub mod sync_twin;
pub mod run_simulation;
pub mod execute_tool;

// Re-export use cases for convenient access
pub use create_conversation::{CreateConversationCommand, CreateConversationUseCase};
pub use send_message::{SendMessageCommand, SendMessageResponse, SendMessageUseCase};
pub use create_twin::{CreateTwinCommand, CreateTwinUseCase};
pub use sync_twin::{SyncTwinCommand, SyncTwinResponse, SyncTwinUseCase};
pub use run_simulation::{
    RunSimulationCommand, RunSimulationResponse, RunSimulationUseCase,
    SimulationParams, SimulationScenario, PredictedReading
};
pub use execute_tool::{ExecuteToolCommand, ExecuteToolResponse, ExecuteToolUseCase};