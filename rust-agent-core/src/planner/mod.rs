//! Planner module
//!
//! Reasoning and planning - prepares actions, doesn't execute them.
//! Uses context and habits to make informed suggestions.

use crate::error::Result;
use crate::types::{Context, Event, Intent};
use std::collections::HashMap;

/// Planner for reasoning about actions and suggestions
pub struct Planner {
    // Configuration
    max_context_events: usize,
}

impl Planner {
    pub fn new() -> Self {
        Self {
            max_context_events: 10,
        }
    }

    /// Build a context summary for LLM prompting
    pub fn build_context_summary(&self, context: &Context) -> String {
        let mut summary = String::new();

        summary.push_str(&format!("User: {}\n", context.user_id));
        summary.push_str(&format!("Timestamp: {}\n", context.timestamp));

        if let Some(location) = &context.current_location {
            summary.push_str(&format!("Location: {}\n", location));
        }

        if let Some(activity) = &context.current_activity {
            summary.push_str(&format!("Activity: {}\n", activity));
        }

        // Recent events
        if !context.recent_events.is_empty() {
            summary.push_str("\nRecent events:\n");
            let events_to_show = context.recent_events
                .iter()
                .rev()
                .take(self.max_context_events);
            
            for event in events_to_show {
                summary.push_str(&format!(
                    "  - {} ({}): {}\n",
                    event.event_type,
                    event.timestamp.format("%Y-%m-%d %H:%M"),
                    event.description
                ));
            }
        }

        // Active habits
        if !context.active_habits.is_empty() {
            summary.push_str("\nActive habits:\n");
            for habit in &context.active_habits {
                let last_completed = habit.last_completed
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "never".to_string());
                
                summary.push_str(&format!(
                    "  - {} ({:?}): last completed {}\n",
                    habit.name,
                    habit.frequency,
                    last_completed
                ));
            }
        }

        summary
    }

    /// Compress events into a summary
    pub fn compress_events(&self, events: &[Event]) -> String {
        if events.is_empty() {
            return "No recent events.".to_string();
        }

        // Group events by type
        let mut by_type: HashMap<String, Vec<&Event>> = HashMap::new();
        for event in events {
            by_type.entry(event.event_type.clone())
                .or_insert_with(Vec::new)
                .push(event);
        }

        let mut summary = String::new();
        summary.push_str("Event summary:\n");

        for (event_type, type_events) in by_type.iter() {
            summary.push_str(&format!(
                "  - {}: {} occurrence(s)\n",
                event_type,
                type_events.len()
            ));

            // Show most important event of this type
            if let Some(most_important) = type_events.iter()
                .max_by(|a, b| a.importance.partial_cmp(&b.importance).unwrap())
            {
                summary.push_str(&format!("    Most important: {}\n", most_important.description));
            }
        }

        summary
    }

    /// Suggest next actions based on context
    /// These are suggestions, not commands - user must authorize
    pub fn suggest_actions(&self, context: &Context) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for habits that might be due
        for habit in &context.active_habits {
            if let Some(last) = habit.last_completed {
                let hours_since = chrono::Utc::now()
                    .signed_duration_since(last)
                    .num_hours();

                let expected_hours = match habit.frequency {
                    crate::types::HabitFrequency::Daily => 24,
                    crate::types::HabitFrequency::Weekly => 168,
                    crate::types::HabitFrequency::Monthly => 720,
                    crate::types::HabitFrequency::Custom(_) => 24,
                };

                if hours_since >= expected_hours {
                    suggestions.push(format!("Consider: {}", habit.name));
                }
            } else {
                suggestions.push(format!("Start habit: {}", habit.name));
            }
        }

        // Context-aware suggestions
        if let Some(activity) = &context.current_activity {
            if activity == "working" {
                suggestions.push("Take a break?".to_string());
            }
        }

        suggestions
    }

    /// Evaluate if an intent makes sense in current context
    pub fn evaluate_intent(&self, intent: &Intent, context: &Context) -> (bool, String) {
        // Check if intent aligns with current activity
        if let Some(activity) = &context.current_activity {
            if activity == "sleeping" && intent.intent_type.starts_with("device.") {
                return (false, "User appears to be sleeping, device control may not be appropriate".to_string());
            }
        }

        // Check confidence threshold
        if intent.confidence < 0.5 {
            return (false, format!("Low confidence: {}", intent.confidence));
        }

        (true, "Intent appears appropriate for current context".to_string())
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Habit, HabitFrequency};
    use chrono::Utc;

    #[test]
    fn test_build_context_summary() {
        let planner = Planner::new();
        let mut context = Context::new("test-user".to_string());
        context.current_location = Some("home".to_string());
        context.current_activity = Some("working".to_string());

        let summary = planner.build_context_summary(&context);
        assert!(summary.contains("test-user"));
        assert!(summary.contains("home"));
        assert!(summary.contains("working"));
    }

    #[test]
    fn test_compress_events() {
        let planner = Planner::new();
        let events = vec![
            Event::new("user_input".to_string(), "Hello".to_string(), 0.5),
            Event::new("user_input".to_string(), "How are you".to_string(), 0.6),
            Event::new("agent_response".to_string(), "I'm good".to_string(), 0.5),
        ];

        let summary = planner.compress_events(&events);
        assert!(summary.contains("user_input"));
        assert!(summary.contains("agent_response"));
        assert!(summary.contains("2 occurrence"));
    }

    #[test]
    fn test_suggest_actions() {
        let planner = Planner::new();
        let mut context = Context::new("test-user".to_string());
        
        // Add a habit that was completed a while ago
        let mut habit = Habit::new(
            "Exercise".to_string(),
            "Daily exercise".to_string(),
            HabitFrequency::Daily,
        );
        habit.last_completed = Some(Utc::now() - chrono::Duration::days(2));
        context.active_habits.push(habit);

        let suggestions = planner.suggest_actions(&context);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("Exercise")));
    }

    #[test]
    fn test_evaluate_intent() {
        let planner = Planner::new();
        let mut context = Context::new("test-user".to_string());
        context.current_activity = Some("sleeping".to_string());

        let intent = Intent::new(
            "device.control".to_string(),
            0.8,
            HashMap::new(),
            "Control device".to_string(),
        );

        let (appropriate, reason) = planner.evaluate_intent(&intent, &context);
        assert!(!appropriate);
        assert!(reason.contains("sleeping"));
    }
}
