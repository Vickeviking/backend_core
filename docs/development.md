# Development

## Purpose

This document captures the current developer workflow, command references, and day-to-day local practices.

## Primary Commands

- `cargo xtask db-init`
- `cargo xtask redis-init`
- `cargo xtask dev`
- `cargo xtask ci`

`cargo xtask dev` is the default local shell. It:

- bootstraps PostgreSQL if needed
- bootstraps Redis if needed
- starts the API watch loop
- starts the worker watch loop
- starts the Vite dev server in `apps/web`

## Direct Entry Points

Use these when you want to run one component at a time:

- `cargo run --bin api`
- `cargo run --bin worker`
- `npm run dev` in `apps/web`

## Backend-Only Fallback Scripts

These remain available as compatibility wrappers and lower-level helpers:

- `./scripts/init_db.sh`
- `SKIP_DOCKER=true ./scripts/init_db.sh` when Postgres is already running
- `./scripts/init_redis.sh`
- `./scripts/ci-check.sh`
- `./scripts/dev/dev.sh`
- `./scripts/dev/worker.sh`

Useful supporting commands:

- `docker ps`
- `./scripts/docker-clean`
- `./scripts/docker-clean --yes`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo sqlx prepare --workspace --check -- --all-targets`

## Prerequisites

Typical local prerequisites are:

- Rust toolchain
- Node.js and npm
- Docker
- `psql`
- `sqlx-cli`

## Docker

- build: `docker build --tag backend_core --file Dockerfile .`
- run API: `docker run -e APP_RUNTIME=api -p 8000:8000 backend_core`
- run worker: `docker run -e APP_RUNTIME=worker backend_core`

The runtime image contains both binaries and selects one through `APP_RUNTIME`.

## Manual HTTP Smoke Test

```bash
curl -i -X POST \
  -d 'email=thomas_wiman@hotmail.com&name=Tom' \
  http://127.0.0.1:8000/subscriptions
```

## Logging Notes

Examples:

- `RUST_LOG="sqlx=error,info" TEST_LOG=true cargo test subscriber_fails_if_there_is_a_fatal_database_error | bunyan`
- `TEST_LOG=true cargo test health_check_works | bunyan`

For JSON log inspection:

```bash
cargo test subscriber_fails_if_there_is_a_fatal_database_error -- --exact --nocapture \
  | jq -Rrc 'fromjson? | select(.)' > /tmp/test.jsonl

lnav /tmp/test.jsonl
```

## Error Handling Notes

- use `thiserror` for typed errors the caller may want to handle explicitly
- use `anyhow` when adding context is more useful than matching exact variants
- avoid duplicate logging when errors are propagated upward unchanged
