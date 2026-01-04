//! # Local Agent Core (Rust)
//!
//! Privacy-first personal AI agent core designed to run locally.
//!
//! ## Core Principles
//! - **Separation of thinking and acting**: Agent emits intents, never executes actions
//! - **Local-first & privacy-first**: All cognition runs locally
//! - **Bounded agency**: Prepares, suggests, summarizes - human authorizes
//!
//! ## Architecture
//! This crate implements the cognitive core of the agent:
//! - Memory layer (SQLite-based)
//! - Habit/routine modeling
//! - Event compression and summarization
//! - Planning and reasoning
//! - Policy engine
//! - Intent generation (outputs structured JSON)

pub mod error;
pub mod types;
pub mod memory;
pub mod habit;
pub mod planner;
pub mod policy;
pub mod intent;
pub mod llm;
pub mod voice;

// Re-export commonly used types
pub use error::{AgentError, Result};
pub use types::{Intent, Context, Event, Habit as HabitType};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_structure() {
        // Basic sanity check that modules compile
        assert!(true);
    }
}
