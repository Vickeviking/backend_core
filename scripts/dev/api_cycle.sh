#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"
TEST_LOG_FILE="/tmp/backend_core-test-log.jsonl"
TEST_OUTPUT_FILE=""

export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"

cleanup() {
  if [[ -n "${TEST_OUTPUT_FILE}" ]]; then
    rm -f "${TEST_OUTPUT_FILE}"
  fi
}

extract_failed_tests() {
  local output_file="$1"

  sed -n 's/^test \([^[:space:]]\+\) \.\.\. FAILED$/\1/p' "${output_file}" | awk '!seen[$0]++'
}

run_bunyan_view() {
  local test_name="$1"

  echo >&2
  echo "Re-running ${test_name} with bunyan output." >&2
  set +e
  RUST_LOG="${RUST_LOG:-sqlx=error,info}" TEST_LOG=true \
    cargo test "${test_name}" -- --exact --nocapture | bunyan
  set -e
}

run_lnav_view() {
  local test_name="$1"

  echo >&2
  echo "Re-running ${test_name} and writing JSON logs to ${TEST_LOG_FILE}." >&2
  set +e
  TEST_LOG=true cargo test "${test_name}" -- --exact --nocapture \
    | jq -Rrc 'fromjson? | select(.)' > "${TEST_LOG_FILE}"
  set -e

  lnav "${TEST_LOG_FILE}"
}

inspect_failed_test() {
  local test_name="$1"
  local previous_ps3="${PS3:-}"

  while true; do
    echo >&2
    echo "Inspect ${test_name}:" >&2
    PS3="Choose an inspection mode: "
    select action in \
      "bunyan" \
      "lnav" \
      "back to failed tests" \
      "continue watching"; do
      case "${action:-}" in
        bunyan)
          run_bunyan_view "${test_name}"
          break
          ;;
        lnav)
          run_lnav_view "${test_name}"
          break
          ;;
        "back to failed tests")
          PS3="${previous_ps3}"
          return 0
          ;;
        "continue watching")
          PS3="${previous_ps3}"
          return 1
          ;;
        *)
          echo "Pick one of the numbered options." >&2
          ;;
      esac
    done
  done
}

prompt_for_failed_tests() {
  local failed_tests=("$@")
  local previous_ps3="${PS3:-}"

  while true; do
    echo >&2
    echo "Failed tests:" >&2
    PS3="Choose a failed test: "
    select test_name in "${failed_tests[@]}" "continue watching"; do
      case "${test_name:-}" in
        "continue watching")
          PS3="${previous_ps3}"
          return 0
          ;;
        "")
          echo "Pick one of the numbered options." >&2
          ;;
        *)
          if ! inspect_failed_test "${test_name}"; then
            PS3="${previous_ps3}"
            return 0
          fi
          break
          ;;
      esac
    done
  done
}

main() {
  local cargo_test_status
  local failed_tests=()

  cd "${REPO_ROOT}"

  TEST_OUTPUT_FILE="$(mktemp)"
  trap cleanup EXIT

  set +e
  cargo test 2>&1 | tee "${TEST_OUTPUT_FILE}"
  cargo_test_status=${PIPESTATUS[0]}
  set -e

  if [[ ${cargo_test_status} -eq 0 ]]; then
    cleanup
    trap - EXIT
    exec cargo run --bin api
  fi

  mapfile -t failed_tests < <(extract_failed_tests "${TEST_OUTPUT_FILE}")

  if [[ ${#failed_tests[@]} -eq 0 ]]; then
    echo >&2
    echo "Tests failed, but no individual failing test names were detected." >&2
    echo "Review the output above, then press Enter to continue watching." >&2
    read -r
    return 1
  fi

  prompt_for_failed_tests "${failed_tests[@]}"
  return 1
}

main "$@"
