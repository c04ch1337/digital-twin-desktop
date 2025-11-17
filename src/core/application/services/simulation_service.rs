use crate::core::{
    domain::{
        errors::DomainError,
        models::digital_twin::{TwinId, SimulationResult},
        traits::repository::{DigitalTwinRepository, SensorDataRepository},
    },
    application::use_cases::run_simulation::{
        RunSimulationCommand, RunSimulationResponse, RunSimulationUseCase,
        SimulationParams, SimulationScenario, PredictedReading,
    },
};
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{Utc, DateTime};

/// Service for simulation orchestration
pub struct SimulationService {
    run_simulation_use_case: RunSimulationUseCase,
    twin_repo: Arc<dyn DigitalTwinRepository>,
    sensor_repo: Arc<dyn SensorDataRepository>,
}

/// Simulation configuration
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub duration_hours: u32,
    pub time_step_minutes: u32,
    pub scenarios: Vec<ScenarioConfig>,
    pub parameters: HashMap<String, f64>,
}

/// Scenario configuration
#[derive(Debug, Clone)]
pub struct ScenarioConfig {
    pub name: String,
    pub start_hour: u32,
    pub end_hour: u32,
    pub conditions: HashMap<String, f64>,
}

/// Batch simulation request
#[derive(Debug, Clone)]
pub struct BatchSimulationRequest {
    pub twin_ids: Vec<TwinId>,
    pub simulation_type: String,
    pub config: SimulationConfig,
}

/// Batch simulation result
#[derive(Debug)]
pub struct BatchSimulationResult {
    pub successful: Vec<(TwinId, RunSimulationResponse)>,
    pub failed: Vec<(TwinId, DomainError)>,
}

impl SimulationService {
    pub fn new(
        run_simulation_use_case: RunSimulationUseCase,
        twin_repo: Arc<dyn DigitalTwinRepository>,
        sensor_repo: Arc<dyn SensorDataRepository>,
    ) -> Self {
        Self {
            run_simulation_use_case,
            twin_repo,
            sensor_repo,
        }
    }

    /// Run a single simulation
    pub async fn run_simulation(
        &self,
        twin_id: TwinId,
        simulation_type: String,
        config: SimulationConfig,
    ) -> Result<RunSimulationResponse, DomainError> {
        let params = self.config_to_params(config);
        
        let command = RunSimulationCommand {
            twin_id,
            simulation_type,
            params,
        };

        self.run_simulation_use_case.execute(command).await
    }

    /// Run batch simulations on multiple twins
    pub async fn run_batch_simulations(
        &self,
        request: BatchSimulationRequest,
    ) -> Result<BatchSimulationResult, DomainError> {
        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for twin_id in request.twin_ids {
            let params = self.config_to_params(request.config.clone());
            
            let command = RunSimulationCommand {
                twin_id: twin_id.clone(),
                simulation_type: request.simulation_type.clone(),
                params,
            };

            match self.run_simulation_use_case.execute(command).await {
                Ok(response) => successful.push((twin_id, response)),
                Err(error) => failed.push((twin_id, error)),
            }
        }

        Ok(BatchSimulationResult {
            successful,
            failed,
        })
    }

    /// Get simulation history for a twin
    pub async fn get_simulation_history(
        &self,
        twin_id: &TwinId,
    ) -> Result<Vec<SimulationResult>, DomainError> {
        let twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        Ok(twin.simulation_results.into_values().collect())
    }

    /// Get latest simulation result for a twin
    pub async fn get_latest_simulation(
        &self,
        twin_id: &TwinId,
    ) -> Result<Option<SimulationResult>, DomainError> {
        let twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        Ok(twin.simulation_results
            .into_values()
            .max_by_key(|result| result.end_time))
    }

    /// Run comparative simulations
    pub async fn run_comparative_simulation(
        &self,
        twin_id: TwinId,
        simulation_type: String,
        configs: Vec<SimulationConfig>,
    ) -> Result<Vec<RunSimulationResponse>, DomainError> {
        let mut results = Vec::new();

        for config in configs {
            let params = self.config_to_params(config);
            
            let command = RunSimulationCommand {
                twin_id: twin_id.clone(),
                simulation_type: simulation_type.clone(),
                params,
            };

            let response = self.run_simulation_use_case.execute(command).await?;
            results.push(response);
        }

        Ok(results)
    }

    /// Schedule a simulation for future execution
    pub async fn schedule_simulation(
        &self,
        twin_id: TwinId,
        simulation_type: String,
        config: SimulationConfig,
        scheduled_time: DateTime<Utc>,
    ) -> Result<String, DomainError> {
        // For now, we'll just validate that the twin exists
        // In a real implementation, this would integrate with a job scheduler
        self.twin_repo
            .find_by_id(&twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        // Generate a schedule ID
        let schedule_id = format!("schedule_{}_{}", twin_id, Utc::now().timestamp());

        // TODO: Integrate with job scheduler to actually schedule the simulation
        
        Ok(schedule_id)
    }

    /// Analyze simulation results
    pub async fn analyze_simulation_results(
        &self,
        twin_id: &TwinId,
        simulation_id: &str,
    ) -> Result<SimulationAnalysis, DomainError> {
        let twin = self
            .twin_repo
            .find_by_id(twin_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Digital twin not found".to_string()))?;

        let result = twin.simulation_results
            .get(simulation_id)
            .ok_or_else(|| DomainError::NotFound("Simulation result not found".to_string()))?;

        // Perform analysis
        let total_metrics = result.metrics.len();
        let avg_metric_value = if total_metrics > 0 {
            result.metrics.values().sum::<f64>() / total_metrics as f64
        } else {
            0.0
        };

        let key_insights = self.generate_insights(result);

        Ok(SimulationAnalysis {
            simulation_id: simulation_id.to_string(),
            simulation_type: result.simulation_type.clone(),
            duration: result.end_time - result.start_time,
            metrics_summary: MetricsSummary {
                total_metrics,
                average_value: avg_metric_value,
                key_metrics: result.metrics.clone(),
            },
            insights: key_insights,
            recommendations: result.recommendations.clone(),
        })
    }

    /// Convert SimulationConfig to SimulationParams
    fn config_to_params(&self, config: SimulationConfig) -> SimulationParams {
        SimulationParams {
            duration_hours: config.duration_hours,
            time_step_minutes: config.time_step_minutes,
            scenarios: config.scenarios.into_iter().map(|sc| {
                SimulationScenario {
                    name: sc.name,
                    conditions: sc.conditions,
                    start_hour: sc.start_hour,
                    end_hour: sc.end_hour,
                }
            }).collect(),
            variables: config.parameters,
        }
    }

    /// Generate insights from simulation results
    fn generate_insights(&self, result: &SimulationResult) -> Vec<String> {
        let mut insights = Vec::new();

        // Analyze metrics for insights
        for (key, value) in &result.metrics {
            match key.as_str() {
                "total_energy_saved_kwh" => {
                    if *value > 100.0 {
                        insights.push(format!(
                            "Significant energy savings of {:.1} kWh achieved",
                            value
                        ));
                    }
                }
                "max_failure_probability" => {
                    if *value > 0.05 {
                        insights.push("High failure risk detected".to_string());
                    } else if *value < 0.01 {
                        insights.push("System reliability is excellent".to_string());
                    }
                }
                "peak_demand_kw" => {
                    if *value > 1000.0 {
                        insights.push(format!(
                            "Peak demand of {:.0} kW may require demand response measures",
                            value
                        ));
                    }
                }
                _ => {}
            }
        }

        if insights.is_empty() {
            insights.push("Simulation completed successfully".to_string());
        }

        insights
    }
}

/// Simulation analysis results
#[derive(Debug, Clone)]
pub struct SimulationAnalysis {
    pub simulation_id: String,
    pub simulation_type: String,
    pub duration: chrono::Duration,
    pub metrics_summary: MetricsSummary,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Metrics summary
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub total_metrics: usize,
    pub average_value: f64,
    pub key_metrics: HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::models::digital_twin::{DigitalTwin, TwinStatus, TwinMetadata};
    use crate::core::domain::models::sensor_data::SensorData;
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
            Ok(self.twins.lock().unwrap().clone())
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
    async fn test_simulation_service() {
        let twin_id = TwinId::new();
        
        let mut simulation_results = HashMap::new();
        simulation_results.insert(
            "sim_test_123".to_string(),
            SimulationResult {
                simulation_type: "test".to_string(),
                start_time: Utc::now() - chrono::Duration::hours(1),
                end_time: Utc::now(),
                status: "completed".to_string(),
                metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("test_metric".to_string(), 42.0);
                    metrics
                },
                recommendations: vec!["Test recommendation".to_string()],
            }
        );
        
        let twin = DigitalTwin::new(
            twin_id.clone(),
            "Test Twin".to_string(),
            None,
            "test_type".to_string(),
            HashMap::new(),
            HashMap::new(),
            TwinStatus::Active,
            simulation_results,
            TwinMetadata::default(),
            Utc::now(),
            Utc::now(),
        );
        
        let twins = Arc::new(Mutex::new(vec![twin]));
        
        let twin_repo = Arc::new(MockDigitalTwinRepository { twins });
        let sensor_repo = Arc::new(MockSensorDataRepository {});
        let run_simulation_use_case = RunSimulationUseCase::new(twin_repo.clone(), sensor_repo.clone());
        
        let service = SimulationService::new(
            run_simulation_use_case,
            twin_repo,
            sensor_repo,
        );
        
        // Test get simulation history
        let history = service.get_simulation_history(&twin_id).await;
        assert!(history.is_ok());
        assert_eq!(history.unwrap().len(), 1);
        
        // Test analyze results
        let analysis = service.analyze_simulation_results(&twin_id, "sim_test_123").await;
        assert!(analysis.is_ok());
        let analysis_result = analysis.unwrap();
        assert_eq!(analysis_result.simulation_type, "test");
        assert_eq!(analysis_result.metrics_summary.total_metrics, 1);
    }
}