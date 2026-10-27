# Current handoff

## Accepted scope

Released baseline and local HEAD: `4223530b60eefa8653c05e132b1528ebf0f5b2d1`
(Jobs 0.17.32). All ten items from the third complexity review are implemented
locally and uncommitted. The compatible changelog draft is 0.17.33; package
versions remain 0.17.32. Staging, commits, releases and publication remain
maintainer-owned.

The consumer returns named dispatch results and only the delivery envelope
needed by its callers. Native fixtures share committed-byte reconstruction and
inspection-reply projection. Admission/management cases are named rather than
selected by numeric or array positions. Capture tests reuse the shared cleanup
guard, which preserves fixture evidence on panic.

The three demo commands delegate launch plumbing to one local helper. Make
still owns compilation and initial server admission; Testkit owns server
lifetimes. The launcher is a checkpointed validation input. Retention fixtures
now run without metadata-editor or formatter setup. Deadline display shares a
pure formatter, and the watch banner precedes its loop.

## Maintained selections and contracts

Core source, manifests, dependencies and lockfile are unchanged. Build/Host/
Testkit remain 0.68.9, Memory/Storage 0.51.5, Timers 0.31.5, Platform/macros
0.6.19, Wire/derive 0.7.18 and transitive Metrics 0.16.3. Shared Tooling remains
0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`. Development Rust is
1.99.1; core and consumer-owned Timers integration retain separate 1.91 floors.

The published core remains Serde-only. Every decoded Job enters through
`Job::restore`; Running/Uncertain work stays blocked until explicit disposition.
Storage, payloads, authentication, lifecycle, timers and effect policy stay
consumer-owned. RPC/storage formats, receipt comparisons, error precedence and
commit-before-delivery ordering are unchanged. No migration or client
regeneration is required. Watch interruption still precedes advancing IC time.
The 32-job lookup remains linear pending a measured need. Normal releases start
fresh; only explicit `release-resume` selects saved intent.

## Qualification and evidence

`/tmp/ic-jobs-complexity-3.FqDuU7u5/` retains the incoming source archive, final
patch, input/tool identities and focused logs. Rust 1.91 consumer tests pass
(29 library, 2 codec, 19 composition). Capture tests pass (4), along with the
non-live queue example and recovery harness tests; live scenarios remain ignored.
Native and consumer Wasm Clippy pass with both Rust 1.91 and development Rust.
Unchanged core-floor qualification remains separate in the preceding evidence.

Shell syntax, ShellCheck, formatting, documentation links and release-command
routing pass. The actual retention Make target passes with the metadata-editor
environment unset; the metadata group also passes through its prepared-tool path.
The consumer-tooling fixture passes seven isolated phases, including actual
Make/demo-launcher routing with paths containing spaces, admission refusal,
frozen-artifact custody and validation stopping if the lock, Testkit selector or
demo launcher changes. Cargo/server/optimizer effects are substituted there.

An initial direct script invocation omitted the capture-binary build prerequisite
and stopped in artifact capture. Its evidence remains at
`/tmp/jobs-consumer-tooling.Y8SQaD/`. The supported Make route built that
prerequisite and passed; the final fixture rerun also passed.

No full CI, fresh live/optimized recovery or native macOS qualification was
selected. Native tests and Wasm compilation do not establish IC recovery.
Earlier batch evidence remains at `/tmp/ic-jobs-complexity-2.uetcef2y/` and
`/tmp/ic-jobs-simplify.c9lx1yg4/`. Earlier Linux live evidence at
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older dependencies, not this source.
The last recorded three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
