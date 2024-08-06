set dotenv-load

initdb:
  sqlite3 "$DATABASE_PATH" < object_db/schema.sql

ingest data:
  cargo run --release -p object_db -- ingest {{ data }}

[no-cd]
exs input_path *output_path:
  cargo run --release -p source_extractor -- {{ input_path }} {{ output_path }}

test:
  cargo t --release -- --nocapture
