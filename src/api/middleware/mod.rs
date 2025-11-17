//! Middleware for Tauri commands
//!
//! This module provides middleware components for authentication,
//! rate limiting, and input validation for Tauri commands.

mod auth_middleware;
mod rate_limit_middleware;
mod validation_middleware;

pub use auth_middleware::{AuthMiddleware, authenticate, require_permission};
pub use rate_limit_middleware::{RateLimitMiddleware, rate_limit};
pub use validation_middleware::{ValidationMiddleware, validate_input};

use std::sync::Arc;

use crate::infrastructure::config::SecurityConfig;
use crate::infrastructure::security::{ApiKeyManager, AuthService, EncryptionService, PermissionManager, RateLimiter};

/// Initialize middleware components
pub fn init(config: &SecurityConfig) -> (AuthMiddleware, RateLimitMiddleware, ValidationMiddleware) {
    // Initialize encryption service
    let encryption = Arc::new(EncryptionService::new(&config.secret_key).expect("Failed to initialize encryption service"));
    
    // Initialize API key manager
    let api_key_manager = Arc::new(ApiKeyManager::new(encryption.clone()));
    
    // Initialize auth service
    let auth_service = Arc::new(AuthService::new(api_key_manager, encryption, config.clone()));
    
    // Initialize permission manager
    let permission_manager = Arc::new(PermissionManager::new());
    
    // Initialize rate limiter
    let rate_limiter = Arc::new(RateLimiter::new(config));
    
    // Create middleware components
    let auth_middleware = AuthMiddleware::new(auth_service, permission_manager);
    let rate_limit_middleware = RateLimitMiddleware::new(rate_limiter);
    let validation_middleware = ValidationMiddleware::new();
    
    (auth_middleware, rate_limit_middleware, validation_middleware)
}