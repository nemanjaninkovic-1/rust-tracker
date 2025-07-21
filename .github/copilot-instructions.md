# Copilot Instructions for RustTracker

**No Emojis**: HARD RULE - Do not use emojis, emoticons, or decorative symbols in documentation except for checkmarks (✓) and X marks (✗) when indicating status, validation, or pass/fail conditionsST API server

## HARD RULES **Critical**:

- Only use those specific ✓ (Check mark, U+2713) and ✗ (X mark, U+2717) and no other emojies in a codbase.
- Always use `make` commands for project operations unless there is no make command available for the specific action you want to perform.

## Project Overview

RustTracker is a full-stack task management web application built entirely in Rust with:

- Backend: Axum
- Front # Backend tests REQUIRE database - use make test instead
  # ✗ DON'T: cargo test -p backend (will fail without database)
  # ✓ DO: make test: Leptos reactive web application
- Database: PostgreSQL
- Containerization: Docker and Docker Compose
- Shared models between frontend and backend

## Architecture

### Project Structure

```text
rust-tracker/
├── README.md                       # Project documentation
├── Cargo.toml                      # Workspace configuration
├── Makefile                        # Development shortcuts
├── docker/                         # Docker configuration
│   ├── docker-compose.yml          # Container orchestration
│   ├── docker-compose.test.yml     # Test environment
│   ├── Dockerfile.test             # Testing container
│   ├── Dockerfile.backend          # Backend container definition
│   └── Dockerfile.frontend         # Frontend container definition
├── .env                            # Environment variables
├── backend/                        # Axum REST API
│   ├── src/
│   │   ├── main.rs                 # Server entry point
│   │   ├── handlers.rs             # HTTP request handlers
│   │   ├── database.rs             # Database operations
│   │   ├── error.rs                # Error handling
│   │   └── tests/                  # Comprehensive test suite
│   │       ├── mod.rs              # Test module exports
│   │       ├── database_tests.rs   # Database layer tests (23 tests)
│   │       ├── handler_tests.rs    # HTTP handler tests (20 tests)
│   │       ├── error_tests.rs      # Error handling tests (8 tests)
│   │       ├── integration_tests.rs # Integration tests (6 tests)
│   │       └── benchmarks.rs       # Performance benchmarks (8 tests)
│   ├── migrations/                 # Database schema
│   │   └── 001_initial.sql         # Initial database setup
├── frontend/                       # Leptos WASM app
│   ├── src/
│   │   ├── lib.rs                  # App entry point
│   │   ├── api.rs                  # HTTP client
│   │   ├── components/             # UI components
│   │   │   ├── header.rs           # Application header
│   │   │   ├── task_form.rs        # Task creation/editing form
│   │   │   ├── task_item.rs        # Individual task display
│   │   │   ├── task_list.rs        # Task list container
│   │   │   └── mod.rs              # Component exports
│   │   ├── pages/                  # App pages
│   │   │   ├── home.rs             # Main task management page
│   │   │   └── mod.rs              # Page exports
│   │   └── tests/                  # Test modules
│   │       ├── api_tests.rs        # API client tests (12 tests)
│   │       ├── component_tests.rs  # Component logic tests (15 tests)
│   │       └── mod.rs              # Test exports
│   ├── index.html                  # HTML entry point
│   ├── nginx.conf                  # Web server config
├── common/                         # Shared types
│   └── src/
│       ├── lib.rs                  # Data models and enums
│       └── tests/                  # Data structure tests (19 tests)
│           ├── data_structures.rs
│           └── mod.rs
└── scripts/                        # Development tools
    ├── build-frontend.sh           # Frontend build script
    ├── dev-server.sh               # Frontend development server  
    ├── quick-test.sh               # Quick test script
    └── test-suite.sh               # Comprehensive test runner
```

### Technology Stack

- **Language**: Rust (Full-stack single language)
- **Backend**: Axum framework + SQLx + PostgreSQL
- **Frontend**: Leptos framework + WASM + Tailwind CSS
- **Database**: PostgreSQL with custom enum types
- **Containerization**: Docker + Docker Compose
- **Build System**: Cargo workspaces
- **Web Server**: Nginx (for frontend static files)
- **Testing**: Comprehensive test suite with 120+ tests
- **Development Tools**: Custom scripts and Makefile

## Development Guidelines

### CRITICAL: Make Command Priority Rule

**HARD RULE: Always use make commands for project operations unless there is no make command available for the specific action you want to perform.**

Available make commands (use these instead of manual docker/cargo commands):

- `make setup` - Initial setup and start all services
- `make start` - Start all services
- `make stop` - Stop all services
- `make restart` - Restart all services
- `make rebuild` - Rebuild and start all services
- `make test` - Run comprehensive test suite
- `make logs` - Show logs for all services
- `make clean` - Stop services and clean up
- `make db` - Connect to database

**Examples**:

- ✓ Use: `make restart`
- ✗ Don't use: `docker compose restart`
- ✓ Use: `make test`
- ✗ Don't use: `cargo test` or manual docker commands
- ✓ Use: `make rebuild`
- ✗ Don't use: `docker compose down && docker compose up --build`

Only use manual docker/cargo commands when the required functionality is not available through make commands.

### Component Architecture

- **Backend**: Located in `backend/`, exposes REST API under `/api` prefix, connects to PostgreSQL at `db:5432`, runs on port 8080
- **Frontend**: Located in `frontend/`, reactive Leptos application, makes API calls to backend, runs on port 3000
- **Common**: Located in `common/`, contains shared data models and types used by both frontend and backend

### Data Models

Task model with enhanced fields:

- UUID-based primary keys
- TaskStatus enum (Todo, InProgress, Completed)
- TaskPriority enum (Low, Medium, High, Urgent) - supports Kanban board workflow
- Due dates with chrono DateTime support
- Created/updated timestamps

### Database

- PostgreSQL database with SQLx for migrations and queries
- Connection: `postgres://postgres:password@db:5432/rusttracker`
- Data persisted in Docker volume

## API Endpoints

REST API for task management:

- `GET /api/tasks` - List all tasks (supports priority filtering)
- `POST /api/tasks` - Create new task
- `PUT /api/tasks/:id` - Update existing task
- `DELETE /api/tasks/:id` - Delete task
- `GET /health` - Health check endpoint

All endpoints use JSON format with proper error handling, CORS support, and structured logging.

## Development Workflow

### Quick Commands

**ALWAYS use these make commands instead of manual docker/cargo commands:**

```bash
# Essential commands (USE THESE)
make setup      # Initial setup and start all services
make test       # Run comprehensive test suite
make start      # Start all services
make stop       # Stop all services
make restart    # Restart all services
make rebuild    # Rebuild and start all services
make logs       # Show logs for all services
make clean      # Stop services and clean up
make db         # Connect to database

# Manual commands (ONLY when no make command exists)
docker compose up --build          # Build and start all services
docker compose down -v             # Stop and remove containers with volumes
```

### Environment Configuration

Key environment variables in `.env`:

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level

### Container Architecture

- **Backend**: Builds from `docker/Dockerfile.backend`, exposes port 8080
- **Frontend**: Builds from `docker/Dockerfile.frontend`, exposes port 3000
- **Database**: PostgreSQL container with persistent volume

### Docker Organization

**All Docker-related files are centralized in the `/docker/` folder for consistency and maintainability:**

- `docker/Dockerfile.backend` - Backend container definition
- `docker/Dockerfile.frontend` - Frontend container definition
- `docker/Dockerfile.test` - Test environment container
- `docker/docker-compose.yml` - Main orchestration
- `docker/docker-compose.test.yml` - Test orchestration

**Benefits of this organization:**

- **Consistency**: All Docker configs in one location
- **Maintainability**: Easy to find and manage containerization files
- **Best Practices**: Separates application code from infrastructure code
- **Team Collaboration**: Clear structure for DevOps and development workflows

## Code Style and Patterns

### General Guidelines

- Generate clear, readable, and maintainable code
- Follow language idioms and established coding conventions
- Use consistent naming for variables, functions, classes, and files
- Prefer modular, focused, and reusable code
- Respect existing project architecture and technology conventions
- Provide clear, practical suggestions for fixing issues
- Keep responses concise, relevant, and easy to apply
- Prioritize stability and maintainability in all output

### Documentation Standards

- **Professional Formatting**: Maintain professional documentation standards
- **No Emojis**: HARD RULE - Do not use emojis, emoticons, or decorative symbols in documentation except for checkmarks (✓) and X marks (❌) when indicating status, validation, or pass/fail conditions
- **Clear Language**: Use precise, technical language appropriate for software documentation
- **Consistent Style**: Follow established markdown formatting conventions
- **Visual Clarity**: Use diagrams and code blocks for technical communication

### Rust Development Patterns

- Use standard Rust formatting and naming conventions
- Leverage Rust's type system and ownership model
- Handle errors appropriately with `Result` types
- Use async/await for I/O operations
- Structure handlers using Axum extractors
- Use SQLx for database operations
- Implement proper error handling and HTTP status codes
- Use Leptos components and reactive signals
- Implement proper state management
- Handle async operations with Leptos resources
- Define common data structures in `common` crate
- Use serde for JSON serialization/deserialization
- Ensure types are compatible between frontend and backend

### Test Development Guidelines

- **Backend Tests**: Use `#[tokio::test]` for async tests, `#[serial]` for database tests
- **Frontend Tests**: Use `#[wasm_bindgen_test]` for WASM component tests
- **Database Tests**: Always use `serial_test::serial` to prevent concurrent access issues
- **Integration Tests**: Test complete workflows, not just individual components
- **Performance Tests**: Include benchmarks for critical operations
- **Error Testing**: Verify all error paths and edge cases
- **Mock Data**: Use consistent test data factories for repeatability

### CRITICAL: Test-First Development Workflow

**ALWAYS run tests and update documentation after every code change or fix!**

#### Required Actions After Every Change:

1. **Immediate Verification**:

   ```bash
   cargo check --workspace          # Check compilation
   make test                        # PREFERRED: Run tests with database setup
   ./scripts/test-runner.sh         # Alternative: Use test runner script
   ```

2. **Test Updates Required When**:

   - Adding new functions/methods → Add corresponding unit tests
   - Modifying API endpoints → Update handler and integration tests
   - Changing data models → Update serialization and validation tests
   - Adding error cases → Add error handling tests
   - Performance improvements → Add/update benchmark tests

3. **Documentation Updates Required When**:

   - Adding new features → Update README.md
   - Changing API → Update README.md API documentation section
   - Adding test files → Update README.md test coverage section
   - Modifying architecture → Update README.md and copilot-instructions.md

4. **Before Committing**:
   ```bash
   make test                        # REQUIRED: Full test suite with database setup
   ./scripts/test-runner.sh         # Alternative: Comprehensive test runner
   cargo clippy --workspace -- -D warnings    # Code quality checks
   cargo fmt --check               # Format checks
   ```

**Never commit with failing tests. Always use `make test` or `./scripts/test-runner.sh` to ensure all tests pass with proper database setup.**

#### Test Coverage Maintenance:

- **Target**: Maintain 120+ tests across all layers
- **New Code**: Must include tests before being considered complete
- **Failing Tests**: Fix immediately, never commit with failing tests
- **Test Documentation**: Update README.md test coverage section when adding new test files

### README.md Maintenance Workflow

**The README.md file is the single source of truth for all project documentation and MUST be updated after every change that affects the project.**

#### Mandatory README.md Updates Required For:

1. **Feature Additions**: New API endpoints, components, dependencies, scripts/tools
2. **Architecture Changes**: Database schema modifications, service configuration changes, new testing patterns
3. **Project Structure Changes**: New files/directories, moved/renamed files, new crates/packages
4. **Development Workflow Changes**: New make targets, scripts, environment variables, Docker changes

#### README.md Update Checklist:

Before any commit, verify:

- ✓ All new features documented
- ✓ File paths and references updated
- ✓ Test coverage numbers current
- ✓ API documentation matches implementation
- ✓ All markdown warnings resolved
- ✓ Links work correctly
- ✓ Code examples are valid
- ✓ Version numbers updated where relevant

## Testing and Debugging

### Comprehensive Test Coverage

RustTracker includes a robust test suite with 120+ tests across all layers:

#### Backend Tests (65 tests)

- **Database Tests (23 tests)**: Connection management, CRUD operations, error handling, concurrent access
- **Handler Tests (20 tests)**: HTTP endpoints, request validation, response formatting, error cases
- **Error Tests (8 tests)**: Custom error types, HTTP status mapping, error serialization
- **Integration Tests (6 tests)**: End-to-end API workflows, complex scenarios
- **Performance Benchmarks (8 tests)**: Database operations, API response times, load testing

#### Frontend Tests (27 tests)

- **API Client Tests (12 tests)**: HTTP requests, error handling, response parsing
- **Component Tests (15 tests)**: UI logic, state management, data validation

#### Common Crate Tests (19 tests)

- **Data Structure Tests**: Serialization, validation, enum conversions, type safety

#### Testing Infrastructure

- **Docker Testing Environment**: `docker/Dockerfile.test` for isolated test execution
- **Test Runner Script**: `scripts/test-runner.sh` for comprehensive test execution
- **Database Isolation**: Uses `serial_test` for safe concurrent testing
- **WASM Testing**: `wasm-bindgen-test` for frontend component testing

### Test Execution

```bash
# Run all tests
make test

# Run backend tests only
cargo test -p backend

# Run frontend tests (WASM)
cd frontend && wasm-pack test --node

# Run performance benchmarks
cargo test benchmarks --release

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --exclude-files "*/tests/*"
```

### Logging and Debugging

- Use `RUST_LOG` environment variable for log levels
- Backend logs available via Docker logs
- Frontend logs available in browser console
- Use `cargo check` and `cargo clippy` for code quality
- Leverage Rust's compiler for catching errors early
- Use Docker logs for debugging container issues
- Frontend development can use browser dev tools

## Common Tasks

When working on this project, **ALWAYS follow the test-first workflow**:

1. **Adding new API endpoints**:

   - Update backend handlers and ensure frontend can consume them
   - **REQUIRED**: Add handler tests and integration tests
   - **REQUIRED**: Update API documentation

2. **Modifying data models**:

   - Update `common` crate and propagate changes to both frontend and backend
   - **REQUIRED**: Add/update serialization tests and validation tests
   - **REQUIRED**: Update affected handler and database tests

3. **Database changes**:

   - Update SQLx migrations and corresponding Rust structs
   - **REQUIRED**: Add database operation tests
   - **REQUIRED**: Update integration tests

4. **Frontend components**:

   - Create reusable Leptos components following reactive patterns
   - **REQUIRED**: Add component logic tests with `wasm-bindgen-test`
   - **REQUIRED**: Update API client tests if needed

5. **Error handling**:

   - Implement consistent error handling across the stack
   - **REQUIRED**: Add error case tests and validation
   - **REQUIRED**: Update integration tests for error scenarios

6. **Writing tests**: Follow the comprehensive testing patterns established in the project:
   - Database tests with `serial_test` for isolation
   - Handler tests with `axum-test` for HTTP testing
   - Component tests with `wasm-bindgen-test` for frontend logic
   - Integration tests for end-to-end workflows
   - Performance benchmarks for critical operations

### Test Execution Guidelines

**ALWAYS use Makefile or Docker for comprehensive testing to ensure proper database setup:**

#### Recommended Test Commands (in order of preference):

1. **Primary Method - Makefile**:

   ```bash
   make test              # Run all tests with database setup
   make setup && make test # Fresh setup + comprehensive tests
   ```

2. **Alternative - Test Runner Script**:

   ```bash
   ./scripts/test-runner.sh  # Comprehensive test execution with database
   ```

3. **Docker-based Testing**:

   ```bash
   # Full containerized testing (recommended for CI)
   docker compose -f docker/docker-compose.test.yml up --build

   # Test in running environment
   docker compose up -d
   docker compose exec backend cargo test
   docker compose down
   ```

4. **Individual Package Testing** (only for isolated tests):

   ```bash
   cargo test -p common     # Common crate (no database needed)
   cargo test -p frontend   # Frontend tests (requires wasm-pack)

   # Backend tests REQUIRE database - use make test instead
   # ❌ DON'T: cargo test -p backend  (will fail without database)
   # ✓ DO: make test
   ```

#### Critical Testing Requirements:

- **Database Tests**: Backend tests require PostgreSQL test database
- **Serial Execution**: Database tests use `#[serial]` to prevent conflicts
- **Environment Setup**: Test scripts handle database migrations and setup
- **WASM Tests**: Frontend tests require `wasm-bindgen-test` environment
- **Integration Tests**: Need full application stack running

**Never run backend tests with bare `cargo test -p backend` - they will fail without database!**

**Remember**: Every code change must include corresponding tests and documentation updates. No exceptions!

## Dependencies and Crates

Key dependencies to be aware of:

- `axum` - Web framework for backend
- `leptos` - Frontend framework
- `sqlx` - Database toolkit
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `postgres` - PostgreSQL driver

### Testing Dependencies

- `axum-test` - HTTP testing for Axum applications
- `serial_test` - Database test isolation
- `wasm-bindgen-test` - WASM testing for frontend
- `tokio-test` - Async testing utilities
- `mockall` - Mocking framework
- `pretty_assertions` - Enhanced assertion output

This project uses Cargo workspaces to manage multiple crates efficiently.
