// Package main is the entry point for the device agent
package main

import (
	"context"
	"encoding/json"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/vinod901/local-agent-core/go-device-agent/pkg/executor"
	"github.com/vinod901/local-agent-core/go-device-agent/pkg/gateway"
)

func main() {
	logger := log.New(os.Stdout, "[device-agent] ", log.LstdFlags)
	logger.Println("Starting device agent...")

	// Create intent gateway
	gw := gateway.NewGateway(logger)

	// Register executors
	gw.RegisterExecutor(executor.NewDeviceExecutor())
	gw.RegisterExecutor(executor.NewNotificationExecutor())
	gw.RegisterExecutor(executor.NewMockExecutor("time", []string{"time.query"}))
	gw.RegisterExecutor(executor.NewMockExecutor("weather", []string{"weather.query"}))

	logger.Println("Device agent ready. Registered executors:")
	for _, e := range gw.GetExecutors() {
		logger.Printf("  - %s: %v", e.Name(), e.SupportedActions())
	}

	// Example: Process a sample intent
	sampleIntent := `{
		"id": "550e8400-e29b-41d4-a716-446655440000",
		"intent_type": "device.control",
		"confidence": 0.9,
		"parameters": {
			"device": "living_room_light",
			"action": "on"
		},
		"reasoning": "User wants to turn on the living room light",
		"requires_permission": true,
		"target_module": "device",
		"created_at": "2026-01-03T15:00:00Z"
	}`

	logger.Println("\nProcessing sample intent...")
	ctx := context.Background()
	result, err := gw.ProcessIntent(ctx, []byte(sampleIntent))
	if err != nil {
		logger.Printf("Error processing intent: %v", err)
	} else {
		resultJSON, _ := json.MarshalIndent(result, "", "  ")
		logger.Printf("Result:\n%s", string(resultJSON))
	}

	// Wait for interrupt signal
	logger.Println("\nDevice agent running. Press Ctrl+C to exit.")
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	<-sigChan

	logger.Println("\nShutting down device agent...")
}
