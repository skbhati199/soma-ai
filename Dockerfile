# Dockerfile for building Soma AI
# Multi-stage build for minimal final image

# Build stage
FROM rust:1.99-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    cmake \
    g++ \
    make \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files first for caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true

# Copy actual source
COPY src ./src
COPY assets ./assets
COPY build.rs ./

# Build actual application
ARG TARGET=x86_64-unknown-linux-gnu
RUN cargo build --release && \
    mkdir -p /app/target/${TARGET}/release && \
    cp /app/target/release/soma /app/target/${TARGET}/release/soma

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/soma /usr/local/bin/soma

# Create non-root user
RUN useradd -m -s /bin/bash soma
USER soma
WORKDIR /home/soma

# Default command
ENTRYPOINT ["soma"]
CMD ["--help"]