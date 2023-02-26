ARG RUST_VER=1.67.1

FROM rust:${RUST_VER}-bullseye AS builder

RUN USER=root cargo new --bin app
WORKDIR /app

# Install dependencies first for cache
COPY ../Cargo.* ./
RUN rustup component add clippy rustfmt
RUN cargo install cargo-watch sqlx-cli
RUN cargo build --release
RUN rm src/*.rs

COPY ../ /app
CMD ["make", "serve"]
