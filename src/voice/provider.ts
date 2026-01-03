/**
 * Abstract interface for voice input (speech-to-text)
 */
export interface VoiceInputProvider {
  /**
   * Start listening for voice input
   */
  startListening(): Promise<void>;

  /**
   * Stop listening for voice input
   */
  stopListening(): Promise<void>;

  /**
   * Process audio and return transcribed text
   */
  transcribe(audioData: Buffer): Promise<VoiceTranscription>;

  /**
   * Check if provider is available
   */
  isAvailable(): Promise<boolean>;

  /**
   * Get provider name
   */
  getName(): string;
}

/**
 * Voice transcription result
 */
export interface VoiceTranscription {
  text: string;
  confidence: number;
  language: string;
  alternates?: Array<{ text: string; confidence: number }>;
}

/**
 * Abstract interface for voice output (text-to-speech)
 */
export interface VoiceOutputProvider {
  /**
   * Synthesize text to speech
   */
  speak(text: string, options?: VoiceSpeechOptions): Promise<void>;

  /**
   * Generate audio buffer from text
   */
  synthesize(text: string, options?: VoiceSpeechOptions): Promise<Buffer>;

  /**
   * Stop current speech
   */
  stop(): Promise<void>;

  /**
   * Check if provider is available
   */
  isAvailable(): Promise<boolean>;

  /**
   * Get provider name
   */
  getName(): string;
}

/**
 * Options for speech synthesis
 */
export interface VoiceSpeechOptions {
  voice?: string;
  language?: string;
  speed?: number;
  pitch?: number;
  volume?: number;
}

/**
 * Mock voice input provider for testing
 */
export class MockVoiceInputProvider implements VoiceInputProvider {
  private listening = false;
  private name = 'mock-voice-input';

  async startListening(): Promise<void> {
    this.listening = true;
  }

  async stopListening(): Promise<void> {
    this.listening = false;
  }

  async transcribe(audioData: Buffer): Promise<VoiceTranscription> {
    // Mock transcription
    return {
      text: 'Mock transcription of audio data',
      confidence: 0.95,
      language: 'en-US',
    };
  }

  async isAvailable(): Promise<boolean> {
    return true;
  }

  getName(): string {
    return this.name;
  }
}

/**
 * Mock voice output provider for testing
 */
export class MockVoiceOutputProvider implements VoiceOutputProvider {
  private name = 'mock-voice-output';

  async speak(text: string, _options?: VoiceSpeechOptions): Promise<void> {
    console.log(`[Voice Output]: ${text}`);
  }

  async synthesize(text: string, _options?: VoiceSpeechOptions): Promise<Buffer> {
    // Return empty buffer for mock
    return Buffer.from(text);
  }

  async stop(): Promise<void> {
    // Nothing to stop in mock
  }

  async isAvailable(): Promise<boolean> {
    return true;
  }

  getName(): string {
    return this.name;
  }
}
