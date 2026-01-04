//! LLM abstraction layer
//!
//! Abstract interface for language models - works with any provider.
//! Supports local models (llama.cpp family) or cloud providers.

use crate::error::{AgentError, Result};
use crate::types::{LlmOptions, LlmResponse, LlmUsage};
use serde::{Deserialize, Serialize};

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

/// Ollama API request structure
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<OllamaOptions>,
}

/// Ollama API options
#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    #[serde(rename = "num_predict")]
    max_tokens: i32,
    top_p: f32,
}

/// Ollama API response
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    #[serde(default)]
    response: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

/// Ollama LLM provider - connects to local Ollama server
/// Ollama is easy to run locally and supports many models
pub struct OllamaProvider {
    name: String,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    /// Default endpoint is http://localhost:11434
    pub fn new(model: String) -> Self {
        Self {
            name: format!("ollama-{}", model),
            base_url: "http://localhost:11434".to_string(),
            model,
        }
    }

    /// Create a new Ollama provider with custom endpoint
    pub fn with_endpoint(model: String, base_url: String) -> Self {
        Self {
            name: format!("ollama-{}", model),
            base_url,
            model,
        }
    }
}

impl LlmProvider for OllamaProvider {
    fn complete(&self, prompt: &str, options: &LlmOptions) -> Result<LlmResponse> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(OllamaOptions {
                temperature: options.temperature,
                max_tokens: options.max_tokens as i32,
                top_p: options.top_p,
            }),
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| AgentError::Llm(format!("Failed to send request to Ollama: {}", e)))?;

        if !response.status().is_success() {
            return Err(AgentError::Llm(format!(
                "Ollama API returned error status: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .map_err(|e| AgentError::Llm(format!("Failed to parse Ollama response: {}", e)))?;

        Ok(LlmResponse {
            text: ollama_response.response,
            finish_reason: if ollama_response.done {
                "stop".to_string()
            } else {
                "length".to_string()
            },
            usage: LlmUsage {
                prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
                completion_tokens: ollama_response.eval_count.unwrap_or(0),
                total_tokens: ollama_response.prompt_eval_count.unwrap_or(0)
                    + ollama_response.eval_count.unwrap_or(0),
            },
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        // Try to connect to Ollama server
        let url = format!("{}/api/tags", self.base_url);
        reqwest::blocking::Client::new()
            .get(&url)
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
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

    #[test]
    fn test_ollama_provider_creation() {
        let provider = OllamaProvider::new("llama2".to_string());
        assert_eq!(provider.name(), "ollama-llama2");
    }

    #[test]
    fn test_ollama_provider_with_endpoint() {
        let provider = OllamaProvider::with_endpoint(
            "llama2".to_string(),
            "http://localhost:11434".to_string(),
        );
        assert_eq!(provider.name(), "ollama-llama2");
    }

    // This test requires Ollama to be running
    // Skip it if Ollama is not available
    #[test]
    #[ignore]
    fn test_ollama_provider_complete() {
        let provider = OllamaProvider::new("llama2".to_string());
        
        if !provider.is_available() {
            println!("Ollama not available, skipping test");
            return;
        }

        let options = LlmOptions::default();
        let response = provider.complete("Say hello in one word", &options);
        
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(!response.text.is_empty());
    }
}
