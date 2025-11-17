use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use anyhow::{Result, anyhow};

use crate::infrastructure::config::SecurityConfig;

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at
    pub iat: u64,
    /// Expiration time
    pub exp: u64,
    /// Roles
    pub roles: Vec<String>,
    /// Permissions
    pub permissions: Vec<String>,
}

/// Password hasher using Argon2
pub struct PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    /// Create a new password hasher
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    /// Hash a password
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

/// JWT token manager
pub struct TokenManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration: u64,
}

impl TokenManager {
    /// Create a new token manager
    pub fn new(config: &SecurityConfig) -> Self {
        let secret = config.secret_key.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expiration: config.token_expiration,
        }
    }

    /// Generate a JWT token
    pub fn generate_token(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + self.expiration,
            roles,
            permissions,
        };

        Ok(encode(&Header::default(), &claims, &self.encoding_key)?)
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    /// Check if a token has expired
    pub fn is_token_expired(&self, claims: &Claims) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        claims.exp < now
    }
}

/// Rate limiter using a token bucket algorithm
pub struct RateLimiter {
    requests: u32,
    window: u64,
    buckets: dashmap::DashMap<String, (u32, u64)>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: &SecurityConfig) -> Self {
        Self {
            requests: config.rate_limit.requests,
            window: config.rate_limit.window_seconds,
            buckets: dashmap::DashMap::new(),
        }
    }

    /// Check if a request is allowed
    pub fn is_allowed(&self, key: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut allowed = true;
        self.buckets.entry(key.to_string()).and_modify(|(count, window_start)| {
            if now - *window_start >= self.window {
                // Reset window
                *count = 1;
                *window_start = now;
            } else if *count >= self.requests {
                // Rate limit exceeded
                allowed = false;
            } else {
                // Increment counter
                *count += 1;
            }
        }).or_insert((1, now));

        allowed
    }

    /// Reset rate limit for a key
    pub fn reset(&self, key: &str) {
        self.buckets.remove(key);
    }
}

/// Permission checker
pub struct PermissionChecker;

impl PermissionChecker {
    /// Check if user has required permissions
    pub fn has_permission(required: &[&str], user_permissions: &[String]) -> bool {
        required.iter().all(|p| user_permissions.contains(&p.to_string()))
    }

    /// Check if user has any of the required permissions
    pub fn has_any_permission(required: &[&str], user_permissions: &[String]) -> bool {
        required.iter().any(|p| user_permissions.contains(&p.to_string()))
    }

    /// Check if user has required role
    pub fn has_role(required: &[&str], user_roles: &[String]) -> bool {
        required.iter().all(|r| user_roles.contains(&r.to_string()))
    }

    /// Check if user has any of the required roles
    pub fn has_any_role(required: &[&str], user_roles: &[String]) -> bool {
        required.iter().any(|r| user_roles.contains(&r.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_password_hasher() {
        let hasher = PasswordHasher::new();
        let password = "test_password";

        let hash = hasher.hash_password(password).unwrap();
        assert!(hasher.verify_password(password, &hash).unwrap());
        assert!(!hasher.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_token_manager() {
        let config = SecurityConfig {
            secret_key: "test_secret".to_string(),
            token_expiration: 3600,
            cors_origins: vec![],
            rate_limit: Default::default(),
        };

        let manager = TokenManager::new(&config);
        let user_id = Uuid::new_v4();
        let roles = vec!["admin".to_string()];
        let permissions = vec!["read".to_string(), "write".to_string()];

        let token = manager.generate_token(user_id, roles.clone(), permissions.clone()).unwrap();
        let claims = manager.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.roles, roles);
        assert_eq!(claims.permissions, permissions);
        assert!(!manager.is_token_expired(&claims));
    }

    #[test]
    fn test_rate_limiter() {
        let config = SecurityConfig {
            secret_key: "test_secret".to_string(),
            token_expiration: 3600,
            cors_origins: vec![],
            rate_limit: crate::infrastructure::config::RateLimitConfig {
                requests: 2,
                window_seconds: 1,
            },
        };

        let limiter = RateLimiter::new(&config);
        let key = "test_key";

        assert!(limiter.is_allowed(key));
        assert!(limiter.is_allowed(key));
        assert!(!limiter.is_allowed(key));

        thread::sleep(Duration::from_secs(1));
        assert!(limiter.is_allowed(key));
    }

    #[test]
    fn test_permission_checker() {
        let user_permissions = vec!["read".to_string(), "write".to_string()];
        let user_roles = vec!["user".to_string(), "admin".to_string()];

        assert!(PermissionChecker::has_permission(&["read", "write"], &user_permissions));
        assert!(!PermissionChecker::has_permission(&["read", "delete"], &user_permissions));
        assert!(PermissionChecker::has_any_permission(&["delete", "write"], &user_permissions));

        assert!(PermissionChecker::has_role(&["user", "admin"], &user_roles));
        assert!(!PermissionChecker::has_role(&["super_admin"], &user_roles));
        assert!(PermissionChecker::has_any_role(&["super_admin", "admin"], &user_roles));
    }
}