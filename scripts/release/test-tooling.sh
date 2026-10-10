#!/usr/bin/env bash
set -Eeuo pipefail

# Real Git fixtures; the complete release gate and crates.io effects are replaced.
# Dependencies: Bash 3.2, Git, Make, Cargo/cargo-edit/cargo-sort, jq/yq and Perl.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
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
unset TOOLING_FETCH_FAILURE
export RUSTUP_AUTO_INSTALL=0
export TOOLING_REAL_CARGO
TOOLING_REAL_CARGO="$(command -v cargo)"
mkdir "$fixture/cache-bin"
cat > "$fixture/cache-bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == fetch ]]; then
    printf '%s\n' "$*" >> "$TOOLING_FETCH_EVENTS"
    [[ "$*" == 'fetch --locked' ]] || { echo 'unexpected cache preparation arguments' >&2; exit 45; }
    if [[ "${TOOLING_FETCH_FAILURE:-0}" != 0 ]]; then
        echo 'fixture registry fetch failed' >&2
        exit "$TOOLING_FETCH_FAILURE"
    fi
    if [[ "${CARGO_NET_OFFLINE:-false}" == true && ! -f "$TOOLING_CACHE_READY" ]]; then
        echo 'fixture dependency is absent from the offline cache' >&2
        exit 44
    fi
    touch "$TOOLING_CACHE_READY"
    exit 0
fi
exec "$TOOLING_REAL_CARGO" "$@"
CARGO
chmod +x "$fixture/cache-bin/cargo"
export PATH="$fixture/cache-bin:$PATH"
base_version="$(bash "$root/scripts/ci/read-cargo-workspace-version.sh" --stable "$root/Cargo.toml")"

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
    export TOOLING_FETCH_EVENTS="$repository/.git/fetch-events"
    export TOOLING_CACHE_READY="$repository/.git/cache-ready"
    : > "$TOOLING_FETCH_EVENTS"
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
    else
        refusal_status=$?
    fi
}

# Exercise actual metadata preparation, commits, annotated tags and local atomic
# pushes for every increment, without running the consumer's complete gate.
for kind in patch minor major; do
    new_repository "release-$kind"
    candidate="$(bash scripts/ci/next-release-version.sh "$base_version" "$kind")"
    printf '# Changelog\n\n## [%s]\n\n- Fixture release.\n' "$candidate" > CHANGELOG.md
    cat >> Makefile <<'MAKE'

# Fixture-only substitute for the complete consumer gate.
release-verify:
	@test "$${TOOLING_GATE_FAILURE:-0}" = 0
	@printf 'substitute gate\n' >> .git/gate-events
.PHONY: tooling-nested tooling-selection
tooling-nested:
	+@$(MAKE) tooling-selection
tooling-selection:
	@test "$(TOOLING_QUOTED_SELECTION)" = "owner's selection"
MAKE
    commit_source
    base="$(git rev-parse HEAD)"
    # The outer Make must refuse execution modes which can turn a failed or
    # skipped recipe into successful release/formatting evidence. Exercise both
    # direct flags and inherited controls before any cache or runner effects.
    if [[ "$kind" == patch ]]; then
        # Harmless goals use only the selected admission helper, even with an
        # unrelated runtime root. Parallel recursive commands retain selections
        # without loading their consumer Makefiles into the isolated probe.
        make --no-print-directory help SHARED_TOOLING_ROOT="$fixture/unselected" > "$fixture/external-root.log" 2>&1
        make -k -s --no-print-directory help "AUDIT_SELECTION=owner's selection" > "$fixture/ordinary-flags.log" 2>&1
        recursive_make="$(command -v make) --no-print-directory -f Makefile"
        make -j2 --no-print-directory tooling-nested "MAKE=$recursive_make" \
            "TOOLING_QUOTED_SELECTION=owner's selection" SHARED_TOOLING_ROOT="$fixture/unselected" \
            > "$fixture/recursive-selection.log" 2>&1
        if grep -i jobserver "$fixture/recursive-selection.log" > /dev/null; then exit 1; fi
        for target in release-patch release-minor release-major release-resume fmt fmt-check publish; do
            for mode in -i --ignore-errors -n --dry-run -t --touch -q --question -kin; do
                for controls in direct inherited cleared replaced erased-mflags; do
                    label="refused-mode-$target-$mode-$controls"
                    cp .git/index "$fixture/mode-index"
                    if [[ "$controls" == direct ]]; then
                        refuse "$label" make --no-print-directory "$mode" "$target" VERSION="$candidate"
                    elif [[ "$controls" == inherited ]]; then
                        refuse "$label" env _shared_make_execution_checked=yes MAKEFLAGS="$mode" \
                            make --no-print-directory "$target" VERSION="$candidate"
                    elif [[ "$controls" == cleared ]]; then
                        refuse "$label" make --no-print-directory "$mode" "$target" VERSION="$candidate" MAKEFLAGS=
                    elif [[ "$controls" == replaced ]]; then
                        refuse "$label" make --no-print-directory "$mode" "$target" VERSION="$candidate" MAKEFLAGS=--no-print-directory
                    else
                        refuse "$label" make --no-print-directory "$mode" "$target" VERSION="$candidate" MAKEFLAGS= MFLAGS=
                    fi
                    [[ "$refusal_status" == 2 ]]
                    if [[ "$controls" == erased-mflags ]]; then
                        grep -F "generated MFLAGS" "$fixture/$label.log" > /dev/null
                    else
                        grep -F 'Make recipe execution and failure propagation' "$fixture/$label.log" > /dev/null
                    fi
                    cmp .git/index "$fixture/mode-index"
                    [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" ]]
                    [[ ! -e .git/gate-events && ! -s "$TOOLING_FETCH_EVENTS" ]]
                    [[ ! -e .git/release-state ]]
                    git diff --exit-code HEAD -- > /dev/null
                done
            done
        done
        # Even harmless invocations cannot independently assign the evidence of
        # the running Make's options, whether by CLI or an extra Makefile.
        refuse refused-mflags-assignment make --no-print-directory help MFLAGS=
        grep -F 'generated MFLAGS' "$fixture/refused-mflags-assignment.log" > /dev/null
        printf 'override MFLAGS :=\n' > "$fixture/mflags.mk"
        refuse refused-mflags-include make --no-print-directory -f "$fixture/mflags.mk" -f Makefile help
        grep -F 'generated MFLAGS' "$fixture/refused-mflags-include.log" > /dev/null
    fi
    # Jobs selects direct release delivery only. Unsupported PR delivery must
    # stop before fetching, validating or mutating this committed fixture.
    refuse "refused-pr-$kind" env RELEASE_DELIVERY=pr make --no-print-directory "release-$kind"
    grep -F 'ic-jobs supports direct release delivery only' "$fixture/refused-pr-$kind.log" > /dev/null
    refuse "refused-pr-adapter-$kind" env RELEASE_DELIVERY=pr bash scripts/release/metadata.sh preflight
    grep -F 'ic-jobs supports RELEASE_DELIVERY=direct only' "$fixture/refused-pr-adapter-$kind.log" > /dev/null
    [[ "$(git rev-parse HEAD)" == "$base" && ! -e .git/gate-events && ! -s "$TOOLING_FETCH_EVENTS" ]]
    git diff --exit-code HEAD -- > /dev/null
    if [[ "$kind" == patch ]]; then
        # The consumer adapter reports every refused path before gate/preparation,
        # preserving hidden staged changes, working bytes and the index itself.
        printf '\nFixture staged change.\n' >> README.md
        git add README.md
        git restore --source=HEAD --worktree -- README.md
        printf '\nFixture unstaged change.\n' >> LICENSE
        unusual=$'untracked\nsource.txt'
        printf 'Fixture untracked content.\n' > "$unusual"
        cp .git/index "$fixture/source-index"
        cp LICENSE "$fixture/source-working"
        cp "$unusual" "$fixture/source-untracked"
        refuse refused-source make --no-print-directory release-patch
        for expected in 'staged: README.md' 'unstaged: README.md' 'unstaged: LICENSE' \
            'preflight refused; this attempt has not started validation or version preparation'; do
            grep -F "$expected" "$fixture/refused-source.log" > /dev/null
        done
        printf '  untracked: %q\n' "$unusual" > "$fixture/source-expected"
        grep -Fx -f "$fixture/source-expected" "$fixture/refused-source.log" > /dev/null
        cmp .git/index "$fixture/source-index"
        cmp LICENSE "$fixture/source-working"
        cmp "$unusual" "$fixture/source-untracked"
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" && ! -e .git/gate-events ]]
        [[ ! -s "$TOOLING_FETCH_EVENTS" && ! -e "$TOOLING_CACHE_READY" ]]
        git restore --source=HEAD --staged --worktree -- README.md LICENSE
        rm "$unusual"

        # Release metadata allowances are literal; an allowed rename destination
        # cannot hide a refused original path.
        printf '\nFixture metadata edit.\n' >> Cargo.lock
        cp .git/index "$fixture/source-index"
        cp Cargo.lock "$fixture/source-lock"
        bash scripts/ci/check-release-source.sh --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
        cmp .git/index "$fixture/source-index"
        cmp Cargo.lock "$fixture/source-lock"
        git restore --source=HEAD --worktree -- Cargo.lock
        git mv README.md CHANGELOG-renamed.md
        refuse renamed-source bash scripts/ci/check-release-source.sh --allow CHANGELOG-renamed.md
        grep -F 'staged: README.md' "$fixture/renamed-source.log" > /dev/null
        git restore --source=HEAD --staged --worktree -- README.md
        git restore --source=HEAD --staged -- CHANGELOG-renamed.md
        rm CHANGELOG-renamed.md

        # A failed Git observation stays distinct from a dirty checkout and cannot
        # reach validation; invoke the actual adapter with a substituted observer.
        mkdir "$fixture/source-bin"
        export TOOLING_REAL_GIT
        TOOLING_REAL_GIT="$(command -v git)"
        cat > "$fixture/source-bin/git" <<'GIT'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == status ]]; then echo 'fixture Git status observation failed' >&2; exit 9; fi
exec "$TOOLING_REAL_GIT" "$@"
GIT
        chmod +x "$fixture/source-bin/git"
        cp .git/index "$fixture/source-index"
        refuse source-observation env PATH="$fixture/source-bin:$PATH" \
            RELEASE_PREVIOUS="$base_version" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-09 \
            bash scripts/release/metadata.sh preflight
        grep -F 'fixture Git status observation failed' "$fixture/source-observation.log" > /dev/null
        grep -F 'cannot inspect release-source status' "$fixture/source-observation.log" > /dev/null
        if grep -F 'uncommitted paths' "$fixture/source-observation.log" > /dev/null; then exit 1; fi
        cmp .git/index "$fixture/source-index"
        [[ ! -e .git/gate-events && -z "$(git status --porcelain)" ]]

        # Preparation respects caller offline mode, preserves the selected lock
        # and stops before the gate/version writes when fetching cannot complete.
        cp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cp "$path" "$fixture/cache-$path"; done
        preflight=(env RELEASE_PREVIOUS="$base_version" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-09)
        refuse cold-offline "${preflight[@]}" CARGO_NET_OFFLINE=true bash scripts/release/metadata.sh preflight
        [[ "$refusal_status" == 44 && ! -e "$TOOLING_CACHE_READY" ]]
        grep -F 'fixture dependency is absent from the offline cache' "$fixture/cold-offline.log" > /dev/null
        refuse failed-fetch "${preflight[@]}" CARGO_NET_OFFLINE=false TOOLING_FETCH_FAILURE=43 bash scripts/release/metadata.sh preflight
        [[ "$refusal_status" == 43 && ! -e "$TOOLING_CACHE_READY" ]]
        grep -F 'fixture registry fetch failed' "$fixture/failed-fetch.log" > /dev/null
        cmp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/cache-$path"; done
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" && ! -e .git/gate-events ]]
        [[ ! -e ".git/release-state/$candidate.plan" ]]
        "${preflight[@]}" CARGO_NET_OFFLINE=false bash scripts/release/metadata.sh preflight
        [[ -f "$TOOLING_CACHE_READY" ]]
        "${preflight[@]}" CARGO_NET_OFFLINE=true bash scripts/release/metadata.sh preflight
        [[ "$(wc -l < "$TOOLING_FETCH_EVENTS" | tr -d ' ')" == 4 ]]
        cmp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/cache-$path"; done
        [[ ! -e .git/gate-events && ! -e ".git/release-state/$candidate.plan" ]]

        refuse failed-gate env TOOLING_GATE_FAILURE=1 make --no-print-directory release-patch
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" ]]
        [[ "$(bash scripts/release/metadata.sh version)" == "$base_version" ]]
        [[ -z "$(git status --porcelain)" ]]
    fi
    # The full offline gate runs fixtures against prepared dependencies. Model
    # that input for the ordinary release series; cold-cache cases above select
    # their offline/online outcomes explicitly without any actual fetch.
    if [[ "${CARGO_NET_OFFLINE:-false}" == true ]]; then touch "$TOOLING_CACHE_READY"; fi
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
tag="refs/tags/v$base_version"
git tag -a "v$base_version" -m 'Fixture release'
tag_object="$(git rev-parse "$tag")"
git push --quiet origin "$tag"
# Keep the remote's ordinary path while exercising a checkout whose name contains
# spaces and ends in newlines. Command substitution must preserve its identity.
publication_checkout="$fixture/"$'publication checkout\n\n'
publication_remote="$repository.git"
cd "$fixture"
mv "$repository" "$publication_checkout"
repository="$publication_checkout"
cd "$repository"
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
git update-ref "$tag" "$commit"
refuse lightweight-tag bash scripts/release/publish.sh origin
git update-ref "$tag" "$tag_object"
git commit --quiet --allow-empty -m 'Fixture descendant'
refuse tag-not-at-head bash scripts/release/publish.sh origin
git update-ref refs/heads/main "$commit"
git -C "$publication_remote" update-ref -d "$tag"
refuse missing-remote-tag bash scripts/release/publish.sh origin
git -C "$publication_remote" update-ref "$tag" "$commit"
refuse conflicting-remote-tag bash scripts/release/publish.sh origin
git -C "$publication_remote" update-ref "$tag" "$tag_object"
git remote set-url --add --push origin "$publication_remote"
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
