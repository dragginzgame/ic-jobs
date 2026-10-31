#!/usr/bin/env bash
set -euo pipefail
mode="${1:-all}"
case "$mode" in all|retention|metadata|delivery|recovery|publication) ;; *) exit 2 ;; esac
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
suite="$(fixture_create "${TMPDIR:-/tmp}/scheduling-release-groups.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$suite" "Release-group fixture retained"' EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_COMMIT RELEASE_VERSION RELEASE_PREVIOUS RELEASE_DATE RELEASE_PREPARATION_SOURCE RELEASE_GROUP
export GROUP_TEST_REAL_MAKE GROUP_TEST_REAL_EDITOR GROUP_TEST_REAL_CARGO
GROUP_TEST_REAL_MAKE="$(command -v make)"
GROUP_TEST_REAL_CARGO="$(command -v cargo)"
GROUP_TEST_REAL_EDITOR="${CARGO_RELEASE_METADATA_EDITOR:-}"
if [[ -z "$GROUP_TEST_REAL_EDITOR" ]]; then
    GROUP_TEST_REAL_EDITOR="$(make --silent --no-print-directory -C "$root" release-tools-check)"
fi
mkdir "$suite/source" "$suite/bin"
cp "$root"/{Cargo.toml,Cargo.lock,CHANGELOG.md,README.md,LICENSE,Makefile,.gitignore,.shared-tooling.snapshot,rust-toolchain.toml} "$suite/source/"
cp -R "$root"/{crates,apps,scripts,make,ci,docs} "$suite/source/"
cat > "$suite/bin/editor" <<'EDITOR'
#!/usr/bin/env bash
set -euo pipefail
if [[ ( "${GROUP_TEST_EDITOR_FAILURE:-}" == manifest && "$*" == *group-manifest* ) ||
      ( "${GROUP_TEST_EDITOR_FAILURE:-}" == lock && "$*" == *' lock '* ) ]]; then
    echo 'partial invalid candidate'
    exit 37
fi
exec "$GROUP_TEST_REAL_EDITOR" "$@"
EDITOR
cat > "$suite/bin/make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
    case "$arg" in
        RELEASE_SOURCE=*|RELEASE_PREVIOUS=*|RELEASE_VERSION=*|RELEASE_DATE=*|RELEASE_REMOTE=*|RELEASE_BRANCH=*|RELEASE_COMMIT=*|RELEASE_GROUP=*|RELEASE_PREPARATION_SOURCE=*) export "$arg" ;;
    esac
done
for arg in "$@"; do
    case "$arg" in
        release-verify)
            printf '%s\n' "${RELEASE_GROUP:?}" >> "$GROUP_TEST_EVENTS"
            bash scripts/release/metadata.sh validated
            exit 0 ;;
        install-tools|tools-check|install-release-tools) exit 0 ;;
        release-tools-check) printf '%s\n' "$GROUP_TEST_EDITOR"; exit 0 ;;
        release-push-check)
            if [[ "${GROUP_TEST_PUSH_FAILURE:-0}" == 1 ]]; then exit 37; fi ;;
    esac
done
exec "$GROUP_TEST_REAL_MAKE" "$@"
MAKE
cat > "$suite/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == clippy ]]; then exit 0; fi
if [[ "${1:-}" == publish || "${1:-}" == package ]]; then
    printf '%s\n' "$*" >> "$GROUP_TEST_UPLOADS"
    exit 0
fi
exec "$GROUP_TEST_REAL_CARGO" "$@"
CARGO
chmod +x "$suite/bin/"*
export PATH="$suite/bin:$PATH" GROUP_TEST_EDITOR="$suite/bin/editor"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
for selected in jobs timers; do
    repo="$suite/$selected"
    cp -R "$suite/source" "$repo"
    (
        cd "$repo"
        export RELEASE_GROUP="$selected" RELEASE_MAKE="$suite/bin/make"
        export GROUP_TEST_EVENTS="$suite/$selected-gates" GROUP_TEST_UPLOADS="$suite/$selected-uploads"
        : > "$GROUP_TEST_EVENTS"; : > "$GROUP_TEST_UPLOADS"
        ledger=CHANGELOG.md; other_ledger=crates/ic-timers/CHANGELOG.md; package=ic-jobs; other=ic-timers
        if [[ "$selected" == timers ]]; then
            ledger=crates/ic-timers/CHANGELOG.md; other_ledger=CHANGELOG.md; package=ic-timers; other=ic-jobs
        fi
        previous="$(bash scripts/release/group.sh version)"
        candidate="$(bash scripts/ci/next-release-version.sh "$previous" patch)"
        other_group=timers; [[ "$selected" != timers ]] || other_group='jobs'
        other_version="$(RELEASE_GROUP="$other_group" bash scripts/release/group.sh version)"
        printf '# Fixture ledger\n\n## [%s] - Unreleased\n\n### 🩹 Fixed\n\n- Fixture group preparation.\n' "$candidate" > "$ledger"
        git init -q -b main
        git config user.name 'Scheduling fixture'; git config user.email 'fixture@example.invalid'
        git config core.hooksPath /dev/null
        git add -- .; git commit -qm baseline
        base="$(git rev-parse HEAD)"
        export RELEASE_SOURCE="$base" RELEASE_PREVIOUS="$previous" RELEASE_VERSION="$candidate"
        export RELEASE_DATE=2026-10-31 RELEASE_REMOTE=origin RELEASE_BRANCH=main
        git init -q --bare "$suite/$selected-remote.git"
        git remote add origin "$suite/$selected-remote.git"
        git push -q origin main
        plan="$(bash scripts/release/group.sh plan)"
        [[ "$(printf '%s\n' "$plan" | jq -r .primary)" == "$package" ]]
        [[ "$(make -s release-version)" == "$previous" ]]
        printf '%s\n' "$plan" > "$suite/$selected-plan.json"
        bash scripts/release/group.sh files > "$suite/$selected-files"
        cp "$other_ledger" "$suite/$selected-other-ledger"
        cargo metadata --locked --offline --format-version 1 > "$suite/$selected-before.json"
        requirement="$(yq -p toml -o yaml -r '.workspace.dependencies.ic-timers.version' Cargo.toml)"
        if [[ "$mode" == all || "$mode" == metadata || "$mode" == retention ]]; then
            bash scripts/release/metadata.sh validated
            receipts=".git/release-state/$selected-$candidate.receipts"
            cp "$receipts/validated" "$suite/$selected-valid-receipt"
            printf 'tampered\n' >> "$receipts/validated"
            if bash scripts/release/metadata.sh prepare > "$suite/$selected-tampered-receipt.log" 2>&1; then exit 1; fi
            git diff --exit-code HEAD -- > /dev/null
            cp "$suite/$selected-valid-receipt" "$receipts/validated"
            for failure in manifest lock; do
                status=0
                GROUP_TEST_EDITOR_FAILURE="$failure" RELEASE_PREVIOUS="$previous" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-31 \
                    RELEASE_PREPARATION_SOURCE="$base" bash scripts/release/metadata.sh prepare > "$suite/$selected-rollback-$failure.log" 2>&1 || status=$?
                [[ "$status" != 0 ]] || exit 1
                git diff --exit-code HEAD -- > /dev/null
            done
        fi
        if [[ "$mode" == metadata || "$mode" == retention ]]; then
            RELEASE_PREVIOUS="$previous" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-31 \
                RELEASE_PREPARATION_SOURCE="$base" bash scripts/release/metadata.sh prepare > "$suite/$selected-prepare.log" 2>&1
            RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-31 bash scripts/release/metadata.sh check
        else
            status=0
            GROUP_TEST_PUSH_FAILURE=1 make release-patch RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-release.log" 2>&1 || status=$?
            [[ "$status" != 0 ]] || exit 1
            [[ "$(cat "$GROUP_TEST_EVENTS")" == "$selected" ]]
            [[ -f ".git/release-state/$selected-$candidate.plan" ]]
            receipts=".git/release-state/$selected-$candidate.receipts"
            [[ -f "$receipts/validated" && -f "$receipts/prepared" ]]
            grep -Fx "$selected" ".git/release-state/$selected-$candidate.plan" > /dev/null
            cmp "$suite/$selected-files" ".git/release-state/$selected-$candidate.plan.files"
            # Resume must not accept another group's saved intent.
            cp ".git/release-state/$selected-$candidate.plan" ".git/release-state/$other_group-$candidate.plan"
            if make release-resume RELEASE_GROUP="$other_group" VERSION="$candidate" RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-wrong-resume.log" 2>&1; then exit 1; fi
            grep -F 'release group changed' "$suite/$selected-wrong-resume.log" > /dev/null
            rm ".git/release-state/$other_group-$candidate.plan"
            make release-resume VERSION="$candidate" RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-resume.log" 2>&1
            [[ "$(cat "$GROUP_TEST_EVENTS")" == "$selected" ]]
            git show --format= --name-only HEAD | LC_ALL=C sort > "$suite/$selected-committed"
            tr '\0' '\n' < "$suite/$selected-files" | LC_ALL=C sort > "$suite/$selected-expected"
            cmp "$suite/$selected-committed" "$suite/$selected-expected"
            [[ "$(git log -1 --format=%s)" == "Release $selected $candidate" ]]
            bash scripts/ci/check-release-tag.sh "$(git rev-parse HEAD)" "$candidate" "$selected"
            [[ "$(git --git-dir="$suite/$selected-remote.git" rev-parse main)" == "$(git rev-parse HEAD)" ]]
            # Registry observation is substituted; real Git/tag/source admission stays active.
            cat > scripts/ci/check-crates-io-version.sh <<'REGISTRY'
#!/usr/bin/env bash
exit "${GROUP_TEST_REGISTRY_STATUS:-1}"
REGISTRY
            git add -- scripts/ci/check-crates-io-version.sh
            git commit -qm 'fixture registry observation'
            # Move fixture-only tag to the fixture observation source.
            git tag -d "$selected-v$candidate" > /dev/null
            git tag -a "$selected-v$candidate" -m fixture
            git push -q origin main "+refs/tags/$selected-v$candidate"
            if GROUP_TEST_REGISTRY_STATUS=2 bash scripts/release/check-publication.sh origin > "$suite/$selected-unknown-registry.log" 2>&1; then exit 1; fi
            if RELEASE_GROUP="$other_group" bash scripts/release/check-publication.sh origin > "$suite/$selected-wrong-publication.log" 2>&1; then exit 1; fi
            touch unexpected-source
            if bash scripts/release/check-publication.sh origin > "$suite/$selected-dirty-publication.log" 2>&1; then exit 1; fi
            rm unexpected-source
            bash scripts/release/check-publication.sh origin
            bash scripts/release/publish.sh origin
            [[ "$(cat "$GROUP_TEST_UPLOADS")" == "publish -p $package --all-features --locked --registry crates-io" ]]
            : > "$GROUP_TEST_UPLOADS"
            make publish-check > "$suite/$selected-publish-check.log" 2>&1
            [[ "$(cat "$GROUP_TEST_UPLOADS")" == "publish -p $package --all-features --locked --registry crates-io --allow-dirty --dry-run" ]]
        fi
        cmp "$other_ledger" "$suite/$selected-other-ledger"
        cargo metadata --locked --offline --format-version 1 > "$suite/$selected-after.json"
        jq -e --arg other "$other" --arg version "$other_version" '.packages[] | select(.name == $other and .source == null) | .version == $version' "$suite/$selected-after.json" > /dev/null
        if [[ "$selected" == jobs ]]; then
            [[ "$(yq -p toml -o yaml -r '.workspace.dependencies.ic-timers.version' Cargo.toml)" == "$requirement" ]]
        else
            [[ "$(yq -p toml -o yaml -r '.workspace.package.version' Cargo.toml)" == "$other_version" ]]
            [[ "$(yq -p toml -o yaml -r '.workspace.dependencies.ic-timers.version' Cargo.toml)" == "$candidate" ]]
            # The unchanged standalone Jobs consumer still builds against Jobs
            # its retained version after the independent Timers upgrade.
            CARGO_TARGET_DIR="$suite/jobs-consumer-target" bash scripts/dev/minimum-rust.sh cargo check \
                -p ic-jobs --example persisted_job --locked --offline
        fi
        printf '%s group %s passed (real Cargo metadata/editor/Git; substituted full gate, tool setup and upload)\n' "$selected" "$mode"
    )
done
fixture_complete=true
