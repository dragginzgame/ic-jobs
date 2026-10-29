# Current handoff

## Accepted scope

Released baseline: `6d4f167133741a822a016d394c7bd27b03e87c21` (Jobs 0.17.39).
The accepted batch includes the preceding upstream adoption/Metrics review and
now a packaged adapter reference, integration contract and acceptance checklist.
The compatible changelog draft remains 0.17.40; package versions stay 0.17.39.
Changes are uncommitted. Staging, commits, releases and publication remain
maintainer-owned. No implementation blocker remains. Siblings stayed read-only.

## Adapter reference

Extend the existing packaged `crates/ic-jobs/examples/persisted_job.rs` instead
of adding a second example or new public abstraction. A consumer-owned bounded
snapshot retains one canonical JobRecord and immutable payload. Every load uses
Job::restore. Failed commits discard tentative jobs and preserve committed bytes.
The example selects work, publishes intent after commit, projects its deadline,
simulates an effect, rejects a failed result commit, reconstructs the active
attempt from retained authority, blocks redelivery and resolves an exact receipt.
It also refuses a stale receipt without replacing unresolved state.

Atomic byte replacement, the 4 KiB ceiling, counted destination and receipt are
native example policy/substitutes. They do not establish real storage transactions,
authentication, IC rollback, watchdog delivery or installed upgrade recovery.
Canic retains those boundaries and its own adapter implementation.

The package README includes a self-contained adapter integration contract:
owner responsibilities, bounded restoration, commit/index ordering, discarded
tentative state, post-await reload, exact identities, explicit uncertainty
resolution, host runtime/lifecycle composition and wakeup reconciliation.
`docs/test-consumers.md` links nine native acceptance scenarios plus installed
upgrade/rollback/receipt cases. Root README links the example, contract and checklist.
No library API, storage trait, database dependency, handler registry, endpoint,
lifecycle export or timer provider was introduced. No dependency files changed
in this adapter-reference slice; incoming manifests and lock are preserved.

## Upstream adoption retained

Root minimums select Memory/Storage 0.53.1, Build/Host/Testkit 0.69.8, Platform
0.7.4, Timers 0.31.16 and Wire 0.7.21. The incoming lock already selected these
and transitive Metrics 0.16.5, including aws-lc-rs 1.19.0 / aws-lc-sys 0.47.0;
its bytes remain unchanged. No feature selections changed.

Published Jobs remains Serde-only. Metrics reaches the qualification Wasm through
Timers' unchanged counter/summary arithmetic; the resolved feature set is only
`default`, without `recording`. New measurement windows are unused. No Metrics
API/storage migration or automatic recording/endpoint is needed.

Memory 0.53.1 fixes backing-claim teardown ordering. Truthful backing identity,
duplicate-owner refusal and custody through detached handles remain required.
Full manager checks occur at construction, bootstrap, lease opening and growth;
ordinary IO trusts exclusive custody while checking bounds and poisoning.
Prohibited raw edits need not be detected before ordinary IO. Jobs IC costs remain
unmeasured. Every restore crosses Job::restore and uncertain effects stay blocked;
exact receipts, error ordering, scheduling/retry/management behavior and retained
formats remain unchanged. Rust support remains 1.91 for core and integration.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 payloads). Selected Tooling 0.69.8
CLIs and PocketIC 16.1.0 were prepared/admitted; earlier installations are retained.

## Qualification and evidence

`/tmp/ic-jobs-adapter-reference.rbwXpr7F/` retains incoming edits/example,
format/lint/docs logs, both package attempts, unmodified unpacked archives,
Rust 1.91 example execution, commands, identities and final patch. The first
isolated run refused a copied workspace manifest in the evidence parent before
compilation. That snapshot was renamed to incoming-Cargo.toml; the final archive
and its example run pass without modifying packaged sources or manifests.

The final packaged example passes its assertions on Rust 1.91. The extracted
example and README match working files byte for byte. Archive verification,
strict example Clippy, doctests, strict rustdoc, formatting, links and diff checks
pass. This is native package/example qualification, not Canic integration.

Prior full metadata/graph/lint and focused runtime evidence remains at
`/tmp/ic-jobs-upstream-metrics.BHriS83n/`: all five graphs, 51 consumer tests,
focused harness tests, Rust 1.91 consumer Wasm and tooling fixtures pass. Those
consumer sources/dependencies remain unchanged. Core Wasm floor evidence at
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/` remains applicable to unchanged runtime
source/Serde dependencies. No full `make ci`, live PocketIC or native macOS
qualification was selected for this slice.

Prior Shared hook/formatter evidence is at `/tmp/ic-jobs-shared-02625.5rE0846k/`.
Linux live evidence at `/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs.
The last recorded three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
