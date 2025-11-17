use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
    layer::SubscriberExt,
    Registry,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use std::path::PathBuf;
use anyhow::Result;

use crate::infrastructure::config::LoggingConfig;

/// Initialize logging infrastructure
pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    // Parse log level
    let level = match config.level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // Create env filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.as_str()));

    // Create subscriber
    let subscriber = Registry::default();

    // Add console layer
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE);

    let subscriber = if config.json_format {
        // Add JSON formatting layers
        subscriber
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new("digital-twin".into(), std::io::stdout))
    } else {
        subscriber.with(console_layer)
    };

    // Add file appender if configured
    let subscriber = if let Some(file_path) = &config.file_path {
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            "logs",
            file_path.file_name().unwrap().to_str().unwrap(),
        );

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_writer(non_blocking);

        subscriber.with(file_layer)
    } else {
        subscriber
    };

    // Set global subscriber
    tracing::subscriber::set_global_default(subscriber.with(env_filter))?;

    Ok(())
}

/// Create a logging guard for a specific scope
pub fn scope_guard(name: &str) -> impl Drop {
    let span = tracing::info_span!("scope", name = name);
    span.enter()
}

/// Log execution metrics
pub fn log_metrics(
    operation: &str,
    duration_ms: u64,
    success: bool,
    metadata: Option<serde_json::Value>,
) {
    if success {
        tracing::info!(
            operation = operation,
            duration_ms = duration_ms,
            metadata = ?metadata,
            "Operation completed successfully"
        );
    } else {
        tracing::error!(
            operation = operation,
            duration_ms = duration_ms,
            metadata = ?metadata,
            "Operation failed"
        );
    }
}

/// Create a request logger middleware
pub fn request_logger() -> impl Fn(String) + Clone {
    |route: String| {
        let start = std::time::Instant::now();
        let span = tracing::info_span!(
            "request",
            route = %route,
            request_id = %uuid::Uuid::new_v4(),
        );
        let _enter = span.enter();

        move || {
            let duration = start.elapsed();
            tracing::info!(
                duration_ms = duration.as_millis() as u64,
                "Request completed"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_logging_init() {
        let temp_dir = tempdir().unwrap();
        let config = LoggingConfig {
            level: "debug".to_string(),
            file_path: Some(temp_dir.path().join("test.log")),
            json_format: false,
        };

        assert!(init_logging(&config).is_ok());

        tracing::info!("Test log message");
        tracing::debug!("Test debug message");
        tracing::warn!("Test warning message");
        tracing::error!("Test error message");
    }

    #[test]
    fn test_scope_guard() {
        let _guard = scope_guard("test_scope");
        tracing::info!("Message within test scope");
    }

    #[test]
    fn test_log_metrics() {
        log_metrics(
            "test_operation",
            100,
            true,
            Some(serde_json::json!({
                "key": "value"
            })),
        );

        log_metrics(
            "failed_operation",
            50,
            false,
            None,
        );
    }

    #[test]
    fn test_request_logger() {
        let logger = request_logger();
        let completion = logger("GET /test".to_string());
        
        // Simulate request processing
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        completion();
    }
}