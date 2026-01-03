import { PermissionManager } from '../actions/permission-manager';

describe('PermissionManager', () => {
  let permissionManager: PermissionManager;

  beforeEach(() => {
    permissionManager = new PermissionManager();
  });

  describe('permission requests', () => {
    it('should create a permission request', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'User wants to control lights',
      });

      expect(requestId).toBeDefined();
      expect(typeof requestId).toBe('string');
    });

    it('should list pending requests', () => {
      permissionManager.requestPermission({
        intentId: 'test-intent-1',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      const pending = permissionManager.getPendingRequests();
      expect(pending.length).toBe(1);
    });
  });

  describe('permission grants', () => {
    it('should grant permission', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      const permission = permissionManager.grantPermission(requestId);

      expect(permission).toBeDefined();
      expect(permission?.module).toBe('device');
      expect(permission?.actions).toContain('device.control');
    });

    it('should remove request after granting', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      permissionManager.grantPermission(requestId);
      const pending = permissionManager.getPendingRequests();

      expect(pending.length).toBe(0);
    });
  });

  describe('permission checks', () => {
    it('should return false when no permission exists', () => {
      const permitted = permissionManager.isPermitted('device', 'device.control');
      expect(permitted).toBe(false);
    });

    it('should return true when permission exists', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      permissionManager.grantPermission(requestId);

      const permitted = permissionManager.isPermitted('device', 'device.control');
      expect(permitted).toBe(true);
    });

    it('should check scope when provided', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      permissionManager.grantPermission(requestId);

      expect(permissionManager.isPermitted('device', 'device.control', ['living room'])).toBe(
        true
      );
      expect(permissionManager.isPermitted('device', 'device.control', ['bedroom'])).toBe(false);
    });
  });

  describe('permission revocation', () => {
    it('should revoke all permissions for a module', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      permissionManager.grantPermission(requestId);
      expect(permissionManager.isPermitted('device', 'device.control')).toBe(true);

      permissionManager.revokeModule('device');
      expect(permissionManager.isPermitted('device', 'device.control')).toBe(false);
    });
  });

  describe('permission expiration', () => {
    it('should grant permission with expiration', () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      const permission = permissionManager.grantPermission(requestId, {
        expiresIn: 1000,
      });

      expect(permission?.expiresAt).toBeDefined();
    });

    it('should clear expired permissions', async () => {
      const requestId = permissionManager.requestPermission({
        intentId: 'test-intent',
        action: 'device.control',
        module: 'device',
        scope: ['living room'],
        reasoning: 'Test',
      });

      permissionManager.grantPermission(requestId, {
        expiresIn: 100,
      });

      // Wait for expiration
      await new Promise((resolve) => setTimeout(resolve, 150));

      const cleared = permissionManager.clearExpired();
      expect(cleared).toBe(1);
    });
  });
});
