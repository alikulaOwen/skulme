# Multi-stage production container build for Skul.me API & Web Platform
FROM rust:1.80-slim as builder

WORKDIR /usr/src/skulme

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace source files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY problems ./problems

# Build the API binary in release mode
RUN cargo build --release -p skulme-api

# Minimal distroless/debian runtime container
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    python3 \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled API binary from builder
COPY --from=builder /usr/src/skulme/target/release/skulme-api /app/skulme-api

# Copy web frontend and problem definitions for runtime static serving and dynamic ingestion
COPY crates/skulme-web /app/crates/skulme-web
COPY problems /app/problems

EXPOSE 3000

ENV RUST_LOG=info

ENTRYPOINT ["/app/skulme-api"]

