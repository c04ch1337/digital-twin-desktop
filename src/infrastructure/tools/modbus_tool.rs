use async_trait::async_trait;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use std::time::Duration;
use chrono::Utc;
use tracing::{debug, error, info};

use crate::core::domain::{
    models::{Tool, ToolResult, ExecutionStatus, ExecutionMetrics},
    traits::tool_executor::{
        ToolExecutor, ExecutorResult, ExecutorError, ExecutionRequest,
        ExecutionContext, ValidationResult, ValidationError,
    },
};

/// Modbus protocol tool executor
pub struct ModbusToolExecutor {
    timeout: Duration,
    max_retries: u32,
}

impl ModbusToolExecutor {
    /// Create a new Modbus tool executor
    pub fn new(timeout: Duration, max_retries: u32) -> Self {
        Self {
            timeout,
            max_retries,
        }
    }

    /// Create Modbus TCP client
    async fn create_tcp_client(
        &self,
        host: &str,
        port: u16,
    ) -> ExecutorResult<client::Context> {
        let socket_addr = format!("{}:{}", host, port)
            .parse()
            .map_err(|e| ExecutorError::ValidationError {
                parameter: "host".to_string(),
                reason: e.to_string(),
            })?;

        let ctx = tcp::connect(socket_addr)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(ctx)
    }

    /// Create Modbus RTU client
    async fn create_rtu_client(
        &self,
        port: &str,
        baud_rate: u32,
    ) -> ExecutorResult<client::Context> {
        let builder = tokio_serial::new(port, baud_rate);
        let serial = SerialStream::open(&builder)
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        let ctx = rtu::connect_slave(serial, Slave(1))
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;

        Ok(ctx)
    }

    /// Read holding registers
    async fn read_holding_registers(
        &self,
        ctx: &mut client::Context,
        address: u16,
        quantity: u16,
    ) -> ExecutorResult<Vec<u16>> {
        let mut retries = 0;
        loop {
            match ctx.read_holding_registers(address, quantity).await {
                Ok(values) => return Ok(values),
                Err(e) => {
                    if retries >= self.max_retries {
                        return Err(ExecutorError::ExecutionFailed(e.to_string()));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    /// Write holding registers
    async fn write_holding_registers(
        &self,
        ctx: &mut client::Context,
        address: u16,
        values: &[u16],
    ) -> ExecutorResult<()> {
        let mut retries = 0;
        loop {
            match ctx.write_multiple_registers(address, values).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries >= self.max_retries {
                        return Err(ExecutorError::ExecutionFailed(e.to_string()));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
}

#[async_trait]
impl ToolExecutor for ModbusToolExecutor {
    async fn execute(&self, request: ExecutionRequest) -> ExecutorResult<ToolResult> {
        let start_time = Utc::now();
        let mut metrics = ExecutionMetrics::default();

        // Get transport type
        let transport = request.parameters.get("transport")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "modbus".to_string(),
                parameter: "transport".to_string(),
            })?;

        // Create client based on transport
        let mut ctx = match transport.to_lowercase().as_str() {
            "tcp" => {
                let host = request.parameters.get("host")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "modbus".to_string(),
                        parameter: "host".to_string(),
                    })?;

                let port = request.parameters.get("port")
                    .and_then(Value::as_u64)
                    .unwrap_or(502) as u16;

                self.create_tcp_client(host, port).await?
            },
            "rtu" => {
                let port = request.parameters.get("port")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "modbus".to_string(),
                        parameter: "port".to_string(),
                    })?;

                let baud_rate = request.parameters.get("baud_rate")
                    .and_then(Value::as_u64)
                    .unwrap_or(9600) as u32;

                self.create_rtu_client(port, baud_rate).await?
            },
            _ => return Err(ExecutorError::ValidationError {
                parameter: "transport".to_string(),
                reason: format!("Invalid transport type: {}", transport),
            }),
        };

        // Get operation type
        let operation = request.parameters.get("operation")
            .and_then(Value::as_str)
            .ok_or_else(|| ExecutorError::MissingParameter {
                tool: "modbus".to_string(),
                parameter: "operation".to_string(),
            })?;

        // Execute operation
        let result = match operation.to_lowercase().as_str() {
            "read" => {
                let address = request.parameters.get("address")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "modbus".to_string(),
                        parameter: "address".to_string(),
                    })? as u16;

                let quantity = request.parameters.get("quantity")
                    .and_then(Value::as_u64)
                    .unwrap_or(1) as u16;

                let values = self.read_holding_registers(&mut ctx, address, quantity).await?;
                metrics.bytes_read = (values.len() * 2) as u64;

                serde_json::json!({
                    "values": values,
                })
            },
            "write" => {
                let address = request.parameters.get("address")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "modbus".to_string(),
                        parameter: "address".to_string(),
                    })? as u16;

                let values = request.parameters.get("values")
                    .and_then(Value::as_array)
                    .ok_or_else(|| ExecutorError::MissingParameter {
                        tool: "modbus".to_string(),
                        parameter: "values".to_string(),
                    })?
                    .iter()
                    .map(|v| v.as_u64().ok_or_else(|| ExecutorError::ValidationError {
                        parameter: "values".to_string(),
                        reason: "Values must be unsigned integers".to_string(),
                    }))
                    .collect::<Result<Vec<u64>, _>>()?
                    .into_iter()
                    .map(|v| v as u16)
                    .collect::<Vec<u16>>();

                self.write_holding_registers(&mut ctx, address, &values).await?;
                metrics.bytes_written = (values.len() * 2) as u64;

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

        // Validate transport
        match parameters.get("transport").and_then(Value::as_str) {
            Some(transport) => {
                if !["tcp", "rtu"].contains(&transport.to_lowercase().as_str()) {
                    errors.push(ValidationError {
                        parameter: "transport".to_string(),
                        code: "INVALID_TRANSPORT".to_string(),
                        message: format!("Invalid transport type: {}", transport),
                        expected: Some("tcp or rtu".to_string()),
                        actual: Some(transport.to_string()),
                    });
                }
            },
            None => {
                errors.push(ValidationError {
                    parameter: "transport".to_string(),
                    code: "MISSING_PARAMETER".to_string(),
                    message: "Transport parameter is required".to_string(),
                    expected: None,
                    actual: None,
                });
            }
        }

        // Validate operation
        match parameters.get("operation").and_then(Value::as_str) {
            Some(operation) => {
                if !["read", "write"].contains(&operation.to_lowercase().as_str()) {
                    errors.push(ValidationError {
                        parameter: "operation".to_string(),
                        code: "INVALID_OPERATION".to_string(),
                        message: format!("Invalid operation: {}", operation),
                        expected: Some("read or write".to_string()),
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
        if !context.security.permissions.contains(&"modbus:read".to_string()) &&
           !context.security.permissions.contains(&"modbus:write".to_string()) {
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
    async fn test_modbus_validation() {
        let executor = ModbusToolExecutor::new(
            Duration::from_secs(1),
            3,
        );

        let result = executor.validate_parameters(
            Uuid::new_v4(),
            &serde_json::json!({
                "transport": "tcp",
                "operation": "read",
                "host": "localhost",
                "port": 502,
                "address": 0,
                "quantity": 10,
            })
            .as_object()
            .unwrap()
            .clone(),
        ).await.unwrap();

        assert!(result.valid);
        assert!(result.errors.is_empty());
    }
}