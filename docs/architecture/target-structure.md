# Target Structure

## Repository Shape

The refactor target keeps the backend package at the repository root and adds:

- a Rust workspace entry for `xtask`
- a frontend app at `apps/web`

The backend remains one crate for business code in the first pass.

## Backend Module Layout

```text
src/
  bin/
    api.rs
    worker.rs
  features/
    subscriptions/
      domain/
        entities/
        value_objects/
        services/
        errors.rs
      application/
        dto/
        use_cases/
        ports/
          mod.rs
      infrastructure/
        persistence/
          repo.rs
          models.rs
          mappers.rs
      presentation/
        http/
          controllers/
          requests/
          responses/
    newsletter/
      ...
    authentication/
      ...
  infrastructure/
    auth/
    config/
    db/
    logging/
  operational/
    http/
  shared/
```

## Ownership Rules

### Features

Each feature owns business behavior for one capability area. The first pass covers:

- `subscriptions`
- `newsletter`
- `authentication`

### Infrastructure

`src/infrastructure/` owns framework and external-system concerns that are not business features, such as:

- configuration loading
- database connection management
- tracing and logging setup
- framework-facing authentication helpers

### Operational Code

Operational endpoints such as health checks and the home page remain outside feature folders until a real domain boundary for them exists.

### Shared Code

`src/shared/` is only for cross-feature code that does not belong to a single business capability, such as:

- email client primitives
- session helpers
- idempotency support
- generic utility code with clear cross-feature ownership

## Dependency Direction

The intended dependency direction is:

1. presentation depends on application
2. application depends on domain and declared ports
3. infrastructure implements ports and may depend on external systems
4. domain stays free of Actix, SQLx, and provider-specific concerns

## Runtime Commands

The repository uses explicit runtime entrypoints:

- `cargo run --bin api`
- `cargo run --bin worker`
- `npm run dev` in `apps/web`
- `cargo xtask dev`
- `cargo xtask ci`

## Test Layout

The target test structure is now:

- co-located unit tests inside domain and application modules where practical
- `tests/integration/main.rs`
- `tests/integration/support/`
- `tests/integration/http/`
- `tests/integration/worker/`
