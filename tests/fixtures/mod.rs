//! Test fixtures for the Digital Twin Desktop application.
//!
//! This module provides reusable test data and fixtures for various test scenarios.

use digital_twin_desktop::core::domain::models::{
    agent::Agent,
    conversation::{Conversation, Message, MessageRole},
    digital_twin::{
        DigitalTwin, TwinState, TwinType, TwinProperties, SyncConfiguration,
        VisualizationConfig, TwinMetadata,
    },
    sensor_data::{SensorData, SensorType},
    tool::{Tool, ToolExecution, ToolExecutionStatus},
};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Fixture for creating a test digital twin
pub fn create_test_twin() -> DigitalTwin {
    DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        twin_type: TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: Some("TestCorp".to_string()),
            model: Some("T1000".to_string()),
        },
        state: TwinState::Active,
        agent_ids: Vec::new(),
        data_sources: Vec::new(),
        properties: TwinProperties::default(),
        sync_config: SyncConfiguration::default(),
        visualization_config: VisualizationConfig::default(),
        metadata: TwinMetadata {
            version: "1.0.0".to_string(),
            owner: "test-user".to_string(),
            tags: vec!["test".to_string()],
            documentation: Vec::new(),
            custom_fields: std::collections::HashMap::new(),
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_sync_at: None,
    }
}

/// Fixture for creating a test agent
pub fn create_test_agent() -> Agent {
    Agent {
        id: Uuid::new_v4(),
        name: "Test Agent".to_string(),
        description: "A test agent".to_string(),
        twin_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        configuration: serde_json::json!({
            "model": "test-model",
            "temperature": 0.7,
            "max_tokens": 1000
        }),
    }
}

/// Fixture for creating a test conversation
pub fn create_test_conversation() -> Conversation {
    Conversation {
        id: Uuid::new_v4(),
        title: "Test Conversation".to_string(),
        twin_id: Uuid::new_v4(),
        agent_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        messages: vec![
            Message {
                id: Uuid::new_v4(),
                role: MessageRole::User,
                content: "Hello, test agent".to_string(),
                timestamp: Utc::now(),
            },
            Message {
                id: Uuid::new_v4(),
                role: MessageRole::Assistant,
                content: "Hello, I am a test agent".to_string(),
                timestamp: Utc::now(),
            },
        ],
    }
}

/// Fixture for creating test sensor data
pub fn create_test_sensor_data() -> SensorData {
    SensorData {
        id: Uuid::new_v4(),
        twin_id: Uuid::new_v4(),
        sensor_id: "sensor-001".to_string(),
        sensor_type: SensorType::Temperature,
        value: 25.5,
        unit: "celsius".to_string(),
        timestamp: Utc::now(),
        metadata: serde_json::json!({
            "location": "test-location",
            "accuracy": 0.1
        }),
    }
}

/// Fixture for creating a test tool
pub fn create_test_tool() -> Tool {
    Tool {
        id: Uuid::new_v4(),
        name: "Test Tool".to_string(),
        description: "A test tool".to_string(),
        tool_type: "test".to_string(),
        configuration: serde_json::json!({
            "param1": "value1",
            "param2": "value2"
        }),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Fixture for creating a test tool execution
pub fn create_test_tool_execution() -> ToolExecution {
    ToolExecution {
        id: Uuid::new_v4(),
        tool_id: Uuid::new_v4(),
        agent_id: Uuid::new_v4(),
        conversation_id: Uuid::new_v4(),
        status: ToolExecutionStatus::Completed,
        input: serde_json::json!({
            "command": "test-command",
            "args": ["arg1", "arg2"]
        }),
        output: serde_json::json!({
            "result": "test-result",
            "status": "success"
        }),
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
        error: None,
    }
}

/// Database fixtures for integration tests
pub mod db {
    use super::*;
    use rusqlite::Connection;
    
    /// Create a test database with schema
    pub fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
        
        // Apply schema migrations
        conn.execute_batch(include_str!("../../src/infrastructure/db/migrations/20251117000000_initial_schema.sql"))
            .expect("Failed to apply schema migrations");
            
        conn
    }
    
    /// Populate test database with sample data
    pub fn populate_test_db(conn: &Connection) {
        // Insert test twin
        let twin = create_test_twin();
        conn.execute(
            "INSERT INTO digital_twins (id, name, description, status, created_at, updated_at, metadata) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                twin.id.to_string(),
                &twin.name,
                &twin.description,
                "active",
                twin.created_at.to_rfc3339(),
                twin.updated_at.to_rfc3339(),
                twin.metadata.to_string(),
            ),
        ).expect("Failed to insert test twin");
        
        // Insert more test data as needed
    }
}