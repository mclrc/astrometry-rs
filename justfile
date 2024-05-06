set dotenv-load

initdb:
  sqlite3 "$DATABASE_PATH" < object_db/schema.sql

ingest data:
  cargo run --release -p object_db -- ingest {{ data }}
