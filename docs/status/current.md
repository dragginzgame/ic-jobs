# Current handoff

## Source and authority

Delivered Jobs **0.9.2**: `9421b97fb395cb2f7b1e3716b5c72162680ae731`.
The working tree prepares compatible **0.9.3**; own package metadata stays 0.9.2.
Release/publication, staging and Git refs remain maintainer-owned.

Adopted Shared Tooling **0.11.4** from exact public source
`031d214ed044705b989d43e2379f30e6a3d44749`, through a clean temporary checkout
and its canonical exporter. Sibling dirty files remain untouched. The 91-file
snapshot retains the old roots, adds common publication admission and its fixture
lifecycle companion, and explicitly selects the catalog-bindings guide required
by the updated snapshot-drift task. The initial missing-link failure is retained;
[Shared #168](https://github.com/dragginzgame/shared-tooling/issues/168) records the
producer companion gap. Common host/Rust/IC pins and digests are unchanged.

Incoming Cargo.lock already selected IC Host **0.17.7** and Metrics **0.9.2**.
Its bytes are preserved. Official registry metadata confirms all three selected
Host packages are non-yanked, match their locked checksums and retain Rust 1.88.
Host remains confined to the unpublished Testkit recovery harness; core scheduling,
job records and canister storage authority are unchanged.

## Owner convergence and focused evidence

Jobs' publication wrapper delegates source/version/tag/destination/registry
admission and post-observation rechecks to the canonical Shared checker.
Jobs retains package/features/upload policy and its existing metadata adapter.
No release or upload was performed. [#42](https://github.com/dragginzgame/ic-jobs/issues/42)
still awaits delivery/final-source consumer acceptance and independent IC Build
artifact-retention adoption under [Build #14](https://github.com/dragginzgame/ic-build/issues/14).
Prepared producer patches are not active Jobs adoption.

Evidence: `target/qualification/shared-0114-host-0177/`, including snapshot review,
registry/cache preparation, common setup/admission and actual disposable Jobs
release/publication, formatting/evidence and graph-input refusal fixtures.
Release/publication fixtures pass on Bash 5 and genuine Bash 3.2 including child
shells. ShellCheck and final snapshot/pin/link checks pass. Canonical Testkit 0.37
CLI refresh/admission passes on the selected Host graph, without PocketIC setup.
Rust 1.88 core/timers tests pass separately, with native/Wasm minimum checks,
all 16 stable-consumer tests, release Wasm, harness compilation, strict native/Wasm
Clippy, docs/doctests and formatting. These are local Linux results; no complete
local gate or live PocketIC qualification was selected.

## Native acceptance blocker

Delivered 0.9.2 [CI 38439119750](https://github.com/dragginzgame/ic-jobs/actions/runs/38439119750)
passes Linux but both macOS architectures fail before harness compilation in
Host 0.17.6 retained staging: `Mode::from_bits_truncate` receives u32, while
Darwin expects u16. Actual logs were inspected and retained. Published Host
0.17.7 still contains that call; this is source inspection, not new macOS execution.
[Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84) owns the repair
and published downstream follow-up. Do not add a Jobs patch/shim or treat the
version update as macOS acceptance. [#41](https://github.com/dragginzgame/ic-jobs/issues/41)
and [#43](https://github.com/dragginzgame/ic-jobs/issues/43) remain open for complete
final-source native acceptance; cancelled earlier runs do not satisfy it.

Last inspected live recovery remains Jobs 0.8.1
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.
That source-bound evidence does not qualify this graph.

See [history through 0.9.1](history-through-0.9.1.md) for frozen prior preparation,
[architecture](../../README.md), [safety](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md), and
[delivery procedure](../releasing.md).
