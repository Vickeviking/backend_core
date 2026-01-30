Dev cycle:
Change -> Compile -> test -> run

Cargo tools:

- cargo-watch: cargo watch -x check -x test -x run
  early abortions if a prev chained cmd fails
- cargo-tarpaulin: cargo tarpaulin --ignore-tests
- clippy: cargo clippy -- -D warnings
  linting, and failing CI pipeline on warnings
- rustfmt: cargo fmt -- --check
  fails if a commit contains unformatted code
- cargo-audit: cargo audit
  checks dependency tree for reported vulnerabilities on crates.io
