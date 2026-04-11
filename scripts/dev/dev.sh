#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd -- "${SCRIPTS_ROOT}/.." && pwd)"

POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_HOST="${POSTGRES_HOST:-127.0.0.1}"
REDIS_PORT="${REDIS_PORT:-6379}"
REDIS_HOST="${REDIS_HOST:-127.0.0.1}"

bootstrap_postgres() {
  local log_file
  local status

  log_file="$(mktemp)"

  set +e
  "${SCRIPTS_ROOT}/init_db.sh" 2>&1 | tee "${log_file}"
  status=${PIPESTATUS[0]}
  set -e

  if [[ ${status} -eq 0 ]]; then
    rm -f "${log_file}"
    return 0
  fi

  if grep -Eiq \
    'port is already allocated|address already in use|bind for 0\.0\.0\.0:[0-9]+ failed|driver failed programming external connectivity|container name .* is already in use|Conflict\.' \
    "${log_file}"; then
    echo
    echo "Postgres container looks to be running already. Re-running migrations with SKIP_DOCKER=true."
    rm -f "${log_file}"
    SKIP_DOCKER=true "${SCRIPTS_ROOT}/init_db.sh"
    return 0
  fi

  rm -f "${log_file}"
  return "${status}"
}

redis_is_reachable() {
  nc -z "${REDIS_HOST}" "${REDIS_PORT}" >/dev/null 2>&1
}

bootstrap_redis() {
  if redis_is_reachable; then
    echo "Redis is already reachable on ${REDIS_HOST}:${REDIS_PORT}, skipping init."
    return 0
  fi

  "${SCRIPTS_ROOT}/init_redis.sh"
}

main() {
  cd "${REPO_ROOT}"

  bootstrap_postgres
  bootstrap_redis

  export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"

  echo
  echo "Starting API cargo watch. Failed tests will open an inspection menu before the watch loop idles."
  exec cargo watch -c "$@" -x check -s "./scripts/dev/api_cycle.sh"
}

main "$@"
