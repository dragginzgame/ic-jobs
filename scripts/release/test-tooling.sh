#!/usr/bin/env bash
set -Eeuo pipefail

# Real Git fixtures; the complete release gate and crates.io effects are replaced.
# Dependencies: Bash 3.2, Git, Make, Cargo/cargo-edit/cargo-sort, jq/yq and Perl.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/jobs-release-tooling.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf "$fixture";
    else echo "Failed release tooling fixture retained: $fixture" >&2; fi
}
trap finish EXIT
trap 'printf "error: release tooling failed at %s:%s\n" "${BASH_SOURCE[0]}" "$LINENO" >&2' ERR
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_DELIVERY RELEASE_SOURCE RELEASE_COMMIT RELEASE_VERSION RELEASE_PREVIOUS RELEASE_DATE
unset RELEASE_REMOTE RELEASE_BRANCH RELEASE_MAKE VERSION
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
unset TOOLING_GATE_FAILURE TOOLING_REGISTRY_HTTP TOOLING_REGISTRY_RESULT TOOLING_UPLOAD_RESULT
export RUSTUP_AUTO_INSTALL=0
export TOOLING_REAL_CARGO
TOOLING_REAL_CARGO="$(command -v cargo)"

new_repository() {
    local name="$1"
    repository="$fixture/$name"
    mkdir -p "$repository"
    cp "$root"/{Cargo.toml,Cargo.lock,CHANGELOG.md,README.md,LICENSE,Makefile,.gitignore,rust-toolchain.toml} "$repository/"
    cp -R "$root/crates" "$root/scripts" "$root/make" "$root/ci" "$repository/"
    cd "$repository"
    git init --quiet --initial-branch=main
    git config user.name 'Release tooling fixture'
    git config user.email fixture@example.invalid
    git config commit.gpgSign false
    git config tag.gpgSign false
    git config core.hooksPath /dev/null
    git init --quiet --bare "$repository.git"
    git remote add origin "$repository.git"
}
commit_source() {
    git add -- .
    git commit --quiet -m 'Fixture source'
    git push --quiet origin main
}
refuse() {
    local label="$1"
    shift
    if "$@" > "$fixture/$label.log" 2>&1; then
        echo "Unexpected acceptance: $label" >&2; exit 1
    fi
}

# Exercise actual metadata preparation, commits, annotated tags and local atomic
# pushes for every increment, without running the consumer's complete gate.
for kind in patch minor major; do
    new_repository "release-$kind"
    candidate="$(bash scripts/ci/next-release-version.sh 0.1.0 "$kind")"
    printf '# Changelog\n\n## [%s]\n\n- Fixture release.\n' "$candidate" > CHANGELOG.md
    cat >> Makefile <<'MAKE'

# Fixture-only substitute for the complete consumer gate.
release-verify:
	@test "$${TOOLING_GATE_FAILURE:-0}" = 0
	@printf 'substitute gate\n' >> .git/gate-events
MAKE
    commit_source
    base="$(git rev-parse HEAD)"
    if [[ "$kind" == patch ]]; then
        refuse failed-gate env TOOLING_GATE_FAILURE=1 make --no-print-directory release-patch
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" ]]
        [[ "$(bash scripts/release/metadata.sh version)" == 0.1.0 ]]
        [[ -z "$(git status --porcelain)" ]]
    fi
    make --no-print-directory "release-$kind" > "$fixture/release-$kind.log" 2>&1
    [[ "$(bash scripts/release/metadata.sh version)" == "$candidate" ]]
    [[ "$(git log -1 --format=%s)" == "Release $candidate" ]]
    [[ "$(git log -1 --format=%P)" == "$base" ]]
    bash scripts/ci/check-release-tag.sh "$(git rev-parse HEAD)" "$candidate"
    [[ "$(git -C "$repository.git" rev-parse refs/heads/main)" == "$(git rev-parse HEAD)" ]]
    [[ "$(git -C "$repository.git" rev-parse "refs/tags/v$candidate")" == "$(git rev-parse "refs/tags/v$candidate")" ]]
    # Compare machine-owned paths in byte order, regardless of the caller's locale.
    [[ "$(git show --format= --name-only HEAD | LC_ALL=C sort)" == $'CHANGELOG.md\nCargo.lock\nCargo.toml' ]]
    [[ -z "$(git status --porcelain)" && -f ".git/release-state/$candidate.plan" ]]
    [[ "$(wc -l < .git/gate-events | tr -d ' ')" == 1 ]]
    make --no-print-directory release-resume "VERSION=$candidate" > "$fixture/resume-$kind.log" 2>&1
    [[ "$(wc -l < .git/gate-events | tr -d ' ')" == 1 ]]
done

# The actual publication adapter and shared helpers use real local Git. Only
# Cargo's upload and curl's registry observation are substituted.
new_repository publication
commit_source
commit="$(git rev-parse HEAD)"
git tag -a v0.1.0 -m 'Fixture release'
tag_object="$(git rev-parse refs/tags/v0.1.0)"
git push --quiet origin refs/tags/v0.1.0
mkdir "$fixture/bin"
export TOOLING_UPLOAD_EVENTS="$fixture/upload-events"
: > "$TOOLING_UPLOAD_EVENTS"
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == publish ]]; then
    printf '%s\n' "$*" >> "$TOOLING_UPLOAD_EVENTS"
    exit "${TOOLING_UPLOAD_RESULT:-0}"
fi
exec "$TOOLING_REAL_CARGO" "$@"
CARGO
cat > "$fixture/bin/curl" <<'CURL'
#!/usr/bin/env bash
set -euo pipefail
printf '%s' "${TOOLING_REGISTRY_HTTP:-404}"
exit "${TOOLING_REGISTRY_RESULT:-0}"
CURL
chmod +x "$fixture/bin/cargo" "$fixture/bin/curl"
export PATH="$fixture/bin:$PATH"

printf '\nFixture staged change.\n' >> README.md
git add README.md
git restore --source=HEAD --worktree -- README.md
refuse staged-source bash scripts/release/publish.sh origin
git restore --source=HEAD --staged -- README.md
printf '\nFixture unstaged change.\n' >> README.md
refuse unstaged-source bash scripts/release/publish.sh origin
git restore --source=HEAD --worktree -- README.md
touch untracked-source
refuse untracked-source bash scripts/release/publish.sh origin
rm untracked-source

for flags in -n -i -t; do
    refuse "make-mode-${flags#-}" env "MAKEFLAGS=$flags" bash scripts/release/publish.sh origin
done
refuse existing-version env TOOLING_REGISTRY_HTTP=200 bash scripts/release/publish.sh origin
refuse unavailable-registry env TOOLING_REGISTRY_HTTP=503 bash scripts/release/publish.sh origin
refuse transport-failure env TOOLING_REGISTRY_HTTP=200 TOOLING_REGISTRY_RESULT=7 bash scripts/release/publish.sh origin
git update-ref refs/tags/v0.1.0 "$commit"
refuse lightweight-tag bash scripts/release/publish.sh origin
git update-ref refs/tags/v0.1.0 "$tag_object"
git commit --quiet --allow-empty -m 'Fixture descendant'
refuse tag-not-at-head bash scripts/release/publish.sh origin
git update-ref refs/heads/main "$commit"
git -C "$repository.git" update-ref -d refs/tags/v0.1.0
refuse missing-remote-tag bash scripts/release/publish.sh origin
git -C "$repository.git" update-ref refs/tags/v0.1.0 "$commit"
refuse conflicting-remote-tag bash scripts/release/publish.sh origin
git -C "$repository.git" update-ref refs/tags/v0.1.0 "$tag_object"
git remote set-url --add --push origin "$repository.git"
git remote set-url --add --push origin "$fixture/another.git"
refuse multiple-destinations bash scripts/release/publish.sh origin
git config --unset-all remote.origin.pushurl
[[ ! -s "$TOOLING_UPLOAD_EVENTS" ]]

bash scripts/release/publish.sh origin > "$fixture/publication.log" 2>&1
[[ "$(cat "$TOOLING_UPLOAD_EVENTS")" == 'publish -p ic-jobs --all-features --locked --registry crates-io' ]]
: > "$TOOLING_UPLOAD_EVENTS"
refuse failed-upload env TOOLING_UPLOAD_RESULT=42 bash scripts/release/publish.sh origin
[[ "$(wc -l < "$TOOLING_UPLOAD_EVENTS" | tr -d ' ')" == 1 ]]
refuse uncertain-upload-now-present env TOOLING_REGISTRY_HTTP=200 bash scripts/release/publish.sh origin
[[ "$(wc -l < "$TOOLING_UPLOAD_EVENTS" | tr -d ' ')" == 1 ]]
[[ -z "$(git status --porcelain)" ]]
echo 'Release and publication fixtures passed (real local Git; substitute gate and registry/upload effects)'
