# Coordinated Scheduling history through 0.33.1

This entry covered both Jobs and Timers before independent release groups.
It is retained once; group ledgers link here. Earlier Jobs history remains in
[the Jobs ledger](jobs/CHANGELOG.md), and earlier Timers history in
[the Timers ledger](timers/CHANGELOG.md).

## [0.33.1] - 2026-10-30

### 🩹 Fixed

- Read retained input-drift diagnostics in the consumer-tooling fixture so it
  also passes inside nested release validation, where console details are omitted.

### ⚠️ Breaking

- Adopt Metrics 0.21.0 for Timers' public measurement types. Consumers exchanging
  those types must align their Metrics dependency to 0.21 and native Wire
  metadata to 0.10. Coordinate runtime users on Platform 0.11. Report fields,
  Serde layouts and arithmetic remain unchanged.

### 📦 Workspace

- Consolidate IC Jobs and IC Timers as IC Scheduling at the approved shared
  version 0.33.0, retaining separate published crates and consumer-owned durable
  state. Import the Timers runtime, native tests, probes, safety contract and
  historical evidence into one workspace and lockfile.
- Adopt Platform 0.11.1 and Wire 0.10.0 with the local Timers source. Require the
  embedded runtime guard and exclude registry executor/IC0 from deployed graphs.
- Share formatting, setup, CI and two-package publication admission; preserve
  independent native, Wasm and installed recovery qualification.

### 📦 Dependencies

- Adopt Tooling/Testkit 0.72.0 across Build, Host and Testkit dependencies,
  aligning both native harnesses with Wire 0.10.0 and Platform 0.11.1.
  This resolves the previous Testkit/Wire limits incompatibility.
- Adopt Shared Tooling 0.27.11 at `62edfbd43087a718bf0283e9040676a49211976d`
  through its exporter, and select Memory/Storage 0.58.0, removing the older
  Platform and Wire copies from the resolved graph.
  Preserve the existing queue/ledger formats, bounded RPC contracts and Rust 1.91
  floor.
- Adopt Metrics and Logging 0.21.0, preserving report layouts, heap-only logging
  and retained-text limits.

### 🧹 Cleanup

- Align the three private Jobs support package names and directories under
  `ic-jobs-*`, including workspace references, artifact paths and tooling fixtures.
  Retain all seven unpublished support packages and their dependency boundaries.
- Replace path-based test module wiring with ordinary module discovery and a
  private shared host-support crate. Keep that transport out of the published
  Jobs library, qualification Wasm and capture binary's normal dependency graph.

