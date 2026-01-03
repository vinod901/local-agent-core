//! Habit modeling module
//!
//! Statistical learning of user routines and habits:
//! - Descriptive, not moralizing
//! - Handles variance naturally
//! - No assumptions about "should"

use crate::error::Result;
use crate::types::{Habit, HabitFrequency};
use chrono::{DateTime, Utc, Duration};

/// Habit analyzer for understanding patterns
pub struct HabitAnalyzer {
    // Statistical threshold for considering a pattern
    confidence_threshold: f32,
}

impl HabitAnalyzer {
    pub fn new() -> Self {
        Self {
            confidence_threshold: 0.7,
        }
    }

    /// Calculate variance in habit completion times
    /// Returns statistical variance (lower = more consistent)
    pub fn calculate_variance(&self, completions: &[DateTime<Utc>]) -> Option<f32> {
        if completions.len() < 2 {
            return None;
        }

        // Calculate time differences between consecutive completions
        let mut intervals: Vec<i64> = Vec::new();
        for i in 1..completions.len() {
            let diff = completions[i]
                .signed_duration_since(completions[i - 1])
                .num_hours();
            intervals.push(diff);
        }

        // Calculate mean
        let mean = intervals.iter().sum::<i64>() as f32 / intervals.len() as f32;

        // Calculate variance
        let variance = intervals
            .iter()
            .map(|&x| {
                let diff = x as f32 - mean;
                diff * diff
            })
            .sum::<f32>()
            / intervals.len() as f32;

        Some(variance)
    }

    /// Suggest habit frequency based on completion history
    pub fn suggest_frequency(&self, completions: &[DateTime<Utc>]) -> Option<HabitFrequency> {
        if completions.len() < 3 {
            return None;
        }

        let mut intervals: Vec<i64> = Vec::new();
        for i in 1..completions.len() {
            let diff = completions[i]
                .signed_duration_since(completions[i - 1])
                .num_hours();
            intervals.push(diff);
        }

        let mean_hours = intervals.iter().sum::<i64>() as f32 / intervals.len() as f32;

        // Suggest frequency based on mean interval
        if mean_hours <= 30.0 {
            // ~Daily
            Some(HabitFrequency::Daily)
        } else if mean_hours <= 200.0 {
            // ~Weekly
            Some(HabitFrequency::Weekly)
        } else if mean_hours <= 800.0 {
            // ~Monthly
            Some(HabitFrequency::Monthly)
        } else {
            Some(HabitFrequency::Custom(format!(
                "every {} days",
                (mean_hours / 24.0) as u32
            )))
        }
    }

    /// Check if a habit is due based on history
    /// Returns confidence level (0.0 - 1.0)
    pub fn is_habit_due(&self, habit: &Habit, completions: &[DateTime<Utc>]) -> f32 {
        if completions.is_empty() {
            return 0.0;
        }

        let last_completion = completions.last().unwrap();
        let hours_since = Utc::now()
            .signed_duration_since(*last_completion)
            .num_hours();

        // Calculate expected interval based on frequency
        let expected_hours = match &habit.frequency {
            HabitFrequency::Daily => 24,
            HabitFrequency::Weekly => 168,
            HabitFrequency::Monthly => 720,
            HabitFrequency::Custom(s) => {
                // Parse "every X days" format
                if let Some(days) = s.strip_prefix("every ").and_then(|s| s.strip_suffix(" days")) {
                    days.parse::<i64>().unwrap_or(24) * 24
                } else {
                    24
                }
            }
        };

        // Calculate confidence based on how overdue it is
        // Use variance if available to adjust confidence
        let variance_multiplier = habit.variance.map(|v| {
            // Lower variance = higher confidence when due
            1.0 + (1.0 / (1.0 + v / 100.0))
        }).unwrap_or(1.0);

        let overdue_factor = hours_since as f32 / expected_hours as f32;
        let confidence = (overdue_factor * variance_multiplier).min(1.0);

        confidence
    }

    /// Summarize habit patterns for user
    pub fn summarize_habit(&self, habit: &Habit, completions: &[DateTime<Utc>]) -> String {
        if completions.is_empty() {
            return format!("{}: No completions yet", habit.name);
        }

        let variance = self.calculate_variance(completions);
        let consistency = variance.map(|v| {
            if v < 50.0 {
                "very consistent"
            } else if v < 200.0 {
                "moderately consistent"
            } else {
                "variable"
            }
        }).unwrap_or("unknown");

        let last = completions.last().unwrap();
        let hours_since = Utc::now().signed_duration_since(*last).num_hours();
        
        format!(
            "{}: {} pattern, last completed {} hours ago (total: {} times)",
            habit.name,
            consistency,
            hours_since,
            completions.len()
        )
    }
}

impl Default for HabitAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_calculate_variance_consistent() {
        let analyzer = HabitAnalyzer::new();
        let base = Utc::now();
        
        // Daily completions at same time
        let completions = vec![
            base - Duration::days(3),
            base - Duration::days(2),
            base - Duration::days(1),
            base,
        ];

        let variance = analyzer.calculate_variance(&completions);
        assert!(variance.is_some());
        assert!(variance.unwrap() < 10.0); // Very consistent
    }

    #[test]
    fn test_suggest_frequency_daily() {
        let analyzer = HabitAnalyzer::new();
        let base = Utc::now();
        
        let completions = vec![
            base - Duration::days(3),
            base - Duration::days(2),
            base - Duration::days(1),
            base,
        ];

        let freq = analyzer.suggest_frequency(&completions);
        assert!(matches!(freq, Some(HabitFrequency::Daily)));
    }

    #[test]
    fn test_suggest_frequency_weekly() {
        let analyzer = HabitAnalyzer::new();
        let base = Utc::now();
        
        let completions = vec![
            base - Duration::days(21),
            base - Duration::days(14),
            base - Duration::days(7),
            base,
        ];

        let freq = analyzer.suggest_frequency(&completions);
        assert!(matches!(freq, Some(HabitFrequency::Weekly)));
    }

    #[test]
    fn test_is_habit_due() {
        let analyzer = HabitAnalyzer::new();
        let base = Utc::now();
        
        let habit = Habit::new(
            "Test".to_string(),
            "Test habit".to_string(),
            HabitFrequency::Daily,
        );

        // Just completed - not due
        let completions = vec![base];
        let confidence = analyzer.is_habit_due(&habit, &completions);
        assert!(confidence < 0.1);

        // 24 hours ago - should be due
        let completions = vec![base - Duration::days(1)];
        let confidence = analyzer.is_habit_due(&habit, &completions);
        assert!(confidence > 0.5);
    }
}
