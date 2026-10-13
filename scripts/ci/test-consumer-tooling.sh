#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-consumer-tooling.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else echo "Consumer tooling fixture retained: $fixture" >&2; fi
    exit "$status"
}
trap finish EXIT
mkdir -p "$fixture/scripts/dev" "$fixture/scripts/ci" "$fixture/.tools/rust/common/bin"
cp "$root/Makefile" "$fixture/"
cp -R "$root/make" "$root/ci" "$fixture/"
cp "$root/scripts/dev/"{install-rust-tools,tool-store,cargo-tool-build}.sh "$fixture/scripts/dev/"
cp "$root/scripts/ci/"{check-make-execution,verify-file-checksum,test-canister}.sh "$fixture/scripts/ci/"
# Substitute only compiler identity probes. Any Cargo effect remains forbidden;
# the real installer/parser must refuse the isolated, unprepared source cache.
mkdir -p "$fixture/compiler/bin"
(umask 077; mkdir -p "$fixture/store/cargo-cache")
export SHARED_TOOLING_STORE="$fixture/store" JOBS_CONSUMER_COMPILER="$fixture/compiler"
export CARGO_NET_OFFLINE=true
cat > "$fixture/.tools/rust/common/bin/rustc" <<'RUSTC'
#!/usr/bin/env bash
case "$*" in
    '--print sysroot') printf '%s\n' "$JOBS_CONSUMER_COMPILER" ;;
    -vV) printf 'rustc fixture\nhost: fixture-host\nrelease: 1.88.0\n' ;;
    *) printf 'unexpected rustc dispatch\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"; exit 49 ;;
esac
RUSTC
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
if [[ "$*" == -vV ]]; then printf 'cargo fixture\n'; exit 0; fi
printf '%s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
echo 'unexpected Cargo dispatch before CLI admission' >&2
exit 49
CARGO
chmod +x "$fixture/.tools/rust/common/bin/"{cargo,rustc}
cp "$fixture/.tools/rust/common/bin/"{cargo,rustc} "$fixture/compiler/bin/"
export JOBS_CONSUMER_TOOLING_EVENTS="$fixture/events"
: > "$fixture/events"

# Use the real TOML reader, shared installer and actual Jobs Makefile/wrapper.
# Both selections are deliberately unprepared; neither may fall back to a pin
# or dispatch compilation/installation after offline admission fails.
for version in 0.0.1 0.0.2; do
    cat > "$fixture/Cargo.lock" <<LOCK
version = 4

[[package]]
name = "ic-testkit"
version = "$version"
source = "registry+https://github.com/rust-lang/crates.io-index"
LOCK
    cp "$fixture/Cargo.lock" "$fixture/expected.lock"
    for target in testkit-tools-check consumer-server-check test-canister test-canister-optimized; do
        status=0
        make --no-print-directory -j2 -C "$fixture" "$target" \
            > "$fixture/$version-$target.log" 2>&1 || status=$?
        [[ "$status" == 2 ]] || exit 1
        grep -F "missing or ambiguous Cargo tool source: ic-testkit $version; run explicit setup" \
            "$fixture/$version-$target.log" > /dev/null
        [[ ! -s "$fixture/events" ]] || exit 1
        cmp "$fixture/Cargo.lock" "$fixture/expected.lock"
    done
    for mode in original optimized; do
        args=()
        [[ "$mode" != optimized ]] || args=(--optimized)
        status=0
        (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" \
            "$BASH" scripts/ci/test-canister.sh ${args[@]+"${args[@]}"}) \
            > "$fixture/$version-wrapper-$mode.log" 2>&1 || status=$?
        [[ "$status" == 1 ]] || exit 1
        grep -F "missing or ambiguous Cargo tool source: ic-testkit $version; run explicit setup" \
            "$fixture/$version-wrapper-$mode.log" > /dev/null
        [[ ! -s "$fixture/events" ]] || exit 1
        cmp "$fixture/Cargo.lock" "$fixture/expected.lock"
    done
done
echo 'Lock-selected consumer CLI refusal precedes Cargo/PocketIC effects (real caller/parser; substitute compiler identity; no installation)'

# Isolate the wrapper's Wasm-file admission after CLI admission. Substitute only
# the selected CLI and Cargo metadata; invalid inputs must create no attempt or
# start a server. The shared installer's selection contract is tested above.
mkdir -p "$fixture/evidence" "$fixture/wasm/wasm32-unknown-unknown/release"
cat > "$fixture/scripts/dev/install-rust-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
printf '%s\n' "$JOBS_CONSUMER_CLI"
INSTALLER
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
printf 'metadata\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
printf '{"target_directory":"%s"}\n' "$JOBS_CONSUMER_WASM_TARGET"
CARGO
export JOBS_CONSUMER_CLI="$fixture/server" JOBS_CONSUMER_WASM_TARGET="$fixture/wasm"
cat > "$fixture/server" <<'SERVER'
#!/usr/bin/env bash
printf 'unexpected server dispatch\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
exit 49
SERVER
chmod +x "$fixture/server"
printf 'retained Wasm input\n' > "$fixture/input"
input="$fixture/wasm/wasm32-unknown-unknown/release/jobs_test_consumer.wasm"
for kind in absent symlink directory; do
    [[ "$kind" != symlink ]] || ln -s "$fixture/input" "$input"
    [[ "$kind" != directory ]] || mkdir "$input"
    : > "$fixture/events"
    status=0
    (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" RUNNER_TEMP="$fixture/evidence" \
        "$BASH" scripts/ci/test-canister.sh) > "$fixture/wasm-$kind.log" 2>&1 || status=$?
    [[ "$status" == 1 && "$(cat "$fixture/events")" == metadata ]] || exit 1
    [[ -z "$(ls -A "$fixture/evidence")" && "$(cat "$fixture/input")" == 'retained Wasm input' ]] || exit 1
    if [[ "$kind" == symlink ]]; then rm "$input"
    elif [[ "$kind" == directory ]]; then rmdir "$input"; fi
done
echo 'Invalid Wasm input refuses before attempt/server dispatch (real wrapper; substitute metadata/CLI)'

# The real recovery wrapper must consume one frozen original even if the live
# producer is replaced during qualification. Only transforms/server effects are
# substituted; flags, paths, hashes, logs and all dispatch plumbing are real.
mkdir -p "$fixture/selected/bin" "$fixture/ic-bin" "$fixture/.tools/ic"
cp "$root/.tools/ic/"{pins.tsv,files.sha256} "$fixture/.tools/ic/"
cp "$root/.shared-tooling.snapshot" "$fixture/"
printf '{}\n' > "$fixture/selected/selection.json"
export JOBS_CONSUMER_CLI="$fixture/selected/bin/ic-testkit-server"
export JOBS_CONSUMER_IC_BIN="$fixture/ic-bin" JOBS_CONSUMER_FROZEN="$fixture/input"
export JOBS_CONSUMER_PRODUCER="$input" JOBS_CONSUMER_CAPTURED="$fixture/consumed"
cat > "$fixture/scripts/dev/install-ic-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
[[ "$*" == --check ]] || exit 49
printf '%s\n' "$JOBS_CONSUMER_IC_BIN"
INSTALLER
cat > "$fixture/.tools/rust/common/bin/git" <<'GIT'
#!/usr/bin/env bash
[[ "$*" == 'rev-parse HEAD' ]] || exit 49
printf '%040d\n' 1
GIT
cat > "$fixture/.tools/rust/common/bin/rustc" <<'RUSTC'
#!/usr/bin/env bash
[[ "$*" == '+1.88.0 --version --verbose' ]] || exit 49
echo 'rustc fixture'
RUSTC
cat > "$fixture/ic-bin/wasm-opt" <<'OPTIMIZER'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == --version ]]; then echo 'wasm-opt fixture'; exit 0; fi
[[ $# == 4 && "$3" == -o && ( "$2" == -O3 || "$2" == -Os || "$2" == -Oz ) ]] || exit 49
cmp "$1" "$JOBS_CONSUMER_FROZEN"
cat "$1" > "$4"
printf '%s\n' "$2" >> "$4"
OPTIMIZER
cat > "$JOBS_CONSUMER_CLI" <<'SERVER'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == run && "$2" == --ttl && "$3" == 900 && "$4" == --server-stdout && "$6" == --server-stderr && "$8" == -- ]] || exit 49
variant="${5%/server.stdout}"; variant="${variant##*/}"
printf 'server stdout\n' > "$5"
printf 'server stderr\n' > "$7"
shift 8
[[ "$*" == 'cargo +1.88.0 test -p jobs-canister-tests --test recovery --locked --offline -- --ignored --test-threads=1 --nocapture' ]] || exit 49
printf '%s\n' "$IC_JOBS_CONSUMER_WASM" >> "$JOBS_CONSUMER_CAPTURED/paths"
cp "$IC_JOBS_CONSUMER_WASM" "$JOBS_CONSUMER_CAPTURED/$variant.wasm"
printf 'replaced mutable producer\n' > "$JOBS_CONSUMER_PRODUCER"
echo 'substitute qualification passed'
SERVER
chmod +x "$fixture/.tools/rust/common/bin/"{git,rustc} "$fixture/ic-bin/wasm-opt" "$JOBS_CONSUMER_CLI"
for mode in original optimized; do
 mkdir "$fixture/$mode-evidence"
 mkdir -p "$JOBS_CONSUMER_CAPTURED"
 : > "$JOBS_CONSUMER_CAPTURED/paths"
 cp "$fixture/input" "$input"
 args=(); variants=(original)
 if [[ "$mode" == optimized ]]; then args=(--optimized); variants+=(O3 Os Oz); fi
 (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" RUNNER_TEMP="$fixture/$mode-evidence" \
  "$BASH" scripts/ci/test-canister.sh ${args[@]+"${args[@]}"}) > "$fixture/$mode-wrapper.log" 2>&1
 evidence=("$fixture/$mode-evidence/"*)
 [[ ${#evidence[@]} == 1 && "$(head -n 1 "$JOBS_CONSUMER_CAPTURED/paths")" == "${evidence[0]}/jobs_test_consumer.wasm" ]] || exit 1
 cmp "$fixture/input" "$JOBS_CONSUMER_CAPTURED/original.wasm"
 for variant in "${variants[@]}"; do
  if [[ "$variant" != original ]]; then
   { cat "$fixture/input"; printf '%s\n' "-$variant"; } > "$fixture/expected.wasm"
   cmp "$fixture/expected.wasm" "$JOBS_CONSUMER_CAPTURED/$variant.wasm"
  fi
  actual="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$JOBS_CONSUMER_CAPTURED/$variant.wasm")"
  [[ "$(cat "${evidence[0]}/$variant/consumer-wasm.sha256")" == "$actual" && -s "${evidence[0]}/$variant/test.log" && -s "${evidence[0]}/$variant/server.stdout" && -s "${evidence[0]}/$variant/server.stderr" ]] || exit 1
 done
 [[ "$(cat "$input")" == 'replaced mutable producer' ]] || exit 1
done
rm "$fixture/.tools/rust/common/bin/git"
echo 'Recovery consumes one frozen original with preserved variants/hashes/logs after producer replacement (substitute optimizer/server; no live gate)'

# Run Jobs' actual CI route only through its first admission stage. Substitute
# tool effects and mutate a selected input there; the logger must block all
# subsequent product checks without compilation, installation or server effects.
cp "$root/"{Cargo.toml,.shared-tooling.snapshot,rust-toolchain.toml} "$fixture/"
for member in crates/ic-jobs apps/job-consumer crates/jobs-canister-tests; do
    mkdir -p "$fixture/$member"
    cp "$root/$member/Cargo.toml" "$fixture/$member/"
done
cp "$root/scripts/ci/"{run-validation-targets,check-format-tools}.sh "$fixture/scripts/ci/"
cat > "$fixture/scripts/ci/verify-shared-tooling-snapshot.sh" <<'VERIFY'
#!/usr/bin/env bash
exit 0
VERIFY
for toolset in host ic rust; do
    cat > "$fixture/scripts/dev/install-$toolset-tools.sh" <<'ADMISSION'
#!/usr/bin/env bash
set -euo pipefail
[[ "${@: -1}" == --check ]] || exit 49
case "${0##*/}" in
    install-host-tools.sh)
        printf 'admission\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
        if [[ "$JOBS_CONSUMER_INPUT_DRIFT" == changed ]]; then
            printf '\n# concurrent selected graph edit\n' >> "$PWD/Cargo.lock"
        else
            rm "$PWD/Cargo.lock"
        fi ;;
    install-rust-tools.sh)
        if [[ "$1" == --package ]]; then
            printf '%s/.tools/rust/common/bin/fixture-version-editor\n' "$PWD"
        fi ;;
esac
ADMISSION
done
# shellcheck disable=SC1091
. "$root/ci/release-tools.env"
# shellcheck disable=SC1091
. "$root/ci/tool-versions.env"
export JOBS_CONSUMER_RELEASE_VERSION="$IC_JOBS_CARGO_EDIT_VERSION"
export JOBS_CONSUMER_SORT_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION"
cat > "$fixture/.tools/rust/common/bin/fixture-version-editor" <<'EDITOR'
#!/usr/bin/env bash
[[ "$*" == 'set-version --version' ]] || exit 49
printf 'cargo-edit-set-version %s\n' "$JOBS_CONSUMER_RELEASE_VERSION"
EDITOR
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
case "$*" in
    'sort --version') printf 'cargo-sort %s\n' "$JOBS_CONSUMER_SORT_VERSION" ;;
    'fmt --version') exit 0 ;;
    *) printf 'unexpected Cargo effect\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"; exit 49 ;;
esac
CARGO
chmod +x "$fixture/.tools/rust/common/bin/fixture-version-editor"
for drift in changed missing; do
    cp "$root/Cargo.lock" "$fixture/Cargo.lock"
    : > "$fixture/events"
    status=0
    JOBS_CONSUMER_INPUT_DRIFT="$drift" VALIDATION_LOG_DIR="$fixture/$drift-logs" \
        make --no-print-directory -j2 -C "$fixture" ci \
        > "$fixture/ci-$drift.log" 2>&1 || status=$?
    [[ "$status" == 2 && "$(cat "$fixture/events")" == admission ]] || exit 1
    logs=("$fixture/$drift-logs"/validation.*)
    [[ ${#logs[@]} == 1 ]] || exit 1
    [[ "$(wc -l < "${logs[0]}/timings.tsv" | tr -d ' ')" == 2 ]] || exit 1
    grep -F $'validation-tools-check\tFAIL\t' "${logs[0]}/timings.tsv" > /dev/null
    [[ -f "${logs[0]}/inputs.json" && -f "${logs[0]}/0-after-inputs.json" ]] || exit 1
    if [[ "$drift" == changed ]]; then
        grep -F 'validation input changed: Cargo.lock' "$fixture/ci-$drift.log" > /dev/null
    else
        grep -F 'cannot observe validation input Cargo.lock' "$fixture/ci-$drift.log" > /dev/null
    fi
done
echo 'Changed/missing graph inputs stop Jobs CI after admission with retained checkpoints (substitute tool effects; no product validation)'
fixture_complete=true
