// Package gateway implements the intent gateway - the secure boundary
// between agent core (thinking) and device agents (acting)
package gateway

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"sync"

	"github.com/vinod901/local-agent-core/go-device-agent/pkg/intent"
)

// Gateway is the secure boundary between thinking and acting
type Gateway struct {
	executors map[string]Executor
	mu        sync.RWMutex
	logger    *log.Logger
}

// Executor interface for action executors
type Executor interface {
	// Name returns the executor name (matches target_module)
	Name() string

	// SupportedActions returns the actions this executor supports
	SupportedActions() []string

	// Execute executes an intent and returns a result
	Execute(ctx context.Context, intent *intent.Intent) (*ExecutionResult, error)

	// IsAvailable checks if the executor is available
	IsAvailable() bool
}

// ExecutionResult represents the result of executing an intent
type ExecutionResult struct {
	Success   bool                   `json:"success"`
	IntentID  string                 `json:"intent_id"`
	Module    string                 `json:"module"`
	Action    string                 `json:"action"`
	Result    map[string]interface{} `json:"result,omitempty"`
	Error     string                 `json:"error,omitempty"`
	Timestamp string                 `json:"timestamp"`
}

// NewGateway creates a new intent gateway
func NewGateway(logger *log.Logger) *Gateway {
	if logger == nil {
		logger = log.Default()
	}
	return &Gateway{
		executors: make(map[string]Executor),
		logger:    logger,
	}
}

// RegisterExecutor registers an action executor
func (g *Gateway) RegisterExecutor(executor Executor) {
	g.mu.Lock()
	defer g.mu.Unlock()

	name := executor.Name()
	g.executors[name] = executor
	g.logger.Printf("Registered executor: %s (actions: %v)", name, executor.SupportedActions())
}

// UnregisterExecutor removes an executor
func (g *Gateway) UnregisterExecutor(name string) {
	g.mu.Lock()
	defer g.mu.Unlock()

	delete(g.executors, name)
	g.logger.Printf("Unregistered executor: %s", name)
}

// ProcessIntent processes an intent through the gateway
func (g *Gateway) ProcessIntent(ctx context.Context, intentData []byte) (*ExecutionResult, error) {
	// Parse intent
	i, err := intent.ParseIntent(intentData)
	if err != nil {
		return nil, fmt.Errorf("failed to parse intent: %w", err)
	}

	// Validate intent
	if err := i.Validate(); err != nil {
		return nil, fmt.Errorf("invalid intent: %w", err)
	}

	g.logger.Printf("Processing intent: %s (type: %s, confidence: %.2f)",
		i.ID, i.IntentType, i.Confidence)

	// Find executor
	g.mu.RLock()
	executor, ok := g.executors[*i.TargetModule]
	g.mu.RUnlock()

	if !ok {
		return &ExecutionResult{
			Success:  false,
			IntentID: i.ID,
			Module:   *i.TargetModule,
			Action:   i.IntentType,
			Error:    fmt.Sprintf("no executor found for module: %s", *i.TargetModule),
		}, nil
	}

	// Check if executor is available
	if !executor.IsAvailable() {
		return &ExecutionResult{
			Success:  false,
			IntentID: i.ID,
			Module:   *i.TargetModule,
			Action:   i.IntentType,
			Error:    fmt.Sprintf("executor '%s' is not available", executor.Name()),
		}, nil
	}

	// Execute intent
	result, err := executor.Execute(ctx, i)
	if err != nil {
		g.logger.Printf("Execution error for intent %s: %v", i.ID, err)
		return &ExecutionResult{
			Success:  false,
			IntentID: i.ID,
			Module:   executor.Name(),
			Action:   i.IntentType,
			Error:    err.Error(),
		}, nil
	}

	g.logger.Printf("Intent %s executed successfully", i.ID)
	return result, nil
}

// GetExecutors returns all registered executors
func (g *Gateway) GetExecutors() []Executor {
	g.mu.RLock()
	defer g.mu.RUnlock()

	executors := make([]Executor, 0, len(g.executors))
	for _, e := range g.executors {
		executors = append(executors, e)
	}
	return executors
}

// ToJSON converts the result to JSON
func (r *ExecutionResult) ToJSON() ([]byte, error) {
	return json.MarshalIndent(r, "", "  ")
}
