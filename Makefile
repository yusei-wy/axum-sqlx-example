init:
	mkdir -p docker/postgres-data

clean:
	cd docker && docker compose down -v

start-db:
	cd docker && docker compose up --build

serve:
	cargo watch -x run
