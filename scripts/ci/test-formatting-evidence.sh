#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

# Execute the selected wrapper and collector; the hosted upload is not invoked.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-formatting-evidence.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf "$fixture";
    else echo "Formatting evidence fixture retained: $fixture" >&2; fi
}
trap finish EXIT
mkdir "$fixture/temp" "$fixture/repository" "$fixture/unpacked"
yq -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
yq -o json '.' "$root/.github/actions/retain-failure-evidence/action.yml" > "$fixture/collector.json"
jq -e '
    .jobs.validate.steps as $steps |
    [$steps | to_entries[] | select(.value.run == "make ci")] as $gates |
    [$steps | to_entries[] | select(.value.uses == "./.github/actions/retain-failure-evidence")] as $collectors |
    ($gates | length) == 1 and ($collectors | length) == 1 and
    $gates[0].value["continue-on-error"] != true and
    $collectors[0].key > $gates[0].key and $collectors[0].value.if == "failure()"
' "$fixture/workflow.json" > /dev/null

# Complete stdout and stderr survive the compact two-line presentation, along
# with the wrapped command's original status.
status=0
RUNNER_TEMP="$fixture/temp" bash "$root/scripts/ci/run-formatting.sh" --check \
    bash -ec 'echo retained-formatter-diff; echo formatter-error >&2; exit 23' \
    > "$fixture/output" 2>&1 || status=$?
[[ "$status" == 23 ]]
[[ "$(wc -l < "$fixture/output" | tr -d ' ')" == 2 ]]
grep -Fx 'Checking formatting... FAILED (exit 23)' "$fixture/output" > /dev/null
logs=("$fixture/temp"/formatting.*)
[[ ${#logs[@]} == 1 && -f "${logs[0]}" ]]
grep -Fx retained-formatter-diff "${logs[0]}" > /dev/null
grep -Fx formatter-error "${logs[0]}" > /dev/null

# Exercise Jobs' actual Makefile and selected includes, substituting only Cargo
# in this disposable checkout. A sorter failure must stop the later rustfmt call.
consumer="$fixture/jobs"
mkdir -p "$consumer/scripts/ci" "$consumer/.tools/rust/bin"
cp "$root/Makefile" "$consumer/"
cp -R "$root/make" "$root/ci" "$consumer/"
cp "$root/scripts/ci/"{check-format-tools,check-make-execution,run-formatting}.sh "$consumer/scripts/ci/"
cat > "$consumer/.tools/rust/bin/formatting-fixture-cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
[[ "$CARGO_NET_OFFLINE" == true && "$RUSTUP_AUTO_INSTALL" == 0 ]]
case "$*" in
    'sort --version') echo "cargo-sort ${FORMAT_EVIDENCE_VERSION:?}"; exit 0 ;;
    'fmt --version') exit 0 ;;
esac
printf '%s\n' "$*" >> "$FORMAT_EVIDENCE_EVENTS"
case "$*" in
    'sort --workspace --check')
        echo retained-sorter-diff
        if [[ "${FORMAT_EVIDENCE_FAIL:-}" == sort ]]; then
            echo sorter-error >&2; exit 23
        fi ;;
    'fmt --all -- --check') echo successful-rustfmt-detail ;;
    *) exit 45 ;;
esac
CARGO
chmod +x "$consumer/.tools/rust/bin/formatting-fixture-cargo"
# The pin catalog is selected from the resolved consumer root at runtime.
# shellcheck disable=SC1091
. "$root/ci/tool-versions.env"
export FORMAT_EVIDENCE_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION"
export FORMAT_EVIDENCE_EVENTS="$fixture/events"
: > "$FORMAT_EVIDENCE_EVENTS"
status=0
RUNNER_TEMP="$fixture/temp" FORMAT_EVIDENCE_FAIL=sort \
    make --no-print-directory -C "$consumer" fmt-check FORMAT_CARGO=formatting-fixture-cargo \
    > "$fixture/make-failure" 2>&1 || status=$?
[[ "$status" == 2 ]]
printf 'sort --workspace --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
grep -Fx 'Checking formatting... FAILED (exit 23)' "$fixture/make-failure" > /dev/null
logs=("$fixture/temp"/formatting.*)
[[ ${#logs[@]} == 2 ]]
grep -Flx sorter-error "${logs[@]}" > "$fixture/sorter-logs"
[[ "$(wc -l < "$fixture/sorter-logs" | tr -d ' ')" == 1 ]]
sorter_log="$(cat "$fixture/sorter-logs")"
grep -Fx retained-sorter-diff "$sorter_log" > /dev/null
: > "$FORMAT_EVIDENCE_EVENTS"
RUNNER_TEMP="$fixture/temp" make --no-print-directory -C "$consumer" \
    fmt-check FORMAT_CARGO=formatting-fixture-cargo > "$fixture/make-success"
printf 'sort --workspace --check\nfmt --all -- --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
printf 'Checking formatting... ok\n' > "$fixture/expected"
cmp "$fixture/expected" "$fixture/make-success"

jq -er '.runs.steps[] | select(.id == "archive") | .run' "$fixture/collector.json" > "$fixture/collect.sh"
EVIDENCE_TEMP_ROOT="$fixture/temp" EVIDENCE_REPOSITORY_ROOT="$fixture/repository" \
    EVIDENCE_ACTION_ROOT="$root/.github/actions/retain-failure-evidence" \
    RUNNER_TEMP="$fixture/temp" GITHUB_OUTPUT="$fixture/archive-output" \
    bash "$fixture/collect.sh" > "$fixture/collection.log" 2>&1
archive="$(sed -n 's/^path=//p' "$fixture/archive-output")"
[[ -f "$archive" ]]
tar -xzf "$archive" -C "$fixture/unpacked"
for log in "${logs[@]}"; do cmp "$log" "$fixture/unpacked/${log##*/}"; done
# Successful formatting reports one line and removes only its own temporary log.
RUNNER_TEMP="$fixture/temp" bash "$root/scripts/ci/run-formatting.sh" --write \
    bash -c 'echo successful-formatter-detail' > "$fixture/success"
printf 'Formatting... ok\n' > "$fixture/expected"
cmp "$fixture/expected" "$fixture/success"
remaining=("$fixture/temp"/formatting.*)
[[ ${#remaining[@]} == 2 ]]
for log in "${logs[@]}"; do [[ -f "$log" ]]; done
echo 'Formatting failure propagation and local archive retention passed (no hosted upload)'
