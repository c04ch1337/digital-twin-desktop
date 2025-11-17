//! Tool executor implementations.

mod file_tool;
mod web_tool;
mod modbus_tool;
mod mqtt_tool;
mod twin_tool;

pub use file_tool::FileToolExecutor;
pub use web_tool::WebToolExecutor;
pub use modbus_tool::ModbusToolExecutor;
pub use mqtt_tool::MqttToolExecutor;
pub use twin_tool::TwinToolExecutor;

use async_trait::async_trait;
use std::{sync::Arc, collections::HashMap};
use anyhow::Result;

use crate::core::domain::{
    models::ToolType,
    traits::tool_executor::{
        ToolExecutor, ExecutorResult, ExecutorError,
        ToolExecutorFactory, ToolExecutorRegistry,
    },
};

/// Default tool executor factory
pub struct DefaultToolExecutorFactory {
    registry: Arc<DefaultToolExecutorRegistry>,
}

impl DefaultToolExecutorFactory {
    /// Create a new tool executor factory
    pub fn new(registry: Arc<DefaultToolExecutorRegistry>) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl ToolExecutorFactory for DefaultToolExecutorFactory {
    async fn create_executor(
        &self,
        tool_type: &ToolType,
    ) -> ExecutorResult<Box<dyn ToolExecutor>> {
        self.registry.get_executor(&tool_type.to_string()).await
    }

    fn available_executors(&self) -> Vec<String> {
        self.registry.list_executors()
    }
}

/// Default tool executor registry
pub struct DefaultToolExecutorRegistry {
    executors: HashMap<String, Box<dyn ToolExecutor>>,
}

impl DefaultToolExecutorRegistry {
    /// Create a new tool executor registry
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }
}

#[async_trait]
impl ToolExecutorRegistry for DefaultToolExecutorRegistry {
    async fn register_executor(
        &self,
        name: &str,
        executor: Box<dyn ToolExecutor>,
    ) -> ExecutorResult<()> {
        if self.executors.contains_key(name) {
            return Err(ExecutorError::Other(
                format!("Executor already registered: {}", name)
            ));
        }

        self.executors.insert(name.to_string(), executor);
        Ok(())
    }

    async fn get_executor(
        &self,
        name: &str,
    ) -> ExecutorResult<Box<dyn ToolExecutor>> {
        self.executors.get(name)
            .cloned()
            .ok_or_else(|| ExecutorError::Other(
                format!("Executor not found: {}", name)
            ))
    }

    async fn list_executors(&self) -> Vec<String> {
        self.executors.keys().cloned().collect()
    }

    async fn unregister_executor(
        &self,
        name: &str,
    ) -> ExecutorResult<()> {
        self.executors.remove(name)
            .ok_or_else(|| ExecutorError::Other(
                format!("Executor not found: {}", name)
            ))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_executor_registry() {
        let registry = Arc::new(DefaultToolExecutorRegistry::new());
        let factory = DefaultToolExecutorFactory::new(registry.clone());

        // Register file executor
        let temp_dir = tempdir().unwrap();
        let file_executor = FileToolExecutor::new(
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            vec!["txt".to_string()],
        );
        registry.register_executor(
            "file",
            Box::new(file_executor),
        ).await.unwrap();

        // Register web executor
        let web_executor = WebToolExecutor::new(
            1024 * 1024,
            vec!["localhost".to_string()],
            Duration::from_secs(30),
        ).unwrap();
        registry.register_executor(
            "web",
            Box::new(web_executor),
        ).await.unwrap();

        assert_eq!(registry.list_executors().await.len(), 2);
        assert!(registry.get_executor("file").await.is_ok());
        assert!(registry.get_executor("web").await.is_ok());
        assert!(registry.get_executor("unknown").await.is_err());
    }
}