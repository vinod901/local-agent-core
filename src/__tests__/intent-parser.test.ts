import { IntentParser } from '../core/intent-parser';

describe('IntentParser', () => {
  let parser: IntentParser;

  beforeEach(() => {
    parser = new IntentParser();
  });

  describe('intent parsing', () => {
    it('should parse reminder intent', () => {
      const intents = parser.parseIntents('Remind me to call John at 3pm');

      expect(intents.length).toBe(1);
      expect(intents[0].type).toBe('reminder.create');
      expect(intents[0].parameters.task).toBe('call John');
      expect(intents[0].parameters.time).toBe('3pm');
    });

    it('should parse weather query intent', () => {
      const intents = parser.parseIntents("What's the weather like?");

      expect(intents.length).toBe(1);
      expect(intents[0].type).toBe('weather.query');
    });

    it('should parse time query intent', () => {
      const intents = parser.parseIntents('What time is it?');

      expect(intents.length).toBe(1);
      expect(intents[0].type).toBe('time.query');
    });

    it('should parse message send intent', () => {
      const intents = parser.parseIntents('Send a message to Alice saying hello');

      expect(intents.length).toBe(1);
      expect(intents[0].type).toBe('message.send');
      expect(intents[0].parameters.recipient).toBe('Alice');
      expect(intents[0].parameters.message).toBe('hello');
      expect(intents[0].requiresPermission).toBe(true);
    });

    it('should parse device control intent', () => {
      const intents = parser.parseIntents('Turn on the living room light');

      expect(intents.length).toBe(1);
      expect(intents[0].type).toBe('device.control');
      expect(intents[0].parameters.action).toBe('on');
      expect(intents[0].parameters.device).toBe('living room light');
      expect(intents[0].requiresPermission).toBe(true);
    });

    it('should return empty array for unrecognized input', () => {
      const intents = parser.parseIntents('This is just a random statement');

      expect(intents.length).toBe(0);
    });
  });

  describe('permission detection', () => {
    it('should mark sensitive intents as requiring permission', () => {
      const intents = parser.parseIntents('Send a message to Bob saying test');

      expect(intents[0].requiresPermission).toBe(true);
    });

    it('should not require permission for safe intents', () => {
      const intents = parser.parseIntents('What time is it?');

      expect(intents[0].requiresPermission).toBe(false);
    });
  });

  describe('target module extraction', () => {
    it('should extract module from intent type', () => {
      const intents = parser.parseIntents('What time is it?');
      const module = parser.getTargetModule(intents[0]);

      expect(module).toBe('time');
    });

    it('should extract device module', () => {
      const intents = parser.parseIntents('Turn off the bedroom light');
      const module = parser.getTargetModule(intents[0]);

      expect(module).toBe('device');
    });
  });
});
