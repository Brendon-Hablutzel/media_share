FROM rust:slim-bullseye AS builder

RUN apt-get update && apt-get install -y \    
    ca-certificates pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli

WORKDIR /usr/src/app

COPY templates ./templates
COPY migrations ./migrations
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

EXPOSE 8080

COPY run.sh .
RUN chmod +x run.sh
ENTRYPOINT ./run.sh