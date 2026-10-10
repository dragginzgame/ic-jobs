#!/usr/bin/env bash
set -euo pipefail

# Jobs selects the inputs; the canonical checker owns formatting/preservation
# assertions. Every Git mutation occurs in its disposable copies.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-formatting.XXXXXX")"
complete=false
finish() {
    local status=$?
    [[ "$complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else echo "Jobs formatting input retained: $fixture" >&2; fi
    exit "$status"
}
trap finish EXIT

# Only reorder two adjacent dependencies; keep names, versions and features.
awk '
    $1 == "candid" { saved=$0; next }
    saved != "" {
        if ($1 != "ic-cdk") exit 1
        print; print saved; saved=""; swapped=1; next
    }
    { print }
    END { if (!swapped || saved != "") exit 1 }
' "$root/Cargo.toml" > "$fixture/unsorted.toml"
bash "$root/scripts/ci/check-formatting-hooks.sh" "$root" \
    crates/ic-jobs/src/job.rs Cargo.toml "$fixture/unsorted.toml" \
    Cargo.lock make/tools.mk make/rust-format.mk make/release.mk make/execution.mk \
    ci/tool-versions.env ci/release-tools.env scripts/ci/check-format-tools.sh scripts/ci/run-formatting.sh
complete=true
