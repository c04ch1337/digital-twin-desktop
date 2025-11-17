//! Rate limiting middleware for Tauri commands
//!
//! This module provides middleware for rate limiting API calls
//! to prevent abuse and ensure fair usage of resources.

use std::sync::Arc;
use tauri::{command, State};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::net::IpAddr;

use crate::infrastructure::security::{RateLimiter, RateLimitKey, RateLimitInfo, RateLimitError};
use crate::api::error::ApiError;

/// Rate limit middleware error
#[derive(Debug, Error)]
pub enum RateLimitMiddlewareError {
    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again in {0} seconds.")]
    RateLimitExceeded(u64),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<RateLimitMiddlewareError> for ApiError {
    fn from(err: RateLimitMiddlewareError) -> Self {
        match err {
            RateLimitMiddlewareError::RateLimitExceeded(seconds) => {
                ApiError::TooManyRequests(format!("Rate limit exceeded. Try again in {} seconds.", seconds))
            }
            RateLimitMiddlewareError::Internal(msg) => ApiError::Internal(msg),
        }
    }
}

impl From<RateLimitError> for RateLimitMiddlewareError {
    fn from(err: RateLimitError) -> Self {
        match err {
            RateLimitError::RateLimitExceeded(seconds) => RateLimitMiddlewareError::RateLimitExceeded(seconds),
            RateLimitError::Internal(msg) => RateLimitMiddlewareError::Internal(msg),
        }
    }
}

/// Rate limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    /// Rate limit information
    pub rate_limit: RateLimitInfo,
}

/// Rate limit middleware
pub struct RateLimitMiddleware {
    /// Rate limiter
    rate_limiter: Arc<RateLimiter>,
}

impl RateLimitMiddleware {
    /// Create a new rate limit middleware
    pub fn new(rate_limiter: Arc<RateLimiter>) -> Self {
        Self { rate_limiter }
    }
    
    /// Check rate limit for an IP address
    pub fn check_ip(&self, ip: IpAddr) -> Result<RateLimitInfo, RateLimitMiddlewareError> {
        let key = RateLimitKey::IpAddress(ip);
        self.rate_limiter.check(&key).map_err(RateLimitMiddlewareError::from)
    }
    
    /// Check rate limit for an API key
    pub fn check_api_key(&self, api_key: &str) -> Result<RateLimitInfo, RateLimitMiddlewareError> {
        let key = RateLimitKey::ApiKey(api_key.to_string());
        self.rate_limiter.check(&key).map_err(RateLimitMiddlewareError::from)
    }
    
    /// Check rate limit for a user
    pub fn check_user(&self, user_id: &str) -> Result<RateLimitInfo, RateLimitMiddlewareError> {
        let key = RateLimitKey::UserId(user_id.to_string());
        self.rate_limiter.check(&key).map_err(RateLimitMiddlewareError::from)
    }
    
    /// Check rate limit for a custom key
    pub fn check_custom(&self, key: &str) -> Result<RateLimitInfo, RateLimitMiddlewareError> {
        let key = RateLimitKey::Custom(key.to_string());
        self.rate_limiter.check(&key).map_err(RateLimitMiddlewareError::from)
    }
    
    /// Reset rate limit for a key
    pub fn reset(&self, key_type: &str, key_value: &str) {
        let key = match key_type {
            "ip" => {
                if let Ok(ip) = key_value.parse::<IpAddr>() {
                    RateLimitKey::IpAddress(ip)
                } else {
                    return;
                }
            }
            "api" => RateLimitKey::ApiKey(key_value.to_string()),
            "user" => RateLimitKey::UserId(key_value.to_string()),
            "custom" => RateLimitKey::Custom(key_value.to_string()),
            _ => return,
        };
        
        self.rate_limiter.reset(&key);
    }
}

/// Rate limit middleware function
///
/// This function can be used as a middleware for Tauri commands
/// to apply rate limiting.
#[command]
pub async fn rate_limit(
    rate_limit_middleware: State<'_, RateLimitMiddleware>,
    key_type: String,
    key_value: String,
) -> Result<RateLimitResponse, ApiError> {
    let info = match key_type.as_str() {
        "ip" => {
            let ip = key_value.parse::<IpAddr>()
                .map_err(|_| ApiError::BadRequest("Invalid IP address".to_string()))?;
            rate_limit_middleware.check_ip(ip)
        }
        "api" => rate_limit_middleware.check_api_key(&key_value),
        "user" => rate_limit_middleware.check_user(&key_value),
        "custom" => rate_limit_middleware.check_custom(&key_value),
        _ => return Err(ApiError::BadRequest("Invalid key type".to_string())),
    }
    .map_err(ApiError::from)?;
    
    Ok(RateLimitResponse { rate_limit: info })
}

/// Custom rate limit middleware function
///
/// This function can be used to apply custom rate limits for specific endpoints.
pub fn with_rate_limit<F, T, E>(
    rate_limit_middleware: &RateLimitMiddleware,
    key_type: &str,
    key_value: &str,
    func: F,
) -> Result<T, ApiError>
where
    F: FnOnce() -> Result<T, E>,
    E: Into<ApiError>,
{
    // Check rate limit
    let info = match key_type {
        "ip" => {
            let ip = key_value.parse::<IpAddr>()
                .map_err(|_| ApiError::BadRequest("Invalid IP address".to_string()))?;
            rate_limit_middleware.check_ip(ip)
        }
        "api" => rate_limit_middleware.check_api_key(key_value),
        "user" => rate_limit_middleware.check_user(key_value),
        "custom" => rate_limit_middleware.check_custom(key_value),
        _ => return Err(ApiError::BadRequest("Invalid key type".to_string())),
    }
    .map_err(ApiError::from)?;
    
    // Execute the function
    let result = func().map_err(Into::into);
    
    // Add rate limit headers to the response
    // Note: In a real implementation, we would add headers to the HTTP response
    // but Tauri doesn't expose this directly, so we would need to handle it differently
    
    result
}