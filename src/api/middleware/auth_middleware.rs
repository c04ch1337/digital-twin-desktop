//! Authentication middleware for Tauri commands
//!
//! This module provides middleware for authenticating users
//! and checking permissions for Tauri commands.

use std::sync::Arc;
use tauri::{command, State};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::infrastructure::security::{AuthService, PermissionManager, AuthError};
use crate::api::error::ApiError;

/// Authentication middleware error
#[derive(Debug, Error)]
pub enum AuthMiddlewareError {
    /// Unauthorized
    #[error("Unauthorized")]
    Unauthorized,
    
    /// Forbidden
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<AuthMiddlewareError> for ApiError {
    fn from(err: AuthMiddlewareError) -> Self {
        match err {
            AuthMiddlewareError::Unauthorized => ApiError::Unauthorized("Authentication required".to_string()),
            AuthMiddlewareError::Forbidden(msg) => ApiError::Forbidden(msg),
            AuthMiddlewareError::AuthError(e) => ApiError::Unauthorized(e.to_string()),
            AuthMiddlewareError::Internal(msg) => ApiError::Internal(msg),
        }
    }
}

/// Authentication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    /// User ID
    pub user_id: String,
    
    /// Username
    pub username: String,
    
    /// Roles
    pub roles: Vec<String>,
    
    /// Permissions
    pub permissions: Vec<String>,
}

/// Authentication middleware
pub struct AuthMiddleware {
    /// Authentication service
    auth_service: Arc<AuthService>,
    
    /// Permission manager
    permission_manager: Arc<PermissionManager>,
}

impl AuthMiddleware {
    /// Create a new authentication middleware
    pub fn new(auth_service: Arc<AuthService>, permission_manager: Arc<PermissionManager>) -> Self {
        Self {
            auth_service,
            permission_manager,
        }
    }
    
    /// Authenticate a user with username and password
    pub async fn login(&self, username: &str, password: &str) -> Result<AuthContext, AuthMiddlewareError> {
        // Authenticate the user
        let user = self.auth_service.authenticate(username, password)
            .map_err(AuthMiddlewareError::AuthError)?;
        
        // Get user permissions
        let permissions = self.permission_manager.get_user_permissions(&user.id.to_string())
            .map_err(|e| AuthMiddlewareError::Internal(e.to_string()))?;
        
        // Get user roles
        let roles = self.permission_manager.get_user_roles(&user.id.to_string())
            .map_err(|e| AuthMiddlewareError::Internal(e.to_string()))?;
        
        Ok(AuthContext {
            user_id: user.id.to_string(),
            username: user.username,
            roles: roles.into_iter().collect(),
            permissions: permissions.into_iter().collect(),
        })
    }
    
    /// Authenticate a user with an API key
    pub async fn login_with_api_key(&self, api_key: &str) -> Result<AuthContext, AuthMiddlewareError> {
        // Authenticate with API key
        let (user, api_key_info) = self.auth_service.authenticate_with_api_key(api_key)
            .map_err(AuthMiddlewareError::AuthError)?;
        
        // Get user permissions (including API key permissions)
        let mut permissions = self.permission_manager.get_user_permissions(&user.id.to_string())
            .map_err(|e| AuthMiddlewareError::Internal(e.to_string()))?;
        
        // Add API key specific permissions
        for perm in &api_key_info.permissions {
            permissions.insert(perm.id.clone());
        }
        
        // Get user roles
        let roles = self.permission_manager.get_user_roles(&user.id.to_string())
            .map_err(|e| AuthMiddlewareError::Internal(e.to_string()))?;
        
        Ok(AuthContext {
            user_id: user.id.to_string(),
            username: user.username,
            roles: roles.into_iter().collect(),
            permissions: permissions.into_iter().collect(),
        })
    }
    
    /// Check if a user has a permission
    pub fn check_permission(&self, auth_context: &AuthContext, permission: &str) -> Result<(), AuthMiddlewareError> {
        if auth_context.permissions.contains(&permission.to_string()) {
            Ok(())
        } else {
            Err(AuthMiddlewareError::Forbidden(format!("Missing permission: {}", permission)))
        }
    }
    
    /// Check if a user has a role
    pub fn check_role(&self, auth_context: &AuthContext, role: &str) -> Result<(), AuthMiddlewareError> {
        if auth_context.roles.contains(&role.to_string()) {
            Ok(())
        } else {
            Err(AuthMiddlewareError::Forbidden(format!("Missing role: {}", role)))
        }
    }
}

/// Authenticate middleware function
///
/// This function can be used as a middleware for Tauri commands
/// to authenticate users.
#[command]
pub async fn authenticate(
    auth_middleware: State<'_, AuthMiddleware>,
    auth_header: Option<String>,
) -> Result<AuthContext, ApiError> {
    match auth_header {
        Some(header) => {
            // Check if it's a Bearer token
            if header.starts_with("Bearer ") {
                let token = header.trim_start_matches("Bearer ").trim();
                auth_middleware.login_with_api_key(token).await.map_err(ApiError::from)
            } else if header.starts_with("Basic ") {
                // Basic authentication
                let credentials = header.trim_start_matches("Basic ").trim();
                let decoded = base64::decode(credentials)
                    .map_err(|_| ApiError::Unauthorized("Invalid Basic authentication".to_string()))?;
                
                let credentials_str = String::from_utf8(decoded)
                    .map_err(|_| ApiError::Unauthorized("Invalid Basic authentication".to_string()))?;
                
                let parts: Vec<&str> = credentials_str.split(':').collect();
                if parts.len() != 2 {
                    return Err(ApiError::Unauthorized("Invalid Basic authentication".to_string()));
                }
                
                let username = parts[0];
                let password = parts[1];
                
                auth_middleware.login(username, password).await.map_err(ApiError::from)
            } else {
                Err(ApiError::Unauthorized("Invalid authentication header".to_string()))
            }
        }
        None => Err(ApiError::Unauthorized("Authentication required".to_string())),
    }
}

/// Require permission middleware function
///
/// This function can be used as a middleware for Tauri commands
/// to check if a user has a specific permission.
#[command]
pub async fn require_permission(
    auth_middleware: State<'_, AuthMiddleware>,
    auth_context: AuthContext,
    permission: String,
) -> Result<(), ApiError> {
    auth_middleware.check_permission(&auth_context, &permission)
        .map_err(ApiError::from)
}