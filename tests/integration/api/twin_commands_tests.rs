//! Integration tests for twin-related API commands.

use digital_twin_desktop::api::commands::twin_commands::{
    CreateTwinCommand, GetTwinCommand, UpdateTwinStatusCommand, DeleteTwinCommand, ListTwinsCommand
};
use digital_twin_desktop::core::application::services::twin_service::TwinService;
use digital_twin_desktop::core::domain::models::digital_twin::{DigitalTwin, TwinStatus};
use digital_twin_desktop::core::domain::traits::repository::Repository;
use digital_twin_desktop::infrastructure::db::repositories::twin_repository::TwinRepository;
use digital_twin_desktop::api::dto::TwinResponseDto;

use crate::common;
use crate::fixtures;
use crate::helpers::InMemoryRepository;

use std::sync::Arc;
use uuid::Uuid;
use serde_json::json;
use tauri::State;

// Helper function to create a test environment
async fn setup_test_env() -> (
    CreateTwinCommand,
    GetTwinCommand,
    UpdateTwinStatusCommand,
    DeleteTwinCommand,
    ListTwinsCommand,
    Arc<InMemoryRepository<DigitalTwin>>
) {
    // Create an in-memory repository
    let repo = Arc::new(InMemoryRepository::<DigitalTwin>::new());
    
    // Create the service with the repository
    let service = TwinService::new(repo.clone());
    
    // Create the command handlers
    let create_cmd = CreateTwinCommand::new(service.clone());
    let get_cmd = GetTwinCommand::new(service.clone());
    let update_status_cmd = UpdateTwinStatusCommand::new(service.clone());
    let delete_cmd = DeleteTwinCommand::new(service.clone());
    let list_cmd = ListTwinsCommand::new(service);
    
    (create_cmd, get_cmd, update_status_cmd, delete_cmd, list_cmd, repo)
}

#[tokio::test]
async fn test_create_twin_command() {
    // Arrange
    common::setup();
    let (create_cmd, _, _, _, _, repo) = setup_test_env().await;
    
    let payload = json!({
        "name": "Test Twin",
        "description": "A test digital twin",
        "status": "active",
        "metadata": {
            "version": "1.0",
            "type": "test"
        }
    });
    
    // Act
    let result = create_cmd.execute(payload).await;
    
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.name, "Test Twin");
    assert_eq!(response.description, "A test digital twin");
    assert_eq!(response.status, "active");
    
    // Verify the twin was saved in the repository
    let all_twins = repo.get_all();
    assert_eq!(all_twins.len(), 1);
    assert_eq!(all_twins[0].name, "Test Twin");
}

#[tokio::test]
async fn test_get_twin_command() {
    // Arrange
    common::setup();
    let (create_cmd, get_cmd, _, _, _, _) = setup_test_env().await;
    
    // First create a twin
    let create_payload = json!({
        "name": "Test Twin",
        "description": "A test digital twin",
        "status": "active",
        "metadata": {
            "version": "1.0",
            "type": "test"
        }
    });
    
    let create_result = create_cmd.execute(create_payload).await.unwrap();
    let twin_id = create_result.id;
    
    // Act - Get the twin
    let get_payload = json!({
        "id": twin_id
    });
    
    let result = get_cmd.execute(get_payload).await;
    
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, twin_id);
    assert_eq!(response.name, "Test Twin");
    assert_eq!(response.description, "A test digital twin");
    assert_eq!(response.status, "active");
}

#[tokio::test]
async fn test_update_twin_status_command() {
    // Arrange
    common::setup();
    let (create_cmd, _, update_status_cmd, _, _, _) = setup_test_env().await;
    
    // First create a twin
    let create_payload = json!({
        "name": "Test Twin",
        "description": "A test digital twin",
        "status": "active",
        "metadata": {
            "version": "1.0",
            "type": "test"
        }
    });
    
    let create_result = create_cmd.execute(create_payload).await.unwrap();
    let twin_id = create_result.id;
    
    // Act - Update the twin status
    let update_payload = json!({
        "id": twin_id,
        "status": "maintenance"
    });
    
    let result = update_status_cmd.execute(update_payload).await;
    
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, twin_id);
    assert_eq!(response.status, "maintenance");
}

#[tokio::test]
async fn test_delete_twin_command() {
    // Arrange
    common::setup();
    let (create_cmd, get_cmd, _, delete_cmd, _, _) = setup_test_env().await;
    
    // First create a twin
    let create_payload = json!({
        "name": "Test Twin",
        "description": "A test digital twin",
        "status": "active",
        "metadata": {
            "version": "1.0",
            "type": "test"
        }
    });
    
    let create_result = create_cmd.execute(create_payload).await.unwrap();
    let twin_id = create_result.id;
    
    // Act - Delete the twin
    let delete_payload = json!({
        "id": twin_id
    });
    
    let result = delete_cmd.execute(delete_payload).await;
    
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Verify the twin is no longer retrievable
    let get_payload = json!({
        "id": twin_id
    });
    
    let get_result = get_cmd.execute(get_payload).await;
    assert!(get_result.is_ok());
    assert!(get_result.unwrap().is_none());
}

#[tokio::test]
async fn test_list_twins_command() {
    // Arrange
    common::setup();
    let (create_cmd, _, _, _, list_cmd, _) = setup_test_env().await;
    
    // Create multiple twins
    for i in 1..=3 {
        let create_payload = json!({
            "name": format!("Test Twin {}", i),
            "description": format!("A test digital twin {}", i),
            "status": "active",
            "metadata": {
                "version": "1.0",
                "type": "test",
                "index": i
            }
        });
        
        create_cmd.execute(create_payload).await.unwrap();
    }
    
    // Act
    let result = list_cmd.execute(json!({})).await;
    
    // Assert
    assert!(result.is_ok());
    let twins = result.unwrap();
    assert_eq!(twins.len(), 3);
    
    // Verify the twins are returned in the expected order (by name)
    assert_eq!(twins[0].name, "Test Twin 1");
    assert_eq!(twins[1].name, "Test Twin 2");
    assert_eq!(twins[2].name, "Test Twin 3");
}