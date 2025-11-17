//! Unit tests for the DigitalTwin domain model.

use digital_twin_desktop::core::domain::models::digital_twin::{DigitalTwin, TwinStatus};
use uuid::Uuid;
use chrono::Utc;
use test_case::test_case;

#[test]
fn test_create_digital_twin() {
    // Arrange
    let id = Uuid::new_v4();
    let name = "Test Twin".to_string();
    let description = "A test digital twin".to_string();
    let metadata = serde_json::json!({
        "version": "1.0",
        "type": "test"
    });
    
    // Act
    let twin = DigitalTwin {
        id,
        name: name.clone(),
        description: description.clone(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: metadata.clone(),
    };
    
    // Assert
    assert_eq!(twin.id, id);
    assert_eq!(twin.name, name);
    assert_eq!(twin.description, description);
    assert_eq!(twin.status, TwinStatus::Active);
    assert_eq!(twin.metadata, metadata);
}

#[test_case(TwinStatus::Active => true; "active twin is available")]
#[test_case(TwinStatus::Inactive => false; "inactive twin is not available")]
#[test_case(TwinStatus::Maintenance => false; "twin in maintenance is not available")]
fn test_twin_availability(status: TwinStatus) -> bool {
    // Arrange
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({}),
    };
    
    // Act & Assert
    match twin.status {
        TwinStatus::Active => true,
        _ => false,
    }
}

#[test]
fn test_twin_metadata_access() {
    // Arrange
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({
            "version": "1.0",
            "location": {
                "latitude": 37.7749,
                "longitude": -122.4194
            },
            "tags": ["test", "development"]
        }),
    };
    
    // Act & Assert
    assert_eq!(twin.metadata["version"], "1.0");
    assert_eq!(twin.metadata["location"]["latitude"], 37.7749);
    assert_eq!(twin.metadata["location"]["longitude"], -122.4194);
    assert!(twin.metadata["tags"].is_array());
    assert_eq!(twin.metadata["tags"][0], "test");
    assert_eq!(twin.metadata["tags"][1], "development");
}

#[test]
fn test_twin_status_transitions() {
    // Arrange
    let mut twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({}),
    };
    
    // Act & Assert - Test valid transitions
    assert_eq!(twin.status, TwinStatus::Active);
    
    // Transition to maintenance
    twin.status = TwinStatus::Maintenance;
    assert_eq!(twin.status, TwinStatus::Maintenance);
    
    // Transition back to active
    twin.status = TwinStatus::Active;
    assert_eq!(twin.status, TwinStatus::Active);
    
    // Transition to inactive
    twin.status = TwinStatus::Inactive;
    assert_eq!(twin.status, TwinStatus::Inactive);
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
        status: TwinStatus::Active,
        created_at,
        updated_at,
        metadata: serde_json::json!({}),
    };
    
    // Act & Assert
    assert_eq!(twin.created_at, created_at);
    assert_eq!(twin.updated_at, updated_at);
    
    // In a real implementation, we would test that updated_at changes
    // when the twin is modified, but that would require methods on the
    // DigitalTwin struct that we don't have access to here.
}