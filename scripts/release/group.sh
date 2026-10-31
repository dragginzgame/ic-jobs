#!/usr/bin/env bash
set -euo pipefail

# Consumer default and publication roster; Shared owns policy/metadata admission.
export RELEASE_GROUP="${RELEASE_GROUP:-jobs}"
case "$RELEASE_GROUP" in jobs|timers) ;; *) echo 'select RELEASE_GROUP=jobs or timers' >&2; exit 2 ;; esac
operation="${1:?select plan, version, files or package}"
case "$operation" in
    plan|version|files) exec bash scripts/ci/cargo-release-group.sh "$operation" "$RELEASE_GROUP" ;;
    package)
        plan="$(bash scripts/ci/cargo-release-group.sh plan "$RELEASE_GROUP")"
        printf '%s\n' "$plan" | jq -er .primary ;;
    *) exit 2 ;;
esac
