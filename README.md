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

Run `make test-consumers` for native notification and maintenance scenarios,
including commit failures, reconstruction, exact receipt reconciliation and
bounded recurring work. These fixtures request IC Memory allocations through the
[workspace-selected dependency](Cargo.toml); the host fixture reopens actual
stable cells. See [test consumers](docs/test-consumers.md) for their storage and
timer boundaries.

The package requires Rust 1.88, including its optional timers feature. Compiler
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

Run `make install-tools` to prepare the selected host, IC and Rust toolsets,
including Jobs' cargo-edit selection; `make tools-check` admits them offline.
For an interactive shell, prepend
`$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
Use the same `SHARED_TOOLING_STORE` for setup, checks and evidence collection;
see [local setup](docs/local-setup.md) for prerequisites, storage and reuse rules.
`make help` lists focused checks. `make install-hooks` enables the reviewed
formatting hook after tool preparation; `make formatting-hook-check` verifies it
in disposable repositories.

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
