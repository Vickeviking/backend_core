# Testing

## Purpose

This document explains the current testing strategy and the intended direction as the project evolves.

## Current Testing Style

The project currently uses a useful mix of:

- unit tests for domain validation
- integration tests that boot the application
- mocked external email delivery using `wiremock`

This is a strong foundation for a service of this size.

## What Is Working Well

The test suite already exercises full request flows such as:

- subscription creation
- confirmation flow
- admin login
- newsletter publishing behavior

This is valuable because it tests behavior through the public API instead of only testing implementation details.

## Intended Testing Layers

As the codebase is refactored, tests should be organized around three layers:

### Domain Tests

These verify pure business rules and validation logic without HTTP, database, or network dependencies.

### Application Tests

These verify use-case behavior, such as subscribing a user or publishing an issue, against mocked repositories or services.

### Integration Tests

These verify wiring, persistence, routing, middleware, authentication boundaries, and infrastructure behavior through the real application surface.

## Testing Rule of Thumb

If a rule can be tested without a server, database, or network, it should usually be tested that way first.

If a behavior depends on routing, middleware, SQL queries, sessions, or external APIs, it should also have integration coverage.

## Future Additions

As the product grows, the test strategy should likely expand to include:

- database-focused repository tests
- contract tests for external providers
- regression tests for authentication and authorization
- smoke tests for deployment environments
