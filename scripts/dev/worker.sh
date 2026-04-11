#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"

main() {
  cd "${REPO_ROOT}"

  export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"

  echo "Starting worker cargo watch."
  exec cargo watch -c "$@" -x "run --bin worker"
}

main "$@"
