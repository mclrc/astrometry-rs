# Postgres database
FROM postgres:16.2 AS postgres

COPY ./setup.sql /docker-entrypoint-initdb.d/setup.sql
