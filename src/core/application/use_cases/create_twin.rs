use crate::core::domain::{
    errors::DomainError,
    models::digital_twin::{DigitalTwin, TwinId, TwinStatus, TwinMetadata},
    traits::repository::DigitalTwinRepository,
};
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;

/// Command to create a new digital twin
#[derive(Debug, Clone)]
pub struct CreateTwinCommand {
    pub name: String,
    pub description: Option<String>,
    pub twin_type: String,
    pub initial_properties: HashMap<String, serde_json::Value>,
    pub tags: Vec<String>,
}

/// Use case for creating a new digital twin
pub struct CreateTwinUseCase {
    twin_repo: Arc<dyn DigitalTwinRepository>,
}

impl CreateTwinUseCase {
    pub fn new(twin_repo: Arc<dyn DigitalTwinRepository>) -> Self {
        Self { twin_repo }
    }

    pub async fn execute(
        &self,
        command: CreateTwinCommand,
    ) -> Result<DigitalTwin, DomainError> {
        // Validate command
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Twin name cannot be empty".to_string(),
            ));
        }

        if command.twin_type.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Twin type cannot be empty".to_string(),
            ));
        }

        // Create metadata
        let metadata = TwinMetadata {
            version: "1.0.0".to_string(),
            schema_version: "1.0".to_string(),
            tags: command.tags,
            custom_fields: HashMap::new(),
        };

        // Create new digital twin
        let twin = DigitalTwin::new(
            TwinId::new(),
            command.name,
            command.description,
            command.twin_type,
            command.initial_properties,
            HashMap::new(), // Empty sensor data initially
            TwinStatus::Idle,
            HashMap::new(), // Empty simulation results initially
            metadata,
            Utc::now(),
            Utc::now(),
        );

        // Save to repository
        self.twin_repo
            .save(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(twin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockDigitalTwinRepository {
        saved_twins: Arc<Mutex<Vec<DigitalTwin>>>,
    }

    #[async_trait]
    impl DigitalTwinRepository for MockDigitalTwinRepository {
        async fn save(&self, twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.saved_twins.lock().unwrap().push(twin.clone());
            Ok(())
        }

        async fn find_by_id(&self, _id: &TwinId) -> Result<Option<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_all(&self) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, _twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn delete(&self, _id: &TwinId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_type(&self, _twin_type: &str) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_create_twin_success() {
        let saved_twins = Arc::new(Mutex::new(Vec::new()));
        let repo = Arc::new(MockDigitalTwinRepository {
            saved_twins: saved_twins.clone(),
        });
        let use_case = CreateTwinUseCase::new(repo);

        let mut initial_properties = HashMap::new();
        initial_properties.insert(
            "temperature".to_string(),
            serde_json::Value::Number(serde_json::Number::from(25)),
        );
        initial_properties.insert(
            "location".to_string(),
            serde_json::Value::String("Building A".to_string()),
        );

        let command = CreateTwinCommand {
            name: "HVAC System Twin".to_string(),
            description: Some("Digital twin of building HVAC system".to_string()),
            twin_type: "hvac_system".to_string(),
            initial_properties,
            tags: vec!["hvac".to_string(), "building".to_string()],
        };

        let result = use_case.execute(command.clone()).await;
        assert!(result.is_ok());

        let twin = result.unwrap();
        assert_eq!(twin.name, command.name);
        assert_eq!(twin.description, command.description);
        assert_eq!(twin.twin_type, command.twin_type);
        assert_eq!(twin.properties.len(), 2);
        assert_eq!(twin.status, TwinStatus::Idle);
        assert_eq!(twin.metadata.tags, command.tags);

        let saved = saved_twins.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].id, twin.id);
    }

    #[tokio::test]
    async fn test_create_twin_empty_name_fails() {
        let saved_twins = Arc::new(Mutex::new(Vec::new()));
        let repo = Arc::new(MockDigitalTwinRepository {
            saved_twins: saved_twins.clone(),
        });
        let use_case = CreateTwinUseCase::new(repo);

        let command = CreateTwinCommand {
            name: "".to_string(),
            description: None,
            twin_type: "test".to_string(),
            initial_properties: HashMap::new(),
            tags: vec![],
        };

        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::ValidationError(_)));
    }

    #[tokio::test]
    async fn test_create_twin_empty_type_fails() {
        let saved_twins = Arc::new(Mutex::new(Vec::new()));
        let repo = Arc::new(MockDigitalTwinRepository {
            saved_twins: saved_twins.clone(),
        });
        let use_case = CreateTwinUseCase::new(repo);

        let command = CreateTwinCommand {
            name: "Test Twin".to_string(),
            description: None,
            twin_type: "".to_string(),
            initial_properties: HashMap::new(),
            tags: vec![],
        };

        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::ValidationError(_)));
    }
}