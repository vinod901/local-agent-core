//! Voice I/O abstraction layer
//!
//! Interfaces for:
//! - Wake word detection (OpenWakeWord / Porcupine)
//! - Speech-to-text (whisper.cpp)
//! - Text-to-speech (Piper TTS)

use crate::error::{AgentError, Result};
use crate::types::VoiceTranscription;

/// Trait for wake word detection
pub trait WakeWordDetector: Send + Sync {
    /// Detect wake word in audio buffer
    /// Returns true if wake word detected
    fn detect(&self, audio_data: &[f32]) -> Result<bool>;

    /// Get wake word(s) this detector recognizes
    fn wake_words(&self) -> Vec<String>;
}

/// Trait for speech-to-text
pub trait SpeechToText: Send + Sync {
    /// Transcribe audio to text
    fn transcribe(&self, audio_data: &[u8]) -> Result<VoiceTranscription>;

    /// Get supported languages
    fn supported_languages(&self) -> Vec<String>;
}

/// Trait for text-to-speech
pub trait TextToSpeech: Send + Sync {
    /// Synthesize text to audio
    fn speak(&self, text: &str, voice: Option<&str>) -> Result<Vec<u8>>;

    /// Get available voices
    fn available_voices(&self) -> Vec<String>;
}

/// Mock wake word detector for testing
pub struct MockWakeWordDetector {
    wake_words: Vec<String>,
}

impl MockWakeWordDetector {
    pub fn new(wake_words: Vec<String>) -> Self {
        Self { wake_words }
    }
}

impl Default for MockWakeWordDetector {
    fn default() -> Self {
        Self::new(vec!["hey agent".to_string()])
    }
}

impl WakeWordDetector for MockWakeWordDetector {
    fn detect(&self, _audio_data: &[f32]) -> Result<bool> {
        // Mock implementation always returns false
        // In real implementation, would analyze audio_data
        Ok(false)
    }

    fn wake_words(&self) -> Vec<String> {
        self.wake_words.clone()
    }
}

/// Mock speech-to-text for testing
pub struct MockSpeechToText {
    languages: Vec<String>,
}

impl MockSpeechToText {
    pub fn new() -> Self {
        Self {
            languages: vec!["en".to_string()],
        }
    }
}

impl Default for MockSpeechToText {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeechToText for MockSpeechToText {
    fn transcribe(&self, audio_data: &[u8]) -> Result<VoiceTranscription> {
        // Mock implementation returns fixed transcription
        Ok(VoiceTranscription {
            text: "This is a mock transcription".to_string(),
            confidence: 0.95,
            language: "en".to_string(),
            duration_ms: audio_data.len() as u32,
        })
    }

    fn supported_languages(&self) -> Vec<String> {
        self.languages.clone()
    }
}

/// Mock text-to-speech for testing
pub struct MockTextToSpeech {
    voices: Vec<String>,
}

impl MockTextToSpeech {
    pub fn new() -> Self {
        Self {
            voices: vec!["default".to_string()],
        }
    }
}

impl Default for MockTextToSpeech {
    fn default() -> Self {
        Self::new()
    }
}

impl TextToSpeech for MockTextToSpeech {
    fn speak(&self, text: &str, _voice: Option<&str>) -> Result<Vec<u8>> {
        // Mock implementation returns dummy audio data
        // In real implementation, would synthesize actual audio
        Ok(vec![0u8; text.len() * 100])
    }

    fn available_voices(&self) -> Vec<String> {
        self.voices.clone()
    }
}

/// Whisper.cpp STT provider (placeholder for actual integration)
pub struct WhisperSttProvider {
    model_path: String,
}

impl WhisperSttProvider {
    pub fn new(model_path: String) -> Self {
        Self { model_path }
    }
}

impl SpeechToText for WhisperSttProvider {
    fn transcribe(&self, _audio_data: &[u8]) -> Result<VoiceTranscription> {
        // TODO: Integrate with whisper.cpp
        Err(AgentError::Voice(
            "Whisper.cpp integration not yet implemented. Use MockSpeechToText for testing."
                .to_string(),
        ))
    }

    fn supported_languages(&self) -> Vec<String> {
        // Whisper supports many languages
        vec!["en", "es", "fr", "de", "it", "pt", "nl", "pl", "ru", "zh"]
            .into_iter()
            .map(String::from)
            .collect()
    }
}

/// Piper TTS provider (placeholder for actual integration)
pub struct PiperTtsProvider {
    model_path: String,
}

impl PiperTtsProvider {
    pub fn new(model_path: String) -> Self {
        Self { model_path }
    }
}

impl TextToSpeech for PiperTtsProvider {
    fn speak(&self, _text: &str, _voice: Option<&str>) -> Result<Vec<u8>> {
        // TODO: Integrate with Piper TTS
        Err(AgentError::Voice(
            "Piper TTS integration not yet implemented. Use MockTextToSpeech for testing."
                .to_string(),
        ))
    }

    fn available_voices(&self) -> Vec<String> {
        // Piper has various voices
        vec!["en_US-lessac-medium".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_wake_word() {
        let detector = MockWakeWordDetector::default();
        assert_eq!(detector.wake_words(), vec!["hey agent"]);
        
        let audio = vec![0.0f32; 1000];
        let result = detector.detect(&audio).unwrap();
        assert!(!result); // Mock always returns false
    }

    #[test]
    fn test_mock_stt() {
        let stt = MockSpeechToText::new();
        let audio = vec![0u8; 1000];
        
        let transcription = stt.transcribe(&audio).unwrap();
        assert!(!transcription.text.is_empty());
        assert_eq!(transcription.language, "en");
    }

    #[test]
    fn test_mock_tts() {
        let tts = MockTextToSpeech::new();
        let text = "Hello world";
        
        let audio = tts.speak(text, None).unwrap();
        assert!(!audio.is_empty());
    }
}
