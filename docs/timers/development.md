# Timers in IC Scheduling

This workspace owns both published libraries and all their qualification packages.
Use the root [agent instructions](../../AGENTS.md), [release guide](../releasing.md)
and [current handoff](../status/current.md). Older evidence and design documents
retain their tested source identities; old repository commands are historical.

## Commands

- `make test-timers`: minimum-compiler library, doctest and protocol tests.
- `make check-timer-provider check-timer-dependencies`: provider confinement and
  native/Wasm dependency admission, including the matching Platform runtime guard.
- `make test-timer-dependencies`: deliberate forbidden-edge rejection fixtures.
- `make check-timer-probes`: probe compilation and each size-probe feature selection.
- `make install-testkit-tools install-consumer-server`: prepare the shared CLI/server.
- `make test-timer-watchdog`: explicit installed watchdog/runtime qualification.
- `make test-timer-cohorts`: explicit installed policy comparison with retained reports.
- `make test-canister`: explicit Jobs/Timers composition recovery qualification.

The root owns one Cargo lock, Rust 1.91 floor, development toolchain, formatting,
CI and publication path. The `timer-probe` profile retains its previous optimization
settings. Ordinary builds default to both published crates. Probe packages remain
unpublished. The timer registry, callbacks and unsafe IC dispatch remain private
to Timers; consumers still own durable job state and lifecycle reconstruction.

## Host support

Linux and macOS Intel/Apple Silicon remain supported qualification hosts.
Native tests cannot establish IC recovery. The manually selected recovery workflow
runs both Jobs recovery and Timers watchdog/cohort gates on each host. Check the
handoff for what was actually run on this combined source and graph.

## Release and retained evidence

Both libraries use the approved 0.33.0 workspace version. The combined changelog
records the migration. The imported [Timers changelog](../../crates/ic-timers/CHANGELOG.md)
retains its preceding release history. Publication selects both libraries and
requires explicit authorization; moving source does not publish or deploy it.
Historical receipts under `evidence/`, `audits/` and `changelog/` describe their
original checkouts and cannot qualify this migration. Original sibling source
remains available for rollback and provenance.
