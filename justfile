set dotenv-load

initdb:
  sqlite3 "$DATABASE_PATH" < object_db/schema.sql

ingest data:
  cargo run --release -p object_db -- ingest {{ data }}

[no-cd]
exs input_path *args:
  cargo run --release -p source_extractor -- {{ input_path }} {{ args }}

test:
  cargo t --release -- --nocapture
