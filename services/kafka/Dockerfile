# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.90 as builder
WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY common/Cargo.toml common/Cargo.toml
COPY kafka-messages/Cargo.toml kafka-messages/Cargo.toml
COPY services/kafka/Cargo.toml services/kafka/Cargo.toml

# Pre-fetch dependencies
RUN mkdir -p common/src kafka-messages/src services/kafka/src && \
    echo 'fn main() {}' > services/kafka/src/main.rs && \
    touch common/src/lib.rs kafka-messages/src/lib.rs && \
    cargo build -p kafka --release && \
    rm -r common/src kafka-messages/src services/kafka/src

# Copy actual source
COPY . .

# Build release binary
RUN cargo build -p kafka --release

# Runtime stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN adduser --disabled-login --gecos '' appuser

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/kafka /usr/local/bin/kafka

# Directory for persistent logs
RUN mkdir -p /var/log/kafka && chown appuser:appuser /var/log/kafka
VOLUME /var/log/kafka

USER appuser
ENV RUST_LOG=info
CMD ["sh", "-c", "kafka > /var/log/kafka/consumer.log 2>&1"]
