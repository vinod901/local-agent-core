//! LLM abstraction layer
//!
//! Abstract interface for language models - works with any provider.
//! Supports local models (llama.cpp family) or cloud providers.

use crate::error::{AgentError, Result};
use crate::types::{LlmOptions, LlmResponse, LlmUsage};

/// Trait for LLM providers
pub trait LlmProvider: Send + Sync {
    /// Complete a prompt
    fn complete(&self, prompt: &str, options: &LlmOptions) -> Result<LlmResponse>;

    /// Get provider name
    fn name(&self) -> &str;

    /// Check if provider is available
    fn is_available(&self) -> bool {
        true
    }
}

/// Mock LLM provider for testing
pub struct MockLlmProvider {
    name: String,
}

impl MockLlmProvider {
    pub fn new() -> Self {
        Self {
            name: "mock".to_string(),
        }
    }
}

impl Default for MockLlmProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmProvider for MockLlmProvider {
    fn complete(&self, prompt: &str, _options: &LlmOptions) -> Result<LlmResponse> {
        // Simple mock response
        let response_text = if prompt.contains("weather") {
            "The weather is sunny today."
        } else if prompt.contains("time") {
            "It is currently 3:00 PM."
        } else {
            "I understand. How can I help you?"
        };

        Ok(LlmResponse {
            text: response_text.to_string(),
            finish_reason: "stop".to_string(),
            usage: LlmUsage {
                prompt_tokens: prompt.split_whitespace().count() as u32,
                completion_tokens: response_text.split_whitespace().count() as u32,
                total_tokens: (prompt.split_whitespace().count()
                    + response_text.split_whitespace().count()) as u32,
            },
        })
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Local LLM provider (placeholder for llama.cpp integration)
/// In production, this would integrate with llama.cpp via FFI or API
pub struct LocalLlmProvider {
    name: String,
    model_path: String,
}

impl LocalLlmProvider {
    pub fn new(model_path: String) -> Self {
        Self {
            name: "local-llama".to_string(),
            model_path,
        }
    }
}

impl LlmProvider for LocalLlmProvider {
    fn complete(&self, _prompt: &str, _options: &LlmOptions) -> Result<LlmResponse> {
        // TODO: Integrate with llama.cpp
        // For now, return an error indicating this needs implementation
        Err(AgentError::Llm(
            "Local LLM integration not yet implemented. Use MockLlmProvider for testing."
                .to_string(),
        ))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        // Check if model file exists
        std::path::Path::new(&self.model_path).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_provider() {
        let provider = MockLlmProvider::new();
        let options = LlmOptions::default();

        let response = provider.complete("What's the weather?", &options).unwrap();
        assert!(response.text.contains("weather") || response.text.contains("sunny"));
    }

    #[test]
    fn test_mock_provider_time() {
        let provider = MockLlmProvider::new();
        let options = LlmOptions::default();

        let response = provider.complete("What time is it?", &options).unwrap();
        assert!(response.text.contains("time") || response.text.contains("PM"));
    }
}
