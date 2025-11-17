# Digital Twin Desktop Testing Guidelines

## Overview
This prompt provides comprehensive guidance for testing the Digital Twin Desktop application. Use this when implementing new tests, improving test coverage, or debugging test failures.

## Testing Philosophy

The Digital Twin Desktop follows these testing principles:

1. **Test Pyramid**: Prioritize unit tests, followed by integration tests, and then end-to-end tests
2. **Test Independence**: Tests should not depend on each other
3. **Test Isolation**: Tests should run in isolation and not affect the system state
4. **Test Determinism**: Tests should produce the same results on every run
5. **Test Readability**: Tests should be easy to understand and maintain

## Test Types

### Unit Tests

Unit tests verify individual components in isolation:

- Test domain models and business logic
- Test use cases with mocked dependencies
- Test utility functions and helpers

Location: `tests/unit/`

Example unit test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_digital_twin_updates_sensor_value() {
        // Arrange
        let mut twin = DigitalTwin::new("test-twin", "Test Twin");
        let sensor_id = "temperature";
        let initial_value = 20.0;
        let new_value = 25.5;
        
        // Act
        twin.add_sensor(sensor_id, "Temperature", "°C", initial_value).unwrap();
        twin.update_sensor_value(sensor_id, new_value).unwrap();
        
        // Assert
        let sensor = twin.get_sensor(sensor_id).unwrap();
        assert_eq!(sensor.current_value(), new_value);
    }
}
```

### Integration Tests

Integration tests verify that components work together correctly:

- Test repositories with actual database
- Test LLM clients with mock servers
- Test tool implementations with controlled environments

Location: `tests/integration/`

Example integration test:

```rust
#[tokio::test]
async fn test_twin_repository_saves_and_retrieves_twin() {
    // Setup
    let db_path = temp_db_path();
    let pool = create_test_db_pool(&db_path).await;
    run_migrations(&pool).await.unwrap();
    
    let repo = TwinRepositorySqlite::new(pool);
    
    // Create a twin
    let twin = DigitalTwin::new("test-twin", "Test Twin");
    twin.add_sensor("temp", "Temperature", "°C", 20.0).unwrap();
    
    // Save it
    repo.save(&twin).await.unwrap();
    
    // Retrieve it
    let retrieved = repo.find_by_id("test-twin").await.unwrap();
    
    // Verify
    assert_eq!(twin.id(), retrieved.id());
    assert_eq!(twin.name(), retrieved.name());
    
    let sensor = retrieved.get_sensor("temp").unwrap();
    assert_eq!(sensor.name(), "Temperature");
    assert_eq!(sensor.unit(), "°C");
    assert_eq!(sensor.current_value(), 20.0);
    
    // Cleanup
    std::fs::remove_file(db_path).unwrap();
}
```

### End-to-End Tests

E2E tests verify complete user workflows:

- Test critical user journeys
- Test frontend-backend integration
- Test with actual or realistic data

Location: `tests/e2e/`

Example E2E test:

```rust
#[tokio::test]
async fn test_create_twin_workflow() {
    // Setup application
    let app = TestApp::spawn().await;
    
    // Create a twin via API
    let twin_id = app.create_twin("Test Twin").await.unwrap();
    
    // Add sensors
    app.add_sensor(twin_id, "temperature", "Temperature", "°C", 20.0).await.unwrap();
    app.add_sensor(twin_id, "humidity", "Humidity", "%", 45.0).await.unwrap();
    
    // Verify twin exists and has correct data
    let twin = app.get_twin(twin_id).await.unwrap();
    assert_eq!(twin.name, "Test Twin");
    assert_eq!(twin.sensors.len(), 2);
    
    // Cleanup
    app.shutdown().await;
}
```

## Test Doubles

Use appropriate test doubles for different scenarios:

1. **Mocks**: Objects that record interactions and can verify expectations
2. **Stubs**: Objects that provide canned answers to calls
3. **Fakes**: Objects with simplified implementations of real components
4. **Spies**: Objects that record interactions but don't verify them
5. **Dummies**: Objects that are passed around but not used

Example mock implementation:

```rust
struct MockLlmClient {
    responses: Vec<String>,
    calls: RefCell<Vec<String>>,
}

impl MockLlmClient {
    fn new() -> Self {
        Self {
            responses: Vec::new(),
            calls: RefCell::new(Vec::new()),
        }
    }
    
    fn with_response(mut self, response: &str) -> Self {
        self.responses.push(response.to_string());
        self
    }
    
    fn verify_called_with(&self, expected: &str) -> bool {
        self.calls.borrow().contains(&expected.to_string())
    }
}

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn generate(&self, prompt: &str) -> Result<String, LlmError> {
        self.calls.borrow_mut().push(prompt.to_string());
        Ok(self.responses.get(0).cloned().unwrap_or_default())
    }
}
```

## Test Fixtures

Use fixtures to set up common test data:

```rust
// tests/fixtures/mod.rs
pub fn create_test_twin() -> DigitalTwin {
    let mut twin = DigitalTwin::new("test-twin", "Test Twin");
    twin.add_sensor("temperature", "Temperature", "°C", 20.0).unwrap();
    twin.add_sensor("humidity", "Humidity", "%", 45.0).unwrap();
    twin
}

pub fn create_test_conversation() -> Conversation {
    let mut conversation = Conversation::new("test-conversation");
    conversation.add_message(Message::user("Hello, how can I monitor my twin?"));
    conversation.add_message(Message::assistant("I can help you monitor your digital twin..."));
    conversation
}
```

## Test Helpers

Create helper functions for common test operations:

```rust
// tests/helpers/mod.rs
pub async fn create_test_db_pool(path: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}", path))
        .await
        .unwrap()
}

pub fn temp_db_path() -> String {
    format!("file:{}?mode=memory&cache=shared", Uuid::new_v4())
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./src/infrastructure/db/migrations")
        .run(pool)
        .await
}
```

## Testing Async Code

For testing async code:

1. Use `#[tokio::test]` for async tests
2. Use `tokio::spawn` for concurrent operations
3. Use `tokio::time::timeout` for time-limited tests
4. Use `tokio::sync::oneshot` for coordinating async tests

Example async test:

```rust
#[tokio::test]
async fn test_agent_responds_within_timeout() {
    // Setup
    let agent = create_test_agent();
    
    // Test with timeout
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        agent.process_message("Hello")
    ).await;
    
    // Verify
    assert!(result.is_ok(), "Agent did not respond within timeout");
    let response = result.unwrap();
    assert!(response.contains("Hello"), "Response should acknowledge greeting");
}
```

## Testing Error Handling

Test both success and error paths:

```rust
#[test]
fn test_twin_update_sensor_errors_for_unknown_sensor() {
    // Arrange
    let mut twin = DigitalTwin::new("test-twin", "Test Twin");
    
    // Act
    let result = twin.update_sensor_value("nonexistent", 25.0);
    
    // Assert
    assert!(result.is_err());
    match result {
        Err(DomainError::EntityNotFound(entity, id)) => {
            assert_eq!(entity, "Sensor");
            assert_eq!(id, "nonexistent");
        }
        _ => panic!("Expected EntityNotFound error"),
    }
}
```

## Test Coverage

Aim for high test coverage:

- 90%+ for domain models and core business logic
- 80%+ for application services and use cases
- 70%+ for infrastructure components

Use tools like `cargo-tarpaulin` to measure coverage:

```bash
cargo tarpaulin --out Html --output-dir coverage
```

## Test Organization

Organize tests to mirror the production code structure:

```
tests/
├── unit/                  # Unit tests
│   ├── core/              # Tests for core domain
│   │   ├── domain/        # Tests for domain models
│   │   └── application/   # Tests for use cases
│   └── infrastructure/    # Tests for infrastructure components
│
├── integration/           # Integration tests
│   ├── api/               # Tests for API layer
│   ├── db/                # Tests for database repositories
│   └── llm/               # Tests for LLM clients
│
├── e2e/                   # End-to-end tests
│   └── scenarios/         # Test scenarios
│
├── fixtures/              # Test fixtures
│   ├── mod.rs
│   └── data/              # Test data files
│
├── helpers/               # Test helpers
│   └── mod.rs
│
└── mocks/                 # Mock implementations
    ├── llm/
    └── repositories/
```

## Test Naming Conventions

Follow these naming conventions:

- Test modules: `#[cfg(test)] mod tests`
- Test functions: `test_<subject>_<behavior>_<condition>`
- Test files: `<module_name>_tests.rs`

Examples:
- `test_digital_twin_updates_sensor_value`
- `test_agent_executes_tool_when_requested`
- `test_repository_returns_error_for_nonexistent_entity`

## Test Documentation

Document tests with clear comments:

```rust
/// Tests that the digital twin correctly updates sensor values
/// 
/// This test verifies:
/// 1. A sensor can be added to a twin
/// 2. The sensor value can be updated
/// 3. The updated value is correctly retrieved
#[test]
fn test_digital_twin_updates_sensor_value() {
    // Test implementation...
}
```

## Continuous Integration

Run tests in CI:

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions-rs/cargo@v1
        with:
          command: test
          args: --all-features
```

## Performance Testing

For performance-critical components:

1. Use benchmarks to measure performance
2. Set performance budgets
3. Compare performance before and after changes
4. Test with realistic data volumes

Example benchmark:

```rust
#[bench]
fn bench_twin_sensor_updates(b: &mut Bencher) {
    // Setup
    let mut twin = create_large_test_twin(100); // Twin with 100 sensors
    
    b.iter(|| {
        // Operation to benchmark
        for i in 0..100 {
            twin.update_sensor_value(&format!("sensor_{}", i), i as f64).unwrap();
        }
    });
}
```

## Security Testing

Test security aspects:

1. Test input validation
2. Test authentication and authorization
3. Test rate limiting
4. Test data encryption

Example security test:

```rust
#[test]
fn test_tool_executor_rejects_unauthorized_tools() {
    // Setup
    let executor = ToolExecutor::new(
        ToolRegistry::default(),
        Permissions::with_allowed_tools(vec!["calculator"])
    );
    
    // Act
    let result = executor.execute("web_search", "query");
    
    // Assert
    assert!(matches!(result, Err(ToolError::Unauthorized(_))));
}
```

## Debugging Test Failures

When tests fail:

1. Use `RUST_BACKTRACE=1` for detailed stack traces
2. Use `println!` or logging for debugging information
3. Isolate the failing test with `cargo test test_name`
4. Use `--nocapture` to see stdout output: `cargo test -- --nocapture`

## Test Maintenance

Keep tests maintainable:

1. Refactor tests when refactoring production code
2. Extract common setup into fixtures and helpers
3. Avoid test interdependencies
4. Delete tests for removed functionality
5. Update tests when requirements change