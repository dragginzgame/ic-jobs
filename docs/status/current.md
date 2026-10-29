# Current handoff

## Accepted scope and inputs

Released baseline: `23ae2c813bfd60ca20ec377f6745045235e33b41` (Jobs 0.17.45).
The accepted pending batch adopts Platform/macros 0.8.1, now extended with
Build/Host/Testkit 0.70.0 and Wire/derive 0.7.22. Root requirements match the new
Tooling/Wire selections; existing feature lists and frozen vectors are retained.
Concurrent lock updates are preserved: Memory/Storage 0.55.2 and Metrics 0.17.3.
Logging stays 0.17.2 and Timers 0.32.0. Sibling repositories stay read-only.

The compatible draft remains 0.17.46; package versions remain 0.17.45. Published
Jobs APIs, Serde-only runtime, Rust 1.91 floor, stored formats and principal bytes
are unchanged. No data reset or published-library migration is needed. Staging,
commits, pushes, releases and publication remain maintainer-owned.
Shared Tooling stays reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Tooling and Wire adoption

Tooling release `ecc8d61d19f0b711788f7e10e5dd728d0db6ff60` adopts Platform 0.8.1.
Ledger values and owned Registry principals now have that Platform identity;
Jobs uses neither interface and needs no such caller migration. Testkit explicitly
selects principal-external and preserves its existing external Principal API.
Build/Host and the selected Testkit runtime sources are unchanged from 0.69.14.
The matching CLIs use existing make install-testkit-tools/install-release-tools
setup and offline admission; PocketIC remains the 16.1.0 client/server pair.

Wire release `ac08fdc1757b7d3f1483caecadd7ac1c817a30d8` changes producer
qualification dependencies/comparison coverage only. Wire/derive runtime sources
and member manifests are unchanged from 0.7.21. Jobs retains method identities,
Wire/JSON bytes, decode/reply/node/depth/allocation budgets and admission behavior.
The frozen principal-platform076.json vectors are preserved, never regenerated
with the new codec. Their old producer and hash remain in the
[fixture guide](../../apps/job-consumer/tests/fixtures/README.md#principal-transport-continuity).

## Platform selections and runtime contract

Platform 0.8.1 producer `10c2610511d6a5525816185c999b25fd159c1d89` retains owned
principal bytes/Serde and CDK runtime behavior. Conversion/hashing are explicit.
Normal native DTOs select principal; consumer tests select principal-self-authenticating.
The harness selects pocket-ic plus hashing, with external conversions implied
by PocketIC. The deployed app selects principal/cdk without hashing/conversion.
ICRC DTOs and native profiler source-profile requirements are outside Jobs' features.

Memory/Storage 0.55.2 and native Testkit now share Platform 0.8.1. Published
Timers 0.32.0 still uses Platform 0.7.6 solely for executor/ic0 re-exports. The
graph admits at most two facade identities while requiring one Timers, executor
2.0.0, IC0 1.2.0 and Platform-macros 0.8.1. Wasm excludes principal-external and
SHA-2; native DTOs exclude ic_principal and SHA-2. Stable-structures independently
retains ic_principal. Candid/full CDK exclusions remain intact. There is no sibling
override or compatibility facade. The [graph guide](../../README.md#dependency-exposure)
and [consumer guide](../../apps/job-consumer/README.md) own these selections.

Consumer metadata, storage, lifecycle, timer and effect authority remain local.
Every restore crosses Job::restore; uncertain effects stay blocked until explicit
exact-receipt disposition. Error ordering, retry/scheduling policy and stored
records are unchanged. Exclusive custody and dropping all handles before reopening
remain required. Jobs uses neither Metrics Stable nor Logging Stable.

Metrics records successful committed transitions only, retaining u128 totals,
overflow quality, trusted IC version and nonwrapping reset sequence. Reset cannot
change durable records or timers. Reopen starts empty; compare within one
canister/version and available reset sequence. Logging retains 128 redacted heap
events with 192-byte messages and 32-byte labels. Its protected reader scans at
most 32 rows and returns eight matches; filtered empty pages may continue.
Refused appends increment an overflow-aware omitted counter without refusing
committed work. Reopen clears logs; metrics reset preserves them. Heap cursors
are never durable identity. No stable log store, age timer or log reset is added.

## Qualification and remaining work

Current evidence: `/tmp/ic-jobs-tooling070-wire0722.q0s4pZzw/` retains incoming
pending patch/lock/handoff/frozen vectors/Wasm, producer review and VCS identities,
empty Wire runtime diff, qualified graph, command logs and final artifacts.
An initial offline metadata attempt lacked already-selected Wire source; explicit
locked preparation completed before qualification. Passing evidence is graph-bound.

Fresh passes: all five graphs and snapshot/declaration checks; strict native
consumer/harness and Wasm Clippy; 61 Rust 1.91 consumer tests, six native harness
tests and release consumer Wasm; both selected CLI setup/admission, all seven
consumer tooling phases, release-metadata fixtures, formatting, links and diff.
Exact outcomes and identities are in the
evidence directory. Core 1.91 native/Wasm evidence remains applicable to unchanged
source/features/Serde at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`.

Full make ci, native macOS and live PocketIC are not selected. Eleven live
recovery cases remain explicitly gated; compilation/native fixtures establish
neither IC recovery nor instruction/cycle/rollback costs. Current installed
recovery and Platform call ownership remain tracked in
[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

Prior evidence remains at `/tmp/ic-jobs-platform08.IrOFhP58/`,
`/tmp/ic-jobs-memory055.HcVkLZC1/`, `/tmp/ic-jobs-upstream.LcvPwy08/`,
`/tmp/ic-jobs-pr77.wuvimOgp/` and `/tmp/ic-jobs-adapter-reference.rbwXpr7F/`.
[PR #77 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/40861537940) passed
Linux and both macOS hosts on its older graph. The last recorded three-host live
run used Jobs 0.17.14:
[108 successful executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
