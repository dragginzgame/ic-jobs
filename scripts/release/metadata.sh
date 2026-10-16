#!/usr/bin/env bash
set -euo pipefail

# Consumer adapter. Dependencies: Cargo, jq, yq, Git, tar, awk and Unix utilities.
operation="${1:-}"
[[ $# -eq 1 ]] || exit 2
if [[ "${RELEASE_DELIVERY-direct}" != direct ]]; then
    echo 'ic-jobs supports RELEASE_DELIVERY=direct only' >&2
    exit 2
fi
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
reader="$root/scripts/ci/read-cargo-workspace-version.sh"
admit_files() {
    bash "$root/scripts/ci/check-release-source.sh" \
        --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
}
case "$operation" in
    version) bash "$reader" Cargo.toml ;;
    preflight)
        current_version="$(bash "$reader" Cargo.toml)"
        [[ "$current_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        admit_files
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do
            [[ -f "$path" && ! -L "$path" ]] || exit 1
        done
        # Refuse conflicting pending notes before a gate or preparation intent.
        awk -v version="${RELEASE_VERSION:?}" -v previous="$RELEASE_PREVIOUS" -v date="${RELEASE_DATE:?}" \
            -f scripts/ci/finalize-release-changelog.awk CHANGELOG.md > /dev/null
        # Jobs admits source/candidate first; Shared owns locked preparation,
        # selected Make execution and unchanged tool-catalog admission.
        bash "$root/scripts/ci/cargo-release-preflight.sh" "$root" 1 \
            --pin "$root/ci/tool-versions.env" --pin "$root/ci/ic-tools.tsv" \
            --check tools-check --prepare install-tools --prepare tools-check || {
            status=$?
            echo 'release dependency/tool preparation failed; selected metadata is unchanged' >&2
            echo 'prepare the graph with cargo fetch --locked and tools with make install-tools; explicit offline settings still apply' >&2
            exit "$status"
        }
        ;;
    prepare)
        current_version="$(bash "$reader" Cargo.toml)"
        [[ "$current_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        bash "$root/scripts/ci/cargo-release-metadata.sh" prepare ic-jobs
        ;;
    check|commit-check)
        # Shared owns selected-tree metadata checks and retained failure custody.
        bash "$root/scripts/ci/cargo-release-metadata.sh" check ic-jobs
        admit_files
        if [[ "$operation" == commit-check ]]; then
            [[ -z "${RELEASE_COMMIT:-}" ]] || exit 1
            git diff --quiet -- Cargo.toml Cargo.lock CHANGELOG.md
        fi
        ;;
    *) echo 'usage: metadata.sh version|preflight|prepare|check|commit-check' >&2; exit 2 ;;
esac
