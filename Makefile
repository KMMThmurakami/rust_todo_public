build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

down-a:
	docker-compose down --rmi 'all'

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x run

test:
	cargo test

# standalone test
test-s:
	cargo test --no-default-features
