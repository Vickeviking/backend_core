# Deployment

## Purpose

This document describes the current production deployment shape and the assumptions behind it.

## Current Deployment Shape

The repository contains:

- a DigitalOcean app specification in `spec.yaml`
- a production-oriented `Dockerfile`
- environment-specific configuration through environment variables

The production image contains two runtimes:

- `api`
- `worker`

`spec.yaml` models these as two DigitalOcean App Platform components built from the same repository and Dockerfile:

- one routable API service
- one background worker

The selected runtime is controlled by `APP_RUNTIME`.

The deployment also depends on:

- PostgreSQL
- Redis managed separately from `spec.yaml`
- an email provider integration

## Production Configuration

Production behavior is enabled with:

`APP_ENVIRONMENT=production`

That causes the application to load `configuration/production.yaml` on top of the base configuration.

## Migrations

Database migrations must be run as part of deployment. That should be treated as a required operational step, not an optional manual action.

The current repository guidance is:

1. update the app definition when deployment configuration changes
2. push the relevant branch to GitHub
3. run `sqlx migrate run` against the target database before or during rollout

## DigitalOcean Notes

The current `spec.yaml` targets region `fra1` and defines:

- an `api` service with the `/health_check` probe and public route
- a `worker` component with no public route
- a managed PostgreSQL database

Redis is not provisioned from the spec file today. It must be created and wired separately.

Additional runtime secrets are still expected to be configured for both components outside the checked-in spec:

- `APP_APPLICATION__HMAC_SECRET`
- `APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN`
- `APP_REDIS_URI`

## Web App

`apps/web` is part of the monorepo, but it is not deployed from `spec.yaml` yet. The current production spec only covers the backend API and worker components.
