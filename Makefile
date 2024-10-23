.PHONY: migrate-database
migrate-database:
	@POSTGRES_URL="postgres://admin:admin@localhost:5432/iot-manager" cargo run --bin tool
