//! Authentication system with API key management
//!
//! This module provides functionality for managing API keys,
//! authenticating users, and handling authentication-related operations.

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::infrastructure::config::SecurityConfig;
use crate::infrastructure::security::encryption::EncryptionService;
use crate::infrastructure::security::permissions::{Permission, Role};

/// Authentication error types
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    /// API key not found
    #[error("API key not found")]
    ApiKeyNotFound,
    
    /// API key expired
    #[error("API key expired")]
    ApiKeyExpired,
    
    /// API key revoked
    #[error("API key revoked")]
    ApiKeyRevoked,
    
    /// User not found
    #[error("User not found")]
    UserNotFound,
    
    /// Unauthorized
    #[error("Unauthorized")]
    Unauthorized,
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// API key ID
    pub id: Uuid,
    
    /// API key name
    pub name: String,
    
    /// API key prefix (for display)
    pub prefix: String,
    
    /// API key hash (for verification)
    pub hash: String,
    
    /// User ID associated with the API key
    pub user_id: Uuid,
    
    /// Creation date
    pub created_at: DateTime<Utc>,
    
    /// Expiration date (if any)
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Revocation date (if revoked)
    pub revoked_at: Option<DateTime<Utc>>,
    
    /// Last used date
    pub last_used_at: Option<DateTime<Utc>>,
    
    /// Permissions associated with this API key
    pub permissions: Vec<Permission>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: Uuid,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// Password hash
    pub password_hash: String,
    
    /// Roles
    pub roles: Vec<Role>,
    
    /// Permissions
    pub permissions: Vec<Permission>,
    
    /// Creation date
    pub created_at: DateTime<Utc>,
    
    /// Last login date
    pub last_login_at: Option<DateTime<Utc>>,
}

/// API key manager
pub struct ApiKeyManager {
    /// Encryption service
    encryption: Arc<EncryptionService>,
    
    /// API keys (in-memory cache)
    api_keys: RwLock<HashMap<String, ApiKey>>,
    
    /// Random number generator
    rng: SystemRandom,
}

impl ApiKeyManager {
    /// Create a new API key manager
    pub fn new(encryption: Arc<EncryptionService>) -> Self {
        Self {
            encryption,
            api_keys: RwLock::new(HashMap::new()),
            rng: SystemRandom::new(),
        }
    }
    
    /// Generate a new API key
    pub fn generate_api_key(&self, name: &str, user_id: Uuid, expires_at: Option<DateTime<Utc>>, permissions: Vec<Permission>) -> Result<(String, ApiKey)> {
        // Generate a random API key
        let mut key_bytes = [0u8; 32];
        self.rng.fill(&mut key_bytes).map_err(|e| anyhow!("Failed to generate random bytes: {}", e))?;
        
        // Convert to base64
        let key = base64::encode(key_bytes);
        let prefix = key[0..8].to_string();
        
        // Hash the key for storage
        let hash = self.hash_api_key(&key)?;
        
        let api_key = ApiKey {
            id: Uuid::new_v4(),
            name: name.to_string(),
            prefix,
            hash,
            user_id,
            created_at: Utc::now(),
            expires_at,
            revoked_at: None,
            last_used_at: None,
            permissions,
        };
        
        // Store the API key
        let mut api_keys = self.api_keys.write().unwrap();
        api_keys.insert(api_key.id.to_string(), api_key.clone());
        
        Ok((key, api_key))
    }
    
    /// Verify an API key
    pub fn verify_api_key(&self, key: &str) -> Result<ApiKey, AuthError> {
        let hash = self.hash_api_key(key).map_err(|e| AuthError::Internal(e.to_string()))?;
        
        // Find the API key by hash
        let api_keys = self.api_keys.read().unwrap();
        let api_key = api_keys.values()
            .find(|k| k.hash == hash)
            .cloned()
            .ok_or(AuthError::ApiKeyNotFound)?;
        
        // Check if the API key is expired
        if let Some(expires_at) = api_key.expires_at {
            if expires_at < Utc::now() {
                return Err(AuthError::ApiKeyExpired);
            }
        }
        
        // Check if the API key is revoked
        if api_key.revoked_at.is_some() {
            return Err(AuthError::ApiKeyRevoked);
        }
        
        // Update last used time
        drop(api_keys);
        let mut api_keys = self.api_keys.write().unwrap();
        if let Some(key) = api_keys.get_mut(&api_key.id.to_string()) {
            key.last_used_at = Some(Utc::now());
        }
        
        Ok(api_key)
    }
    
    /// Revoke an API key
    pub fn revoke_api_key(&self, key_id: Uuid) -> Result<(), AuthError> {
        let mut api_keys = self.api_keys.write().unwrap();
        let api_key = api_keys.get_mut(&key_id.to_string()).ok_or(AuthError::ApiKeyNotFound)?;
        
        api_key.revoked_at = Some(Utc::now());
        
        Ok(())
    }
    
    /// List API keys for a user
    pub fn list_api_keys(&self, user_id: Uuid) -> Vec<ApiKey> {
        let api_keys = self.api_keys.read().unwrap();
        api_keys.values()
            .filter(|k| k.user_id == user_id)
            .cloned()
            .collect()
    }
    
    /// Hash an API key
    fn hash_api_key(&self, key: &str) -> Result<String> {
        // Use a secure hash function (HMAC-SHA256)
        use ring::hmac;
        
        let key_bytes = key.as_bytes();
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, b"api-key-salt");
        let signature = hmac::sign(&signing_key, key_bytes);
        
        Ok(base64::encode(signature.as_ref()))
    }
}

/// Authentication service
pub struct AuthService {
    /// API key manager
    api_key_manager: Arc<ApiKeyManager>,
    
    /// Encryption service
    encryption: Arc<EncryptionService>,
    
    /// Users (in-memory cache)
    users: RwLock<HashMap<Uuid, User>>,
    
    /// Security configuration
    config: SecurityConfig,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(api_key_manager: Arc<ApiKeyManager>, encryption: Arc<EncryptionService>, config: SecurityConfig) -> Self {
        Self {
            api_key_manager,
            encryption,
            users: RwLock::new(HashMap::new()),
            config,
        }
    }
    
    /// Register a new user
    pub fn register_user(&self, username: &str, email: &str, password: &str, roles: Vec<Role>) -> Result<User, AuthError> {
        // Hash the password
        let password_hash = self.hash_password(password)?;
        
        // Create the user
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            roles,
            permissions: vec![],
            created_at: Utc::now(),
            last_login_at: None,
        };
        
        // Store the user
        let mut users = self.users.write().unwrap();
        users.insert(user.id, user.clone());
        
        Ok(user)
    }
    
    /// Authenticate a user
    pub fn authenticate(&self, username: &str, password: &str) -> Result<User, AuthError> {
        // Find the user
        let users = self.users.read().unwrap();
        let user = users.values()
            .find(|u| u.username == username)
            .cloned()
            .ok_or(AuthError::UserNotFound)?;
        
        // Verify the password
        if !self.verify_password(password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }
        
        // Update last login time
        drop(users);
        let mut users = self.users.write().unwrap();
        if let Some(user) = users.get_mut(&user.id) {
            user.last_login_at = Some(Utc::now());
        }
        
        Ok(user)
    }
    
    /// Authenticate with API key
    pub fn authenticate_with_api_key(&self, api_key: &str) -> Result<(User, ApiKey), AuthError> {
        // Verify the API key
        let api_key = self.api_key_manager.verify_api_key(api_key)?;
        
        // Find the user
        let users = self.users.read().unwrap();
        let user = users.get(&api_key.user_id).cloned().ok_or(AuthError::UserNotFound)?;
        
        Ok((user, api_key))
    }
    
    /// Hash a password
    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };
        
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::Internal(e.to_string()))?
            .to_string();
        
        Ok(hash)
    }
    
    /// Verify a password
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };
        
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::Internal(e.to_string()))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::RateLimitConfig;
    use crate::infrastructure::security::encryption::EncryptionService;
    
    fn create_test_config() -> SecurityConfig {
        SecurityConfig {
            secret_key: "test-secret-key".to_string(),
            token_expiration: 3600,
            cors_origins: vec!["*".to_string()],
            rate_limit: RateLimitConfig {
                requests: 100,
                window_seconds: 60,
            },
        }
    }
    
    #[test]
    fn test_api_key_generation_and_verification() {
        let encryption = Arc::new(EncryptionService::new("test-key").unwrap());
        let api_key_manager = ApiKeyManager::new(encryption.clone());
        
        let user_id = Uuid::new_v4();
        let (key, api_key) = api_key_manager.generate_api_key("Test Key", user_id, None, vec![]).unwrap();
        
        // Verify the API key
        let verified_key = api_key_manager.verify_api_key(&key).unwrap();
        assert_eq!(verified_key.id, api_key.id);
        assert_eq!(verified_key.user_id, user_id);
    }
    
    #[test]
    fn test_api_key_revocation() {
        let encryption = Arc::new(EncryptionService::new("test-key").unwrap());
        let api_key_manager = ApiKeyManager::new(encryption.clone());
        
        let user_id = Uuid::new_v4();
        let (key, api_key) = api_key_manager.generate_api_key("Test Key", user_id, None, vec![]).unwrap();
        
        // Revoke the API key
        api_key_manager.revoke_api_key(api_key.id).unwrap();
        
        // Verify the API key (should fail)
        let result = api_key_manager.verify_api_key(&key);
        assert!(matches!(result, Err(AuthError::ApiKeyRevoked)));
    }
    
    #[test]
    fn test_user_registration_and_authentication() {
        let encryption = Arc::new(EncryptionService::new("test-key").unwrap());
        let api_key_manager = Arc::new(ApiKeyManager::new(encryption.clone()));
        let config = create_test_config();
        let auth_service = AuthService::new(api_key_manager, encryption, config);
        
        // Register a user
        let user = auth_service.register_user("testuser", "test@example.com", "password123", vec![]).unwrap();
        
        // Authenticate the user
        let authenticated_user = auth_service.authenticate("testuser", "password123").unwrap();
        assert_eq!(authenticated_user.id, user.id);
        
        // Authenticate with wrong password (should fail)
        let result = auth_service.authenticate("testuser", "wrongpassword");
        assert!(matches!(result, Err(AuthError::InvalidCredentials)));
    }
}