//! Intent generation module
//!
//! The agent emits structured intents (JSON) but NEVER executes actions.
//! This is the security boundary between thinking and acting.

use crate::error::{AgentError, Result};
use crate::types::Intent;
use serde_json::Value;
use std::collections::HashMap;

/// Intent generator - converts reasoning into structured intents
pub struct IntentGenerator {
    // Configuration for intent generation
    min_confidence: f32,
}

impl IntentGenerator {
    pub fn new() -> Self {
        Self {
            min_confidence: 0.5,
        }
    }

    /// Generate an intent from parsed understanding
    pub fn generate(
        &self,
        intent_type: String,
        confidence: f32,
        parameters: HashMap<String, Value>,
        reasoning: String,
    ) -> Result<Intent> {
        if confidence < self.min_confidence {
            return Err(AgentError::InvalidIntent(format!(
                "Confidence {} below minimum {}",
                confidence, self.min_confidence
            )));
        }

        let mut intent = Intent::new(intent_type.clone(), confidence, parameters, reasoning);

        // Determine if permission is required based on intent type
        intent.requires_permission = self.requires_permission(&intent_type);

        // Extract target module from intent type (e.g., "device.control" -> "device")
        if let Some(module) = intent_type.split('.').next() {
            intent.target_module = Some(module.to_string());
        }

        Ok(intent)
    }

    /// Serialize intent to JSON for transmission to device agents
    pub fn to_json(&self, intent: &Intent) -> Result<String> {
        let json = serde_json::to_string_pretty(intent)?;
        Ok(json)
    }

    /// Determine if an intent type requires permission
    fn requires_permission(&self, intent_type: &str) -> bool {
        // Actions that modify state or interact with external systems require permission
        let requires_permission_prefixes = [
            "device.",
            "message.",
            "email.",
            "calendar.",
            "file.",
            "network.",
            "location.",
            "camera.",
            "microphone.",
            "notification.",
        ];

        requires_permission_prefixes
            .iter()
            .any(|prefix| intent_type.starts_with(prefix))
    }

    /// Validate intent structure
    pub fn validate(&self, intent: &Intent) -> Result<()> {
        if intent.confidence < 0.0 || intent.confidence > 1.0 {
            return Err(AgentError::InvalidIntent(
                "Confidence must be between 0.0 and 1.0".to_string(),
            ));
        }

        if intent.intent_type.is_empty() {
            return Err(AgentError::InvalidIntent(
                "Intent type cannot be empty".to_string(),
            ));
        }

        if intent.reasoning.is_empty() {
            return Err(AgentError::InvalidIntent(
                "Reasoning cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Parse common intent patterns from text
    /// This is a simple pattern matcher - in production, use LLM for better understanding
    pub fn parse_from_text(&self, text: &str) -> Vec<Intent> {
        let mut intents = Vec::new();
        let text_lower = text.to_lowercase();

        // Pattern: "remind me to..."
        if text_lower.contains("remind me to") || text_lower.contains("reminder") {
            let mut params = HashMap::new();
            params.insert("text".to_string(), Value::String(text.to_string()));
            
            if let Ok(intent) = self.generate(
                "reminder.create".to_string(),
                0.8,
                params,
                "User requested a reminder".to_string(),
            ) {
                intents.push(intent);
            }
        }

        // Pattern: "turn on/off..."
        if text_lower.contains("turn on") || text_lower.contains("turn off") {
            let action = if text_lower.contains("turn on") { "on" } else { "off" };
            let mut params = HashMap::new();
            params.insert("action".to_string(), Value::String(action.to_string()));
            
            if let Ok(intent) = self.generate(
                "device.control".to_string(),
                0.7,
                params,
                format!("User wants to turn {} a device", action),
            ) {
                intents.push(intent);
            }
        }

        // Pattern: "what's the weather" / "weather"
        if text_lower.contains("weather") {
            if let Ok(intent) = self.generate(
                "weather.query".to_string(),
                0.9,
                HashMap::new(),
                "User asking about weather".to_string(),
            ) {
                intents.push(intent);
            }
        }

        // Pattern: "what time" / "current time"
        if text_lower.contains("what time") || text_lower.contains("current time") {
            if let Ok(intent) = self.generate(
                "time.query".to_string(),
                0.95,
                HashMap::new(),
                "User asking about current time".to_string(),
            ) {
                intents.push(intent);
            }
        }

        intents
    }
}

impl Default for IntentGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_intent() {
        let generator = IntentGenerator::new();
        let mut params = HashMap::new();
        params.insert("device".to_string(), Value::String("light".to_string()));

        let intent = generator
            .generate(
                "device.control".to_string(),
                0.8,
                params,
                "User wants to control a device".to_string(),
            )
            .unwrap();

        assert_eq!(intent.intent_type, "device.control");
        assert_eq!(intent.confidence, 0.8);
        assert!(intent.requires_permission);
        assert_eq!(intent.target_module, Some("device".to_string()));
    }

    #[test]
    fn test_requires_permission() {
        let generator = IntentGenerator::new();

        assert!(generator.requires_permission("device.control"));
        assert!(generator.requires_permission("message.send"));
        assert!(!generator.requires_permission("weather.query"));
        assert!(!generator.requires_permission("time.query"));
    }

    #[test]
    fn test_parse_from_text_reminder() {
        let generator = IntentGenerator::new();
        let intents = generator.parse_from_text("remind me to call mom");

        assert_eq!(intents.len(), 1);
        assert_eq!(intents[0].intent_type, "reminder.create");
    }

    #[test]
    fn test_parse_from_text_weather() {
        let generator = IntentGenerator::new();
        let intents = generator.parse_from_text("what's the weather like today?");

        assert_eq!(intents.len(), 1);
        assert_eq!(intents[0].intent_type, "weather.query");
        assert!(!intents[0].requires_permission);
    }

    #[test]
    fn test_parse_from_text_device_control() {
        let generator = IntentGenerator::new();
        let intents = generator.parse_from_text("turn on the lights");

        assert_eq!(intents.len(), 1);
        assert_eq!(intents[0].intent_type, "device.control");
        assert!(intents[0].requires_permission);
    }

    #[test]
    fn test_validate_intent() {
        let generator = IntentGenerator::new();
        let mut params = HashMap::new();
        params.insert("test".to_string(), Value::String("value".to_string()));

        let intent = generator
            .generate(
                "test.action".to_string(),
                0.8,
                params,
                "Testing".to_string(),
            )
            .unwrap();

        assert!(generator.validate(&intent).is_ok());
    }

    #[test]
    fn test_to_json() {
        let generator = IntentGenerator::new();
        let mut params = HashMap::new();
        params.insert("device".to_string(), Value::String("light".to_string()));

        let intent = generator
            .generate(
                "device.control".to_string(),
                0.8,
                params,
                "Testing".to_string(),
            )
            .unwrap();

        let json = generator.to_json(&intent).unwrap();
        assert!(json.contains("device.control"));
        assert!(json.contains("light"));
    }
}
