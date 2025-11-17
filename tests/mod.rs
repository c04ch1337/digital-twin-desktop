//! Root test module for the Digital Twin Desktop application.
//! 
//! This module organizes all tests for the application, including:
//! - Unit tests (domain, application, infrastructure)
//! - Integration tests (API, database, LLM)
//! - End-to-end tests
//! - Mock implementations

pub mod common;
pub mod fixtures;
pub mod helpers;
pub mod unit;
pub mod integration;
pub mod e2e;
pub mod mocks;