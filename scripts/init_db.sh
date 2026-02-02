#!/usr/bin/env bash
set -x
set -eo pipefail

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
