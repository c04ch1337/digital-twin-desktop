use crate::core::{
    domain::{
        errors::DomainError,
        models::digital_twin::{DigitalTwin, TwinId, TwinStatus},
        traits::repository::{DigitalTwinRepository, SensorDataRepository},
    },
    application::use_cases::{
        create_twin::{CreateTwinCommand, CreateTwinUseCase},
        sync_twin::{SyncTwinCommand, SyncTwinResponse, SyncTwinUseCase},
        run_simulation::{
            RunSimulationCommand, RunSimulationResponse, RunSimulationUseCase,
            SimulationParams, SimulationScenario,
        },
    },
};
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

/// Service for digital twin management
pub struct TwinService {
    create_twin_use_case: CreateTwinUseCase,
    sync_twin_use_case: SyncTwinUseCase,
    run_simulation_use_case: RunSimulationUseCase,
    twin_repo: Arc<dyn DigitalTwinRepository>,
    sensor_repo: Arc<dyn SensorDataRepository>,
}

impl TwinService {
    pub fn new(
        create_twin_use_case: CreateTwinUseCase,
        sync_twin_use_case: SyncTwinUseCase,
        run_simulation_use_case: RunSimulationUseCase,
        twin_repo: Arc<dyn DigitalTwinRepository>,
        sensor_repo: Arc<dyn SensorDataRepository>,
    ) -> Self {
        Self {
            create_twin_use_case,
            sync_twin_use_case,
            run_simulation_use_case,
            twin_repo,
            sensor_repo,
        }
    }

    /// Create a new digital twin
    pub async fn create_twin(
        &self,
        name: String,
        description: Option<String>,
        twin_type: String,
        initial_properties: HashMap<String, Value>,
        tags: Vec<String>,
    ) -> Result<DigitalTwin, DomainError> {
        let command = CreateTwinCommand {
            name,
            description,
            twin_type,
            initial_properties,
            tags,
        };

        self.create_twin_use_case.execute(command).await
    }

    /// Get twin by ID
    pub async fn get_twin(&self, twin_id: &TwinId) -> Result<Option<DigitalTwin>, DomainError> {
        self.twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// List all twins
    pub async fn list_twins(&self) -> Result<Vec<DigitalTwin>, DomainError> {
        self.twin_repo
            .find_all()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// List twins by type
    pub async fn list_twins_by_type(&self, twin_type: &str) -> Result<Vec<DigitalTwin>, DomainError> {
        self.twin_repo
            .find_by_type(twin_type)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Synchronize twin with data sources
    pub async fn sync_twin(
        &self,
        twin_id: TwinId,
        force_sync: bool,
    ) -> Result<SyncTwinResponse, DomainError> {
        let command = SyncTwinCommand {
            twin_id,
            force_sync,
        };

        self.sync_twin_use_case.execute(command).await
    }

    /// Run simulation on twin
    pub async fn run_simulation(
        &self,
        twin_id: TwinId,
        simulation_type: String,
        params: SimulationParams,
    ) -> Result<RunSimulationResponse, DomainError> {
        let command = RunSimulationCommand {
            twin_id,
            simulation_type,
            params,
        };

        self.run_simulation_use_case.execute(command).await
    }

    /// Update twin properties
    pub async fn update_twin_properties(
        &self,
        twin_id: &TwinId,
        properties: HashMap<String, Value>,
    ) -> Result<DigitalTwin, DomainError> {
        let mut twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        // Update properties
        for (key, value) in properties {
            twin.properties.insert(key, value);
        }

        twin.updated_at = chrono::Utc::now();

        // Save updated twin
        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(twin)
    }

    /// Update twin status
    pub async fn update_twin_status(
        &self,
        twin_id: &TwinId,
        status: TwinStatus,
    ) -> Result<DigitalTwin, DomainError> {
        let mut twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        twin.status = status;
        twin.updated_at = chrono::Utc::now();

        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(twin)
    }

    /// Delete a digital twin
    pub async fn delete_twin(&self, twin_id: &TwinId) -> Result<(), DomainError> {
        // Verify twin exists
        self.twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        // Delete associated sensor data
        self.sensor_repo
            .delete_by_twin_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        // Delete twin
        self.twin_repo
            .delete(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    /// Get simulation results for a twin
    pub async fn get_simulation_results(
        &self,
        twin_id: &TwinId,
    ) -> Result<HashMap<String, crate::core::domain::models::digital_twin::SimulationResult>, DomainError> {
        let twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        Ok(twin.simulation_results)
    }

    /// Clone a digital twin
    pub async fn clone_twin(
        &self,
        source_twin_id: &TwinId,
        new_name: String,
    ) -> Result<DigitalTwin, DomainError> {
        // Get source twin
        let source_twin = self
            .twin_repo
            .find_by_id(source_twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Source twin not found".to_string()))?;

        // Create new twin with copied data
        let command = CreateTwinCommand {
            name: new_name,
            description: source_twin.description.map(|d| format!("Clone of: {}", d)),
            twin_type: source_twin.twin_type,
            initial_properties: source_twin.properties,
            tags: {
                let mut tags = source_twin.metadata.tags;
                tags.push("cloned".to_string());
                tags
            },
        };

        self.create_twin_use_case.execute(command).await
    }

    /// Get twins with active simulations
    pub async fn get_twins_with_active_simulations(&self) -> Result<Vec<DigitalTwin>, DomainError> {
        let all_twins = self.twin_repo
            .find_all()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(all_twins
            .into_iter()
            .filter(|twin| twin.status == TwinStatus::Simulating)
            .collect())
    }

    /// Get twins in error state
    pub async fn get_twins_in_error(&self) -> Result<Vec<DigitalTwin>, DomainError> {
        let all_twins = self.twin_repo
            .find_all()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(all_twins
            .into_iter()
            .filter(|twin| twin.status == TwinStatus::Error)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::models::digital_twin::{DigitalTwin, TwinMetadata};
    use crate::core::domain::models::sensor_data::SensorData;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockDigitalTwinRepository {
        twins: Arc<Mutex<Vec<DigitalTwin>>>,
    }

    #[async_trait]
    impl DigitalTwinRepository for MockDigitalTwinRepository {
        async fn save(&self, twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.twins.lock().unwrap().push(twin.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &TwinId) -> Result<Option<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            let twins = self.twins.lock().unwrap();
            Ok(twins.iter().find(|t| t.id == *id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.twins.lock().unwrap().clone())
        }

        async fn update(&self, twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut twins = self.twins.lock().unwrap();
            if let Some(index) = twins.iter().position(|t| t.id == twin.id) {
                twins[index] = twin.clone();
            }
            Ok(())
        }

        async fn delete(&self, id: &TwinId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut twins = self.twins.lock().unwrap();
            twins.retain(|t| t.id != *id);
            Ok(())
        }

        async fn find_by_type(&self, twin_type: &str) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            let twins = self.twins.lock().unwrap();
            Ok(twins.iter()
                .filter(|t| t.twin_type == twin_type)
                .cloned()
                .collect())
        }
    }

    struct MockSensorDataRepository {}

    #[async_trait]
    impl SensorDataRepository for MockSensorDataRepository {
        async fn save(&self, _data: &SensorData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }

        async fn find_by_twin_id(&self, _twin_id: &TwinId) -> Result<Vec<SensorData>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }

        async fn find_by_sensor_name(&self, _twin_id: &TwinId, _sensor_name: &str) -> Result<Option<SensorData>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(None)
        }

        async fn update(&self, _data: &SensorData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }

        async fn delete_by_twin_id(&self, _twin_id: &TwinId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_and_update_twin() {
        let twins = Arc::new(Mutex::new(Vec::new()));
        
        let twin_repo = Arc::new(MockDigitalTwinRepository { twins: twins.clone() });
        let sensor_repo = Arc::new(MockSensorDataRepository {});
        
        let create_twin_use_case = CreateTwinUseCase::new(twin_repo.clone());
        let sync_twin_use_case = SyncTwinUseCase::new(twin_repo.clone(), sensor_repo.clone());
        let run_simulation_use_case = RunSimulationUseCase::new(twin_repo.clone(), sensor_repo.clone());
        
        let service = TwinService::new(
            create_twin_use_case,
            sync_twin_use_case,
            run_simulation_use_case,
            twin_repo.clone(),
            sensor_repo,
        );
        
        // Create twin
        let result = service.create_twin(
            "Test Twin".to_string(),
            Some("Test description".to_string()),
            "test_type".to_string(),
            HashMap::new(),
            vec!["test".to_string()],
        ).await;
        
        assert!(result.is_ok());
        let twin = result.unwrap();
        
        // Update properties
        let mut new_props = HashMap::new();
        new_props.insert("temperature".to_string(), serde_json::json!(25.0));
        
        let update_result = service.update_twin_properties(&twin.id, new_props).await;
        assert!(update_result.is_ok());
        
        let updated_twin = update_result.unwrap();
        assert!(updated_twin.properties.contains_key("temperature"));
    }
}