init:
	docker network create axum-sqlx-example
	mkdir -p docker/postgres-data

clean:
	cd docker && docker compose down -v

login-db:
	cd docker && docker compose exec db psql -U postgres sample

build:
	cd docker && docker compose up --build

add-migrate:
	sqlx migrate add -r ${NAME}

dev:
	sqlx database create
	sqlx migrate run
	RUST_LOG=debug cargo watch -x run -w src
