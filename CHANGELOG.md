# Changelog

## [0.2.0]

- **Breaking:** Require Rust 1.88 for the package, with or without the optional
  timers feature. Upgrade older consumer toolchains; native and Wasm minimum
  checks now use the same floor.
- Report all staged, unstaged and untracked paths refused by release or
  publication preflight, preserving source and index bytes. Adopt Shared Tooling
  0.1.30 and its PocketIC 16.1.0 setup pin
  ([shared #74](https://github.com/dragginzgame/shared-tooling/issues/74),
  [shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).

## [0.1.2] - 2026-10-09

- Add a bounded batch scheduler that selects the earliest due job and returns
  its checked dispatch intent for application-owned persistence and execution.
- Qualify releases against the current workspace version so release tooling
  checks continue to work after the initial release.

## [0.1.1] - 2026-10-08

- Provide package verification, publication dry runs and a separate crates.io
  upload command for clean releases with matching pushed annotated tags.
- Include regular README and license files in the crate package, supporting
  packaged rustdoc and the repository's pre-commit formatting snapshot.
- Keep release fixture checks independent of the shell locale, and report the
  failing source line when a tooling check fails.

First tagged release, adding delivery tooling and fixes to the initial 0.1.0
implementation described below.

## [0.1.0]

- Introduce durable job metadata with one-shot and recurring schedules, bounded
  retries, pending cancellation and explicit uncertain-effect reconciliation.
- Add validated record reconstruction, a persistence example and an optional
  IC Timers watchdog adapter.

Initial implementation snapshot; no separate `v0.1.0` tag was created.
