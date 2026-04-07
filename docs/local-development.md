# Local Development

## Purpose

This document describes how to run the project locally and what supporting services are required.

## Required Services

The backend depends on:

- PostgreSQL
- Redis

The repository already includes helper scripts for local setup:

- `scripts/init_db.sh`
- `scripts/init_redis.sh`

## Basic Flow

The expected local workflow is:

1. start PostgreSQL
2. start Redis
3. run migrations
4. run the application
5. run tests and checks

## Common Commands

Typical commands mentioned elsewhere in the repository include:

- `./scripts/init_db.sh`
- `./scripts/init_redis.sh`
- `cargo check`
- `cargo test`
- `cargo watch -x check -x test -x run`
- `./scripts/ci-check.sh`

## Environment and Configuration

Configuration is read from:

- `configuration/base.yaml`
- `configuration/local.yaml`
- environment variables prefixed with `APP_`

For local development, the application defaults to the local environment unless `APP_ENVIRONMENT` is explicitly set.

## Developer Expectations

At the moment, local setup knowledge is spread across `dev.md`, scripts, and source code. This file should become the canonical local onboarding document over time.

The long-term goal is that a new developer can clone the repository, read this file, and successfully run the service without needing tribal knowledge.
