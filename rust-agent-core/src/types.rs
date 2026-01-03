//! Core type definitions for the agent

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Structured intent emitted by the agent
/// Agent emits intents but NEVER executes actions directly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: Uuid,
    pub intent_type: String,
    pub confidence: f32,
    pub parameters: HashMap<String, serde_json::Value>,
    pub reasoning: String,
    pub requires_permission: bool,
    pub target_module: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Intent {
    pub fn new(
        intent_type: String,
        confidence: f32,
        parameters: HashMap<String, serde_json::Value>,
        reasoning: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            intent_type,
            confidence,
            parameters,
            reasoning,
            requires_permission: false,
            target_module: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_permission(mut self, requires: bool) -> Self {
        self.requires_permission = requires;
        self
    }

    pub fn with_target_module(mut self, module: String) -> Self {
        self.target_module = Some(module);
        self
    }
}

/// Event in the user's life or system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub description: String,
    pub importance: f32,
    pub metadata: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    pub fn new(event_type: String, description: String, importance: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            description,
            importance,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// User habit/routine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub frequency: HabitFrequency,
    pub schedule: Option<String>,
    pub completion_count: u32,
    pub last_completed: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub variance: Option<f32>, // Statistical variance in timing/completion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitFrequency {
    Daily,
    Weekly,
    Monthly,
    Custom(String),
}

impl Habit {
    pub fn new(name: String, description: String, frequency: HabitFrequency) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            frequency,
            schedule: None,
            completion_count: 0,
            last_completed: None,
            created_at: Utc::now(),
            variance: None,
        }
    }

    pub fn complete(&mut self) {
        self.completion_count += 1;
        self.last_completed = Some(Utc::now());
    }
}

/// User context at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub user_id: String,
    pub current_location: Option<String>,
    pub current_activity: Option<String>,
    pub recent_events: Vec<Event>,
    pub active_habits: Vec<Habit>,
    pub timestamp: DateTime<Utc>,
}

impl Context {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            current_location: None,
            current_activity: None,
            recent_events: Vec::new(),
            active_habits: Vec::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Voice transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTranscription {
    pub text: String,
    pub confidence: f32,
    pub language: String,
    pub duration_ms: u32,
}

/// LLM completion options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmOptions {
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

impl Default for LlmOptions {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 500,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        }
    }
}

/// LLM completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
    pub finish_reason: String,
    pub usage: LlmUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
