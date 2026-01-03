import { Permission, PermissionRequest } from '../core/types';

/**
 * Permission manager for action delegation
 * Ensures all real-world actions require explicit permission
 */
export class PermissionManager {
  private permissions: Map<string, Permission[]> = new Map();
  private pendingRequests: Map<string, PermissionRequest> = new Map();

  /**
   * Request permission for an action
   */
  requestPermission(request: PermissionRequest): string {
    const requestId = this.generateId();
    this.pendingRequests.set(requestId, request);
    return requestId;
  }

  /**
   * Grant permission
   */
  grantPermission(
    requestId: string,
    options?: { expiresIn?: number }
  ): Permission | null {
    const request = this.pendingRequests.get(requestId);
    if (!request) {
      return null;
    }

    const permission: Permission = {
      module: request.module,
      actions: [request.action],
      scope: request.scope,
      grantedAt: new Date(),
    };

    if (options?.expiresIn) {
      permission.expiresAt = new Date(Date.now() + options.expiresIn);
    }

    const modulePermissions = this.permissions.get(request.module) || [];
    modulePermissions.push(permission);
    this.permissions.set(request.module, modulePermissions);

    this.pendingRequests.delete(requestId);
    return permission;
  }

  /**
   * Deny permission
   */
  denyPermission(requestId: string): boolean {
    return this.pendingRequests.delete(requestId);
  }

  /**
   * Check if action is permitted
   */
  isPermitted(module: string, action: string, scope?: string[]): boolean {
    const modulePermissions = this.permissions.get(module);
    if (!modulePermissions) {
      return false;
    }

    const now = new Date();

    return modulePermissions.some((perm) => {
      // Check if permission has expired
      if (perm.expiresAt && perm.expiresAt < now) {
        return false;
      }

      // Check if action is permitted
      if (!perm.actions.includes(action)) {
        return false;
      }

      // Check scope if provided
      if (scope && scope.length > 0) {
        return scope.every((s) => perm.scope.includes(s));
      }

      return true;
    });
  }

  /**
   * Revoke all permissions for a module
   */
  revokeModule(module: string): void {
    this.permissions.delete(module);
  }

  /**
   * Get all permissions for a module
   */
  getPermissions(module: string): Permission[] {
    return this.permissions.get(module) || [];
  }

  /**
   * Get all pending permission requests
   */
  getPendingRequests(): PermissionRequest[] {
    return Array.from(this.pendingRequests.values());
  }

  /**
   * Clear expired permissions
   */
  clearExpired(): number {
    const now = new Date();
    let cleared = 0;

    for (const [module, permissions] of this.permissions.entries()) {
      const active = permissions.filter((perm) => !perm.expiresAt || perm.expiresAt >= now);
      cleared += permissions.length - active.length;

      if (active.length === 0) {
        this.permissions.delete(module);
      } else {
        this.permissions.set(module, active);
      }
    }

    return cleared;
  }

  private generateId(): string {
    return `perm-${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
  }
}
