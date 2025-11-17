//! Tauri commands that can be invoked from the frontend
//!
//! This module exports all command modules for the Digital Twin Desktop application.
//! Each submodule contains related commands for a specific domain area.

// Re-export existing commands for backward compatibility
pub use crate::core::domain::models::*;
pub use anyhow::Result;

// Export command modules
pub mod conversation_commands;
pub mod agent_commands;
pub mod twin_commands;
pub mod simulation_commands;
pub mod tool_commands;

// Re-export all commands for convenient access
pub use conversation_commands::*;
pub use agent_commands::*;
pub use twin_commands::*;
pub use simulation_commands::*;
pub use tool_commands::*;

// Legacy commands - kept for backward compatibility
// These will be removed in a future version

#[tauri::command]
pub async fn create_digital_twin(name: String, twin_type: String) -> Result<DigitalTwin, String> {
    // Create a new digital twin
    let twin_type = match twin_type.as_str() {
        "industrial" => TwinType::Industrial,
        "infrastructure" => TwinType::Infrastructure,
        "process" => TwinType::Process,
        custom => TwinType::Custom(custom.to_string()),
    };
    
    let twin = DigitalTwin {
        id: uuid::Uuid::new_v4(),
        metadata: TwinMetadata {
            name,
            description: "".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        twin_type,
        configuration: serde_json::json!({}),
        data_sources: vec![],
        properties: TwinProperties::default(),
        state: TwinState::Inactive,
        sync_configuration: SyncConfiguration {
            mode: SyncMode::Manual,
            interval_seconds: 60,
            enabled: false,
            retention_policy: RetentionPolicy::KeepAll,
        },
    };
    
    Ok(twin)
}

#[tauri::command]
pub async fn list_digital_twins() -> Result<Vec<DigitalTwin>, String> {
    // List all digital twins
    Ok(vec![])
}

#[tauri::command]
pub async fn start_simulation(twin_id: String, parameters: serde_json::Value) -> Result<Simulation, String> {
    // Start a new simulation
    let simulation = Simulation {
        id: uuid::Uuid::new_v4(),
        twin_id: uuid::Uuid::parse_str(&twin_id).unwrap_or_default(),
        name: "Simulation".to_string(),
        status: SimulationStatus::Pending,
        parameters,
        progress: 0.0,
        started_at: chrono::Utc::now(),
        completed_at: None,
        results: None,
    };
    
    Ok(simulation)
}

#[tauri::command]
pub async fn generate_with_ai(prompt: String) -> Result<String, String> {
    // Generate content with AI
    Ok(format!("AI response to: {}", prompt))
}