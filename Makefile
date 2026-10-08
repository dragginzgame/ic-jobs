.DEFAULT_GOAL := help
RELEASE_REMOTE ?= origin
RELEASE_BRANCH ?= main
export RELEASE_DELIVERY ?= direct
include make/tools.mk

install-tools: install-rust-tools install-release-tools
tools-check: rust-tools-check release-tools-check

.PHONY: help version install-hooks install-release-tools release-tools-check format-tools-check fmt fmt-check check test-jobs check-msrv check-wasm clippy docs-check check-doc-links shared-tooling-check check-pins check-release-commands test-release-tooling package publish-check publish ci
help:
	@echo "Focused: check, test-jobs, check-msrv, check-wasm, clippy, docs-check, fmt-check"
	@echo "Metadata: shared-tooling-check, check-doc-links, check-pins, check-release-commands"
	@echo "Release tooling: version, release-tools-check, test-release-tooling"
	@echo "Registry preparation: package (offline), publish-check (registry dry run; no upload)"
	@echo "Explicit setup: install-tools, tools-check, install-hooks"
	@echo "Complete gate: ci (explicit request or configured CI)"
	@echo "Authorized release: release-patch, release-minor, release-major (requires a configured remote)"
	@echo "Authorized publication: publish (clean HEAD, matching pushed annotated tag; crates.io)"
	@echo "Recovery: rerun the release target, or release-resume VERSION=X.Y.Z"

install-release-tools:
	@. ci/release-tools.env && cargo install cargo-edit --version "=$$IC_JOBS_CARGO_EDIT_VERSION" --locked \
		--no-default-features --features set-version --root "$(CURDIR)/.tools/rust" --target-dir "$(CURDIR)/.tools/rust/build"

release-tools-check:
	@. ci/release-tools.env && actual="$$(cargo set-version --version)" && test "$$actual" = "cargo-edit-set-version $$IC_JOBS_CARGO_EDIT_VERSION" || \
		{ echo 'Missing or mismatched cargo-edit; run make install-release-tools' >&2; exit 1; }

install-hooks:
	bash scripts/dev/install-git-hooks.sh

format-tools-check:
	@. ci/tool-versions.env && bash scripts/ci/check-format-tools.sh "$${SHARED_TOOLING_CARGO_SORT_VERSION:?}"

fmt: format-tools-check
	cargo sort --workspace
	cargo fmt --all

fmt-check: format-tools-check
	cargo sort --workspace --check
	cargo fmt --all -- --check

check:
	cargo check -p ic-jobs --all-targets --all-features --locked --offline

test-jobs:
	cargo test -p ic-jobs --test jobs --all-features --locked --offline

check-msrv:
	cargo +1.85.0 check -p ic-jobs --lib --locked --offline
	cargo +1.88.0 check -p ic-jobs --lib --all-features --locked --offline

check-wasm:
	cargo +1.85.0 check -p ic-jobs --lib --target wasm32-unknown-unknown --locked --offline
	cargo +1.88.0 check -p ic-jobs --lib --all-features --target wasm32-unknown-unknown --locked --offline

clippy:
	cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc -p ic-jobs --all-features --locked --offline --no-deps
	cargo test -p ic-jobs --doc --all-features --locked --offline

shared-tooling-check:
	bash scripts/ci/verify-shared-tooling-snapshot.sh

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root . *.md docs/*.md docs/principles/*.md docs/status/*.md rules/*.md audits/*.md tasks/*.md

check-pins:
	bash scripts/ci/check-dependency-pins.sh --cargo-inheritance

check-release-commands:
	bash scripts/ci/check-release-commands.sh . make/tools.mk

test-release-tooling:
	bash scripts/release/test-tooling.sh

package:
	cargo package -p ic-jobs --all-features --locked --offline --allow-dirty

publish-check:
	cargo publish -p ic-jobs --all-features --locked --registry crates-io --allow-dirty --dry-run

publish:
	+@bash scripts/release/publish.sh "$(RELEASE_REMOTE)"

ci: shared-tooling-check check-doc-links check-pins check-release-commands test-release-tooling fmt-check check test-jobs check-msrv check-wasm clippy docs-check package

.PHONY: release-patch release-minor release-major release-resume release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
ifneq ($(word 2,$(filter release-patch release-minor release-major release-resume,$(MAKECMDGOALS))),)
$(error Select exactly one release target)
endif
_release_targets := release-patch release-minor release-major release-resume release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
ifneq ($(filter $(_release_targets),$(MAKECMDGOALS)),)
ifneq ($(RELEASE_DELIVERY),direct)
$(error ic-jobs supports direct release delivery only)
endif
endif

release-patch release-minor release-major:
	+@bash scripts/ci/run-release.sh "$(@:release-%=%)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

release-resume:
	+@bash scripts/ci/run-release.sh resume "$(VERSION)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

version release-version:
	@bash scripts/release/metadata.sh version

release-preflight: release-tools-check
	@bash scripts/release/metadata.sh preflight

release-verify:
	+CARGO_NET_OFFLINE=true VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" bash scripts/ci/run-validation-targets.sh --fail-fast ci

release-prepare-version:
	@bash scripts/release/metadata.sh prepare

release-prepared-check release-committed-check release-tagged-check release-push-check:
	@bash scripts/release/metadata.sh check

release-commit-check:
	@bash scripts/release/metadata.sh commit-check

release-files:
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md
