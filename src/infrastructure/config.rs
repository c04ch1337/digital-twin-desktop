use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// LLM configuration
    pub llm: LLMConfig,
    
    /// Tool configuration
    pub tools: ToolConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database file path
    pub path: PathBuf,
    
    /// Maximum connections in pool
    pub max_connections: u32,
    
    /// Enable foreign key constraints
    pub foreign_keys: bool,
    
    /// Enable WAL mode
    pub wal_mode: bool,
}

/// LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Default provider
    pub default_provider: String,
    
    /// OpenAI configuration
    pub openai: OpenAIConfig,
    
    /// Anthropic configuration
    pub anthropic: AnthropicConfig,
}

/// OpenAI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// API key
    pub api_key: Option<String>,
    
    /// Organization ID
    pub organization_id: Option<String>,
    
    /// Default model
    pub default_model: String,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Anthropic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    /// API key
    pub api_key: Option<String>,
    
    /// Default model
    pub default_model: String,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    /// File tool configuration
    pub file: FileToolConfig,
    
    /// Web tool configuration
    pub web: WebToolConfig,
    
    /// Modbus tool configuration
    pub modbus: ModbusToolConfig,
    
    /// MQTT tool configuration
    pub mqtt: MqttToolConfig,
}

/// File tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileToolConfig {
    /// Base directory for file operations
    pub base_path: PathBuf,
    
    /// Maximum file size in bytes
    pub max_file_size: u64,
    
    /// Allowed file extensions
    pub allowed_extensions: Vec<String>,
}

/// Web tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebToolConfig {
    /// Maximum response size in bytes
    pub max_response_size: usize,
    
    /// Allowed domains
    pub allowed_domains: Vec<String>,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Modbus tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusToolConfig {
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// Maximum retries
    pub max_retries: u32,
}

/// MQTT tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttToolConfig {
    /// Broker URL
    pub broker_url: String,
    
    /// Broker port
    pub broker_port: u16,
    
    /// Client ID
    pub client_id: String,
    
    /// Username
    pub username: Option<String>,
    
    /// Password
    pub password: Option<String>,
    
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Secret key for JWT tokens
    pub secret_key: String,
    
    /// Token expiration time in seconds
    pub token_expiration: u64,
    
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
    
    /// API key configuration
    pub api_keys: ApiKeyConfig,
    
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    
    /// Sandbox configuration
    pub sandbox: SandboxConfig,
    
    /// Permission configuration
    pub permissions: PermissionConfig,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Enable API key authentication
    pub enabled: bool,
    
    /// API key expiration time in seconds (0 for no expiration)
    pub expiration: u64,
    
    /// Maximum number of API keys per user
    pub max_keys_per_user: u32,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: String,
    
    /// Key derivation iterations
    pub key_derivation_iterations: u32,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Enable sandbox for tool execution
    pub enabled: bool,
    
    /// Maximum execution time in seconds
    pub max_execution_time: u64,
    
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    
    /// Allow network access
    pub allow_network: bool,
    
    /// Allow file system access
    pub allow_filesystem: bool,
}

/// Permission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    /// Default role for new users
    pub default_role: String,
    
    /// Enable strict permission checking
    pub strict_checking: bool,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Number of requests
    pub requests: u32,
    
    /// Time window in seconds
    pub window_seconds: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    
    /// Log file path
    pub file_path: Option<PathBuf>,
    
    /// Enable JSON formatting
    pub json_format: bool,
}

impl AppConfig {
    /// Load configuration from files and environment
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // Start with default values
            .set_default("database.max_connections", 5)?
            .set_default("database.foreign_keys", true)?
            .set_default("database.wal_mode", true)?
            .set_default("llm.default_provider", "openai")?
            .set_default("llm.openai.default_model", "gpt-4")?
            .set_default("llm.openai.timeout_seconds", 30)?
            .set_default("llm.anthropic.default_model", "claude-2")?
            .set_default("llm.anthropic.timeout_seconds", 30)?
            .set_default("tools.file.max_file_size", 1024 * 1024)?
            .set_default("tools.web.max_response_size", 1024 * 1024)?
            .set_default("tools.web.timeout_seconds", 30)?
            .set_default("tools.modbus.timeout_seconds", 5)?
            .set_default("tools.modbus.max_retries", 3)?
            .set_default("tools.mqtt.broker_port", 1883)?
            .set_default("tools.mqtt.timeout_seconds", 30)?
            .set_default("security.token_expiration", 3600)?
            .set_default("logging.level", "info")?
            .set_default("logging.json_format", false)?;

        // Add configuration from files
        if let Ok(env) = std::env::var("APP_ENV") {
            builder = builder.add_source(File::with_name(&format!("config/{}", env)));
        }
        builder = builder.add_source(File::with_name("config/default"));

        // Add configuration from environment variables
        builder = builder.add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .try_parsing(true)
        );

        // Build and deserialize configuration
        builder.build()?.try_deserialize()
    }

    /// Get database configuration
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    /// Get LLM configuration
    pub fn llm(&self) -> &LLMConfig {
        &self.llm
    }

    /// Get tool configuration
    pub fn tools(&self) -> &ToolConfig {
        &self.tools
    }

    /// Get security configuration
    pub fn security(&self) -> &SecurityConfig {
        &self.security
    }

    /// Get logging configuration
    pub fn logging(&self) -> &LoggingConfig {
        &self.logging
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                path: "digital_twin.db".into(),
                max_connections: 5,
                foreign_keys: true,
                wal_mode: true,
            },
            llm: LLMConfig {
                default_provider: "openai".to_string(),
                openai: OpenAIConfig {
                    api_key: None,
                    organization_id: None,
                    default_model: "gpt-4".to_string(),
                    timeout_seconds: 30,
                },
                anthropic: AnthropicConfig {
                    api_key: None,
                    default_model: "claude-2".to_string(),
                    timeout_seconds: 30,
                },
            },
            tools: ToolConfig {
                file: FileToolConfig {
                    base_path: ".".into(),
                    max_file_size: 1024 * 1024,
                    allowed_extensions: vec![],
                },
                web: WebToolConfig {
                    max_response_size: 1024 * 1024,
                    allowed_domains: vec![],
                    timeout_seconds: 30,
                },
                modbus: ModbusToolConfig {
                    timeout_seconds: 5,
                    max_retries: 3,
                },
                mqtt: MqttToolConfig {
                    broker_url: "localhost".to_string(),
                    broker_port: 1883,
                    client_id: "digital-twin".to_string(),
                    username: None,
                    password: None,
                    timeout_seconds: 30,
                },
            },
            security: SecurityConfig {
                secret_key: "change-me".to_string(),
                token_expiration: 3600,
                cors_origins: vec!["*".to_string()],
                rate_limit: RateLimitConfig {
                    requests: 100,
                    window_seconds: 60,
                },
                api_keys: ApiKeyConfig {
                    enabled: true,
                    expiration: 30 * 24 * 3600, // 30 days
                    max_keys_per_user: 5,
                },
                encryption: EncryptionConfig {
                    algorithm: "ChaCha20-Poly1305".to_string(),
                    key_derivation_iterations: 10000,
                },
                sandbox: SandboxConfig {
                    enabled: true,
                    max_execution_time: 30,
                    max_memory_mb: 256,
                    allow_network: false,
                    allow_filesystem: false,
                },
                permissions: PermissionConfig {
                    default_role: "user".to_string(),
                    strict_checking: true,
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: None,
                json_format: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.database.max_connections, 5);
        assert_eq!(config.llm.default_provider, "openai");
        assert_eq!(config.tools.modbus.max_retries, 3);
        assert_eq!(config.security.token_expiration, 3600);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_config_load() {
        std::env::set_var("APP_DATABASE__MAX_CONNECTIONS", "10");
        std::env::set_var("APP_LLM__DEFAULT_PROVIDER", "anthropic");

        let config = AppConfig::load().unwrap();
        assert_eq!(config.database.max_connections, 10);
        assert_eq!(config.llm.default_provider, "anthropic");
    }
}