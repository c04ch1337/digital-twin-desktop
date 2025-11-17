//! Command objects for use case inputs
//! 
//! Commands represent intentions to change the state of the system.
//! They encapsulate all the data needed to perform an operation.

use crate::core::domain::{
    models::{
        agent::{AgentId, ModelConfig},
        conversation::ConversationId,
        digital_twin::TwinId,
        tool::ToolId,
    },
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Command to create a new agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentCommand {
    pub name: String,
    pub instructions: String,
    pub model_config: ModelConfig,
    pub tool_ids: Vec<ToolId>,
}

/// Command to update an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentCommand {
    pub agent_id: AgentId,
    pub name: Option<String>,
    pub instructions: Option<String>,
    pub model_config: Option<ModelConfig>,
    pub tool_ids: Option<Vec<ToolId>>,
}

/// Command to delete an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentCommand {
    pub agent_id: AgentId,
}

/// Command to start a new conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartConversationCommand {
    pub agent_id: AgentId,
    pub initial_context: Option<String>,
}

/// Command to send a message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageCommand {
    pub conversation_id: ConversationId,
    pub content: String,
}

/// Command to execute a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteToolCommand {
    pub conversation_id: ConversationId,
    pub tool_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Command to create a digital twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDigitalTwinCommand {
    pub name: String,
    pub description: Option<String>,
    pub twin_type: String,
    pub initial_properties: HashMap<String, serde_json::Value>,
    pub tags: Vec<String>,
}

/// Command to update digital twin properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTwinPropertiesCommand {
    pub twin_id: TwinId,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Command to delete a digital twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTwinCommand {
    pub twin_id: TwinId,
}

/// Command to sync a digital twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDigitalTwinCommand {
    pub twin_id: TwinId,
    pub force_sync: bool,
}

/// Command to run a simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSimulationCommand {
    pub twin_id: TwinId,
    pub simulation_type: String,
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<SimulationScenarioCommand>,
    pub variables: HashMap<String, f64>,
}

/// Simulation scenario command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioCommand {
    pub name: String,
    pub conditions: HashMap<String, f64>,
    pub start_hour: u32,
    pub end_hour: u32,
}

/// Command to register a new tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterToolCommand {
    pub name: String,
    pub description: String,
    pub category: String,
    pub required_parameters: Vec<String>,
    pub parameter_types: HashMap<String, String>,
    pub config: serde_json::Value,
}

/// Command to update tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolCommand {
    pub tool_id: ToolId,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

/// Command to delete a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolCommand {
    pub tool_id: ToolId,
}

/// Command to add sensor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSensorDataCommand {
    pub twin_id: TwinId,
    pub sensor_name: String,
    pub sensor_type: String,
    pub unit: String,
    pub value: f64,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Command to batch add sensor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddSensorDataCommand {
    pub twin_id: TwinId,
    pub readings: Vec<SensorReadingCommand>,
}

/// Individual sensor reading in batch command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReadingCommand {
    pub sensor_name: String,
    pub sensor_type: String,
    pub unit: String,
    pub value: f64,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Command to clear conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearConversationCommand {
    pub conversation_id: ConversationId,
}

/// Command to clone an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneAgentCommand {
    pub source_agent_id: AgentId,
    pub new_name: String,
}

/// Command to clone a digital twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneTwinCommand {
    pub source_twin_id: TwinId,
    pub new_name: String,
}

/// Command to batch run simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRunSimulationsCommand {
    pub twin_ids: Vec<TwinId>,
    pub simulation_type: String,
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<SimulationScenarioCommand>,
    pub variables: HashMap<String, f64>,
}

/// Command to schedule a simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSimulationCommand {
    pub twin_id: TwinId,
    pub simulation_type: String,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<SimulationScenarioCommand>,
    pub variables: HashMap<String, f64>,
}

/// Command validation trait
pub trait CommandValidator {
    /// Validate the command
    fn validate(&self) -> Result<(), Vec<String>>;
}

// Example implementation for CreateAgentCommand
impl CommandValidator for CreateAgentCommand {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push("Agent name cannot be empty".to_string());
        }

        if self.instructions.trim().is_empty() {
            errors.push("Agent instructions cannot be empty".to_string());
        }

        if self.model_config.provider.trim().is_empty() {
            errors.push("Model provider cannot be empty".to_string());
        }

        if self.model_config.model.trim().is_empty() {
            errors.push("Model name cannot be empty".to_string());
        }

        if let Some(temp) = self.model_config.temperature {
            if !(0.0..=2.0).contains(&temp) {
                errors.push("Temperature must be between 0.0 and 2.0".to_string());
            }
        }

        if let Some(tokens) = self.model_config.max_tokens {
            if tokens == 0 {
                errors.push("Max tokens must be greater than 0".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Builder pattern for complex commands
pub struct SimulationCommandBuilder {
    twin_id: Option<TwinId>,
    simulation_type: Option<String>,
    duration_hours: u32,
    time_step_minutes: u32,
    scenarios: Vec<SimulationScenarioCommand>,
    variables: HashMap<String, f64>,
}

impl SimulationCommandBuilder {
    pub fn new() -> Self {
        Self {
            twin_id: None,
            simulation_type: None,
            duration_hours: 24,
            time_step_minutes: 60,
            scenarios: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn twin_id(mut self, id: TwinId) -> Self {
        self.twin_id = Some(id);
        self
    }

    pub fn simulation_type(mut self, sim_type: String) -> Self {
        self.simulation_type = Some(sim_type);
        self
    }

    pub fn duration_hours(mut self, hours: u32) -> Self {
        self.duration_hours = hours;
        self
    }

    pub fn time_step_minutes(mut self, minutes: u32) -> Self {
        self.time_step_minutes = minutes;
        self
    }

    pub fn add_scenario(mut self, scenario: SimulationScenarioCommand) -> Self {
        self.scenarios.push(scenario);
        self
    }

    pub fn add_variable(mut self, name: String, value: f64) -> Self {
        self.variables.insert(name, value);
        self
    }

    pub fn build(self) -> Result<RunSimulationCommand, String> {
        let twin_id = self.twin_id.ok_or("Twin ID is required")?;
        let simulation_type = self.simulation_type.ok_or("Simulation type is required")?;

        Ok(RunSimulationCommand {
            twin_id,
            simulation_type,
            duration_hours: self.duration_hours,
            time_step_minutes: self.time_step_minutes,
            scenarios: self.scenarios,
            variables: self.variables,
        })
    }
}

impl Default for SimulationCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}