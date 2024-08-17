generate-migration:
	diesel migration generate --migration-dir infrastructure/migrations $(name)

redo-migrations:
	@echo -n "WARNING: WILL DROP ALL TABLES AND REMOVE ALL STORED DATA. Are you sure? [yes/N] " && read ans && [ $${ans:-N} = yes ]
	diesel migration redo --all --migration-dir infrastructure/migrations

migrate:
	diesel migration run --migration-dir infrastructure/migrations

preview-schema:
	diesel print-schema
generate-schema:
	diesel print-schema > domain/src/schema.rs

reset-database:
	@echo -n "WARNING: WILL DROP ALL TABLES AND REMOVE ALL STORED DATA. Are you sure? [yes/N] " && read ans && [ $${ans:-N} = yes ]
	diesel database reset --migration-dir infrastructure/migrations

database:
	psql -U admin -d otl -h localhost
