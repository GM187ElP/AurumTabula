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

