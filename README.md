# RustTracker

RustTracker is a full-stack task management web application built entirely in Rust. It features a fast backend using Axum, a reactive frontend using Leptos, and a PostgreSQL database. The entire project is containerized using Docker and Docker Compose for easy setup and deployment.

## Features

- Create, read, update, and delete tasks
- Filter by status, category, or due date
- RESTful API and responsive web interface
- Shared Rust models between frontend and backend
- PostgreSQL for persistent data storage
- Fully containerized using Docker

## Project Structure

```text
rust-tracker/
├── backend/              # Axum API server
│   ├── src/
│   └── Dockerfile
├── frontend/             # Leptos web application
│   ├── src/
│   └── Dockerfile
├── common/               # Shared data models
│   └── src/
├── docker-compose.yml    # Docker Compose orchestration
├── .env                  # Environment variables
└── README.md
```

## Requirements

- Docker
- Docker Compose

No need to install Rust, PostgreSQL, or frontend tooling locally.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/nemanjaninkovic-1/rust-tracker.git
   cd rust-tracker
   ```

2. Build and run the application:

   ```bash
   docker-compose up --build
   ```

This will:

- Start the PostgreSQL container
- Build and start the backend on <http://localhost:8080>
- Build and start the frontend on <http://localhost:3000>

## Environment Configuration

Create a `.env` file based on `.env.example`:

```env
DATABASE_URL=postgres://postgres:password@db:5432/rusttracker
RUST_LOG=info
```

These values are used by both the backend and SQLx migrations.

## Example API Endpoints

- GET /api/tasks
- POST /api/tasks
- PUT /api/tasks/:id
- DELETE /api/tasks/:id

All endpoints use JSON and the Task model from the `common` crate.

## Volumes and Persistence

PostgreSQL data is stored in a Docker volume defined in `docker-compose.yml`.

To remove containers and volumes:

```bash
docker-compose down -v
```

## Technologies Used

- Rust
- Actix-Web
- Leptos
- SQLx
- PostgreSQL
- Docker
- Docker Compose
- Cargo workspaces

## License

MIT License

## Notes for GitHub Copilot Agent

- The backend is a REST API using Actix-Web and connects to PostgreSQL at `db:5432`
- The frontend is built using Leptos and makes API calls to the backend
- Shared Rust types are located in the `common` crate
- Backend exposes `/api` routes
- Frontend calls backend using the `fetch` API
- The project is managed via Cargo workspaces and Docker Compose
