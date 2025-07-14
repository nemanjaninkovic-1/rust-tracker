# Frontend Dockerfile
FROM rust:latest AS builder

# Install wasm-pack for building WASM applications
RUN cargo install wasm-pack

WORKDIR /app

# Copy workspace configuration first
COPY Cargo.toml ./

# Copy all crate manifests to establish dependencies
COPY common/Cargo.toml ./common/
COPY frontend/Cargo.toml ./frontend/

# Create minimal backend structure for workspace satisfaction
RUN mkdir -p backend/src
RUN echo '[package]\nname = "backend"\nversion = "0.1.0"\nedition = "2021"' > backend/Cargo.toml
RUN echo 'fn main() {}' > backend/src/main.rs

# Copy the lockfile after workspace is set up
COPY Cargo.lock ./

# Copy source code
COPY common/src/ ./common/src/
COPY frontend/src/ ./frontend/src/

# Build the frontend WASM package
RUN cd frontend && wasm-pack build --target web --out-dir pkg --release

# Nginx stage for serving the frontend
FROM nginx:alpine

# Copy the built WASM files
COPY --from=builder /app/frontend/pkg /usr/share/nginx/html/pkg

# Copy static files
COPY frontend/index.html /usr/share/nginx/html/
COPY frontend/nginx.conf /etc/nginx/nginx.conf

EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]
