FROM rust:1.73.0-slim AS builder
WORKDIR /usr/src/wagmi-9000
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/wagmi-9000/target/release/wagmi-9000 /usr/local/bin/wagmi-9000
EXPOSE 8000
CMD ["wagmi-9000"]