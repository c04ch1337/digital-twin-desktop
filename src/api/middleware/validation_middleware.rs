//! Input validation middleware for Tauri commands
//!
//! This module provides middleware for validating input data
//! for Tauri commands to prevent security issues and ensure data integrity.

use tauri::{command, State};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;
use validator::{Validate, ValidationErrors};
use std::collections::HashMap;

use crate::api::error::ApiError;

/// Validation middleware error
#[derive(Debug, Error)]
pub enum ValidationMiddlewareError {
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<ValidationMiddlewareError> for ApiError {
    fn from(err: ValidationMiddlewareError) -> Self {
        match err {
            ValidationMiddlewareError::ValidationError(msg) => ApiError::BadRequest(msg),
            ValidationMiddlewareError::InvalidInput(msg) => ApiError::BadRequest(msg),
            ValidationMiddlewareError::Internal(msg) => ApiError::Internal(msg),
        }
    }
}

impl From<ValidationErrors> for ValidationMiddlewareError {
    fn from(errors: ValidationErrors) -> Self {
        let error_map = errors.field_errors();
        let mut error_messages = HashMap::new();
        
        for (field, errors) in error_map {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| {
                    if let Some(message) = &error.message {
                        message.clone()
                    } else {
                        format!("{} is invalid", field)
                    }
                })
                .collect();
            
            error_messages.insert(field.to_string(), messages);
        }
        
        ValidationMiddlewareError::ValidationError(serde_json::to_string(&error_messages).unwrap_or_else(|_| "Validation failed".to_string()))
    }
}

/// Validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResponse {
    /// Validation result
    pub valid: bool,
    
    /// Validation errors (if any)
    pub errors: Option<Value>,
}

/// Validation middleware
pub struct ValidationMiddleware;

impl ValidationMiddleware {
    /// Create a new validation middleware
    pub fn new() -> Self {
        Self
    }
    
    /// Validate input data
    pub fn validate<T>(&self, data: &T) -> Result<(), ValidationMiddlewareError>
    where
        T: Validate + Serialize,
    {
        // Validate the data
        data.validate().map_err(ValidationMiddlewareError::from)?;
        
        Ok(())
    }
    
    /// Validate JSON data against a schema
    pub fn validate_json(&self, data: &Value, schema: &Value) -> Result<(), ValidationMiddlewareError> {
        // Use jsonschema to validate the data
        let compiled_schema = jsonschema::JSONSchema::compile(schema)
            .map_err(|e| ValidationMiddlewareError::Internal(format!("Failed to compile schema: {}", e)))?;
        
        let result = compiled_schema.validate(data);
        
        if let Err(errors) = result {
            let error_messages: Vec<String> = errors
                .map(|error| error.to_string())
                .collect();
            
            return Err(ValidationMiddlewareError::ValidationError(
                serde_json::to_string(&error_messages).unwrap_or_else(|_| "Validation failed".to_string())
            ));
        }
        
        Ok(())
    }
    
    /// Sanitize input data
    pub fn sanitize(&self, data: &Value) -> Value {
        match data {
            Value::Object(map) => {
                let mut sanitized = serde_json::Map::new();
                
                for (key, value) in map {
                    sanitized.insert(key.clone(), self.sanitize(value));
                }
                
                Value::Object(sanitized)
            }
            Value::Array(arr) => {
                let sanitized: Vec<Value> = arr
                    .iter()
                    .map(|value| self.sanitize(value))
                    .collect();
                
                Value::Array(sanitized)
            }
            Value::String(s) => {
                // Sanitize strings (e.g., remove HTML tags, escape special characters)
                let sanitized = ammonia::clean(s);
                Value::String(sanitized)
            }
            _ => data.clone(),
        }
    }
}

/// Validate input middleware function
///
/// This function can be used as a middleware for Tauri commands
/// to validate input data.
#[command]
pub async fn validate_input(
    validation_middleware: State<'_, ValidationMiddleware>,
    data: Value,
    schema: Option<Value>,
) -> Result<ValidationResponse, ApiError> {
    // Sanitize the input data
    let sanitized_data = validation_middleware.sanitize(&data);
    
    // Validate against schema if provided
    if let Some(schema) = schema {
        validation_middleware.validate_json(&sanitized_data, &schema)
            .map_err(|err| {
                ApiError::from(err)
            })?;
    }
    
    Ok(ValidationResponse {
        valid: true,
        errors: None,
    })
}

/// Custom validation middleware function
///
/// This function can be used to validate input data for specific endpoints.
pub fn with_validation<F, T, E, V>(
    validation_middleware: &ValidationMiddleware,
    data: &V,
    func: F,
) -> Result<T, ApiError>
where
    F: FnOnce() -> Result<T, E>,
    E: Into<ApiError>,
    V: Validate + Serialize,
{
    // Validate the data
    validation_middleware.validate(data)
        .map_err(ApiError::from)?;
    
    // Execute the function
    func().map_err(Into::into)
}

/// JSON schema validation middleware function
///
/// This function can be used to validate JSON data against a schema.
pub fn with_json_validation<F, T, E>(
    validation_middleware: &ValidationMiddleware,
    data: &Value,
    schema: &Value,
    func: F,
) -> Result<T, ApiError>
where
    F: FnOnce() -> Result<T, E>,
    E: Into<ApiError>,
{
    // Validate the data against the schema
    validation_middleware.validate_json(data, schema)
        .map_err(ApiError::from)?;
    
    // Execute the function
    func().map_err(Into::into)
}