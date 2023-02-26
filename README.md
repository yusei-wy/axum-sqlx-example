# axum-sqlx-example

axum x sqlx 

## Requirements

- Docker
- Rust 1.67.1
- cargo-watch
- sqlx-cli

## Commands

| command | description |
| --- | --- |
| `make build` | PostgreSQL DB を docker 上で起動する |
| `make add-migrate NAME=<name>` | Migration ファイルを追加 |
| `make serve` | cargo-watch で監視した状態で `cargo run` する |
