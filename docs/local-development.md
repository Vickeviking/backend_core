# Local Development

## Purpose

This document describes the shortest path to a working local environment.

## Required Services

The backend depends on:

- PostgreSQL
- Redis

The repository now also includes a frontend app in `apps/web`.

## Basic Flow

The expected local workflow is:

1. start PostgreSQL
2. start Redis
3. install frontend dependencies
4. run the application
5. run tests and checks

## Common Commands

Primary commands:

- `cargo xtask dev`
- `cargo xtask ci`
- `cargo xtask db-init`
- `cargo xtask redis-init`
- `cargo run --bin api`
- `cargo run --bin worker`
- `npm run dev` in `apps/web`

Legacy fallback wrappers:

- `./scripts/init_db.sh`
- `./scripts/init_redis.sh`
- `./scripts/ci-check.sh`
- `./scripts/docker-clean`
- `./scripts/docker-clean --yes`

## Recommended Flow

1. `cargo xtask db-init`
2. `cargo xtask redis-init`
3. `cargo xtask dev`
4. `cargo xtask ci`

For SQLx query freshness checks, also run:

- `cargo sqlx prepare --workspace --check -- --all-targets`

## Environment And Configuration

Configuration is read from:

- `configuration/base.yaml`
- `configuration/local.yaml`
- environment variables prefixed with `APP_`

For local development, the application defaults to the local environment unless `APP_ENVIRONMENT` is explicitly set.

## Current Runtime Shape

Today the backend is split into:

- `api` for the HTTP server
- `worker` for the newsletter delivery loop

The frontend developer shell runs from `apps/web`.

Longer-form workflow notes and command references live in `docs/development.md`.
