//! Mock implementation of the TwinRepository.

use digital_twin_desktop::core::domain::models::digital_twin::DigitalTwin;
use digital_twin_desktop::core::domain::traits::repository::Repository;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;

/// Mock implementation of the TwinRepository for testing.
pub struct MockTwinRepository {
    twins: Arc<Mutex<HashMap<Uuid, DigitalTwin>>>,
}

impl MockTwinRepository {
    /// Create a new mock twin repository.
    pub fn new() -> Self {
        Self {
            twins: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Create a new mock twin repository with initial data.
    pub fn with_twins(twins: Vec<DigitalTwin>) -> Self {
        let mut map = HashMap::new();
        for twin in twins {
            map.insert(twin.id, twin);
        }
        
        Self {
            twins: Arc::new(Mutex::new(map)),
        }
    }
    
    /// Get all twins in the repository.
    pub fn get_all_twins(&self) -> Vec<DigitalTwin> {
        let twins = self.twins.lock().unwrap();
        twins.values().cloned().collect()
    }
}

#[async_trait]
impl Repository<DigitalTwin> for MockTwinRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DigitalTwin>> {
        let twins = self.twins.lock().unwrap();
        Ok(twins.get(&id).cloned())
    }
    
    async fn find_all(&self) -> Result<Vec<DigitalTwin>> {
        let twins = self.twins.lock().unwrap();
        Ok(twins.values().cloned().collect())
    }
    
    async fn save(&self, entity: DigitalTwin) -> Result<DigitalTwin> {
        let mut twins = self.twins.lock().unwrap();
        let id = entity.id;
        twins.insert(id, entity.clone());
        Ok(entity)
    }
    
    async fn delete(&self, id: Uuid) -> Result<bool> {
        let mut twins = self.twins.lock().unwrap();
        Ok(twins.remove(&id).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use digital_twin_desktop::core::domain::models::digital_twin::TwinStatus;
    use chrono::Utc;
    
    #[tokio::test]
    async fn test_mock_twin_repository() {
        // Create a mock repository
        let repo = MockTwinRepository::new();
        
        // Create a test twin
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
        let saved_twin = repo.save(twin.clone()).await.unwrap();
        assert_eq!(saved_twin.id, twin.id);
        
        // Find the twin by ID
        let found_twin = repo.find_by_id(twin.id).await.unwrap();
        assert!(found_twin.is_some());
        assert_eq!(found_twin.unwrap().id, twin.id);
        
        // Find all twins
        let all_twins = repo.find_all().await.unwrap();
        assert_eq!(all_twins.len(), 1);
        assert_eq!(all_twins[0].id, twin.id);
        
        // Delete the twin
        let deleted = repo.delete(twin.id).await.unwrap();
        assert!(deleted);
        
        // Verify the twin is gone
        let not_found = repo.find_by_id(twin.id).await.unwrap();
        assert!(not_found.is_none());
    }
}