.DEFAULT_GOAL := help
export RELEASE_DELIVERY ?= direct
override SHARED_TOOLING_ROOT := $(CURDIR)
override RELEASE_DELIVERY_POLICY := $(CURDIR)/ci/release-delivery.txt
# Optimized qualification uses Binaryen; no other IC executable is required.
IC_TOOLS := wasm-opt
LOCAL_TOOL_INSTALL_TARGETS := install-ic-tools
LOCAL_TOOL_CHECK_TARGETS := ic-tools-check
export PATH := $(CURDIR)/.tools/ic/bin:$(PATH)
include make/consumer.mk
TESTKIT_TOOL = bash scripts/dev/install-rust-tools.sh --package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server --profile release --features cli
RELEASE_METADATA_LOCKFILE ?= $(CURDIR)/Cargo.lock
RELEASE_METADATA_TOOL = bash scripts/dev/install-rust-tools.sh --package ic-build --lockfile "$(RELEASE_METADATA_LOCKFILE)" --bin ic-build-release-metadata --profile debug --features cli
VALIDATION_INPUTS := --input Cargo.toml --input Cargo.lock --input rust-toolchain.toml \
	--input .shared-tooling.snapshot --input Makefile --input ci/tool-versions.env \
	--input ci/ic-tools.tsv --input ci/release-delivery.txt --input ci/cargo-dependency-boundaries.json \
	--input crates/ic-jobs/Cargo.toml --input apps/job-consumer/Cargo.toml \
	--input crates/jobs-canister-tests/Cargo.toml

.PHONY: help version install-hooks check test-jobs test-consumers check-msrv check-wasm clippy docs-check check-doc-links shared-tooling-check check-pins check-release-commands test-release-tooling test-formatting-evidence package publish-check publish ci
help:
	@echo "Focused selections: see README (Consumer qualification) for package/test/target commands"
	@echo "Aggregates: check, test-jobs, test-consumers, check-msrv, check-wasm, clippy, docs-check, fmt-check"
	@echo "Consumer: check-consumer (native, Wasm, harness compilation); build-consumer (Wasm artifact)"
	@echo "Explicit IC qualification: install-testkit-tools; install-consumer-server; test-canister[-optimized]"
	@echo "Local canister queue: demo (interactive); test-demo (live scenario); test-demo-cli (process shutdown)"
	@echo "Metadata: shared-tooling-check, check-doc-links, check-pins, check-dependency-boundaries, check-dependency-graphs, check-release-commands"
	@echo "Tooling fixtures: version, test-release-tooling, test-formatting-evidence, test-consumer-tooling"
	@echo "Hook qualification: formatting-hook-check (disposable Git copies; prepared formatters required)"
	@echo "Registry preparation: package (offline), publish-check (registry dry run; no upload)"
	@echo "Explicit setup: install-tools, tools-check, install-hooks"
	@echo "Release editor: install-release-tools; release-tools-check (offline-admitted path)"
	@echo "Complete gate: ci (explicit request or configured CI)"
	@echo "Authorized release: release-patch, release-minor, release-major (requires a configured remote)"
	@echo "Authorized publication: publish (clean HEAD, matching pushed annotated tag; crates.io)"
	@echo "Recovery: release-resume VERSION=X.Y.Z (normal release targets start fresh)"

check:
	+cargo check -p ic-jobs --all-targets --all-features --locked --offline

test-jobs:
	+cargo test -p ic-jobs --tests --all-features --locked --offline

test-consumers:
	+cargo +1.91.0 test -p jobs-test-consumer --test composition --locked --offline

.PHONY: check-consumer build-consumer install-testkit-tools testkit-tools-check install-consumer-server consumer-server-check test-canister test-canister-optimized
check-consumer:
	+cargo +1.91.0 test -p jobs-test-consumer --lib --test api_wire --locked --offline
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.91.0 test -p jobs-canister-tests --bin jobs-capture-artifact --example queue --test queue_cli --test recovery --locked --offline

build-consumer:
	+cargo +1.91.0 build -p jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline

.PHONY: build-recovery-tools
build-recovery-tools:
	+cargo +1.91.0 build -p jobs-canister-tests --bin jobs-capture-artifact --locked --offline

install-testkit-tools:
	+$(TESTKIT_TOOL)

testkit-tools-check:
	+$(TESTKIT_TOOL) --check

install-consumer-server:
	+@cli="$$( $(TESTKIT_TOOL) --check )" && "$$cli" setup

consumer-server-check:
	+@cli="$$( $(TESTKIT_TOOL) --check )" && "$$cli" check

# Only this explicitly selected target launches PocketIC. Admission precedes
# artifact compilation; normal CI compiles the harness without executing it.
test-canister test-canister-optimized: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+$(MAKE) --no-print-directory build-recovery-tools
	+cargo +1.91.0 test -p jobs-canister-tests --test recovery --locked --offline --no-run
	+bash scripts/ci/test-canister.sh $(if $(filter test-canister-optimized,$@),--optimized)

.PHONY: demo test-demo test-demo-cli
demo: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.91.0 build -p jobs-canister-tests --example queue --locked --offline
	+@cli="$$( $(TESTKIT_TOOL) --check )" && \
		target="$$(cargo +1.91.0 metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)" && \
		server="$$( "$$cli" check )" && \
		"$$target/debug/examples/queue" --server "$$server" \
		--wasm "$$target/wasm32-unknown-unknown/release/jobs_test_consumer.wasm"

test-demo: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.91.0 test -p jobs-canister-tests --example queue --locked --offline --no-run
	+@cli="$$( $(TESTKIT_TOOL) --check )" && \
		target="$$(cargo +1.91.0 metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)" && \
		IC_JOBS_CONSUMER_WASM="$$target/wasm32-unknown-unknown/release/jobs_test_consumer.wasm" \
		"$$cli" run --ttl 900 -- cargo +1.91.0 test -p jobs-canister-tests \
		--example queue --locked --offline -- --ignored --nocapture --test-threads=1

# Spawn-mode process tests intentionally do not borrow the batch runner's server.
test-demo-cli: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+cargo +1.91.0 build -p jobs-canister-tests --example queue --locked --offline
	+@cli="$$( $(TESTKIT_TOOL) --check )" && \
		target="$$(cargo +1.91.0 metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)" && \
		IC_JOBS_QUEUE_SERVER="$$( "$$cli" check )" \
		IC_JOBS_QUEUE_BIN="$$target/debug/examples/queue" \
		IC_JOBS_CONSUMER_WASM="$$target/wasm32-unknown-unknown/release/jobs_test_consumer.wasm" \
		cargo +1.91.0 test -p jobs-canister-tests --test queue_cli --locked --offline -- --ignored --nocapture

check-msrv:
	+cargo +1.91.0 check -p ic-jobs --lib --locked --offline
	+cargo +1.91.0 check -p jobs-test-consumer --test composition --locked --offline

check-wasm:
	+cargo +1.91.0 check -p ic-jobs --lib --target wasm32-unknown-unknown --locked --offline
	+cargo +1.91.0 check -p jobs-test-consumer --lib --target wasm32-unknown-unknown --locked --offline

clippy:
	+cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings
	+cargo clippy -p jobs-test-consumer -p jobs-canister-tests --all-targets --locked --offline -- -D warnings
	+cargo clippy -p jobs-test-consumer --target wasm32-unknown-unknown --locked --offline -- -D warnings

docs-check:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-jobs --all-features --locked --offline --no-deps
	+cargo test -p ic-jobs --doc --all-features --locked --offline

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root . *.md docs/*.md docs/principles/*.md docs/status/*.md rules/*.md audits/*.md tasks/*.md $(wildcard apps/*/README.md apps/*/tests/fixtures/README.md crates/*/README.md crates/*/tests/fixtures/README.md)

check-pins:
	+bash scripts/ci/check-dependency-pins.sh --cargo-inheritance

.PHONY: check-dependency-boundaries
check-dependency-boundaries:
	+bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)" --external-ic

# Jobs owns these concrete exclusions; Shared owns observation and matching.
CORE_GRAPH_FORBIDDEN := clap ctrlc candid candid_derive candid_parser ic-cdk ic-cdk-macros \
	ic-cdk-executor ic-cdk-timers ic-platform ic-platform-macros ic0 ic-memory ic-stable-structures ic-storage \
	ic-timers ic-metrics ic-wire ic-wire-derive postcard ic-build ic-host-fs ic-host-process ic-testkit \
	ic-testkit-candid ic-testkit-native jobs-test-consumer pocket-ic serde_json serde_test
QUALIFICATION_GRAPH_FORBIDDEN := clap ctrlc ic-storage candid candid_derive candid_parser ic-cdk ic-agent ic-build ic-host-fs \
	ic-host-process ic-host-artifacts ic-host-text ic-testkit ic-testkit-candid ic-testkit-native pocket-ic tokio
# Timers and the fixture share Platform 0.6. Keep one wrapper, executor,
# raw-system provider and entrypoint generator in the deployed graph.
.PHONY: check-dependency-graphs
check-dependency-graphs:
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target wasm32-unknown-unknown --all-features \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package jobs-test-consumer \
		--target wasm32-unknown-unknown --all-features \
		--require ic-timers=1 --require ic-cdk-executor=1 --require ic0=1 \
		--require ic-platform=1 --require ic-platform-macros=1 \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package jobs-test-consumer \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid ic-cdk --forbid ic-platform-macros --forbid-feature ic-platform/cdk \
		--forbid ic-jobs --forbid ic-memory --forbid ic-timers --forbid serde_json \
		--forbid ic-stable-structures --forbid ic-cdk-executor --forbid ic0 --forbid ic-metrics \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package jobs-canister-tests \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid candid --forbid candid_derive --forbid candid_parser \
		--forbid pocket-ic --forbid ic-testkit --forbid ctrlc --forbid-feature ic-platform/candid \
		--forbid-feature ic-platform/pocket-ic

check-release-commands:
	bash scripts/ci/check-release-commands.sh . make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh scripts/ci/check-release-delivery.sh ci/release-delivery.txt

test-release-tooling: format-tools-check
	+@editor="$$( $(RELEASE_METADATA_TOOL) --check )" && \
		CARGO_RELEASE_METADATA_EDITOR="$$editor" CARGO_RELEASE_METADATA_MAX_BYTES=1048576 \
		bash scripts/release/test-tooling.sh
	+bash scripts/release/test-publication.sh

.PHONY: install-release-tools release-tools-check
install-release-tools:
	+$(RELEASE_METADATA_TOOL)

release-tools-check:
	+@$(RELEASE_METADATA_TOOL) --check

test-formatting-evidence:
	+bash scripts/ci/test-formatting-evidence.sh

.PHONY: test-consumer-tooling
test-consumer-tooling: build-recovery-tools
	+bash scripts/ci/test-consumer-tooling.sh

.PHONY: formatting-hook-check
formatting-hook-check: format-tools-check
	+bash scripts/ci/check-jobs-formatting.sh

package:
	+cargo package -p ic-jobs --all-features --locked --offline --allow-dirty

publish-check:
	+$(MAKE) --no-print-directory clippy
	+cargo publish -p ic-jobs --all-features --locked --registry crates-io --allow-dirty --dry-run

publish:
	+@bash scripts/release/publish.sh "$(RELEASE_REMOTE)"

.PHONY: validation-tools-check
validation-tools-check: tools-check format-tools-check

# Ordered recursive calls preserve admission before dependent work under -j.
ci: shared-tooling-check
	+bash scripts/ci/run-validation-targets.sh --fail-fast $(VALIDATION_INPUTS) \
		validation-tools-check clippy check-doc-links check-pins check-dependency-boundaries check-dependency-graphs check-release-commands \
		test-release-tooling test-formatting-evidence test-consumer-tooling \
		formatting-hook-check fmt-check check test-jobs test-consumers \
		check-consumer check-msrv check-wasm docs-check package

.PHONY: release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
version release-version:
	@bash scripts/release/metadata.sh version

release-preflight:
	+@bash scripts/release/metadata.sh preflight

release-verify:
	+CARGO_NET_OFFLINE=true VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" bash scripts/ci/run-validation-targets.sh --fail-fast $(VALIDATION_INPUTS) ci

release-prepare-version:
	+@bash scripts/release/metadata.sh prepare

release-prepared-check release-committed-check release-tagged-check release-push-check:
	+@bash scripts/release/metadata.sh check

release-commit-check:
	+@bash scripts/release/metadata.sh commit-check

release-files:
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md
