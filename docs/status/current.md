# Current handoff

## Selected source and dependencies

The released baseline is `fa951dc` (Jobs 0.17.26). Incoming work already selects
Memory/Storage 0.51.0, Timers 0.31.0, transitive Metrics 0.16.1, Tooling 0.68.1
and Wire/derive 0.7.8. This batch preserves those selections and requires Wire
0.7.8 in the catalog. Platform/macros stay 0.6.15. Metrics remains transitive;
Jobs does not consume the stable adapter moved to IC Persistence. Metrics 0.16
arithmetic and Jobs' stored formats need no conversion or reset.

Reviewed released Metrics `b6193cb` (0.16.1) and Wire `dece7fc` (0.7.8). Supplier
drafts are excluded. Wire's current metadata/browser changes require no Jobs
field adoption or generated clients; existing runtime frames and budgets remain.
The incoming Timers/Memory minor updates exchange no public core Jobs types.

The native queue client uses private typed Clap commands instead of string/type
lookups. Existing command names, aliases, flags and validation are retained.
Clap derive is enabled only in harness dev-dependencies, where the example lives.
The CLI and recovery harness delegate normal management RPC framing/admission to
Testkit's Wire feature through one native-only projection of consumer-owned
method names, IDs and limits. Recovery exposes original typed RejectResponses;
codec failures still fail the harness and deliberately malformed calls stay raw.
No transport retry, server ownership or canister scheduling policy was added.

Shared Tooling remains 0.26.13 at `2f4a95f88743982716f12def87f0a30c197dd07b`, exported
from its clean committed source through the canonical preview/refresh workflow.
The snapshot verifies 117 payloads. Common executable pins, Rust 1.99.1 development
and Rust 1.91 core/consumer floors are unchanged. Jobs still selects only
`wasm-opt`; the release editor and Testkit CLI are prepared under current policy.
Jobs enables the common external IC dependency inventory, retaining local
bans on direct Platform macros and the retired Testkit Candid adapter.

The compatible batch is recorded in the 0.17.27 draft. Package versions, staging,
commits, release delivery and publication remain user-owned.

## Wire and ownership contracts

Wire 0.7 removes decode-time reserialization and `Error::Noncanonical`.
Decode-only helpers require `DeserializeOwned`; encode-only helpers require
`Serialize`. Nonminimal integers still refuse as `Malformed`. Existing directional
frame/traversal budgets, emitted layouts and method identities are unchanged.
The Deserialize-only reply test and existing malformed/limit/identity tests
cover the affected Jobs boundary.

The fixture's receipts compare complete typed delivery envelopes, including
payload and attempt identity. They do not reconstruct raw frames for hashes or
signatures. Future exact-byte retries/signatures must retain original immutable
frames: successful decoding no longer proves identical re-encoding. The
[consumer guide](../../apps/job-consumer/README.md) records this contract. Rebuild
fixture and native harness together; stored JSON needs no conversion or reset.

The published core remains Serde-only. Consumers own storage, payloads,
authentication, lifecycle, scheduling integration and effect disposition. Every
decoded Job crosses `Job::restore`; Running/Uncertain effects remain blocked until
explicit reconciliation. No handler registry, database or direct timer provider
was added. The consumer Wasm graph retains one Platform identity and excludes
Candid/Host; native PocketIC dependencies do not establish IC recovery.

## Qualification and retained evidence

Inputs, prior handoff, source/lock digests and focused logs are under
`target/qualification/native-cleanup-metrics-wire/`. They bind source `fa951dc`,
the incoming dependency update and this native-tooling/catalog patch.
The maintainer committed the code batch as `d210e86` while checks ran; its Rust,
catalog and lockfile bytes match the tested selection. These final validation
notes were added after that commit.
Native consumer/Wire, capture, parser, composition and memory-observer tests
pass. Separate core/consumer Rust 1.91/Wasm checks, native/Wasm strict Clippy,
all five graph contexts, dependency boundaries/pins, snapshot integrity,
formatting, doc links and consumer-tooling fixtures pass. Prepared 0.68.1 tools
retain their existing admission and ownership boundaries.

Explicit Linux PocketIC qualification passes `make test-demo test-demo-cli
test-canister`: seconds/minutes/weeks and recurrence management, real CLI
quit/EOF/idle-Ctrl+C cleanup, and all nine recovery scenarios on original Wasm.
The installed Wasm SHA256 is
`bd6c9884c935827556e00975d9f09f922c5778967997b0a98973762ce596e55f`.
Retained IC evidence is `/tmp/jobs-recovery.Y82NZe`; the full selected run is
`live-canister.log` in this task's qualification directory. The first attempt's
sandbox-localhost bind failure is retained separately; the passing run executed
outside that sandbox. Expected invalid-init trap diagnostics are passing refusal
cases, not failed scenarios. Full CI, optimizer variants and fresh native macOS
qualification were not run; the Linux result does not establish those claims.

Prior Timers/Platform evidence remains at
`target/qualification/timers-03012-platform-0615/`. Wire 0.7.1/Tooling 0.68 evidence remains at
`target/qualification/wire-071-tooling-068/`. Wire/Shared adoption evidence remains at
`target/qualification/upstream-wire-070/`, including the native 330-value frame
comparison and recovery-fixture alignment. Tooling 0.67 evidence remains at
`target/qualification/tooling-067/`.
The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its source-bound artifacts remain in `target/qualification/recovery-01714/`;
they do not qualify the current graph. Preserve earlier qualification directories,
failed attempts, release plans and unresolved effects.
