# Postgres database
FROM postgres:17.1 AS postgres

COPY ./init.sql /docker-entrypoint-initdb.d/init.sql
