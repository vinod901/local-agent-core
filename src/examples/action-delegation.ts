/**
 * Example demonstrating action delegation with permissions
 */

import {
  Agent,
  MockLLMProvider,
  AgentConfig,
  MockActionModule,
  Intent,
} from '../index';

async function main(): Promise<void> {
  console.log('=== Action Delegation Example ===\n');

  // Configure agent
  const config: AgentConfig = {
    userId: 'demo-user',
    llmProvider: 'mock',
    voiceEnabled: false,
    privacyMode: 'balanced',
    dataRetentionDays: 30,
    allowedModules: ['device', 'message', 'reminder'],
  };

  const agent = new Agent(config, new MockLLMProvider());

  // Register action modules
  const moduleRegistry = agent.getModuleRegistry();
  const permissionManager = agent.getPermissionManager();

  // Register a device control module
  const deviceModule = new MockActionModule('device', ['device.control', 'device.query']);
  moduleRegistry.register(deviceModule);

  // Register a message module
  const messageModule = new MockActionModule('message', ['message.send', 'message.read']);
  moduleRegistry.register(messageModule);

  console.log('Registered modules:');
  moduleRegistry.getAllModules().forEach((mod) => {
    console.log(`  - ${mod.getName()}: ${mod.getSupportedActions().join(', ')}`);
  });
  console.log('\n---\n');

  // Example 1: Try to execute an action without permission (should fail)
  console.log('Example 1: Execute action without permission');
  const intent1: Intent = {
    type: 'device.control',
    confidence: 0.9,
    parameters: { device: 'living room light', action: 'on' },
    requiresPermission: true,
    targetModule: 'device',
    timestamp: new Date(),
  };

  const result1 = await agent.executeIntent(intent1);
  console.log(`Execution result: ${result1.success ? 'Success' : 'Failed'}`);
  console.log(`Error: ${result1.error}`);
  console.log('\n---\n');

  // Example 2: Request and grant permission
  console.log('Example 2: Request and grant permission');
  const permRequest = permissionManager.requestPermission({
    intentId: 'test-intent-1',
    action: 'device.control',
    module: 'device',
    scope: ['device', 'action'],
    reasoning: 'User wants to control device',
  });

  console.log(`Permission request created: ${permRequest}`);

  // Grant permission (expires in 1 hour)
  const permission = permissionManager.grantPermission(permRequest, {
    expiresIn: 3600000,
  });

  console.log('Permission granted:');
  console.log(`  Module: ${permission?.module}`);
  console.log(`  Actions: ${permission?.actions.join(', ')}`);
  console.log(`  Scope: ${permission?.scope.join(', ')}`);
  console.log('\n---\n');

  // Example 3: Execute action with permission (should succeed)
  console.log('Example 3: Execute action with permission');
  const result2 = await agent.executeIntent(intent1);
  console.log(`Execution result: ${result2.success ? 'Success' : 'Failed'}`);
  if (result2.success) {
    console.log(`Result: ${JSON.stringify(result2.result)}`);
  } else {
    console.log(`Error: ${result2.error}`);
  }
  console.log('\n---\n');

  // Example 4: Check permissions
  console.log('Example 4: Check current permissions');
  const isPermitted = permissionManager.isPermitted('device', 'device.control', ['device', 'action']);
  console.log(`Device control permitted: ${isPermitted}`);

  const devicePermissions = permissionManager.getPermissions('device');
  console.log(`Total device permissions: ${devicePermissions.length}`);
  console.log('\n---\n');

  // Example 5: Revoke permissions
  console.log('Example 5: Revoke module permissions');
  permissionManager.revokeModule('device');
  console.log('Device module permissions revoked');

  const stillPermitted = permissionManager.isPermitted('device', 'device.control');
  console.log(`Device control still permitted: ${stillPermitted}`);
  console.log('\n---\n');

  console.log('Action delegation example complete!');
  console.log('Key takeaways:');
  console.log('  - Actions require explicit permission');
  console.log('  - Permissions are scoped and can expire');
  console.log('  - Modules are sandboxed and isolated');
  console.log('  - All actions are logged for transparency');
}

if (require.main === module) {
  main().catch(console.error);
}

export { main };
