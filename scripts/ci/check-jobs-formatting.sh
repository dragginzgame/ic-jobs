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
fixture="$(fixture_create "${TMPDIR:-/tmp}/jobs-formatting.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Jobs formatting input retained"' EXIT

# Only reorder two adjacent dependencies; keep names, versions and features.
perl "$root/scripts/ci/prepare-cargo-formatting-fixture.pl" \
    "$root/Cargo.toml" workspace.dependencies ic-build ic-host-fs > "$fixture/unsorted.toml"
bash "$root/scripts/ci/check-formatting-hooks.sh" "$root" \
    crates/ic-jobs/src/job.rs Cargo.toml "$fixture/unsorted.toml" \
    --rust-sources \
    crates/ic-jobs/Cargo.toml apps/job-consumer/Cargo.toml crates/jobs-canister-tests/Cargo.toml \
    Cargo.lock make/consumer.mk make/tools.mk make/rust-format.mk make/release.mk make/execution.mk \
    ci/tool-versions.env scripts/ci/check-format-tools.sh scripts/ci/run-formatting.sh \
    scripts/ci/run-rust-formatting.pl
fixture_complete=true
