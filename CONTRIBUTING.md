# Contributing

## Purpose

This repository is being refactored in small, reviewable steps. Keep changes scoped, behavior-preserving by default, and easy to verify.

## Branch Naming

Use short descriptive branches with a clear prefix when possible.

Examples:

- `feat/<topic>`
- `fix/<topic>`
- `refactor/<topic>`
- `chore/<topic>`
- `docs/<topic>`
- `test/<topic>`
- `ci/<topic>`
- `build/<topic>`

## Commit Prefixes

Use one of these prefixes at the start of the commit subject:

- `feat`
- `fix`
- `refactor`
- `chore`
- `docs`
- `test`
- `ci`
- `build`

Examples:

- `refactor: split api and worker binaries`
- `docs: replace architecture overview with folder index`
- `test: reorganize integration support helpers`

## Pull Request Expectations

- explain the problem and the chosen approach
- call out any behavior changes explicitly
- list the verification commands you ran
- keep unrelated cleanup out of the PR unless it is necessary for the change

## Verification

At minimum, run the checks that match the scope of your change. Today the primary local CI entrypoint is:

- `cargo xtask ci`

Additional checks that are often relevant:

- `npm run lint` in `apps/web`
- `cargo sqlx prepare --workspace --check -- --all-targets` when migrations or SQLx queries change

During the refactor, phase gates should also include a short summary and the verification results before moving on.
