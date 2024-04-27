ingest data:
  cargo run -p object_db -- ingest {{ data }}

startdb:
  docker compose up -d
