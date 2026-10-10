.DEFAULT_GOAL := help
export RELEASE_DELIVERY ?= direct
LOCAL_TOOL_INSTALL_TARGETS := install-release-tools
LOCAL_TOOL_CHECK_TARGETS := release-tools-check
include make/tools.mk
include make/release.mk
include make/rust-format.mk
export PATH := $(CURDIR)/.tools/rust/cargo-edit-$(shell . ci/release-tools.env && printf '%s' "$$IC_JOBS_CARGO_EDIT_VERSION")/bin:$(PATH)

.PHONY: help version install-hooks install-release-tools release-tools-check check test-jobs test-consumers check-msrv check-wasm clippy docs-check check-doc-links shared-tooling-check check-pins check-release-commands test-release-tooling test-formatting-evidence package publish-check publish ci
help:
	@echo "Focused: check, test-jobs, test-consumers, check-msrv, check-wasm, clippy, docs-check, fmt-check"
	@echo "Consumer: check-consumer (native, Wasm, harness compilation); build-consumer (Wasm artifact)"
	@echo "Explicit IC qualification: install-testkit-tools; install-consumer-server; test-canister[-optimized]"
	@echo "Metadata: shared-tooling-check, check-doc-links, check-pins, check-release-commands"
	@echo "Tooling fixtures: version, release-tools-check, test-release-tooling, test-formatting-evidence"
	@echo "Registry preparation: package (offline), publish-check (registry dry run; no upload)"
	@echo "Explicit setup: install-tools, tools-check, install-hooks"
	@echo "Complete gate: ci (explicit request or configured CI)"
	@echo "Authorized release: release-patch, release-minor, release-major (requires a configured remote)"
	@echo "Authorized publication: publish (clean HEAD, matching pushed annotated tag; crates.io)"
	@echo "Recovery: rerun the release target, or release-resume VERSION=X.Y.Z"

install-release-tools:
	+@. ci/release-tools.env && selected_root="$(CURDIR)/.tools/rust/cargo-edit-$$IC_JOBS_CARGO_EDIT_VERSION" && \
		if test -e "$$selected_root" || test -L "$$selected_root"; then \
			$(MAKE) --no-print-directory release-tools-check; \
		else \
			cargo install cargo-edit --version "=$$IC_JOBS_CARGO_EDIT_VERSION" --locked \
				--no-default-features --features set-version --root "$$selected_root" --target-dir "$(CURDIR)/.tools/rust/build"; \
		fi

release-tools-check:
	+@. ci/release-tools.env && test -x "$(CURDIR)/.tools/rust/cargo-edit-$$IC_JOBS_CARGO_EDIT_VERSION/bin/cargo-set-version" && actual="$$(cargo set-version --version)" && test "$$actual" = "cargo-edit-set-version $$IC_JOBS_CARGO_EDIT_VERSION" || \
		{ echo 'Missing or mismatched cargo-edit; run make install-release-tools' >&2; exit 1; }

install-hooks:
	bash scripts/dev/install-git-hooks.sh

check:
	+cargo check -p ic-jobs --all-targets --all-features --locked --offline

test-jobs:
	+cargo test -p ic-jobs --test jobs --all-features --locked --offline

test-consumers:
	+cargo +1.88.0 test -p ic-jobs --test consumers --no-default-features --locked --offline
	+cargo +1.88.0 test -p ic-jobs --test consumers --all-features --locked --offline

.PHONY: check-consumer build-consumer install-testkit-tools testkit-tools-check install-consumer-server consumer-server-check test-canister test-canister-optimized
check-consumer:
	+cargo +1.88.0 test -p jobs-test-consumer --locked --offline
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.88.0 test -p jobs-canister-tests --locked --offline --no-run
	+cargo clippy -p jobs-test-consumer -p jobs-canister-tests --all-targets --locked --offline -- -D warnings
	+cargo clippy -p jobs-test-consumer --target wasm32-unknown-unknown --locked --offline -- -D warnings

build-consumer:
	+cargo +1.88.0 build -p jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline

install-testkit-tools:
	+@. ci/testkit-tools.env && bash scripts/dev/install-rust-tools.sh --package ic-testkit --version "$$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release

testkit-tools-check:
	+@. ci/testkit-tools.env && bash scripts/dev/install-rust-tools.sh --package ic-testkit --version "$$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release --check

install-consumer-server: testkit-tools-check
	+@. ci/testkit-tools.env && cli="$$(bash scripts/dev/install-rust-tools.sh --package ic-testkit --version "$$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release --check)" && "$$cli" setup

consumer-server-check: testkit-tools-check
	+@. ci/testkit-tools.env && cli="$$(bash scripts/dev/install-rust-tools.sh --package ic-testkit --version "$$IC_JOBS_TESTKIT_VERSION" --bin ic-testkit-server --profile release --check)" && "$$cli" check

# Only this explicitly selected target launches PocketIC. Admission precedes
# artifact compilation; normal CI compiles the harness without executing it.
test-canister test-canister-optimized: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.88.0 test -p jobs-canister-tests --test recovery --locked --offline --no-run
	+bash scripts/ci/test-canister.sh $(if $(filter test-canister-optimized,$@),--optimized)

check-msrv:
	+cargo +1.88.0 check -p ic-jobs --lib --locked --offline
	+cargo +1.88.0 check -p ic-jobs --lib --all-features --locked --offline

check-wasm:
	+cargo +1.88.0 check -p ic-jobs --lib --target wasm32-unknown-unknown --locked --offline
	+cargo +1.88.0 check -p ic-jobs --lib --all-features --target wasm32-unknown-unknown --locked --offline

clippy:
	+cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings

docs-check:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-jobs --all-features --locked --offline --no-deps
	+cargo test -p ic-jobs --doc --all-features --locked --offline

shared-tooling-check:
	bash scripts/ci/verify-shared-tooling-snapshot.sh

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root . *.md docs/*.md docs/principles/*.md docs/status/*.md rules/*.md audits/*.md tasks/*.md apps/job-consumer/README.md

check-pins:
	+bash scripts/ci/check-dependency-pins.sh --cargo-inheritance

check-release-commands:
	bash scripts/ci/check-release-commands.sh . make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh

test-release-tooling: release-tools-check format-tools-check
	+bash scripts/release/test-tooling.sh

test-formatting-evidence:
	+bash scripts/ci/test-formatting-evidence.sh

package:
	+cargo package -p ic-jobs --all-features --locked --offline --allow-dirty

publish-check:
	+cargo publish -p ic-jobs --all-features --locked --registry crates-io --allow-dirty --dry-run

publish:
	+@bash scripts/release/publish.sh "$(RELEASE_REMOTE)"

.PHONY: validation-tools-check
validation-tools-check: tools-check format-tools-check

# Ordered recursive calls preserve admission before dependent work under -j.
ci: shared-tooling-check
	+$(MAKE) --no-print-directory validation-tools-check
	+$(MAKE) --no-print-directory check-doc-links check-pins check-release-commands test-release-tooling test-formatting-evidence fmt-check check test-jobs test-consumers check-consumer check-msrv check-wasm clippy docs-check package

.PHONY: release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
_release_targets := release-patch release-minor release-major release-resume release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
ifneq ($(filter $(_release_targets),$(MAKECMDGOALS)),)
ifneq ($(RELEASE_DELIVERY),direct)
$(error ic-jobs supports direct release delivery only)
endif
endif

version release-version:
	@bash scripts/release/metadata.sh version

release-preflight:
	+@bash scripts/release/metadata.sh preflight

release-verify:
	+CARGO_NET_OFFLINE=true VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" bash scripts/ci/run-validation-targets.sh --fail-fast ci

release-prepare-version:
	+@bash scripts/release/metadata.sh prepare

release-prepared-check release-committed-check release-tagged-check release-push-check:
	+@bash scripts/release/metadata.sh check

release-commit-check:
	+@bash scripts/release/metadata.sh commit-check

release-files:
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md
