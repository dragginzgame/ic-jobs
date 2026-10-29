# Current handoff

## Accepted scope and inputs

Released baseline: `878e93c752ff5777ad5b11ba2e44611b3c0c319e` (Jobs 0.17.46).
The user authorized completing outstanding issues, current CI inspection and
qualification of incoming Timers 0.32.1/Metrics 0.17.4 lock selections. Root
requirements now match those releases. The retained Wire/derive 0.7.23 selection
is also qualified. Build/Host/Testkit 0.70.0, Platform/macros 0.8.1, Memory/Storage
0.55.2 and Logging 0.17.2 remain selected. Siblings stayed read-only.

The compatible draft is 0.17.47; package versions remain 0.17.46. Public Jobs
APIs, Serde-only runtime, stored formats, principal bytes and Rust 1.91 floor
are unchanged. No data reset, physical retry or new lifecycle/storage policy is
introduced. Staging, commits, pushes and release/publication remain user-owned.
Shared Tooling stays reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Dependency and ownership review

Timers release `a1dc2856a515ae5bbb8d48bf8e6e49fbc6d881a9` leaves runtime source
unchanged from 0.32.0 and adopts Platform 0.8.1. All selected consumers now share
one Platform facade, executor 2.0.0, IC0 1.2.0 and entrypoint generator. The graph
again requires exactly one Platform identity; the two-facade allowance is removed.
Wasm excludes application principal conversion and SHA-2, and native DTOs exclude
ic_principal and SHA-2. Stable-structures retains its independent Principal use.

Metrics 0.17.4 only refactors histogram restoration relative to 0.17.3; counter,
window and transport behavior are unchanged. Wire 0.7.23 leaves runtime sources
and member manifests unchanged from 0.7.22. Existing fields, budgets, frame IDs,
encoded principal values and the independent frozen vectors remain unchanged.
The consumer uses neither stable diagnostics adapter; their format cuts do not
require map retirement, reset epochs or data migration here.

The application submits each owned Wire envelope directly through Platform's
Call::unbounded_wait once. It commits Running intent before dispatch, preserves
exact job/occurrence/attempt/payload/receipt identities, and blocks uncertain
work until explicit disposition. Rejects/invalid replies are not no-effect proof.
Reopening, upgrade, timeout or abandoned futures never authorize redispatch.
Platform explicitly supports this raw borrowed-method route; no ic-call adapter,
generic call layer, fallback or mandatory Relay dependency is needed.
The [consumer guide](../../apps/job-consumer/README.md) records this boundary.

Every decoded job still crosses Job::restore. Metadata, allocation, stable state,
clock, timer, lifecycle and effect authority stay consumer-owned. Error ordering,
retry/scheduling policy, allocation custody and existing stable/Wire formats stay
intact. All handles drop before reopening a backing.

Metrics observes committed transitions with wide totals, actual-overflow flags
and nonwrapping reset sequence; reset changes no job or timer authority. Logs
remain bounded, redacted and heap-only, independent of metrics reset. Diagnostics
never establish effect completion or permit retry. Reconstruction starts empty;
heap window/cursor identity cannot promise durable continuity.

## Local qualification

Evidence: `/tmp/ic-jobs-issue-qualification.qMGHSSmz/` retains incoming inputs,
producer reviews, qualified metadata/lock, ownership review, command logs and
final patch. Local source is released HEAD plus the saved working-tree changes.
Fresh passes include snapshot/pins/boundaries/all five graphs, strict native
consumer/harness and Wasm Clippy, 61 Rust 1.91 consumer tests, six native harness
tests, and the Rust 1.91 release Wasm built by the live gate. Server/optimizer
and both selected Tooling CLI bytes are prepared/admitted. Links/format/diff checks
cover the maintained documents. Full make ci was not selected locally.

Actual Linux PocketIC 16.1.0 evidence is retained at
`/tmp/ic-jobs-issue-qualification.qMGHSSmz/live/jobs-recovery.Tdw2kS/`.
All eleven recovery cases pass on original/O3/Os/Oz Wasm: 44 installed executions.
The existing watchdog trap case now additionally checks zero dispatch/success/
permanent-failure metrics after rollback. This new assertion is uncommitted and
qualified on Linux; hosted tag runs do not contain it. No instruction/cycle cost
claim is made. Source, lock, optimizer, Wasm hashes and server logs are retained.

## Delivered acceptance and hosted work

Latest delivered [0.17.46 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/40905859960)
passes Linux, Apple Silicon macOS and Intel macOS. Issue
[#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76)
is closed with implementation/current-CI acceptance evidence. No histogram is
needed by the selected fixed-counter reporting requirement.

[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75)
remains open while [hosted recovery 40909177479](https://github.com/dragginzgame/ic-jobs/actions/runs/40909177479)
finishes. It was explicitly dispatched on immutable v0.17.46/878e93c: eleven cases
x original/O3/Os/Oz x Linux/Apple Silicon/Intel. Linux and Apple Silicon passed;
Intel is preparing Testkit. Its result must be observed before full acceptance or closing
that issue. Its delivered graph uses Timers 0.32.0/Metrics 0.17.3/Wire 0.7.22;
it is distinct from the local newer-dependency qualification above.

Prior evidence remains at `/tmp/ic-jobs-tooling070-wire0722.q0s4pZzw/`,
`/tmp/ic-jobs-platform08.IrOFhP58/`, `/tmp/ic-jobs-memory055.HcVkLZC1/`,
`/tmp/ic-jobs-upstream.LcvPwy08/` and `/tmp/ic-jobs-adapter-reference.rbwXpr7F/`.
Core floor evidence remains applicable to unchanged source/features/Serde at
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`. Older hosted three-host live evidence
used Jobs 0.17.14; it is retained historically and is superseded only when the
new source-bound run completes successfully.
