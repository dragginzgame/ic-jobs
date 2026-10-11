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
fixture_complete=true
