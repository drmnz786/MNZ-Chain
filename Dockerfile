FROM rust:1.88-slim AS builder
WORKDIR /app

# Install build dependencies required for compiling openssl-sys
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/mnz_chain /app/mnz_chain
EXPOSE 8080
CMD ["./mnz_chain"]


