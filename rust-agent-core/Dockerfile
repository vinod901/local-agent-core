# Build stage
FROM rust:1.75 as builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/librust_agent_core.so /app/ || true
COPY --from=builder /build/target/release/librust_agent_core.a /app/ || true

# Create data directory
RUN mkdir -p /data

# Note: This Dockerfile is for the library.
# In production, you'd build a binary that uses this library.
CMD ["/bin/bash"]
