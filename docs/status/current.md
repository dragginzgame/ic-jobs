# Current handoff

## Source and pending work

Delivered Jobs **0.10.3**: `37937543a58cd1fd54eea8dbd9bb0382f658a194`.
Pending compatible **0.10.4** uses direct owned-row extraction and shared receipt
lookup, avoids cloning equal duplicate acknowledgements, and reconciles fault
endpoints from their committed dispatch deadlines. Their await/trap boundaries
and public Candid replies are unchanged.

The follow-up centralizes full-queue deadline derivation on `LoadedSnapshot` and
shares pure Candid reply decoding. Exact receipt comparison and outcome policy
stay in each caller; decode failure cannot authorize replay. Deliberate fault
endpoints retain strict decoding. Storage reads still restore every record through
`Job::restore`; stored shape, keys, ordering and effect disposition are unchanged.

Incoming Cargo selections are preserved: Memory 0.38.0, Testkit 0.42.0, Timers
0.22.5 and Build 0.3.2. Own versions, index, Git refs, release and publication
remain maintainer-owned. No release or upload ran.

## Current proof

Evidence: `target/qualification/deadline-reply-cleanup/`, including selected-source
hashes. Rust 1.88 consumer tests cover the existing storage/transition/receipt
contracts plus pure reply decoding: valid evidence, explicit absent receipt,
transport failure, malformed Candid and application refusal. Release Wasm,
capture tests, recovery harness compilation and strict native/Wasm Clippy pass.
Core/timers native/Wasm floor checks, formatting, snapshot/pin admission and local
links pass. The reply helper is native-testable and shared by the actual canister;
these tests do not execute inter-canister calls.

The preceding extraction proof remains in
`target/qualification/receipt-extraction-cleanup/`: multi-row reads, duplicate
acknowledgements, mismatch/authority refusals and reopen preserve stable bytes
and ordering. Backing-memory instrumentation proves those reads/idempotent paths
perform zero writes. Older receipts, proposals and tool checks are preserved in
[frozen history through 0.10.3](history-through-0.10.3.md), with their original
source/graph scope; earlier history remains in
[history through 0.9.1](history-through-0.9.1.md).

## Acceptance boundaries

Delivered 0.10.3 [CI 38590045221](https://github.com/dragginzgame/ic-jobs/actions/runs/38590045221)
is queued at inspection. The dirty 0.10.4 tree has no hosted qualification.
Full `make ci` and live PocketIC recovery were not selected. Local Linux checks
do not establish macOS Intel/Apple Silicon acceptance or real IC rollback,
installed upgrades and timer delivery for this source/graph.

Native acceptance remains with [#41](https://github.com/dragginzgame/ic-jobs/issues/41),
[#42](https://github.com/dragginzgame/ic-jobs/issues/42),
[#43](https://github.com/dragginzgame/ic-jobs/issues/43),
[#44](https://github.com/dragginzgame/ic-jobs/issues/44) and
[#46](https://github.com/dragginzgame/ic-jobs/issues/46).
Shared fixture/completion adoption is tracked in
[#47](https://github.com/dragginzgame/ic-jobs/issues/47) and
[#48](https://github.com/dragginzgame/ic-jobs/issues/48); active wrappers still use
the reviewed Shared 0.12.7 snapshot. Historical upstream proposals remain linked
in the frozen handoff; their owning GitHub issues carry current status.

Last inspected live evidence remains Jobs 0.8.1
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.
It does not qualify this pending source.

See [architecture](../../README.md), [safety](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md) and
[delivery procedure](../releasing.md).
