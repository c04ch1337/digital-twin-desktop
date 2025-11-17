use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::core::domain::{
    models::{Tool, ToolResult, ExecutionStatus, ExecutionMetrics, TwinId},
    traits::{
        repository::{TwinRepository, SensorDataRepository},
        tool_executor::{
            ToolExecutor, ExecutorResult, ExecutorError, ExecutionRequest,
            ExecutionContext, ValidationResult, ValidationError,
        },
    },
};

/// Digital twin query tool executor
pub struct TwinToolExecutor {
    twin_repo: Arc<dyn TwinRepository>,
    sensor_repo: Arc<dyn SensorDataRepository>,
}

impl TwinToolExecutor {
    /// Create a new digital twin tool executor
    pub fn new(
        twin_repo: Arc<dyn TwinRepository>,
        sensor_repo: Arc<dyn SensorDataRepository>,
    ) -> Self {
        Self {
            twin_repo,
            sensor_repo,
        }
    }

    /// Get twin properties
    async fn get_properties(&self, twin_id: TwinId) -> ExecutorResult<Value> {
        let twin = self.twin_repo.get_by_id(twin_id)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(serde_json::to_value(twin.properties)
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?)
    }

    /// Get twin sensor data
    async fn get_sensor_data(
        &self,
        twin_id: TwinId,
        sensor_type: Option<String>,
    ) -> ExecutorResult<Value> {
        let sensors = self.sensor_repo.get_by_twin_id(
            twin_id,
            Default::default(),
        )
        .await
        .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        let mut result = Vec::new();
        for sensor in sensors.items {
            if let Some(ref type_filter) = sensor_type {
                if sensor.sensor_type != *type_filter {
                    continue;
                }
            }

            if let Some(reading) = self.sensor_repo.get_latest_reading(sensor.id)
                .await
                .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))? {
                result.push(serde_json::json!({
                    "sensor_type": sensor.sensor_type,
                    "unit": sensor.unit,
                    "value": reading.value,
                    "timestamp": reading.timestamp,
                }));
            }
        }

        Ok(serde_json::json!(result))
    }

    /// Get twin state
    async fn get_state(&self, twin_id: TwinId) -> ExecutorResult<Value> {
        let twin = self.twin_repo.get_by_id(twin_id)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(serde_json::json!({
            "id": twin.id,
            "name": twin.name,
            "type": twin.twin_type,
            "state": twin.state,
            "agent_id": twin.agent_id,
            "last_sync": twin.last_sync,
        }))
    }
}

#[async_trait]
impl ToolExecutor for TwinToolExecutor {
    async fn execute(&self, request: ExecutionRequest) -> ExecutorResult<ToolResult> {
        let start_time = Utc::now();
        let mut metrics = ExecutionMetrics::default();

        // Get twin ID
        let twin_id = request.parameters.get("twin_id")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "twin".to_string(),
                parameter: "twin_id".to_string(),
            })?;

        let twin_id = TwinId::parse_str(twin_id)
            .map_err(|e| ExecutorError::ValidationError {
                parameter: "twin_id".to_string(),
                reason: e.to_string(),
            })?;

        // Get query type
        let query = request.parameters.get("query")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "twin".to_string(),
                parameter: "query".to_string(),
            })?;

        // Execute query
        let result = match query.to_lowercase().as_str() {
            "properties" => {
                self.get_properties(twin_id).await?
            },
            "sensor_data" => {
                let sensor_type = request.parameters.get("sensor_type")
                    .and_then(Value::as_str)
                    .map(String::from);

                self.get_sensor_data(twin_id, sensor_type).await?
            },
            "state" => {
                self.get_state(twin_id).await?
            },
            _ => return Err(ExecutorError::ValidationError {
                parameter: "query".to_string(),
                reason: format!("Invalid query type: {}", query),
            }),
        };

        let end_time = Utc::now();
        metrics.execution_time_ms = (end_time - start_time).num_milliseconds() as u64;

        Ok(ToolResult {
            execution_id: request.execution_id,
            tool_id: request.tool_id,
            status: ExecutionStatus::Completed,
            parameters: request.parameters,
            output: Some(result),
            started_at: start_time,
            completed_at: Some(end_time),
            error: None,
            metrics,
        })
    }

    async fn validate_parameters(
        &self,
        tool_id: ToolId,
        parameters: &HashMap<String, Value>,
    ) -> ExecutorResult<ValidationResult> {
        let mut errors = Vec::new();

        // Validate twin ID
        match parameters.get("twin_id").and_then(Value::as_str) {
            Some(twin_id) => {
                if TwinId::parse_str(twin_id).is_err() {
                    errors.push(ValidationError {
                        parameter: "twin_id".to_string(),
                        code: "INVALID_UUID".to_string(),
                        message: "Twin ID must be a valid UUID".to_string(),
                        expected: None,
                        actual: Some(twin_id.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "twin_id".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Twin ID parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate query type
        match parameters.get("query").and_then(Value::as_str) {
            Some(query) => {
                if !["properties", "sensor_data", "state"].contains(&query.to_lowercase().as_str()) {
                    errors.push(ValidationError {
                        parameter: "query".to_string(),
                        code: "INVALID_QUERY".to_string(),
                        message: format!("Invalid query type: {}", query),
                        expected: Some("properties, sensor_data, or state".to_string()),
                        actual: Some(query.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "query".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Query parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
            normalized_parameters: None,
        })
    }

    async fn can_execute(
        &self,
        tool_id: ToolId,
        context: &ExecutionContext,
    ) -> ExecutorResult<bool> {
        // Check if context has required permissions
        if !context.security.permissions.contains(&"twin:read".to_string()) {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    use uuid::Uuid;

    mock! {
        TwinRepo {}
        #[async_trait]
        impl TwinRepository for TwinRepo {
            async fn get_by_id(&self, id: TwinId) -> RepositoryResult<DigitalTwin>;
            // ... other required methods ...
        }
    }

    mock! {
        SensorRepo {}
        #[async_trait]
        impl SensorDataRepository for SensorRepo {
            async fn get_by_twin_id(&self, twin_id: TwinId, pagination: Pagination) -> RepositoryResult<PaginatedResult<SensorData>>;
            async fn get_latest_reading(&self, sensor_id: SensorDataId) -> RepositoryResult<Option<SensorReading>>;
            // ... other required methods ...
        }
    }

    #[tokio::test]
    async fn test_twin_tool() {
        let twin_id = Uuid::new_v4();
        let mut twin_repo = MockTwinRepo::new();
        let mut sensor_repo = MockSensorRepo::new();

        twin_repo.expect_get_by_id()
            .with(eq(twin_id))
            .returning(|_| Ok(DigitalTwin {
                id: twin_id,
                name: "Test Twin".to_string(),
                description: "Test Description".to_string(),
                twin_type: TwinType::Device,
                state: TwinState::Active,
                properties: HashMap::new(),
                agent_id: None,
                last_sync: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: Default::default(),
            }));

        let executor = TwinToolExecutor::new(
            Arc::new(twin_repo),
            Arc::new(sensor_repo),
        );

        let result = executor.validate_parameters(
            Uuid::new_v4(),
            &serde_json::json!({
                "twin_id": twin_id.to_string(),
                "query": "state",
            })
            .as_object()
            .unwrap()
            .clone(),
        ).await.unwrap();

        assert!(result.valid);
        assert!(result.errors.is_empty());
    }
}