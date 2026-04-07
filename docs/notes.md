# Notes

## Proposed Structure

Below is a proposed target structure for a future refactor:

```text
src/
  main.rs
  app/
    bootstrap.rs
    router.rs
    state.rs
  modules/
    subscriptions/
      http.rs
      application.rs
      domain.rs
      repository.rs
    newsletters/
      http.rs
      application.rs
      domain.rs
      repository.rs
    admin/
      http.rs
      application.rs
      repository.rs
  infra/
    postgres/
    email/
    session/
  shared/
    config.rs
    error.rs
    telemetry.rs
```

This should be treated as a direction, not a rigid law. The main idea is to organize the code by business capability first and technical concern second.

## What Each Area Should Own

### `main.rs`

This file should stay very small. Its job is to start the application, initialize tracing, load configuration, and delegate to the bootstrap layer.

### `app/`

This package should own application-wide composition.

- `bootstrap.rs` should assemble dependencies and start the server.
- `router.rs` should define routes and middleware wiring.
- `state.rs` should define shared application state passed into handlers.

This area should know about frameworks and infrastructure because it is the composition root.

### `modules/`

Each module should represent a business capability, not a transport mechanism.

For example:

- `subscriptions` owns the subscribe and confirm flows
- `newsletters` owns newsletter publishing behavior
- `admin` owns login, session-aware admin actions, password management, and later authorization concerns

Each module can then contain smaller internal boundaries.

## DTO Guidance

DTOs should exist where data crosses a boundary.

Typical DTO categories are:

- HTTP request and response DTOs
- application command or query DTOs
- persistence row mapping DTOs when needed

### Where HTTP DTOs Should Live

HTTP-facing DTOs usually belong close to the transport layer, which means near `http.rs`.

If a module grows, it can become:

```text
modules/
  subscriptions/
    http/
      mod.rs
      dto.rs
```

That keeps transport-specific shape separate from business types.

Examples of HTTP DTOs:

- form payloads
- JSON payloads
- query parameter structs
- response serialization structs

These should not become your domain model by default.

### When Application DTOs Make Sense

Application DTOs are useful when a use case should remain independent from HTTP or database concerns.

Examples:

- `SubscribeCommand`
- `PublishNewsletterCommand`
- `ChangePasswordCommand`

These represent input to a use case, not an HTTP body and not a SQL row.

### What Should Not Be a DTO

Domain objects should not be called DTOs if they carry business meaning and rules.

For example:

- `SubscriberEmail`
- `SubscriberName`
- `SubscriptionStatus`

These are domain types, not transport containers.

## Error Placement

Errors should be defined where they can be understood and handled correctly.

### Domain Errors

Domain errors belong in the domain layer when they represent violated business rules or invalid state.

Examples:

- invalid subscriber email
- invalid subscriber name
- illegal status transition

These errors should not mention HTTP status codes.

### Application Errors

Application errors belong in the use-case layer when orchestration fails for a meaningful business reason.

Examples:

- subscriber already exists
- confirmation token not found
- current password is invalid
- newsletter cannot be published to an empty audience

These errors still should not decide whether the transport returns `400`, `401`, `404`, or `500`.

### Infrastructure Errors

Infrastructure errors belong in adapters or infrastructure modules when the failure is about an external dependency.

Examples:

- database query failed
- Redis session store failed
- email provider returned an error

These errors should usually be translated upward into application-level failures when appropriate.

### HTTP Errors

HTTP concerns should stay in `http.rs`.

This layer is responsible for:

- mapping domain or application errors to HTTP responses
- choosing status codes
- building redirects
- formatting error bodies or flash messages

This keeps framework-specific details out of business logic.

## Repository Guidance

The proposed `repository.rs` files should define the interfaces that the application layer needs from persistence, not necessarily the SQLx implementation itself.

For example, a module repository may define capabilities such as:

- store pending subscriber
- fetch confirmed subscribers
- mark subscriber as confirmed
- update admin password

The concrete SQLx implementation can then live under `infra/postgres/`.

This pattern keeps the application layer focused on behavior while still allowing pragmatic direct SQLx usage in the infrastructure layer.

## Service Guidance

External systems should be treated similarly to repositories.

For example:

- email sending can be exposed to the application layer as an interface
- session operations can be wrapped behind a small abstraction if they start leaking framework details into use cases

Do not abstract everything preemptively. Introduce ports where they improve clarity, testability, or replaceability.

## Shared Code Guidance

`shared/` should contain only code that is truly cross-cutting and not owned by one module.

Good candidates:

- configuration loading
- telemetry setup
- generic error helpers

Bad candidates:

- business logic that belongs to one feature
- catch-all helpers with unclear ownership

If a helper is only used by one module, it should usually live in that module instead.

## Suggested Rule Set

Use these rules to keep the structure healthy:

1. Domain types should not depend on Actix, Axum, SQLx, or reqwest.
2. HTTP DTOs should not be reused as domain entities by default.
3. HTTP status code decisions should stay in the transport layer.
4. SQL queries should not live in `http.rs`.
5. Repository traits should describe business needs, not database tables.
6. Shared modules should remain small and boring.

## Final Note

If the service remains small, this structure can stay within one crate and one binary for a long time.

If the service grows into asynchronous delivery, jobs, analytics, or template processing, then additional binaries or workers may become justified. That should happen because the product demands it, not because the folder tree looked nice in advance.
