build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

down-a:
	docker-compose down --rmi 'all'

reset:
	docker-compose down --rmi all --volumes --remove-orphans

dev:
	docker-compose up -d
	sqlx db create
	sqlx migrate run
	cargo watch -x run

test:
	cargo test -- --color always --nocapture

# standalone test
test-s:
	cargo test --no-default-features
