//! Unit tests for the DigitalTwin domain model.

use digital_twin_desktop::core::domain::models::digital_twin::{
    DigitalTwin, TwinState, TwinType, TwinProperties, SyncConfiguration,
    VisualizationConfig, TwinMetadata,
};
use uuid::Uuid;
use chrono::Utc;
use test_case::test_case;

#[test]
fn test_create_digital_twin() {
    // Arrange
    let id = Uuid::new_v4();
    let name = "Test Twin".to_string();
    let description = "A test digital twin".to_string();
    let twin_type = TwinType::Device {
        device_type: "sensor".to_string(),
        manufacturer: Some("TestCorp".to_string()),
        model: Some("T1000".to_string()),
    };
    
    // Act
    let twin = DigitalTwin::new(name.clone(), description.clone(), twin_type.clone());
    
    // Assert
    assert_eq!(twin.name, name);
    assert_eq!(twin.description, description);
    assert_eq!(twin.twin_type, twin_type);
    assert_eq!(twin.state, TwinState::Idle); // Default state is Idle
    assert!(twin.agent_ids.is_empty());
    assert!(twin.data_sources.is_empty());
}

#[test_case(TwinState::Active => true; "active twin is available")]
#[test_case(TwinState::Idle => false; "idle twin is not available")]
#[test_case(TwinState::Paused => false; "paused twin is not available")]
#[test_case(TwinState::Disconnected => false; "disconnected twin is not available")]
#[test_case(TwinState::Error => false; "error twin is not available")]
#[test_case(TwinState::Archived => false; "archived twin is not available")]
fn test_twin_availability(state: TwinState) -> bool {
    // Arrange
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        twin_type: TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: None,
            model: None,
        },
        state,
        agent_ids: Vec::new(),
        data_sources: Vec::new(),
        properties: TwinProperties::default(),
        sync_config: SyncConfiguration::default(),
        visualization_config: VisualizationConfig::default(),
        metadata: TwinMetadata::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_sync_at: None,
    };
    
    // Act & Assert
    match twin.state {
        TwinState::Active => true,
        _ => false,
    }
}

#[test]
fn test_twin_metadata_access() {
    // Arrange
    let mut metadata = TwinMetadata::default();
    metadata.version = "1.0".to_string();
    metadata.owner = "test-user".to_string();
    metadata.tags = vec!["test".to_string(), "development".to_string()];
    metadata.custom_fields.insert(
        "location".to_string(),
        serde_json::json!({
            "latitude": 37.7749,
            "longitude": -122.4194
        }),
    );
    
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        twin_type: TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: None,
            model: None,
        },
        state: TwinState::Active,
        agent_ids: Vec::new(),
        data_sources: Vec::new(),
        properties: TwinProperties::default(),
        sync_config: SyncConfiguration::default(),
        visualization_config: VisualizationConfig::default(),
        metadata: metadata.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_sync_at: None,
    };
    
    // Act & Assert
    assert_eq!(twin.metadata.version, "1.0");
    assert_eq!(twin.metadata.owner, "test-user");
    assert_eq!(twin.metadata.tags.len(), 2);
    assert_eq!(twin.metadata.tags[0], "test");
    assert_eq!(twin.metadata.tags[1], "development");
    assert!(twin.metadata.custom_fields.contains_key("location"));
    let location = twin.metadata.custom_fields.get("location").unwrap();
    assert_eq!(location["latitude"], 37.7749);
    assert_eq!(location["longitude"], -122.4194);
}

#[test]
fn test_twin_state_transitions() {
    // Arrange
    let mut twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        twin_type: TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: None,
            model: None,
        },
        state: TwinState::Active,
        agent_ids: Vec::new(),
        data_sources: Vec::new(),
        properties: TwinProperties::default(),
        sync_config: SyncConfiguration::default(),
        visualization_config: VisualizationConfig::default(),
        metadata: TwinMetadata::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_sync_at: None,
    };
    
    // Act & Assert - Test valid transitions
    assert_eq!(twin.state, TwinState::Active);
    
    // Transition to syncing
    twin.state = TwinState::Syncing;
    assert_eq!(twin.state, TwinState::Syncing);
    
    // Transition back to active
    twin.state = TwinState::Active;
    assert_eq!(twin.state, TwinState::Active);
    
    // Transition to paused
    twin.state = TwinState::Paused;
    assert_eq!(twin.state, TwinState::Paused);
    
    // Transition to disconnected
    twin.state = TwinState::Disconnected;
    assert_eq!(twin.state, TwinState::Disconnected);
}

#[test]
fn test_twin_timestamps() {
    // Arrange
    let created_at = Utc::now();
    let updated_at = created_at;
    
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        twin_type: TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: None,
            model: None,
        },
        state: TwinState::Active,
        agent_ids: Vec::new(),
        data_sources: Vec::new(),
        properties: TwinProperties::default(),
        sync_config: SyncConfiguration::default(),
        visualization_config: VisualizationConfig::default(),
        metadata: TwinMetadata::default(),
        created_at,
        updated_at,
        last_sync_at: None,
    };
    
    // Act & Assert
    assert_eq!(twin.created_at, created_at);
    assert_eq!(twin.updated_at, updated_at);
    assert!(twin.last_sync_at.is_none());
    
    // Test that add_agent updates the updated_at timestamp
    let mut twin2 = DigitalTwin::new(
        "Test Twin 2".to_string(),
        "Another test twin".to_string(),
        TwinType::Device {
            device_type: "sensor".to_string(),
            manufacturer: None,
            model: None,
        },
    );
    let initial_updated = twin2.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    twin2.add_agent(Uuid::new_v4());
    assert!(twin2.updated_at > initial_updated);
}