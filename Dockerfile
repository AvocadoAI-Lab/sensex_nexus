# Builder stage
FROM rust:1.70-slim-bullseye as builder

# Create a new empty shell project
WORKDIR /usr/src/sensex_nexus

# Install required dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/sensex_nexus/target/release/sensex_nexus /usr/local/bin/sensex_nexus

# Expose port if needed (adjust based on your application)
EXPOSE 8000

# Set the startup command
CMD ["sensex_nexus"]
