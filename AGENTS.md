# IC Scheduling agent instructions

Apply the reviewed [Dragginzgame baseline](DRAGGINZGAME.md) first.
Its exact revision is recorded in [.shared-tooling.snapshot](.shared-tooling.snapshot).
This file is the local overlay. Read [the current handoff](docs/status/current.md).

- Scope is this workspace; other sibling repositories remain read-only.
- In `ic-jobs`, keep metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- For Canic adapter work, read [the integration guide](docs/canic-integration.md).
  It describes a proposed consumer integration, not an implemented Canic API.
- Follow the [shared validation cadence](DRAGGINZGAME.md#validation-and-evidence).
  Local focused checks include package tests, MSRV/Wasm checks and tooling
  fixtures; `make ci` is the full release suite. Live PocketIC qualification
  retains its separate explicit selection.
- Timers owns the sole private IC timer provider; retain its unsafe FFI confinement,
  dependency checks, runtime qualification and policy cohort gates. Jobs stays
  Serde-only and does not acquire a runtime dependency on Timers.
- Both libraries and all probes belong to this root workspace at shared version
  0.33.0. Sibling Timers source is preserved as provenance, not a build input.
- Rust floor: 1.91 for core and consumer-owned Timers integration. Qualify both separately.
- Host workflows support Linux and macOS Intel/Apple Silicon. Bind native and
  live IC recovery qualification to their tested source and dependency graph;
  native tests do not establish IC recovery.
- Use the common contribution and release authority rules. The public remote is
  `dragginzgame/ic-scheduling`; configuring tooling does not authorize releases or
  publication. Direct release delivery uses `origin`/`main`; `make publish` is
  a separate, explicitly authorized crates.io upload.
