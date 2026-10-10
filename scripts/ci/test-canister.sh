#!/usr/bin/env bash
set -euo pipefail

# Caller explicitly selects this live gate. The canonical Cargo-tool checker and
# Testkit own executable selection and PocketIC admission/lifecycle respectively.
# shellcheck disable=SC1091
. ci/testkit-tools.env
cli="$(bash scripts/dev/install-rust-tools.sh --package ic-testkit \
    --version "$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release --check)"
target="$(cargo +1.88.0 metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
export IC_JOBS_CONSUMER_WASM="$target/wasm32-unknown-unknown/release/jobs_test_consumer.wasm"
[[ -f "$IC_JOBS_CONSUMER_WASM" && ! -L "$IC_JOBS_CONSUMER_WASM" ]]
evidence="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/jobs-recovery.XXXXXX")"
printf 'Live recovery evidence: %s\n' "$evidence"
cp "$IC_JOBS_CONSUMER_WASM" "$evidence/jobs_test_consumer.wasm"
export IC_JOBS_CONSUMER_WASM="$evidence/jobs_test_consumer.wasm"
bash scripts/ci/verify-file-checksum.sh --print sha256 "$evidence/jobs_test_consumer.wasm" \
    > "$evidence/consumer-wasm.sha256"
# Testkit's run command checks the prepared server bundle; it never downloads.
"$cli" run --ttl 900 --server-stdout "$evidence/server.stdout" --server-stderr "$evidence/server.stderr" \
    -- cargo +1.88.0 test -p jobs-canister-tests \
    --test recovery --locked --offline -- --ignored --test-threads=1 --nocapture
