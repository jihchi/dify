FROM rust:1 as builder
WORKDIR /usr/src/dify
COPY Cargo.toml .
COPY ./src ./src
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dify /usr/local/bin/dify
WORKDIR /mnt/dify
ENTRYPOINT ["dify"]
