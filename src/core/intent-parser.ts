import { Intent } from '../core/types';

/**
 * Intent parser - extracts structured intents from LLM responses
 */
export class IntentParser {
  /**
   * Parse intents from text
   */
  parseIntents(text: string): Intent[] {
    const intents: Intent[] = [];

    // Pattern matching for common intents
    const patterns = [
      {
        pattern: /remind me to (.+?) (?:at|on) (.+)/i,
        type: 'reminder.create',
        extractor: (match: RegExpMatchArray) => ({
          task: match[1],
          time: match[2],
        }),
      },
      {
        pattern: /what'?s? the weather/i,
        type: 'weather.query',
        extractor: () => ({}),
      },
      {
        pattern: /what time is it/i,
        type: 'time.query',
        extractor: () => ({}),
      },
      {
        pattern: /send (?:a )?message to (.+?) saying (.+)/i,
        type: 'message.send',
        extractor: (match: RegExpMatchArray) => ({
          recipient: match[1],
          message: match[2],
        }),
      },
      {
        pattern: /turn (on|off) (?:the )?(.+)/i,
        type: 'device.control',
        extractor: (match: RegExpMatchArray) => ({
          action: match[1],
          device: match[2],
        }),
      },
    ];

    for (const { pattern, type, extractor } of patterns) {
      const match = text.match(pattern);
      if (match) {
        const intent: Intent = {
          type,
          confidence: 0.8,
          parameters: extractor(match),
          requiresPermission: this.requiresPermission(type),
          timestamp: new Date(),
        };
        intents.push(intent);
      }
    }

    return intents;
  }

  /**
   * Determine if an intent type requires permission
   */
  private requiresPermission(intentType: string): boolean {
    const permissionRequired = [
      'message.send',
      'device.control',
      'file.write',
      'network.request',
      'location.access',
    ];

    return permissionRequired.some((prefix) => intentType.startsWith(prefix));
  }

  /**
   * Extract target module from intent type
   */
  getTargetModule(intent: Intent): string {
    const [module] = intent.type.split('.');
    return module;
  }
}
