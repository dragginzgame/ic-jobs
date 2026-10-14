# Changelog

## [0.9.4] - 2026-10-14

- Select adjacent dependency entries dynamically in Jobs' formatting fixture,
  so adding a sorted dependency does not break the hook qualification check.

- Reject stored records whose retry history requires more elapsed time than
  their retained evidence allows, including uncertain attempts and rescheduled
  successors. Valid records and explicit effect reconciliation remain unchanged
  ([#44](https://github.com/dragginzgame/ic-jobs/issues/44)).

- Adopt Shared Tooling 0.12.0: release candidates are numbered and top-positioned;
  current delivered implementations and their required CI matrix own routine
  issue acceptance. Contributor tooling changes leave Jobs’ API/storage unchanged.

- Delegate recovery-artifact capture and explicit retention to published IC Build,
  preserving frozen inputs, variant evidence and failed captures without changing
  live qualification authority ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

## [0.9.3] - 2026-10-14

- Adopt Shared Tooling 0.11.4 and its canonical publication admission; retain
  Jobs’ package/features, upload authority and release metadata adapter
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).
- Qualify the selected IC Host 0.17.7 graph in the unpublished recovery harness;
  core scheduling and stored job records are unchanged.

## [0.9.2] - 2026-10-13

- Carry validated completion timestamps directly through recurring restore checks,
  retaining stored-record admission and refusal behavior.
- Reuse admitted notification state across management commands and deadline
  replies, keeping authorization and commit-before-delivery ordering.

## [0.9.1] - 2026-10-13

- Converge test-consumer management authorization, stable mutation and timer
  reconciliation through one synchronous boundary.
- Reuse the validated job's active attempt for delivery completion, retaining
  exact receipt matching and refusal ordering.
- Derive test-consumer deadline summaries without a temporary mutable job batch,
  retaining complete snapshot admission and checked job restoration.
- Compare completion envelopes and notification receipt evidence using borrowed
  payloads; notification cancellation carries its admitted state through commit
  and reply without repeated snapshot loads.

## [0.9.0] - 2026-10-13

- **Breaking:** adopt IC Timers 0.22 and its Metrics 0.9 measurement-summary
  identity. Align direct timer dependencies to 0.22; use the Timers summary
  re-export or Metrics 0.9 for exchanged measurements, updating Metrics overflow
  accessors and error matches. Job records and retained storage need no reset
  ([#41](https://github.com/dragginzgame/ic-jobs/issues/41)).

- Remove duplicate standalone formatter checks now owned by Shared Tooling,
  retaining Jobs’ workflow and failure-evidence integration checks
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

- Simplify dispatch attempt increments using validated Pending-state budget
  guarantees, retaining sequence overflow and error atomicity
  ([#43](https://github.com/dragginzgame/ic-jobs/issues/43)).
- Qualify recovery against one frozen original artifact and remove repeated
  fixture authorization; keep variant hashes/logs and create authorization
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

## [0.8.7] - 2026-10-13

- Adopt Shared Tooling 0.10: common setup, formatting and failure evidence use
  snapshot-owned tool catalogs. Host and Rust overrides are independent;
  tool versions and retained installations are unchanged
  ([#40](https://github.com/dragginzgame/ic-jobs/issues/40),
  [Shared #153](https://github.com/dragginzgame/shared-tooling/issues/153)).
- Retain filesystem release-lock errors and reject line breaks in LOC report
  paths before selecting a checkout
  ([Shared #152](https://github.com/dragginzgame/shared-tooling/issues/152),
  [#151](https://github.com/dragginzgame/shared-tooling/issues/151)).
- Separate Running and Uncertain restore history checks, retaining common
  timestamp admission and blocked-effect semantics
  ([#39](https://github.com/dragginzgame/ic-jobs/issues/39)).

## [0.8.6] - 2026-10-13

- Adopt Shared Tooling 0.9.15: reject maintenance paths containing line breaks
  before tool probes or state writes; retain failed-attempt log recovery guidance
  ([#37](https://github.com/dragginzgame/ic-jobs/issues/37),
  [Shared #150](https://github.com/dragginzgame/shared-tooling/issues/150),
  [#149](https://github.com/dragginzgame/shared-tooling/issues/149)).
- Adopt Testkit 0.36 and its IC Host 0.16 dependencies in the unpublished recovery
  harness. Refresh the lock-selected Testkit CLI; prior builds remain retained
  ([#38](https://github.com/dragginzgame/ic-jobs/issues/38)).

## [0.8.5] - 2026-10-13

- Remove the repeated one-shot restore deadline check, retaining common deadline
  admission and the occurrence-origin guard
  ([#36](https://github.com/dragginzgame/ic-jobs/issues/36)).
- Adopt Shared Tooling 0.9.13: preserve filesystem lock and Cargo source-download
  failure statuses, and clarify locked dependency-cache preparation. Refresh
  prepared common and release Rust tools; existing builds remain retained
  ([#35](https://github.com/dragginzgame/ic-jobs/issues/35),
  [Shared #145](https://github.com/dragginzgame/shared-tooling/issues/145),
  [#146](https://github.com/dragginzgame/shared-tooling/issues/146),
  [#147](https://github.com/dragginzgame/shared-tooling/issues/147)).

## [0.8.4] - 2026-10-13

- Adopt IC Memory 0.36 in test consumers using explicit owned allocation requests.
  Preserve retained storage keys, job records and consumer-owned bootstrap
  ([#34](https://github.com/dragginzgame/ic-jobs/issues/34)).

## [0.8.3] - 2026-10-13

- Simplify stored-record admission by removing checks already implied by retained
  invariants, preserving accepted records and rejection behavior
  ([#32](https://github.com/dragginzgame/ic-jobs/issues/32)).
- Complete test-consumer deliveries from one validated snapshot, preserving exact
  receipt checks, refusal ordering and blocked effects
  ([#33](https://github.com/dragginzgame/ic-jobs/issues/33)).

## [0.8.2] - 2026-10-13

- Fix retry-policy decoding for Serde formats that retain struct names: the checked
  reader now accepts the same `RetryPolicy` representation emitted by the writer.
  Preserve checked configuration and existing JSON/stable-memory bytes
  ([#30](https://github.com/dragginzgame/ic-jobs/issues/30)).

## [0.8.1] - 2026-10-12

- Adopt Shared Tooling 0.9.7: reject non-boolean Cargo offline configuration
  before tool setup or admission while respecting explicit environment overrides.
  Refresh common and release Rust tools to select the new build policy; existing
  builds remain retained ([#29](https://github.com/dragginzgame/ic-jobs/issues/29),
  [Shared #140](https://github.com/dragginzgame/shared-tooling/issues/140)).

## [0.8.0] - 2026-10-12

- **Breaking:** require nonzero recurring intervals and checked, immutable retry
  configuration. `Job::new` now returns `Job` directly; update constructors and
  exhaustive error matches. Malformed configuration fails decoding, while valid
  retained records keep their existing serialized shape and require no reset
  ([#28](https://github.com/dragginzgame/ic-jobs/issues/28)).
- **Breaking:** select IC Timers 0.21 for the optional adapter and its Metrics
  0.8 summary type. Align direct dependencies; failed first reconciliation now
  leaves the registration slot empty so retry can install its supplied callback.
- Select Testkit 0.35 for the native recovery harness. Jobs uses its maintained
  PocketIC API and needs no dependency on the artifact APIs moved to IC Build.

## [0.7.5] - 2026-10-12

- Adopt Shared Tooling 0.9.4: preserve installer failures while releasing locks,
  retry transient asset downloads within fixed limits, and reject malformed
  checksum manifests, consumer paths and Git identities
  ([#27](https://github.com/dragginzgame/ic-jobs/issues/27),
  [Shared #138](https://github.com/dragginzgame/shared-tooling/issues/138),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133)).
- Reject stored dispatch sequences that exceed the retained occurrence retry
  budget, including after schedule replacement. Preserve valid histories and
  block the entire consumer queue before effects when counts are contradictory
  ([#26](https://github.com/dragginzgame/ic-jobs/issues/26)).

## [0.7.4] - 2026-10-12

- Adopt Shared Tooling 0.9.0: snapshot refresh and verification share one
  validated manifest reader, rejecting NUL bytes and emitting no records for
  malformed input. Existing snapshots need no conversion
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133)).
- Support structured workspace LOC reports through `cloc.sh --json`, alongside
  the existing human-readable output.

## [0.7.3] - 2026-10-12

- Adopt Shared Tooling 0.8.3: reject malformed snapshot fields, retain build
  identity diagnostics without following redirected evidence paths, and admit
  formatters against the selected Rust catalog. Include the read-only Cargo
  tool freshness guide and complete Make snapshot companions
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133),
  [Shared #134](https://github.com/dragginzgame/shared-tooling/issues/134),
  [Shared #135](https://github.com/dragginzgame/shared-tooling/issues/135),
  [Shared #136](https://github.com/dragginzgame/shared-tooling/issues/136),
  [Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131)).
- Expand durable-record qualification across mixed management, retry and
  uncertainty histories and deadline extremes. Check serialization/restore
  agreement and unchanged records after rejected transitions
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23),
  [#24](https://github.com/dragginzgame/ic-jobs/issues/24)).
- Allow 45 minutes for native CI after cold Intel qualification reached the
  previous 30-minute limit during compilation
  ([#25](https://github.com/dragginzgame/ic-jobs/issues/25)).

## [0.7.2] - 2026-10-12

- Adopt Shared Tooling 0.8.0: snapshot refresh derives required companions from
  recorded entrypoints and automatically retires verified obsolete payloads.
  Keep Jobs' existing selection; no files are retired by this refresh
  ([Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131)).
- Use the macro-safe cargo-sort-derives 0.14.1 pin.
  Refresh common Rust tools with `make install-rust-tools`, then
  `make rust-tools-check` ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #113](https://github.com/dragginzgame/shared-tooling/issues/113)).
- Refresh the selected Host 0.14.5, Memory 0.35.14, Metrics 0.7.1 and Testkit
  0.34.2 patches. Prepare/check the separately selected Testkit CLI before
  recovery qualification.
- Correct the timer adapter guides to the Timers 0.19 dependency already
  selected by Jobs 0.7.1. Applications sharing its public timer types must use
  the same Timers family; the job storage contract is unchanged.

## [0.7.1] - 2026-10-12

- Adopt Shared Tooling 0.6.5 and its common Make entrypoint for setup, formatting,
  hooks, snapshot verification and releases. Admit prepared Rust tools during
  staged formatting and stop later validation stages if selected inputs change
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #130](https://github.com/dragginzgame/shared-tooling/issues/130),
  [Shared #129](https://github.com/dragginzgame/shared-tooling/issues/129)).
- Qualify release recovery through the shared recipes without duplicate recipe
  warnings or suppressing diagnostics
  ([Shared #128](https://github.com/dragginzgame/shared-tooling/issues/128)).
- Refresh the locked Host crates to 0.14.3 through Testkit; published Rust
  sources, dependencies and the Rust 1.88 floor match 0.14.2.

## [0.7.0] - 2026-10-12

- **Breaking:** Select IC Timers 0.18 for the optional timer adapter. Consumers
  using its exposed timer types must update their IC Timers dependency to 0.18;
  direct timer registrations now require an explicit initial schedule. Jobs'
  watchdog reconciliation and stored job format are unchanged.
- Refresh the locked Host crates to 0.14.2 through the native Testkit harness;
  their published Rust sources, dependencies and Rust 1.88 floor are unchanged.

- Reject contradictory retained timing when restoring cancelled jobs or running
  attempts: retry backoff and recurring successor deadlines remain validated
  after management and dispatch. Invalid stable records block the complete
  consumer queue before effects ([#24](https://github.com/dragginzgame/ic-jobs/issues/24)).

## [0.6.1] - 2026-10-11

- Refresh the locked Timers selection to 0.17.14 and Metrics to 0.6.6. Their
  published Rust sources, dependency declarations and compiler requirements match
  the preceding patches; Jobs runtime, stored contracts and Rust 1.88 are unchanged.

## [0.6.0] - 2026-10-11

- **Breaking:** Add durable pause/resume and schedule replacement for unstarted
  occurrences. Update exhaustive state/error matches and stored JobRecord codecs
  for the new paused state, cadence origin and independent outcome timestamp.
  Retire/reset the approved disposable 0.5 test installations before creating
  fresh 0.6 consumer state; there is no old-format reader or migration path
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23)).
- Preserve deadlines, retry evidence and effect identities through management;
  running/uncertain effects remain blocked. Extend the existing consumer and
  qualification harness to management persistence, wakeup changes and rollback
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23)).

## [0.5.13] - 2026-10-11

- Adopt Shared Tooling 0.6.3: normal direct-release commands restart superseded
  preparation from committed metadata, preserve the previous plan and rerun
  complete validation
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
- Refresh the locked graph to Timers 0.17.12 and Testkit 0.33.8. The Timers
  patch corrects its private native test provider; Jobs' production timer
  behavior is unchanged
  ([Timers #45](https://github.com/dragginzgame/ic-timers/issues/45)). Rerun
  `make install-testkit-tools` and `make testkit-tools-check` before recovery
  qualification.

## [0.5.12] - 2026-10-11

- Adopt Shared Tooling 0.6.2 for clearer interrupted-release source conflicts
  and guarded maintainer recovery guidance
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
- Refresh the locked graph to Memory 0.35.10, Timers 0.17.11 and Host 0.13.5;
  retain the existing library contracts and Rust 1.88 floor.

## [0.5.11] - 2026-10-11

- Adopt Shared Tooling 0.6.1 so Cargo-tool setup stops on lock, validation and
  activation failures, preserves the original status and retains failed attempts.
  Refresh selections with `make install-tools` and `make tools-check`; prepare
  the selected Testkit CLI separately before recovery qualification
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120)).

## [0.5.10] - 2026-10-11

- Adopt Shared Tooling 0.6.0: share verified Cargo-built tools, including Jobs'
  pinned cargo-edit, and retain safe local failure evidence when shared-store
  admission refuses. Rerun `make install-tools` and `make tools-check`; prepare
  the selected Testkit CLI separately before recovery qualification
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121)).
- Keep release-tool qualification compatible with common Rust setup preflight
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21)).
- Align the consumer memory guide with the selected IC Memory 0.35 family and
  distinguish completed recovery evidence from qualification of the current
  watchdog scenario ([#22](https://github.com/dragginzgame/ic-jobs/issues/22)).

## [0.5.9] - 2026-10-11

- Adopt Shared Tooling 0.5.0 for shared verified host/IC bundles, checkout-local
  common Rust-tool slots, coordinated hook/PATH selection and retained
  installation evidence. Contributors should rerun `make install-tools` and
  `make tools-check`, and use `.tools/rust/common/bin` on their shell PATH
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21)).
- Keep formatting-adoption checks working with relative temporary directories
  ([Shared #116](https://github.com/dragginzgame/shared-tooling/issues/116)).
- Contain known local counter failures in the test consumer: fail the exact job
  without rolling back unrelated bounded work or repeatedly trapping its watchdog
  ([#20](https://github.com/dragginzgame/ic-jobs/issues/20)).
- Add native batch-order/reconstruction coverage and a real-watchdog rollback,
  surviving-wakeup and subsequent-progress qualification case
  ([#20](https://github.com/dragginzgame/ic-jobs/issues/20)).
- Clarify catch-up backlog costs and the distinction between timer callback
  completion and individual job outcomes. Empty consumer batches report no work.

## [0.5.8] - 2026-10-11

- Correct timer composition guidance to IC Timers 0.17 and its canonical
  measurement-summary re-export
  ([#19](https://github.com/dragginzgame/ic-jobs/issues/19)). Select Timers 0.17.8
  for the optional adapter and recovery consumer.

## [0.5.7] - 2026-10-11

- Align the recovery harness and lock-selected CLI on Testkit 0.33.2. Rerun
  `make install-testkit-tools` before live recovery qualification
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.6] - 2026-10-11

- Adopt Shared Tooling 0.3.11: Rust-tool setup and checks use the selected
  consumer's Cargo configuration; retained validation summaries keep early
  fatal, LLVM and missing-file diagnostics
  ([Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101),
  [Shared #110](https://github.com/dragginzgame/shared-tooling/issues/110)).
- Adopt the latest Shared release-command check, supporting relative and
  symlinked temporary directories without losing nested Make checks
  ([Shared #112](https://github.com/dragginzgame/shared-tooling/issues/112)).
- Include failed native tooling fixtures and their inner logs in CI artifacts
  ([#18](https://github.com/dragginzgame/ic-jobs/issues/18)).

## [0.5.5] - 2026-10-10

- Qualify the actual pre-commit hook and Jobs formatting in native CI, including
  failed Git observations, manifest sorting and preservation of staged/working
  files ([#17](https://github.com/dragginzgame/ic-jobs/issues/17)).

## [0.5.4] - 2026-10-10

- Select the Testkit CLI directly from `Cargo.lock` for setup, offline admission
  and recovery qualification; remove its duplicate version pin. Rerun
  `make install-testkit-tools` after changing the locked Testkit version
  ([#17](https://github.com/dragginzgame/ic-jobs/issues/17)).
- Adopt Shared Tooling 0.3.6: pre-commit stops on failed Git tree observations,
  preserving staging and working files
  ([shared #106](https://github.com/dragginzgame/shared-tooling/issues/106)).
- Adopt Shared Tooling 0.3.7: reject failed Cargo-tool activation, release
  metadata, Wasm-input and fixture checks on Bash 3.2, retaining failed evidence
  ([shared #107](https://github.com/dragginzgame/shared-tooling/issues/107),
  [#16](https://github.com/dragginzgame/ic-jobs/issues/16)).
- Cancel superseded CI runs while preserving the host matrix on the latest run
  ([shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

- Keep only the newest CI run per workflow and branch or PR, cancelling older
  queued and running checks while retaining the existing host matrix and gates
  ([Shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

## [0.5.3] - 2026-10-10

- Align the recovery harness and selected CLI on Testkit 0.32.2 for the updated
  consumer dependency graph. Rerun `make install-testkit-tools` before live
  qualification; Job APIs and stored records remain unchanged
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.2] - 2026-10-10

- Align the recovery harness and its selected CLI on Testkit 0.32.1. Rerun
  `make install-testkit-tools` before live qualification
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Adopt Shared Tooling 0.3.4: preserve Cargo's parallel-build coordination and
  reject unavailable setup prerequisites before downloads. Incomplete Bash 3.2
  validation and metadata checks fail while retaining originals and evidence
  ([#16](https://github.com/dragginzgame/ic-jobs/issues/16)).
- Select Binaryen 133 in the common tools; rerun `make install-ic-tools`.
  Explicit `test-canister-optimized` and the manual recovery workflow test the
  original and `-O3`, `-Os`, `-Oz` canisters with retained per-variant evidence.
  Normal canister builds remain unchanged
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.1] - 2026-10-10

- Add a manually triggered canister-recovery workflow for Linux and both macOS
  architectures. Retain source/graph selections, the tested Wasm, test output and
  PocketIC logs; compile the harness before starting the bounded server lifetime
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.0] - 2026-10-10

- **Breaking tooling:** Adopt Shared Tooling 0.3.0. `install-tools` prepares the
  complete host, IC and Rust toolsets in order, then selected cargo-edit;
  `tools-check` admits the same selections offline. Host setup always includes
  ripgrep with PCRE2 and cloc; the optional selection flags are removed. Rerun
  `make install-tools` before validation. CI and admitted release preflight use
  the same complete setup/check path
  ([#15](https://github.com/dragginzgame/ic-jobs/issues/15)).
- Align the explicit recovery harness's Testkit CLI with its selected 0.31.0
  library. PocketIC provisioning remains a separate explicit setup step.
- Require actual canister-trap rejections in recovery checks and use Testkit's
  bounded installation-rate-limit retry after controller snapshot loading
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Prepare the development Wasm target in CI for the consumer's Clippy check.

## [0.4.7] - 2026-10-10

- Add an unpublished canister consumer with authorized task management, stable
  queue storage, timer reconstruction and independent delivery receipts. Add an
  explicit PocketIC harness for upgrade, rollback and lost-result qualification;
  ordinary CI builds it without launching the server
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Include app-owned workspace members in committed release-metadata checks.

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
