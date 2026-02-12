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
- cargo-udeps: cargo +nightly udeps

  # Database storage

  run a postgres instance with $./scripts/init_db.sh
  docker ps, to see if running,
  to run migrations against existing postgres container, use SKIP_DOCKER flag:
  $ SKIP_DOCKER=true ./scripts/init_db.sh

  sqlx is used as a bridge between our rust code and PostgreSQL,

  $dbeaver to open graphical view of database

  # Before commit

  cargo fmt
  cargo fmt --check
  cargo clippy
  cargo audit
  cargo sqlx prepare --workspace -- --all-targets
  cargo sqlx prepare --workspace --check

  # manual http fire

  curl -i -X POST -d 'email=thomas_wiman@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions

  # logging

  tracing spans are used, with different layers and json formatting, ElasticSearch could be used to query loggs
