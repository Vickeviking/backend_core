# API

## Purpose

This document describes the current HTTP surface at a high level. It is intentionally lightweight and is meant to be a readable reference for developers.

There is no dedicated OpenAPI document in the repository yet. If the project grows or external consumers are added, an `openapi.yaml` should be introduced separately.

## Current Endpoints

### `GET /health`

Returns a successful response when the service is running.

### `GET /health_check`

Returns a successful response when the service is running. This currently overlaps with `/health`.

One of these routes should eventually become canonical to avoid duplicate health endpoints.

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

Authenticates an admin and creates a session-backed login state.

### `GET /admin/dashboard`

Returns the admin dashboard for authenticated users.

### `GET /admin/password`

Returns the password change form for authenticated users.

### `POST /admin/password`

Attempts to change the password for the logged-in admin user.

This flow is present in the HTTP surface but is not fully implemented yet in the application code.

### `POST /admin/newsletters`

Publishes a newsletter issue to confirmed subscribers.

Expected JSON payload:

```json
{
  "title": "Newsletter title",
  "content": {
    "html": "<p>Hello</p>",
    "text": "Hello"
  }
}
```

This endpoint requires an authenticated admin session.

## API Notes

The system currently serves form-based admin flows backed by sessions,
including JSON newsletter publishing under the `/admin` scope.

## Recommended Next Step

When the product scope becomes clearer, the API should be described in one of two ways:

- a human-readable internal API reference plus tests, if the service remains internal
- a proper OpenAPI contract, if the service will support external clients or a separate frontend
