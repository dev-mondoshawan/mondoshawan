# Mondoshawan Blockchain Node - Dockerfile
FROM rust:1.75 as builder

WORKDIR /app

# Copy source code
COPY mondoshawan-blockchain/Cargo.toml mondoshawan-blockchain/Cargo.toml
COPY mondoshawan-blockchain/src mondoshawan-blockchain/src

# Build the node
WORKDIR /app/mondoshawan-blockchain
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/mondoshawan-blockchain/target/release/node /app/node

# Create data directory
RUN mkdir -p /data

# Expose ports
EXPOSE 8080 8545 9090

# Default command
CMD ["./node"]
