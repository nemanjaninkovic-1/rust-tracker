# Backend Dockerfile
FROM rust:latest AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Create frontend directory with minimal Cargo.toml to satisfy workspace
RUN mkdir -p frontend/src
RUN echo '[package]\nname = "frontend"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]' > frontend/Cargo.toml
RUN echo 'fn main() {}' > frontend/src/lib.rs

# Copy actual source code
COPY common/ ./common/
COPY backend/ ./backend/

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
