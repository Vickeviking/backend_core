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

  ## The best way to use

  cutting out sqlx logs
  RUST_LOG="sqlx=error,info" TEST_LOG=true cargo t subscriber_fails_if_there_is_a_fatal_database_error | bunyan

  ## observe test logs

  TEST_LOG=true cargo test health_check_works | bunyan

  ## lnav with clean json logs, cumbersome to use, but perhaps usefull for a bigger log?

  cargo t subscriber_fails_if_there_is_a_fatal_database_error \
   | jq -Rrc 'fromjson? | select(.)' > /tmp/test.jsonl

  lnav /tmp/test.jsonl

  ## Error handling

  ### thiserror

  User might want to handle the error and match on it, build error enums with
  thiserror for less boilerplate,

  ### anyhow

  If end-user dont want to match on error and just want some context, use anyhow to enrich error
  with context.

  A mix can be used if we want a few error types to match on, and an opaque other error with attached context.

  Errors should be logged when handeled, if propagating upwards with '?' logging the error will cause duplicates
  and a harder time debugging and reading logs. It can if it makes sence add more context to it tough.

  ### Docker
  - build:
    docker build --tag backend_core --file Dockerfile .
  - run:
    docker run -p 8000:8000 backend_core
