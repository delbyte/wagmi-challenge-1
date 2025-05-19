FROM rust:1.82-slim-buster AS builder
WORKDIR /usr/src/wagmi-9000
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/wagmi-9000/target/release/wagmi-9000 /usr/local/bin/wagmi-9000
RUN ulimit -n 8192
EXPOSE 8000
CMD ["wagmi-9000"]