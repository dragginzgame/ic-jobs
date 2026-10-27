# Current handoff

## Accepted scope

Released baseline and local HEAD: `ff9ec9f` (Jobs 0.17.33). All ten items from
the fourth complexity review are implemented locally and uncommitted. The
compatible changelog draft is 0.17.34; package versions remain 0.17.33.
Staging, commits, releases and publication remain maintainer-owned.

Demo scheduling requests and labels now come from one branch; pagination uses
one local page limit. Valid native installation requests share their Wire
identity/limit selection, while malformed installation fixtures stay independent.
The consumer shares delivery-sender authorization before input/storage checks,
returns named summary totals, and binds outbound delivery names to their existing
identities with a closed internal selection. This adds no handler registry.

Publication tests isolate stubbed and real-Git environments in subshell functions,
with the outer fixture retaining completion/cleanup ownership. Native composition
fixtures have explicit exports/imports and serialize fixed initial contents
without constructing a temporary Vec. Generic codec proof selection is documented
in the consumer README; Jobs retains its own envelope/budget/refusal evidence.

## Maintained selections and contracts

Core source, manifests, dependencies and lockfile are unchanged. Build/Host/
Testkit remain 0.68.9, Memory/Storage 0.51.5, Timers 0.31.5, Platform/macros
0.6.19, Wire/derive 0.7.18 and transitive Metrics 0.16.3. Shared Tooling remains
0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`. Development Rust is
1.99.1; core and consumer-owned Timers integration retain separate 1.91 floors.

The published core remains Serde-only. Every decoded Job enters through
`Job::restore`; Running/Uncertain work stays blocked until explicit disposition.
Storage, payloads, authentication, lifecycle, timers and effect policy stay
consumer-owned. RPC/storage bytes, method identities, receipt comparisons,
error precedence and commit-before-delivery ordering are unchanged. No migration
or client regeneration is required. Watch interruption still precedes advancing
IC time. The bounded 32-job lookup remains linear pending a measured need.
Normal releases start fresh; only explicit `release-resume` selects saved intent.

## Qualification and evidence

`/tmp/ic-jobs-complexity-4.EAgwljAf/` retains incoming source, final patch,
commands/tool identities and focused logs. Rust 1.91 consumer tests pass
(29 library, 2 codec, 19 composition), including unauthorized delivery/receipt
refusal before decoding corrupt storage without replacing its bytes. Capture
tests pass (4), along with the non-live queue example and recovery harness cases;
live cases remain ignored. Development-toolchain Clippy passes for both affected
native packages and consumer Wasm. Consumer Wasm compilation passes on Rust 1.91;
unchanged core-floor qualification remains separate in preceding evidence.

Publication ordering/race/refusal checks and real local Git eligibility pass
with registry transport substituted and no upload. Release-fixture retention
probes, shell syntax, ShellCheck, formatting and documentation links pass. The
reviewed Shared Tooling snapshot verifies all 120 files. ShellCheck initially
flagged the intentionally isolated PATH changes; narrow, explained annotations
now record that phase-local contract. The initial diagnostic is retained.

Wire 0.7.18's packaged VCS identity is
`c748379325266d7fa614fefee9d745eb4a88acca`. Its native tests own exhaustive
prefix truncation and nonminimal integer proofs. The exact source and packaged
identity are retained in this batch's evidence; upstream tests were inspected,
not rerun. Jobs retains representative request/reply truncation, malformed and
retired frames, typed result/absence behavior, exact envelopes and actual limits.
No speed or Wasm-footprint improvement is claimed.

No full CI, fresh live/optimized recovery or native macOS qualification was
selected. Native tests and Wasm compilation do not establish IC recovery.
Earlier batch evidence remains at `/tmp/ic-jobs-complexity-3.FqDuU7u5/`,
`/tmp/ic-jobs-complexity-2.uetcef2y/` and `/tmp/ic-jobs-simplify.c9lx1yg4/`.
Earlier Linux live evidence at `/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses
older dependencies, not this source. The last recorded three-host live run
qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
