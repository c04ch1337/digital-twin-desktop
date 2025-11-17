# Tool Development Guide

## Overview

The Digital Twin Desktop application features an extensible tool system that allows agents to interact with digital twins, external systems, and perform various operations. This guide explains how to develop new tools for the system.

## Tool Architecture

Tools in the Digital Twin Desktop follow a plugin-like architecture:

1. **Tool Interface**: All tools implement the `Tool` trait
2. **Tool Registry**: Tools are registered in a central registry
3. **Tool Executor**: Tools are executed with proper sandboxing and error handling
4. **Tool Descriptions**: Tools provide descriptions for the LLM to understand their capabilities

## The Tool Trait

All tools must implement the `Tool` trait:

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the name of the tool
    fn name(&self) -> &str;
    
    /// Returns a description of what the tool does
    fn description(&self) -> &str;
    
    /// Returns a JSON schema describing the tool's parameters (optional)
    fn parameter_schema(&self) -> Option<serde_json::Value>;
    
    /// Executes the tool with the given input
    async fn execute(&self, input: &str) -> Result<String, ToolError>;
}
```

## Creating a New Tool

### Step 1: Define the Tool Structure

Start by defining a struct for your tool:

```rust
pub struct CalculatorTool;

// Or with configuration
pub struct WebSearchTool {
    api_key: String,
    max_results: usize,
}
```

### Step 2: Implement the Tool Trait

Implement the `Tool` trait for your struct:

```rust
#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "Evaluates mathematical expressions. Input should be a valid mathematical expression like '2 + 2', '5 * 3', or 'sqrt(16)'."
    }
    
    fn parameter_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        }))
    }
    
    async fn execute(&self, input: &str) -> Result<String, ToolError> {
        // Parse the input as JSON
        let parsed: serde_json::Value = serde_json::from_str(input)
            .map_err(|e| ToolError::InvalidInput(format!("Invalid JSON: {}", e)))?;
        
        // Extract the expression
        let expression = parsed["expression"].as_str()
            .ok_or_else(|| ToolError::InvalidInput("Missing 'expression' field".to_string()))?;
        
        // Evaluate the expression (using a safe expression evaluator)
        let result = evaluate_expression(expression)
            .map_err(|e| ToolError::ExecutionError(format!("Evaluation error: {}", e)))?;
        
        // Return the result
        Ok(result.to_string())
    }
}
```

### Step 3: Register the Tool

Register your tool with the `ToolRegistry`:

```rust
let mut registry = ToolRegistry::new();
registry.register(CalculatorTool);
registry.register(WebSearchTool::new("api-key", 5));
```

## Tool Input/Output Format

### Input Format

Tools receive input as a JSON string. The format should be documented in the parameter schema:

```json
{
  "expression": "2 + 2"
}
```

### Output Format

Tools should return their output as a string. The format should be documented in the tool description:

```
4
```

For complex outputs, consider using a structured format like JSON:

```json
{
  "result": 4,
  "precision": "exact",
  "calculation_time_ms": 0.5
}
```

## Tool Categories

The Digital Twin Desktop supports several categories of tools:

### 1. Digital Twin Tools

Tools for interacting with digital twins:

- **TwinQueryTool**: Query the state of a digital twin
- **TwinUpdateTool**: Update the configuration of a digital twin
- **SensorDataTool**: Retrieve historical sensor data
- **SimulationTool**: Run simulations on a digital twin

Example:

```rust
pub struct TwinQueryTool {
    twin_service: Arc<dyn TwinService>,
}

#[async_trait]
impl Tool for TwinQueryTool {
    fn name(&self) -> &str {
        "twin_query"
    }
    
    fn description(&self) -> &str {
        "Queries the current state of a digital twin. Provide the twin ID and optionally specific sensor IDs to query."
    }
    
    async fn execute(&self, input: &str) -> Result<String, ToolError> {
        let parsed: serde_json::Value = serde_json::from_str(input)?;
        
        let twin_id = parsed["twin_id"].as_str()
            .ok_or_else(|| ToolError::InvalidInput("Missing 'twin_id' field".to_string()))?;
        
        let twin = self.twin_service.get_twin(twin_id).await
            .map_err(|e| ToolError::ExecutionError(format!("Failed to get twin: {}", e)))?;
        
        // Format the twin state as JSON
        let result = serde_json::to_string_pretty(&twin)?;
        
        Ok(result)
    }
}
```

### 2. External Integration Tools

Tools for interacting with external systems:

- **ModbusTool**: Communicate with industrial devices via Modbus
- **MqttTool**: Publish/subscribe to MQTT topics
- **WebApiTool**: Make HTTP requests to web APIs
- **DatabaseTool**: Query external databases

Example:

```rust
pub struct ModbusTool {
    client: ModbusClient,
    timeout: Duration,
}

#[async_trait]
impl Tool for ModbusTool {
    fn name(&self) -> &str {
        "modbus"
    }
    
    fn description(&self) -> &str {
        "Reads or writes Modbus registers on industrial devices."
    }
    
    async fn execute(&self, input: &str) -> Result<String, ToolError> {
        let parsed: serde_json::Value = serde_json::from_str(input)?;
        
        let operation = parsed["operation"].as_str()
            .ok_or_else(|| ToolError::InvalidInput("Missing 'operation' field".to_string()))?;
        
        let address = parsed["address"].as_u64()
            .ok_or_else(|| ToolError::InvalidInput("Missing or invalid 'address' field".to_string()))?;
        
        match operation {
            "read" => {
                let count = parsed["count"].as_u64().unwrap_or(1) as u16;
                
                let values = tokio::time::timeout(
                    self.timeout,
                    self.client.read_holding_registers(address as u16, count)
                ).await??;
                
                Ok(serde_json::to_string(&values)?)
            },
            "write" => {
                let value = parsed["value"].as_u64()
                    .ok_or_else(|| ToolError::InvalidInput("Missing 'value' field for write operation".to_string()))?;
                
                tokio::time::timeout(
                    self.timeout,
                    self.client.write_single_register(address as u16, value as u16)
                ).await??;
                
                Ok("Write successful".to_string())
            },
            _ => Err(ToolError::InvalidInput(format!("Unknown operation: {}", operation))),
        }
    }
}
```

### 3. Utility Tools

General-purpose utility tools:

- **CalculatorTool**: Evaluate mathematical expressions
- **WebSearchTool**: Search the web for information
- **DateTimeTool**: Get current date/time information
- **FileOperationsTool**: Read/write files

Example:

```rust
pub struct DateTimeTool;

#[async_trait]
impl Tool for DateTimeTool {
    fn name(&self) -> &str {
        "datetime"
    }
    
    fn description(&self) -> &str {
        "Gets current date and time information, or performs date/time calculations."
    }
    
    async fn execute(&self, input: &str) -> Result<String, ToolError> {
        let parsed: serde_json::Value = serde_json::from_str(input)?;
        
        let operation = parsed["operation"].as_str()
            .unwrap_or("now");
        
        match operation {
            "now" => {
                let now = chrono::Utc::now();
                
                let result = serde_json::json!({
                    "iso8601": now.to_rfc3339(),
                    "unix_timestamp": now.timestamp(),
                    "date": {
                        "year": now.year(),
                        "month": now.month(),
                        "day": now.day(),
                    },
                    "time": {
                        "hour": now.hour(),
                        "minute": now.minute(),
                        "second": now.second(),
                    }
                });
                
                Ok(serde_json::to_string_pretty(&result)?)
            },
            // Other operations like "add", "subtract", "format", etc.
            _ => Err(ToolError::InvalidInput(format!("Unknown operation: {}", operation))),
        }
    }
}
```

## Tool Safety

When developing tools, consider these safety measures:

### 1. Input Validation

Always validate tool inputs:

```rust
fn validate_input(&self, input: &serde_json::Value) -> Result<(), ToolError> {
    // Check required fields
    if !input.is_object() {
        return Err(ToolError::InvalidInput("Input must be a JSON object".to_string()));
    }
    
    if !input.get("required_field").is_some() {
        return Err(ToolError::InvalidInput("Missing 'required_field'".to_string()));
    }
    
    // Validate field types
    if let Some(value) = input.get("numeric_field") {
        if !value.is_number() {
            return Err(ToolError::InvalidInput("'numeric_field' must be a number".to_string()));
        }
    }
    
    // Validate value ranges
    if let Some(value) = input.get("percentage") {
        if let Some(num) = value.as_f64() {
            if num < 0.0 || num > 100.0 {
                return Err(ToolError::InvalidInput("'percentage' must be between 0 and 100".to_string()));
            }
        }
    }
    
    Ok(())
}
```

### 2. Timeouts

Implement timeouts for operations that might hang:

```rust
async fn execute_with_timeout<F, T, E>(&self, operation: F, timeout_duration: Duration) -> Result<T, ToolError>
where
    F: Future<Output = Result<T, E>>,
    E: Into<ToolError>,
{
    match tokio::time::timeout(timeout_duration, operation).await {
        Ok(result) => result.map_err(|e| e.into()),
        Err(_) => Err(ToolError::Timeout("Operation timed out".to_string())),
    }
}
```

### 3. Resource Limits

Limit resource usage:

```rust
fn limit_resource_usage(&self, input: &serde_json::Value) -> Result<(), ToolError> {
    // Limit batch size
    if let Some(items) = input.get("items").and_then(|v| v.as_array()) {
        if items.len() > self.max_batch_size {
            return Err(ToolError::ResourceLimit(
                format!("Batch size exceeds maximum of {}", self.max_batch_size)
            ));
        }
    }
    
    // Limit query complexity
    if let Some(query) = input.get("query").and_then(|v| v.as_str()) {
        if query.len() > self.max_query_length {
            return Err(ToolError::ResourceLimit(
                format!("Query length exceeds maximum of {}", self.max_query_length)
            ));
        }
    }
    
    Ok(())
}
```

### 4. Error Handling

Implement comprehensive error handling:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

## Testing Tools

### Unit Testing

Test tools in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_calculator_tool_addition() {
        let tool = CalculatorTool;
        
        let input = r#"{"expression": "2 + 2"}"#;
        let result = tool.execute(input).await.unwrap();
        
        assert_eq!(result, "4");
    }
    
    #[tokio::test]
    async fn test_calculator_tool_invalid_input() {
        let tool = CalculatorTool;
        
        let input = r#"{"wrong_field": "2 + 2"}"#;
        let result = tool.execute(input).await;
        
        assert!(result.is_err());
        match result {
            Err(ToolError::InvalidInput(_)) => (),
            _ => panic!("Expected InvalidInput error"),
        }
    }
}
```

### Integration Testing

Test tools with their dependencies:

```rust
#[tokio::test]
async fn test_twin_query_tool_integration() {
    // Setup
    let db_pool = create_test_db_pool().await;
    let twin_repo = TwinRepositorySqlite::new(db_pool.clone());
    let twin_service = TwinService::new(twin_repo);
    
    // Create a test twin
    let mut twin = DigitalTwin::new("test-twin", "Test Twin");
    twin.add_sensor("temp", "Temperature", "°C", 20.0).unwrap();
    twin_service.save_twin(&twin).await.unwrap();
    
    // Create the tool
    let tool = TwinQueryTool::new(Arc::new(twin_service));
    
    // Execute the tool
    let input = r#"{"twin_id": "test-twin"}"#;
    let result = tool.execute(input).await.unwrap();
    
    // Verify
    let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(result_json["id"], "test-twin");
    assert_eq!(result_json["name"], "Test Twin");
    assert!(result_json["sensors"].as_array().unwrap().len() > 0);
    
    // Cleanup
    cleanup_test_db(db_pool).await;
}
```

### Mocking External Dependencies

Use mocks for external dependencies:

```rust
struct MockModbusClient {
    read_responses: HashMap<u16, Vec<u16>>,
    write_responses: HashMap<u16, Result<(), String>>,
}

impl MockModbusClient {
    fn new() -> Self {
        Self {
            read_responses: HashMap::new(),
            write_responses: HashMap::new(),
        }
    }
    
    fn with_read_response(mut self, address: u16, values: Vec<u16>) -> Self {
        self.read_responses.insert(address, values);
        self
    }
    
    fn with_write_response(mut self, address: u16, result: Result<(), String>) -> Self {
        self.write_responses.insert(address, result);
        self
    }
}

#[async_trait]
impl ModbusClient for MockModbusClient {
    async fn read_holding_registers(&self, address: u16, count: u16) -> Result<Vec<u16>, ModbusError> {
        if let Some(values) = self.read_responses.get(&address) {
            Ok(values.clone())
        } else {
            Err(ModbusError::NotFound)
        }
    }
    
    async fn write_single_register(&self, address: u16, value: u16) -> Result<(), ModbusError> {
        if let Some(result) = self.write_responses.get(&address) {
            result.clone().map_err(|e| ModbusError::Other(e))
        } else {
            Err(ModbusError::NotFound)
        }
    }
}
```

## Best Practices

### 1. Clear Descriptions

Write clear, detailed descriptions for your tools:

```rust
fn description(&self) -> &str {
    "Searches the web for information on a given query. \
    Provide a search query and optionally the number of results to return. \
    Returns a list of search results with titles, URLs, and snippets."
}
```

### 2. Structured Parameter Schemas

Provide detailed parameter schemas:

```rust
fn parameter_schema(&self) -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "The search query"
            },
            "num_results": {
                "type": "integer",
                "description": "Number of results to return",
                "default": 3,
                "minimum": 1,
                "maximum": 10
            }
        },
        "required": ["query"]
    }))
}
```

### 3. Informative Error Messages

Provide informative error messages:

```rust
Err(ToolError::InvalidInput(format!(
    "Invalid value for 'temperature': {}. Must be between -50 and 150.",
    temperature
)))
```

### 4. Stateless Design

Design tools to be stateless when possible:

```rust
// Good: Stateless tool
pub struct CalculatorTool;

// When state is needed, make it explicit and thread-safe
pub struct CachingWebSearchTool {
    api_key: String,
    cache: Arc<RwLock<LruCache<String, Vec<SearchResult>>>>,
}
```

### 5. Graceful Degradation

Implement graceful degradation for external services:

```rust
async fn execute(&self, input: &str) -> Result<String, ToolError> {
    let parsed: serde_json::Value = serde_json::from_str(input)?;
    let query = parsed["query"].as_str().ok_or_else(|| ToolError::InvalidInput("Missing 'query'".to_string()))?;
    
    // Try primary API
    match self.search_primary_api(query).await {
        Ok(results) => return Ok(serde_json::to_string(&results)?),
        Err(e) => {
            tracing::warn!("Primary API failed: {}. Falling back to secondary API.", e);
            
            // Try fallback API
            match self.search_fallback_api(query).await {
                Ok(results) => return Ok(serde_json::to_string(&results)?),
                Err(fallback_err) => {
                    // Both APIs failed
                    return Err(ToolError::ExecutionError(format!(
                        "All search APIs failed. Primary: {}, Fallback: {}",
                        e, fallback_err
                    )));
                }
            }
        }
    }
}
```

## Tool Documentation

Document your tools thoroughly:

```rust
/// A tool for searching the web.
///
/// This tool uses the Brave Search API to find information on the web.
/// It supports pagination, filtering, and safe search options.
///
/// # Examples
///
/// Basic search:
/// ```json
/// {
///   "query": "rust programming language"
/// }
/// ```
///
/// Advanced search:
/// ```json
/// {
///   "query": "rust programming language",
///   "num_results": 5,
///   "safe_search": true,
///   "time_range": "past_month"
/// }
/// ```
///
/// # Parameters
///
/// - `query`: The search query (required)
/// - `num_results`: Number of results to return (default: 3, max: 10)
/// - `safe_search`: Whether to enable safe search (default: true)
/// - `time_range`: Time range for results (options: "all", "past_day", "past_week", "past_month", "past_year")
///
/// # Returns
///
/// A JSON array of search results, each with:
/// - `title`: The title of the result
/// - `url`: The URL of the result
/// - `description`: A snippet of text from the result
/// - `published_date`: When the content was published (if available)
pub struct WebSearchTool {
    // Implementation details...
}
```

## Conclusion

By following these guidelines, you can create robust, safe, and effective tools for the Digital Twin Desktop application. Tools should be focused on specific tasks, well-documented, and handle errors gracefully.