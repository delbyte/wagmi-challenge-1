# Build stage
FROM rust:1.87-slim AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Final stage
FROM debian:stable-slim

# Install necessary runtime dependencies for Actix-web (OpenSSL)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/wagmi-9000 /usr/local/bin/wagmi-9000

# Expose the default port (for documentation)
EXPOSE 8000

# Run the application
CMD ["wagmi-9000"]