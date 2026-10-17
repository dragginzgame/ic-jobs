#!/usr/bin/env bash
set -euo pipefail

# Consumer publication policy. Dependencies: Bash 3.2, Git, Cargo, jq/yq,
# curl and the local publication-admission checker.
# Effects: package verification and one ic-jobs upload to crates.io; no Git writes.
[[ $# == 1 && "$1" =~ ^[A-Za-z0-9._-]+$ ]] || {
    echo 'usage: publish.sh REMOTE' >&2; exit 2;
}
remote="$1"
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"
bash "$root/scripts/release/check-publication.sh" "$remote"
cargo publish -p ic-jobs --all-features --locked --registry crates-io
