# Current handoff

## Accepted scope and checkout

Checkout baseline: `6fbf86b5ed505722aaaacfbc344b58544d5829b9` (Release 0.17.48).
The user selected preparation for a Canic-owned Jobs adapter: stored Wire tests,
a stronger packaged example, a Canic integration guide and correction of confusing
README/consumer/safety/status documentation. Draft notes target 0.17.49; package
versions remain 0.17.48. The published library API, runtime source and Serde-only
dependency boundary are unchanged. No Canic adapter/mount API is implemented.

The incoming dirty Cargo.lock updates are unrelated and preserved byte-for-byte;
they are not a new dependency-adoption batch. Current relevant selections remain
Build/Host/Testkit 0.70.2, Memory/Storage 0.56.0, Platform/macros 0.8.2,
Timers 0.32.1, Metrics 0.17.4, Logging 0.17.2 and Wire/derive 0.7.23.
Shared Tooling stays reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`.
Sibling checkouts are read-only. No staging, commit, push, own version change,
release, publication, deployment, hosted workflow or retained-data reset selected.

## Adapter preparation

The [Canic integration guide](../canic-integration.md) maps storage, lifecycle,
timers, management, identities and receipts to their owners. The
[package contract](../../crates/ic-jobs/README.md#adapter-integration-contract)
owns commit/recovery ordering; the
[acceptance checklist](../test-consumers.md#adapter-acceptance-checklist)
links executable reference scenarios. Canic retains its own dependency versions;
this library does not require the qualification fixture's Memory 0.56 selection.

Native stored Wire tests cover all job states/schedules, wide IDs, attempts,
retry history, rescheduled cadence origin and refusal of inconsistent history,
truncated/trailing input and byte-limit overflow. Every decoded record used as
authority crosses Job::restore. Wire remains outside the published core crate.
These tests select no Canic stable envelope and change no canister JSON bytes.

The runnable example demonstrates rejected intent/result commits, cold blocked
restoration, exact receipt recovery, duplicate receipt refusal, safe retry with
an unchanged logical effect/payload, late-continuation refusal and preservation
of editable metadata committed during delivery. Its byte store/destination are
native substitutes. `make test-consumers` now runs composition, stored Wire and
the example on Rust 1.91; focused commands remain in the README.

Jobs Running/Uncertain state stays blocked until explicit result or retry-safety
evidence. Canic lease expiry/watchdog takeover cannot supply that disposition.
Existing funding/pool/renewal domain protocols need individual assessment before
replacement. Canic must implement and qualify its own actual storage, endpoints,
authorization, lifecycle and installed recovery; reference results are not Canic
acceptance. No registry, lifecycle export, storage trait or timer provider added.

## Current checks and evidence

`/tmp/ic-jobs-canic-preparation.ew0F1QwW/` retains incoming patch/lock/handoff,
qualified metadata, commands, source/lock identities, final patch and logs.
Rust 1.91 passed 19 composition tests, three stored Wire tests and the expanded
example. Strict Clippy passed for the changed example and stored Wire target.
Documentation references, command consistency and diff review cover the edited
guides. No new live PocketIC, Wasm build, full suite or native macOS run selected;
production/runtime source and dependency requirements are unchanged.

## Persistence boundary and earlier qualification

Memory 0.56 uses bounded Wire ledger/Cell values (`ICML`/`ICMWIRE1`) and refuses
earlier CBOR ledgers without writes. Application queue JSON and Jobs records are
unchanged. Use fresh qualification state or an explicitly owned/qualified
transition before replacing retained data. Preserve old binaries/backups and
outstanding effect evidence. Never clear only the ledger, auto-reset on refusal
or infer reinstall permission from dependency adoption. See the
[consumer boundary](../../apps/job-consumer/README.md)
and [frozen fixture provenance](../../crates/ic-jobs/tests/fixtures/README.md).

The 0.17.48 adoption working tree (base cdb48cc9 + retained patch) passed 62
consumer and six native harness tests, strict native/Wasm lint, release Wasm build,
tooling fixtures and graph admission. Its evidence is at
`/tmp/ic-jobs-tooling0702-memory056.8W4xazfe/`; it does not qualify installed
Memory 0.56 recovery or this turn's incoming lock changes. Unchanged core floor
evidence remains at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`.

[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75)
and [#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76)
are closed. [Recovery run 40909177479](https://github.com/dragginzgame/ic-jobs/actions/runs/40909177479)
passed eleven cases on original/O3/Os/Oz across Linux and both macOS architectures
(132 executions). It tests v0.17.46/878e93c752ff5777ad5b11ba2e44611b3c0c319e,
Memory 0.55.2, Platform 0.8.1, Timers 0.32.0, Metrics 0.17.3 and Wire 0.7.22.
Hosted artifacts retain source/lock/tools/Wasm/logs; closure observations remain
at `/tmp/ic-jobs-close75.TxcKAWyv/`. Delivered
[0.17.47 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/40917200431)
passed all three hosts. Its separate Linux live graph used Memory 0.55.2,
Platform 0.8.1, Timers 0.32.1, Metrics 0.17.4 and Wire 0.7.23: 44 executions
including metrics trap rollback, retained under
`/tmp/ic-jobs-issue-qualification.qMGHSSmz/live/jobs-recovery.Tdw2kS/`.
These are historical graph-specific proofs, not current Memory 0.56 or Canic
composition receipts. Native tests do not establish IC recovery or cycle costs.
