//! Unit tests for the TwinService.

use digital_twin_desktop::core::application::services::twin_service::TwinService;
use digital_twin_desktop::core::domain::models::digital_twin::{DigitalTwin, TwinStatus};
use digital_twin_desktop::core::domain::traits::repository::Repository;
use digital_twin_desktop::core::application::dtos::TwinDto;
use mockall::predicate::*;
use mockall::mock;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use anyhow::Result;
use rstest::*;

// Create a mock repository for DigitalTwin
mock! {
    TwinRepository {}
    
    #[async_trait::async_trait]
    impl Repository<DigitalTwin> for TwinRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<DigitalTwin>>;
        async fn find_all(&self) -> Result<Vec<DigitalTwin>>;
        async fn save(&self, entity: DigitalTwin) -> Result<DigitalTwin>;
        async fn delete(&self, id: Uuid) -> Result<bool>;
    }
}

#[fixture]
fn test_twin() -> DigitalTwin {
    DigitalTwin {
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
    }
}

#[rstest]
#[tokio::test]
async fn test_get_twin_by_id_success(test_twin: DigitalTwin) {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twin_id = test_twin.id;
    
    mock_repo
        .expect_find_by_id()
        .with(eq(twin_id))
        .times(1)
        .returning(move |_| Ok(Some(test_twin.clone())));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.get_twin_by_id(twin_id).await;
    
    // Assert
    assert!(result.is_ok());
    let twin_opt = result.unwrap();
    assert!(twin_opt.is_some());
    let twin = twin_opt.unwrap();
    assert_eq!(twin.id, twin_id);
    assert_eq!(twin.name, "Test Twin");
}

#[tokio::test]
async fn test_get_twin_by_id_not_found() {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twin_id = Uuid::new_v4();
    
    mock_repo
        .expect_find_by_id()
        .with(eq(twin_id))
        .times(1)
        .returning(|_| Ok(None));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.get_twin_by_id(twin_id).await;
    
    // Assert
    assert!(result.is_ok());
    let twin_opt = result.unwrap();
    assert!(twin_opt.is_none());
}

#[rstest]
#[tokio::test]
async fn test_create_twin(test_twin: DigitalTwin) {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twin_dto = TwinDto {
        id: None,
        name: test_twin.name.clone(),
        description: test_twin.description.clone(),
        status: "active".to_string(),
        metadata: test_twin.metadata.clone(),
    };
    
    mock_repo
        .expect_save()
        .withf(|twin: &DigitalTwin| {
            twin.name == twin_dto.name && 
            twin.description == twin_dto.description &&
            twin.status == TwinStatus::Active
        })
        .times(1)
        .returning(|twin| Ok(twin));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.create_twin(twin_dto).await;
    
    // Assert
    assert!(result.is_ok());
    let created_twin = result.unwrap();
    assert_eq!(created_twin.name, test_twin.name);
    assert_eq!(created_twin.description, test_twin.description);
    assert_eq!(created_twin.status, TwinStatus::Active);
}

#[rstest]
#[tokio::test]
async fn test_update_twin_status(test_twin: DigitalTwin) {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twin_id = test_twin.id;
    let mut updated_twin = test_twin.clone();
    updated_twin.status = TwinStatus::Maintenance;
    
    mock_repo
        .expect_find_by_id()
        .with(eq(twin_id))
        .times(1)
        .returning(move |_| Ok(Some(test_twin.clone())));
    
    mock_repo
        .expect_save()
        .withf(move |twin: &DigitalTwin| {
            twin.id == twin_id && twin.status == TwinStatus::Maintenance
        })
        .times(1)
        .returning(|twin| Ok(twin));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.update_twin_status(twin_id, "maintenance").await;
    
    // Assert
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert!(updated.is_some());
    let twin = updated.unwrap();
    assert_eq!(twin.id, twin_id);
    assert_eq!(twin.status, TwinStatus::Maintenance);
}

#[tokio::test]
async fn test_delete_twin() {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twin_id = Uuid::new_v4();
    
    mock_repo
        .expect_delete()
        .with(eq(twin_id))
        .times(1)
        .returning(|_| Ok(true));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.delete_twin(twin_id).await;
    
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[rstest]
#[tokio::test]
async fn test_get_all_twins(test_twin: DigitalTwin) {
    // Arrange
    let mut mock_repo = MockTwinRepository::new();
    let twins = vec![test_twin.clone(), {
        let mut twin2 = test_twin.clone();
        twin2.id = Uuid::new_v4();
        twin2.name = "Test Twin 2".to_string();
        twin2
    }];
    
    mock_repo
        .expect_find_all()
        .times(1)
        .returning(move || Ok(twins.clone()));
    
    let service = TwinService::new(Arc::new(mock_repo));
    
    // Act
    let result = service.get_all_twins().await;
    
    // Assert
    assert!(result.is_ok());
    let all_twins = result.unwrap();
    assert_eq!(all_twins.len(), 2);
    assert_eq!(all_twins[0].name, "Test Twin");
    assert_eq!(all_twins[1].name, "Test Twin 2");
}