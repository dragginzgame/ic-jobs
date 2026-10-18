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

## Scope and validation

The library owns metadata and checked transitions. Applications own storage,
payloads, handlers, longer audit history and disposition of uncertain effects.
It supplies no global queue, database, lifecycle exports or exactly-once effects.

| Boundary | Owner and entry point |
| --- | --- |
| Validated records, attempts, transitions and due-time selection | Published `ic-jobs`: `Job::restore`, `Job` methods, `next_due_ns` and `start_next`; Serde is its only runtime dependency |
| Payloads, authorization, stable queue and lifecycle endpoints | Consumer applications; the unpublished `apps/job-consumer` demonstrates this integration |
| Stable allocation and fixture Cell admission | IC Memory; raw substrate access here is isolated test setup/corruption |
| Volatile wakeups and watchdog recovery | IC Timers, called directly by the consumer |
| Candid contracts, Principal and bounded delivery decoding | Published IC Platform; Jobs retains schemas, limits and error policy |
| PocketIC transport and startup | IC Testkit; call helpers share Platform decoder limits and error types |
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
| Qualification `jobs_test_consumer.wasm` | Platform's `candid` feature supplies API derives, identities and bounded replies. Direct Candid remains only for CDK-generated entrypoints/service export | `src/canister.rs` still needs the unavailable full runtime boundary; Timers and CDK share one executor |
| Native recovery test executable | Platform supplies DTO bounds/Principal/encoding and the decoder limits/errors consumed by Testkit 0.54 | The consumer's direct `ic-cdk` dependency is Wasm-only |
| `jobs-capture-artifact` host binary | No Candid dependency in its normal/build graph | No CDK dependency in its normal/build graph |

The Wasm above is an unpublished test fixture, not a production service. This
checkout does not establish what other deployed canisters use. Audit each actual
application's selected target graph and source rather than counting manifests.
For example, inspect the qualification Wasm's paths with:

```sh
cargo tree -p jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert candid --locked --offline
cargo tree -p jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-cdk --locked --offline
cargo tree -p jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-cdk-executor --locked --offline
```

The selected published boundary is **ic-platform 0.1.9**, with defaults disabled
and only `candid` enabled by the qualification app and host recovery tests. Its
upstream type identities and decoder behavior preserve the existing wire/storage
contract; frozen pre-adoption API bytes are checked by `make check-consumer`.
No parser, certification or Platform executor feature is selected. IC Wire-owned
protocols remain with IC Wire; no protocol or storage conversion is introduced.
The earlier update from locked 0.1.5 changed only Platform in the resolved graph.
Its bounded decoder implementation was byte-identical; the new default-decoder
exports retain a different policy and do not replace Jobs’ explicit five limits.

The remaining direct exceptions are Wasm-only Candid/CDK in the test canister,
for [runtime/macros and protected-spawn semantics (#1)](https://github.com/dragginzgame/ic-platform/issues/1#issuecomment-6247576553).
Platform centralizes the supported dependency boundary; upstream Candid/CDK code
still exists transitively. This is not a claim of upstream removal or smaller Wasm.

Published Testkit 0.54.0 consumes Platform's `DecodeLimits` and `DecodeError`.
The harness now uses that same decoder directly, preserving all five limits and
argument-tuple reply handling. The retired `ic-testkit-candid` dependency is absent
from manifests and the lockfile, and direct declarations are forbidden by policy.
No local-path override, limits conversion or replacement transport is introduced.
Earlier disposable producer qualification remains historical evidence in
`target/qualification/fleet-convergence/`; published-adoption evidence is retained
in `target/qualification/testkit-054/`. This graph also includes native-only Metrics
0.14.1 for Testkit, while the qualification canister retains Timers' Metrics 0.13.4.

For the production library, locked normal/build graphs on native Linux and Wasm,
with all features selected, contain Serde and its macro dependencies and exclude
every tests/tooling-only catalog dependency (apart from the selected root itself).
The qualification Wasm intentionally includes its test integrations above; those
cannot be prohibited there using the production library's exclusion list.

`make check-dependency-boundaries` runs the reviewed shared checker and is part of
CI/release validation. Its [policy](ci/cargo-dependency-boundaries.json) admits only
the exceptions above and checks canonical dependency names, including aliases,
across member normal/build/dev/target declarations. It does not enforce feature
selection, constrain exceptions to a particular table, inspect unused root catalog
entries or scan independent workspaces; this workspace has no maintained independent
Cargo roots and its catalog and feature selections are reviewed separately.

`make check-dependency-graphs` adds locked, offline normal/build graph admission
in CI/release validation: all-feature native and Wasm core graphs exclude the
local tests/tooling and IC integrations; the all-feature qualification Wasm excludes
host-only packages and requires one Timers and one CDK executor rendered identity.
The policy lives in Make; Shared owns the [observation/checker](docs/cargo-graph-checks.md).
The snapshot selects committed topic source from
[Shared PR #285](https://github.com/dragginzgame/shared-tooling/pull/285), not a new
published Shared release. No normal sibling checkout is a runtime prerequisite.

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
Shared Tooling 0.24.0's common Rust set contains cargo-sort and cargo-sort-derives.
Jobs has no Candid extractor caller and selects no extractor installation; older
retained installations are preserved outside the newly selected common PATH.
For an interactive shell, prepend
`$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
Use the same `SHARED_TOOLING_STORE` for setup, checks and evidence collection;
see [local setup](docs/local-setup.md) for prerequisites, storage and reuse rules.
`make help` lists focused checks. `make install-hooks` enables the reviewed
formatting hook after tool preparation; `make formatting-hook-check` verifies it
in disposable repositories.
This target exercises Jobs' actual manifests, Make recipes, hook refresh and
preservation behavior. Shared Tooling owns the general hook/frontend test matrix;
this repository has no frontend formatter integration.

The unpublished harness uses the [selected Testkit dependency](Cargo.toml) and
its separately admitted CLI: run `make install-testkit-tools` and
`make testkit-tools-check` when updating that selection. PocketIC setup/check and
live execution remain separate; see [consumer qualification](apps/job-consumer/README.md).
Common setup and ordinary CI do not launch PocketIC.

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
