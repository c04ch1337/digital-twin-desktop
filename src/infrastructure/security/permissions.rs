//! Permission management system
//!
//! This module provides functionality for managing permissions,
//! roles, and access control for the Digital Twin Desktop application.

use anyhow::{Result, anyhow};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

/// Permission error types
#[derive(Debug, Error)]
pub enum PermissionError {
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Role not found
    #[error("Role not found: {0}")]
    RoleNotFound(String),
    
    /// Permission not found
    #[error("Permission not found: {0}")]
    PermissionNotFound(String),
    
    /// User not found
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    /// Internal error
    #[error("Internal permission error: {0}")]
    Internal(String),
}

/// Permission type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    /// Permission ID
    pub id: String,
    
    /// Permission name
    pub name: String,
    
    /// Permission description
    pub description: String,
    
    /// Resource type
    pub resource_type: String,
    
    /// Action
    pub action: String,
}

impl Permission {
    /// Create a new permission
    pub fn new(resource_type: &str, action: &str, name: &str, description: &str) -> Self {
        let id = format!("{}:{}", resource_type, action);
        
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            resource_type: resource_type.to_string(),
            action: action.to_string(),
        }
    }
    
    /// Create a permission for reading a resource
    pub fn read(resource_type: &str) -> Self {
        Self::new(
            resource_type,
            "read",
            &format!("Read {}", resource_type),
            &format!("Permission to read {} resources", resource_type),
        )
    }
    
    /// Create a permission for writing a resource
    pub fn write(resource_type: &str) -> Self {
        Self::new(
            resource_type,
            "write",
            &format!("Write {}", resource_type),
            &format!("Permission to create or update {} resources", resource_type),
        )
    }
    
    /// Create a permission for deleting a resource
    pub fn delete(resource_type: &str) -> Self {
        Self::new(
            resource_type,
            "delete",
            &format!("Delete {}", resource_type),
            &format!("Permission to delete {} resources", resource_type),
        )
    }
    
    /// Create a permission for executing a tool
    pub fn execute_tool(tool_id: &str) -> Self {
        Self::new(
            "tool",
            tool_id,
            &format!("Execute {}", tool_id),
            &format!("Permission to execute the {} tool", tool_id),
        )
    }
    
    /// Create a permission for administering the system
    pub fn admin() -> Self {
        Self::new(
            "system",
            "admin",
            "System Administrator",
            "Full administrative access to the system",
        )
    }
}

/// Role type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Role {
    /// Role ID
    pub id: String,
    
    /// Role name
    pub name: String,
    
    /// Role description
    pub description: String,
    
    /// Permissions granted by this role
    pub permissions: Vec<String>,
}

impl Role {
    /// Create a new role
    pub fn new(id: &str, name: &str, description: &str, permissions: Vec<String>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            permissions,
        }
    }
    
    /// Create an admin role
    pub fn admin() -> Self {
        Self::new(
            "admin",
            "Administrator",
            "Full administrative access to the system",
            vec![Permission::admin().id],
        )
    }
    
    /// Create a user role
    pub fn user() -> Self {
        Self::new(
            "user",
            "User",
            "Standard user access",
            vec![
                Permission::read("twin").id,
                Permission::read("conversation").id,
                Permission::read("simulation").id,
                Permission::write("conversation").id,
            ],
        )
    }
    
    /// Create a guest role
    pub fn guest() -> Self {
        Self::new(
            "guest",
            "Guest",
            "Limited guest access",
            vec![
                Permission::read("twin").id,
                Permission::read("conversation").id,
            ],
        )
    }
    
    /// Create a tool user role
    pub fn tool_user() -> Self {
        Self::new(
            "tool_user",
            "Tool User",
            "Access to use tools",
            vec![
                Permission::read("tool").id,
                Permission::execute_tool("file_tool").id,
                Permission::execute_tool("web_tool").id,
            ],
        )
    }
}

/// User permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    /// User ID
    pub user_id: String,
    
    /// Roles assigned to the user
    pub roles: HashSet<String>,
    
    /// Direct permissions assigned to the user
    pub permissions: HashSet<String>,
}

impl UserPermissions {
    /// Create new user permissions
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            roles: HashSet::new(),
            permissions: HashSet::new(),
        }
    }
    
    /// Add a role
    pub fn add_role(&mut self, role_id: &str) {
        self.roles.insert(role_id.to_string());
    }
    
    /// Remove a role
    pub fn remove_role(&mut self, role_id: &str) {
        self.roles.remove(role_id);
    }
    
    /// Add a permission
    pub fn add_permission(&mut self, permission_id: &str) {
        self.permissions.insert(permission_id.to_string());
    }
    
    /// Remove a permission
    pub fn remove_permission(&mut self, permission_id: &str) {
        self.permissions.remove(permission_id);
    }
    
    /// Check if the user has a role
    pub fn has_role(&self, role_id: &str) -> bool {
        self.roles.contains(role_id)
    }
    
    /// Check if the user has a permission directly
    pub fn has_direct_permission(&self, permission_id: &str) -> bool {
        self.permissions.contains(permission_id)
    }
}

/// Permission manager
pub struct PermissionManager {
    /// Available permissions
    permissions: RwLock<HashMap<String, Permission>>,
    
    /// Available roles
    roles: RwLock<HashMap<String, Role>>,
    
    /// User permissions
    user_permissions: DashMap<String, UserPermissions>,
    
    /// Resource permissions
    resource_permissions: DashMap<String, HashSet<String>>,
}

impl PermissionManager {
    /// Create a new permission manager
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        let mut roles = HashMap::new();
        
        // Add default permissions
        let default_permissions = vec![
            Permission::admin(),
            Permission::read("twin"),
            Permission::write("twin"),
            Permission::delete("twin"),
            Permission::read("conversation"),
            Permission::write("conversation"),
            Permission::delete("conversation"),
            Permission::read("simulation"),
            Permission::write("simulation"),
            Permission::delete("simulation"),
            Permission::read("tool"),
            Permission::execute_tool("file_tool"),
            Permission::execute_tool("web_tool"),
            Permission::execute_tool("modbus_tool"),
            Permission::execute_tool("mqtt_tool"),
            Permission::execute_tool("twin_tool"),
        ];
        
        for permission in default_permissions {
            permissions.insert(permission.id.clone(), permission);
        }
        
        // Add default roles
        let default_roles = vec![
            Role::admin(),
            Role::user(),
            Role::guest(),
            Role::tool_user(),
        ];
        
        for role in default_roles {
            roles.insert(role.id.clone(), role);
        }
        
        Self {
            permissions: RwLock::new(permissions),
            roles: RwLock::new(roles),
            user_permissions: DashMap::new(),
            resource_permissions: DashMap::new(),
        }
    }
    
    /// Register a new permission
    pub fn register_permission(&self, permission: Permission) -> Result<()> {
        let mut permissions = self.permissions.write().unwrap();
        permissions.insert(permission.id.clone(), permission);
        Ok(())
    }
    
    /// Register a new role
    pub fn register_role(&self, role: Role) -> Result<()> {
        let mut roles = self.roles.write().unwrap();
        roles.insert(role.id.clone(), role);
        Ok(())
    }
    
    /// Assign a role to a user
    pub fn assign_role(&self, user_id: &str, role_id: &str) -> Result<()> {
        // Check if the role exists
        let roles = self.roles.read().unwrap();
        if !roles.contains_key(role_id) {
            return Err(PermissionError::RoleNotFound(role_id.to_string()).into());
        }
        
        // Get or create user permissions
        let mut user_perms = self.user_permissions
            .entry(user_id.to_string())
            .or_insert_with(|| UserPermissions::new(user_id));
        
        user_perms.add_role(role_id);
        
        Ok(())
    }
    
    /// Revoke a role from a user
    pub fn revoke_role(&self, user_id: &str, role_id: &str) -> Result<()> {
        if let Some(mut user_perms) = self.user_permissions.get_mut(user_id) {
            user_perms.remove_role(role_id);
            Ok(())
        } else {
            Err(PermissionError::UserNotFound(user_id.to_string()).into())
        }
    }
    
    /// Grant a permission to a user
    pub fn grant_permission(&self, user_id: &str, permission_id: &str) -> Result<()> {
        // Check if the permission exists
        let permissions = self.permissions.read().unwrap();
        if !permissions.contains_key(permission_id) {
            return Err(PermissionError::PermissionNotFound(permission_id.to_string()).into());
        }
        
        // Get or create user permissions
        let mut user_perms = self.user_permissions
            .entry(user_id.to_string())
            .or_insert_with(|| UserPermissions::new(user_id));
        
        user_perms.add_permission(permission_id);
        
        Ok(())
    }
    
    /// Revoke a permission from a user
    pub fn revoke_permission(&self, user_id: &str, permission_id: &str) -> Result<()> {
        if let Some(mut user_perms) = self.user_permissions.get_mut(user_id) {
            user_perms.remove_permission(permission_id);
            Ok(())
        } else {
            Err(PermissionError::UserNotFound(user_id.to_string()).into())
        }
    }
    
    /// Grant permission to execute a tool
    pub fn grant_tool_permission(&self, user_id: &str, tool_id: &str) -> Result<()> {
        let permission_id = format!("tool:{}", tool_id);
        self.grant_permission(user_id, &permission_id)
    }
    
    /// Check if a user has a permission
    pub fn has_permission(&self, user_id: &str, permission_id: &str) -> bool {
        // Check if the user has the admin permission
        if self.has_admin_permission(user_id) {
            return true;
        }
        
        // Check if the user has the permission directly
        if let Some(user_perms) = self.user_permissions.get(user_id) {
            if user_perms.has_direct_permission(permission_id) {
                return true;
            }
            
            // Check if the user has the permission through a role
            let roles = self.roles.read().unwrap();
            for role_id in &user_perms.roles {
                if let Some(role) = roles.get(role_id) {
                    if role.permissions.contains(&permission_id.to_string()) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// Check if a user has admin permission
    pub fn has_admin_permission(&self, user_id: &str) -> bool {
        if let Some(user_perms) = self.user_permissions.get(user_id) {
            // Check for direct admin permission
            if user_perms.has_direct_permission("system:admin") {
                return true;
            }
            
            // Check for admin role
            if user_perms.has_role("admin") {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a user can execute a tool
    pub fn can_execute_tool(&self, user_id: &str, tool_id: &str) -> bool {
        let permission_id = format!("tool:{}", tool_id);
        self.has_permission(user_id, &permission_id)
    }
    
    /// Check if a user can access a resource
    pub fn can_access_resource(&self, user_id: &str, resource_type: &str, resource_id: &str, action: &str) -> bool {
        // Check for general permission
        let general_permission_id = format!("{}:{}", resource_type, action);
        if self.has_permission(user_id, &general_permission_id) {
            return true;
        }
        
        // Check for specific resource permission
        let resource_key = format!("{}:{}", resource_type, resource_id);
        if let Some(allowed_users) = self.resource_permissions.get(&resource_key) {
            if allowed_users.contains(user_id) {
                return true;
            }
        }
        
        false
    }
    
    /// Grant permission to access a specific resource
    pub fn grant_resource_permission(&self, user_id: &str, resource_type: &str, resource_id: &str) -> Result<()> {
        let resource_key = format!("{}:{}", resource_type, resource_id);
        
        let mut allowed_users = self.resource_permissions
            .entry(resource_key)
            .or_insert_with(HashSet::new);
        
        allowed_users.insert(user_id.to_string());
        
        Ok(())
    }
    
    /// Revoke permission to access a specific resource
    pub fn revoke_resource_permission(&self, user_id: &str, resource_type: &str, resource_id: &str) -> Result<()> {
        let resource_key = format!("{}:{}", resource_type, resource_id);
        
        if let Some(mut allowed_users) = self.resource_permissions.get_mut(&resource_key) {
            allowed_users.remove(user_id);
            Ok(())
        } else {
            Ok(()) // Resource doesn't exist, so no permissions to revoke
        }
    }
    
    /// Get all permissions for a user
    pub fn get_user_permissions(&self, user_id: &str) -> Result<HashSet<String>> {
        let mut all_permissions = HashSet::new();
        
        if let Some(user_perms) = self.user_permissions.get(user_id) {
            // Add direct permissions
            all_permissions.extend(user_perms.permissions.clone());
            
            // Add permissions from roles
            let roles = self.roles.read().unwrap();
            for role_id in &user_perms.roles {
                if let Some(role) = roles.get(role_id) {
                    all_permissions.extend(role.permissions.clone());
                }
            }
            
            Ok(all_permissions)
        } else {
            Err(PermissionError::UserNotFound(user_id.to_string()).into())
        }
    }
    
    /// Get all roles for a user
    pub fn get_user_roles(&self, user_id: &str) -> Result<HashSet<String>> {
        if let Some(user_perms) = self.user_permissions.get(user_id) {
            Ok(user_perms.roles.clone())
        } else {
            Err(PermissionError::UserNotFound(user_id.to_string()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permission_creation() {
        let perm = Permission::new("twin", "read", "Read Twin", "Permission to read twin data");
        assert_eq!(perm.id, "twin:read");
        assert_eq!(perm.resource_type, "twin");
        assert_eq!(perm.action, "read");
    }
    
    #[test]
    fn test_role_creation() {
        let role = Role::new(
            "viewer",
            "Viewer",
            "Can view resources",
            vec!["twin:read".to_string(), "conversation:read".to_string()],
        );
        
        assert_eq!(role.id, "viewer");
        assert_eq!(role.permissions.len(), 2);
        assert!(role.permissions.contains(&"twin:read".to_string()));
    }
    
    #[test]
    fn test_permission_assignment() {
        let manager = PermissionManager::new();
        
        // Assign roles
        manager.assign_role("user1", "user").unwrap();
        manager.assign_role("admin1", "admin").unwrap();
        
        // Grant direct permissions
        manager.grant_permission("user1", "twin:read").unwrap();
        
        // Check permissions
        assert!(manager.has_permission("user1", "twin:read"));
        assert!(manager.has_permission("user1", "conversation:read")); // From user role
        assert!(!manager.has_permission("user1", "twin:delete")); // Not granted
        
        assert!(manager.has_permission("admin1", "twin:delete")); // Admin has all permissions
        assert!(manager.has_admin_permission("admin1"));
        assert!(!manager.has_admin_permission("user1"));
    }
    
    #[test]
    fn test_tool_permissions() {
        let manager = PermissionManager::new();
        
        // Grant tool permission
        manager.grant_tool_permission("user1", "file_tool").unwrap();
        
        // Check permissions
        assert!(manager.can_execute_tool("user1", "file_tool"));
        assert!(!manager.can_execute_tool("user1", "dangerous_tool"));
        
        // Assign tool user role
        manager.assign_role("user2", "tool_user").unwrap();
        
        // Check permissions
        assert!(manager.can_execute_tool("user2", "file_tool"));
        assert!(manager.can_execute_tool("user2", "web_tool"));
    }
    
    #[test]
    fn test_resource_permissions() {
        let manager = PermissionManager::new();
        
        // Grant general permission
        manager.grant_permission("user1", "twin:read").unwrap();
        
        // Grant specific resource permission
        manager.grant_resource_permission("user2", "twin", "twin123").unwrap();
        
        // Check permissions
        assert!(manager.can_access_resource("user1", "twin", "twin123", "read"));
        assert!(manager.can_access_resource("user1", "twin", "twin456", "read")); // Any twin
        
        assert!(manager.can_access_resource("user2", "twin", "twin123", "read")); // Specific twin
        assert!(!manager.can_access_resource("user2", "twin", "twin456", "read")); // Different twin
    }
}