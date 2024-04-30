ingest data: startdb
  cargo run -p object_db -- ingest {{ data }}

startdb:
  docker compose up -d

shell: startdb
  docker compose exec database /bin/bash

migrate: startdb
  #!/bin/bash
  cd object_db
  diesel migration run
