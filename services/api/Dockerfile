# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.90 as builder
WORKDIR /usr/src/app

# Install system dependencies for building SQLx and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY common/Cargo.toml common/Cargo.toml
COPY services/api/Cargo.toml services/api/Cargo.toml

# Pre-fetch dependencies
RUN mkdir -p common/src services/api/src && \
    echo 'fn main() {}' > services/api/src/main.rs && \
    touch common/src/lib.rs && \
    cargo build -p api --release && \
    rm -r common/src services/api/src

# Copy actual source
COPY . .

# Build release binary
RUN cargo build -p api --release

# Runtime stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN adduser --disabled-login --gecos '' appuser

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin/api

# Directory for persistent logs
RUN mkdir -p /var/log/api && chown appuser:appuser /var/log/api
VOLUME /var/log/api

USER appuser
EXPOSE 3000
ENV RUST_LOG=info
CMD ["sh", "-c", "api > /var/log/api/server.log 2>&1"]
