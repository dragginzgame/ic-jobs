#!/usr/bin/env bash
set -Eeuo pipefail

# Real Git fixtures; the complete release gate and crates.io effects are replaced.
# Dependencies: Bash 3.2, Git, Make, Cargo/cargo-sort, jq/yq and Perl.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/jobs-release-tooling.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed release tooling fixture retained"' EXIT
trap 'printf "error: release tooling failed at %s:%s\n" "${BASH_SOURCE[0]}" "$LINENO" >&2' ERR
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_DELIVERY RELEASE_SOURCE RELEASE_COMMIT RELEASE_VERSION RELEASE_PREVIOUS RELEASE_DATE
unset RELEASE_REMOTE RELEASE_BRANCH RELEASE_MAKE VERSION
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
unset TOOLING_GATE_FAILURE TOOLING_REGISTRY_HTTP TOOLING_UPLOAD_RESULT
unset TOOLING_FETCH_FAILURE
unset TOOLING_INSTALL_FAILURE TOOLING_CHECK_FAILURE TOOLING_BUILD_EVENTS
unset TOOLING_COMMON_CHECK_FAILURE
export RUSTUP_AUTO_INSTALL=0

# Shared owns exit injection and retained-evidence assertions. Jobs selects its
# actual fixture roster; archive round trips and command assertions stay local.
# shellcheck source=scripts/ci/fixture-retention-probes.sh
source "$root/scripts/ci/fixture-retention-probes.sh"
mkdir -p "$fixture/exit-source/scripts/release" "$fixture/exit-source/scripts/ci"
cp "$root/scripts/ci/fixture-retention-probes.sh" "$fixture/exit-source/scripts/ci/"
for relative in scripts/release/test-tooling.sh scripts/release/test-publication.sh scripts/ci/test-formatting-evidence.sh scripts/ci/test-consumer-tooling.sh scripts/ci/check-jobs-formatting.sh; do
    status=0
    FIXTURE_PROBE_ROOT_VARIABLE=root fixture_probe_exits "$root/$relative" "$fixture" "$root" "${relative##*/}" || status=$?
    [[ "$status" == 0 ]] || exit 1
done

# Contradict one explicit command-status assertion in each actual fixture. Bash 3.2
# can ignore a bare [[ ... ]] failure, continue, and delete the evidence after
# reaching completion; these mandatory checks must explicitly stop the fixture.
for relative in scripts/release/test-tooling.sh scripts/release/test-publication.sh scripts/ci/test-formatting-evidence.sh scripts/ci/test-consumer-tooling.sh; do
    # shellcheck disable=SC2016 # Select literal assertions from maintained sources.
    case "$relative" in
        scripts/release/test-tooling.sh) assertion='[[ "$status" == 0 ]]' ;;
        scripts/release/test-publication.sh) assertion='[[ "$status" == 43 ]]' ;;
        scripts/ci/test-formatting-evidence.sh) assertion='[[ "$status" == 37 ]]' ;;
        scripts/ci/test-consumer-tooling.sh) assertion='[[ "$status" == 2 ]]' ;;
    esac
    fixture_probe_assertion "$root/$relative" "$fixture" "$root" "${relative##*/}" \
        --selected "$root/$relative" "$assertion" 'status=97' || exit 1
done

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
    cp "$root"/{Cargo.toml,Cargo.lock,CHANGELOG.md,README.md,LICENSE,Makefile,.gitignore,.shared-tooling.snapshot,rust-toolchain.toml} "$repository/"
    cp -R "$root/crates" "$root/apps" "$root/scripts" "$root/make" "$root/ci" "$repository/"
    # Keep shared recipes intact; replace their external installer effects.
    for toolset in host ic rust; do
        cat > "$repository/scripts/dev/install-$toolset-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
set -euo pipefail
consumer="$(cd -P . && printf '%s/.' "$PWD")"
consumer="${consumer%/.}"
[[ $# == 4 || $# == 5 ]] || exit 49
[[ "$1" == --consumer && "$2" == "$consumer" ]] || exit 49
case "${0##*/}" in
    install-host-tools.sh) toolset=host; catalog=--versions; file=tool-versions.env ;;
    install-ic-tools.sh) toolset=ic; catalog=--pins; file=ic-tools.tsv ;;
    install-rust-tools.sh) toolset=rust; catalog=--versions; file=tool-versions.env ;;
    *) exit 49 ;;
esac
[[ "$3" == "$catalog" && "$4" == "$consumer/ci/$file" ]] || exit 49
case "${5:-}" in
    --preflight)
        [[ "$toolset" != host ]] || exit 49
        if [[ "$toolset" == rust ]]; then printf 'rust-preflight\n' >> .git/tool-events; fi ;;
    --check)
        phase="$toolset-tools-check"
        printf '%s\n' "$phase" >> .git/tool-events
        [[ "${TOOLING_COMMON_CHECK_FAILURE:-}" != "$phase" && "${TOOLING_CHECK_FAILURE:-0}" == 0 ]] || exit 1 ;;
    '')
        printf 'install-%s-tools\n' "$toolset" >> .git/tool-events
        [[ "${TOOLING_INSTALL_FAILURE:-0}" == 0 ]] || exit 1 ;;
    *) exit 49 ;;
esac
INSTALLER
    done
    cat > "$repository/scripts/ci/verify-shared-tooling-snapshot.sh" <<'VERIFY'
#!/usr/bin/env bash
exit 0
VERIFY
    # Keep the production Makefile. Substitute only the validation runner so the
    # real release adapters and CI admission route cannot drift behind a replica.
    cat > "$repository/scripts/ci/run-validation-targets.sh" <<'VALIDATE'
#!/usr/bin/env bash
set -euo pipefail
[[ "${1:-}" == --fail-fast ]] || exit 49
shift
while [[ "${1:-}" == --input ]]; do
    [[ $# -ge 2 && -n "$2" ]] || exit 49
    shift 2
done
if [[ $# == 1 && "$1" == ci ]]; then
    [[ "${TOOLING_GATE_FAILURE:-0}" == 0 ]] || exit 1
    printf 'substitute gate\n' >> .git/gate-events
    exit 0
fi
[[ "${1:-}" == validation-tools-check ]] || exit 49
make --no-print-directory validation-tools-check
printf 'unexpected build\n' >> .git/validation-events
exit 49
VALIDATE
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
        mkdir "$fixture/metadata-errors-temp"
        # A version mismatch must stop before cache/setup or metadata writes. The real
        # reader still validates the manifest; substitute Cargo rejects later effects.
        for operation in preflight prepare; do
            : > "$TOOLING_FETCH_EVENTS"
            refuse "mismatched-$operation" env RELEASE_PREVIOUS=9.9.9 RELEASE_VERSION=9.9.10 \
                RELEASE_DATE=2026-10-10 TOOLING_FETCH_FAILURE=49 \
                TMPDIR="$fixture/metadata-errors-temp" "$BASH" scripts/release/metadata.sh "$operation"
            [[ "$refusal_status" == 1 && ! -s "$TOOLING_FETCH_EVENTS" ]] || exit 1
            [[ -z "$(git status --porcelain)" && -z "$(ls -A "$fixture/metadata-errors-temp")" ]] || exit 1
        done
        refuse incomplete-prepare env -u RELEASE_VERSION RELEASE_PREVIOUS="$base_version" \
            TMPDIR="$fixture/metadata-errors-temp" bash scripts/release/metadata.sh prepare
        [[ "$refusal_status" == 1 && -z "$(git status --porcelain)" ]] || exit 1
        # Shared validates required inputs before creating a mutation backup.
        [[ -z "$(ls -A "$fixture/metadata-errors-temp")" ]] || exit 1
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
        # Shared owns the exhaustive flag/control matrix. Keep each Jobs effect
        # family plus inherited/cleared/replaced/erased controls at this boundary.
        for selection in \
            'release-patch -n direct' 'release-minor -n direct' \
            'release-major -n direct' 'release-resume -n direct' \
            'fmt -n direct' 'fmt-check -n direct' 'publish -n direct' \
            'release-patch -i inherited' 'release-patch -q cleared' \
            'release-patch -t replaced' 'release-patch -kin erased-mflags'; do
            IFS=' ' read -r target mode controls <<< "$selection"
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
            [[ "$refusal_status" == 2 ]] || exit 1
            if [[ "$controls" == erased-mflags ]]; then
                grep -F "generated MFLAGS" "$fixture/$label.log" > /dev/null
            else
                grep -F 'Make recipe execution and failure propagation' "$fixture/$label.log" > /dev/null
            fi
            cmp .git/index "$fixture/mode-index"
            [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" ]] || exit 1
            [[ ! -e .git/gate-events && ! -s "$TOOLING_FETCH_EVENTS" ]] || exit 1
            [[ ! -e .git/release-state ]] || exit 1
            git diff --exit-code HEAD -- > /dev/null
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
    grep -F 'consumer does not support PR delivery' "$fixture/refused-pr-$kind.log" > /dev/null
    refuse "refused-pr-adapter-$kind" env RELEASE_DELIVERY=pr bash scripts/release/metadata.sh preflight
    grep -F 'consumer does not support PR delivery' "$fixture/refused-pr-adapter-$kind.log" > /dev/null
    if [[ "$kind" == patch ]]; then
        # Shared mechanics cannot turn a caller-supplied override into Jobs policy.
        refuse refused-policy-override make --no-print-directory release-patch \
            RELEASE_DELIVERY=pr RELEASE_DELIVERY_POLICY=
        grep -F 'consumer does not support PR delivery' "$fixture/refused-policy-override.log" > /dev/null
    fi
    [[ "$(git rev-parse HEAD)" == "$base" && ! -e .git/gate-events && ! -s "$TOOLING_FETCH_EVENTS" ]] || exit 1
    git diff --exit-code HEAD -- > /dev/null
    if [[ "$kind" == patch ]]; then
        # Exercise Jobs' preflight boundary, not only the common heading parser:
        # invalid candidates must refuse before cache/tool/gate or intent effects.
        for heading in draft misplaced; do
            if [[ "$heading" == draft ]]; then
                printf '# Changelog\n\n## [Draft]\n\n- Invalid candidate.\n' > CHANGELOG.md
            else
                printf '# Changelog\n\n## [%s] - 2026-10-09\n\n## [%s]\n\n- Misplaced candidate.\n' \
                    "$base_version" "$candidate" > CHANGELOG.md
            fi
            cp CHANGELOG.md "$fixture/invalid-heading-before"
            refuse "invalid-heading-$heading" make --no-print-directory release-patch
            cmp CHANGELOG.md "$fixture/invalid-heading-before"
            [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" && ! -e .git/gate-events ]] || exit 1
            [[ ! -s "$TOOLING_FETCH_EVENTS" && ! -e "$TOOLING_CACHE_READY" ]] || exit 1
            [[ ! -e ".git/release-state/$candidate.plan" ]] || exit 1
            git show HEAD:CHANGELOG.md > CHANGELOG.md
        done
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
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" && ! -e .git/gate-events ]] || exit 1
        [[ ! -s "$TOOLING_FETCH_EVENTS" && ! -e "$TOOLING_CACHE_READY" ]] || exit 1
        [[ ! -e .git/tool-events ]] || exit 1
        git restore --source=HEAD --staged --worktree -- README.md LICENSE
        rm "$unusual"

        # Shared's release-source fixture owns literal allowances and rename
        # preservation. Keep Jobs' actual adapter observation boundary below.
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
        [[ ! -e .git/gate-events && -z "$(git status --porcelain)" ]] || exit 1

        # Shared owns the cache matrix. Jobs retains actual fetch refusal,
        # source/index custody and selected Make execution through its adapter.
        cp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cp "$path" "$fixture/cache-$path"; done
        preflight=(env RELEASE_PREVIOUS="$base_version" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-09)
        refuse failed-fetch "${preflight[@]}" CARGO_NET_OFFLINE=false TOOLING_FETCH_FAILURE=43 bash scripts/release/metadata.sh preflight
        [[ "$refusal_status" == 43 && ! -e "$TOOLING_CACHE_READY" ]] || exit 1
        grep -F 'fixture registry fetch failed' "$fixture/failed-fetch.log" > /dev/null
        cmp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/cache-$path"; done
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" && ! -e .git/gate-events ]] || exit 1
        [[ ! -e ".git/release-state/$candidate.plan" ]] || exit 1
        export TOOLING_REAL_MAKE
        TOOLING_REAL_MAKE="$(command -v make)"
        cat > "$fixture/selected make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> .git/selected-make-events
exec "$TOOLING_REAL_MAKE" "$@"
MAKE
        chmod +x "$fixture/selected make"
        "${preflight[@]}" CARGO_NET_OFFLINE=false RELEASE_MAKE="$fixture/selected make" bash scripts/release/metadata.sh preflight
        [[ -f "$TOOLING_CACHE_READY" ]] || exit 1
        printf '%s\n' "--no-print-directory -C $repository install-tools" \
            "--no-print-directory -C $repository tools-check" > "$fixture/expected-make"
        cmp .git/selected-make-events "$fixture/expected-make"
        [[ "$(wc -l < "$TOOLING_FETCH_EVENTS" | tr -d ' ')" == 2 ]] || exit 1
        cmp .git/index "$fixture/cache-index"
        for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/cache-$path"; done
        [[ ! -e .git/gate-events && ! -e ".git/release-state/$candidate.plan" ]] || exit 1
        printf 'rust-preflight\ninstall-host-tools\ninstall-rust-tools\ninstall-ic-tools\nhost-tools-check\nrust-tools-check\nic-tools-check\n' > "$fixture/expected-tools"
        cmp .git/tool-events "$fixture/expected-tools"
        # Shared owns every common leaf failure; Jobs retains late admission
        # refusal through its real parallel CI route before product dispatch.
        refuse missing-rust-parallel-ci env TOOLING_COMMON_CHECK_FAILURE=rust-tools-check \
            make -j4 --no-print-directory ci
        [[ ! -e .git/validation-events && ! -e .git/gate-events ]] || exit 1
        # Both preparation boundaries stop before gate or metadata mutation.
        for failure in install check; do
            if [[ "$failure" == install ]]; then controls=(TOOLING_INSTALL_FAILURE=1);
            else controls=(TOOLING_CHECK_FAILURE=1); fi
            refuse "failed-tool-$failure" env "${controls[@]}" make --no-print-directory release-patch
            [[ ! -e .git/gate-events && -z "$(git tag)" ]] || exit 1
            for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/cache-$path"; done
        done

        refuse failed-gate env TOOLING_GATE_FAILURE=1 make --no-print-directory release-patch
        [[ "$(git rev-parse HEAD)" == "$base" && -z "$(git tag)" ]] || exit 1
        [[ "$(bash scripts/release/metadata.sh version)" == "$base_version" ]] || exit 1
        [[ -z "$(git status --porcelain)" ]] || exit 1
    fi
    # The full offline gate runs fixtures against prepared dependencies. Model
    # that input for the ordinary release series; cold-cache cases above select
    # their offline/online outcomes explicitly without any actual fetch.
    if [[ "${CARGO_NET_OFFLINE:-false}" == true ]]; then touch "$TOOLING_CACHE_READY"; fi
    make --no-print-directory "release-$kind" > "$fixture/release-$kind.log" 2>&1
    [[ "$(bash scripts/release/metadata.sh version)" == "$candidate" ]] || exit 1
    [[ "$(git log -1 --format=%s)" == "Release $candidate" ]] || exit 1
    [[ "$(git log -1 --format=%P)" == "$base" ]] || exit 1
    bash scripts/ci/check-release-tag.sh "$(git rev-parse HEAD)" "$candidate"
    [[ "$(git -C "$repository.git" rev-parse refs/heads/main)" == "$(git rev-parse HEAD)" ]] || exit 1
    [[ "$(git -C "$repository.git" rev-parse "refs/tags/v$candidate")" == "$(git rev-parse "refs/tags/v$candidate")" ]] || exit 1
    # Compare machine-owned paths in byte order, regardless of the caller's locale.
    [[ "$(git show --format= --name-only HEAD | LC_ALL=C sort)" == $'CHANGELOG.md\nCargo.lock\nCargo.toml' ]] || exit 1
    [[ -z "$(git status --porcelain)" && -f ".git/release-state/$candidate.plan" ]] || exit 1
    [[ "$(wc -l < .git/gate-events | tr -d ' ')" == 1 ]] || exit 1
    cp .git/tool-events "$fixture/resumed-tools-$kind"
    make --no-print-directory release-resume "VERSION=$candidate" > "$fixture/resume-$kind.log" 2>&1
    [[ "$(wc -l < .git/gate-events | tr -d ' ')" == 1 ]] || exit 1
    # Reconciliation of delivered intent must not replay preparation.
    cmp .git/tool-events "$fixture/resumed-tools-$kind"
    # Local-row version admission shares the identity parser and must refuse
    # before later dirty-source admission, without changing the selected graph.
    release_date="$(awk -v version="$candidate" '$1 == "##" && $2 == "[" version "]" { print $4; exit }' CHANGELOG.md)"
    cp Cargo.lock .git/lock-before
    perl scripts/ci/rewrite-local-lock-versions.pl Cargo.lock "$candidate" 0.0.0 ic-jobs > .git/invalid.lock
    cp .git/invalid.lock Cargo.lock
    refuse "wrong-local-version-$kind" env RELEASE_VERSION="$candidate" RELEASE_DATE="$release_date" bash scripts/release/metadata.sh check
    grep -F 'invalid coordinated workspace lock identities' "$fixture/wrong-local-version-$kind.log" > /dev/null
    cp .git/lock-before Cargo.lock
    [[ -z "$(git status --porcelain)" ]] || exit 1
done

# The adapter checks the same grouped manifests as fmt-check, including when
# release reconciliation selects a commit instead of the working tree.
new_repository manifest-order
printf '# Changelog\n\n## [%s] - 2026-10-09\n\n- Fixture release.\n' "$base_version" > CHANGELOG.md
commit_source
sorted_commit="$(git rev-parse HEAD)"
cp Cargo.toml .git/sorted-manifest
perl scripts/ci/prepare-cargo-formatting-fixture.pl \
    Cargo.toml workspace.dependencies ic-build ic-host-fs > .git/unsorted-manifest
cp .git/unsorted-manifest Cargo.toml
refuse unsorted-format-check make --no-print-directory fmt-check
cmp .git/unsorted-manifest Cargo.toml
refuse unsorted-release-check env RELEASE_VERSION="$base_version" RELEASE_DATE=2026-10-09 \
    bash scripts/release/metadata.sh check
cmp .git/unsorted-manifest Cargo.toml
# A sorted selected commit must pass even when the working manifest is unsorted.
RELEASE_VERSION="$base_version" RELEASE_DATE=2026-10-09 RELEASE_COMMIT="$sorted_commit" \
    bash scripts/release/metadata.sh check > "$fixture/sorted-selected-commit.log" 2>&1
git add -- Cargo.toml
git commit --quiet -m 'Fixture unsorted dependency group'
unsorted_commit="$(git rev-parse HEAD)"
cp .git/sorted-manifest Cargo.toml
refuse unsorted-selected-commit env RELEASE_VERSION="$base_version" RELEASE_DATE=2026-10-09 \
    RELEASE_COMMIT="$unsorted_commit" bash scripts/release/metadata.sh check
cmp .git/sorted-manifest Cargo.toml

# The actual publication adapter and shared helpers use real local Git. Only
# Cargo's upload and curl's registry observation are substituted.
new_repository publication
commit_source
tag="refs/tags/v$base_version"
git tag -a "v$base_version" -m 'Fixture release'
git push --quiet origin "$tag"
# Keep the remote's ordinary path while exercising a checkout whose name contains
# ordinary spaces. CR/LF checkouts are outside the reviewed baseline.
publication_checkout="$fixture/publication checkout with spaces"
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
exit 0
CURL
chmod +x "$fixture/bin/cargo" "$fixture/bin/curl"
export PATH="$fixture/bin:$PATH"

# The local real-Git publication fixture owns the source/tag/destination
# matrix. Keep representative Jobs adapter refusals, exact upload dispatch and
# reconciliation after an interrupted upload below.
printf '\nFixture unstaged change.\n' >> README.md
refuse unstaged-source bash scripts/release/publish.sh origin
git restore --source=HEAD --worktree -- README.md

refuse make-mode-n env MAKEFLAGS=-n bash scripts/release/publish.sh origin
[[ ! -s "$TOOLING_UPLOAD_EVENTS" ]] || exit 1

bash scripts/release/publish.sh origin > "$fixture/publication.log" 2>&1
[[ "$(cat "$TOOLING_UPLOAD_EVENTS")" == 'publish -p ic-jobs --all-features --locked --registry crates-io' ]] || exit 1
: > "$TOOLING_UPLOAD_EVENTS"
refuse failed-upload env TOOLING_UPLOAD_RESULT=42 bash scripts/release/publish.sh origin
[[ "$refusal_status" == 42 ]] || exit 1
[[ "$(wc -l < "$TOOLING_UPLOAD_EVENTS" | tr -d ' ')" == 1 ]] || exit 1
refuse uncertain-upload-now-present env TOOLING_REGISTRY_HTTP=200 bash scripts/release/publish.sh origin
[[ "$(wc -l < "$TOOLING_UPLOAD_EVENTS" | tr -d ' ')" == 1 ]] || exit 1
[[ -z "$(git status --porcelain)" ]] || exit 1
if grep -E 'warning: (overriding recipe|ignoring old recipe)' "$fixture/"*.log; then
    echo 'release fixture emitted unexpected recipe warnings' >&2
    exit 1
fi
echo 'Release and publication fixtures passed (real local Git; substitute gate and registry/upload effects)'
fixture_complete=true
