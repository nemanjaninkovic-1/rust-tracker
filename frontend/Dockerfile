# Frontend Dockerfile
FROM rust:latest AS builder

# Install wasm-pack for building WASM applications
RUN cargo install wasm-pack

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Create backend directory with minimal Cargo.toml to satisfy workspace
RUN mkdir -p backend/src
RUN echo '[package]\nname = "backend"\nversion = "0.1.0"\nedition = "2021"' > backend/Cargo.toml
RUN echo 'fn main() {}' > backend/src/main.rs

# Copy actual source code
COPY common/ ./common/
COPY frontend/ ./frontend/

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
