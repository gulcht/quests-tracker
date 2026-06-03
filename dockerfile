# Build stage
FROM rust:1.86-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy configuration files
COPY Cargo.toml ./

# Create a dummy source file to pre-build and cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/quests_tracker*

# Copy the real source files
COPY src ./src

# Build the release binary
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Install runtime dependencies (libpq5 is required for postgres)
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from the builder stage
COPY --from=builder /app/target/release/quests-tracker /app/quests-tracker

# Expose port (adjust to your SERVER_PORT env variable default if needed)
EXPOSE 8080

# Command to run the application
CMD ["/app/quests-tracker"]
