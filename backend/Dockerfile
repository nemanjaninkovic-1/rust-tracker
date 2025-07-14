# Backend Dockerfile
FROM rust:latest AS builder

WORKDIR /app

# Copy workspace configuration first
COPY Cargo.toml ./

# Copy all crate manifests to establish dependencies
COPY common/Cargo.toml ./common/
COPY backend/Cargo.toml ./backend/

# Create minimal frontend structure for workspace satisfaction
RUN mkdir -p frontend/src
RUN echo '[package]\nname = "frontend"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]' > frontend/Cargo.toml
RUN echo 'fn main() {}' > frontend/src/lib.rs

# Copy the lockfile after workspace is set up
COPY Cargo.lock ./

# Copy source code
COPY common/src/ ./common/src/
COPY backend/src/ ./backend/src/
COPY backend/migrations/ ./backend/migrations/

# Build the backend specifically
RUN cargo build --release --package backend

# Runtime stage
FROM debian:bookworm-slim

# Install required dependencies for PostgreSQL client
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/backend ./backend

# Copy migrations
COPY backend/migrations ./migrations

EXPOSE 8080

CMD ["./backend"]
