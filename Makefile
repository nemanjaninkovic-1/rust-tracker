# Use modern docker compose syntax
COMPOSE_CMD := docker compose -f docker/docker-compose.yml --env-file .env

# ✓ VERIFIED WORKING COMMANDS:
# - setup, build, start, stop, restart, rebuild, status, logs, clean
# - db (database shell access)
# - test (comprehensive test suite with database setup)
# ✗ TODO/DEVELOPMENT NEEDED:
# - backend-shell (container lacks shell tools)

.PHONY: help setup build start stop restart rebuild logs clean status db test test-only coverage backend-shell dev-frontend build-css

# Default target
help:
	@echo "RustTracker - Task Management App"
	@echo ""
	@echo "Working Commands:"
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
	@echo "  make test          - Run comprehensive test suite with coverage analysis (124+ tests, 70% minimum coverage)"
	@echo "  make test-only     - Run comprehensive test suite only (no coverage analysis)"
	@echo "  make coverage      - Generate test coverage report only (70% minimum)"
	@echo ""
	@echo "Frontend Development:"
	@echo "  make dev-frontend  - Start frontend development server"
	@echo "  make build-css     - Build Tailwind CSS"
	@echo ""
	@echo "TODO/Development Needed:"
	@echo "  make backend-shell - Backend container shell (lacks tools)"
	@echo ""
	@echo "Quick Start:"
	@echo "  make setup         # Start everything"
	@echo "  make status        # Verify services are running"
	@echo "  Frontend: http://localhost:3000 | Backend: http://localhost:8080"

# =============================================================================
# Working - Setup Commands
# =============================================================================

setup:
	@echo "Setting up RustTracker development environment..."
	@$(COMPOSE_CMD) up --build -d --quiet-pull
	@echo "Services started! Frontend: http://localhost:3000 | Backend: http://localhost:8080"

build:
	@echo "Building all Docker images..."
	@$(COMPOSE_CMD) build --quiet
	@echo "All images built successfully!"

# =============================================================================
# Working - Service Management
# =============================================================================

start:
	@echo "Starting RustTracker services..."
	@$(COMPOSE_CMD) up -d --quiet-pull
	@echo "Services started! Frontend: http://localhost:3000 | Backend: http://localhost:8080"

stop:
	@echo "Stopping RustTracker services..."
	@$(COMPOSE_CMD) down
	@echo "Services stopped!"

restart:
	@echo "Restarting RustTracker services..."
	@$(COMPOSE_CMD) down
	@$(COMPOSE_CMD) up -d --quiet-pull
	@echo "Services restarted!"

rebuild:
	@echo "Rebuilding and starting RustTracker services..."
	@$(COMPOSE_CMD) down
	@$(COMPOSE_CMD) up --build -d --quiet-pull
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
# Frontend Development
# =============================================================================

dev-frontend:
	@echo "Starting frontend development server..."
	@./scripts/frontend_dev_server.sh

build-css:
	@echo "Building Tailwind CSS..."
	@cd frontend && npm install
	@cd frontend && npm run build-css-prod
	@echo "CSS build complete!"

# =============================================================================
# Working - Database Access
# =============================================================================

db:
	@echo "Connecting to database..."
	@$(COMPOSE_CMD) exec db psql -U postgres -d rusttracker

# =============================================================================
# Working - Testing
# =============================================================================

test:
	@echo "Running comprehensive test suite with coverage analysis..."
	@echo "This will run all tests including backend tests with proper database setup"
	@echo "Coverage requirement: 70% minimum"
	@echo ""
	@echo "Note: If running in a restricted Docker environment, coverage analysis may fail"
	@echo "due to ASLR permission restrictions. All tests will still run successfully."
	@echo ""
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true
	@docker compose -f docker/docker-compose.test.yml up --build --abort-on-container-exit --quiet-pull
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true

test-only:
	@echo "Running comprehensive test suite (no coverage analysis)..."
	@echo "This will run all tests including backend tests with proper database setup"
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true
	@TEST_MODE=tests-only docker compose -f docker/docker-compose.test.yml up --build --abort-on-container-exit --quiet-pull
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true

coverage:
	@echo "Running coverage analysis only..."
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true
	@TEST_MODE=coverage-only docker compose -f docker/docker-compose.test.yml up --build --abort-on-container-exit --quiet-pull
	@docker compose -f docker/docker-compose.test.yml down -v >/dev/null 2>&1 || true

# =============================================================================
# TODO - Development Needed
# =============================================================================

backend-shell:
	@echo "TODO: Backend container shell access needs debugging"
	@echo "Current issue: Container lacks shell utilities (ps, bash, etc.)"
	@echo "Container runs minimal Debian with just the Rust binary"
	@$(COMPOSE_CMD) exec backend sh || echo "Failed - minimal container without shell tools"
