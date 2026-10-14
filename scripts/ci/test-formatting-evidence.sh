#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

# Execute the selected wrapper and collector; the hosted upload is not invoked.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-formatting-evidence.XXXXXX")"
fixture="$(cd -P "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
fixture_complete=false
finish() {
    fixture_finish "$?" "$fixture" "Formatting evidence fixture retained"
}
trap finish EXIT
mkdir "$fixture/temp" "$fixture/repository" "$fixture/unpacked"
(umask 077; mkdir "$fixture/store")
yq -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
yq -o json '.' "$root/.github/actions/retain-failure-evidence/action.yml" > "$fixture/collector.json"
jq -e '
    .jobs.validate.steps as $steps |
    [$steps | to_entries[] | select(.value.name == "Validate the selected graph")] as $gates |
    [$steps | to_entries[] | select(.value.uses == "./.github/actions/retain-failure-evidence")] as $collectors |
    ($gates | length) == 1 and ($collectors | length) == 1 and
    $gates[0].value["continue-on-error"] != true and
    $collectors[0].key > $gates[0].key and $collectors[0].value.if == "failure()"
' "$fixture/workflow.json" > /dev/null

# Execute the actual workflow gate with a failed fixture. Its temporary root
# must be one the actual collector archives, and the child status must survive.
jq -er '.jobs.validate.steps[] | select(.name == "Validate the selected graph") | .run' \
    "$fixture/workflow.json" > "$fixture/gate.sh"
temporary_selection="$(jq -er '.jobs.validate.steps[] | select(.name == "Validate the selected graph") | .env.TMPDIR' "$fixture/workflow.json")"
# Substitute the literal GitHub expression, not a shell variable.
# shellcheck disable=SC2016
gate_temp="${temporary_selection//'${{ runner.temp }}'/$fixture/temp}"
mkdir "$fixture/bin"
cat > "$fixture/bin/make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == ci ]] || exit 45
attempt="$(mktemp -d "$TMPDIR/jobs-release-tooling.XXXXXX")"
printf 'retained-inner-fixture-failure\n' > "$attempt/failure.log"
exit 37
MAKE
chmod +x "$fixture/bin/make"
status=0
PATH="$fixture/bin:$PATH" TMPDIR="$gate_temp" bash "$fixture/gate.sh" \
    > "$fixture/gate-output" 2>&1 || status=$?
[[ "$status" == 37 ]] || exit 1
attempts=("$gate_temp/"*)
[[ ${#attempts[@]} == 1 && -f "${attempts[0]}/failure.log" ]] || exit 1

# Shared test-make-format.sh owns standalone wrapper presentation/status proof.
# Jobs retains actual workflow, Make and collector integration below.

# Exercise Jobs' actual Makefile and selected includes, substituting only Cargo
# in this disposable checkout. A sorter failure must stop the later rustfmt call.
consumer="$fixture/jobs"
mkdir -p "$consumer/scripts/ci" "$consumer/.tools/rust/common/bin"
cp "$root/Makefile" "$consumer/"
cp -R "$root/make" "$root/ci" "$consumer/"
cp "$root/scripts/ci/"{check-format-tools,check-make-execution,run-formatting}.sh "$consumer/scripts/ci/"
cat > "$consumer/.tools/rust/common/bin/formatting-fixture-cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
[[ "$CARGO_NET_OFFLINE" == true && "$RUSTUP_AUTO_INSTALL" == 0 ]] || exit 1
[[ "${MAKEFLAGS:-}" =~ --jobserver-(auth|fds)=([0-9]+),([0-9]+) ]] || exit 1
reader="${BASH_REMATCH[2]}"; writer="${BASH_REMATCH[3]}"
: <&"$reader"
: >&"$writer"
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
chmod +x "$consumer/.tools/rust/common/bin/formatting-fixture-cargo"
# The pin catalog is selected from the resolved consumer root at runtime.
# shellcheck disable=SC1091
. "$root/ci/tool-versions.env"
export FORMAT_EVIDENCE_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION"
export FORMAT_EVIDENCE_EVENTS="$fixture/events"
: > "$FORMAT_EVIDENCE_EVENTS"
status=0
parallel=(-j2)
if make --help | grep -q -- --jobserver-style; then parallel+=(--jobserver-style=pipe); fi
RUNNER_TEMP="$fixture/temp" FORMAT_EVIDENCE_FAIL=sort \
    make --no-print-directory "${parallel[@]}" -C "$consumer" fmt-check FORMAT_CARGO=formatting-fixture-cargo \
    > "$fixture/make-failure" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
printf 'sort --workspace --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
grep -Fx 'Checking formatting... FAILED (exit 23)' "$fixture/make-failure" > /dev/null
logs=("$fixture/temp"/formatting.*)
[[ ${#logs[@]} == 1 ]] || exit 1
grep -Flx sorter-error "${logs[@]}" > "$fixture/sorter-logs"
[[ "$(wc -l < "$fixture/sorter-logs" | tr -d ' ')" == 1 ]] || exit 1
sorter_log="$(cat "$fixture/sorter-logs")"
grep -Fx retained-sorter-diff "$sorter_log" > /dev/null
: > "$FORMAT_EVIDENCE_EVENTS"
RUNNER_TEMP="$fixture/temp" make --no-print-directory "${parallel[@]}" -C "$consumer" \
    fmt-check FORMAT_CARGO=formatting-fixture-cargo > "$fixture/make-success"
printf 'sort --workspace --check\nfmt --all -- --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
printf 'Checking formatting... ok\n' > "$fixture/expected"
cmp "$fixture/expected" "$fixture/make-success"

jq -er '.runs.steps[] | select(.id == "archive") | .run' "$fixture/collector.json" > "$fixture/collect.sh"
SHARED_TOOLING_STORE="$fixture/store" \
    EVIDENCE_TEMP_ROOT="$fixture/temp" EVIDENCE_REPOSITORY_ROOT="$fixture/repository" \
    EVIDENCE_ACTION_ROOT="$root/.github/actions/retain-failure-evidence" \
    RUNNER_TEMP="$fixture/temp" GITHUB_OUTPUT="$fixture/archive-output" \
    bash "$fixture/collect.sh" > "$fixture/collection.log" 2>&1
archive="$(sed -n 's/^path=//p' "$fixture/archive-output")"
[[ -f "$archive" ]] || exit 1
tar -xzf "$archive" -C "$fixture/unpacked"
for log in "${logs[@]}"; do cmp "$log" "$fixture/unpacked/${log##*/}"; done
cmp "${attempts[0]}/failure.log" \
    "$fixture/unpacked/portable-fixtures/${attempts[0]##*/}/failure.log"

# A refused store must still archive safe local evidence and expose incomplete
# collection. Exercise the selected action's archive and final failure commands;
# the hosted upload remains substituted by local unpacking.
mkdir "$fixture/refused-temp" "$fixture/refused-unpacked"
mkdir -p "$fixture/repository/.tools/rust"
printf 'retained-local-rust-evidence\n' > "$fixture/repository/.tools/rust/evidence.log"
printf 'retained-local-setup-failure\n' > "$fixture/refused-temp/tools-failure.log"
ln -s "$fixture/store" "$fixture/refused-store"
SHARED_TOOLING_STORE="$fixture/refused-store" \
    EVIDENCE_TEMP_ROOT="$fixture/refused-temp" EVIDENCE_REPOSITORY_ROOT="$fixture/repository" \
    EVIDENCE_ACTION_ROOT="$root/.github/actions/retain-failure-evidence" \
    RUNNER_TEMP="$fixture/refused-temp" GITHUB_OUTPUT="$fixture/refused-output" \
    bash "$fixture/collect.sh" > "$fixture/refused-collection.log" 2>&1
grep -Fx 'collection-complete=false' "$fixture/refused-output" > /dev/null
archive="$(sed -n 's/^path=//p' "$fixture/refused-output")"
[[ -f "$archive" ]] || exit 1
tar -xzf "$archive" -C "$fixture/refused-unpacked"
cmp "$fixture/refused-temp/tools-failure.log" "$fixture/refused-unpacked/tools-failure.log"
cmp "$fixture/repository/.tools/rust/evidence.log" "$fixture/refused-unpacked/.tools/rust/evidence.log"
diagnostics=("$fixture/refused-unpacked"/tool-evidence.*)
[[ ${#diagnostics[@]} == 1 && "$(cat "${diagnostics[0]}/selection-status")" == 1 ]] || exit 1
[[ -s "${diagnostics[0]}/selection.log" ]] || exit 1
jq -e '
    .runs.steps as $steps |
    [$steps | to_entries[] | select(.value.id == "archive")] as $archives |
    [$steps | to_entries[] | select(.value.id == "upload")] as $uploads |
    [$steps | to_entries[] | select(.value.id == "completeness")] as $failures |
    ($archives | length) == 1 and ($uploads | length) == 1 and ($failures | length) == 1 and
    $archives[0].key < $uploads[0].key and $uploads[0].key < $failures[0].key and
    ($failures[0].value.if | contains("collection-complete == '\''false'\''"))
' "$fixture/collector.json" > /dev/null
jq -er '.runs.steps[] | select(.id == "completeness") | .run' \
    "$fixture/collector.json" > "$fixture/incomplete.sh"
status=0
bash "$fixture/incomplete.sh" > "$fixture/incomplete.log" 2>&1 || status=$?
[[ "$status" == 1 ]] || exit 1
# Successful formatting reports one line and removes only its own temporary log.
RUNNER_TEMP="$fixture/temp" bash "$root/scripts/ci/run-formatting.sh" --write \
    bash -c 'echo successful-formatter-detail' > "$fixture/success"
printf 'Formatting... ok\n' > "$fixture/expected"
cmp "$fixture/expected" "$fixture/success"
remaining=("$fixture/temp"/formatting.*)
[[ ${#remaining[@]} == 1 ]] || exit 1
for log in "${logs[@]}"; do [[ -f "$log" ]] || exit 1; done
echo 'Formatting failure propagation and local archive retention passed (no hosted upload)'
echo 'Refused shared-store admission retains local evidence before incomplete collection fails (no hosted upload)'
fixture_complete=true
