# Build stage
FROM rust:1.87-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release && strip /app/target/release/wagmi-9000

# Final stage
FROM debian:stable-slim
COPY --from=builder /app/target/release/wagmi-9000 /usr/local/bin/wagmi-9000
EXPOSE 8000
CMD ["wagmi-9000"]