.PHONY: help build test clean run-rust run-go run install deps fmt lint docker-build docker-up docker-down

# Default target
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(BLUE)Local Agent Core - Development Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Available targets:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'

# Installation and setup
install: deps ## Install all dependencies
	@echo "$(BLUE)Installing dependencies...$(NC)"

deps: ## Install Rust and Go dependencies
	@echo "$(BLUE)Installing Rust dependencies...$(NC)"
	cd rust-agent-core && cargo fetch
	@echo "$(BLUE)Installing Go dependencies...$(NC)"
	cd go-device-agent && go mod download
	@echo "$(GREEN)✓ Dependencies installed$(NC)"

# Build targets
build: build-rust build-go ## Build all components
	@echo "$(GREEN)✓ All components built successfully$(NC)"

build-rust: ## Build Rust agent core
	@echo "$(BLUE)Building Rust agent core...$(NC)"
	cd rust-agent-core && cargo build --release
	@echo "$(GREEN)✓ Rust agent core built$(NC)"

build-go: ## Build Go device agent
	@echo "$(BLUE)Building Go device agent...$(NC)"
	cd go-device-agent && go build -o ../bin/device-agent ./cmd/agent
	@echo "$(GREEN)✓ Go device agent built$(NC)"

# Test targets
test: test-rust test-go ## Run all tests
	@echo "$(GREEN)✓ All tests passed$(NC)"

test-rust: ## Run Rust tests
	@echo "$(BLUE)Running Rust tests...$(NC)"
	cd rust-agent-core && cargo test
	@echo "$(GREEN)✓ Rust tests passed$(NC)"

test-go: ## Run Go tests
	@echo "$(BLUE)Running Go tests...$(NC)"
	cd go-device-agent && go test ./...
	@echo "$(GREEN)✓ Go tests passed$(NC)"

# Format and lint
fmt: fmt-rust fmt-go ## Format all code
	@echo "$(GREEN)✓ Code formatted$(NC)"

fmt-rust: ## Format Rust code
	@echo "$(BLUE)Formatting Rust code...$(NC)"
	cd rust-agent-core && cargo fmt

fmt-go: ## Format Go code
	@echo "$(BLUE)Formatting Go code...$(NC)"
	cd go-device-agent && go fmt ./...

lint: lint-rust ## Run linters
	@echo "$(GREEN)✓ Linting complete$(NC)"

lint-rust: ## Run Rust linter
	@echo "$(BLUE)Running Rust linter...$(NC)"
	cd rust-agent-core && cargo clippy -- -D warnings

# Run targets
run: run-demo ## Run the demo application

run-demo: build-rust ## Run the complete workflow demo
	@echo "$(BLUE)Running complete workflow demo...$(NC)"
	cd rust-agent-core && cargo run --release --example complete_workflow

run-rust: build-rust ## Run Rust examples
	@echo "$(BLUE)Running Rust agent core example...$(NC)"
	cd rust-agent-core && cargo run --release --example complete_workflow

run-go: build-go ## Run Go device agent
	@echo "$(BLUE)Running Go device agent...$(NC)"
	cd go-device-agent && go run cmd/agent/main.go

# Docker targets
docker-build: ## Build Docker containers
	@echo "$(BLUE)Building Docker containers...$(NC)"
	docker-compose build
	@echo "$(GREEN)✓ Docker containers built$(NC)"

docker-up: ## Start Docker containers
	@echo "$(BLUE)Starting Docker containers...$(NC)"
	docker-compose up -d
	@echo "$(GREEN)✓ Docker containers started$(NC)"

docker-down: ## Stop Docker containers
	@echo "$(BLUE)Stopping Docker containers...$(NC)"
	docker-compose down
	@echo "$(GREEN)✓ Docker containers stopped$(NC)"

docker-logs: ## View Docker logs
	docker-compose logs -f

# Clean targets
clean: clean-rust clean-go clean-bin ## Clean all build artifacts
	@echo "$(GREEN)✓ Cleaned all build artifacts$(NC)"

clean-rust: ## Clean Rust build artifacts
	@echo "$(BLUE)Cleaning Rust artifacts...$(NC)"
	cd rust-agent-core && cargo clean

clean-go: ## Clean Go build artifacts
	@echo "$(BLUE)Cleaning Go artifacts...$(NC)"
	cd go-device-agent && go clean

clean-bin: ## Remove binary directory
	@echo "$(BLUE)Cleaning binary directory...$(NC)"
	rm -rf bin

# Development helpers
dev: ## Run in development mode with auto-reload
	@echo "$(BLUE)Starting development mode...$(NC)"
	cd rust-agent-core && cargo watch -x 'run --example complete_workflow'

check: ## Quick check (format + lint + test)
	@echo "$(BLUE)Running quick check...$(NC)"
	$(MAKE) fmt
	$(MAKE) lint
	$(MAKE) test
	@echo "$(GREEN)✓ All checks passed$(NC)"
