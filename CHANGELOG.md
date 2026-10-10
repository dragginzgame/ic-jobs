# Changelog

## [0.4.6] - 2026-10-10

- Adopt IC Memory 0.34 in the test consumers, using one host-owned allocation
  pool and key-only opens. Exercise stable-cell cold reopens with retained job
  payloads, deadlines and blocked effects
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11),
  [Memory #44](https://github.com/dragginzgame/ic-memory/issues/44)).
- Adopt Shared Tooling 0.2.13. Prepare the selected cargo-edit installation after
  release source/cache admission, preserve previous tool installations, and check
  tools before parallel validation dispatch
  ([#14](https://github.com/dragginzgame/ic-jobs/issues/14)).

## [0.4.5] - 2026-10-10

- Adopt Shared Tooling 0.2.12 and remove the temporary Make flag parser. Canonical
  admission rejects hidden unsafe modes and overwritten invocation evidence
  ([#12](https://github.com/dragginzgame/ic-jobs/issues/12),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).
- Keep formatting output concise while retaining full failure diagnostics locally
  and collecting them in CI artifacts
  ([#13](https://github.com/dragginzgame/ic-jobs/issues/13)).
- Reject line breaks in snapshot directory paths before export or verification
  can select a different checkout
  ([shared #95](https://github.com/dragginzgame/shared-tooling/issues/95)).

## [0.4.4] - 2026-10-10

- Exercise authorized create, inspect, bounded list, cancel and due-dispatch
  management flows in the notification consumer, deriving wakeup decisions from
  committed records and preserving blocked effects after reconstruction
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Add IC Memory 0.33 to the test consumers for composable, key-only storage
  requests and collision rejection. Allocation and stable-store bootstrap remain
  host-owned ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Refuse unsafe Make modes even when command-line `MAKEFLAGS` hides them. Adopt
  Shared Tooling 0.2.9 for selected-snapshot admission and recursive Make commands
  ([#12](https://github.com/dragginzgame/ic-jobs/issues/12),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).

## [0.4.3] - 2026-10-09

- Adopt Shared Tooling 0.2.7 so release and formatting commands refuse Make's
  ignore-errors, dry-run, touch and question modes before recipes run. Keep
  release adapter checks isolated from inherited tooling roots
  ([#10](https://github.com/dragginzgame/ic-jobs/issues/10),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30),
  [shared #7](https://github.com/dragginzgame/shared-tooling/issues/7)).

## [0.4.2] - 2026-10-09

- Adopt Shared Tooling 0.2.6 for release entrypoints and workspace formatting,
  replacing duplicated Make recipes while retaining direct-only release admission
  and explicitly prepared formatter requirements
  ([#10](https://github.com/dragginzgame/ic-jobs/issues/10)).
- Refresh the locked optional timer dependency to IC Timers 0.16.2 within the
  existing 0.16 requirement. Timer library source, Job records and scheduling
  behavior are unchanged.

## [0.4.1] - 2026-10-09

- Adopt committed Shared Tooling 0.2.5 to preserve literal hook selections and
  support hook setup/formatting in checkouts ending in newlines
  ([shared #89](https://github.com/dragginzgame/shared-tooling/issues/89)).
- Extend the test consumers with exact rejection evidence for lost replies,
  retry-budget exhaustion and recovery after a partially committed maintenance
  batch. Confirmed delivery receipts remain authoritative for duplicate requests.

## [0.4.0] - 2026-10-09

- **Breaking:** The optional adapter now uses IC Timers 0.16 and its IC Metrics
  0.3 package identity. Align all timer owners in a canister to IC Timers 0.16;
  use Metrics 0.3 or `ic_timers::MeasurementSummary` when exchanging telemetry.
  Job records and scheduling behavior remain unchanged.

## [0.3.1] - 2026-10-09

- Add notification and cache-maintenance test consumers for persistence failures,
  lost-reply reconciliation, safe retries, shared wakeup selection and bounded work. Expose
  `make test-consumers` for core/timers qualification and include it in configured CI.
- Install and verify the final selected IC tool even when its pin file has no
  trailing newline, preserving the original pins and version checks. Adopt the
  reviewed Shared Tooling 0.2.2 snapshot
  ([#9](https://github.com/dragginzgame/ic-jobs/issues/9),
  [shared #87](https://github.com/dragginzgame/shared-tooling/issues/87)).

## [0.3.0] - 2026-10-09

- **Breaking:** The shared IC setup now installs five tools; PocketIC provisioning
  belongs to IC Testkit. Explicitly rerun `make install-ic-tools`, then
  `make ic-tools-check`, to replace a six-tool installation. Previous bundles
  and receipts are retained; scheduler APIs and stored jobs are unchanged
  ([#7](https://github.com/dragginzgame/ic-jobs/issues/7),
  [shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).
- Keep prepared checkout-local tools available to isolated pre-commit formatting
  and reject multi-document dependency-pinning exception catalogs. Adopt the
  committed Shared Tooling fixes without adding a Cargo-tool mode
  ([#6](https://github.com/dragginzgame/ic-jobs/issues/6)).
- Refresh the locked optional-timers graph to IC Metrics 0.2.20 within the
  existing compatible dependency requirements.
- Remove the unused PR release transport from the selected snapshot, retaining
  Jobs' direct release and recovery workflow
  ([#8](https://github.com/dragginzgame/ic-jobs/issues/8)).

## [0.2.3] - 2026-10-09

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
