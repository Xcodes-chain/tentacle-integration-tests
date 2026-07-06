#!/usr/bin/env bash
set -euo pipefail

case_name="${1:-}"
if [[ -z "$case_name" ]]; then
  echo "usage: $0 <cargo-test-filter>" >&2
  exit 2
fi

cargo test --features ws,quic "$case_name" -- --ignored --nocapture
