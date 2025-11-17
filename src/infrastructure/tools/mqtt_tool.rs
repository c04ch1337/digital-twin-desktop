use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use chrono::Utc;
use tracing::{debug, error, info};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::core::domain::{
    models::{Tool, ToolResult, ExecutionStatus, ExecutionMetrics},
    traits::tool_executor::{
        ToolExecutor, ExecutorResult, ExecutorError, ExecutionRequest,
        ExecutionContext, ValidationResult, ValidationError,
    },
};

/// MQTT messaging tool executor
pub struct MqttToolExecutor {
    client: Arc<Mutex<Option<AsyncClient>>>,
    broker_url: String,
    broker_port: u16,
    client_id: String,
    username: Option<String>,
    password: Option<String>,
    timeout: Duration,
}

impl MqttToolExecutor {
    /// Create a new MQTT tool executor
    pub fn new(
        broker_url: String,
        broker_port: u16,
        client_id: String,
        username: Option<String>,
        password: Option<String>,
        timeout: Duration,
    ) -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
            broker_url,
            broker_port,
            client_id,
            username,
            password,
            timeout,
        }
    }

    /// Connect to MQTT broker
    async fn connect(&self) -> ExecutorResult<()> {
        let mut mqtt_options = MqttOptions::new(
            &self.client_id,
            &self.broker_url,
            self.broker_port,
        );

        mqtt_options
            .set_keep_alive(self.timeout)
            .set_clean_session(true);

        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            mqtt_options.set_credentials(username, password);
        }

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
        
        // Start event loop in background
        let client_clone = client.clone();
        tokio::spawn(async move {
            while let Ok(notification) = eventloop.poll().await {
                debug!("MQTT Event: {:?}", notification);
            }
            debug!("MQTT event loop terminated");
        });

        // Store client
        let mut client_guard = self.client.lock().await;
        *client_guard = Some(client);

        Ok(())
    }

    /// Ensure client is connected
    async fn ensure_connected(&self) -> ExecutorResult<AsyncClient> {
        let mut client_guard = self.client.lock().await;
        
        if client_guard.is_none() {
            drop(client_guard);
            self.connect().await?;
            client_guard = self.client.lock().await;
        }

        Ok(client_guard.as_ref().unwrap().clone())
    }

    /// Publish message
    async fn publish(
        &self,
        topic: &str,
        payload: &[u8],
        qos: QoS,
        retain: bool,
    ) -> ExecutorResult<()> {
        let client = self.ensure_connected().await?;

        client.publish(topic, qos, retain, payload)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(())
    }

    /// Subscribe to topic
    async fn subscribe(
        &self,
        topic: &str,
        qos: QoS,
    ) -> ExecutorResult<()> {
        let client = self.ensure_connected().await?;

        client.subscribe(topic, qos)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(())
    }

    /// Unsubscribe from topic
    async fn unsubscribe(&self, topic: &str) -> ExecutorResult<()> {
        let client = self.ensure_connected().await?;

        client.unsubscribe(topic)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl ToolExecutor for MqttToolExecutor {
    async fn execute(&self, request: ExecutionRequest) -> ExecutorResult<ToolResult> {
        let start_time = Utc::now();
        let mut metrics = ExecutionMetrics::default();

        // Get operation type
        let operation = request.parameters.get("operation")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "mqtt".to_string(),
                parameter: "operation".to_string(),
            })?;

        // Execute operation
        let result = match operation.to_lowercase().as_str() {
            "publish" => {
                let topic = request.parameters.get("topic")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "mqtt".to_string(),
                        parameter: "topic".to_string(),
                    })?;

                let payload = request.parameters.get("payload")
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "mqtt".to_string(),
                        parameter: "payload".to_string(),
                    })?;

                let payload_bytes = if payload.is_string() {
                    payload.as_str().unwrap().as_bytes().to_vec()
                } else {
                    serde_json::to_vec(payload)
                        .map_err(|e| ExecutorError::ValidationError {
                            parameter: "payload".to_string(),
                            reason: e.to_string(),
                        })?
                };

                let qos = match request.parameters.get("qos").and_then(Value::as_u64) {
                    Some(0) => QoS::AtMostOnce,
                    Some(1) => QoS::AtLeastOnce,
                    Some(2) => QoS::ExactlyOnce,
                    _ => QoS::AtLeastOnce,
                };

                let retain = request.parameters.get("retain")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);

                self.publish(topic, &payload_bytes, qos, retain).await?;
                metrics.bytes_written = payload_bytes.len() as u64;

                serde_json::json!({
                    "success": true,
                })
            },
            "subscribe" => {
                let topic = request.parameters.get("topic")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "mqtt".to_string(),
                        parameter: "topic".to_string(),
                    })?;

                let qos = match request.parameters.get("qos").and_then(Value::as_u64) {
                    Some(0) => QoS::AtMostOnce,
                    Some(1) => QoS::AtLeastOnce,
                    Some(2) => QoS::ExactlyOnce,
                    _ => QoS::AtLeastOnce,
                };

                self.subscribe(topic, qos).await?;

                serde_json::json!({
                    "success": true,
                })
            },
            "unsubscribe" => {
                let topic = request.parameters.get("topic")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "mqtt".to_string(),
                        parameter: "topic".to_string(),
                    })?;

                self.unsubscribe(topic).await?;

                serde_json::json!({
                    "success": true,
                })
            },
            _ => return Err(ExecutorError::ValidationError {
                parameter: "operation".to_string(),
                reason: format!("Invalid operation: {}", operation),
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

        // Validate operation
        match parameters.get("operation").and_then(Value::as_str) {
            Some(operation) => {
                if !["publish", "subscribe", "unsubscribe"].contains(&operation.to_lowercase().as_str()) {
                    errors.push(ValidationError {
                        parameter: "operation".to_string(),
                        code: "INVALID_OPERATION".to_string(),
                        message: format!("Invalid operation: {}", operation),
                        expected: Some("publish, subscribe, or unsubscribe".to_string()),
                        actual: Some(operation.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "operation".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Operation parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate topic
        if parameters.get("topic").and_then(Value::as_str).is_none() {
            errors.push(ValidationError {
                parameter: "topic".to_string(),
                code: "MISSING_PARAMETER".to_string(),
                message: "Topic parameter is required".to_string(),
                expected: None,
                actual: None,
            });
        }

        // Validate QoS
        if let Some(qos) = parameters.get("qos").and_then(Value::as_u64) {
            if qos > 2 {
                errors.push(ValidationError {
                    parameter: "qos".to_string(),
                    code: "INVALID_VALUE".to_string(),
                    message: "QoS must be 0, 1, or 2".to_string(),
                    expected: Some("0, 1, or 2".to_string()),
                    actual: Some(qos.to_string()),
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
        if !context.security.permissions.contains(&"mqtt:publish".to_string()) &&
           !context.security.permissions.contains(&"mqtt:subscribe".to_string()) {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mqtt_validation() {
        let executor = MqttToolExecutor::new(
            "localhost".to_string(),
            1883,
            "test-client".to_string(),
            None,
            None,
            Duration::from_secs(30),
        );

        let result = executor.validate_parameters(
            Uuid::new_v4(),
            &serde_json::json!({
                "operation": "publish",
                "topic": "test/topic",
                "payload": "test message",
                "qos": 1,
                "retain": false,
            })
            .as_object()
            .unwrap()
            .clone(),
        ).await.unwrap();

        assert!(result.valid);
        assert!(result.errors.is_empty());
    }
}