// Package intent defines the intent structure received from the Rust agent core
package intent

import (
	"encoding/json"
	"time"
)

// Intent represents a structured intent emitted by the agent core
// This is the security boundary - agent emits intents, device agents execute
type Intent struct {
	ID                string                 `json:"id"`
	IntentType        string                 `json:"intent_type"`
	Confidence        float32                `json:"confidence"`
	Parameters        map[string]interface{} `json:"parameters"`
	Reasoning         string                 `json:"reasoning"`
	RequiresPermission bool                   `json:"requires_permission"`
	TargetModule      *string                `json:"target_module,omitempty"`
	CreatedAt         time.Time              `json:"created_at"`
}

// ParseIntent parses a JSON intent from the agent core
func ParseIntent(data []byte) (*Intent, error) {
	var intent Intent
	if err := json.Unmarshal(data, &intent); err != nil {
		return nil, err
	}
	return &intent, nil
}

// ToJSON converts the intent to JSON
func (i *Intent) ToJSON() ([]byte, error) {
	return json.MarshalIndent(i, "", "  ")
}

// Validate checks if the intent is valid
func (i *Intent) Validate() error {
	if i.IntentType == "" {
		return &ValidationError{Field: "intent_type", Message: "cannot be empty"}
	}
	if i.Confidence < 0.0 || i.Confidence > 1.0 {
		return &ValidationError{Field: "confidence", Message: "must be between 0.0 and 1.0"}
	}
	if i.Reasoning == "" {
		return &ValidationError{Field: "reasoning", Message: "cannot be empty"}
	}
	return nil
}

// ValidationError represents a validation error
type ValidationError struct {
	Field   string
	Message string
}

func (e *ValidationError) Error() string {
	return "validation error for field '" + e.Field + "': " + e.Message
}
