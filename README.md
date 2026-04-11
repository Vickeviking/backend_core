# backend_core

`backend_core` is a fullstack skeleton with its center of gravity in the backend.
It packages a production-oriented Rust backend with a lightweight React shell, giving
you a starting point that already includes operational concerns such as tracing,
structured logging, typed errors, background workers, and task-driven local tooling.

The project started as a refactor of the architecture and delivery ideas from
[Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production),
then evolved into a stricter backend foundation for continued product work.

## What It Optimizes For

- a production-ready backend baseline rather than a frontend showcase
- explicit runtime entrypoints for `api` and `worker`
- observability and failure visibility through tracing and structured logs
- consistent error handling with typed domain/application errors
- local and CI workflows through `cargo xtask`
- a modular monolith that can grow without immediate service sprawl

## Architecture Direction

The backend has been refactored so clean architecture boundaries are enforced more
deliberately:

- `src/features/*` owns feature-specific domain, application, infrastructure, and presentation code
- `src/infrastructure/` owns framework and platform concerns such as config, auth, logging, and DB setup
- `src/shared/` holds cross-feature building blocks
- `src/operational/` keeps non-business HTTP endpoints separate from product features

This keeps domain logic away from transport and persistence details while preserving
the speed of a single Rust crate.

## Quick Start

- `cargo xtask db-init`
- `cargo xtask redis-init`
- `cargo xtask dev`
- `cargo xtask ci`

Direct entrypoints:

- `cargo run --bin api`
- `cargo run --bin worker`
- `npm run dev` in `apps/web`

## Repository Shape

```text
.
├── apps/web
├── configuration
├── docs
├── src
│   ├── bin
│   ├── features
│   ├── infrastructure
│   ├── operational
│   └── shared
└── xtask
```

## Documentation

- [docs/README.md](docs/README.md)
- [docs/architecture/README.md](docs/architecture/README.md)
- [docs/development.md](docs/development.md)
- [docs/deployment.md](docs/deployment.md)
