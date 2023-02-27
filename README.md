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
| `make dev` | RUST_LOG=debug で cargo-watch で監視し `cargo run` する |


## API

| path | method |  request |
| --- | --- | --- |
| /api/users | POST | `curl -X POST -H "Content-Type: application/json" -d '{"nickname": "", "birthday": "1993-01-01"}' localhost:3000/api/users` |
| /api/users/:user_id | GET | `curl localhost:3000/api/users/:user_id` |
