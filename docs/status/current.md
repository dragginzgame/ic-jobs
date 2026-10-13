# Current handoff

## Source and authority

Delivered Jobs **0.9.1**: `fae7f2ac268199770482f769e50310bbb752afe2`.
The working tree prepares compatible **0.9.2**. Package metadata stays 0.9.1;
release/publication, staging and Git refs remain maintainer-owned.
Shared Tooling remains reviewed **0.10.0** at
`ed9c26f015691859f67b50b512a94709dcdc0c6c`. No dependency/snapshot adoption is
part of this batch. Incoming Cargo.lock changes are preserved.

## Pending implementation

- Notification management admits the complete bounded queue once per command.
  Enqueue, inspection, listing, cancellation and due dispatch carry that state
  through the reply and earliest deadline. Authorization stays first; invalid
  page limits still precede storage decoding. Enqueue/dispatch fixture helpers
  converge on the same operations; no delivery escapes before a successful commit.
- Recurring restore binds the completion timestamp after checking presence and
  the completion floor, then carries it into the canonical successor calculation.
  Stored fields, typed refusal behavior and blocked effects remain unchanged.
- The former handoff is preserved byte-for-byte in
  [history through 0.9.1](history-through-0.9.1.md). It contains historical
  preparation/acceptance observations, not current authority. Keeping it at this
  directory level preserves its relative links and source-bound evidence.

Focused evidence lives in `target/qualification/restore-management-cleanup/`.
Core/timers package tests pass separately on Rust 1.88, including frozen-record
roundtrips and 21 consumer tests per configuration. Minimum native/Wasm checks,
all 16 stable-consumer tests, release Wasm, harness compilation, strict Clippy and
docs/doctests pass. Final consumer rechecks pass after simplifying a linted guard;
the first lint failure is retained. The tested incoming graph includes Timers
0.22.1, Memory 0.36.3, Metrics 0.9.1, Testkit 0.37.0 and Host 0.17.6.
Commands/results are recorded there; native and compile checks do not
establish live IC timer, rollback or upgrade acceptance. Previous target-based
local evidence may have been removed by the maintainer's cargo clean; historical
records identify the inputs originally tested and do not qualify this batch.

## Acceptance and remaining owners

- Delivered 0.9.1 [CI 38428581972](https://github.com/dragginzgame/ic-jobs/actions/runs/38428581972)
  is queued at inspection. The 0.9.0 run 38421834204 was cancelled; its Linux
  success does not complete all-host acceptance for [#41](https://github.com/dragginzgame/ic-jobs/issues/41)
  or [#43](https://github.com/dragginzgame/ic-jobs/issues/43).
- [#39](https://github.com/dragginzgame/ic-jobs/issues/39) and
  [#40](https://github.com/dragginzgame/ic-jobs/issues/40) are closed against
  delivered 0.8.7 `0951f3083c961204bd6b40da97251d0812798031`,
  [CI 38395094597](https://github.com/dragginzgame/ic-jobs/actions/runs/38395094597):
  the actual complete logs pass Linux and both macOS architectures.
- [#42](https://github.com/dragginzgame/ic-jobs/issues/42) retains publication
  and artifact-capture adoption. [Shared #165](https://github.com/dragginzgame/shared-tooling/issues/165)
  reports delivered Shared 0.11.3 `eadab78`, with macOS acceptance pending and
  canonical Jobs adoption outstanding. [Build #14](https://github.com/dragginzgame/ic-build/issues/14)
  records an integrated retention draft with pending 0.2.4 delivery/publication.
  Active Jobs paths remain until their producer and consumer prerequisites pass.
- Last inspected live recovery is Jobs 0.8.1
  `f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
  [run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
  84 passes across seven scenarios, four Wasm variants and three hosts.
  No live gate is selected by this batch; that evidence does not qualify new graphs.

See [architecture and usage](../../README.md), [safety](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md), and
[delivery procedure](../releasing.md).
