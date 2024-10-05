generate-migration:
	docker compose exec -it backend diesel migration generate --migration-dir infrastructure/migrations $(name)

redo-migrations:
	@echo -n "WARNING: WILL DROP ALL TABLES AND REMOVE ALL STORED DATA. Are you sure? [yes/N] " && read ans && [ $${ans:-N} = yes ]
	docker compose exec -it backend diesel migration redo --all --migration-dir infrastructure/migrations

migrate:
	docker compose exec -it backend diesel migration run --migration-dir infrastructure/migrations

preview-schema:
	docker compose exec -it backend diesel print-schema
generate-schema:
	docker compose exec -it backend diesel print-schema > domain/src/schema.rs

reset-database:
	@echo -n "WARNING: WILL DROP ALL TABLES AND REMOVE ALL STORED DATA. Are you sure? [yes/N] " && read ans && [ $${ans:-N} = yes ]
	docker compose exec -it backend diesel database reset --migration-dir infrastructure/migrations

database:
	psql -U admin -d otl -h localhost

build:
	cross build --target aarch64-unknown-linux-gnu --release
