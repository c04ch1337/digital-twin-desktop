//! Security module for the Digital Twin Desktop application
//! 
//! This module provides authentication, encryption, rate limiting,
//! sandboxed execution, and permission management functionality.

mod auth;
mod encryption;
mod rate_limiter;
mod sandbox;
mod permissions;

pub use auth::{ApiKeyManager, AuthService, AuthError};
pub use encryption::{EncryptionService, EncryptionError};
pub use rate_limiter::{RateLimiter, RateLimitError};
pub use sandbox::{Sandbox, SandboxError};
pub use permissions::{PermissionManager, Permission, Role, PermissionError};

/// Initialize the security module
pub fn init() -> anyhow::Result<()> {
    tracing::info!("Initializing security module");
    Ok(())
}