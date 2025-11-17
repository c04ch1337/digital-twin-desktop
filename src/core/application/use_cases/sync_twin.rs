use crate::core::domain::{
    errors::DomainError,
    models::digital_twin::{DigitalTwin, TwinId, TwinStatus},
    models::sensor_data::{SensorData, SensorReading},
    traits::repository::{DigitalTwinRepository, SensorDataRepository},
};
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;

/// Command to synchronize a digital twin with data sources
#[derive(Debug, Clone)]
pub struct SyncTwinCommand {
    pub twin_id: TwinId,
    pub force_sync: bool, // Force sync even if recently synced
}

/// Response from synchronizing a twin
#[derive(Debug, Clone)]
pub struct SyncTwinResponse {
    pub twin: DigitalTwin,
    pub new_readings: Vec<SensorReading>,
    pub updated_properties: HashMap<String, serde_json::Value>,
}

/// Use case for synchronizing a digital twin with its data sources
pub struct SyncTwinUseCase {
    twin_repo: Arc<dyn DigitalTwinRepository>,
    sensor_repo: Arc<dyn SensorDataRepository>,
}

impl SyncTwinUseCase {
    pub fn new(
        twin_repo: Arc<dyn DigitalTwinRepository>,
        sensor_repo: Arc<dyn SensorDataRepository>,
    ) -> Self {
        Self {
            twin_repo,
            sensor_repo,
        }
    }

    pub async fn execute(
        &self,
        command: SyncTwinCommand,
    ) -> Result<SyncTwinResponse, DomainError> {
        // Retrieve the digital twin
        let mut twin = self
            .twin_repo
            .find_by_id(&command.twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        // Check if sync is needed (unless forced)
        if !command.force_sync {
            let last_sync = twin.updated_at;
            let time_since_sync = Utc::now() - last_sync;
            
            // Skip if synced within last minute
            if time_since_sync.num_seconds() < 60 && twin.status != TwinStatus::Error {
                return Ok(SyncTwinResponse {
                    twin,
                    new_readings: vec![],
                    updated_properties: HashMap::new(),
                });
            }
        }

        // Update status to syncing
        twin.status = TwinStatus::Syncing;
        twin.updated_at = Utc::now();
        
        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        // Fetch latest sensor data
        let sensor_data = self
            .sensor_repo
            .find_by_twin_id(&command.twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let mut new_readings = Vec::new();
        let mut updated_properties = HashMap::new();

        // Process sensor data
        for data in sensor_data {
            // Get latest reading for each sensor
            if let Some(latest_reading) = data.readings.last() {
                new_readings.push(latest_reading.clone());
                
                // Update twin's sensor data
                twin.sensor_data.insert(
                    data.sensor_name.clone(),
                    data.clone(),
                );
                
                // Update properties based on sensor readings
                match data.sensor_type.as_str() {
                    "temperature" => {
                        updated_properties.insert(
                            "current_temperature".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                        twin.properties.insert(
                            "current_temperature".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                    }
                    "humidity" => {
                        updated_properties.insert(
                            "current_humidity".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                        twin.properties.insert(
                            "current_humidity".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                    }
                    "pressure" => {
                        updated_properties.insert(
                            "current_pressure".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                        twin.properties.insert(
                            "current_pressure".to_string(),
                            serde_json::json!(latest_reading.value),
                        );
                    }
                    _ => {
                        // Generic property update
                        let property_name = format!("sensor_{}", data.sensor_name);
                        updated_properties.insert(
                            property_name.clone(),
                            serde_json::json!(latest_reading.value),
                        );
                        twin.properties.insert(
                            property_name,
                            serde_json::json!(latest_reading.value),
                        );
                    }
                }
            }
        }

        // Check for anomalies or alerts
        let has_anomalies = new_readings.iter().any(|reading| {
            reading.metadata.as_ref()
                .and_then(|m| m.get("alert"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        });

        // Update status based on sync results
        twin.status = if has_anomalies {
            TwinStatus::Error
        } else if !new_readings.is_empty() {
            TwinStatus::Active
        } else {
            TwinStatus::Idle
        };

        twin.updated_at = Utc::now();

        // Save updated twin
        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(SyncTwinResponse {
            twin,
            new_readings,
            updated_properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockDigitalTwinRepository {
        twins: Arc<Mutex<Vec<DigitalTwin>>>,
    }

    #[async_trait]
    impl DigitalTwinRepository for MockDigitalTwinRepository {
        async fn save(&self, _twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_id(&self, id: &TwinId) -> Result<Option<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            let twins = self.twins.lock().unwrap();
            Ok(twins.iter().find(|t| t.id == *id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, twin: &DigitalTwin) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut twins = self.twins.lock().unwrap();
            if let Some(index) = twins.iter().position(|t| t.id == twin.id) {
                twins[index] = twin.clone();
            }
            Ok(())
        }

        async fn delete(&self, _id: &TwinId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_type(&self, _twin_type: &str) -> Result<Vec<DigitalTwin>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    struct MockSensorDataRepository {
        sensor_data: Arc<Mutex<Vec<SensorData>>>,
    }

    #[async_trait]
    impl SensorDataRepository for MockSensorDataRepository {
        async fn save(&self, _data: &SensorData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn find_by_twin_id(&self, twin_id: &TwinId) -> Result<Vec<SensorData>, Box<dyn std::error::Error + Send + Sync>> {
            let data = self.sensor_data.lock().unwrap();
            Ok(data
                .iter()
                .filter(|d| d.twin_id == *twin_id)
                .cloned()
                .collect())
        }

        async fn find_by_sensor_name(&self, _twin_id: &TwinId, _sensor_name: &str) -> Result<Option<SensorData>, Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn update(&self, _data: &SensorData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }

        async fn delete_by_twin_id(&self, _twin_id: &TwinId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_sync_twin_with_sensor_data() {
        let twin_id = TwinId::new();
        
        let twin = DigitalTwin::new(
            twin_id.clone(),
            "Test Twin".to_string(),
            None,
            "test_type".to_string(),
            HashMap::new(),
            HashMap::new(),
            TwinStatus::Idle,
            HashMap::new(),
            Default::default(),
            Utc::now() - chrono::Duration::minutes(5), // Created 5 minutes ago
            Utc::now() - chrono::Duration::minutes(5), // Last updated 5 minutes ago
        );
        
        let sensor_data = vec![
            SensorData {
                twin_id: twin_id.clone(),
                sensor_name: "temp_sensor".to_string(),
                sensor_type: "temperature".to_string(),
                unit: "celsius".to_string(),
                readings: vec![
                    SensorReading {
                        timestamp: Utc::now(),
                        value: 25.5,
                        metadata: None,
                    },
                ],
            },
            SensorData {
                twin_id: twin_id.clone(),
                sensor_name: "humidity_sensor".to_string(),
                sensor_type: "humidity".to_string(),
                unit: "percent".to_string(),
                readings: vec![
                    SensorReading {
                        timestamp: Utc::now(),
                        value: 65.0,
                        metadata: None,
                    },
                ],
            },
        ];
        
        let twins = Arc::new(Mutex::new(vec![twin]));
        let sensor_data_store = Arc::new(Mutex::new(sensor_data));
        
        let twin_repo = Arc::new(MockDigitalTwinRepository { twins: twins.clone() });
        let sensor_repo = Arc::new(MockSensorDataRepository {
            sensor_data: sensor_data_store,
        });
        
        let use_case = SyncTwinUseCase::new(twin_repo, sensor_repo);
        
        let command = SyncTwinCommand {
            twin_id,
            force_sync: false,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.new_readings.len(), 2);
        assert_eq!(response.updated_properties.len(), 2);
        assert!(response.updated_properties.contains_key("current_temperature"));
        assert!(response.updated_properties.contains_key("current_humidity"));
        assert_eq!(response.twin.status, TwinStatus::Active);
        assert_eq!(response.twin.sensor_data.len(), 2);
    }

    #[tokio::test]
    async fn test_sync_twin_skip_recent_sync() {
        let twin_id = TwinId::new();
        
        let twin = DigitalTwin::new(
            twin_id.clone(),
            "Test Twin".to_string(),
            None,
            "test_type".to_string(),
            HashMap::new(),
            HashMap::new(),
            TwinStatus::Active,
            HashMap::new(),
            Default::default(),
            Utc::now(),
            Utc::now(), // Just updated
        );
        
        let twins = Arc::new(Mutex::new(vec![twin]));
        let sensor_data_store = Arc::new(Mutex::new(vec![]));
        
        let twin_repo = Arc::new(MockDigitalTwinRepository { twins: twins.clone() });
        let sensor_repo = Arc::new(MockSensorDataRepository {
            sensor_data: sensor_data_store,
        });
        
        let use_case = SyncTwinUseCase::new(twin_repo, sensor_repo);
        
        let command = SyncTwinCommand {
            twin_id,
            force_sync: false,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.new_readings.len(), 0);
        assert_eq!(response.updated_properties.len(), 0);
        assert_eq!(response.twin.status, TwinStatus::Active); // Status unchanged
    }
}