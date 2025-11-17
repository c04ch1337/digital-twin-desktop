use crate::core::domain::{
    errors::DomainError,
    models::digital_twin::{DigitalTwin, TwinId, TwinStatus, SimulationResult},
    models::sensor_data::{SensorData, SensorReading},
    traits::repository::{DigitalTwinRepository, SensorDataRepository},
};
use std::sync::Arc;
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Simulation parameters
#[derive(Debug, Clone)]
pub struct SimulationParams {
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<SimulationScenario>,
    pub variables: HashMap<String, f64>,
}

/// Simulation scenario definition
#[derive(Debug, Clone)]
pub struct SimulationScenario {
    pub name: String,
    pub conditions: HashMap<String, f64>,
    pub start_hour: u32,
    pub end_hour: u32,
}

/// Command to run a simulation on a digital twin
#[derive(Debug, Clone)]
pub struct RunSimulationCommand {
    pub twin_id: TwinId,
    pub simulation_type: String,
    pub params: SimulationParams,
}

/// Response from running a simulation
#[derive(Debug, Clone)]
pub struct RunSimulationResponse {
    pub twin: DigitalTwin,
    pub simulation_id: String,
    pub results: SimulationResult,
    pub predicted_readings: Vec<PredictedReading>,
}

/// Predicted sensor reading from simulation
#[derive(Debug, Clone)]
pub struct PredictedReading {
    pub timestamp: DateTime<Utc>,
    pub sensor_name: String,
    pub predicted_value: f64,
    pub confidence: f64,
}

/// Use case for running simulations on digital twins
pub struct RunSimulationUseCase {
    twin_repo: Arc<dyn DigitalTwinRepository>,
    sensor_repo: Arc<dyn SensorDataRepository>,
}

impl RunSimulationUseCase {
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
        command: RunSimulationCommand,
    ) -> Result<RunSimulationResponse, DomainError> {
        // Validate parameters
        if command.params.duration_hours == 0 {
            return Err(DomainError::ValidationError(
                "Simulation duration must be greater than 0".to_string(),
            ));
        }

        if command.params.time_step_minutes == 0 {
            return Err(DomainError::ValidationError(
                "Time step must be greater than 0".to_string(),
            ));
        }

        // Retrieve the digital twin
        let mut twin = self
            .twin_repo
            .find_by_id(&command.twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        // Check if twin is in a valid state for simulation
        if twin.status == TwinStatus::Error {
            return Err(DomainError::InvalidState(
                "Cannot run simulation on twin in error state".to_string(),
            ));
        }

        // Update status to simulating
        twin.status = TwinStatus::Simulating;
        twin.updated_at = Utc::now();
        
        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        // Get historical sensor data for calibration
        let sensor_data = self
            .sensor_repo
            .find_by_twin_id(&command.twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        // Generate simulation ID
        let simulation_id = format!("sim_{}_{}_{}", 
            command.twin_id.to_string(),
            command.simulation_type,
            Utc::now().timestamp()
        );

        // Run the simulation based on type
        let (results, predicted_readings) = match command.simulation_type.as_str() {
            "hvac_optimization" => {
                self.simulate_hvac_optimization(&twin, &sensor_data, &command.params)
            }
            "failure_prediction" => {
                self.simulate_failure_prediction(&twin, &sensor_data, &command.params)
            }
            "energy_consumption" => {
                self.simulate_energy_consumption(&twin, &sensor_data, &command.params)
            }
            _ => {
                self.run_generic_simulation(&twin, &sensor_data, &command.params)
            }
        }?;

        // Store simulation results
        twin.simulation_results.insert(
            simulation_id.clone(),
            results.clone(),
        );

        // Update twin status
        twin.status = TwinStatus::Active;
        twin.updated_at = Utc::now();

        // Save updated twin
        self.twin_repo
            .update(&twin)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(RunSimulationResponse {
            twin,
            simulation_id,
            results,
            predicted_readings,
        })
    }

    fn simulate_hvac_optimization(
        &self,
        twin: &DigitalTwin,
        sensor_data: &[SensorData],
        params: &SimulationParams,
    ) -> Result<(SimulationResult, Vec<PredictedReading>), DomainError> {
        let mut predicted_readings = Vec::new();
        let mut metrics = HashMap::new();
        
        // Calculate baseline from sensor data
        let baseline_temp = self.calculate_average_value(sensor_data, "temperature");
        let baseline_energy = twin.properties.get("energy_consumption")
            .and_then(|v| v.as_f64())
            .unwrap_or(100.0);

        // Simulate over time periods
        let total_steps = (params.duration_hours * 60) / params.time_step_minutes;
        
        let mut current_time = Utc::now();
        let mut total_energy_saved = 0.0;
        let mut comfort_violations = 0;

        for step in 0..total_steps {
            let hour = (step * params.time_step_minutes / 60) as u32;
            
            // Apply scenarios
            let mut target_temp = 22.0; // Default target
            for scenario in &params.scenarios {
                if hour >= scenario.start_hour && hour <= scenario.end_hour {
                    target_temp = scenario.conditions.get("target_temperature")
                        .copied()
                        .unwrap_or(target_temp);
                }
            }

            // Simple HVAC model
            let outdoor_temp = 30.0 - (hour as f64 * 0.5); // Simplified outdoor temp curve
            let heat_load = (outdoor_temp - target_temp).abs() * 2.0;
            let optimized_energy = heat_load * 0.7; // 30% efficiency improvement
            
            total_energy_saved += baseline_energy - optimized_energy;

            // Check comfort violations
            if (target_temp - 20.0).abs() > 3.0 {
                comfort_violations += 1;
            }

            // Generate predictions
            predicted_readings.push(PredictedReading {
                timestamp: current_time,
                sensor_name: "temperature".to_string(),
                predicted_value: target_temp,
                confidence: 0.85,
            });

            predicted_readings.push(PredictedReading {
                timestamp: current_time,
                sensor_name: "energy_consumption".to_string(), 
                predicted_value: optimized_energy,
                confidence: 0.75,
            });

            current_time = current_time + chrono::Duration::minutes(params.time_step_minutes as i64);
        }

        metrics.insert("total_energy_saved_kwh".to_string(), total_energy_saved);
        metrics.insert("comfort_violations".to_string(), comfort_violations as f64);
        metrics.insert("avg_comfort_score".to_string(), 100.0 - (comfort_violations as f64 / total_steps as f64 * 100.0));

        let result = SimulationResult {
            simulation_type: "hvac_optimization".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(params.duration_hours as i64),
            end_time: Utc::now(),
            status: "completed".to_string(),
            metrics,
            recommendations: vec![
                "Implement adaptive temperature setpoints based on occupancy".to_string(),
                "Consider upgrading to variable speed HVAC units".to_string(),
            ],
        };

        Ok((result, predicted_readings))
    }

    fn simulate_failure_prediction(
        &self,
        _twin: &DigitalTwin,
        sensor_data: &[SensorData],
        params: &SimulationParams,
    ) -> Result<(SimulationResult, Vec<PredictedReading>), DomainError> {
        let mut predicted_readings = Vec::new();
        let mut metrics = HashMap::new();

        // Analyze historical patterns for anomalies
        let vibration_trend = self.calculate_trend(sensor_data, "vibration");
        let temperature_variance = self.calculate_variance(sensor_data, "temperature");

        // Predict failure probability over time
        let mut current_time = Utc::now();
        let total_steps = (params.duration_hours * 60) / params.time_step_minutes;
        let mut max_failure_prob = 0.0;

        for step in 0..total_steps {
            // Simple failure model based on degradation
            let hours_elapsed = step * params.time_step_minutes / 60;
            let base_failure_rate = 0.001; // 0.1% per hour baseline
            let degradation_factor = 1.0 + (vibration_trend * hours_elapsed as f64);
            let temp_factor = 1.0 + (temperature_variance * 0.1);
            
            let failure_probability = base_failure_rate * degradation_factor * temp_factor;
            max_failure_prob = max_failure_prob.max(failure_probability);

            predicted_readings.push(PredictedReading {
                timestamp: current_time,
                sensor_name: "failure_probability".to_string(),
                predicted_value: failure_probability,
                confidence: 0.7,
            });

            current_time = current_time + chrono::Duration::minutes(params.time_step_minutes as i64);
        }

        metrics.insert("max_failure_probability".to_string(), max_failure_prob);
        metrics.insert("time_to_maintenance_hours".to_string(), (1.0 / max_failure_prob).min(8760.0));
        
        let recommendations = if max_failure_prob > 0.05 {
            vec![
                "Schedule preventive maintenance within 48 hours".to_string(),
                "Increase monitoring frequency for critical sensors".to_string(),
            ]
        } else {
            vec!["Continue normal monitoring schedule".to_string()]
        };

        let result = SimulationResult {
            simulation_type: "failure_prediction".to_string(),
            start_time: Utc::now(),
            end_time: current_time,
            status: "completed".to_string(),
            metrics,
            recommendations,
        };

        Ok((result, predicted_readings))
    }

    fn simulate_energy_consumption(
        &self,
        twin: &DigitalTwin,
        sensor_data: &[SensorData],
        params: &SimulationParams,
    ) -> Result<(SimulationResult, Vec<PredictedReading>), DomainError> {
        let mut predicted_readings = Vec::new();
        let mut metrics = HashMap::new();

        // Base consumption from properties
        let base_consumption = twin.properties.get("rated_power")
            .and_then(|v| v.as_f64())
            .unwrap_or(1000.0);

        let mut current_time = Utc::now();
        let total_steps = (params.duration_hours * 60) / params.time_step_minutes;
        let mut total_consumption = 0.0;
        let mut peak_demand = 0.0;

        for step in 0..total_steps {
            let hour = (step * params.time_step_minutes / 60) % 24;
            
            // Usage pattern based on time of day
            let usage_factor = match hour {
                6..=9 | 17..=21 => 0.9,   // Peak hours
                10..=16 => 0.7,            // Normal hours
                _ => 0.3,                  // Off-peak hours
            };

            let consumption = base_consumption * usage_factor;
            total_consumption += consumption * (params.time_step_minutes as f64 / 60.0);
            peak_demand = peak_demand.max(consumption);

            predicted_readings.push(PredictedReading {
                timestamp: current_time,
                sensor_name: "power_consumption".to_string(),
                predicted_value: consumption,
                confidence: 0.8,
            });

            current_time = current_time + chrono::Duration::minutes(params.time_step_minutes as i64);
        }

        metrics.insert("total_consumption_kwh".to_string(), total_consumption);
        metrics.insert("peak_demand_kw".to_string(), peak_demand);
        metrics.insert("avg_consumption_kw".to_string(), total_consumption / params.duration_hours as f64);

        let result = SimulationResult {
            simulation_type: "energy_consumption".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(params.duration_hours as i64),
            end_time: Utc::now(),
            status: "completed".to_string(),
            metrics,
            recommendations: vec![
                "Consider load shifting to off-peak hours".to_string(),
                "Implement demand response strategies during peak periods".to_string(),
            ],
        };

        Ok((result, predicted_readings))
    }

    fn run_generic_simulation(
        &self,
        twin: &DigitalTwin,
        _sensor_data: &[SensorData],
        params: &SimulationParams,
    ) -> Result<(SimulationResult, Vec<PredictedReading>), DomainError> {
        let mut metrics = HashMap::new();
        metrics.insert("simulation_steps".to_string(), 
            ((params.duration_hours * 60) / params.time_step_minutes) as f64);
        metrics.insert("scenarios_count".to_string(), params.scenarios.len() as f64);

        let result = SimulationResult {
            simulation_type: "generic".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(params.duration_hours as i64),
            status: "completed".to_string(),
            metrics,
            recommendations: vec!["Generic simulation completed successfully".to_string()],
        };

        Ok((result, Vec::new()))
    }

    // Helper methods
    fn calculate_average_value(&self, sensor_data: &[SensorData], sensor_type: &str) -> f64 {
        let relevant_data: Vec<_> = sensor_data
            .iter()
            .filter(|d| d.sensor_type == sensor_type)
            .collect();

        if relevant_data.is_empty() {
            return 0.0;
        }

        let total: f64 = relevant_data
            .iter()
            .flat_map(|d| &d.readings)
            .map(|r| r.value)
            .sum();

        let count = relevant_data
            .iter()
            .map(|d| d.readings.len())
            .sum::<usize>();

        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    fn calculate_trend(&self, sensor_data: &[SensorData], sensor_type: &str) -> f64 {
        let relevant_data: Vec<_> = sensor_data
            .iter()
            .filter(|d| d.sensor_type == sensor_type)
            .flat_map(|d| &d.readings)
            .collect();

        if relevant_data.len() < 2 {
            return 0.0;
        }

        // Simple linear trend
        let first_value = relevant_data.first().unwrap().value;
        let last_value = relevant_data.last().unwrap().value;
        
        (last_value - first_value) / relevant_data.len() as f64
    }

    fn calculate_variance(&self, sensor_data: &[SensorData], sensor_type: &str) -> f64 {
        let values: Vec<f64> = sensor_data
            .iter()
            .filter(|d| d.sensor_type == sensor_type)
            .flat_map(|d| &d.readings)
            .map(|r| r.value)
            .collect();

        if values.is_empty() {
            return 0.0;
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;

        variance.sqrt()
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
    async fn test_run_hvac_simulation() {
        let twin_id = TwinId::new();
        
        let mut properties = HashMap::new();
        properties.insert("energy_consumption".to_string(), serde_json::json!(150.0));
        
        let twin = DigitalTwin::new(
            twin_id.clone(),
            "HVAC Twin".to_string(),
            None,
            "hvac_system".to_string(),
            properties,
            HashMap::new(),
            TwinStatus::Active,
            HashMap::new(),
            Default::default(),
            Utc::now(),
            Utc::now(),
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
                        value: 22.5,
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
        
        let use_case = RunSimulationUseCase::new(twin_repo, sensor_repo);
        
        let params = SimulationParams {
            duration_hours: 2,
            time_step_minutes: 30,
            scenarios: vec![
                SimulationScenario {
                    name: "Morning".to_string(),
                    conditions: {
                        let mut conditions = HashMap::new();
                        conditions.insert("target_temperature".to_string(), 21.0);
                        conditions
                    },
                    start_hour: 0,
                    end_hour: 12,
                },
            ],
            variables: HashMap::new(),
        };
        
        let command = RunSimulationCommand {
            twin_id,
            simulation_type: "hvac_optimization".to_string(),
            params,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.simulation_id.starts_with("sim_"));
        assert_eq!(response.results.simulation_type, "hvac_optimization");
        assert!(!response.predicted_readings.is_empty());
        assert!(response.results.metrics.contains_key("total_energy_saved_kwh"));
    }
}