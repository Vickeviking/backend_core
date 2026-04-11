# Security

## Purpose

This document captures the current security posture and the main areas that require attention as the project matures.

## Current Security-Relevant Areas

The service currently deals with:

- admin authentication
- session state
- password hashing
- email-based confirmation
- application secrets in configuration

## Authentication Model

The current privileged flows use session-based authentication for the admin web surface.

The main moving parts are:

- login creates a session-backed admin state
- middleware protects `/admin/*` routes by rejecting anonymous users
- logout clears session state
- password changes require the current password

## Password Handling

Password verification is already based on Argon2, which is the right class of algorithm.

The next security questions are operational:

- how admin credentials are provisioned
- how secrets are rotated
- whether seeded credentials should exist in production-like paths
- whether password change and logout flows are fully covered during the refactor

## Secret Management

Secrets should not be treated as stable values committed to the repository for real environments.

Local defaults may exist for development, but production secrets should come from a proper secret management path through environment variables or managed infrastructure settings.

## Confirmation Flow

Subscription confirmation currently depends on a token stored in the database and sent by email.

If this flow becomes business-critical, future hardening may include:

- explicit token expiration
- single-use guarantees
- audit logging
- better error handling and abuse monitoring

## Recommended Security Priorities

1. remove any production-like seeded secrets or credentials from committed defaults
2. keep password change and logout flows regression-tested during the refactor
3. define a clear long-term authentication and authorization strategy for admin operations
4. document secret sources and rotation expectations
5. keep security decisions written down as ADRs when they affect architecture
