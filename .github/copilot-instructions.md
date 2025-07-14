# Copilot Instructions for RustTracker

## Project Overview

RustTracker is a full-stack task management web application built entirely in Rust. It features:

- Backend: Axum REST API server
- Frontend: Leptos reactive web application
- Database: PostgreSQL
- Containerization: Docker and Docker Compose
- Shared models between frontend and backend

## Architecture

### Project Structure

```
rust-tracker/
├── 📋 README.md                    # Project documentation
├── ⚙️  Cargo.toml                  # Workspace configuration
├── 🔧 Makefile                     # Development shortcuts
├── 🐳 docker/                      # Docker configuration
│   ├── docker-compose.yml         # Container orchestration
│   ├── docker-compose.test.yml    # Test environment
│   └── Dockerfile.test            # Testing container
├── 🌍 .env                         # Environment variables
├── 📄 PROJECT.md                   # Detailed project overview
├── � TESTING.md                   # Testing documentation
├── 📈 TEST_COVERAGE_SUMMARY.md     # Test coverage overview

├── �📦 backend/                     # Axum REST API
│   ├── 🦀 src/
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
│   ├── 🗄️  migrations/             # Database schema
│   │   └── 001_initial.sql         # Initial database setup
│   └── 🐳 Dockerfile               # Backend container
├── 🎨 frontend/                    # Leptos WASM app
│   ├── 🦀 src/
│   │   ├── lib.rs                  # App entry point
│   │   ├── api.rs                  # HTTP client
│   │   ├── api_tests.rs            # API client tests (12 tests)
│   │   ├── component_tests.rs      # Component logic tests (15 tests)
│   │   ├── components/             # UI components
│   │   │   ├── header.rs           # Application header
│   │   │   ├── task_form.rs        # Task creation/editing form
│   │   │   ├── task_item.rs        # Individual task display
│   │   │   ├── task_list.rs        # Task list container
│   │   │   └── mod.rs              # Component exports
│   │   └── pages/                  # App pages
│   │       ├── home.rs             # Main task management page
│   │       └── mod.rs              # Page exports
│   ├── 🌐 index.html               # HTML entry point
│   ├── ⚙️  nginx.conf               # Web server config
│   └── 🐳 Dockerfile               # Frontend container
├── 📚 common/                      # Shared types
│   └── 🦀 src/
│       ├── lib.rs                  # Data models and enums
│       └── tests.rs                # Data structure tests (22 tests)
└── 🛠️  scripts/                    # Development tools
    ├── setup.sh                   # Initial setup
    ├── dev.sh                     # Development helper
    ├── test-runner.sh              # Comprehensive test runner
    └── build-quiet.sh              # Quiet build script
```

### Technology Stack

- **Language**: Rust 🦀 (Full-stack single language)
- **Backend**: Axum framework + SQLx + PostgreSQL
- **Frontend**: Leptos framework + WASM + Tailwind CSS
- **Database**: PostgreSQL with custom enum types
- **Containerization**: Docker + Docker Compose
- **Build System**: Cargo workspaces
- **Web Server**: Nginx (for frontend static files)
- **Testing**: Comprehensive test suite with 123+ tests
  - Unit tests, integration tests, performance benchmarks
  - WASM testing for frontend components
  - Database isolation with serial_test
  - GitHub Actions CI/CD pipeline
- **Development Tools**: Custom scripts and Makefile

## Development Guidelines

### Backend (Axum)

- Located in `backend/` directory
- Exposes REST API endpoints under `/api` prefix
- Connects to PostgreSQL at `db:5432` (Docker network)
- Uses SQLx for database operations
- Runs on port 8080

### Frontend (Leptos)

- Located in `frontend/` directory
- Reactive web application using Leptos framework
- Makes API calls to backend using fetch API
- Runs on port 3000
- Uses shared types from `common` crate

### Common Crate

- Located in `common/` directory
- Contains shared data models and types
- Used by both backend and frontend
- Defines Task model with enhanced fields:
  - UUID-based primary keys
  - TaskStatus enum (Todo, InProgress, Completed)
  - TaskCategory enum (Work, Personal, Shopping, Health, Other)
  - Due dates with chrono DateTime support
  - Created/updated timestamps
- Ensures type safety across the full stack

### Database

- PostgreSQL database
- Connection string: `postgres://postgres:password@db:5432/rusttracker`
- Uses SQLx for migrations and queries
- Data persisted in Docker volume

## API Endpoints

Standard REST API for task management:

- `GET /api/tasks` - List all tasks
- `POST /api/tasks` - Create new task
- `PUT /api/tasks/:id` - Update existing task
- `DELETE /api/tasks/:id` - Delete task
- `GET /health` - Health check endpoint

All endpoints use JSON format and the Task model from the `common` crate. The API includes proper error handling, CORS support, and structured logging.

## Development Workflow

### Running the Application

```bash
# Build and start all services
docker compose up --build

# Stop and remove containers with volumes
docker compose down -v

# Development shortcuts via Makefile
make setup      # Initial setup and start all services
make start      # Start all services
make stop       # Stop all services
make restart    # Restart all services
make rebuild    # Rebuild and start all services
make logs       # Show logs for all services
make clean      # Stop services and clean up
make status     # Show service status
make db         # Connect to database
make test       # Run tests
```

### Environment Configuration

- Uses `.env` file for environment variables
- Key variables:
  - `DATABASE_URL`: PostgreSQL connection string
  - `RUST_LOG`: Logging level

### Container Architecture

- **Backend**: Builds from `backend/Dockerfile`, exposes port 8080
- **Frontend**: Builds from `frontend/Dockerfile`, exposes port 3000
- **Database**: PostgreSQL container with persistent volume

### Development Scripts

- `scripts/setup.sh` - Initial project setup and environment configuration
- `scripts/dev.sh` - Development helper script with commands:
  - `start` - Start all services
  - `stop` - Stop all services
  - `restart` - Restart all services
  - `rebuild` - Rebuild and start all services
  - `logs` - Show logs for all services
  - `clean` - Stop services and clean up
  - `status` - Show service status
  - `db` - Connect to database
  - `test` - Run tests
- `scripts/test-runner.sh` - Comprehensive test execution script with database setup
- `scripts/build-quiet.sh` - Quiet build script for CI environments

## Code Style and Patterns

### General Guidelines

- Generate clear, readable, and maintainable code
- Follow language idioms and established coding conventions
- Use consistent naming for variables, functions, classes, and files
- Prefer modular, focused, and reusable code
- Respect the existing project architecture and technology conventions
- Provide clear, practical suggestions for fixing issues
- Keep responses concise, relevant, and easy to apply
- Prioritize stability and maintainability in all output

### Documentation Standards

- **Professional Formatting**: Maintain professional documentation standards
- **No Emoticons**: Avoid using emoticons, emojis, or decorative symbols in documentation
- **Clear Language**: Use precise, technical language appropriate for software documentation
- **Consistent Style**: Follow established markdown formatting conventions
- **Visual Clarity**: Use diagrams and code blocks for technical communication instead of visual decorations

### General Rust Guidelines

- Use standard Rust formatting and naming conventions
- Leverage Rust's type system and ownership model
- Handle errors appropriately with `Result` types
- Use async/await for I/O operations

### Backend Patterns

- Structure handlers using Axum extractors
- Use SQLx for database operations
- Implement proper error handling and HTTP status codes
- Use shared types from `common` crate

### Frontend Patterns

- Use Leptos components and reactive signals
- Implement proper state management
- Handle async operations with Leptos resources
- Use shared types from `common` crate for API communication

### Shared Code

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

### 🚨 CRITICAL: Test-First Development Workflow

**ALWAYS run tests and update documentation after every code change or fix!**

#### Required Actions After Every Change:

1. **Immediate Verification**:

   ```bash
   # Check compilation
   cargo check --workspace

   # PREFERRED: Run tests with proper database setup
   make test

   # Alternative: Use test runner script
   ./scripts/test-runner.sh

   # Docker-based testing (recommended for CI)
   docker compose -f docker/docker-compose.yml -f docker/Dockerfile.test up --build

   # Only for unit tests without database dependencies
   cargo test -p common
   ```

   **⚠️ Important**: Database-dependent tests (backend) require a PostgreSQL test database. Use `make test` or the test scripts to ensure proper setup.

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
   # REQUIRED: Full test suite with database setup
   make test

   # Alternative: Comprehensive test runner
   ./scripts/test-runner.sh

   # Code quality checks
   cargo clippy --workspace -- -D warnings
   cargo fmt --check

   # Docker-based verification (optional but recommended)
   docker compose up --build -d
   docker compose exec backend cargo test
   docker compose down
   ```

   **🚨 Critical**: Never commit with failing tests. Always use `make test` or `./scripts/test-runner.sh` to ensure all tests pass with proper database setup.

#### Test Coverage Maintenance:

- **Target**: Maintain 123+ tests across all layers
- **New Code**: Must include tests before being considered complete
- **Failing Tests**: Fix immediately, never commit with failing tests
- **Test Documentation**: Update README.md test coverage section when adding new test files

#### Documentation Synchronization:

- **copilot-instructions.md**: Update after architectural changes
- **README.md**: Contains all project documentation - update after any feature additions, API changes, or architectural modifications

### 📚 CRITICAL: README.md Maintenance Workflow

**The README.md file is the single source of truth for all project documentation. It MUST be updated after every change that affects the project.**

#### Mandatory README.md Updates Required For:

1. **Feature Additions**:

   - New API endpoints → Update API documentation section
   - New components → Update frontend architecture section
   - New dependencies → Update technology stack section
   - New scripts/tools → Update development workflow section

2. **Architecture Changes**:

   - Database schema modifications → Update database section
   - Service configuration changes → Update container architecture section
   - New testing patterns → Update testing documentation
   - Performance improvements → Update relevant sections

3. **Project Structure Changes**:

   - New files/directories → Update project structure tree
   - Moved/renamed files → Update all file path references
   - New crates/packages → Update workspace structure
   - Removed components → Remove from documentation

4. **Development Workflow Changes**:
   - New make targets → Update Makefile commands section
   - New scripts → Update development scripts section
   - Environment variable changes → Update configuration section
   - Docker changes → Update container documentation

#### README.md Quality Standards:

```bash
# ALWAYS run these checks after updating README.md:

# 1. Markdown linting (fix ALL warnings)
markdownlint README.md

# 2. Link validation
markdown-link-check README.md

# 3. Spelling check
aspell check README.md

# 4. Format consistency check
prettier --check README.md
```

#### Critical Sections That Require Regular Updates:

- **Project Structure Tree**: Must reflect current directory structure exactly
- **Test Coverage Numbers**: Update when test counts change (currently 123+ tests)
- **API Endpoints**: Keep synchronized with actual backend routes
- **Technology Stack**: Update when dependencies change
- **Development Commands**: Verify all commands work as documented

#### README.md Update Checklist:

Before any commit, verify:

- ✅ All new features documented
- ✅ File paths and references updated
- ✅ Test coverage numbers current
- ✅ API documentation matches implementation
- ✅ All markdown warnings resolved
- ✅ Links work correctly
- ✅ Code examples are valid
- ✅ Version numbers updated where relevant

#### Automated Checks Integration:

```bash
# Add to pre-commit workflow:
#!/bin/bash
# Pre-commit README.md validation

echo "🔍 Validating README.md..."

# Check for markdown issues
if ! markdownlint README.md; then
    echo "❌ README.md has markdown linting errors - fix before committing"
    exit 1
fi

# Verify project structure section matches reality
if ! ./scripts/verify-readme-structure.sh; then
    echo "❌ README.md project structure is outdated"
    exit 1
fi

echo "✅ README.md validation passed"
```

**Remember**: A well-maintained README.md is critical for project success. Outdated documentation leads to developer confusion and onboarding difficulties.

## Testing and Debugging

### Comprehensive Test Coverage

RustTracker includes a robust test suite with 123+ tests across all layers:

#### Backend Tests (65 tests)

- **Database Tests (23 tests)**: Connection management, CRUD operations, error handling, concurrent access
- **Handler Tests (20 tests)**: HTTP endpoints, request validation, response formatting, error cases
- **Error Tests (8 tests)**: Custom error types, HTTP status mapping, error serialization
- **Integration Tests (6 tests)**: End-to-end API workflows, complex scenarios
- **Performance Benchmarks (8 tests)**: Database operations, API response times, load testing

#### Frontend Tests (27 tests)

- **API Client Tests (12 tests)**: HTTP requests, error handling, response parsing
- **Component Tests (15 tests)**: UI logic, state management, data validation

#### Common Crate Tests (22 tests)

- **Data Structure Tests**: Serialization, validation, enum conversions, type safety

#### Testing Infrastructure

- **Docker Testing Environment**: `docker/Dockerfile.test` for isolated test execution
- **Test Runner Script**: `scripts/test-runner.sh` for comprehensive test execution
- **GitHub Actions CI/CD**: Automated testing on push/PR
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

### Logging

- Use `RUST_LOG` environment variable for log levels
- Backend logs available via Docker logs
- Frontend logs available in browser console

### Development Tips

- Use `cargo check` and `cargo clippy` for code quality
- Leverage Rust's compiler for catching errors early
- Use Docker logs for debugging container issues
- Frontend development can use browser dev tools

## Common Tasks

When working on this project, consider these common patterns and **ALWAYS follow the test-first workflow**:

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

### 🔧 Test Execution Guidelines

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
   docker compose -f docker/docker-compose.yml -f docker/Dockerfile.test up --build

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
   # ✅ DO: make test
   ```

#### ⚠️ Critical Testing Requirements:

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
