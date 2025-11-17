//! Rate limiting for API calls
//!
//! This module provides functionality for rate limiting API calls
//! to prevent abuse and ensure fair usage of resources.

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::Arc;
use thiserror::Error;

use crate::infrastructure::config::SecurityConfig;

/// Rate limit error types
#[derive(Debug, Error)]
pub enum RateLimitError {
    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again in {0} seconds.")]
    RateLimitExceeded(u64),
    
    /// Internal error
    #[error("Internal rate limiting error: {0}")]
    Internal(String),
}

/// Rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Number of requests allowed in the window
    pub limit: u32,
    
    /// Number of requests remaining in the window
    pub remaining: u32,
    
    /// Time when the rate limit resets
    pub reset_at: DateTime<Utc>,
    
    /// Time window in seconds
    pub window_seconds: u64,
}

/// Rate limit key types
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RateLimitKey {
    /// IP address
    IpAddress(IpAddr),
    
    /// API key
    ApiKey(String),
    
    /// User ID
    UserId(String),
    
    /// Custom key
    Custom(String),
}

impl ToString for RateLimitKey {
    fn to_string(&self) -> String {
        match self {
            RateLimitKey::IpAddress(ip) => format!("ip:{}", ip),
            RateLimitKey::ApiKey(key) => format!("api:{}", key),
            RateLimitKey::UserId(id) => format!("user:{}", id),
            RateLimitKey::Custom(key) => format!("custom:{}", key),
        }
    }
}

/// Rate limit bucket
#[derive(Debug, Clone)]
struct RateLimitBucket {
    /// Number of requests allowed in the window
    limit: u32,
    
    /// Number of requests made in the current window
    count: u32,
    
    /// Start time of the current window
    window_start: DateTime<Utc>,
    
    /// Time window in seconds
    window_seconds: u64,
}

impl RateLimitBucket {
    /// Create a new rate limit bucket
    fn new(limit: u32, window_seconds: u64) -> Self {
        Self {
            limit,
            count: 0,
            window_start: Utc::now(),
            window_seconds,
        }
    }
    
    /// Check if the bucket is expired
    fn is_expired(&self) -> bool {
        let now = Utc::now();
        let window_duration = Duration::seconds(self.window_seconds as i64);
        now > self.window_start + window_duration
    }
    
    /// Reset the bucket
    fn reset(&mut self) {
        self.count = 0;
        self.window_start = Utc::now();
    }
    
    /// Get the time until reset
    fn time_until_reset(&self) -> u64 {
        let now = Utc::now();
        let window_duration = Duration::seconds(self.window_seconds as i64);
        let reset_time = self.window_start + window_duration;
        
        if now >= reset_time {
            0
        } else {
            (reset_time - now).num_seconds() as u64
        }
    }
    
    /// Get rate limit information
    fn get_info(&self) -> RateLimitInfo {
        let window_duration = Duration::seconds(self.window_seconds as i64);
        let reset_at = self.window_start + window_duration;
        
        RateLimitInfo {
            limit: self.limit,
            remaining: self.limit.saturating_sub(self.count),
            reset_at,
            window_seconds: self.window_seconds,
        }
    }
}

/// Rate limiter for API calls
pub struct RateLimiter {
    /// Rate limit buckets
    buckets: DashMap<String, RateLimitBucket>,
    
    /// Default request limit
    default_limit: u32,
    
    /// Default time window in seconds
    default_window: u64,
    
    /// Custom limits for specific keys
    custom_limits: DashMap<String, (u32, u64)>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: &SecurityConfig) -> Self {
        Self {
            buckets: DashMap::new(),
            default_limit: config.rate_limit.requests,
            default_window: config.rate_limit.window_seconds,
            custom_limits: DashMap::new(),
        }
    }
    
    /// Set a custom limit for a specific key
    pub fn set_custom_limit(&self, key_prefix: &str, limit: u32, window_seconds: u64) {
        self.custom_limits.insert(key_prefix.to_string(), (limit, window_seconds));
    }
    
    /// Check if a request is allowed
    pub fn check(&self, key: &RateLimitKey) -> Result<RateLimitInfo, RateLimitError> {
        let key_str = key.to_string();
        
        // Get or create the bucket
        let mut bucket = self.get_or_create_bucket(&key_str);
        
        // Check if the bucket is expired
        if bucket.is_expired() {
            bucket.reset();
        }
        
        // Check if the limit is exceeded
        if bucket.count >= bucket.limit {
            return Err(RateLimitError::RateLimitExceeded(bucket.time_until_reset()));
        }
        
        // Increment the counter
        bucket.count += 1;
        
        // Update the bucket
        self.buckets.insert(key_str, bucket.clone());
        
        Ok(bucket.get_info())
    }
    
    /// Get rate limit information without incrementing the counter
    pub fn get_info(&self, key: &RateLimitKey) -> RateLimitInfo {
        let key_str = key.to_string();
        
        // Get or create the bucket
        let bucket = self.get_or_create_bucket(&key_str);
        
        bucket.get_info()
    }
    
    /// Reset rate limit for a key
    pub fn reset(&self, key: &RateLimitKey) {
        let key_str = key.to_string();
        
        if let Some(mut entry) = self.buckets.get_mut(&key_str) {
            entry.reset();
        }
    }
    
    /// Get or create a rate limit bucket
    fn get_or_create_bucket(&self, key: &str) -> RateLimitBucket {
        // Check if there's a custom limit for this key
        let (limit, window) = self.get_limits_for_key(key);
        
        // Get or create the bucket
        if let Some(bucket) = self.buckets.get(key) {
            // Update the bucket if the limits have changed
            if bucket.limit != limit || bucket.window_seconds != window {
                let mut new_bucket = bucket.clone();
                new_bucket.limit = limit;
                new_bucket.window_seconds = window;
                return new_bucket;
            }
            
            bucket.clone()
        } else {
            RateLimitBucket::new(limit, window)
        }
    }
    
    /// Get limits for a key
    fn get_limits_for_key(&self, key: &str) -> (u32, u64) {
        // Check if there's a custom limit for this key
        for entry in self.custom_limits.iter() {
            if key.starts_with(entry.key()) {
                return *entry.value();
            }
        }
        
        // Use default limits
        (self.default_limit, self.default_window)
    }
}

/// Rate limiter middleware
pub struct RateLimiterMiddleware {
    /// Rate limiter
    limiter: Arc<RateLimiter>,
}

impl RateLimiterMiddleware {
    /// Create a new rate limiter middleware
    pub fn new(limiter: Arc<RateLimiter>) -> Self {
        Self { limiter }
    }
    
    /// Check if a request is allowed
    pub fn check(&self, key: &RateLimitKey) -> Result<RateLimitInfo, RateLimitError> {
        self.limiter.check(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::thread::sleep;
    use std::time::Duration as StdDuration;
    
    fn create_test_config() -> SecurityConfig {
        use crate::infrastructure::config::RateLimitConfig;
        
        SecurityConfig {
            secret_key: "test-secret-key".to_string(),
            token_expiration: 3600,
            cors_origins: vec!["*".to_string()],
            rate_limit: RateLimitConfig {
                requests: 5,
                window_seconds: 1,
            },
        }
    }
    
    #[test]
    fn test_rate_limiting() {
        let config = create_test_config();
        let limiter = RateLimiter::new(&config);
        
        let key = RateLimitKey::IpAddress(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        
        // Make 5 requests (should succeed)
        for i in 0..5 {
            let result = limiter.check(&key);
            assert!(result.is_ok(), "Request {} should be allowed", i);
            
            let info = result.unwrap();
            assert_eq!(info.limit, 5);
            assert_eq!(info.remaining, 5 - (i + 1));
        }
        
        // Make another request (should fail)
        let result = limiter.check(&key);
        assert!(result.is_err(), "Request should be rate limited");
        
        if let Err(RateLimitError::RateLimitExceeded(seconds)) = result {
            assert!(seconds <= 1, "Rate limit should reset in <= 1 second");
        } else {
            panic!("Expected RateLimitExceeded error");
        }
        
        // Wait for the rate limit to reset
        sleep(StdDuration::from_secs(1));
        
        // Make another request (should succeed)
        let result = limiter.check(&key);
        assert!(result.is_ok(), "Request should be allowed after reset");
    }
    
    #[test]
    fn test_custom_limits() {
        let config = create_test_config();
        let limiter = RateLimiter::new(&config);
        
        // Set a custom limit for API keys
        limiter.set_custom_limit("api:", 2, 1);
        
        let ip_key = RateLimitKey::IpAddress(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        let api_key = RateLimitKey::ApiKey("test-key".to_string());
        
        // IP address should have the default limit (5)
        let info = limiter.get_info(&ip_key);
        assert_eq!(info.limit, 5);
        
        // API key should have the custom limit (2)
        let info = limiter.get_info(&api_key);
        assert_eq!(info.limit, 2);
        
        // Make 2 requests with the API key (should succeed)
        for _ in 0..2 {
            let result = limiter.check(&api_key);
            assert!(result.is_ok());
        }
        
        // Make another request with the API key (should fail)
        let result = limiter.check(&api_key);
        assert!(result.is_err());
    }
}