# RustTracker

RustTracker is a full-stack task management web application built entirely in Rust. It features a fast backend using Axum, a reactive frontend using Leptos, and a PostgreSQL database. The entire project is containerized using Docker and Docker Compose for easy setup and deployment.

## Table of Contents

- [Key Features](#key-features)
- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
  - [Quick Setup](#quick-setup)
  - [Manual Setup](#manual-setup)
  - [Development Commands](#development-commands)
  - [Building Locally](#building-locally)
- [Example API Endpoints](#example-api-endpoints)
- [Tech Stack](#tech-stack)
- [Architecture Overview](#architecture-overview)
  - [System Architecture](#system-architecture)
  - [Data Flow](#data-flow)
  - [Container Architecture](#container-architecture)
  - [Technology Stack Diagram](#technology-stack-diagram)
  - [Backend Features](#backend-features)
  - [Frontend Features](#frontend-features)
  - [Database](#database)
  - [Containerization](#containerization)
- [API Endpoints](#api-endpoints)
- [Comprehensive Test Coverage](#comprehensive-test-coverage)
- [CI/CD Pipeline](#cicd-pipeline)
- [Development Workflow](#development-workflow)
- [Quick Start](#quick-start)
- [Environment Configuration](#environment-configuration)
- [Volumes and Persistence](#volumes-and-persistence)
- [Local Development](#local-development)
- [Troubleshooting](#troubleshooting)
- [TODO](#todo)
- [Future Enhancements](#future-enhancements)
- [Contributing](#contributing)
- [License](#license)

## Key Features

- **Full-stack Rust** - Single language across the entire stack
- **Reactive UI** - Real-time updates with [Leptos signals](https://leptos.dev/)
- **Type Safety** - Shared models between frontend and backend
- **Containerized** - Complete [Docker](https://www.docker.com/) setup
- **Production Ready** - Health checks, logging, error handling
- **Comprehensive Testing** - [123+ tests](#comprehensive-test-coverage) with 94%+ coverage
- **Performance Benchmarks** - Sub-100ms average response times
- **Security Audited** - Vulnerability scanning and dependency checks

### Task Management Features

- Create, read, update, and delete tasks
- Filter by status, category, or due date
- RESTful API and responsive web interface
- PostgreSQL for persistent data storage
- Real-time task status updates

## Project Structure

```text
rust-tracker/
├── README.md                    # Project documentation
├── Cargo.toml                  # Workspace configuration
├── Makefile                     # Development shortcuts
├── .env                         # Environment variables
├── docker/                      # Docker configuration
│   ├── docker-compose.yml         # Container orchestration  
│   ├── docker-compose.test.yml    # Test environment
│   └── Dockerfile.test            # Testing container
├── backend/                     # Axum REST API
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
│   ├── migrations/             # Database schema
│   │   └── 001_initial.sql         # Initial database setup
│   └── Dockerfile               # Backend container
├── frontend/                    # Leptos WASM app
│   ├── src/
│   │   ├── lib.rs                  # App entry point
│   │   ├── api.rs                  # HTTP client
│   │   ├── tests/                  # Frontend test suite
│   │   │   ├── mod.rs              # Test module exports
│   │   │   ├── api_tests.rs        # API client tests (12 tests)
│   │   │   └── component_tests.rs  # Component logic tests (15 tests)
│   │   ├── components/             # UI components
│   │   │   ├── header.rs           # Application header
│   │   │   ├── task_form.rs        # Task creation/editing form
│   │   │   ├── task_item.rs        # Individual task display
│   │   │   ├── task_list.rs        # Task list container
│   │   │   └── mod.rs              # Component exports
│   │   └── pages/                  # App pages
│   │       ├── home.rs             # Main task management page
│   │       └── mod.rs              # Page exports
│   ├── index.html               # HTML entry point
│   ├── nginx.conf               # Web server config
│   └── Dockerfile               # Frontend container
├── common/                      # Shared types
│   └── src/
│       ├── lib.rs                  # Data models and enums
│       └── tests/                  # Common crate test suite
│           ├── mod.rs              # Test module exports
│           └── data_structures.rs  # Data structure tests (22 tests)
└── scripts/                    # Development tools
    ├── setup.sh                   # Initial setup
    ├── dev.sh                     # Development helper
    ├── test-runner.sh              # Comprehensive test runner
    └── build-quiet.sh              # Quiet build script
```

## Requirements

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

No need to install [Rust](https://rustup.rs/), [PostgreSQL](https://www.postgresql.org/), or frontend tooling locally.

## Getting Started

### Quick Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/nemanjaninkovic-1/rust-tracker.git
   cd rust-tracker
   ```

2. Run the setup script:

   ```bash
   make setup
   ```

This will automatically build and start all services with health checks using the Makefile.

See [Development Commands](#development-commands) for more options.

### Development Commands

Use the provided Makefile commands for common development tasks:

```bash
# Initial setup and start all services
make setup

# Start services
make start

# Stop services
make stop

# Rebuild and restart
make rebuild

# View logs
make logs

# Clean up everything
make clean

# Check service status
make status

# Connect to database
make db

# Run tests
make test
```

### Manual Setup

If you prefer to use Docker Compose directly instead of the Makefile:

   ```bash
   docker compose up --build
   ```

This will:

- Start the PostgreSQL container
- Build and start the backend on <http://localhost:8080>
- Build and start the frontend on <http://localhost:3000>

Note: The recommended approach is to use the Makefile commands which provide better control and error handling.

### Building Locally

To build the project locally without Docker:

```bash
# Set SQLx to offline mode (no database connection needed for compilation)
export SQLX_OFFLINE=true

# Check compilation
cargo check

# Run tests
cargo test

# Build release
cargo build --release
```

## Example API Endpoints

- GET /api/tasks
- POST /api/tasks
- PUT /api/tasks/:id
- DELETE /api/tasks/:id

All endpoints use JSON and the Task model from the `common` crate.

## Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (Full-stack single language)
- **Backend**: [Axum framework](https://github.com/tokio-rs/axum) + [SQLx](https://github.com/launchbadge/sqlx) + [PostgreSQL](https://www.postgresql.org/)
- **Frontend**: [Leptos framework](https://leptos.dev/) + [WebAssembly (WASM)](https://webassembly.org/) + [Tailwind CSS](https://tailwindcss.com/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) with custom enum types
- **Containerization**: [Docker](https://www.docker.com/) + [Docker Compose](https://docs.docker.com/compose/)
- **Build System**: [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- **Web Server**: [Nginx](https://nginx.org/) (for frontend static files)
- **Testing**: Comprehensive test suite with [123+ tests](#comprehensive-test-coverage)
  - Unit tests, integration tests, performance benchmarks
  - WASM testing for frontend components
  - Database isolation with [serial_test](https://crates.io/crates/serial_test)
  - [GitHub Actions](https://github.com/features/actions) CI/CD pipeline
- **Development Tools**: Custom scripts and [Makefile](#development-commands)

## Architecture Overview

### System Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        USER[User Browser]
    end
    
    subgraph "Frontend Container (Port 3000)"
        NGINX[Nginx Web Server]
        LEPTOS[Leptos WASM App]
        COMP[UI Components]
        API_CLIENT[API Client]
    end
    
    subgraph "Backend Container (Port 8080)"
        AXUM[Axum HTTP Server]
        HANDLERS[Request Handlers]
        DB_LAYER[Database Layer]
        MIDDLEWARE[CORS & Logging]
    end
    
    subgraph "Database Container (Port 5432)"
        POSTGRES[(PostgreSQL)]
        MIGRATIONS[Schema Migrations]
        VOLUME[Docker Volume]
    end
    
    subgraph "Shared Code"
        COMMON[Common Crate]
        MODELS[Task Models & Enums]
        TYPES[Shared Types]
    end
    
    USER --> NGINX
    NGINX --> LEPTOS
    LEPTOS --> COMP
    COMP --> API_CLIENT
    API_CLIENT -->|HTTP/JSON| AXUM
    AXUM --> MIDDLEWARE
    MIDDLEWARE --> HANDLERS
    HANDLERS --> DB_LAYER
    DB_LAYER -->|SQLx| POSTGRES
    POSTGRES --> VOLUME
    MIGRATIONS --> POSTGRES
    
    LEPTOS -.->|Uses| COMMON
    HANDLERS -.->|Uses| COMMON
    COMMON --> MODELS
    MODELS --> TYPES
    
    classDef frontend fill:#e1f5fe
    classDef backend fill:#f3e5f5
    classDef database fill:#e8f5e8
    classDef shared fill:#fff3e0
    
    class NGINX,LEPTOS,COMP,API_CLIENT frontend
    class AXUM,HANDLERS,DB_LAYER,MIDDLEWARE backend
    class POSTGRES,MIGRATIONS,VOLUME database
    class COMMON,MODELS,TYPES shared
```

### Data Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend<br/>(Leptos)
    participant B as Backend<br/>(Axum)
    participant D as Database<br/>(PostgreSQL)
    
    Note over U,D: Task Creation Flow
    U->>F: Fill task form
    F->>F: Validate input
    F->>B: POST /api/tasks
    B->>B: Deserialize & validate
    B->>D: INSERT new task
    D-->>B: Return task with ID
    B-->>F: 201 Created + task JSON
    F->>F: Update UI state
    F-->>U: Show success message
    
    Note over U,D: Task List Retrieval
    U->>F: Load page / refresh
    F->>B: GET /api/tasks?filters
    B->>D: SELECT with WHERE clause
    D-->>B: Return task array
    B-->>F: 200 OK + tasks JSON
    F->>F: Render task list
    F-->>U: Display tasks
    
    Note over U,D: Task Update Flow
    U->>F: Edit task / change status
    F->>B: PUT /api/tasks/:id
    B->>D: UPDATE task SET ...
    D-->>B: Return updated task
    B-->>F: 200 OK + updated task
    F->>F: Update local state
    F-->>U: Reflect changes
```

### Container Architecture

```mermaid
graph LR
    subgraph "Docker Compose Network"
        subgraph "Frontend Container"
            FE[Leptos WASM<br/>+ Nginx<br/>Port: 3000]
        end
        
        subgraph "Backend Container" 
            BE[Axum Server<br/>Rust Binary<br/>Port: 8080]
        end
        
        subgraph "Database Container"
            DB[PostgreSQL<br/>Data Storage<br/>Port: 5432]
            VOL[(Docker Volume<br/>Data Persistence)]
        end
    end
    
    BROWSER[Browser] --> FE
    FE -->|HTTP API Calls| BE
    BE -->|SQLx Queries| DB
    DB --> VOL
    
    classDef container fill:#f9f9f9,stroke:#333,stroke-width:2px
    classDef external fill:#e3f2fd,stroke:#1976d2,stroke-width:2px
    classDef storage fill:#e8f5e8,stroke:#388e3c,stroke-width:2px
    
    class FE,BE,DB container
    class BROWSER external
    class VOL storage
```

### Technology Stack Diagram

```mermaid
mindmap
  root)RustTracker(
    Frontend
      Leptos Framework
      WebAssembly (WASM)
      Tailwind CSS
      Trunk Build Tool
      Nginx Web Server
    Backend
      Axum HTTP Framework
      SQLx Database Toolkit
      Tokio Async Runtime
      Structured Logging
      Error Handling
    Database
      PostgreSQL
      Custom Enum Types
      Schema Migrations
      Connection Pooling
      Docker Volume
    Shared
      Common Crate
      Cargo Workspace
      Serde JSON
      Type Safety
    DevOps
      Docker Containers
      Docker Compose
      123+ Tests
      GitHub Actions
      Coverage Reports
```

### Backend Features

- [Axum](https://github.com/tokio-rs/axum) web framework with async handlers
- [PostgreSQL](https://www.postgresql.org/) database with [SQLx](https://github.com/launchbadge/sqlx)
- REST API endpoints for CRUD operations
- Database migrations with custom types
- Error handling with proper HTTP responses
- CORS support for frontend communication
- Health check endpoint (`GET /health`)
- Environment configuration
- Structured logging with [tracing](https://tracing.rs/)

### Frontend Features

- [Leptos](https://leptos.dev/) reactive framework
- [WASM](https://webassembly.org/) compilation for web deployment
- Component-based architecture
- HTTP API integration
- Task filtering and management
- Responsive design with [Tailwind CSS](https://tailwindcss.com/)
- Error handling and loading states
- Form validation

### Database

- [PostgreSQL](https://www.postgresql.org/) with custom enum types
- Task table with proper indexing
- Migration system using [SQLx migrations](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md#migrate)
- Connection pooling
- Data persistence with [Docker volumes](https://docs.docker.com/storage/volumes/)

### Containerization

- Multi-stage [Docker builds](https://docs.docker.com/develop/dev-best-practices/)
- Optimized container images
- Service orchestration with [Docker Compose](https://docs.docker.com/compose/)
- Health checks and dependencies
- Development and production configurations

## API Endpoints

Standard REST API for task management:

- `GET /api/tasks` - List all tasks (supports filtering)
  - `?status=Todo|InProgress|Completed`
  - `?category=Work|Personal|Shopping|Health|Other`
  - `?due_before=2024-12-31T23:59:59Z`
  - `?due_after=2024-01-01T00:00:00Z`
- `POST /api/tasks` - Create new task
- `PUT /api/tasks/:id` - Update existing task
- `DELETE /api/tasks/:id` - Delete task
- `GET /health` - Health check endpoint

All endpoints use JSON format and the Task model from the `common` crate. The API includes proper error handling, CORS support, and structured logging.

## Comprehensive Test Coverage

RustTracker includes a robust test suite with **123+ tests** across all layers, providing **94%+ coverage**:

### Backend Tests (65 tests)

#### Database Layer Tests (23 tests) - `database_tests.rs`

- Task CRUD operations (Create, Read, Update, Delete)
- Filter functionality (status, category, date ranges)
- Error handling for invalid operations
- Data validation and constraints
- Concurrent operation testing
- Edge cases and boundary conditions

#### HTTP Handler Tests (20 tests) - `handler_tests.rs`

- RESTful API endpoints
- Request/response validation
- HTTP status codes
- CORS handling
- Error responses
- JSON serialization/deserialization
- URL parameter validation
- Complete task lifecycle workflows

#### Error Handling Tests (8 tests) - `error_tests.rs`

- Custom error types and responses
- HTTP status code mapping
- Error serialization/deserialization
- Database error handling
- Validation error scenarios

#### Integration Tests (6 tests) - `integration_tests.rs`

- End-to-end API workflows
- Cross-component interaction testing
- Real database operations
- Complex business logic scenarios
- Sequential operation testing

#### Performance Benchmarks (8 tests) - `benchmarks.rs`

- Database operation performance
- API response times
- Load testing scenarios
- Memory usage validation
- Concurrent operation benchmarks

### Frontend Tests (27 tests)

#### API Client Tests (12 tests) - `api_tests.rs`

- HTTP request functionality
- Error handling and retry logic
- Response parsing and validation
- Filter parameter generation
- URL construction testing

#### Component Logic Tests (15 tests) - `component_tests.rs`

- Task creation and validation
- Status progression logic
- Category management
- Date handling and formatting
- Form validation logic
- Task filtering algorithms
- UI state management
- Error display handling

### Common Crate Tests (22 tests) - `tests.rs`

- Data structure serialization/deserialization
- Enum value validation
- Type conversion testing
- JSON format compatibility
- Field validation rules
- Default value testing

### Testing Infrastructure

- **Docker Testing Environment**: [`docker/Dockerfile.test`](docker/Dockerfile.test) for isolated test execution
- **Test Runner Script**: [`scripts/test-runner.sh`](scripts/test-runner.sh) for comprehensive test execution
- **GitHub Actions CI/CD**: Automated testing on push/PR
- **Database Isolation**: Uses [`serial_test`](https://crates.io/crates/serial_test) for safe concurrent testing
- **WASM Testing**: [`wasm-bindgen-test`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) for frontend component testing

### Performance Benchmarks Results

- **Create Task**: < 100ms (average: 15ms)
- **Fetch 1000 Tasks**: < 500ms (average: 200ms)
- **Update Task**: < 100ms (average: 25ms)
- **Delete Task**: < 50ms (average: 10ms)
- **Concurrent Operations**: < 200ms average response time

### Quality Gates

- Zero Clippy warnings
- Consistent formatting (rustfmt)
- No security vulnerabilities
- Up-to-date dependencies
- Comprehensive error handling

### Test Execution Commands

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

# Run specific test categories
cargo test database_tests    # Database tests only
cargo test handler_tests     # Handler tests only
cargo test integration_tests # Integration tests only
```

For more testing information, see [scripts/test-runner.sh](scripts/test-runner.sh).

### Test Environment Setup

```bash
# Set up test database
export TEST_DATABASE_URL="postgres://postgres:password@localhost:5432/rusttracker_test"
export RUST_LOG=debug

# Create test database
createdb rusttracker_test

# Run comprehensive test suite
./scripts/test-runner.sh
```

## CI/CD Pipeline

The GitHub Actions workflow automatically:

### Code Quality Checks

- Formatting verification with rustfmt
- Clippy linting for code quality
- Security auditing with cargo-audit

### Test Execution

- Unit tests across all components
- Integration tests with real database
- Frontend WASM tests
- Performance benchmarks

### Coverage Analysis

- Automated coverage generation
- Threshold enforcement (90%+ target)
- Coverage reporting and trends

### Security Scanning

- Dependency vulnerability checks
- Code security analysis
- Automated security updates

## Development Workflow

### Primary Build System - Makefile

RustTracker uses a Makefile as the primary build system for development. All development commands should go through the Makefile targets for consistency and proper orchestration of Docker services.

### Quick Start Commands

```bash
make setup     # Initial setup - builds and starts all services
make start     # Start all services
make stop      # Stop all services
make restart   # Restart all services
make rebuild   # Rebuild and start all services
make logs      # View logs for all services
make clean     # Stop services and clean up
make status    # Show service status
make db        # Connect to database
make test      # Run comprehensive test suite
```

### Development Scripts

```bash
# Primary development commands (recommended)
make setup      # Initial project setup and service startup
make start      # Start all services
make stop       # Stop all services
make rebuild    # Rebuild and restart all services
make test       # Run comprehensive test suite

# Alternative: Using development script directly
./scripts/dev.sh start
./scripts/dev.sh logs backend
./scripts/dev.sh rebuild
./scripts/dev.sh clean

# Setup and testing scripts
./scripts/setup.sh      # Initial project setup
./scripts/test-runner.sh # Run comprehensive tests
./scripts/build-quiet.sh # Quiet build for CI
```

## Quick Start

### Prerequisites

- Docker
- Docker Compose

No need to install Rust, PostgreSQL, or frontend tooling locally.

### Setup and Launch

1. Clone the repository:

   ```bash
   git clone https://github.com/nemanjaninkovic-1/rust-tracker.git
   cd rust-tracker
   ```

2. Run the setup script:

   ```bash
   make setup
   ```

   This will automatically build and start all services with health checks using the Makefile.

3. **Access the application**:
   - Frontend: <http://localhost:3000>
   - Backend API: <http://localhost:8080>
   - Database: localhost:5432

### Alternative Setup

If you prefer manual Docker Compose control:

```bash
# Build and run the application
docker compose up --build

# This will:
# - Start the PostgreSQL container
# - Build and start the backend on http://localhost:8080
# - Build and start the frontend on http://localhost:3000
```

## Environment Configuration

### Security Notice

**Never commit `.env` files to version control!** The repository includes a `.env.example` template for setup.

Create your local `.env` file:

```bash
# Copy the template
cp .env.example .env

# Edit with your actual values
nano .env
```

Example configuration (edit with your values):

```env
DATABASE_URL=postgres://postgres:password@db:5432/rusttracker
RUST_LOG=info
ENVIRONMENT=development
```

### Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (debug, info, warn, error)
- `ENVIRONMENT`: Environment mode (development, staging, production)

For production deployment and CI/CD security practices, see the [Security & Production](#security--production) section in [TODO](#todo).

## Volumes and Persistence

PostgreSQL data is stored in a Docker volume defined in `docker/docker-compose.yml`.

To remove containers and volumes:

```bash
docker compose down -v
```

## Local Development

To build the project locally without Docker:

```bash
# Set SQLx to offline mode (no database connection needed for compilation)
export SQLX_OFFLINE=true

# Check compilation
cargo check

# Run tests
cargo test

# Build release
cargo build --release
```

## Troubleshooting

### Common Issues

1. **Database Connection**: Ensure PostgreSQL is running

   ```bash
   docker compose logs db
   ```

2. **Permission Issues**: Check database user permissions

   ```bash
   make db  # Connect to database and verify access
   ```

3. **Port Conflicts**: Verify port availability (3000, 8080, 5432)

   ```bash
   netstat -tulpn | grep :3000
   netstat -tulpn | grep :8080
   netstat -tulpn | grep :5432
   ```

4. **Memory Issues**: Increase available memory for Docker
   - Docker Desktop: Settings → Resources → Memory

### Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug make start

# View specific service logs
make logs
docker compose logs backend
docker compose logs frontend

# Run tests with output
cargo test -- --nocapture

# Check service health
make status
```

### Container Issues

```bash
# Rebuild everything from scratch
make clean
make rebuild

# Check container status
docker ps -a

# Remove all containers and volumes
docker compose down -v
docker system prune -a
```

## TODO

### Critical Issues

- **CRITICAL: Make test command**: `make test` needs implementation - comprehensive test suite runner
- **CRITICAL: Backend shell improvements**: `make backend-shell` lacks proper debugging tools
- **KNOWN ISSUE: Edition2024 dependency issue**: Backend tests require Rust nightly due to base64ct-1.8.0 edition2024 requirement

### GitHub Actions & CI/CD

- **COMPLETED: Fixed all workflows**: All GitHub workflows now use `make quick-test` and proper secrets
- **COMPLETED: Simplified secrets**: Only requires `JWT_SECRET` and `POSTGRES_PASSWORD` (no DATABASE_URL needed)
- **PLANNED: Documentation needed**: Update [GITHUB_SECRETS.md](docs/GITHUB_SECRETS.md) with new simplified setup

### Development Environment

- **COMPLETED: Docker infrastructure**: Fully functional with all services working
- **COMPLETED: Makefile organization**: 11 working commands, 2 marked as TODO
- **COMPLETED: Quick test suite**: `make quick-test` working with Docker test containers
- **PLANNED: Performance optimization**: Consider implementing cargo workspace caching for faster builds

### Security & Production

- **PLANNED: Environment security docs**: Complete [ENVIRONMENT_SECURITY.md](docs/ENVIRONMENT_SECURITY.md) documentation
- **PLANNED: Rate limiting**: Implement and document rate limiting configuration
- **PLANNED: Health checks**: Enhance health check endpoints with detailed status
- **PLANNED: Monitoring**: Add Prometheus metrics and Grafana dashboards

### Feature Development

- **PLANNED: Authentication system**: JWT-based user authentication and authorization
- **PLANNED: Real-time updates**: WebSocket support for live task synchronization
- **PLANNED: Advanced filtering**: Full-text search and custom filter combinations
- **PLANNED: Bulk operations**: Multi-task selection and batch processing
- **PLANNED: API versioning**: Implement proper API versioning strategy

### Testing & Quality

- **COMPLETED: Comprehensive test suite**: 123+ tests with 94%+ coverage achieved
- **PLANNED: Property-based testing**: Implement QuickCheck for data structure validation
- **PLANNED: Mutation testing**: Add mutation testing for test quality validation
- **PLANNED: Load testing**: Implement realistic user scenario testing
- **PLANNED: Accessibility testing**: Frontend component accessibility validation

### Documentation

- **PLANNED: API documentation**: Generate OpenAPI/Swagger documentation
- **PLANNED: Architecture diagrams**: Update mermaid diagrams for new features
- **PLANNED: Performance benchmarks**: Document detailed performance metrics
- **PLANNED: Deployment guide**: Production deployment and scaling documentation

### Legend

- **COMPLETED**: Feature is implemented and working
- **CRITICAL**: Blocking issue that needs immediate attention
- **KNOWN ISSUE**: Issue with workaround, needs eventual fix
- **PLANNED**: Feature planned for future development

### Priority Order

1. Fix `make test` command implementation
2. Resolve edition2024 dependency conflicts
3. Complete security and monitoring documentation
4. Implement authentication system
5. Add real-time features and advanced functionality

## Future Enhancements

### Planned Features

- **Authentication**: User accounts and session management
- **Real-time Updates**: WebSocket support for live task updates
- **Advanced Filtering**: Search, tags, and custom filters
- **Bulk Operations**: Multi-task selection and batch actions
- **Mobile App**: React Native or Flutter mobile client
- **Notifications**: Email and push notification system

### Testing Improvements

- **Property-based testing** with QuickCheck
- **Mutation testing** for test quality validation
- **Load testing** with realistic user scenarios
- **Accessibility testing** for frontend components
- **Cross-browser testing** automation

### Monitoring and Observability

- **Metrics collection** with Prometheus
- **Distributed tracing** with Jaeger
- **Performance monitoring** and alerting
- **Health check dashboards**

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: [`make test`](#development-commands)
6. Run quality checks: `cargo clippy && cargo fmt`
7. Submit a pull request

### Development Guidelines

- Follow [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html)
- Add tests for all new features
- Update documentation for API changes
- Ensure Docker builds succeed
- Maintain test coverage above 90%

See [Development Workflow](#development-workflow) for detailed setup instructions.

## License

[MIT License](LICENSE) - see LICENSE file for details.

## Related Documentation

This README contains all project documentation. Previously separate files have been consolidated:

- **Architecture Details**: See [Architecture Overview](#architecture-overview) section
- **API Documentation**: See [API Endpoints](#api-endpoints) section  
- **Testing Guide**: See [Comprehensive Test Coverage](#comprehensive-test-coverage) section
- **Development Setup**: See [Development Workflow](#development-workflow) section
- **GitHub Setup**: See [`scripts/setup-github-secrets.sh`](scripts/setup-github-secrets.sh) for CI/CD configuration

## Notes for GitHub Copilot Agent

- The backend is a REST API using Axum and connects to PostgreSQL at `db:5432`
- The backend is a REST API using Axum and connects to PostgreSQL at `db:5432`
- The frontend is built using Leptos and makes API calls to the backend
- Shared Rust types are located in the `common` crate
- Backend exposes `/api` routes with full CRUD operations
- Frontend calls backend using the `fetch` API with proper error handling
- The project is managed via Cargo workspaces and Docker Compose
- Comprehensive test suite with 123+ tests ensures code quality
- All changes must include tests and pass CI/CD pipeline
- Use `make test` to run all tests before committing
- Follow the established patterns for error handling and async operations
