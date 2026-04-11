# API

## Purpose

This document describes the current HTTP surface at a high level. It is intentionally lightweight and serves as a readable reference while the codebase is refactored.

There is no dedicated OpenAPI document in the repository yet. If the project grows or external consumers are added, an `openapi.yaml` should be introduced separately.

## Current Endpoints

### `GET /health_check`

Returns a successful response when the service is running.

### `GET /`

Returns the home page for the current web-first backend surface.

### `POST /subscriptions`

Creates a new pending subscription and sends a confirmation email.

Expected form fields:

- `name`
- `email`

### `GET /subscriptions/confirm`

Confirms a pending subscription based on the `subscription_token` query parameter.

### `GET /login`

Returns the admin login form.

### `POST /login`

Authenticates an admin and creates a Redis-backed session.

### `GET /admin/dashboard`

Returns the admin dashboard for authenticated users.

### `GET /admin/password`

Returns the password change form for authenticated users.

### `POST /admin/password`

Attempts to change the password for the logged-in admin user.

### `POST /admin/logout`

Clears the session for the current admin user.

### `GET /admin/newsletters`

Returns the newsletter publishing form for authenticated users.

### `POST /admin/newsletters`

Publishes a newsletter issue to confirmed subscribers.

Expected form fields:

- `title`
- `text_content`
- `html_content`
- `idempotency_key`

This endpoint requires an authenticated admin session.

## API Notes

The current system is HTML form driven:

- public subscription flow is form-based
- admin flows are form-based and session-backed
- newsletter delivery is asynchronous after the issue is accepted

## Recommended Next Step

When the product scope becomes clearer, the API should be described in one of two ways:

- a human-readable internal API reference plus tests, if the service remains internal
- a proper OpenAPI contract, if the service will support external clients or a separate frontend
