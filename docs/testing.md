# Testing

## Purpose

This document explains the current testing structure and the checks expected during development.

## Current Structure

The repository uses:

- co-located unit tests inside feature and shared modules
- a single integration test crate rooted at `tests/integration/main.rs`
- support modules in `tests/integration/support/`
- HTTP flow coverage in `tests/integration/http/`
- worker and queue behavior coverage in `tests/integration/worker/`
- mocked external email delivery using `wiremock`

## What The Integration Helpers Own

The support layer is split by responsibility:

- app boot and isolated database setup
- authenticated test users
- newsletter and subscription fixtures
- bounded worker execution helpers

The worker helpers intentionally drain the queue with bounded iterations instead of spawning the infinite production loop inside tests.

## Current Commands

- `cargo test`
- `cargo xtask ci`
- `cargo sqlx prepare --workspace --check -- --all-targets`
- `npm run lint` in `apps/web`

## Testing Rule Of Thumb

If a rule can be tested without a server, database, or network, it should usually be tested that way first.

If a behavior depends on routing, middleware, SQL queries, sessions, or external APIs, it should also have integration coverage.
