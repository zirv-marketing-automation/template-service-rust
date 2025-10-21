# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.90 as builder
WORKDIR /usr/src/app

# Install system dependencies for building SQLx and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml

# Pre-fetch dependencies
RUN mkdir backend/src && \
    echo 'fn main() {}' > backend/src/main.rs && \
    cargo build -p backend --release && \
    rm -r backend/src

# Copy actual source
COPY . .

# Build release binary
RUN cargo build -p backend --release

# Runtime stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN adduser --disabled-login --gecos '' appuser

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Directory for persistent logs
RUN mkdir -p /var/log/backend && chown appuser:appuser /var/log/backend
VOLUME /var/log/backend

USER appuser
EXPOSE 3000
ENV RUST_LOG=info
CMD ["sh", "-c", "backend > /var/log/backend/server.log 2>&1"]
