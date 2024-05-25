set dotenv-load

initdb:
  sqlite3 "$DATABASE_PATH" < object_db/schema.sql

download-catalog destination:
  cargo run --release -p object_db -- download-catalog {{ destination }}

ingest data:
  cargo run --release -p object_db -- ingest {{ data }}
