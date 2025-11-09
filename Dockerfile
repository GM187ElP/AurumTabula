<<<<<<< HEAD
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin aurumtabula

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/aurumtabula /usr/local/bin
ENTRYPOINT ["/usr/local/bin/aurumtabula"]
=======
# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY templates ./templates

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/aurumtabula /app/aurumtabula

# Copy templates
COPY templates ./templates

# Expose port
EXPOSE 8080

# Set environment variable for the port
ENV PORT=8080

# Run the binary
CMD ["/app/aurumtabula"]

>>>>>>> develop
