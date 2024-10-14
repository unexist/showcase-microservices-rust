.DEFAULT_GOAL := build
.ONESHELL:

PODNAME := showcase
PG_USER := postgres
PG_PASS := postgres

define JSON_TODO
curl -X 'POST' \
  'http://localhost:8080/todo' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "description": "string",
  "done": true,
  "title": "string"
}'
endef
export JSON_TODO

# Tools
todo:
	@echo $$JSON_TODO | bash

list:
	@curl -X 'GET' 'http://localhost:8080/todo' -H 'accept: */*' | jq .

psql:
	@PGPASSWORD=$(PG_PASS) psql -h 127.0.0.1 -U $(PG_USER)

psql-schema:
	@PGPASSWORD=$(PG_PASS) psql -h 127.0.0.1 -U $(PG_USER) -f ./schema.sql

# Build
build-actix:
	@$(SHELL) -c  "cd todo-service-actix; cargo build"

build-spring-rs:
	@$(SHELL) -c  "cd todo-service-spring-rs; cargo build"

# Run
run-actix:
	@$(SHELL) -c  "cd todo-service-actix; APP_DB_USERNAME=$(PG_USER) APP_DB_PASSWORD=$(PG_PASS) APP_DB_NAME=postgres cargo run"

run-spring-rs:
	@$(SHELL) -c  "cd todo-service-spring-rs; APP_DB_USERNAME=$(PG_USER) APP_DB_PASSWORD=$(PG_PASS) APP_DB_NAME=postgres cargo run"

# Tests
test-actix:
	@$(SHELL) -c "cd todo-service-actix; cargo test"

test-spring-rs:
	@$(SHELL) -c "cd todo-service-spring-rs; cargo test"

# Helper
clean:
	@$(SHELL) -c "cd todo-service-actix; cargo clean"
	@$(SHELL) -c "cd todo-service-spring-rs; cargo clean"