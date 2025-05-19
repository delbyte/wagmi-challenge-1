FROM rust:1.82-slim AS builder
WORKDIR /usr/src/wagmi-9000
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/wagmi-9000/target/release/wagmi-9000 /usr/local/bin/wagmi-9000
EXPOSE 8000
CMD ["wagmi-9000"]