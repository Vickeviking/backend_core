# Domain

## Purpose

This service currently models a simple newsletter workflow:

- a visitor subscribes
- the subscription must be confirmed
- an admin logs in
- the admin publishes a newsletter issue to confirmed subscribers

Even though the product is still small, the domain should be described explicitly. Clear language now will reduce refactor cost later.

## Core Concepts

### Subscriber

A subscriber represents a person who has provided an email address and a display name to receive newsletter issues.

### Subscription Status

A subscription can be in one of at least two meaningful states:

- `pending_confirmation`
- `confirmed`

This status is already reflected in the database and should become an explicit domain concern instead of remaining only a database string.

### Subscription Confirmation Token

A confirmation token links a pending subscription to a confirmation action. It is part of the confirmation flow and should be treated as a domain concept, not just a random string stored in a table.

### Admin User

An admin user can authenticate and trigger privileged actions such as newsletter publishing or password changes.

### Newsletter Issue

A newsletter issue is a message prepared for delivery to confirmed subscribers. Today it contains a title and two body formats:

- HTML content
- plain text content

If the product grows, this concept may later include draft state, templates, scheduling, audience targeting, and delivery status.

## Current Domain Strengths

The codebase already includes a good start through validated value objects:

- subscriber email
- subscriber name

These types are important because they prevent invalid values from spreading across the system.

## Recommended Domain Improvements

The next step is to turn more business concepts into explicit domain types. Likely candidates are:

- `SubscriptionStatus`
- `SubscriptionToken`
- `AdminUserId`
- `NewsletterIssue`
- `PlainTextBody`
- `HtmlBody`

This does not need to become over-engineered. The goal is not to wrap every primitive in a new type. The goal is to make important business rules visible and enforceable.

## Domain Rule Placement

Rules that define what is valid or allowed in the business should live close to the domain.

Examples:

- valid subscriber names and emails
- allowed subscription status transitions
- whether only confirmed subscribers can receive newsletters
- whether password change requires valid current credentials

Rules that only describe HTTP behavior or infrastructure constraints should not be placed in the domain.
