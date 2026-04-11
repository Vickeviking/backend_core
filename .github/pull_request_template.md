## Summary

- describe the main change
- describe why it was needed

## Verification

- [ ] `cargo xtask ci`
- [ ] `npm run lint` in `apps/web` when the frontend changed
- [ ] `cargo sqlx prepare --workspace --check -- --all-targets` when migrations or SQLx queries changed

List any additional checks you ran.

## Notes

- call out route changes, schema changes, or deployment changes explicitly
- note follow-up work if this PR is one phase of a larger refactor
