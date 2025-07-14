# Use modern docker compose syntax
COMPOSE_CMD := docker compose -f docker/docker-compose.yml --env-file .env

# ✅ VERIFIED WORKING COMMANDS:
# - setup, build, start, stop, restart, rebuild, status, logs, clean
# - db (database shell access)
# - quick-test (works with common crate tests, notes dependency issues for full tests)
# ❌ TODO/DEVELOPMENT NEEDED:
# - test (needs implementation)
# - backend-shell (container lacks shell tools)

.PHONY: help setup build start stop restart rebuild logs clean status db test quick-test backend-shell

# Default target
help:
	@echo "RustTracker - Task Management App"
	@echo ""
	@echo "✅ Working Commands:"
	@echo "  make setup         - Initial setup and start all services"
	@echo "  make build         - Build all Docker images"
	@echo "  make start         - Start all services"
	@echo "  make stop          - Stop all services"
	@echo "  make restart       - Restart all services"
	@echo "  make rebuild       - Rebuild and start all services"
	@echo "  make status        - Show service status"
	@echo "  make logs          - Show logs for all services"
	@echo "  make clean         - Stop services and clean up"
	@echo "  make db            - Connect to database shell"
	@echo "  make quick-test    - Run quick test suite (common crate tests work)"
	@echo ""
	@echo "❌ TODO/Development Needed:"
	@echo "  make test          - Run comprehensive test suite (needs implementation)"
	@echo "  make backend-shell - Backend container shell (lacks tools)"
	@echo ""
	@echo "🚀 Quick Start:"
	@echo "  make setup         # Start everything"
	@echo "  make status        # Verify services are running"
	@echo "  Frontend: http://localhost:3000 | Backend: http://localhost:8080"

# =============================================================================
# ✅ WORKING - Setup Commands
# =============================================================================

setup:
	@echo "Setting up RustTracker development environment..."
	@$(COMPOSE_CMD) up --build -d
	@echo "Services started! Frontend: http://localhost:3000 | Backend: http://localhost:8080"

build:
	@echo "Building all Docker images..."
	@$(COMPOSE_CMD) build
	@echo "All images built successfully!"

# =============================================================================
# ✅ WORKING - Service Management
# =============================================================================

start:
	@echo "Starting RustTracker services..."
	@$(COMPOSE_CMD) up -d
	@echo "Services started! Frontend: http://localhost:3000 | Backend: http://localhost:8080"

stop:
	@echo "Stopping RustTracker services..."
	@$(COMPOSE_CMD) down
	@echo "Services stopped!"

restart:
	@echo "Restarting RustTracker services..."
	@$(COMPOSE_CMD) down
	@$(COMPOSE_CMD) up -d
	@echo "Services restarted!"

rebuild:
	@echo "Rebuilding and starting RustTracker services..."
	@$(COMPOSE_CMD) down
	@$(COMPOSE_CMD) up --build -d
	@echo "Services rebuilt and started!"

status:
	@echo "Service status:"
	@$(COMPOSE_CMD) ps

logs:
	@echo "Showing logs for all services..."
	@$(COMPOSE_CMD) logs -f

clean:
	@echo "Cleaning up RustTracker..."
	@$(COMPOSE_CMD) down -v
	@docker system prune -f
	@echo "Cleanup complete!"

# =============================================================================
# ✅ WORKING - Database Access
# =============================================================================

db:
	@echo "Connecting to database..."
	@$(COMPOSE_CMD) exec db psql -U postgres -d rusttracker


quick-test:
	@echo "Running Docker-based quick test suite..."
	@./scripts/quick-test.sh

# =============================================================================
# ❌ TODO - Development Needed
# =============================================================================

test:
	@echo "TODO: Comprehensive test suite needs implementation"
	@echo "Current issue: Docker containers lack Rust toolchain for testing"
	@echo "Consider: dedicated test containers or local Rust environment"

backend-shell:
	@echo "TODO: Backend container shell access needs debugging"
	@echo "Current issue: Container lacks shell utilities (ps, bash, etc.)"
	@echo "Container runs minimal Debian with just the Rust binary"
	@$(COMPOSE_CMD) exec backend sh || echo "Failed - minimal container without shell tools"
