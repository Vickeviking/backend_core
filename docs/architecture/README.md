# Architecture

## Purpose

This directory captures the current backend structure and the architectural rules that shape future changes.

## Current State

Today the repository contains:

- the backend Rust package at the repo root
- a workspace member for `xtask`
- a React frontend in `apps/web`

The current runtime characteristics are:

- separate `api` and `worker` binaries own the HTTP server and newsletter delivery loop
- `src/startup.rs` owns most application wiring
- `src/runtime.rs` owns shared process initialization such as tracing and configuration loading
- `src/features/*/presentation/http/` owns feature-facing HTTP handlers
- `src/infrastructure/` owns framework-facing auth, config, logging, and DB setup
- `src/shared/` owns reusable cross-feature helpers such as email, sessions, idempotency, and HTTP utilities
- `src/operational/http/` owns non-business endpoints such as home and health

This is a modular monolith with feature boundaries in place, while still keeping the backend as one crate.

## Target Direction

The architecture keeps the backend as one Rust package while maintaining clear boundaries between:

- feature-level domain logic
- application use cases and ports
- infrastructure adapters
- HTTP presentation code
- cross-cutting shared or operational code

This should improve testability, clarify ownership, and make it easier to add future product capabilities without another structural rewrite.

## Documents In This Folder

- `target-structure.md` describes the intended module layout and ownership rules
- `migration-strategy.md` records the phased migration path that produced the current layout
