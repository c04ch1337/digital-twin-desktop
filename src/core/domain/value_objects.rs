//! Value objects for the Digital Twin Desktop domain.
//!
//! This module defines strongly-typed value objects that encapsulate
//! domain concepts, providing type safety, validation, and domain-specific
//! behavior beyond simple primitive types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::core::domain::errors::{DomainError, ValidationError};

/// Macro to implement common ID value object functionality
macro_rules! impl_id_value_object {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(Uuid);

        impl $name {
            /// Create a new random ID
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Create from an existing UUID
            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Get the inner UUID value
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Convert to UUID
            pub fn into_uuid(self) -> Uuid {
                self.0
            }

            /// Parse from a string
            pub fn parse_str(input: &str) -> Result<Self, DomainError> {
                Uuid::parse_str(input)
                    .map(Self)
                    .map_err(|_| ValidationError::InvalidFormat {
                        field: stringify!($name).to_string(),
                        reason: "Invalid UUID format".to_string(),
                    }.into())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl AsRef<Uuid> for $name {
            fn as_ref(&self) -> &Uuid {
                &self.0
            }
        }
    };
}

// Implement ID value objects
impl_id_value_object!(ConversationId);
impl_id_value_object!(MessageId);
impl_id_value_object!(AgentId);
impl_id_value_object!(TwinId);
impl_id_value_object!(SensorDataId);
impl_id_value_object!(ToolId);
impl_id_value_object!(ExecutionId);

/// Email address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new email address with validation
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email = email.trim().to_lowercase();
        
        // Basic email validation
        if email.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "email".to_string(),
            }.into());
        }
        
        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidEmail {
                field: "email".to_string(),
                email: email.clone(),
            }.into());
        }
        
        // More detailed validation could be added here
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(ValidationError::InvalidEmail {
                field: "email".to_string(),
                email: email.clone(),
            }.into());
        }
        
        Ok(Self(email))
    }
    
    /// Get the email address as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Get the domain part of the email
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
    
    /// Get the local part of the email
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = DomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// URL value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url(String);

impl Url {
    /// Create a new URL with validation
    pub fn new(url: &str) -> Result<Self, DomainError> {
        let url = url.trim();
        
        if url.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "url".to_string(),
            }.into());
        }
        
        // Basic URL validation - check for protocol
        if !url.starts_with("http://") && !url.starts_with("https://") 
            && !url.starts_with("ws://") && !url.starts_with("wss://") {
            return Err(ValidationError::InvalidUrl {
                field: "url".to_string(),
                url: url.to_string(),
            }.into());
        }
        
        Ok(Self(url.to_string()))
    }
    
    /// Get the URL as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Get the protocol (scheme) of the URL
    pub fn protocol(&self) -> &str {
        self.0.split("://").next().unwrap_or("")
    }
    
    /// Get the host part of the URL
    pub fn host(&self) -> Option<&str> {
        self.0.split("://").nth(1)
            .and_then(|s| s.split('/').next())
            .and_then(|s| s.split(':').next())
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Percentage value object (0.0 to 100.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Percentage(f32);

impl Percentage {
    /// Create a new percentage value
    pub fn new(value: f32) -> Result<Self, DomainError> {
        if !(0.0..=100.0).contains(&value) {
            return Err(ValidationError::out_of_range(
                "percentage",
                0.0,
                100.0,
                value,
            ).into());
        }
        Ok(Self(value))
    }
    
    /// Create from a ratio (0.0 to 1.0)
    pub fn from_ratio(ratio: f32) -> Result<Self, DomainError> {
        Self::new(ratio * 100.0)
    }
    
    /// Get the percentage value
    pub fn value(&self) -> f32 {
        self.0
    }
    
    /// Get as a ratio (0.0 to 1.0)
    pub fn as_ratio(&self) -> f32 {
        self.0 / 100.0
    }
    
    /// Create a 0% value
    pub fn zero() -> Self {
        Self(0.0)
    }
    
    /// Create a 100% value
    pub fn full() -> Self {
        Self(100.0)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}

/// Temperature value object with unit conversions
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature {
    celsius: f64,
}

impl Temperature {
    /// Create from Celsius
    pub fn from_celsius(value: f64) -> Result<Self, DomainError> {
        // Validate against absolute zero
        if value < -273.15 {
            return Err(ValidationError::out_of_range(
                "temperature",
                -273.15,
                f64::MAX,
                value,
            ).into());
        }
        Ok(Self { celsius: value })
    }
    
    /// Create from Fahrenheit
    pub fn from_fahrenheit(value: f64) -> Result<Self, DomainError> {
        let celsius = (value - 32.0) * 5.0 / 9.0;
        Self::from_celsius(celsius)
    }
    
    /// Create from Kelvin
    pub fn from_kelvin(value: f64) -> Result<Self, DomainError> {
        let celsius = value - 273.15;
        Self::from_celsius(celsius)
    }
    
    /// Get temperature in Celsius
    pub fn as_celsius(&self) -> f64 {
        self.celsius
    }
    
    /// Get temperature in Fahrenheit
    pub fn as_fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
    
    /// Get temperature in Kelvin
    pub fn as_kelvin(&self) -> f64 {
        self.celsius + 273.15
    }
}

/// Data size value object with unit conversions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DataSize {
    bytes: u64,
}

impl DataSize {
    /// Create from bytes
    pub fn from_bytes(bytes: u64) -> Self {
        Self { bytes }
    }
    
    /// Create from kilobytes
    pub fn from_kilobytes(kb: f64) -> Self {
        Self {
            bytes: (kb * 1_024.0) as u64,
        }
    }
    
    /// Create from megabytes
    pub fn from_megabytes(mb: f64) -> Self {
        Self {
            bytes: (mb * 1_024.0 * 1_024.0) as u64,
        }
    }
    
    /// Create from gigabytes
    pub fn from_gigabytes(gb: f64) -> Self {
        Self {
            bytes: (gb * 1_024.0 * 1_024.0 * 1_024.0) as u64,
        }
    }
    
    /// Get size in bytes
    pub fn as_bytes(&self) -> u64 {
        self.bytes
    }
    
    /// Get size in kilobytes
    pub fn as_kilobytes(&self) -> f64 {
        self.bytes as f64 / 1_024.0
    }
    
    /// Get size in megabytes
    pub fn as_megabytes(&self) -> f64 {
        self.bytes as f64 / (1_024.0 * 1_024.0)
    }
    
    /// Get size in gigabytes
    pub fn as_gigabytes(&self) -> f64 {
        self.bytes as f64 / (1_024.0 * 1_024.0 * 1_024.0)
    }
}

impl fmt::Display for DataSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bytes < 1_024 {
            write!(f, "{} B", self.bytes)
        } else if self.bytes < 1_024 * 1_024 {
            write!(f, "{:.2} KB", self.as_kilobytes())
        } else if self.bytes < 1_024 * 1_024 * 1_024 {
            write!(f, "{:.2} MB", self.as_megabytes())
        } else {
            write!(f, "{:.2} GB", self.as_gigabytes())
        }
    }
}

/// Time window value object for defining time periods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindow {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl TimeWindow {
    /// Create a new time window
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, DomainError> {
        if start >= end {
            return Err(ValidationError::InvalidFormat {
                field: "time_window".to_string(),
                reason: "Start time must be before end time".to_string(),
            }.into());
        }
        Ok(Self { start, end })
    }
    
    /// Create a time window from now for a given duration
    pub fn from_now_with_duration(duration: chrono::Duration) -> Self {
        let start = Utc::now();
        let end = start + duration;
        Self { start, end }
    }
    
    /// Get the start time
    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }
    
    /// Get the end time
    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }
    
    /// Get the duration of the window
    pub fn duration(&self) -> chrono::Duration {
        self.end - self.start
    }
    
    /// Check if a timestamp is within this window
    pub fn contains(&self, timestamp: &DateTime<Utc>) -> bool {
        *timestamp >= self.start && *timestamp <= self.end
    }
    
    /// Check if another window overlaps with this one
    pub fn overlaps(&self, other: &TimeWindow) -> bool {
        self.start < other.end && other.start < self.end
    }
}

/// Semantic version value object
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
}

impl Version {
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
        }
    }
    
    /// Create a version with pre-release
    pub fn with_pre_release(
        major: u32,
        minor: u32,
        patch: u32,
        pre_release: &str,
    ) -> Result<Self, DomainError> {
        if pre_release.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "pre_release".to_string(),
            }.into());
        }
        
        Ok(Self {
            major,
            minor,
            patch,
            pre_release: Some(pre_release.to_string()),
        })
    }
    
    /// Parse from a string (e.g., "1.2.3" or "1.2.3-beta")
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        let parts: Vec<&str> = s.split('-').collect();
        let version_parts: Vec<&str> = parts[0].split('.').collect();
        
        if version_parts.len() != 3 {
            return Err(ValidationError::InvalidFormat {
                field: "version".to_string(),
                reason: "Version must be in format X.Y.Z".to_string(),
            }.into());
        }
        
        let major = version_parts[0].parse::<u32>()
            .map_err(|_| ValidationError::InvalidFormat {
                field: "version.major".to_string(),
                reason: "Major version must be a number".to_string(),
            })?;
            
        let minor = version_parts[1].parse::<u32>()
            .map_err(|_| ValidationError::InvalidFormat {
                field: "version.minor".to_string(),
                reason: "Minor version must be a number".to_string(),
            })?;
            
        let patch = version_parts[2].parse::<u32>()
            .map_err(|_| ValidationError::InvalidFormat {
                field: "version.patch".to_string(),
                reason: "Patch version must be a number".to_string(),
            })?;
        
        let pre_release = if parts.len() > 1 {
            Some(parts[1..].join("-"))
        } else {
            None
        };
        
        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }
    
    /// Check if this is a pre-release version
    pub fn is_pre_release(&self) -> bool {
        self.pre_release.is_some()
    }
    
    /// Check if this version is compatible with another (same major version)
    pub fn is_compatible_with(&self, other: &Version) -> bool {
        self.major == other.major
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = DomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_id_value_objects() {
        let conv_id = ConversationId::new();
        let uuid = conv_id.as_uuid();
        assert_eq!(ConversationId::from_uuid(*uuid), conv_id);
        
        let parsed = ConversationId::parse_str(&conv_id.to_string()).unwrap();
        assert_eq!(parsed, conv_id);
    }
    
    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("TEST@EXAMPLE.COM").is_ok());
        assert!(Email::new("  test@example.com  ").is_ok());
        
        assert!(Email::new("").is_err());
        assert!(Email::new("test").is_err());
        assert!(Email::new("test@").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("test@example").is_err());
        
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.local_part(), "user");
        assert_eq!(email.domain(), "example.com");
    }
    
    #[test]
    fn test_percentage() {
        assert!(Percentage::new(0.0).is_ok());
        assert!(Percentage::new(50.0).is_ok());
        assert!(Percentage::new(100.0).is_ok());
        
        assert!(Percentage::new(-1.0).is_err());
        assert!(Percentage::new(101.0).is_err());
        
        let pct = Percentage::new(75.0).unwrap();
        assert_eq!(pct.value(), 75.0);
        assert_eq!(pct.as_ratio(), 0.75);
        
        let from_ratio = Percentage::from_ratio(0.25).unwrap();
        assert_eq!(from_ratio.value(), 25.0);
    }
    
    #[test]
    fn test_temperature_conversions() {
        let temp = Temperature::from_celsius(0.0).unwrap();
        assert_eq!(temp.as_celsius(), 0.0);
        assert_eq!(temp.as_fahrenheit(), 32.0);
        assert_eq!(temp.as_kelvin(), 273.15);
        
        let temp = Temperature::from_fahrenheit(32.0).unwrap();
        assert!((temp.as_celsius() - 0.0).abs() < 0.01);
        
        let temp = Temperature::from_kelvin(273.15).unwrap();
        assert!((temp.as_celsius() - 0.0).abs() < 0.01);
        
        assert!(Temperature::from_celsius(-300.0).is_err());
    }
    
    #[test]
    fn test_data_size() {
        let size = DataSize::from_bytes(1_024);
        assert_eq!(size.as_bytes(), 1_024);
        assert_eq!(size.as_kilobytes(), 1.0);
        
        let size = DataSize::from_megabytes(1.5);
        assert_eq!(size.as_megabytes(), 1.5);
        
        assert_eq!(DataSize::from_bytes(512).to_string(), "512 B");
        assert_eq!(DataSize::from_kilobytes(2.5).to_string(), "2.50 KB");
        assert_eq!(DataSize::from_megabytes(10.0).to_string(), "10.00 MB");
        assert_eq!(DataSize::from_gigabytes(1.5).to_string(), "1.50 GB");
    }
    
    #[test]
    fn test_time_window() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);
        
        let window = TimeWindow::new(start, end).unwrap();
        assert_eq!(window.duration(), chrono::Duration::hours(1));
        
        let mid = start + chrono::Duration::minutes(30);
        assert!(window.contains(&mid));
        assert!(!window.contains(&(start - chrono::Duration::minutes(1))));
        
        assert!(TimeWindow::new(end, start).is_err());
    }
    
    #[test]
    fn test_version() {
        let v1 = Version::new(1, 2, 3);
        assert_eq!(v1.to_string(), "1.2.3");
        
        let v2 = Version::with_pre_release(1, 2, 3, "beta").unwrap();
        assert_eq!(v2.to_string(), "1.2.3-beta");
        assert!(v2.is_pre_release());
        
        let parsed = Version::parse("2.0.1").unwrap();
        assert_eq!(parsed.to_string(), "2.0.1");
        
        let parsed = Version::parse("2.0.1-alpha.1").unwrap();
        assert_eq!(parsed.to_string(), "2.0.1-alpha.1");
        
        assert!(v1.is_compatible_with(&Version::new(1, 3, 0)));
        assert!(!v1.is_compatible_with(&Version::new(2, 0, 0)));
        
        assert!(Version::parse("1.2").is_err());
        assert!(Version::parse("a.b.c").is_err());
    }
}