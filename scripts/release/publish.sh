#!/usr/bin/env bash
set -euo pipefail

# Consumer publication policy. Dependencies: Bash 3.2, Git, Cargo, jq/yq,
# curl and the reviewed version/tag/registry/Make-execution helpers.
# Effects: package verification and one ic-jobs upload to crates.io; no Git writes.
[[ $# == 1 && "$1" =~ ^[A-Za-z0-9._-]+$ ]] || {
    echo 'usage: publish.sh REMOTE' >&2; exit 2;
}
remote="$1"
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
cd "$root"
fail() { echo "publication refused: $*" >&2; exit 1; }
bash scripts/ci/check-make-execution.sh

# Observe the index as well as the worktree, including untracked source.
status="$(git status --porcelain --untracked-files=all)" || fail 'cannot inspect source'
if [[ -n "$status" ]]; then
    printf '%s\n' "$status" >&2
    fail 'commit the intended source before publication'
fi
commit="$(git rev-parse --verify HEAD)" || fail 'publication requires a committed release'
version="$(bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)"
bash scripts/ci/check-release-tag.sh "$commit" "$version"
tag="refs/tags/v$version"
tag_object="$(git rev-parse --verify "$tag")"
destination="$(git remote get-url --push --all "$remote")" || fail 'cannot inspect push destination'
[[ -n "$destination" && "$destination" != *$'\n'* ]] || fail 'select exactly one push destination'
observed="$(git ls-remote --exit-code -- "$destination" "$tag")" || fail 'cannot verify the pushed release tag'
[[ "$observed" == "$tag_object"$'\t'"$tag" ]] || fail 'remote release tag differs from the local annotated tag'

# Unknown registry state cannot authorize a new upload. Duplicate versions are
# immutable at crates.io; an interrupted upload is observed again on invocation.
registry_status=0
bash scripts/ci/check-crates-io-version.sh ic-jobs "$version" || registry_status=$?
case "$registry_status" in
    0) fail "ic-jobs $version already exists on crates.io; reconcile any earlier upload" ;;
    1) ;;
    *) fail 'crates.io version observation is unavailable; resolve it before retrying' ;;
esac

# Recheck local source after network observations, before dispatching Cargo.
[[ "$(git rev-parse --verify HEAD)" == "$commit" &&
   "$(git rev-parse --verify "$tag")" == "$tag_object" ]] || fail 'release identity changed during preflight'
status="$(git status --porcelain --untracked-files=all)" || fail 'cannot recheck source'
[[ -z "$status" ]] || fail 'source changed during publication preflight'
cargo publish -p ic-jobs --all-features --locked --registry crates-io
