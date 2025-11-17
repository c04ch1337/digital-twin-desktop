//! Domain events for asynchronous communication
//!
//! Events represent things that have happened in the system
//! and can be used for event-driven architectures.

use crate::core::domain::{
    models::{
        agent::AgentId,
        conversation::ConversationId,
        digital_twin::{TwinId, TwinStatus},
        tool::ToolId,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base trait for all domain events
pub trait DomainEvent: Send + Sync {
    /// Get the event ID
    fn event_id(&self) -> &str;
    
    /// Get the event type
    fn event_type(&self) -> &str;
    
    /// Get the timestamp when the event occurred
    fn occurred_at(&self) -> &DateTime<Utc>;
    
    /// Get the aggregate ID this event relates to
    fn aggregate_id(&self) -> &str;
}

/// Agent created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCreated {
    pub event_id: String,
    pub agent_id: AgentId,
    pub name: String,
    pub model_provider: String,
    pub model_name: String,
    pub occurred_at: DateTime<Utc>,
}

/// Agent updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUpdated {
    pub event_id: String,
    pub agent_id: AgentId,
    pub changes: AgentChanges,
    pub occurred_at: DateTime<Utc>,
}

/// Agent changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentChanges {
    pub name: Option<String>,
    pub instructions: Option<String>,
    pub model_config: Option<serde_json::Value>,
    pub tools_added: Vec<ToolId>,
    pub tools_removed: Vec<ToolId>,
}

/// Agent deleted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDeleted {
    pub event_id: String,
    pub agent_id: AgentId,
    pub occurred_at: DateTime<Utc>,
}

/// Conversation started event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStarted {
    pub event_id: String,
    pub conversation_id: ConversationId,
    pub agent_id: AgentId,
    pub initial_context: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Message sent event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSent {
    pub event_id: String,
    pub conversation_id: ConversationId,
    pub message_role: String,
    pub message_content: String,
    pub occurred_at: DateTime<Utc>,
}

/// Tool executed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecuted {
    pub event_id: String,
    pub conversation_id: ConversationId,
    pub tool_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub success: bool,
    pub execution_time_ms: u64,
    pub error_message: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Digital twin created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinCreated {
    pub event_id: String,
    pub twin_id: TwinId,
    pub name: String,
    pub twin_type: String,
    pub tags: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Digital twin updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinUpdated {
    pub event_id: String,
    pub twin_id: TwinId,
    pub changes: TwinChanges,
    pub occurred_at: DateTime<Utc>,
}

/// Twin changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinChanges {
    pub properties_updated: HashMap<String, serde_json::Value>,
    pub properties_removed: Vec<String>,
    pub status_changed: Option<TwinStatus>,
    pub tags_added: Vec<String>,
    pub tags_removed: Vec<String>,
}

/// Digital twin synchronized event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinSynchronized {
    pub event_id: String,
    pub twin_id: TwinId,
    pub new_readings_count: usize,
    pub properties_updated: HashMap<String, serde_json::Value>,
    pub sync_duration_ms: u64,
    pub occurred_at: DateTime<Utc>,
}

/// Digital twin deleted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinDeleted {
    pub event_id: String,
    pub twin_id: TwinId,
    pub occurred_at: DateTime<Utc>,
}

/// Simulation started event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStarted {
    pub event_id: String,
    pub twin_id: TwinId,
    pub simulation_id: String,
    pub simulation_type: String,
    pub duration_hours: u32,
    pub occurred_at: DateTime<Utc>,
}

/// Simulation completed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationCompleted {
    pub event_id: String,
    pub twin_id: TwinId,
    pub simulation_id: String,
    pub simulation_type: String,
    pub success: bool,
    pub key_metrics: HashMap<String, f64>,
    pub recommendations_count: usize,
    pub execution_time_ms: u64,
    pub occurred_at: DateTime<Utc>,
}

/// Simulation failed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFailed {
    pub event_id: String,
    pub twin_id: TwinId,
    pub simulation_id: String,
    pub simulation_type: String,
    pub error_message: String,
    pub occurred_at: DateTime<Utc>,
}

/// Sensor data received event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDataReceived {
    pub event_id: String,
    pub twin_id: TwinId,
    pub sensor_name: String,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub occurred_at: DateTime<Utc>,
}

/// Anomaly detected event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetected {
    pub event_id: String,
    pub twin_id: TwinId,
    pub anomaly_type: String,
    pub severity: AnomalySeverity,
    pub description: String,
    pub affected_sensors: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Anomaly severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Tool registered event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegistered {
    pub event_id: String,
    pub tool_id: ToolId,
    pub name: String,
    pub category: String,
    pub occurred_at: DateTime<Utc>,
}

/// Tool configuration updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfigurationUpdated {
    pub event_id: String,
    pub tool_id: ToolId,
    pub config_changes: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

/// Tool disabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDisabled {
    pub event_id: String,
    pub tool_id: ToolId,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Tool enabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEnabled {
    pub event_id: String,
    pub tool_id: ToolId,
    pub occurred_at: DateTime<Utc>,
}

/// System event for general notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_id: String,
    pub event_type: SystemEventType,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub occurred_at: DateTime<Utc>,
}

/// System event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    Startup,
    Shutdown,
    ConfigurationChanged,
    BackupCompleted,
    MaintenanceStarted,
    MaintenanceCompleted,
    ResourceWarning,
    Error,
}

// Event implementations

impl DomainEvent for AgentCreated {
    fn event_id(&self) -> &str {
        &self.event_id
    }
    
    fn event_type(&self) -> &str {
        "agent.created"
    }
    
    fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
    
    fn aggregate_id(&self) -> &str {
        &self.agent_id.value
    }
}

impl DomainEvent for ConversationStarted {
    fn event_id(&self) -> &str {
        &self.event_id
    }
    
    fn event_type(&self) -> &str {
        "conversation.started"
    }
    
    fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
    
    fn aggregate_id(&self) -> &str {
        &self.conversation_id.value
    }
}

impl DomainEvent for DigitalTwinCreated {
    fn event_id(&self) -> &str {
        &self.event_id
    }
    
    fn event_type(&self) -> &str {
        "digital_twin.created"
    }
    
    fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
    
    fn aggregate_id(&self) -> &str {
        &self.twin_id.value
    }
}

impl DomainEvent for SimulationCompleted {
    fn event_id(&self) -> &str {
        &self.event_id
    }
    
    fn event_type(&self) -> &str {
        "simulation.completed"
    }
    
    fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
    
    fn aggregate_id(&self) -> &str {
        &self.twin_id.value
    }
}

// Event builder for complex events

pub struct AnomalyDetectedBuilder {
    event_id: String,
    twin_id: Option<TwinId>,
    anomaly_type: Option<String>,
    severity: Option<AnomalySeverity>,
    description: Option<String>,
    affected_sensors: Vec<String>,
    recommended_actions: Vec<String>,
}

impl AnomalyDetectedBuilder {
    pub fn new() -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            twin_id: None,
            anomaly_type: None,
            severity: None,
            description: None,
            affected_sensors: Vec::new(),
            recommended_actions: Vec::new(),
        }
    }

    pub fn twin_id(mut self, id: TwinId) -> Self {
        self.twin_id = Some(id);
        self
    }

    pub fn anomaly_type(mut self, anomaly_type: String) -> Self {
        self.anomaly_type = Some(anomaly_type);
        self
    }

    pub fn severity(mut self, severity: AnomalySeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    pub fn description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn add_affected_sensor(mut self, sensor: String) -> Self {
        self.affected_sensors.push(sensor);
        self
    }

    pub fn add_recommended_action(mut self, action: String) -> Self {
        self.recommended_actions.push(action);
        self
    }

    pub fn build(self) -> Result<AnomalyDetected, String> {
        Ok(AnomalyDetected {
            event_id: self.event_id,
            twin_id: self.twin_id.ok_or("Twin ID is required")?,
            anomaly_type: self.anomaly_type.ok_or("Anomaly type is required")?,
            severity: self.severity.ok_or("Severity is required")?,
            description: self.description.ok_or("Description is required")?,
            affected_sensors: self.affected_sensors,
            recommended_actions: self.recommended_actions,
            occurred_at: Utc::now(),
        })
    }
}

impl Default for AnomalyDetectedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Event dispatcher trait
pub trait EventDispatcher: Send + Sync {
    /// Dispatch an event
    fn dispatch(&self, event: Box<dyn DomainEvent>);
}

/// Event handler trait
pub trait EventHandler<E>: Send + Sync
where
    E: DomainEvent,
{
    /// Handle an event
    fn handle(&self, event: &E);
}

/// Aggregate event stream
#[derive(Debug)]
pub struct EventStream {
    events: Vec<Box<dyn DomainEvent>>,
}

impl EventStream {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn append(&mut self, event: Box<dyn DomainEvent>) {
        self.events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.events)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}