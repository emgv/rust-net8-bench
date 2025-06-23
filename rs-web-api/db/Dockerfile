FROM postgres:17.5-alpine

ENV POSTGRES_USER=cars
ENV POSTGRES_PASSWORD=hellopwd
COPY ./pg_hba.conf /var/lib/postgresql/
COPY ./certs/server.key /var/lib/postgresql/
COPY ./certs/server.crt /var/lib/postgresql/
RUN chown postgres:postgres /var/lib/postgresql/server.key && chmod 600 /var/lib/postgresql/server.key
RUN chown postgres:postgres /var/lib/postgresql/server.crt && chmod 600 /var/lib/postgresql/server.crt
RUN chown postgres:postgres /var/lib/postgresql/pg_hba.conf && chmod 600 /var/lib/postgresql/pg_hba.conf

WORKDIR /docker-entrypoint-initdb.d
COPY ./db-init.sh .
RUN chmod +x /docker-entrypoint-initdb.d/db-init.sh
