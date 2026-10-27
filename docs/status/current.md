# Current handoff

## Accepted scope

Released baseline: `d2c1164fa8e31675e47f6d96419962f438b5b7e4` (Jobs 0.17.34).
The dependency/Shared Tooling adoption remains local and uncommitted. The latest
request adds published IC Tooling 0.69.0 and completes the Platform 0.7 migration.
The compatible changelog draft remains 0.17.35; package versions remain 0.17.34.
Staging, commits, releases and publication remain maintainer-owned.

Build/Host/Testkit now select 0.69.0. Workspace minimums also require the incoming
coordinated Memory/Storage 0.51.9 and Timers 0.31.9 selections. The user's direct
Platform 0.7 requirement is preserved; the lock selects Platform/macros 0.7.0.
The former Platform 0.6 identity has been removed from the lock and all graph
policies pass without relaxation. The prior Platform singleton blocker is resolved.
Wire/derive remain 0.7.18 and transitive Metrics remains 0.16.3. No application
Rust code or feature selection changed. Documentation now describes Platform 0.7.

The prior reviewed Shared Tooling 0.26.22 adoption remains intact at
`5fc904a935d82ad31f0fba14434b71ccb8c10345`, with the same 120-file selection.
Its governance clarifies Candid boundaries, native errors and strict native input/
output handling; adoption does not itself add executable enforcement. The prior
GitHub-description review remains valid because Jobs' scope has not changed.

## Maintained contracts

The published core remains Serde-only. Tooling's upstream minor change affects
Platform-owned Principal identities in host registry APIs; Jobs selects Platform
0.7 consistently and does not expose those types through the published library.
Core and consumer-owned Timers integration retain separate Rust 1.91 qualification.
Every decoded Job crosses Job::restore; Running/Uncertain effects stay blocked
until explicit disposition. Storage, payloads, authentication, lifecycle, timers
and effect policy remain consumer-owned. Wire/storage bytes, receipt checks,
error precedence and commit-before-delivery ordering are unchanged. No protocol
regeneration or retained-data migration is required. The bounded queue remains
linear; normal releases start fresh and only release-resume selects saved intent.

## Qualification and evidence

`/tmp/ic-jobs-tooling-069.LLBQDe8r/` retains the incoming dirty patch and lock,
full metadata, scoped lock delta, graph/lint logs, selected CLI receipts, incoming
and qualified Wasm, final patch and tool/artifact identities. Incoming Memory/
Storage/Timers/Platform selections were preserved. The scoped Tooling update
replaced the 0.68 family with 0.69 and removed the obsolete Platform 0.6 identity.
Registry publication of Tooling 0.69.0, Memory 0.51.9 and Timers 0.31.9 was checked.

Full locked metadata and all existing native/Wasm dependency, pin and snapshot
policies pass. Development-toolchain Clippy passes for both affected native
packages and consumer Wasm. Core Rust 1.91 native/Wasm checks pass separately;
Rust 1.91 consumer tests pass (29 library, 2 codec, 19 composition). Capture tests
and non-live harness/example cases pass. The qualification consumer's release
Wasm builds on Rust 1.91 with the newly admitted graph. No footprint or runtime
performance improvement is claimed.

The canonical setup/check targets prepare and admit Testkit and release-editor
CLIs at 0.69.0, preserving earlier immutable installations. PocketIC 16.1.0 bundle
admission passes without server startup. The actual consumer-tooling fixture
passes seven phases with substitute tool/server effects, including input drift
refusal, demo routes and frozen/failed artifact capture. The metadata release
fixture passes with the selected editor and real local Git. Formatting and
local documentation checks pass. Full CI, live/optimized recovery and native
macOS qualification were not selected; native/Wasm checks do not establish IC
recovery. Old live evidence does not qualify this source/dependency graph.

The superseded compatible-only adoption and rejected Platform candidate remain
at `/tmp/ic-jobs-dependency-adoption.HHZ153GK/`. That directory retains its own
inputs and limitations; its failure is not the result of the final 0.69 graph.
Earlier cleanup evidence remains at `/tmp/ic-jobs-complexity-4.EAgwljAf/`,
`/tmp/ic-jobs-complexity-3.FqDuU7u5/`, `/tmp/ic-jobs-complexity-2.uetcef2y/` and
`/tmp/ic-jobs-simplify.c9lx1yg4/`. Earlier Linux live evidence at
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older dependencies. The last recorded
three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
