# Architecture

## Current State

The current application is a small Rust backend built around `actix-web`, `sqlx`, PostgreSQL, Redis-backed sessions, and an email delivery integration.

At the moment, the project behaves like a compact modular monolith with a thin domain layer:

- `src/main.rs` starts the process and loads configuration.
- `src/startup.rs` wires the HTTP server, middleware, routes, and shared dependencies.
- `src/routes/` contains HTTP handlers for subscriptions, confirmation, login, admin pages, and newsletter publishing.
- `src/domain/` contains a few core validated value objects such as subscriber name and email.
- `src/email_client.rs` is an infrastructure adapter for the external email API.
- `src/authentication.rs` contains authentication logic and password verification.

This structure is workable for a small service, but the HTTP layer currently owns too much application logic. Several handlers perform request parsing, business flow coordination, database operations, and external I/O in the same file.

## Recommended Direction

The recommended target is not a highly abstract, multi-crate clean architecture from day one. The better fit for this project is a modular monolith with clear boundaries between:

- transport and HTTP concerns
- application use cases
- domain rules and policies
- infrastructure adapters

That gives most of the benefits commonly associated with clean architecture without forcing unnecessary complexity too early.

## Target Layering

The intended dependency direction should be:

1. HTTP and framework code depend on application code.
2. Application code depends on domain rules and repository or service abstractions.
3. Infrastructure code implements those abstractions.
4. Domain code depends on nothing framework-specific.

In practical terms:

- Request parsing, status codes, redirects, and response formatting should stay in HTTP-facing modules.
- Use-case orchestration should move into application services.
- Validation rules, state transitions, and domain language should live in domain modules.
- SQLx queries, session storage, and external email API calls should live in infrastructure adapters.

## Why This Matters

This separation makes the code easier to extend when the service grows into a larger product. Future features such as draft newsletters, scheduling, audit logging, segments, roles, or background delivery become much easier when the code is organized around business capabilities instead of routes.

## Migration Approach

The safest refactor path is incremental:

1. Keep one binary and one crate for now.
2. Reorganize by feature, not by technical type alone.
3. Extract use cases from handlers before considering a framework migration.
4. Introduce repository and service traits only where they buy isolation and testability.
5. Delay workspace or multiple binaries until there is a real worker, queue, or operational reason.

## Architectural Rule of Thumb

If a change only affects request parsing or HTTP output, it belongs in the HTTP layer.

If a change affects business behavior, it belongs in the application or domain layer.

If a change affects PostgreSQL, Redis, Postmark, or another external system, it belongs in infrastructure.
