#!/usr/bin/env bash
set -euo pipefail

[[ $# == 0 || ( $# == 1 && "$1" == --optimized ) ]] || {
    echo 'usage: test-canister.sh [--optimized]' >&2; exit 2;
}
complete=false
status=0
trap 'status=$?; [[ "$complete" == true || "$status" != 0 ]] || status=1; exit "$status"' EXIT

# Caller explicitly selects this live gate. The canonical Cargo-tool checker and
# Testkit own executable selection and PocketIC admission/lifecycle respectively.
# shellcheck disable=SC1091
. ci/testkit-tools.env
cli="$(bash scripts/dev/install-rust-tools.sh --package ic-testkit \
    --version "$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release --check)"
target="$(cargo +1.88.0 metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
input="$target/wasm32-unknown-unknown/release/jobs_test_consumer.wasm"
[[ -f "$input" && ! -L "$input" ]]
variants=(original)
if [[ $# == 1 ]]; then
    bin="$(bash scripts/dev/install-ic-tools.sh --check)"
    variants+=(O3 Os Oz)
fi
evidence="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/jobs-recovery.XXXXXX")"
printf 'Live recovery evidence: %s\n' "$evidence"
cp "$input" "$evidence/jobs_test_consumer.wasm"
cp Cargo.lock .shared-tooling.snapshot ci/testkit-tools.env "$evidence/"
cp "${cli%/bin/*}/selection.json" "$evidence/testkit-selection.json"
{
    git rev-parse HEAD
    uname -sm
    rustc +1.88.0 --version --verbose
    bash scripts/ci/verify-file-checksum.sh --print sha256 scripts/ci/test-canister.sh
} > "$evidence/source.txt"
if [[ $# == 1 ]]; then
    cp .tools/ic/pins.tsv .tools/ic/files.sha256 "$evidence/"
    "$bin/wasm-opt" --version > "$evidence/optimizer.txt"
    bash scripts/ci/verify-file-checksum.sh --print sha256 "$bin/wasm-opt" >> "$evidence/optimizer.txt"
fi
for variant in "${variants[@]}"; do
    mkdir "$evidence/$variant"
    export IC_JOBS_CONSUMER_WASM="$evidence/$variant/jobs_test_consumer.wasm"
    if [[ "$variant" == original ]]; then
        cp "$evidence/jobs_test_consumer.wasm" "$IC_JOBS_CONSUMER_WASM"
    else
        # Respect the input's declared features; do not enable unrelated Wasm
        # proposals or change the normal product build.
        "$bin/wasm-opt" "$evidence/jobs_test_consumer.wasm" "-$variant" \
            -o "$IC_JOBS_CONSUMER_WASM" \
            > "$evidence/$variant/optimizer.log" 2>&1
    fi
    bash scripts/ci/verify-file-checksum.sh --print sha256 "$IC_JOBS_CONSUMER_WASM" \
        > "$evidence/$variant/consumer-wasm.sha256"
    printf 'Qualifying %s: %s\n' "$variant" "$(cat "$evidence/$variant/consumer-wasm.sha256")"
    # Testkit's run command checks the prepared server bundle; it never downloads.
    "$cli" run --ttl 900 --server-stdout "$evidence/$variant/server.stdout" \
        --server-stderr "$evidence/$variant/server.stderr" \
        -- cargo +1.88.0 test -p jobs-canister-tests \
        --test recovery --locked --offline -- --ignored --test-threads=1 --nocapture \
        2>&1 | tee "$evidence/$variant/test.log"
done
complete=true
