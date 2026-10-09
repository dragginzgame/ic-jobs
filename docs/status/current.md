# Current handoff

IC Jobs package version 0.2.3, released at
`e32d44bccddcbba248aa9461af5b7f90cd54e553` and pushed by the maintainer.
The exact local annotated `v0.2.3` object
`9957db640097b0dadf0c8d3f1432d4e01129c458` matches GitHub's tag reference;
the retained local release plan is complete. The maintainer reports it live,
and an exact crates.io observation confirms 0.2.3 present without binding
registry bytes to a local artifact.
Release-commit [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37909918403)
has passed Linux; both native macOS jobs were queued at the latest observation.
The preceding 0.2.2 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37903041952)
has passed Linux, macOS Apple Silicon and macOS Intel at its exact release SHA.
Those earlier native jobs retain their original identities after the 0.2.3 push:
queued retention is observed, and both retained native jobs have completed successfully.
This does not establish retention of a job already running at push time.
Release 0.2.1 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37899775906)
passed all three hosts. Its earlier native results do not qualify later source
or live IC recovery. Owning issues retain the outstanding native/hosted acceptance.
The library owns job policy and checked metadata transitions. Applications own
persistence and effects; the optional adapter uses the consumer-selected IC Timers
dependency graph.

The maintainer requires Rust 1.88 for the complete package. Manifest metadata,
native/Wasm minimum checks, CI setup and current support documentation now use
that floor for both core and timers. Release 0.2.0 drops the previously advertised
1.85 floor and changes the consumer toolchain contract; consumers must upgrade
to 1.88. The compatible publication-path and release-cache fixes are delivered
in 0.2.3. The next selected pending release is 0.3.0 for the breaking developer
IC bundle hard cut, carrying the earlier compatible shared-tooling fixes and
the maintainer's incoming locked dependency refresh. Package metadata remains
0.2.3. Job APIs, stored fields, scheduling policy and the Rust floor are unchanged.
Earlier 1.85 qualification below records historical evidence only.
The released 0.2.3 lockfile selects IC Timers 0.14.23 and IC Metrics 0.2.18 within the
existing compatible requirements. Explicit `cargo fetch --locked` downloaded the
missing Timers input before delivery; that pre-release lockfile's SHA-256 remained
`3c423922eda86146028b688b0ac37a83ea4cd01490b5149aebc8107d36cfc980`.
Focused Linux checks passed on that graph: Rust 1.88 native and Wasm library
checks for core/all features, all 24 job tests with timers, and warnings-denied
development Clippy. This replaces the earlier missing-cache check limitation;
it does not qualify native macOS or live IC recovery.
The incoming working lockfile selects IC Metrics 0.2.20 with IC Timers 0.14.23.
That maintainer selection is preserved byte-for-byte at SHA-256
`be8149670998dccb7516e292f831b43e3c28131ac248917402a99f79bdc8d23e`.
Its cached graph now passes focused Linux Rust 1.88 core/all-feature native and
Wasm checks, all 24 job tests with timers, and warnings-denied development Clippy.
The tagged Metrics 0.2.18-to-0.2.20 arithmetic source and package requirements are
unchanged; the upstream fixes concern host tooling. Native macOS and live IC
recovery remain separate qualification requirements for this working tree.

The initial compatible pending batch adopted committed Shared Tooling revision
`926a20606591214ab29faa236b0b584e4857439e` through the canonical exporter from
`/tmp/ic-jobs-shared-038.Nj6oHA/source`, preserving the 72-file selection.
The sibling's dirty VERSION bump is excluded. This commit contains the 0.1.38
pin-exception fix but records VERSION 0.1.37; no published 0.1.38 tag was observed.
The hook keeps original-checkout prepared executable lookup while formatting
isolated index inputs. Pin exceptions must be one validated JSON array, so later
documents cannot mask malformed exceptions. The selected Cargo installer also
adopts single-document receipt admission, original failure status and ancestor
rechecks; its consumer-selected mode remains inactive in Jobs.
See [#6](https://github.com/dragginzgame/ic-jobs/issues/6) for reproduction,
adoption and qualification evidence. This clone already has
`core.hooksPath=.githooks`; source adoption does not activate other clones.
Focused qualification passes on Linux Bash 5 and genuine Bash 3.2.57: canonical
dependency, hook and Rust installer fixtures, including multi-document refusal,
local-only executable lookup, wrong/missing tools and payload preservation.
The installer uses substitute Cargo; the hook fixtures combine executable
substitutes with real formatting cases. Jobs' actual Make fmt/fmt-check adapter
also passes disposable-checkout adoption with prepared formatters, including
manifest sorting, partial-stage refusal, failure isolation and unrelated edits.
Snapshot integrity, actual declarations, documentation links, formatting,
release Make adapters, ShellCheck and whitespace checks pass. The full local
gate, native macOS and live IC qualification were not run for this pending batch;
no commit, release, push or upload was made.

The requested hard cut now adopts the latest committed Shared Tooling revision
`8140e3dd1b44409d682c721889ab702f438c6a17` from the clean isolated checkout
`/tmp/ic-jobs-shared-hard-cut.IcFDos/source`. That export retained all 72 files;
Jobs never selected the retired PocketIC checkers/fixture. This source contains
Shared's 0.2.0 hard cut but records VERSION 0.1.38; no published 0.2.0 tag was
observed. Upstream native acceptance remains separate from this local adoption.
The IC matrix/validator/installer now select exactly five tools. Jobs has no
PocketIC/Testkit dependency or server-path/equality caller, so no replacement
server installer or product dependency is added. The existing CI matrix now
explicitly prepares the reviewed bundle and checks it offline on all three hosts.
See [#7](https://github.com/dragginzgame/ic-jobs/issues/7) for adoption acceptance.
An existing six-tool bundle must be replaced by explicit `make install-ic-tools`
before `make ic-tools-check`; previous bundles, pins, receipts and evidence stay
retained. This clone had no installed IC bundle before adoption. No job storage
reset or consumer data conversion is needed. Future application-owned IC recovery
qualification uses Testkit's selected setup/check contract.
Focused Linux qualification passes: the canonical IC installer fixture on Bash 5
and genuine Bash 3.2.57 covers synthetic selections for all three hosts, rejects
six-tool matrices before effects and preserves old bundle bytes/receipts through
failed and successful replacement. Actual `make install-ic-tools` verified the
official five-tool Linux assets; `make ic-tools-check` passed offline. Setup reuse
with a failing curl substitute kept the same active selection, pins and receipt;
the prior six-tool matrix is refused against the actual installer. Installed
artifacts remain under `.tools/ic-set.*`; logs and before-adoption inputs are
retained under `/tmp/ic-jobs-shared-hard-cut.IcFDos/`.
Snapshot, actual declarations, documentation links, formatting, Make adapters,
ShellCheck, actionlint and whitespace checks pass. The incoming Cargo.lock digest
is unchanged. No job code or public symbol was removed; the installer drops only
PocketIC version/download/archive branches and the catalog drops its three rows.
No full local CI, native macOS or live IC run was performed for this hard cut;
delivery and exact-source native acceptance remain with the owning issue. No
commit, version bump, release, push or upload was made.

The subsequent authorized removal omits `scripts/ci/release-pr.sh` and its
snapshot record; the current selection has 71 files at the same shared revision.
Jobs supports direct release delivery only, enforced by both Make and its
metadata adapter. The shared runner loads the removed transport only for PR
delivery; no retained local PR plan exists. Canonical PR-adoption guidance stays
unchanged at its owner. The existing real-Git fixture now checks PR refusal
before fetch, gate or source mutation. Job/scheduler surfaces and restore/recovery
checks remain live and retained. See
[#8](https://github.com/dragginzgame/ic-jobs/issues/8) for the exact removed
function inventory and qualification.
Canonical refresh and the 71-file verifier pass after selection pruning.
`LC_ALL=en_US.UTF-8 CARGO_NET_OFFLINE=true make test-release-tooling` passes on
Linux Bash 5 and genuine Bash 3.2.57, exercising the supported direct
patch/minor/major/recovery and publication flows plus PR-mode refusal before
fetch/gate/Git or metadata mutation. Gate/registry/upload effects are substituted;
no live release or publication was performed. Documentation links, formatting,
release Make adapters, ShellCheck, runner syntax and whitespace checks pass.
The incoming Cargo.lock remains byte-identical. No full CI, native macOS or live
IC qualification was run for this cleanup; it remains local pending 0.3.0 work.

Release 0.2.3 adopts reviewed committed Shared Tooling 0.1.35 at
`be550afa57fe9e16872e5110b5cd69c24b4fa9e8` through the canonical exporter from a
clean temporary checkout. The 72-file selection still omits fleet reporting.
Its new selected Cargo-tool mode is not activated in Jobs; the fixed Rust-tool
bundle remains the setup contract. The owner's fixed/selected installer fixture
passes with substitute Cargo, and snapshot/pin/docs/format/Make checks pass.
The public GitHub description matches the maintained library scope; sibling
repositories remain read-only.

Release preflight now uses `cargo fetch --locked` after source/candidate admission
and saved-intent selection, before the offline gate or metadata preparation.
Explicit Cargo offline settings remain authoritative. Fetch failure retains
Cargo's diagnostics/status and leaves metadata unchanged; ordinary version and
metadata checks remain offline. The new fixture reproduced the old forced-offline
dispatch at `/tmp/jobs-release-tooling.esB0Uj/cold-offline.log`. Its corrected
cases cover cold preparation, explicit offline refusal, failed fetch,
prepared-cache reuse and dirty-source refusal before any fetch, with original
failure status and no gate/version effects. See
[#5](https://github.com/dragginzgame/ic-jobs/issues/5).

The delivered publication fix preserves trailing newlines in the checkout
path, using the metadata adapter's existing path-sentinel pattern in the
publisher and release fixture. The extended fixture reproduced the released
publisher's incorrect `cd` at
`/tmp/jobs-release-tooling.2oZeDV/publication.log`; failure occurred before any
registry observation or upload. The initial fixture-edit failure is retained
at `/tmp/jobs-release-tooling.EZZHDM`; its remote mapping was corrected before
the publisher reproduction. See
[#4](https://github.com/dragginzgame/ic-jobs/issues/4) for the defect and delivery
qualification. The corrected focused release/publication fixture passes with
real local Git and substituted gate/registry/upload effects under
`LC_ALL=en_US.UTF-8`, also using Bash 3.2.57 on Linux from a source checkout
ending in two newlines with `CDPATH=/tmp`. The source copy is retained at
`/tmp/jobs-release-checkout.iInCSL`. ShellCheck, documentation links, snapshot
integrity, release Make adapters and whitespace checks pass. The agent did not
run the complete gate or deliver this patch; the maintainer subsequently
released/published 0.2.3. Its Linux CI passes, while native macOS qualification
remains queued as recorded above. The combined cache/path release fixture
also passes under the offline gate environment and Bash 3.2.57 on Linux from
`/tmp/jobs-release-cache-checkout.xAx7mC/source checkout` followed by two newlines,
with `CDPATH=/tmp`. Gate/registry/upload and cache outcomes in that fixture are
substituted; the actual locked registry fetch above is separate evidence.
ShellCheck also passes for the adopted Rust installer and consumer adapters.

## Delivered 0.2.2 tooling evidence

Release 0.2.2 adopts Shared Tooling 0.1.34 at reviewed committed
revision `3d33cd250fcae7dbe5cabe44b2abd6b2c91a1822` through the canonical exporter
from a clean temporary checkout. The 72-file snapshot omits the unused fleet
reporter and its manifest record; local LOC reporting, pinned cloc and required
verification companions remain selected. The canonical Make include now explains
the optional fleet selection instead of calling a missing file. See
[#1](https://github.com/dragginzgame/ic-jobs/issues/1) for adoption evidence.
No fleet reporter regression is selected by Jobs CI, and sibling repositories
remain read-only. The public GitHub description matches the maintained scope.

The same snapshot adopts the independent IC installer reuse correction from
[shared #79](https://github.com/dragginzgame/shared-tooling/issues/79): compare
validated pin records rather than file presentation. Comment or row-order edits
reuse a verified bundle without rewriting receipts; changed records, checksums
and versions retain their admission checks. Actual tool pins are unchanged.

The CI workflow now includes the pushed commit's SHA in its concurrency group;
only superseded revisions of a pull request share a cancellable group. This
preserves queued and running push qualification without changing the three-host
matrix, permissions, gates or timeout. Delivery is complete; hosted
consecutive-push observations remain to qualify the policy, tracked in
[#3](https://github.com/dragginzgame/ic-jobs/issues/3).

Focused Linux tooling checks passed: the committed owner's command fixture with
substitute installers/reporters and IC installation/retention fixture with
synthetic assets for all three host selections; actual offline host-tool
verification and local
`make cloc`; optional fleet-report refusal; snapshot integrity, documentation
links, dependency declarations, release Make adapters, formatting, ShellCheck
and actionlint. The first local LOC attempt lacked cloc; explicit pinned
`make install-host-tools` prepared the checkout-local bundle, with pins unchanged.
Install/check aggregate wiring was inspected with Make dry runs; the complete
Rust/IC bundles were not installed or checked here. The full CI/release gate and
native macOS qualification were not run by the agent for that working-tree
batch. The maintainer subsequently delivered it as 0.2.2; its native macOS CI
qualification passes on all three supported hosts as recorded above.

## Delivered restore and earlier tooling evidence

Continuation source review found that `Job::restore` admitted contradictory
pending records, tracked in [#2](https://github.com/dragginzgame/ic-jobs/issues/2).
Three new regression tests reproduced the incorrect admission against released
0.2.0 before the fix. Restoration now rejects zero-attempt success with no
completed recurring occurrence, retry completion before its scheduled time,
and successful recurring successors inconsistent with completion. The existing
Schedule successor calculation owns those deadlines; CatchUp completion is
checked against its previous scheduled occurrence. Running/Uncertain remain
blocked and public transition error behavior is unchanged. There are no public
API, stored-field, dependency or compiler-floor changes. Records produced by
valid public transitions remain admitted; this is the compatible 0.2.1 patch.
The recurring transition/restore series now covers AfterCompletion as well as
both fixed-rate policies, including retries, uncertainty and cancellation.
Two additional regression cases reproduced shifted first-occurrence deadlines
and cancellation timestamps preceding a retained completion in the earlier
working patch. Restoration now requires the first occurrence's original deadline
in every state and derives a shared completion-time lower bound for untouched
recurrences, including cancelled ones. Pending successors still reuse the
canonical scheduling calculation. Valid cancellation at completion or after
the next deadline is retained; recovery cases also cover zero-time completion,
maximum representable intervals/deadlines and unchanged scheduler selection.
Focused Linux qualification passed: 24 all-feature and 23 core job tests on
Rust 1.88.0, core/all-feature native and Wasm minimum checks, warnings-denied
Clippy, rustdoc and both doctests, and verified all-feature packaging.
Formatting, snapshot integrity, dependency declarations and documentation links
pass. The agent did not run a full gate or deliver this repair; the maintainer
subsequently released and pushed it as 0.2.1, whose three-host native CI now
passes as recorded above. Live IC recovery remains unqualified.

The initial fleet cleanup review found no intentional local caller but awaited
a committed upstream optional-selection correction. That blocker was resolved
by Shared Tooling 0.1.33 and the current 0.1.34 adoption described above.
Focused Linux qualification passed with Rust 1.88.0: native and Wasm library
checks both without timers and with all features, plus all 18 job tests with
all features against that lockfile. Formatting, documentation links, dependency
declarations and the unchanged shared snapshot also verify. Full CI, release
gates, native macOS and live IC qualification were not run for this batch.

The maintainer's subsequent `make release-minor` failed in the CI gate at the
development Clippy `manual_is_multiple_of` lint, newly applicable after raising
the advertised floor. The failure is retained at
`.git/release-state/validation-failures/20261009T061629Z-1284332-0-ci.log`.
Fixed-rate restoration now uses `u64::is_multiple_of`, supported by the required
Rust 1.88 compiler; schedule validation still rejects zero intervals before
this check. Accepted records and error transitions are unchanged. Focused repair
checks passed: warnings-denied Clippy, all 18 job tests on Rust 1.88.0, core and
all-feature native/Wasm minimum checks, and formatting. No release was retried
or metadata bumped by the repair.

The earlier Shared Tooling 0.1.30 adoption used committed revision
`4e274a2219c0b0cc3af68ec65658b373253518fb` through the canonical exporter,
using a clean temporary checkout and excluding dirty sibling work. The snapshot
contains 73 files, including the explicitly selected shared release-source
checker. Both metadata and publication adapters delegate source observations;
only the metadata adapter permits the three local release files. Initial
preflight refusals identify that validation/preparation have not started, and
Git observation failures remain distinct from dirty source.
The shared PocketIC setup pin advances to 16.1.0; no executable installation or
live protocol qualification was performed. Optional npm checks and Cargo-install
qualification are not activated. Focused Linux checks passed: locale-sensitive
real-Git release/publication fixtures with substitute gate/registry/upload
effects, all-path diagnostics, literal allowances, rename-source refusal,
Git-observation failure and source/index preservation. ShellCheck, snapshot
integrity, documentation links, dependency declarations, formatting and standard
Make release adapters passed. The public GitHub description matches current
purpose. No full CI, commit, release, push or upload was performed.

Upstream inspection on 2026-10-09 confirmed IC Host Tooling main at 0.8.4
(`97187b2a46d6f8a6964224a36a133d858ef0d223`); the earlier IC Host Tools repository
remains at 0.2.0. No host dependency is needed by the scheduler library now.
Bounded process capture and child cleanup could serve a future application-owned
PocketIC test harness outside the canister package. No host-crate adoption was
performed.

The maintainer subsequently requested removal of the older GitHub repository
`dragginzgame/ic-host-tools` (0.2.0). Its local checkout was already absent.
A fresh Git mirror at `be7d73908225e73ab5babf8fbb9fe959f40d439b`, repository
metadata, all 10 issues and 15 comments, and empty release metadata were archived
and verified before attempting removal. The retained backup is
`.git/retired-repositories/ic-host-tools-20261009/backup.tar`, with a checked
SHA-256 beside it. GitHub rejected DELETE with HTTP 403: the authenticated CLI
token lacks `delete_repo` scope. No enabled browser session or repository-deletion
connector was available. On the subsequent 0.2.0 continuation, authenticated
GitHub inspection returned 404 for the old repository while the token retained
`repo` access, consistent with its removal outside this agent's attempted DELETE.
The backup remains retained. This retirement did not remove crates.io packages
or the successor `dragginzgame/ic-host-tooling`.

## Earlier implementation and qualification evidence

Core, persistence example, focused state/recovery tests and repository tooling
are implemented. Local Linux validation passed: 14 focused tests with the timer
adapter, Clippy with warnings denied, rustdoc and its example, the runnable
persistence example, and native/Wasm checks at Rust 1.85 core / Rust 1.88 timers.
The adopted snapshot and documentation links verify; release entrypoints passed
their substitute-runner check. Dependency declarations now pass against the
actual tracked lockfile. The complete validation-only release gate passed on
Linux with the locale repair before the 0.1.2 scheduler work. Live IC and native
macOS qualification remain pending.
The public GitHub repository `dragginzgame/ic-jobs` is configured as `origin`,
with authenticated HTTPS push credentials. The initial source is committed at
`da4a555`; the tooling repair is committed at `de61cd3`. The first release,
0.1.1, is tagged at `d5a2603` and the exact annotated tag is present on `origin`.
The retained release plan is complete. A current crates.io observation reports
`ic-jobs` 0.1.1 present; that observation alone does not bind registry bytes to
the local artifact.

Release and publication setup is committed. The Shared Tooling runner remains
at reviewed revision
`1872ed2c20f6c70689bb2249050b1d673c60bfa0`, with its annotated-tag and exact
crates.io observation helpers added to the snapshot. `make publish` separately
requires clean source, the matching pushed annotated tag and a confirmed absent
registry version. Pinned cargo-edit setup, package verification, publication dry
runs and configured CI coverage are available. Source changes must be committed
and Cargo credentials prepared before actual release/publication.

Focused Linux checks passed: real local Git patch/minor/major release fixtures,
failure before mutation, completed-release resume and publication refusals with
substitute gate/registry/upload effects, also under Bash 3.2. Cargo's actual
publication dry run verified the all-feature tarball without upload; rustdoc and
its example, native/Wasm Rust 1.85 core / Rust 1.88 timers, formatting, dependency
declarations, snapshot integrity and documentation links passed. The initial
tarball failure exposed a workspace-relative README include, now fixed with
crate-local README/license files. The initial symlink implementation was rejected
by pre-commit snapshot admission and replaced with a regular package guide and
license file. An offline publication dry run was refused by
Cargo's required HTTP observation; `publish-check` now explicitly uses registry
access. These passes do not establish native macOS or live publication evidence.

A maintainer invocation of `make release-patch` failed in the validation gate's
release fixture at its changed-path assertion; logs and the fixture remain at
`.git/release-state/validation-failures/20261008T172821Z-3674222-0-ci.log` and
`/tmp/jobs-release-tooling.KU6QTX`. The failure was reproduced with
`LC_ALL=en_US.UTF-8`: locale-sensitive sorting disagreed with the expected byte
order after successful delivery to the fixture's local bare remote. The comparison
now explicitly uses the C locale, and failed fixture assertions name their source
line. The actual repository's version and tags were not changed by that failed
gate or its repair.

With the repair applied, `LC_ALL=en_US.UTF-8 make test-release-tooling` and
`LC_ALL=en_US.UTF-8 make release-verify` passed. The latter ran the full `ci`
validation target through the same nested logger path as the failed attempt;
it did not prepare metadata, commit, tag or push. The maintainer subsequently
committed the repair and completed the normal 0.1.1 release, with fresh validation
as required by the standard runner.

The corrected changelog separates the untagged initial 0.1.0 implementation from
the dated 0.1.1 delivery tooling and fixes. The 0.1.1 release date is preserved;
the documentation correction does not change package metadata, tags or the
public API/storage contract.

The selected next draft is 0.1.2 for an additive scheduler wrapper. Its release-tooling
prerequisite is implemented: fixtures read the actual workspace version for
increment, failure and publication checks instead of assuming the initial 0.1.0.
The hardcoded assumption was reproduced failing against the released 0.1.1 at
`/tmp/jobs-release-tooling.YMX527`; the revised fixtures pass with
`LC_ALL=en_US.UTF-8`. ShellCheck, snapshot integrity and documentation links pass.
Package metadata remains 0.1.1.

The accepted consumer requirement is a layer between application job intent and
actual due-work/wakeup selection. `Scheduler` now borrows a bounded batch of
application-owned, validated jobs. It derives the earliest pending deadline and
starts one earliest due attempt, returning its exact Running record for the
consumer to commit before executing the effect. Ties follow batch order; restored
Running/Uncertain and terminal jobs remain excluded. All transition checks stay
in `Job::start`, with errors leaving the batch unchanged and no fallthrough to
another job. Storage, payloads, handlers, global indexing and lifecycle remain
consumer-owned; the existing IC Timers adapter receives the derived deadline.
Only a batch covering the global earliest work may supply the global wakeup.

Focused Linux qualification for the working-tree wrapper passed: 18 job tests
with all features, Clippy with warnings denied, rustdoc and both examples,
native/Wasm compilation at Rust 1.85 core / Rust 1.88 timers, verified all-feature
packaging, and the runnable persistence/recovery example. Current-version release
fixtures passed under `LC_ALL=en_US.UTF-8`; declaration, snapshot, formatting and
documentation checks passed. The complete CI/release gate and live IC/native
macOS qualification were not run for this scheduler batch. There are no dependency,
MSRV, durable-record or existing public API changes; 0.1.2 remains the compatible
pending release. No commit, release, tag push or upload was made for this work.

See [the README](../../README.md), [safety contract](../../SAFETY.md) and
[delivery procedure](../releasing.md).
