#!/usr/bin/env bash
set -x
set -eo pipefail



# ensure sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# check if custom user has been created otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check if a custom database name has been set otherwise default to 'backend_core'
DB_NAME="${POSTGRES_DB:=backend_core}"
# check if a fdefault port has been set otherwise default to port 5432
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set otherwise set to localhost
DB_HOST="${POSTGRES_HOST:=localhost}"

#launch postgres using Docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
# increased maximum of connections due to testing 

# Keep pinging postgres until it's ready to accept commands
export PGPASSWORD="$DB_PASSWORD"
until psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -d postgres -c '\q' >/dev/null 2>&1; do
  >&2 echo "Postgres is still unavailable, sleeping"
  sleep 1
done


>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# Create the application database
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
sqlx database create
