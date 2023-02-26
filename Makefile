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

serve:
	sqlx database create
	sqlx migrate run
	cargo watch -x run -w src
