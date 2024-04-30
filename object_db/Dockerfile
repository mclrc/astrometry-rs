FROM postgres:16.2 AS postgres

WORKDIR /app

COPY build_healpix_extension.sh .
RUN chmod +x build_healpix_extension.sh
RUN ./build_healpix_extension.sh
RUN chown -R postgres:postgres /app

USER postgres

COPY install_healpix_extension.sql /docker-entrypoint-initdb.d/
