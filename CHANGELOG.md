# Changelog

## [0.2.3]

- Preserve checkout paths ending in newlines when publishing, and qualify
  publication refusals and the intended upload invocation from unusual paths
  ([#4](https://github.com/dragginzgame/ic-jobs/issues/4)).
- Prepare the selected locked dependency cache during release preflight before
  offline validation, respecting explicit offline settings and preserving
  metadata on fetch failure. Adopt Shared Tooling 0.1.35
  ([#5](https://github.com/dragginzgame/ic-jobs/issues/5)).
- Refresh the locked optional-timers graph to IC Timers 0.14.23 and IC Metrics
  0.2.18 within the existing compatible dependency requirements.

## [0.2.2] - 2026-10-09

- Keep fleet reporting in Shared Tooling while retaining local tool setup,
  offline checks and workspace LOC reporting. Adopt the reviewed 0.1.34 snapshot
  and remove the unused consumer reporter
  ([#1](https://github.com/dragginzgame/ic-jobs/issues/1)).
- Reuse verified IC tool bundles when only pin comments or row order change,
  preserving checksums, version admission and original installation receipts
  ([shared #79](https://github.com/dragginzgame/shared-tooling/issues/79)).
- Preserve CI qualification for each pushed commit, including queued native
  checks; superseded revisions of the same pull request may still be cancelled
  ([#3](https://github.com/dragginzgame/ic-jobs/issues/3)).

## [0.2.1] - 2026-10-09

- Reject contradictory occurrence records during restoration, including successful
  one-shot work made pending again, shifted first-occurrence deadlines, retries
  before their scheduled time, and recurring deadlines or cancellation times
  inconsistent with preceding completion. Preserve valid recovery, CatchUp
  and cancellation behavior
  ([#2](https://github.com/dragginzgame/ic-jobs/issues/2)).

## [0.2.0] - 2026-10-09

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
