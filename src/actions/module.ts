import { Intent, ActionResult } from '../core/types';

/**
 * Interface for action modules
 * Modules execute real-world actions (digital or physical)
 */
export interface ActionModule {
  /**
   * Get module name
   */
  getName(): string;

  /**
   * Get supported actions
   */
  getSupportedActions(): string[];

  /**
   * Execute an action
   */
  execute(intent: Intent): Promise<ActionResult>;

  /**
   * Check if module is available
   */
  isAvailable(): Promise<boolean>;

  /**
   * Get module capabilities and description
   */
  getCapabilities(): ModuleCapabilities;
}

/**
 * Module capabilities
 */
export interface ModuleCapabilities {
  name: string;
  description: string;
  actions: ActionDescriptor[];
  requiresHardware?: boolean;
  requiresNetwork?: boolean;
}

/**
 * Action descriptor
 */
export interface ActionDescriptor {
  name: string;
  description: string;
  parameters: Array<{
    name: string;
    type: string;
    required: boolean;
    description: string;
  }>;
  riskLevel: 'low' | 'medium' | 'high';
}

/**
 * Module registry for managing action modules
 */
export class ModuleRegistry {
  private modules: Map<string, ActionModule> = new Map();

  /**
   * Register an action module
   */
  register(module: ActionModule): void {
    this.modules.set(module.getName(), module);
  }

  /**
   * Unregister a module
   */
  unregister(moduleName: string): boolean {
    return this.modules.delete(moduleName);
  }

  /**
   * Get a module by name
   */
  getModule(moduleName: string): ActionModule | undefined {
    return this.modules.get(moduleName);
  }

  /**
   * Get all registered modules
   */
  getAllModules(): ActionModule[] {
    return Array.from(this.modules.values());
  }

  /**
   * Find modules that support an action
   */
  findModulesForAction(action: string): ActionModule[] {
    return this.getAllModules().filter((module) =>
      module.getSupportedActions().includes(action)
    );
  }

  /**
   * Get all available capabilities
   */
  async getAllCapabilities(): Promise<ModuleCapabilities[]> {
    const modules = this.getAllModules();
    const capabilities: ModuleCapabilities[] = [];

    for (const module of modules) {
      if (await module.isAvailable()) {
        capabilities.push(module.getCapabilities());
      }
    }

    return capabilities;
  }
}

/**
 * Mock action module for testing
 */
export class MockActionModule implements ActionModule {
  private name: string;
  private actions: string[];

  constructor(name: string, actions: string[]) {
    this.name = name;
    this.actions = actions;
  }

  getName(): string {
    return this.name;
  }

  getSupportedActions(): string[] {
    return this.actions;
  }

  async execute(intent: Intent): Promise<ActionResult> {
    return {
      success: true,
      intentId: intent.timestamp.toISOString(),
      module: this.name,
      action: intent.type,
      result: { message: `Mock execution of ${intent.type}` },
      timestamp: new Date(),
    };
  }

  async isAvailable(): Promise<boolean> {
    return true;
  }

  getCapabilities(): ModuleCapabilities {
    return {
      name: this.name,
      description: `Mock module: ${this.name}`,
      actions: this.actions.map((action) => ({
        name: action,
        description: `Mock action: ${action}`,
        parameters: [],
        riskLevel: 'low',
      })),
    };
  }
}
