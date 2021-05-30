SHELL := /bin/zsh

db_url := postgres://postgres:my_password@localhost:5434/my_database

run_server:
	DATABASE_URL=$(db_url) \
		       RUST_BACKTRACE=full \
		       cargo run --bin server


.PHONY: run_server

create_migration:
	DATABASE_URL=$(db_url) diesel migration generate $(name) --migration-dir=db/migrations

migrate:
	DATABASE_URL=$(db_url) diesel migration run --migration-dir=db/migrations

redo_migrate:
	DATABASE_URL=$(db_url) diesel migration redo --migration-dir=db/migrations


