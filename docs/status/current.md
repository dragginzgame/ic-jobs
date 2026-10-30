# Current handoff

## Current scope and delivery

Checkout baseline is `26108182368b3ef9687f195feaf2ce6c38021dfc`
(Release 0.17.49). The user selected implementation of Jobs issue 78 and adoption
of new upstream packages. Both are prepared in the working tree. Pending notes
are 0.17.50; package versions remain unchanged. No staging, commit, push, release,
publication, deployed-data reset or issue-state change was performed.

The incoming lockfile already selected Platform 0.8.3, Metrics/Logging 0.17.7,
Wire 0.7.24, Timers 0.32.3 and Build/Host/Testkit 0.70.4. Those selections are
preserved and the catalog minimums now match. Memory/Storage additionally move
from 0.56.0 to published 0.56.1; no other incoming lock selections changed.
The selected Testkit and release-metadata CLIs were prepared and admitted.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`; its snapshot files are unchanged.

## Queue contract and retained installations

[#78 — Replace job-consumer JSON queue persistence with bounded IC Wire payloads](https://github.com/dragginzgame/ic-jobs/issues/78)
is implemented locally. The qualification consumer stores native Wire under the
queue-owned identity `JQW\x01`, separately from Memory's ledger. Admission is
24 KiB total including the four-byte prefix, 1 MiB traversal allocation, 8,192
nodes and depth 32. The existing 32-job/32-receipt and 256-byte message limits
remain. Every record crosses Job::restore; header/task/receipt checks, error
precedence, authorization and commit-before-dispatch ownership remain consumer-owned.

There is no production JSON queue reader, converter, reset or blind redispatch.
The [consumer guide](../../apps/job-consumer/README.md) owns the retained-installation
retirement procedure. Preserve the old binary, backups, attempts and exact
receipts; use the old reader to resolve Running/Uncertain obligations or obtain
explicit owner disposition before retiring an installation. Missing evidence
never establishes retry safety. Fresh qualification runs create new canisters;
source adoption does not authorize destruction of an existing installation.

A frozen Memory 0.56.0 installation retains JSON with Running and Uncertain
attempts and their exact receipts. Current recovery/fresh creation refuse while
preserving the Cell header and payload. Memory bootstrap may update allocation
bookkeeping first. Earlier incompatible CBOR Memory ledgers retain their separate
zero-write refusal proof. Do not clear only a queue or allocation ledger.
[Fixture provenance](../../crates/ic-jobs/tests/fixtures/README.md) identifies the
producer, old-reader proof and preserved artifact location.

## Qualification

Evidence root: `/tmp/ic-jobs-78-upstream.Mso2Zod0/`. It retains incoming inputs,
old source/lock/Wasm and fixture producer, failed development checks, final source
patch/archive, selected dependency graph, tool identities and final Wasm hashes.
Use those dirty-source identities with the baseline; HEAD alone is not this proof.

On Rust 1.91, 68 selected consumer native tests pass: library, API Wire,
composition and stored-record Wire. Four native capture-tool tests pass.
Separate core native/Wasm compiler-floor checks pass, and the actual consumer
Wasm builds on 1.91. Strict consumer native/Wasm and recovery/capture lint pass.
Pin, direct-boundary and all five selected dependency-graph checks pass, including
Serde-only core and no JSON in the qualification Wasm. Formatting/manifest order
and documentation links pass. No full repository CI suite was selected.

The final original Wasm passes all eleven Linux PocketIC recovery tests through
`make test-canister`, with Testkit 0.70.4/PocketIC 16.1.0. Actual evidence is at
`live/jobs-recovery.mDLrLt/` within the evidence root; Wasm SHA-256 is
`e60ea3527f279465b04d76405b7d976a7dd29ea1de97ce43f055596fd3d12fb1`.
This covers same-Wire upgrades, exact blocked attempt/payload retention, real
receiver receipts, lost replies, absent-receipt blocking, corruption/snapshot
recovery, timer rollback and volatile Metrics/Logging reconstruction. No new
optimized-variant, native macOS, provider or deployment acceptance is claimed.
Issue 78 remains open pending delivery and source-matching hosted acceptance.

## Earlier Canic preparation

The separate private Canic contribution remains isolated at
`/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch `codex/jobs-application-adapter`,
commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1` on Canic base
`d09d1692e29878221e53f5e5b71479025c09e7f2`. Its format patch and native/installed
evidence are retained beside that checkout. No sibling checkout was edited.
At that delivery review upstream main was still Release 0.112.0; no PR/push was
created containing its unrelated unpublished release base. Integrate the scoped
commit onto published source and qualify its resulting graph before delivery.
This Jobs queue cut does not retroactively qualify that separate Canic graph.

[Canic #502 — Assess one IC Jobs metadata boundary before adopting it](https://github.com/dragginzgame/canic/issues/502)
remains open. The concrete placement-acknowledgement assessment leaves its
aggregate lease/domain-operation owner unchanged; the private notification
component is a separate consumer. The [integration guide](../canic-integration.md)
does not advertise a shipped Canic mount API.

Prior Jobs preparation remains at `/tmp/ic-jobs-canic-preparation.ew0F1QwW/` and
Memory/Tooling adoption at `/tmp/ic-jobs-tooling0702-memory056.8W4xazfe/`.
[Recovery run 40909177479](https://github.com/dragginzgame/ic-jobs/actions/runs/40909177479)
qualified 132 executions on Linux and both macOS architectures for v0.17.46 /
878e93c752ff5777ad5b11ba2e44611b3c0c319e, Memory 0.55.2 / Platform 0.8.1.
The separate 0.17.47 graph's 44 Linux executions remain at
`/tmp/ic-jobs-issue-qualification.qMGHSSmz/live/jobs-recovery.Tdw2kS/`.
Keep those historical source/graphs distinct from the current proof and any
instruction/cycle-cost claim. Published core runtime APIs remain Serde-only.
