# Domain

## Purpose

The current codebase models a simple newsletter workflow:

- a visitor subscribes
- the subscription must be confirmed
- an admin logs in
- the admin publishes a newsletter issue to confirmed subscribers

Even though the product is still small, the domain should be described explicitly. The naming stays newsletter-focused for now, while the architecture is being prepared for broader product use later.

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

A newsletter issue is a message prepared for delivery to confirmed subscribers. Today it contains:

- a title
- HTML content
- plain text content

If the product grows, this concept may later include draft state, templates, scheduling, audience targeting, and delivery status.

## Current Domain Strengths

The codebase already includes a good start through validated value objects:

- subscriber email
- subscriber name

These types are important because they prevent invalid values from spreading across the system.

## Recommended Domain Improvements

Likely next candidates for explicit domain types are:

- `SubscriptionStatus`
- `SubscriptionToken`
- `AdminUserId`
- `NewsletterIssue`
- `PlainTextBody`
- `HtmlBody`

The goal is not to wrap every primitive in a new type. The goal is to make important business rules visible and enforceable.

## Product-Neutral Direction

The repository structure should become product-neutral before the domain language does.

That means:

- keep `subscription` and `newsletter` names where they already reflect the current behavior
- avoid hard-coding those concepts into cross-cutting architecture decisions
- move toward feature boundaries that can later host additional product capabilities without another structural rewrite

## Domain Rule Placement

Rules that define what is valid or allowed in the business should live close to the domain.

Examples:

- valid subscriber names and emails
- allowed subscription status transitions
- whether only confirmed subscribers can receive newsletters
- whether password change requires valid current credentials

Rules that only describe HTTP behavior or infrastructure constraints should not be placed in the domain.
