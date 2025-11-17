//! Common test utilities for the Digital Twin Desktop application.
//!
//! This module provides shared functionality used across different test types,
//! including test setup, teardown, and common assertions.

use std::sync::Once;
use tokio::runtime::Runtime;

/// Initialize test environment once
static INIT: Once = Once::new();

/// Setup function to initialize the test environment
pub fn setup() {
    INIT.call_once(|| {
        // Initialize logging for tests
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    });
}

/// Create a tokio runtime for async tests
pub fn create_test_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime for tests")
}

/// Helper to run async tests
pub fn run_async<F>(test_fn: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    setup();
    let rt = create_test_runtime();
    rt.block_on(test_fn);
}

/// Test database connection string for SQLite in-memory database
pub fn test_db_connection() -> String {
    ":memory:".to_string()
}

/// Assertion helpers
pub mod assertions {
    /// Assert that two values are approximately equal within a given epsilon
    pub fn assert_approx_eq<T>(a: T, b: T, epsilon: T)
    where
        T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy,
    {
        let diff = if a > b { a - b } else { b - a };
        assert!(diff <= epsilon, "Values differ by more than epsilon: {:?} vs {:?}", a, b);
    }
}