#!/usr/bin/env bash
set -euo pipefail

# One projection of Cargo's declared floor for builds, launchers and CI setup.
# This helper never installs a toolchain or resolves dependencies.
[[ $# -ge 1 ]] || { echo 'usage: minimum-rust.sh --print|cargo|rustc [ARGS...]' >&2; exit 2; }
case "$1" in
    --print) [[ $# == 1 ]] || exit 2 ;;
    cargo|rustc) ;;
    *) echo 'expected --print, cargo or rustc' >&2; exit 2 ;;
esac
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
version="$(bash "$root/scripts/ci/read-cargo-workspace-rust-version.sh" "$root/Cargo.toml")"
[[ "$version" == *.*.* ]] || version="$version.0"
if [[ "$1" == --print ]]; then printf '%s\n' "$version"; exit 0; fi
tool="$1"
shift
exec "$tool" "+$version" "$@"
