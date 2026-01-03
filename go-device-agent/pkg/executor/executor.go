// Package executor provides example executors for different action types
package executor

import (
	"context"
	"fmt"
	"time"

	"github.com/vinod901/local-agent-core/go-device-agent/pkg/gateway"
	"github.com/vinod901/local-agent-core/go-device-agent/pkg/intent"
)

// MockExecutor is a simple mock executor for testing
type MockExecutor struct {
	name    string
	actions []string
}

// NewMockExecutor creates a new mock executor
func NewMockExecutor(name string, actions []string) *MockExecutor {
	return &MockExecutor{
		name:    name,
		actions: actions,
	}
}

func (e *MockExecutor) Name() string {
	return e.name
}

func (e *MockExecutor) SupportedActions() []string {
	return e.actions
}

func (e *MockExecutor) Execute(ctx context.Context, i *intent.Intent) (*gateway.ExecutionResult, error) {
	// Mock execution - just return success
	return &gateway.ExecutionResult{
		Success:   true,
		IntentID:  i.ID,
		Module:    e.name,
		Action:    i.IntentType,
		Result:    map[string]interface{}{"message": fmt.Sprintf("Mock execution of %s", i.IntentType)},
		Timestamp: time.Now().Format(time.RFC3339),
	}, nil
}

func (e *MockExecutor) IsAvailable() bool {
	return true
}

// DeviceExecutor handles device control actions
// This would integrate with actual device APIs in production
type DeviceExecutor struct {
	devices map[string]bool // device name -> state (on/off)
}

// NewDeviceExecutor creates a new device executor
func NewDeviceExecutor() *DeviceExecutor {
	return &DeviceExecutor{
		devices: make(map[string]bool),
	}
}

func (e *DeviceExecutor) Name() string {
	return "device"
}

func (e *DeviceExecutor) SupportedActions() []string {
	return []string{"device.control", "device.query"}
}

func (e *DeviceExecutor) Execute(ctx context.Context, i *intent.Intent) (*gateway.ExecutionResult, error) {
	result := &gateway.ExecutionResult{
		IntentID:  i.ID,
		Module:    "device",
		Action:    i.IntentType,
		Timestamp: time.Now().Format(time.RFC3339),
	}

	switch i.IntentType {
	case "device.control":
		deviceName, ok := i.Parameters["device"].(string)
		if !ok {
			result.Success = false
			result.Error = "missing or invalid 'device' parameter"
			return result, nil
		}

		action, ok := i.Parameters["action"].(string)
		if !ok {
			result.Success = false
			result.Error = "missing or invalid 'action' parameter"
			return result, nil
		}

		// Mock device control
		if action == "on" {
			e.devices[deviceName] = true
		} else if action == "off" {
			e.devices[deviceName] = false
		}

		result.Success = true
		result.Result = map[string]interface{}{
			"device": deviceName,
			"action": action,
			"state":  e.devices[deviceName],
		}

	case "device.query":
		deviceName, ok := i.Parameters["device"].(string)
		if !ok {
			result.Success = false
			result.Error = "missing or invalid 'device' parameter"
			return result, nil
		}

		state, exists := e.devices[deviceName]
		result.Success = true
		result.Result = map[string]interface{}{
			"device": deviceName,
			"exists": exists,
			"state":  state,
		}

	default:
		result.Success = false
		result.Error = fmt.Sprintf("unsupported action: %s", i.IntentType)
	}

	return result, nil
}

func (e *DeviceExecutor) IsAvailable() bool {
	return true
}

// NotificationExecutor handles notification actions
type NotificationExecutor struct{}

// NewNotificationExecutor creates a new notification executor
func NewNotificationExecutor() *NotificationExecutor {
	return &NotificationExecutor{}
}

func (e *NotificationExecutor) Name() string {
	return "notification"
}

func (e *NotificationExecutor) SupportedActions() []string {
	return []string{"notification.send", "notification.clear"}
}

func (e *NotificationExecutor) Execute(ctx context.Context, i *intent.Intent) (*gateway.ExecutionResult, error) {
	result := &gateway.ExecutionResult{
		IntentID:  i.ID,
		Module:    "notification",
		Action:    i.IntentType,
		Timestamp: time.Now().Format(time.RFC3339),
	}

	switch i.IntentType {
	case "notification.send":
		message, ok := i.Parameters["message"].(string)
		if !ok {
			result.Success = false
			result.Error = "missing or invalid 'message' parameter"
			return result, nil
		}

		// Mock notification send
		fmt.Printf("📢 Notification: %s\n", message)

		result.Success = true
		result.Result = map[string]interface{}{
			"message": message,
			"sent":    true,
		}

	case "notification.clear":
		// Mock notification clear
		result.Success = true
		result.Result = map[string]interface{}{
			"cleared": true,
		}

	default:
		result.Success = false
		result.Error = fmt.Sprintf("unsupported action: %s", i.IntentType)
	}

	return result, nil
}

func (e *NotificationExecutor) IsAvailable() bool {
	return true
}
