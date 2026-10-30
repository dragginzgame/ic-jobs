#!/usr/bin/env bash
set -euo pipefail
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/publication-test.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Publication fixture retained"' EXIT

phase_stubbed_checker() (
trap - EXIT
mkdir -p "$fixture/snapshot/scripts/ci" "$fixture/snapshot/scripts/release" "$fixture/consumer" "$fixture/bin"
cp "$root/scripts/release/check-publication.sh" "$fixture/snapshot/scripts/release/"
export PUBLICATION_TEST_TRACE="$fixture/trace" PUBLICATION_TEST_CONSUMER="$fixture/consumer"
for helper in check-make-execution check-release-source read-cargo-workspace-version check-release-tag check-crates-io-version; do
 cat > "$fixture/snapshot/scripts/ci/$helper.sh" <<'HELPER'
#!/usr/bin/env bash
set -euo pipefail
[[ "$PWD" == "$PUBLICATION_TEST_CONSUMER" ]] || exit 48
name="${0##*/}"; printf '%s %s\n' "$name" "$*" >> "$PUBLICATION_TEST_TRACE"
case "$name" in
 check-release-source.sh) [[ "${PUBLICATION_TEST_SOURCE:-clean}" == clean && ! -e "$PUBLICATION_TEST_CONSUMER/dirty" ]] ;;
 read-cargo-workspace-version.sh) echo 1.2.3; [[ "${PUBLICATION_TEST_FAILURE:-}" != version ]] || exit 43 ;;
 check-release-tag.sh) [[ "${PUBLICATION_TEST_TAG:-annotated}" == annotated ]] ;;
 check-crates-io-version.sh) [[ "$*" == 'ic-jobs 1.2.3' || "$*" == 'ic-timers 1.2.3' ]] || exit 48; [[ "${PUBLICATION_TEST_RACE:-}" != source ]] || touch "$PUBLICATION_TEST_CONSUMER/dirty"; exit "${PUBLICATION_TEST_REGISTRY:-1}" ;;
 *) ;;
esac
HELPER
 done
cat > "$fixture/bin/git" <<'GIT'
#!/usr/bin/env bash
set -euo pipefail
printf 'git %s\n' "$*" >> "$PUBLICATION_TEST_TRACE"
case "$*" in
 'rev-parse --verify HEAD') if [[ "${PUBLICATION_TEST_RACE:-}" == head && "$(grep -c '^git rev-parse --verify HEAD' "$PUBLICATION_TEST_TRACE")" == 2 ]]; then printf '%040d\n' 3; else printf '%040d\n' 1; fi ;;
 'rev-parse --verify refs/tags/v1.2.3') if [[ "${PUBLICATION_TEST_RACE:-}" == tag && "$(grep -c '^git rev-parse --verify refs/tags/v1.2.3' "$PUBLICATION_TEST_TRACE")" == 2 ]]; then printf '%040d\n' 3; else printf '%040d\n' 2; fi ;;
 'remote get-url --push --all origin') if [[ "${PUBLICATION_TEST_REMOTE:-single}" == single ]]; then echo fixture-remote; else printf 'one\ntwo\n'; fi ;;
 'ls-remote --exit-code -- fixture-remote refs/tags/v1.2.3') printf '%040d\trefs/tags/v1.2.3\n' "${PUBLICATION_TEST_OBSERVED:-2}" ;;
 *) exit 48 ;;
esac
if [[ "${PUBLICATION_TEST_FAILURE:-}" == head && "$*" == 'rev-parse --verify HEAD' &&
      "$(grep -c '^git rev-parse --verify HEAD' "$PUBLICATION_TEST_TRACE")" == 2 ]]; then exit 43; fi
if [[ "${PUBLICATION_TEST_FAILURE:-}" == tag && "$*" == 'rev-parse --verify refs/tags/v1.2.3' &&
      "$(grep -c '^git rev-parse --verify refs/tags/v1.2.3' "$PUBLICATION_TEST_TRACE")" == 2 ]]; then exit 43; fi
if [[ "${PUBLICATION_TEST_FAILURE:-}" == remote && "$*" == 'remote get-url --push --all origin' ]]; then exit 43; fi
if [[ "${PUBLICATION_TEST_FAILURE:-}" == observation && "$1" == ls-remote ]]; then exit 43; fi
GIT
chmod +x "$fixture/bin/git"
# shellcheck disable=SC2030 # The stubbed PATH intentionally ends with this phase.
export PATH="$fixture/bin:$PATH"
cd "$fixture/consumer"
checker="$fixture/snapshot/scripts/release/check-publication.sh"
: > "$PUBLICATION_TEST_TRACE"
bash "$checker" origin
[[ "$(grep -c '^check-release-source.sh' "$PUBLICATION_TEST_TRACE")" == 2 ]] || exit 1
for package in ic-jobs ic-timers; do
 grep -Fx "check-crates-io-version.sh $package 1.2.3" "$PUBLICATION_TEST_TRACE" > /dev/null || exit 1
done
for mode in dirty tag remote mismatch present unknown; do
 : > "$PUBLICATION_TEST_TRACE"
 status=0
 case "$mode" in
 dirty) PUBLICATION_TEST_SOURCE=dirty bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 tag) PUBLICATION_TEST_TAG=lightweight bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 remote) PUBLICATION_TEST_REMOTE=multiple bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 mismatch) PUBLICATION_TEST_OBSERVED=3 bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 present) PUBLICATION_TEST_REGISTRY=0 bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 unknown) PUBLICATION_TEST_REGISTRY=2 bash "$checker" origin > "$fixture/$mode.log" 2>&1 || status=$? ;;
 esac
 [[ "$status" != 0 ]] || exit 1
 if [[ "$mode" == dirty || "$mode" == tag || "$mode" == remote || "$mode" == mismatch ]]; then
  ! grep -q '^check-crates-io-version.sh' "$PUBLICATION_TEST_TRACE" || exit 1
 fi
done
for failure in version head tag remote observation; do
 : > "$PUBLICATION_TEST_TRACE"
 status=0
 PUBLICATION_TEST_FAILURE="$failure" bash "$checker" origin > "$fixture/failure-$failure.log" 2>&1 || status=$?
 [[ "$status" != 0 ]] || { echo "failed observation accepted: $failure" >&2; exit 1; }
 case "$failure" in
  version|head|tag)
   [[ "$status" == 43 ]] || exit 1
   ;;
 esac
 if [[ "$failure" == version || "$failure" == remote || "$failure" == observation ]]; then
  ! grep -q '^check-crates-io-version.sh' "$PUBLICATION_TEST_TRACE" || exit 1
 fi
done
for race in source head tag; do
 : > "$PUBLICATION_TEST_TRACE"
 if PUBLICATION_TEST_RACE="$race" bash "$checker" origin > "$fixture/race-$race.log" 2>&1; then exit 1; fi
 rm -f "$PUBLICATION_TEST_CONSUMER/dirty"
done
if bash "$checker" origin extra > "$fixture/invalid.log" 2>&1; then exit 1; fi
if bash "$checker" --push > "$fixture/invalid-remote.log" 2>&1; then exit 1; fi
for suffix in $'\n' $'\r'; do
 bad="$fixture/bad$suffix"
 mkdir "$bad"
 if (cd "$bad" && bash "$checker" origin) > "$fixture/bad-path.log" 2>&1; then exit 1; fi
 grep -F 'directory contains CR/LF' "$fixture/bad-path.log" > /dev/null
 ln -s "$bad" "$fixture/alias"
 if (cd "$fixture/alias" && bash "$checker" origin) > "$fixture/alias-path.log" 2>&1; then exit 1; fi
 grep -F 'directory contains CR/LF' "$fixture/alias-path.log" > /dev/null
 rm "$fixture/alias"
 rmdir "$bad"
done
)

# Own native Git eligibility alongside the ordering tests above. Only registry
# transport is substituted; the actual checker, source/version/tag helpers and
# local bare destination participate. No publication command can be dispatched.
phase_real_git() (
    trap - EXIT
    unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
    unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
    repository="$fixture/real publication checkout"
    destination="$fixture/publication destination.git"
    mkdir -p "$repository" "$fixture/registry-bin"
    cat > "$fixture/registry-bin/curl" <<'CURL'
#!/usr/bin/env bash
printf '%s' "${PUBLICATION_HTTP:-404}"
exit "${PUBLICATION_TRANSPORT:-0}"
CURL
    chmod +x "$fixture/registry-bin/curl"
    # shellcheck disable=SC2031 # This phase starts from the caller PATH, without Git stubs.
    export PATH="$fixture/registry-bin:$PATH"
    git init --quiet --initial-branch=main "$repository"
    git init --quiet --bare "$destination"
    cd "$repository"
    git config user.name 'Publication fixture'
    git config user.email fixture@example.invalid
    git config commit.gpgSign false
    git config tag.gpgSign false
    git config core.hooksPath /dev/null
    git remote add origin "$destination"
    printf '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "1.2.3"\n' > Cargo.toml
    printf 'Publication fixture\n' > README.md
    git add -- Cargo.toml README.md
    git commit --quiet -m 'Fixture release'
    commit="$(git rev-parse HEAD)"
    tag=refs/tags/v1.2.3
    git tag -a v1.2.3 -m 'Fixture release'
    tag_object="$(git rev-parse "$tag")"
    git push --quiet origin "$tag"
    checker="$root/scripts/release/check-publication.sh"
    refuse_real() {
        local label="$1" expected="$2" status=0
        shift 2
        "$@" > "$fixture/real-$label.log" 2>&1 || status=$?
        [[ "$status" != 0 ]] || { echo "Unexpected real-Git acceptance: $label" >&2; exit 1; }
        grep -F -- "$expected" "$fixture/real-$label.log" > /dev/null
    }
    "$BASH" "$checker" origin
    printf '\nStaged fixture change\n' >> README.md
    git add -- README.md
    git restore --source=HEAD --worktree -- README.md
    refuse_real staged "staged: README.md" "$BASH" "$checker" origin
    git restore --source=HEAD --staged -- README.md
    printf '\nUnstaged fixture change\n' >> README.md
    refuse_real unstaged "unstaged: README.md" "$BASH" "$checker" origin
    git restore --source=HEAD --worktree -- README.md
    touch untracked-source
    refuse_real untracked "untracked: untracked-source" "$BASH" "$checker" origin
    rm untracked-source
    refuse_real present "already exists on crates.io" env PUBLICATION_HTTP=200 "$BASH" "$checker" origin
    refuse_real unavailable "HTTP 503" env PUBLICATION_HTTP=503 "$BASH" "$checker" origin
    refuse_real transport "transport failure" env PUBLICATION_HTTP=200 PUBLICATION_TRANSPORT=7 "$BASH" "$checker" origin
    git update-ref "$tag" "$commit"
    refuse_real lightweight "must be annotated" "$BASH" "$checker" origin
    git update-ref "$tag" "$tag_object"
    git commit --quiet --allow-empty -m 'Fixture descendant'
    refuse_real wrong-head "does not point to" "$BASH" "$checker" origin
    git update-ref refs/heads/main "$commit"
    git -C "$destination" update-ref -d "$tag"
    refuse_real missing-remote "cannot verify the pushed release tag" "$BASH" "$checker" origin
    git -C "$destination" update-ref "$tag" "$commit"
    refuse_real conflicting-remote "remote release tag differs" "$BASH" "$checker" origin
    git -C "$destination" update-ref "$tag" "$tag_object"
    git remote set-url --add --push origin "$destination"
    git remote set-url --add --push origin "$fixture/another.git"
    refuse_real multiple-destinations "select exactly one push destination" "$BASH" "$checker" origin
    git config --unset-all remote.origin.pushurl
    "$BASH" "$checker" origin
    [[ "$(git rev-parse HEAD)" == "$commit" && "$(git rev-parse "$tag")" == "$tag_object" && -z "$(git status --porcelain)" ]] || exit 1
)
phase_stubbed_checker
phase_real_git
fixture_complete=true
echo 'Publication ordering and real-Git eligibility passed (substitute registry; no upload)'
