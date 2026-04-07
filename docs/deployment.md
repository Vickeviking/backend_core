# Deployment

## Purpose

This document describes how the service is currently expected to be deployed and what operational assumptions exist today.

## Current Deployment Shape

The repository contains a DigitalOcean app specification in `spec.yaml` and a production-oriented `Dockerfile`.

The deployment model assumes:

- one web application process
- PostgreSQL
- Redis managed separately
- environment-specific configuration through environment variables

## Production Configuration

Production behavior is enabled with:

`APP_ENVIRONMENT=production`

That causes the application to load `configuration/production.yaml` on top of the base configuration.

## Current Operational Dependencies

The service depends on:

- a PostgreSQL database
- a Redis instance for sessions
- an email provider integration

## Migrations

Database migrations must be run as part of deployment. That should be treated as a required operational step, not an optional manual action.

## Recommended Improvement

This document should eventually include:

- deployment steps for each environment
- migration policy
- rollback expectations
- secret management approach
- health check and observability expectations
- ownership and incident contact guidance

For now, this file is intentionally short and should be kept aligned with actual deployment practice.
