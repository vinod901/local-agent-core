//! Policy engine module
//!
//! Enforces safety boundaries and permission checks.
//! The agent prepares and suggests - humans authorize.

use crate::error::{AgentError, Result};
use crate::types::Intent;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Permission grant from user
#[derive(Debug, Clone)]
pub struct Permission {
    pub module: String,
    pub actions: Vec<String>,
    pub scope: Vec<String>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Policy engine for enforcing safety boundaries
pub struct PolicyEngine {
    permissions: HashMap<String, Vec<Permission>>,
    allowed_modules: Vec<String>,
}

impl PolicyEngine {
    pub fn new(allowed_modules: Vec<String>) -> Self {
        Self {
            permissions: HashMap::new(),
            allowed_modules,
        }
    }

    /// Grant permission for a module and actions
    pub fn grant_permission(&mut self, permission: Permission) {
        let module = permission.module.clone();
        self.permissions
            .entry(module)
            .or_insert_with(Vec::new)
            .push(permission);
    }

    /// Check if an intent is permitted
    pub fn check_intent(&self, intent: &Intent) -> Result<()> {
        // If no permission required, allow
        if !intent.requires_permission {
            return Ok(());
        }

        // Check if target module is allowed
        if let Some(module) = &intent.target_module {
            if !self.allowed_modules.is_empty() && !self.allowed_modules.contains(module) {
                return Err(AgentError::PolicyViolation(format!(
                    "Module '{}' is not in allowed modules list",
                    module
                )));
            }

            // Check permissions
            if let Some(perms) = self.permissions.get(module) {
                let now = Utc::now();
                
                // Check if any permission grants access
                for perm in perms {
                    // Check expiration
                    if let Some(expires) = perm.expires_at {
                        if expires < now {
                            continue; // Permission expired
                        }
                    }

                    // Check if action is permitted
                    if perm.actions.iter().any(|a| {
                        a == &intent.intent_type || intent.intent_type.starts_with(&format!("{}.", a))
                    }) {
                        return Ok(());
                    }
                }
            }

            return Err(AgentError::PolicyViolation(format!(
                "No valid permission found for intent type '{}'",
                intent.intent_type
            )));
        }

        Err(AgentError::PolicyViolation(
            "Intent requires permission but has no target module".to_string(),
        ))
    }

    /// Revoke all permissions for a module
    pub fn revoke_module(&mut self, module: &str) {
        self.permissions.remove(module);
    }

    /// Clear expired permissions
    pub fn clear_expired(&mut self) -> usize {
        let now = Utc::now();
        let mut cleared = 0;

        for perms in self.permissions.values_mut() {
            let original_len = perms.len();
            perms.retain(|p| p.expires_at.map(|exp| exp > now).unwrap_or(true));
            cleared += original_len - perms.len();
        }

        self.permissions.retain(|_, perms| !perms.is_empty());

        cleared
    }

    /// Get active permissions for a module
    pub fn get_permissions(&self, module: &str) -> Vec<Permission> {
        let now = Utc::now();
        
        self.permissions
            .get(module)
            .map(|perms| {
                perms
                    .iter()
                    .filter(|p| p.expires_at.map(|exp| exp > now).unwrap_or(true))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if a specific action is permitted
    pub fn is_action_permitted(&self, module: &str, action: &str) -> bool {
        let now = Utc::now();
        
        if let Some(perms) = self.permissions.get(module) {
            return perms.iter().any(|p| {
                // Check not expired
                if let Some(expires) = p.expires_at {
                    if expires < now {
                        return false;
                    }
                }
                
                // Check action match
                p.actions.iter().any(|a| a == action || action.starts_with(&format!("{}.", a)))
            });
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::collections::HashMap;

    #[test]
    fn test_check_intent_no_permission_required() {
        let engine = PolicyEngine::new(vec![]);
        let intent = Intent::new(
            "weather.query".to_string(),
            0.9,
            HashMap::new(),
            "User asking about weather".to_string(),
        );

        assert!(engine.check_intent(&intent).is_ok());
    }

    #[test]
    fn test_check_intent_with_permission() {
        let mut engine = PolicyEngine::new(vec!["device".to_string()]);
        
        let permission = Permission {
            module: "device".to_string(),
            actions: vec!["device.control".to_string()],
            scope: vec![],
            granted_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::hours(1)),
        };
        engine.grant_permission(permission);

        let intent = Intent::new(
            "device.control".to_string(),
            0.8,
            HashMap::new(),
            "Control device".to_string(),
        )
        .with_permission(true)
        .with_target_module("device".to_string());

        assert!(engine.check_intent(&intent).is_ok());
    }

    #[test]
    fn test_check_intent_without_permission() {
        let engine = PolicyEngine::new(vec!["device".to_string()]);
        
        let intent = Intent::new(
            "device.control".to_string(),
            0.8,
            HashMap::new(),
            "Control device".to_string(),
        )
        .with_permission(true)
        .with_target_module("device".to_string());

        assert!(engine.check_intent(&intent).is_err());
    }

    #[test]
    fn test_expired_permission() {
        let mut engine = PolicyEngine::new(vec!["device".to_string()]);
        
        let permission = Permission {
            module: "device".to_string(),
            actions: vec!["device.control".to_string()],
            scope: vec![],
            granted_at: Utc::now() - Duration::hours(2),
            expires_at: Some(Utc::now() - Duration::hours(1)),
        };
        engine.grant_permission(permission);

        let intent = Intent::new(
            "device.control".to_string(),
            0.8,
            HashMap::new(),
            "Control device".to_string(),
        )
        .with_permission(true)
        .with_target_module("device".to_string());

        assert!(engine.check_intent(&intent).is_err());
    }

    #[test]
    fn test_clear_expired() {
        let mut engine = PolicyEngine::new(vec!["device".to_string()]);
        
        // Add expired permission
        let expired = Permission {
            module: "device".to_string(),
            actions: vec!["device.control".to_string()],
            scope: vec![],
            granted_at: Utc::now() - Duration::hours(2),
            expires_at: Some(Utc::now() - Duration::hours(1)),
        };
        engine.grant_permission(expired);

        // Add valid permission
        let valid = Permission {
            module: "device".to_string(),
            actions: vec!["device.query".to_string()],
            scope: vec![],
            granted_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::hours(1)),
        };
        engine.grant_permission(valid);

        let cleared = engine.clear_expired();
        assert_eq!(cleared, 1);
        
        let perms = engine.get_permissions("device");
        assert_eq!(perms.len(), 1);
        assert_eq!(perms[0].actions[0], "device.query");
    }

    #[test]
    fn test_revoke_module() {
        let mut engine = PolicyEngine::new(vec!["device".to_string()]);
        
        let permission = Permission {
            module: "device".to_string(),
            actions: vec!["device.control".to_string()],
            scope: vec![],
            granted_at: Utc::now(),
            expires_at: None,
        };
        engine.grant_permission(permission);

        assert!(engine.is_action_permitted("device", "device.control"));
        
        engine.revoke_module("device");
        
        assert!(!engine.is_action_permitted("device", "device.control"));
    }
}
