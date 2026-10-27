# IC Jobs

IC Jobs provides durable job metadata and scheduling policies for Internet
Computer applications. Applications persist records, maintain their due-time
index and execute handlers; IC Timers owns volatile wakeups.

## Use

The [package API guide](crates/ic-jobs/README.md), also included in rustdoc,
contains usage examples, checked configuration, batch scheduling, timer
integration and migration requirements. The packaged guide remains self-contained.
See [SAFETY.md](SAFETY.md) for persistence and recovery obligations.

Run `cargo run -p ic-jobs --locked --example persisted_job` for a runnable
reconstruction example. The unpublished [canister consumer](apps/job-consumer/README.md)
demonstrates application-owned storage, management and lifecycle integration.
Its PocketIC harness is compiled by CI; live qualification is explicitly selected.

To queue jobs for seconds, minutes or weeks and see their current schedule, run
the [local canister queue demo](crates/jobs-canister-tests/README.md). `make demo`
opens a terminal connected to the real fixture canister in local PocketIC;
`watch` shows countdowns and `advance` fast-forwards IC time.

## Scope and validation

The library owns metadata and checked transitions. Applications own storage,
payloads, handlers, longer audit history and disposition of uncertain effects.
It supplies no global queue, database, lifecycle exports or exactly-once effects.

| Boundary | Owner and entry point |
| --- | --- |
| Validated records, attempts, transitions and due-time selection | Published `ic-jobs`: `Job::restore`, `Job` methods, `next_due_ns` and `start_next`; Serde is its only runtime dependency |
| Payloads, authorization, stable queue and lifecycle endpoints | Consumer applications; the unpublished `apps/job-consumer` demonstrates this integration |
| Stable allocation and fixture Cell admission | IC Memory; IC Storage supplies governed native test cells |
| Volatile wakeups and watchdog recovery | IC Timers, called directly by the consumer |
| Principal and explicit-codec canister runtime | Published IC Platform; Jobs retains endpoints, schemas, limits and error policy |
| Installation, management and delivery encoding and bounded decoding | Published IC Wire; Jobs retains method identities, limits and receipt policy |
| PocketIC transport and startup | IC Testkit; Jobs supplies Wire frames and retains scenario policy |
| Artifact retention and native filesystem/process mechanics | IC Build and IC Host; Jobs retains optimizer flags and evidence naming |
| Common setup, formatting and release mechanics | Reviewed Shared Tooling snapshot; Jobs owns its gates, direct-delivery policy and package upload admission |

## Dependency exposure

Root `[workspace.dependencies]` entries select versions; each member opts into
them. Their presence in the root catalog does not put them in every build.
The catalog labels `Production dependencies` and `Tests and tooling only`
describe use here, independently of registry publication. Serde is shared by
production and tests, so it belongs to production. The qualification Wasm and
capture/recovery harness are `Tests and tooling packages`; published `ic-jobs`
is the sole production package. Paired grouped sort commands preserve these
boundaries in `make fmt`, `make fmt-check` and the formatting hook. Release
metadata checks use the same grouped policy for working and selected commits.

| Built artifact | Candid exposure | CDK exposure |
| --- | --- | --- |
| Published `ic-jobs` library | None | None; normal dependencies contain only Serde |
| Qualification `jobs_test_consumer.wasm` | None; all owned inputs/replies use Wire | Platform's Wasm-only `cdk` runtime supplies entrypoints; upstream `ic-cdk` is absent, and Timers shares its registry executor |
| Native recovery tests and queue demo | PocketIC requires Candid for external simulator/management protocols; Jobs RPCs use Wire | Platform's `cdk` feature is enabled only for the qualification Wasm |
| `jobs-capture-artifact` host binary | No Candid dependency in its normal/build graph | No CDK dependency in its normal/build graph |

The Wasm above is an unpublished test fixture, not a production service. This
checkout does not establish what other deployed canisters use. Audit each actual
application's selected target graph and source rather than counting manifests.
For example, inspect the qualification Wasm's paths with:

```sh
cargo tree -p jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-platform@0.6 --locked --offline
cargo tree -p jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-cdk-executor --locked --offline
```

The selected published boundary is IC Platform, with defaults disabled.
The [workspace catalog](Cargo.toml) owns requirements; [Cargo.lock](Cargo.lock)
owns the resolved versions. The [handoff](docs/status/current.md) records the
selected adoption and its source-bound evidence.
The qualification app selects Platform's `cdk` feature only on Wasm. Native
consumer builds explicitly select Platform's `principal` feature for Serde DTOs
and Wire contracts. Jobs, Memory, Timers and JSON remain Wasm/test dependencies. Parser, Agent,
certification and crypto features are not selected. Platform 0.6 owns the reviewed
runtime adapters and generator; registry executor 2.0.0 and ic0 1.2.0 remain shared
with Timers. The qualification Wasm selects neither Candid nor full `ic-cdk`.

Installation, management, fixture controls and delivery use IC Wire native Serde
frames. Consumer-owned method identities and request/reply budgets live in
`management_wire` and `delivery_wire`. Platform's explicit hooks validate unit
RPC requests too; guards admit length before argument copies. The init hook
performs that admission explicitly because lifecycle macros have no guards.
No Candid service or compatibility endpoint is generated. Frozen earlier Candid
bytes are negative fixtures only.

This is an unpublished fixture protocol cut. Quiesce management/delivery activity,
dispose of outstanding old management requests and rebuild the fixture and native
harness together. Existing delivery identities and stored JSON do not change;
Running/Uncertain effects still require explicit disposition and exact evidence.
The [consumer guide](apps/job-consumer/README.md) describes the boundaries, and the
[handoff](docs/status/current.md) records qualification by source. Native Testkit
uses Platform 0.6 for external simulator interfaces. The harness keeps the
upstream Principal for PocketIC calls and converts explicitly to Platform 0.6's
owned Principal when constructing application DTOs. No decoder or transport is replaced.

For the production library, locked normal/build graphs on native Linux and Wasm,
with all features selected, contain Serde and its macro dependencies and exclude
every tests/tooling-only catalog dependency (apart from the selected root itself).
The qualification Wasm intentionally includes its test integrations above; those
cannot be prohibited there using the production library's exclusion list.

`make check-dependency-boundaries` runs the reviewed shared checker and is part of
CI/release validation. It composes the [shared IC inventory](ci/external-ic-dependencies.txt)
with [local bans](ci/cargo-dependency-boundaries.json) on direct generator and
retired decoder dependencies, including aliases,
across member normal/build/dev/target declarations. It does not enforce feature
selection, inspect unused root catalog
entries or scan independent workspaces; this workspace has no maintained independent
Cargo roots and its catalog and feature selections are reviewed separately.

`make check-dependency-graphs` adds locked, offline normal/build graph admission
in CI/release validation: all-feature native and Wasm core graphs exclude the
local tests/tooling and IC integrations; the all-feature qualification Wasm excludes
host-only packages, Candid and full CDK. It requires one Timers, executor,
raw-system, Platform and Platform-macro identity. The fixture and Timers share
Platform 0.6.
A fourth context excludes Jobs, Memory, Timers, JSON and the IC runtime from
ordinary native consumer builds, including the
Platform `cdk` feature.
The fifth context keeps Candid, PocketIC and Testkit out of the native capture
binary's normal/build graph. Candid remains only in native recovery tests:
Testkit enables Platform's PocketIC integration, and PocketIC itself uses Candid
management and transport DTOs. Testkit owns no Candid codec adapter. Disabling
dependency defaults
cannot remove those required edges. Jobs-owned RPCs continue to use Wire.
The policy lives in Make; Shared owns the [observation/checker](docs/cargo-graph-checks.md).
The [snapshot manifest](.shared-tooling.snapshot) records the reviewed Shared
Tooling version, source revision and payload digests. No sibling checkout is a
runtime prerequisite.

Remaining enforcement limits: these are explicit known-name policies for the
selected roots/targets/features, not inference from role comments or exhaustive
feature-matrix admission. Rendered Cargo identities do not authenticate sources.
Jobs has no production-canister deployment command. Downstream deployments must
bind their own target/features, exclusions, successful locked build, source/lock
identity and final artifact digest to the bytes installed. A passing graph or
snapshot check alone does not establish deployment admission.

## Consumer qualification

Run `make test-consumers` for native notification and maintenance scenarios,
including commit failures, reconstruction, exact receipt reconciliation and
bounded recurring work. These fixtures request IC Memory allocations through the
[workspace-selected dependency](Cargo.toml); the host fixture reopens actual
stable cells. See [test consumers](docs/test-consumers.md) for their storage and
timer boundaries.

Choose checks for the affected caller. For native consumer storage/timer
composition, use `cargo +1.91.0 test -p jobs-test-consumer --test composition
--locked --offline`; for endpoint codecs, select `--test api_wire` instead.
Qualify the core Rust floor separately with `cargo +1.91.0 check -p ic-jobs
--lib --locked --offline`. For the consumer's Wasm floor, use `cargo +1.91.0
check -p jobs-test-consumer --lib --target wasm32-unknown-unknown --locked
--offline`. Lint the affected consumer with `cargo clippy -p jobs-test-consumer
--all-targets --locked --offline -- -D warnings`, or select the Wasm target
when runtime integration changes.

`make clippy`, `make check-wasm`, `make check-msrv` and the other package-group
targets aggregate several checks. `make ci` runs the full release suite and
requires an explicit request or release authorization for local execution.

Release-adapter fixtures can be selected independently with
`make test-release-tooling-retention`, `make test-release-tooling-delivery`,
`make test-release-tooling-recovery`, `make test-release-tooling-metadata`, or
`make test-release-tooling-publication`. The existing `test-release-tooling`
aggregate still runs every group and the publication eligibility fixture.
The retention selection needs no prepared metadata editor or formatters.

The package and its direct Timers qualification consumer require Rust 1.91. Compiler
selections are recorded in [rust-toolchain.toml](rust-toolchain.toml) and
[the local tool catalog](ci/tool-versions.env). Linux and macOS Intel/Apple Silicon
host workflows are maintained. See [the current handoff](docs/status/current.md)
for source-bound checks and outstanding native/live qualification; native tests
do not establish IC message rollback or upgrade recovery.

## Repository setup

The reviewed [Shared Tooling snapshot](.shared-tooling.snapshot) records the
canonical revision, entrypoints and payload digests. The
[common Make interface](docs/make-interface.md) owns setup, formatting, hooks,
snapshot checks and release routing. Jobs retains product checks, dependency
selections and release/publication policy.

Run `make install-tools` to prepare common host/Rust tools and Jobs' selected IC
bundle for optimized qualification; `make tools-check` admits them offline.
Shared Tooling's common Rust set contains cargo-sort and cargo-sort-derives.
Jobs has no Candid extractor caller and selects no extractor installation; older
retained installations are preserved outside the newly selected common PATH.
For an interactive shell, prepend
`$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
Use the same `SHARED_TOOLING_STORE` for setup, checks and evidence collection;
see [local setup](docs/local-setup.md) for prerequisites, storage and reuse rules.
`make help` lists check groups and points to focused selections.
`make install-hooks` enables the reviewed formatting hook after tool preparation;
`make formatting-hook-check` verifies it in disposable repositories.
This target exercises Jobs' actual manifests, Make recipes, hook refresh and
preservation behavior. Shared Tooling owns the general hook/frontend test matrix;
this repository has no frontend formatter integration.

The unpublished harness uses the [selected Testkit dependency](Cargo.toml) and
its separately admitted CLI: run `make install-testkit-tools` and
`make testkit-tools-check` when updating that selection. PocketIC setup/check and
live execution remain separate; see [consumer qualification](apps/job-consumer/README.md).
Common setup and ordinary CI do not launch PocketIC.

Release metadata qualification additionally selects IC Build's published
`ic-build-release-metadata` executable through `make install-release-tools` and
offline `make release-tools-check`. See [release preparation](docs/releasing.md#preparation)
for its explicit path and byte allowance. The capture CLI uses Clap for argument
parsing and preserves the original/optimized capture invocation forms.

## Delivery

The public repository is [dragginzgame/ic-jobs](https://github.com/dragginzgame/ic-jobs).
`make version` inspects the local version. `make package` verifies the crate
archive offline; `make publish-check` performs a publication dry run with registry
access. Both permit working edits and upload nothing.

Explicitly authorized `make release-patch`, `make release-minor` and
`make release-major` commands run the complete gate, prepare metadata, commit,
tag and atomically push to `origin`/`main`. Package upload is a separate
`make publish` operation requiring clean source and the matching pushed annotated
tag. See [delivery procedure](docs/releasing.md) for prerequisites and interruption
recovery. Configuring these commands does not authorize a release or upload.
