#!/usr/bin/env bash
set -euo pipefail

# Consumer adapter. Dependencies: Cargo, jq, yq, Git, tar, awk and Unix utilities.
operation="${1:-}"
[[ $# -eq 1 ]] || exit 2
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
bash "$root/scripts/ci/check-release-delivery.sh" \
    "$root/ci/release-delivery.txt" "${RELEASE_DELIVERY-direct}"
reader="$root/scripts/ci/read-cargo-workspace-version.sh"
admit_files() {
    bash "$root/scripts/ci/check-release-source.sh" \
        --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
}
# The runner reconciles saved intent before dispatch. Its presence only removes
# setup authority here; the runner remains the owner of journal interpretation.
select_release_tools() {
    local plan source
    export RELEASE_METADATA_LOCKFILE="$root/Cargo.lock"
    prepare_tools=1
    plan="$(git rev-parse --git-path "release-state/${RELEASE_VERSION:?}.plan")"
    if [[ -e "$plan" || -L "$plan" ]]; then
        prepare_tools=0
        source="${RELEASE_PREPARATION_SOURCE:?select the admitted preparation source}"
        [[ "$source" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ && "$(git cat-file -t "$source")" == commit ]] || exit 1
        selection="$(mktemp -d "${TMPDIR:-/tmp}/jobs-release-selection.XXXXXX")"
        trap 'rm -rf -- "$selection"' EXIT
        git show "$source:Cargo.lock" > "$selection/Cargo.lock"
        export RELEASE_METADATA_LOCKFILE="$selection/Cargo.lock"
    fi
}
admit_editor() {
    # A failed check must stop even if it printed a plausible executable path.
    CARGO_RELEASE_METADATA_EDITOR="$("${RELEASE_MAKE:-make}" --silent --no-print-directory \
        -C "$root" release-tools-check "RELEASE_METADATA_LOCKFILE=$RELEASE_METADATA_LOCKFILE")" || return $?
    export CARGO_RELEASE_METADATA_EDITOR CARGO_RELEASE_METADATA_MAX_BYTES=1048576
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
        select_release_tools
        if [[ "$prepare_tools" == 0 ]]; then
            # Recovery uses the saved lock without resolving live partial metadata
            # or installing tools. Prepared/committed checks need no editor.
            admit_editor
            exit 0
        fi
        # Jobs admits source/candidate first; Shared owns locked preparation,
        # selected Make execution and unchanged tool-catalog admission.
        bash "$root/scripts/ci/cargo-release-preflight.sh" "$root" 1 \
            --pin "$root/ci/tool-versions.env" --pin "$root/ci/ic-tools.tsv" \
            --check tools-check --prepare install-tools --prepare tools-check \
            --prepare install-release-tools || {
            status=$?
            echo 'release dependency/tool preparation failed; selected metadata is unchanged' >&2
            echo 'prepare the graph with cargo fetch --locked and tools with make install-tools and make install-release-tools; explicit offline settings still apply' >&2
            exit "$status"
        }
        admit_editor
        ;;
    prepare)
        current_version="$(bash "$reader" Cargo.toml)"
        [[ "$current_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        select_release_tools
        admit_editor
        bash "$root/scripts/ci/cargo-release-metadata.sh" prepare ic-jobs
        ;;
    check|commit-check)
        # Shared owns selected-tree metadata checks and retained failure custody.
        bash "$root/scripts/ci/cargo-release-metadata.sh" check ic-jobs --manifest-order-grouped
        admit_files
        if [[ "$operation" == commit-check ]]; then
            [[ -z "${RELEASE_COMMIT:-}" ]] || exit 1
            git diff --quiet -- Cargo.toml Cargo.lock CHANGELOG.md
        fi
        ;;
    *) echo 'usage: metadata.sh version|preflight|prepare|check|commit-check' >&2; exit 2 ;;
esac
