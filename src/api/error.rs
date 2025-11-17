//! Error handling for the API layer
//!
//! This module provides error types and conversion functions for mapping
//! domain and application errors to frontend-friendly error responses.

use serde::{Serialize, Deserialize};
use std::fmt;

/// API error response that will be sent to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Optional details for debugging (not shown to end users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Error codes used in API responses
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    /// Invalid input parameters
    InvalidInput,
    /// Entity not found
    NotFound,
    /// Unauthorized access
    Unauthorized,
    /// Conflict with existing data
    Conflict,
    /// Internal server error
    Internal,
    /// Service unavailable
    Unavailable,
    /// Rate limit exceeded
    RateLimited,
    /// Validation error
    Validation,
}

impl ErrorCode {
    /// Convert error code to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::InvalidInput => "INVALID_INPUT",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::Unauthorized => "UNAUTHORIZED",
            ErrorCode::Conflict => "CONFLICT",
            ErrorCode::Internal => "INTERNAL_ERROR",
            ErrorCode::Unavailable => "SERVICE_UNAVAILABLE",
            ErrorCode::RateLimited => "RATE_LIMITED",
            ErrorCode::Validation => "VALIDATION_ERROR",
        }
    }
}

/// Result type for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Convert any error to an ApiError
pub fn to_api_error<E: std::error::Error>(err: E) -> ApiError {
    // Default to internal error
    ApiError {
        code: ErrorCode::Internal.as_str().to_string(),
        message: "An unexpected error occurred".to_string(),
        details: Some(err.to_string()),
    }
}

/// Map domain errors to appropriate API errors
pub fn map_domain_error(err: Box<dyn std::error::Error + Send + Sync>) -> ApiError {
    // Check error type and map to appropriate API error
    let error_message = err.to_string();
    
    if error_message.contains("not found") {
        return ApiError {
            code: ErrorCode::NotFound.as_str().to_string(),
            message: "The requested resource was not found".to_string(),
            details: Some(error_message),
        };
    } else if error_message.contains("unauthorized") || error_message.contains("forbidden") {
        return ApiError {
            code: ErrorCode::Unauthorized.as_str().to_string(),
            message: "You don't have permission to perform this action".to_string(),
            details: Some(error_message),
        };
    } else if error_message.contains("invalid") || error_message.contains("validation") {
        return ApiError {
            code: ErrorCode::InvalidInput.as_str().to_string(),
            message: "Invalid input parameters".to_string(),
            details: Some(error_message),
        };
    } else if error_message.contains("conflict") || error_message.contains("already exists") {
        return ApiError {
            code: ErrorCode::Conflict.as_str().to_string(),
            message: "This operation conflicts with existing data".to_string(),
            details: Some(error_message),
        };
    }
    
    // Default to internal error
    ApiError {
        code: ErrorCode::Internal.as_str().to_string(),
        message: "An unexpected error occurred".to_string(),
        details: Some(error_message),
    }
}

/// Helper function to convert Result<T, E> to ApiResult<T>
pub fn map_result<T, E: std::error::Error + Send + Sync + 'static>(result: Result<T, E>) -> ApiResult<T> {
    result.map_err(|e| map_domain_error(Box::new(e)))
}