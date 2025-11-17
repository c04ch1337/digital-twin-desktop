//! Sandboxed execution environment for tools
//!
//! This module provides a sandboxed execution environment for running
//! tools safely with resource limitations and isolation.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::infrastructure::security::permissions::PermissionManager;

/// Sandbox error types
#[derive(Debug, Error)]
pub enum SandboxError {
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Resource limit exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    /// Execution timeout
    #[error("Execution timeout after {0} seconds")]
    Timeout(u64),
    
    /// Command execution error
    #[error("Command execution error: {0}")]
    CommandExecution(String),
    
    /// Invalid configuration
    #[error("Invalid sandbox configuration: {0}")]
    InvalidConfiguration(String),
    
    /// Internal error
    #[error("Internal sandbox error: {0}")]
    Internal(String),
}

/// Resource limits for sandboxed execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum execution time in seconds
    pub max_execution_time: u64,
    
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    
    /// Maximum CPU usage in percentage (0-100)
    pub max_cpu_percent: u8,
    
    /// Maximum disk usage in MB
    pub max_disk_mb: u64,
    
    /// Maximum number of processes
    pub max_processes: u32,
    
    /// Maximum number of file descriptors
    pub max_file_descriptors: u32,
    
    /// Maximum network bandwidth in KB/s
    pub max_network_kb_per_sec: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_execution_time: 30,
            max_memory_mb: 256,
            max_cpu_percent: 50,
            max_disk_mb: 100,
            max_processes: 10,
            max_file_descriptors: 100,
            max_network_kb_per_sec: 1024,
        }
    }
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Resource limits
    pub resource_limits: ResourceLimits,
    
    /// Allowed environment variables
    pub allowed_env_vars: Vec<String>,
    
    /// Allowed directories for read access
    pub allowed_read_dirs: Vec<PathBuf>,
    
    /// Allowed directories for write access
    pub allowed_write_dirs: Vec<PathBuf>,
    
    /// Allowed network hosts
    pub allowed_network_hosts: Vec<String>,
    
    /// Allow network access
    pub allow_network: bool,
    
    /// Allow file system access
    pub allow_filesystem: bool,
    
    /// Allow process execution
    pub allow_process_execution: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            resource_limits: ResourceLimits::default(),
            allowed_env_vars: vec![
                "PATH".to_string(),
                "HOME".to_string(),
                "USER".to_string(),
                "TEMP".to_string(),
                "TMP".to_string(),
            ],
            allowed_read_dirs: vec![],
            allowed_write_dirs: vec![],
            allowed_network_hosts: vec![],
            allow_network: false,
            allow_filesystem: false,
            allow_process_execution: false,
        }
    }
}

/// Execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Memory usage in MB
    pub memory_usage_mb: u64,
    
    /// CPU usage in percentage
    pub cpu_usage_percent: f32,
    
    /// Disk usage in MB
    pub disk_usage_mb: u64,
    
    /// Network usage in KB
    pub network_usage_kb: u64,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Exit code
    pub exit_code: i32,
    
    /// Standard output
    pub stdout: String,
    
    /// Standard error
    pub stderr: String,
    
    /// Execution statistics
    pub stats: ExecutionStats,
}

/// Sandbox for executing tools safely
pub struct Sandbox {
    /// Sandbox configuration
    config: SandboxConfig,
    
    /// Permission manager
    permission_manager: Arc<PermissionManager>,
    
    /// Active executions
    active_executions: Mutex<HashMap<String, Instant>>,
}

impl Sandbox {
    /// Create a new sandbox
    pub fn new(config: SandboxConfig, permission_manager: Arc<PermissionManager>) -> Self {
        Self {
            config,
            permission_manager,
            active_executions: Mutex::new(HashMap::new()),
        }
    }
    
    /// Execute a command in the sandbox
    pub async fn execute_command(&self, command: &str, args: &[&str], user_id: &str, tool_id: &str) -> Result<ExecutionResult, SandboxError> {
        // Check permissions
        if !self.permission_manager.can_execute_tool(user_id, tool_id) {
            return Err(SandboxError::PermissionDenied(format!("User {} does not have permission to execute tool {}", user_id, tool_id)));
        }
        
        // Check if process execution is allowed
        if !self.config.allow_process_execution {
            return Err(SandboxError::PermissionDenied("Process execution is not allowed in this sandbox".to_string()));
        }
        
        // Track execution
        let execution_id = format!("{}:{}", user_id, Uuid::new_v4());
        self.track_execution(&execution_id).await?;
        
        // Execute the command with resource limits
        let result = self.execute_with_limits(command, args).await;
        
        // Cleanup
        self.cleanup_execution(&execution_id).await;
        
        result
    }
    
    /// Execute a function in the sandbox
    pub async fn execute_function<F, T>(&self, func: F, user_id: &str, tool_id: &str) -> Result<T, SandboxError>
    where
        F: FnOnce() -> Result<T, anyhow::Error> + Send + 'static,
        T: Send + 'static,
    {
        // Check permissions
        if !self.permission_manager.can_execute_tool(user_id, tool_id) {
            return Err(SandboxError::PermissionDenied(format!("User {} does not have permission to execute tool {}", user_id, tool_id)));
        }
        
        // Track execution
        let execution_id = format!("{}:{}", user_id, Uuid::new_v4());
        self.track_execution(&execution_id).await?;
        
        // Execute the function with a timeout
        let max_time = Duration::from_secs(self.config.resource_limits.max_execution_time);
        let result = timeout(max_time, async move {
            tokio::task::spawn_blocking(move || func())
                .await
                .map_err(|e| SandboxError::Internal(format!("Task join error: {}", e)))?
                .map_err(|e| SandboxError::CommandExecution(e.to_string()))
        })
        .await
        .map_err(|_| SandboxError::Timeout(self.config.resource_limits.max_execution_time))?;
        
        // Cleanup
        self.cleanup_execution(&execution_id).await;
        
        result
    }
    
    /// Check if a file path is allowed for read access
    pub fn is_read_allowed(&self, path: &PathBuf) -> bool {
        if !self.config.allow_filesystem {
            return false;
        }
        
        self.config.allowed_read_dirs.iter().any(|dir| path.starts_with(dir))
    }
    
    /// Check if a file path is allowed for write access
    pub fn is_write_allowed(&self, path: &PathBuf) -> bool {
        if !self.config.allow_filesystem {
            return false;
        }
        
        self.config.allowed_write_dirs.iter().any(|dir| path.starts_with(dir))
    }
    
    /// Check if a network host is allowed
    pub fn is_network_allowed(&self, host: &str) -> bool {
        if !self.config.allow_network {
            return false;
        }
        
        self.config.allowed_network_hosts.is_empty() || self.config.allowed_network_hosts.iter().any(|h| host == h)
    }
    
    /// Track an execution
    async fn track_execution(&self, execution_id: &str) -> Result<(), SandboxError> {
        let mut executions = self.active_executions.lock().await;
        
        // Check if we've reached the maximum number of processes
        if executions.len() >= self.config.resource_limits.max_processes as usize {
            return Err(SandboxError::ResourceLimitExceeded("Maximum number of processes reached".to_string()));
        }
        
        executions.insert(execution_id.to_string(), Instant::now());
        Ok(())
    }
    
    /// Cleanup an execution
    async fn cleanup_execution(&self, execution_id: &str) {
        let mut executions = self.active_executions.lock().await;
        executions.remove(execution_id);
    }
    
    /// Execute a command with resource limits
    async fn execute_with_limits(&self, command: &str, args: &[&str]) -> Result<ExecutionResult, SandboxError> {
        let start_time = Instant::now();
        
        // Prepare the command
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // Set allowed environment variables
        let mut env_vars = std::env::vars()
            .filter(|(key, _)| self.config.allowed_env_vars.contains(key))
            .collect::<HashMap<String, String>>();
        
        // Add resource limits as environment variables
        env_vars.insert("SANDBOX_MAX_MEMORY_MB".to_string(), self.config.resource_limits.max_memory_mb.to_string());
        env_vars.insert("SANDBOX_MAX_CPU_PERCENT".to_string(), self.config.resource_limits.max_cpu_percent.to_string());
        env_vars.insert("SANDBOX_MAX_DISK_MB".to_string(), self.config.resource_limits.max_disk_mb.to_string());
        
        cmd.envs(env_vars);
        
        // Execute the command with a timeout
        let max_time = Duration::from_secs(self.config.resource_limits.max_execution_time);
        let output = timeout(max_time, async {
            cmd.output()
                .map_err(|e| SandboxError::CommandExecution(e.to_string()))
        })
        .await
        .map_err(|_| SandboxError::Timeout(self.config.resource_limits.max_execution_time))??;
        
        let execution_time = start_time.elapsed();
        
        // Create execution statistics (in a real implementation, these would be measured)
        let stats = ExecutionStats {
            execution_time_ms: execution_time.as_millis() as u64,
            memory_usage_mb: 0, // Would be measured in a real implementation
            cpu_usage_percent: 0.0, // Would be measured in a real implementation
            disk_usage_mb: 0, // Would be measured in a real implementation
            network_usage_kb: 0, // Would be measured in a real implementation
        };
        
        // Create execution result
        let result = ExecutionResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            stats,
        };
        
        Ok(result)
    }
}

// Import for the UUID generation
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[tokio::test]
    async fn test_sandbox_execution() {
        use crate::infrastructure::security::permissions::PermissionManager;
        
        // Create a permission manager
        let permission_manager = Arc::new(PermissionManager::new());
        
        // Create a sandbox configuration
        let mut config = SandboxConfig::default();
        config.allow_process_execution = true;
        
        // Create a sandbox
        let sandbox = Sandbox::new(config, permission_manager);
        
        // Grant permission to the user
        sandbox.permission_manager.grant_tool_permission("user1", "echo").unwrap();
        
        // Execute a command
        let result = sandbox.execute_command("echo", &["Hello, world!"], "user1", "echo").await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout.trim(), "Hello, world!");
    }
    
    #[tokio::test]
    async fn test_sandbox_permission_denied() {
        use crate::infrastructure::security::permissions::PermissionManager;
        
        // Create a permission manager
        let permission_manager = Arc::new(PermissionManager::new());
        
        // Create a sandbox configuration
        let mut config = SandboxConfig::default();
        config.allow_process_execution = true;
        
        // Create a sandbox
        let sandbox = Sandbox::new(config, permission_manager);
        
        // Execute a command without permission
        let result = sandbox.execute_command("echo", &["Hello, world!"], "user1", "echo").await;
        
        assert!(result.is_err());
        match result {
            Err(SandboxError::PermissionDenied(_)) => (),
            _ => panic!("Expected PermissionDenied error"),
        }
    }
    
    #[tokio::test]
    async fn test_sandbox_timeout() {
        use crate::infrastructure::security::permissions::PermissionManager;
        
        // Create a permission manager
        let permission_manager = Arc::new(PermissionManager::new());
        
        // Create a sandbox configuration with a short timeout
        let mut config = SandboxConfig::default();
        config.allow_process_execution = true;
        config.resource_limits.max_execution_time = 1;
        
        // Create a sandbox
        let sandbox = Sandbox::new(config, permission_manager);
        
        // Grant permission to the user
        sandbox.permission_manager.grant_tool_permission("user1", "sleep").unwrap();
        
        // Execute a command that will timeout
        let result = sandbox.execute_command("sleep", &["5"], "user1", "sleep").await;
        
        assert!(result.is_err());
        match result {
            Err(SandboxError::Timeout(_)) => (),
            _ => panic!("Expected Timeout error"),
        }
    }
    
    #[test]
    fn test_file_access_permissions() {
        use crate::infrastructure::security::permissions::PermissionManager;
        
        // Create a permission manager
        let permission_manager = Arc::new(PermissionManager::new());
        
        // Create a sandbox configuration
        let mut config = SandboxConfig::default();
        config.allow_filesystem = true;
        config.allowed_read_dirs = vec![PathBuf::from("/tmp"), PathBuf::from("/home/user/data")];
        config.allowed_write_dirs = vec![PathBuf::from("/tmp")];
        
        // Create a sandbox
        let sandbox = Sandbox::new(config, permission_manager);
        
        // Test read permissions
        assert!(sandbox.is_read_allowed(&PathBuf::from("/tmp/file.txt")));
        assert!(sandbox.is_read_allowed(&PathBuf::from("/home/user/data/file.txt")));
        assert!(!sandbox.is_read_allowed(&PathBuf::from("/etc/passwd")));
        
        // Test write permissions
        assert!(sandbox.is_write_allowed(&PathBuf::from("/tmp/file.txt")));
        assert!(!sandbox.is_write_allowed(&PathBuf::from("/home/user/data/file.txt")));
        assert!(!sandbox.is_write_allowed(&PathBuf::from("/etc/passwd")));
    }
    
    #[test]
    fn test_network_permissions() {
        use crate::infrastructure::security::permissions::PermissionManager;
        
        // Create a permission manager
        let permission_manager = Arc::new(PermissionManager::new());
        
        // Create a sandbox configuration
        let mut config = SandboxConfig::default();
        config.allow_network = true;
        config.allowed_network_hosts = vec!["api.example.com".to_string(), "localhost".to_string()];
        
        // Create a sandbox
        let sandbox = Sandbox::new(config, permission_manager);
        
        // Test network permissions
        assert!(sandbox.is_network_allowed("api.example.com"));
        assert!(sandbox.is_network_allowed("localhost"));
        assert!(!sandbox.is_network_allowed("evil.com"));
        
        // Test with empty allowed hosts (allow all)
        let mut config = SandboxConfig::default();
        config.allow_network = true;
        config.allowed_network_hosts = vec![];
        
        let sandbox = Sandbox::new(config, permission_manager);
        assert!(sandbox.is_network_allowed("any.host.com"));
    }
}