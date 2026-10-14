#!/usr/bin/env bash
set -euo pipefail

# Jobs selects the inputs; the canonical checker owns formatting/preservation
# assertions. Every Git mutation occurs in its disposable copies.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-formatting.XXXXXX")"
fixture="$(cd -P "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
fixture_complete=false
finish() {
    fixture_finish "$?" "$fixture" "Jobs formatting input retained"
}
trap finish EXIT

# Only reorder two adjacent dependencies; keep names, versions and features.
awk '
    $0 == "[workspace.dependencies]" { select=1; print; next }
    select {
        if ($0 !~ /^[[:alnum:]_-]+[[:space:]]*=/) exit 1
        if (saved == "") { saved=$0; next }
        print; print saved; saved=""; swapped=1; select=0; next
    }
    { print }
    END { if (!swapped || saved != "") exit 1 }
' "$root/Cargo.toml" > "$fixture/unsorted.toml"
bash "$root/scripts/ci/check-formatting-hooks.sh" "$root" \
    crates/ic-jobs/src/job.rs Cargo.toml "$fixture/unsorted.toml" \
    Cargo.lock make/consumer.mk make/tools.mk make/rust-format.mk make/release.mk make/execution.mk \
    ci/tool-versions.env ci/release-tools.env scripts/ci/check-format-tools.sh scripts/ci/run-formatting.sh
fixture_complete=true
