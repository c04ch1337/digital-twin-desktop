//! Integration tests for the TwinRepository.

use digital_twin_desktop::core::domain::models::digital_twin::{DigitalTwin, TwinStatus};
use digital_twin_desktop::core::domain::traits::repository::Repository;
use digital_twin_desktop::infrastructure::db::repositories::twin_repository::TwinRepository;
use digital_twin_desktop::infrastructure::db::sqlite::SqliteDatabase;

use crate::common;
use crate::fixtures;

use rusqlite::Connection;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use anyhow::Result;

// Helper function to create a test database connection
fn create_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    
    // Apply schema migrations
    conn.execute_batch(include_str!("../../../src/infrastructure/db/migrations/20251117000000_initial_schema.sql"))
        .expect("Failed to apply schema migrations");
        
    conn
}

// Helper function to create a test repository
async fn create_test_repository() -> Result<(TwinRepository, Connection)> {
    let conn = create_test_db();
    let db = SqliteDatabase::new_with_connection(conn.clone())?;
    let repo = TwinRepository::new(Arc::new(db));
    Ok((repo, conn))
}

#[tokio::test]
async fn test_save_and_find_by_id() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({
            "version": "1.0",
            "type": "test"
        }),
    };
    
    // Act - Save the twin
    let saved_twin = repo.save(twin.clone()).await?;
    
    // Assert - Verify the saved twin
    assert_eq!(saved_twin.id, twin.id);
    assert_eq!(saved_twin.name, twin.name);
    assert_eq!(saved_twin.description, twin.description);
    assert_eq!(saved_twin.status, twin.status);
    
    // Act - Find the twin by ID
    let found_twin = repo.find_by_id(twin.id).await?;
    
    // Assert - Verify the found twin
    assert!(found_twin.is_some());
    let found_twin = found_twin.unwrap();
    assert_eq!(found_twin.id, twin.id);
    assert_eq!(found_twin.name, twin.name);
    assert_eq!(found_twin.description, twin.description);
    assert_eq!(found_twin.status, twin.status);
    assert_eq!(found_twin.metadata, twin.metadata);
    
    Ok(())
}

#[tokio::test]
async fn test_find_by_id_not_found() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    let non_existent_id = Uuid::new_v4();
    
    // Act
    let result = repo.find_by_id(non_existent_id).await?;
    
    // Assert
    assert!(result.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_find_all() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    
    // Create multiple twins
    let twin1 = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin 1".to_string(),
        description: "A test digital twin 1".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({"index": 1}),
    };
    
    let twin2 = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin 2".to_string(),
        description: "A test digital twin 2".to_string(),
        status: TwinStatus::Maintenance,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({"index": 2}),
    };
    
    let twin3 = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin 3".to_string(),
        description: "A test digital twin 3".to_string(),
        status: TwinStatus::Inactive,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({"index": 3}),
    };
    
    // Save the twins
    repo.save(twin1.clone()).await?;
    repo.save(twin2.clone()).await?;
    repo.save(twin3.clone()).await?;
    
    // Act
    let all_twins = repo.find_all().await?;
    
    // Assert
    assert_eq!(all_twins.len(), 3);
    
    // Verify all twins are returned
    let has_twin1 = all_twins.iter().any(|t| t.id == twin1.id);
    let has_twin2 = all_twins.iter().any(|t| t.id == twin2.id);
    let has_twin3 = all_twins.iter().any(|t| t.id == twin3.id);
    
    assert!(has_twin1);
    assert!(has_twin2);
    assert!(has_twin3);
    
    Ok(())
}

#[tokio::test]
async fn test_update_twin() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({"version": "1.0"}),
    };
    
    // Save the initial twin
    repo.save(twin.clone()).await?;
    
    // Create an updated version
    let mut updated_twin = twin.clone();
    updated_twin.name = "Updated Twin".to_string();
    updated_twin.description = "An updated test digital twin".to_string();
    updated_twin.status = TwinStatus::Maintenance;
    updated_twin.metadata = serde_json::json!({"version": "1.1"});
    
    // Act - Update the twin
    let result = repo.save(updated_twin.clone()).await?;
    
    // Assert - Verify the update was successful
    assert_eq!(result.id, twin.id);
    assert_eq!(result.name, "Updated Twin");
    assert_eq!(result.description, "An updated test digital twin");
    assert_eq!(result.status, TwinStatus::Maintenance);
    assert_eq!(result.metadata, serde_json::json!({"version": "1.1"}));
    
    // Verify the twin was updated in the database
    let found_twin = repo.find_by_id(twin.id).await?.unwrap();
    assert_eq!(found_twin.name, "Updated Twin");
    assert_eq!(found_twin.description, "An updated test digital twin");
    assert_eq!(found_twin.status, TwinStatus::Maintenance);
    assert_eq!(found_twin.metadata, serde_json::json!({"version": "1.1"}));
    
    Ok(())
}

#[tokio::test]
async fn test_delete_twin() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    
    let twin = DigitalTwin {
        id: Uuid::new_v4(),
        name: "Test Twin".to_string(),
        description: "A test digital twin".to_string(),
        status: TwinStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: serde_json::json!({}),
    };
    
    // Save the twin
    repo.save(twin.clone()).await?;
    
    // Verify the twin exists
    let found = repo.find_by_id(twin.id).await?;
    assert!(found.is_some());
    
    // Act - Delete the twin
    let result = repo.delete(twin.id).await?;
    
    // Assert - Verify the deletion was successful
    assert!(result);
    
    // Verify the twin no longer exists
    let found_after_delete = repo.find_by_id(twin.id).await?;
    assert!(found_after_delete.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_delete_nonexistent_twin() -> Result<()> {
    // Arrange
    common::setup();
    let (repo, _) = create_test_repository().await?;
    let non_existent_id = Uuid::new_v4();
    
    // Act
    let result = repo.delete(non_existent_id).await?;
    
    // Assert - Should return false since no twin was deleted
    assert!(!result);
    
    Ok(())
}