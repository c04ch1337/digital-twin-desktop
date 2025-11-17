//! Prompt Manager Service for prompt versioning and storage.
//!
//! This service manages agent prompts with support for:
//! - Prompt versioning
//! - Prompt storage and retrieval
//! - Tracking prompt changes

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a versioned prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedPrompt {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub version: String,
    pub system_prompt: String,
    pub instructions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Prompt change tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChange {
    pub from_version: String,
    pub to_version: String,
    pub changed_at: DateTime<Utc>,
    pub changes: Vec<PromptDiff>,
}

/// Represents a difference between prompt versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptDiff {
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

/// Prompt manager for handling versioning and storage
pub struct PromptManager {
    prompts: HashMap<Uuid, Vec<VersionedPrompt>>,
    changes: HashMap<Uuid, Vec<PromptChange>>,
}

impl PromptManager {
    /// Creates a new prompt manager
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            changes: HashMap::new(),
        }
    }

    /// Stores a new prompt version
    pub fn store_prompt(&mut self, prompt: VersionedPrompt) -> Result<VersionedPrompt, String> {
        // Validate version format
        if !self.is_valid_version(&prompt.version) {
            return Err("Invalid version format. Use semantic versioning (e.g., 1.0.0)".to_string());
        }

        // Check for duplicate version
        if let Some(prompts) = self.prompts.get(&prompt.agent_id) {
            if prompts.iter().any(|p| p.version == prompt.version) {
                return Err(format!(
                    "Version {} already exists for agent {}",
                    prompt.version, prompt.agent_id
                ));
            }
        }

        let stored_prompt = prompt.clone();
        self.prompts
            .entry(prompt.agent_id)
            .or_insert_with(Vec::new)
            .push(prompt);

        Ok(stored_prompt)
    }

    /// Retrieves a specific prompt version
    pub fn get_prompt(
        &self,
        agent_id: Uuid,
        version: &str,
    ) -> Result<VersionedPrompt, String> {
        self.prompts
            .get(&agent_id)
            .and_then(|prompts| prompts.iter().find(|p| p.version == version).cloned())
            .ok_or_else(|| {
                format!(
                    "Prompt version {} not found for agent {}",
                    version, agent_id
                )
            })
    }

    /// Retrieves the latest prompt version for an agent
    pub fn get_latest_prompt(&self, agent_id: Uuid) -> Result<VersionedPrompt, String> {
        self.prompts
            .get(&agent_id)
            .and_then(|prompts| {
                prompts
                    .iter()
                    .max_by(|a, b| self.compare_versions(&a.version, &b.version))
                    .cloned()
            })
            .ok_or_else(|| format!("No prompts found for agent {}", agent_id))
    }

    /// Retrieves all prompt versions for an agent
    pub fn get_all_prompts(&self, agent_id: Uuid) -> Result<Vec<VersionedPrompt>, String> {
        self.prompts
            .get(&agent_id)
            .map(|prompts| {
                let mut sorted = prompts.clone();
                sorted.sort_by(|a, b| self.compare_versions(&a.version, &b.version));
                sorted
            })
            .ok_or_else(|| format!("No prompts found for agent {}", agent_id))
    }

    /// Updates an existing prompt (creates a new version)
    pub fn update_prompt(
        &mut self,
        agent_id: Uuid,
        new_version: String,
        system_prompt: String,
        instructions: Option<String>,
    ) -> Result<VersionedPrompt, String> {
        // Get the current prompt to track changes
        let current_prompt = self.get_latest_prompt(agent_id)?;

        // Create new prompt
        let new_prompt = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: new_version.clone(),
            system_prompt: system_prompt.clone(),
            instructions: instructions.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        // Track changes
        let changes = self.calculate_changes(
            &current_prompt.system_prompt,
            &current_prompt.instructions,
            &system_prompt,
            &instructions,
        );

        let change_record = PromptChange {
            from_version: current_prompt.version.clone(),
            to_version: new_version.clone(),
            changed_at: Utc::now(),
            changes,
        };

        self.changes
            .entry(agent_id)
            .or_insert_with(Vec::new)
            .push(change_record);

        // Store the new prompt
        self.store_prompt(new_prompt)
    }

    /// Retrieves the change history for an agent's prompts
    pub fn get_change_history(&self, agent_id: Uuid) -> Result<Vec<PromptChange>, String> {
        self.changes
            .get(&agent_id)
            .map(|changes| changes.clone())
            .ok_or_else(|| format!("No change history found for agent {}", agent_id))
    }

    /// Reverts to a previous prompt version
    pub fn revert_to_version(
        &mut self,
        agent_id: Uuid,
        target_version: &str,
    ) -> Result<VersionedPrompt, String> {
        let target_prompt = self.get_prompt(agent_id, target_version)?;

        // Create a new version based on the target
        let new_version = self.increment_version(&target_prompt.version)?;

        self.update_prompt(
            agent_id,
            new_version,
            target_prompt.system_prompt,
            target_prompt.instructions,
        )
    }

    /// Validates version format (semantic versioning)
    fn is_valid_version(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }

    /// Compares two version strings
    fn compare_versions(&self, v1: &str, v2: &str) -> std::cmp::Ordering {
        let v1_parts: Vec<u32> = v1
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        let v2_parts: Vec<u32> = v2
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();

        v1_parts.cmp(&v2_parts)
    }

    /// Increments a version string
    fn increment_version(&self, version: &str) -> Result<String, String> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid version format".to_string());
        }

        let major: u32 = parts[0].parse().map_err(|_| "Invalid major version")?;
        let minor: u32 = parts[1].parse().map_err(|_| "Invalid minor version")?;
        let patch: u32 = parts[2].parse().map_err(|_| "Invalid patch version")?;

        Ok(format!("{}.{}.{}", major, minor, patch + 1))
    }

    /// Calculates differences between two prompts
    fn calculate_changes(
        &self,
        old_system: &str,
        old_instructions: &Option<String>,
        new_system: &str,
        new_instructions: &Option<String>,
    ) -> Vec<PromptDiff> {
        let mut diffs = Vec::new();

        if old_system != new_system {
            diffs.push(PromptDiff {
                field: "system_prompt".to_string(),
                old_value: Some(old_system.to_string()),
                new_value: Some(new_system.to_string()),
            });
        }

        if old_instructions != new_instructions {
            diffs.push(PromptDiff {
                field: "instructions".to_string(),
                old_value: old_instructions.clone(),
                new_value: new_instructions.clone(),
            });
        }

        diffs
    }

    /// Exports prompt as JSON
    pub fn export_prompt(&self, agent_id: Uuid, version: &str) -> Result<String, String> {
        let prompt = self.get_prompt(agent_id, version)?;
        serde_json::to_string_pretty(&prompt)
            .map_err(|e| format!("Failed to serialize prompt: {}", e))
    }

    /// Imports prompt from JSON
    pub fn import_prompt(&mut self, json: &str) -> Result<VersionedPrompt, String> {
        let prompt: VersionedPrompt = serde_json::from_str(json)
            .map_err(|e| format!("Failed to deserialize prompt: {}", e))?;

        self.store_prompt(prompt)
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve_prompt() {
        let mut manager = PromptManager::new();
        let agent_id = Uuid::new_v4();

        let prompt = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "1.0.0".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            instructions: Some("Be concise and clear.".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        let stored = manager.store_prompt(prompt.clone()).unwrap();
        assert_eq!(stored.version, "1.0.0");

        let retrieved = manager.get_prompt(agent_id, "1.0.0").unwrap();
        assert_eq!(retrieved.system_prompt, "You are a helpful assistant.");
    }

    #[test]
    fn test_duplicate_version_error() {
        let mut manager = PromptManager::new();
        let agent_id = Uuid::new_v4();

        let prompt1 = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "1.0.0".to_string(),
            system_prompt: "Prompt 1".to_string(),
            instructions: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        let prompt2 = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "1.0.0".to_string(),
            system_prompt: "Prompt 2".to_string(),
            instructions: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        manager.store_prompt(prompt1).unwrap();
        assert!(manager.store_prompt(prompt2).is_err());
    }

    #[test]
    fn test_get_latest_prompt() {
        let mut manager = PromptManager::new();
        let agent_id = Uuid::new_v4();

        let prompt1 = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "1.0.0".to_string(),
            system_prompt: "Prompt 1".to_string(),
            instructions: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        let prompt2 = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "2.0.0".to_string(),
            system_prompt: "Prompt 2".to_string(),
            instructions: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        manager.store_prompt(prompt1).unwrap();
        manager.store_prompt(prompt2).unwrap();

        let latest = manager.get_latest_prompt(agent_id).unwrap();
        assert_eq!(latest.version, "2.0.0");
    }

    #[test]
    fn test_version_validation() {
        let manager = PromptManager::new();

        assert!(manager.is_valid_version("1.0.0"));
        assert!(manager.is_valid_version("2.5.10"));
        assert!(!manager.is_valid_version("1.0"));
        assert!(!manager.is_valid_version("1.0.0.0"));
        assert!(!manager.is_valid_version("a.b.c"));
    }

    #[test]
    fn test_version_increment() {
        let manager = PromptManager::new();

        let incremented = manager.increment_version("1.0.0").unwrap();
        assert_eq!(incremented, "1.0.1");

        let incremented = manager.increment_version("2.5.10").unwrap();
        assert_eq!(incremented, "2.5.11");
    }

    #[test]
    fn test_update_prompt_tracks_changes() {
        let mut manager = PromptManager::new();
        let agent_id = Uuid::new_v4();

        let prompt1 = VersionedPrompt {
            id: Uuid::new_v4(),
            agent_id,
            version: "1.0.0".to_string(),
            system_prompt: "Old prompt".to_string(),
            instructions: Some("Old instructions".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        manager.store_prompt(prompt1).unwrap();

        manager
            .update_prompt(
                agent_id,
                "1.0.1".to_string(),
                "New prompt".to_string(),
                Some("New instructions".to_string()),
            )
            .unwrap();

        let history = manager.get_change_history(agent_id).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].from_version, "1.0.0");
        assert_eq!(history[0].to_version, "1.0.1");
        assert_eq!(history[0].changes.len(), 2);
    }
}
