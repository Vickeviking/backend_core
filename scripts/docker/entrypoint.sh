#!/usr/bin/env sh
set -eu

APP_RUNTIME="${APP_RUNTIME:-api}"

case "${APP_RUNTIME}" in
  api|worker)
    exec "/app/${APP_RUNTIME}" "$@"
    ;;
  *)
    echo "Unsupported APP_RUNTIME: ${APP_RUNTIME}" >&2
    echo "Expected one of: api, worker" >&2
    exit 1
    ;;
esac
