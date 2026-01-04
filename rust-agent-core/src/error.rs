//! Error types for the agent core

use thiserror::Error;

/// Main error type for agent operations
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Voice processing error: {0}")]
    Voice(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Invalid intent: {0}")]
    InvalidIntent(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// Result type alias using AgentError
pub type Result<T> = std::result::Result<T, AgentError>;
