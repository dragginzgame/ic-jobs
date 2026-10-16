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
        # Standard releases prepare the admitted graph before offline validation.
        # Cargo still honours an explicit offline environment/configuration.
        cargo fetch --locked || {
            status=$?
            echo 'release dependency preparation failed; selected metadata is unchanged' >&2
            echo 'prepare the selected graph with cargo fetch --locked; explicit offline settings still apply' >&2
            exit "$status"
        }
        # The runner has already reconciled saved release intent. Prepare only
        # the complete selected toolset after source/cache admission.
        "${RELEASE_MAKE:-make}" --no-print-directory install-tools
        "${RELEASE_MAKE:-make}" --no-print-directory tools-check
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
