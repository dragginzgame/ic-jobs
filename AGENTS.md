# IC Jobs agent instructions

Apply the reviewed [Dragginzgame baseline](DRAGGINZGAME.md) first.
Its exact revision is recorded in [.shared-tooling.snapshot](.shared-tooling.snapshot):
`4e274a2219c0b0cc3af68ec65658b373253518fb`.
This file is the local overlay. Read [the current handoff](docs/status/current.md).

- Scope is this workspace; other sibling repositories remain read-only.
- Keep job metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- Use focused package checks automatically during authorized implementation.
  `make ci`, release gates and live PocketIC qualification are complete gates
  selected explicitly or by configured CI.
- Rust floor: 1.88 for both core and optional timers. Qualify both separately.
- Host workflows support Linux and macOS Intel/Apple Silicon. Native macOS and
  IC recovery evidence are pending; do not claim native tests establish them.
- Use the common contribution and release authority rules. The public remote is
  `dragginzgame/ic-jobs`; configuring tooling does not authorize releases or
  publication. Direct release delivery uses `origin`/`main`; `make publish` is
  a separate, explicitly authorized crates.io upload.
