# IC Jobs agent instructions

Apply the reviewed [Dragginzgame baseline](DRAGGINZGAME.md) first.
Its exact revision is recorded in [.shared-tooling.snapshot](.shared-tooling.snapshot):
`169d77b8440568c5200eede971625126181f7bb2`.
This file is the local overlay. Read [the current handoff](docs/status/current.md).

- Scope is this workspace; other sibling repositories remain read-only.
- Keep job metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- Use focused package checks during implementation and `make ci` before delivering
  completed code changes. Release execution and live PocketIC qualification remain
  explicitly selected effects; native checks do not establish IC recovery.
- Rust floor: 1.88 for both core and optional timers. Qualify both separately.
- Host workflows support Linux and macOS Intel/Apple Silicon. Bind native
  qualification to its tested source; IC recovery evidence remains pending and
  native tests do not establish it.
- Use the common contribution and release authority rules. The public remote is
  `dragginzgame/ic-jobs`; configuring tooling does not authorize releases or
  publication. Direct release delivery uses `origin`/`main`; `make publish` is
  a separate, explicitly authorized crates.io upload.
