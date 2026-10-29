# Current handoff

## Accepted scope and inputs

Released baseline: `f35bbfebb4dda3e649b92d9d9807c5aa6d8868fb` (Jobs 0.17.44).
The accepted upstream adoption now includes Memory/Storage 0.55.0 alongside
Build/Host/Testkit 0.69.13 and Metrics/Logging 0.17.2.
Root requirements now match these releases. The incoming dirty lock selections
were preserved; no unrelated registry package or feature list was changed.
Platform/macros 0.7.6, Timers 0.32.0 and Wire/derive 0.7.21 remain selected.
Unreleased sibling work is excluded; sibling repositories stayed read-only.

The compatible draft is 0.17.45. Package versions stay 0.17.44; published Jobs
APIs, Serde-only runtime, stored formats and Rust 1.91 floor are unchanged. No
consumer migration or retained-data reset is required. Staging, commits, pushes,
releases and publication remain maintainer-owned. Earlier merge conflicts are
resolved and included in released 0.17.44; no merge operation is pending here.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Producer review and integration

Tooling release `7a44d77f6a923f63c0f1e4c6ff02d3c3bb8e977d` simplifies Cargo
argument construction, admitted input revalidation, streamed artifact discovery,
artifact names/stamps and startup port parsing. Existing API, admission,
freshness and retained-evidence contracts are preserved. Jobs keeps its existing
capture/CLI/server wiring. Contributor CLIs are prepared through
make install-testkit-tools install-release-tools; checks remain offline.
The selected PocketIC client/server pairing remains 16.1.0.

Persistence release `1ff8f93ccb395219a7e28952c40140ea717b78dd` leaves Memory and
Storage library sources/member manifests unchanged from 0.54.1. Its breaking
Metrics Stable format cut affects an adapter absent from Jobs' graph; unpublished
Logging Stable is also absent. No map retirement, epoch change or data reset is
needed here. Governed allocation, backing custody and collection formats remain.

Observability release `775a47fab309c043e5a40ec96e009706d668a49b` leaves Metrics
arithmetic unchanged and adds validated passive Logging checkpoints. Jobs uses
fresh heap-only logs; no checkpoint, storage, lifecycle or reset policy is added.
The consumer still selects Metrics/Logging only on Wasm and in tests. Native DTOs
remain independent primitive projections; keyed Metrics recording stays disabled.

## Maintained runtime and diagnostics

Consumer storage, metadata, lifecycle, effect and timer authority remain local.
Every restore crosses Job::restore; uncertain effects remain blocked until explicit
exact-receipt disposition. Error ordering, retries, scheduling policy, stored
records and existing Wire replies are unchanged. Exclusive allocation custody
and dropping all handles before reopening remain required.

Metrics observes committed dispatch/retry, admitted outcomes and explicit
reconciliation only. Refused operations, stale callbacks, failed writes and idle
turns add nothing. Reports retain u128 totals, actual-overflow flags, trusted IC
version and a nonwrapping reset sequence. Manager-only reset touches neither
stable state nor timers. Reopening starts empty; compare only within one
canister/version and available reset sequence. Settings may advance version
without clearing counts. Diagnostics never authorize retry or imply success.

Logging retains 128 redacted events with 192-byte messages and 32-byte labels.
Manager-only job_logs returns at most eight matches after scanning at most 32
rows; filtered empty pages may carry a continuation. Append refusal increments
an overflow-aware omitted counter without refusing committed work. Reconstruction
starts empty; metrics reset preserves logs. Cursors belong to one heap instance.
There is no age timer, stable log store or log reset. The
[consumer guide](../../apps/job-consumer/README.md#committed-transition-logs)
owns report semantics and the demo's logs [--before SEQUENCE] command.

## Qualification and remaining work

Current evidence: `/tmp/ic-jobs-memory055.HcVkLZC1/` retains the released base,
incoming batch/lock/handoff/Wasm, empty library-source diff, published VCS
identities, bounded two-package update, qualified graph and final artifacts.

Fresh checks pass: snapshot/pins/boundaries and all five graphs, strict native
consumer and Wasm Clippy, 61 Rust 1.91 consumer unit/Wire/composition tests and
Rust 1.91 release consumer Wasm. Formatting, local links, diff and CLI admission
checks bind to the current inputs. Prior native harness Clippy/six tests and seven
tooling fixture phases remain at `/tmp/ic-jobs-upstream.LcvPwy08/`; their scripts,
selected CLI bytes and native dependency paths are unchanged by this update.
Core Rust 1.91 native/Wasm evidence remains applicable to unchanged runtime
source/features/Serde at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`.

Full make ci, live PocketIC and a new native macOS matrix are not selected.
The eleven live recovery cases compile but are ignored during ordinary tests.
No new IC instruction/cycle cost or metrics/logging trap-rollback claim is made.
Current installed recovery and Platform call ownership remain tracked in
[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

Historical [PR #77 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/40861537940)
passed Linux and both macOS hosts on its older graph, including the Make fixture
repair. Prior evidence remains at `/tmp/ic-jobs-pr77.wuvimOgp/`,
`/tmp/ic-jobs-conflicts.GFrtQa01/`, `/tmp/ic-jobs-metrics-platform.kiF7mYHj/`,
`/tmp/ic-jobs-metrics-macos.dHKxquDK/`, `/tmp/ic-jobs-memory054.sqjjQkAo/` and
`/tmp/ic-jobs-adapter-reference.rbwXpr7F/`. Older graphs do not qualify current
installed acceptance. The last recorded three-host live run used Jobs 0.17.14:
[108 successful executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
