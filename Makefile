# Default target
help:
	@echo "Available commands:"
	@echo "  make install              - Install diesel_cli"
	@echo "  make setup                - Set up diesel for the project"
	@echo "  make generate-migration   - Generate a new migration"
	@echo "  make run-migration        - Run all pending migrations"
	@echo "  make revert-migration     - Revert the last migration"
	@echo "  make redo-migration       - Redo the last migration"
	@echo "  make redo-all-migrations  - Redo all migrations"
	@echo "  make build                - Build the application"
	@echo "  make run                  - Run the application"
	@echo "  make test                 - Test the application"
	@echo "  make help                 - Show this help message"

# Install diesel_cli
install:
	cargo install diesel_cli --no-default-features --features postgres

# Set up diesel for the project
setup:
	diesel setup

# Generate a new migration
generate-migration:
	@read -p "Enter migration name: " name; \
	diesel migration generate $$name

# Run all pending migrations
run-migration:
	diesel migration run

# Revert the last migration
revert-migration:
	diesel migration revert

# Redo the last migration
redo-migration:
	diesel migration redo

# Redo all migrations
redo-all-migrations:
	diesel migration redo --all

# Build the application
build:
	cargo build

# Run the application
run:
	cargo run

# Test the application
test:
	./run_tests.sh

# Phony targets
.PHONY: install setup generate-migration run-migration revert-migration redo-migration redo-all-migrations build run test help