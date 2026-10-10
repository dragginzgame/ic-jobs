# Current handoff

Pending compatible **0.5.4** now adopts committed Shared **0.3.7** at
`34e5ad7aac3599306c9572bb547f2239d09df1a3`, through its canonical exporter with
the existing 79-file selection. Shared rejects failed Cargo-tool activation
assertions explicitly on Bash 3.2. Jobs' three tooling fixtures, release metadata
adapter and live Wasm-file admission use explicit mandatory failures too.
Contradicting an observed status now stops the fixture and retains evidence;
release version mismatches stop before cache/setup or metadata mutation.
Existing setup, release and recovery command contracts remain compatible.

The incoming root selection is preserved byte-for-byte: Memory **0.35.3**, Timers
**0.17.4**, Metrics **0.5.4**, Testkit **0.33.0** and all four Host crates
**0.12.6**. Lock SHA-256 is
`3923304ae00bfe855ce268d25ec035b23783d66877b24c919a62c159c30055d9`.
Reviewed public Host release `5f356effea97fbc31dfcca5b9f1b2834f35325a9`;
the four published Rust source trees match 0.12.5 and retain Rust 1.88.0.
Host remains transitive through the host harness and absent from both normal
core/canister Wasm graphs. Job records, effect disposition and consumer storage
authority retain their established contracts.

Both validation workflows keep only the latest run per workflow/ref, preserving
all three native hosts. These incoming workflow changes satisfy Shared's new
policy; cancelled hosts do not qualify their source. Delivered Jobs **0.5.3**
at `d4468fb423c97550c74a2e87a02f8b3e70361ef5` now has passing
[CI on all three hosts](https://github.com/dragginzgame/ic-jobs/actions/runs/38059981517).
That evidence belongs to the delivered graph, not this local repair.

Actual Jobs fixtures pass on current Bash/Make and genuine Linux Bash 3.2.57/
Make 3.81. They cover failed status assertions with retained evidence, original
nonzero statuses, version refusal before effects and invalid Wasm admission
before attempt/server dispatch. Canonical exact-source installer fixtures pass
on both profiles with substitute Cargo. Real selected Testkit CLI/server setup
and admission reuse the prepared 0.33.0/16.1.0 installation. Full local `make ci`
passes, including separate Rust 1.88 core/timers native/Wasm checks and packaging.
All five live scenarios pass afresh on each original and Binaryen 133 `-O3`,
`-Os`, `-Oz` output under parallel Bash 3.2/Make 3.81 without jobserver warnings.

Evidence: `/tmp/ic-jobs-shared-037.FdcYDN/`, including the incoming graph/patch,
snapshot refresh, before reproductions, Jobs/producer fixture logs, package
source review, CLI admission, `ci.log`, `live-bash32.log` and source hashes.
Live attempt `jobs-recovery.YPdQlo/` retains exact variant/server/tool artifacts;
original Wasm SHA-256 remains
`5101fe9b88e51d030ec52a3b3fa818625598a8257c6d8c6389aaf45d5e38707d`.
Source hashes match after CI/live. Final handoff checks/hashes follow separately.
Earlier evidence below remains bound to its original inputs. Local adoption and
assertion repair are recorded in [#17](https://github.com/dragginzgame/ic-jobs/issues/17)
and [#16](https://github.com/dragginzgame/ic-jobs/issues/16); native macOS acceptance
of the new source and hosted optimized recovery acceptance remain pending.
Package metadata remains 0.5.3; no commit, push, release, publication, sibling edit
or hosted dispatch occurred.

## Earlier pending 0.5.4 qualification at Shared 0.3.6

Public Jobs **0.5.3** is at `d4468fb423c97550c74a2e87a02f8b3e70361ef5`.
Pending compatible **0.5.4** adopts Shared **0.3.6** at
`0604bfd730ec7ec288cd2cfdad217a0d42bf256b` through the canonical exporter,
retaining the same 79-file selection. Testkit setup, offline admission and the
live/manual recovery callers now select the CLI directly from `Cargo.lock`;
the duplicate environment-file pin is removed. Existing Make targets retain
their contracts. Rerun `make install-testkit-tools` after a locked Testkit change.
Shared owns parsing, strict admission, installation and selection receipts;
Testkit still owns PocketIC provisioning and lifecycle.

The incoming manifest/lock is preserved: Memory **0.35.3**, Timers **0.17.4**,
Metrics **0.5.4**, Testkit **0.33.0** and four Host crates **0.12.5**. Lock SHA-256
is `2eb1cc33741697393e9b49eb6fe9c93aafc477e8901a089db385a93c2e0cbe8b`.
Host remains transitive in the host harness, outside the Jobs/canister graph.
Testkit's scoped transport-reset classification does not change the direct
PocketIC harness or its typed trap assertions. Jobs APIs, stored records,
consumer authority, uncertain-effect blocking and Rust 1.88 remain unchanged.

Real cold CLI setup and CLI/server admission pass; repeated CLI setup/admission
reuse the installation with Cargo installation and downloads disabled. The old
0.32.2 slot and receipt still verify. Actual Jobs caller fixtures reject changed,
unprepared lock selections before Cargo/PocketIC dispatch on both shell profiles.
Canonical hook/installer fixtures and Jobs' actual hook/formatting admission pass
on current Bash/Make and genuine Linux Bash 3.2.57/Make 3.81. These producer
fixtures use substitute build tools; actual installation and live results are
recorded separately.

Full local `make ci` passes, including separate Rust 1.88 core/timers native/Wasm
checks and packaging. All five live recovery scenarios pass on each original and
Binaryen 133 `-O3`, `-Os`, `-Oz` output under parallel Bash 3.2/Make 3.81, without
jobserver warnings. Source hashes match after CI and live execution. Evidence is
retained under `/tmp/ic-jobs-shared-036.bgUa2J/`: `ci.log`, `live-bash32.log`,
`qualified-source.sha256`, CLI selection, incoming graph and focused fixture logs.
Attempt `jobs-recovery.AXXHen/` retains actual Wasm/hashes and per-variant logs;
original Wasm SHA-256 is
`5101fe9b88e51d030ec52a3b3fa818625598a8257c6d8c6389aaf45d5e38707d`.
Final documentation changes are checked and hashed separately after qualification.

[#17](https://github.com/dragginzgame/ic-jobs/issues/17) records this local repair;
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) retains hosted recovery
and native macOS acceptance. Earlier evidence below is not relabelled. Historical
complete-toolset acceptance in [#15](https://github.com/dragginzgame/ic-jobs/issues/15)
is closed against the three passing native hosts at Jobs 0.5.0. The newer
[0.5.3 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38059981517)
passes Linux; both macOS jobs are running. The 0.5.2 macOS jobs were cancelled,
so [#16](https://github.com/dragginzgame/ic-jobs/issues/16) retains native-host
acceptance. Package metadata remains 0.5.3; this batch is local and uncommitted.

## Earlier 0.5.3 qualification before delivery

Pending compatible **0.5.3** aligns the selected Testkit CLI to the incoming
**0.32.2** harness library; rerun `make install-testkit-tools`. The existing
Shared 0.3.5 snapshot refresh remains local, with all 79 selected file hashes
identical to delivered Shared 0.3.4. Public Jobs main remains released 0.5.2 at
`6516eba4f54f69c50a1c1bcab38fa65354ba8b46`; its exact-source CI now passes
Linux, while both macOS jobs remain queued. The Shared 0.3.5 snapshot refresh
and CLI changes remain uncommitted here.

The incoming lock is preserved: Memory **0.35.3**, Timers **0.17.3**, Metrics
**0.5.3**, Testkit **0.32.2** and four Host crates **0.12.4**, SHA-256
`f0929622a8977029d94d31aed7ffc48d44e7781b57ee9a7115c768a1b9850fac`.
Reviewed published producer commits: Testkit
`9e697b6363441e3fa96477adae5b07820461c3c5`, Memory
`b1fd17cf905fee32675f165f774da2def55b2059`, Timers
`4cc3c64e6b773d73edb66f6bdf2d91cce5e68790`, Host
`5400f159474cebac1ec7ae7c8763abfd258bde03`. Their packaged Rust source trees
match the prior patch versions byte-for-byte; tooling/qualification changed.
Jobs acquires no direct Host dependency or new lifecycle/storage authority.
Job APIs, stored records, scheduling policy and Rust 1.88 remain unchanged.

Locked cache preparation and selected CLI/server admission pass. Full local
`make ci` passes, including separate Rust 1.88 core/timers native/Wasm checks,
native consumers, linked Wasm/harness builds, strict Clippy, docs and packaging.
All five live recovery scenarios pass for the original and Binaryen 133 `-O3`,
`-Os`, `-Oz` Wasm outputs, using genuine Linux Bash 3.2.57/Make 3.81 with parallel
Make and no jobserver warnings. Source hashes match after CI and live execution.

Evidence is retained under `/tmp/ic-jobs-testkit-0322.qotFQz/`: `ci.log`,
`live-bash32.log`, `metadata.json`, `source-review.txt`, selected CLI receipt,
incoming lock and source hashes. Attempt `jobs-recovery.gYgMoY/` retains actual
Wasm/hashes and per-variant optimizer/test/server logs; original Wasm SHA-256 is
`19f44d2f8c83427dff68b3aee7a859c18e701067c5e27df0742cd3d56d00a057`.
Final handoff checks/hash binding follow qualification. Earlier graph evidence
below remains separately bound. Native macOS and hosted recovery/artifact
acceptance remain pending in #11, #15 and #16. No commit, push, release,
publication, sibling edit or hosted dispatch occurred; package metadata remains
0.5.2.

## Earlier Shared 0.3.5 snapshot-only qualification

IC Jobs **0.5.2** is pushed at
`6516eba4f54f69c50a1c1bcab38fa65354ba8b46`, matching public main. Its
[exact-source CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38056524883)
is running on Linux; both macOS jobs are queued. The updated optimized recovery
workflow is delivered, but hosted execution/upload remain unqualified.

The maintainer-requested Shared Tooling **0.3.5** adoption selects committed
`a744d7f1990b9e1451ef45cd6d495de00a141cd3` through the canonical exporter from
clean detached `/tmp/ic-jobs-shared-035.bAP8SC/shared`. All 79 selected file
hashes/modes match the prior 0.3.4 snapshot; only its revision/version and the
local AGENTS reference change. Newer dirty upstream hook edits are excluded.
Binaryen 133 and every executable/dependency selection remain unchanged, so
this adoption needs no reinstall, product changes or governance-only changelog
entry. Package metadata remains 0.5.2.

Shared 0.3.5 isolates its own validation-runner fixture from inherited parent
logs, failure logs and GitHub summaries
([shared #105](https://github.com/dragginzgame/shared-tooling/issues/105)). Jobs
does not vendor that fixture or the producer portable suite. The exact committed
fixture passes from the detached checkout with all three parent destinations
selected, preserving sentinel contents and directory membership, on current
Bash/Make and genuine Linux Bash 3.2.57/Make 3.81. Jobs snapshot integrity,
complete offline tool admission, format-tool admission, pins, documentation links
and substitute release-adapter checks pass; Cargo.lock is unchanged.

Evidence is retained under `/tmp/ic-jobs-shared-035.bAP8SC/`: `refresh.log`,
`parent-current.log`, `parent-bash32.log`, retained parent directories/test logs
and `jobs-focused.log`. Shared 0.3.5
[exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38056455993)
passes lint/security; Linux regression is running and both macOS jobs are queued.
No new full Jobs CI or live tests were run for this metadata-only refresh;
the prior qualification below remains bound to its actual inputs. Native macOS
and hosted recovery acceptance remain pending. No commit, push, release,
publication, sibling edit or hosted dispatch occurred in this adoption.

## Earlier 0.5.2 implementation and Shared 0.3.4 qualification

Pending compatible **0.5.2** now adopts Shared Tooling **0.3.4** at
`169d77b8440568c5200eede971625126181f7bb2`. The canonical exporter refreshed
the same 79 selected files from a clean detached checkout under
`/tmp/ic-jobs-shared-034.Gx5D82/shared`. The common pin matrix selects Binaryen
133; explicit `make install-ic-tools` and offline admission pass. Repeated setup
with downloads disabled reuses the selected set. The previous `ic-set.a1yAq4`
Binaryen 132 bundle's original pins/receipt remain identical and its bytes verify.

`make test-canister-optimized` qualifies the original consumer Wasm and independent
`-O3`, `-Os`, `-Oz` outputs through the existing five live recovery tests. The
manual recovery workflow selects this target on all three native hosts. Inputs,
outputs, hashes, optimizer identity/logs and each variant's test/server logs are
retained, with failures stopping before later variants. Normal `build-consumer`
and the original-only `test-canister` remain available. No production optimizer
policy, Job API, storage authority or Rust 1.88 floor changes are introduced.

The incoming graph is preserved, including its newer Metrics **0.5.3** selection:
Memory **0.35.2**, Timers **0.17.2**, Testkit **0.32.1** and four Host crates
**0.12.3**. The incoming lock SHA-256 remains
`541050654749edfbf664ced622d6f386769c0342fe39eeb2d7a264c4d12c3253`.
Full local `make ci` passes. Final affected shell/workflow/docs checks pass.
All five live scenarios pass for all four variants on Linux under current
Bash/Make and genuine Bash 3.2.57/Make 3.81, with parallel Make and no jobserver
warnings. An incomplete wrapper returns failure on both shells.

Evidence lives under `/tmp/ic-jobs-shared-034.Gx5D82/`: `ci.log`, `metadata.json`,
`live-current.log`, `live-bash32.log`, source hashes and tool receipts. Each live
attempt retains its actual Wasm and per-variant results. The Bash 3.2 attempt is
`jobs-recovery.AwYGvw/`; original Wasm SHA-256 is
`58c59e5d46c15eed68e92a4199d88a9dfe1eb39b299142c9150a3e805ca8127f`.
Source hashes match after CI and live runs. The preliminary feature probe
succeeded without extra feature flags; its initially asserted refusal was
incorrect and is retained with an explanatory summary. Final optimization uses
the input's declared features. Previous graph evidence below is not relabelled.

Shared 0.3.4 [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38054347275)
passes Linux and lint/security; both macOS jobs are queued. Delivered Jobs 0.5.1
CI also passes Linux with both macOS jobs queued. Native macOS optimized execution
and hosted recovery upload remain pending. No commit, push, release, publication
or hosted dispatch occurred; package metadata remains 0.5.1.

## Earlier pending 0.5.2 qualification at Shared 0.3.3

IC Jobs **0.5.1** is pushed at
`cfbe3ed6eccf6370f0d1b92abd34d928f6febd26`, matching public main. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38051154299) passes Linux;
both macOS jobs remain queued at the latest observation. The manual recovery
workflow is delivered. Registry publication was not independently checked.

Pending compatible **0.5.2** aligns the selected Testkit CLI with the maintainer's
updated **0.32.1** harness library. Rerun `make install-testkit-tools` before live
qualification. Reviewed Testkit commit `ff5da12c27bf0835586892ada8e443117229290b`
and published registry sources. Rust source is unchanged from 0.32.0; the Host 0.12 and
PocketIC 16.1.0 contracts remain selected. Jobs has no direct Host dependency.
Its public API, JobRecord, consumer storage, scheduling policy and Rust 1.88 floor
remain unchanged; no consumer API rewrite or stored-data reset is required.

The same patch adopts committed Shared Tooling **0.3.3** at
`d63f0cfaba8ab2961d6012064adbf051c1898bc1`, exporting 79 selected files from clean
detached `/tmp/ic-jobs-shared-033.hHIiH9/shared`. The added README-freshness task is
advisory; no schedule or new CI gate is activated. Newer dirty Binaryen changes
are excluded and executable pins stay unchanged. The exporter initially refused
a mismatched source-remote suffix before mutation; that refusal and the corrected
export are retained in `refresh.log` and `refresh-final.log` under that directory.

Shared's setup preflight now admits the complete platform/pins and available Rust
toolchain before downloads. Its Make includes and Jobs-owned Cargo, selected-tool
and metadata recipes preserve jobserver descriptors under the existing execution
guard. The production validation runner, both Jobs fixtures and both metadata
cleanup paths require explicit completion before successful exit/cleanup.
Actual released Jobs Bash 3.2 reproductions returned zero for incomplete work:
fixture nounset also deleted evidence, and absent RELEASE_VERSION falsely passed
prepare/selected-commit checks. Corrected checks return failure, preserve original
metadata and retain snapshots; original nonzero statuses remain unchanged.
[#16](https://github.com/dragginzgame/ic-jobs/issues/16) records these findings.

Actual Jobs release/publication substitute fixtures and parallel formatting
descriptor checks pass on current Bash/Make and genuine Linux Bash 3.2.57/Make
3.81. Canonical command, host-tool and validation-runner fixtures pass on both
profiles from that exact source. Real parallel Cargo/selected-tool probes no
longer emit closed-jobserver warnings. Logs and before/after exit probes are under
`/tmp/ic-jobs-shared-033.hHIiH9/`. Producer
[exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38052409053)
passes Linux and lint/security; both macOS jobs remain queued.

The incoming catalog/lock changes are preserved: Memory **0.35.2**, Timers
**0.17.2**, Metrics **0.5.2**, Testkit **0.32.1** and all four Host crates **0.12.3**.
Locked cache preparation and selected CLI/server admission pass. The lockfile
remains byte-for-byte equal to the incoming selection, with SHA-256
`833011b09c294fd687da7e4f36cbcb42b9a2ed13af0e783a5e9247603fc8224e`.

Full local `make ci` passes on this graph, including separate Rust 1.88 core and
timers native/Wasm checks, native consumers, linked consumer Wasm and host harness,
strict Clippy, docs and packaging. All five live Linux recovery scenarios pass
through the selected 0.32.1 CLI under Bash 3.2/Make 3.81 with parallel Make and no
jobserver warnings. Evidence is retained under
`/tmp/ic-jobs-shared-033.hHIiH9/graph-0321/`: `ci.log`, `live-bash32.log`,
`metadata.json`, the Testkit selection receipt and `qualified-source.sha256` bind
the execution inputs. Source hashes match after both CI and live execution.
The attempt `jobs-recovery.mGfY08/` retains the tested Wasm and server logs; its
Wasm SHA-256 is
`c3c4842cb4deb31c2af71fe723e3989385508c6a9436b8363541889a7ab31856`.
Final source hashes and the patch additionally bind this handoff update; the
selected new task is also retained separately as `readme-freshness.md`.

Earlier Testkit 0.32.0 qualification remains separately bound in
`/tmp/ic-jobs-testkit-032.8VWsoX/`, and the first Shared 0.3.3 gate/live runs are
retained at `/tmp/ic-jobs-shared-033.hHIiH9/{ci.log,live-bash32.log}`. The lockfile
advanced concurrently before final handoff, detected by the source binding check.
The new CLI pin, cache preparation, complete CI and live gate were then qualified
afresh under `graph-0321/`; earlier evidence is not relabelled as this graph.

[#11](https://github.com/dragginzgame/ic-jobs/issues/11) remains open for hosted
recovery execution/artifact acceptance and native host qualification;
[#15](https://github.com/dragginzgame/ic-jobs/issues/15) awaits delivered macOS CI.
The new [#16](https://github.com/dragginzgame/ic-jobs/issues/16) correction also
awaits delivery and native host qualification.
These Linux results do not establish either macOS host or hosted upload behavior.
No sibling edits, commit, release, push, publication or hosted dispatch occurred.
Package metadata remains 0.5.1.

## Earlier 0.5.1 workflow evidence before delivery

IC Jobs **0.5.0** is pushed at
`b4b7fb6f0e0b8221126019ff29f0eb332c91eb3e`, matching public main. The annotated
`v0.5.0` object `09a3eab5c3e4ce8ffc90e58835b1f4f934f01aa9` matches GitHub;
the retained release plan is complete. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38048491093) passes Linux;
both macOS jobs remain queued at the latest observation. Shared Tooling's
[producer CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38044218125)
now passes on all three hosts and lint/security. Registry publication was not
independently checked.

Pending compatible **0.5.1** adds the manually triggered
[Canister recovery workflow](../../.github/workflows/recovery.yml), running the
existing five PocketIC scenarios on Linux, Apple Silicon and Intel macOS. It
uses the reviewed common tooling, selected Testkit CLI and Testkit-owned server
setup/admission. A fresh job compiles the Wasm and host harness before starting
the 15-minute server lifetime. The test wrapper retains a Wasm copy/hash and
server stdout/stderr in a new attempt directory; tests read that retained copy.
Each workflow host uploads source/graph/tool selections, test outcome/output,
the tested Wasm and server logs for 14 days on success or failure. Failed setup
also uses the canonical failure collector. Push/PR CI remains compile-only for
the live harness. See the [consumer instructions](../../apps/job-consumer/README.md)
for manual execution after delivery to `main`.

The maintainer's delivered catalog selects Memory **0.35.0**, Timers **0.17.0**
and Testkit **0.31.0**. Those selections are preserved; no dependency update was
performed in this batch. The unchanged lock SHA-256 is
`5d1f6362eccaa041f8d61298b29c9b1b7461741dfae17646554bb19644fc1d33`.
Jobs runtime source, JobRecord and Rust 1.88 floor are unchanged. The workflow
adds no library lifecycle, storage or Host dependency ownership.

Full local `make ci` passes at this delivered graph, including separate Rust 1.88
core/timers native/Wasm checks, consumer native tests, linked Wasm/harness
compilation, strict Clippy, docs and packaging. All five live Linux tests pass
with the new log capture; after binding tests directly to the retained copy, the
final wrapper passes again on genuine Linux Bash 3.2.57/Make 3.81. Workflow lint,
ShellCheck, snapshot, pin, formatting and documentation checks pass. Logs are
retained in `/tmp/ic-jobs-recovery-workflow.k03mBT/` (`ci.log`, `live.log`,
`live-final-bash32.log`, `metadata.log`); `final-source.sha256` and
`tracked.patch` plus `recovery.yml` bind the final working tree. The final live
attempt is `jobs-recovery.036j71/` in that directory; its Wasm SHA-256 is
`810f093b5d097a04894fbae2b7b1a53c92fab4f3df25698c89bfdf7b91c56d00`.
The successful server emitted startup stdout and empty stderr, both retained.

Hosted recovery execution and artifact upload remain unqualified until the
workflow is delivered and manually dispatched; local passes do not establish
macOS live behavior or the hosted upload. [#11](https://github.com/dragginzgame/ic-jobs/issues/11)
owns recovery qualification, and [#15](https://github.com/dragginzgame/ic-jobs/issues/15)
still awaits delivered native macOS CI. No commit, push, release, publication
or hosted workflow dispatch occurred in this batch.

## Earlier 0.5.0 adoption evidence before delivery

IC Jobs **0.4.7** is pushed at
`b7833ab0f32008f787167e5ca0b4f84aeb2b4694`, matching public main. The annotated
`v0.4.7` object `73330d87bc4c8ba68807372f123213b57aa445eb` matches GitHub and
the retained release plan is complete. The maintainer reports it live; registry
publication was not independently checked. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38045779398) failed on
Linux because consumer Clippy uses Rust 1.99's Wasm target, which the workflow
prepared only for Rust 1.88. The local workflow now prepares both targets.
Both released macOS jobs remain queued at the latest observation.

Pending **0.5.0** adopts committed Shared Tooling **0.3.0** at
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` through its canonical exporter from
clean detached `/tmp/ic-jobs-qualification.l4gwGK/shared` (78 selected files).
Newer dirty sibling edits were excluded. Common setup/check now orders the full
host, five IC and three Rust toolsets before selected cargo-edit. CI and admitted
release preflight use the same complete path; source/candidate admission and
locked fetch still precede release setup. Previous installations and saved-release
reconciliation are retained. Ordinary checks install nothing; optional Testkit
CLI/server setup remains separate. This is a breaking tooling contract and
requires a minor release; package metadata remains 0.4.7.
[#15](https://github.com/dragginzgame/ic-jobs/issues/15) tracks delivery.

The Testkit CLI now matches the selected 0.31.0 library. Memory 0.34.1, Timers
0.16.7 and the incoming lock selection are preserved, with lock SHA-256
`8f3c8e0964cbad6428dfab246fbef259e67ed5367623dd6da4d370747c83ecc2`.
The public Jobs API, JobRecord format and Rust 1.88 floor are unchanged;
Host/Testkit dependencies remain outside the canister graph.

All five **live Linux PocketIC 16.1.0** scenarios now pass: authenticated task
management and real timer delivery across upgrades, synchronous storage/identity/
timer rollback, lost sender results with independent receipt disposition and no
duplicate delivery, invalid-record upgrade refusal with controller-snapshot
recovery, and overdue CatchUp/Skip reconstruction. Rejection assertions require
the actual canister-trap error. Snapshot recovery uses Testkit's bounded retry
for installation rate limits only; its job deadline exceeds the maximum cooldown.
Two earlier attempts hit the snapshot recovery installation limit and remain
retained separately. The passing log is
`/tmp/ic-jobs-qualification.l4gwGK/live-owner-retry.log`; failed logs are
`live.log` and `live-corrected.log` in the same directory. Exact live-source
hashes are in `live-source-final.sha256`. The linked Rust 1.88 Wasm SHA-256 is
`c6bfe0357a2ad1ec3f413f59a9dbdc9cc29d3e872ad5396f5761c0fe6da70ded`.
[#11's qualification comment](https://github.com/dragginzgame/ic-jobs/issues/11#issuecomment-6096833997)
records the evidence and remaining delivery requirement.

The full local `make ci` passes in `ci.log`, including separate Rust 1.88
core/timers native/Wasm checks, native consumer tests, linked Wasm and host harness
compilation, strict Clippy, docs and packaging. Subsequent fixture-only changes
were rechecked with Bash 5/Make 4.3 and genuine Linux Bash 3.2.57/Make 3.81 in
`release-fixtures-final.log` and `release-fixtures-bash32.log`. They exercise
ordered setup/check, reuse, failures, release reconciliation and refusal before
parallel builds for every missing common tool set. Canonical host/routing fixtures
pass on both shell profiles. ShellCheck, workflow lint, formatting, snapshot,
pin and documentation checks pass. Repeated real setup/check preserves executable
bytes/inodes and receipts (`reuse.log`, `reuse-before.sha256` and inode records).
All these logs are under `/tmp/ic-jobs-qualification.l4gwGK/`; `qualified.patch`
and `qualified-source.sha256` bind the full CI invocation before the final fixture
checks and handoff update, while `final-source.sha256` records the final tree.

Producer [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38044218125)
passes Linux, Apple Silicon and lint/security; Intel remains in progress at the
latest observation. Jobs delivery and native macOS qualification of this working
tree remain pending. Linux live evidence does not establish those host results or
final Canic/IcyDB composition. No sibling edits, commit, release, push or
publication occurred. Both owning issues remain open for source-bound delivery.

## Earlier 0.4.7 implementation evidence before delivery

IC Jobs **0.4.6** is pushed at
`33463211e61b007777aeac0699bd5ea2b01820a9`, matching public main. The annotated
`v0.4.6` object `a557c0f8738be6a6b82d3e4f1ae5dee06643e386` matches GitHub,
and the retained release plan is complete. Its exact-source
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38042280637) passes Linux,
Apple Silicon and Intel macOS. This completes the delivered 0.2.13 adoption
qualification for [#14](https://github.com/dragginzgame/ic-jobs/issues/14).
Registry publication was not independently checked.

Pending compatible **0.4.7** adds the unpublished
[consumer canister](../../apps/job-consumer/README.md) and a separate Testkit
host harness for [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
The public Jobs API, stored JobRecord and Rust 1.88 floor remain unchanged.
The new consumer owns its bounded stable snapshot, authenticated management,
counter/delivery payloads, receipt authority and lifecycle exports. The one
watchdog is reconstructed after all retained records pass `Job::restore`.
It commits Running intent before external dispatch; absent receipts retain
blocked work, while exact retained receipts permit explicit disposition.

The selected graph contains Memory 0.34.1, Timers 0.16.7 and Testkit 0.30.0.
Memory 0.34.1 was already selected in the incoming lock; concurrent selection
updates to Timers 0.16.7 and the Testkit 0.30 catalog are retained. Public Testkit
0.30.0 is `6ac161b8ed689012bf8b0ce926946f94ea9f407d`; Timers 0.16.7 is
`999d9b5c3a84ec5abd729ca72b8f259abbb060e1`. Published selected API sources
were reviewed directly. Host/Testkit dependencies stay in the separate host
package; default workspace commands still select the Jobs library.
PocketIC 16.1.0 requires exact thiserror 2.0.18, so the expanded graph necessarily
replaces the previous compatible 2.0.21 selection. A precise attempt to retain
2.0.21 was refused by that upstream constraint; it made no lock mutation.
Other released registry identities remain selected alongside the required new
host dependencies. The final lock SHA-256 is
`73045efaeecaa8bb4800eb9fd6cf83d9f68bc58b77f995eceb37f09bdaa9e72d`.

Five native stable-cell consumer tests pass at Rust 1.88, and the initial focused
native/Wasm/harness compile-only and strict Clippy checks pass. An initial
compile found undocumented Candid fields and an unused import; those were fixed.
Evidence is retained at `/tmp/ic-jobs-consumer-native.log` and
`/tmp/ic-jobs-consumer-check.log`. Final `make check-consumer` passes in
`/tmp/ic-jobs-consumer-check-final.log`: five native tests, a linked Rust 1.88
release Wasm, host harness compilation and warnings-denied native/Wasm Clippy.
The Wasm SHA-256 is
`bea14e94468abb8426bd68cc93a84d5af420b9145e4d755a9f04d1ec353b3820`;
Candid extraction succeeds at `/tmp/ic-jobs-consumer.did`.
The original twenty consumer tests pass separately for core/timers, all 24 job
tests pass, and Rust 1.88 core/timers native/Wasm plus Jobs Clippy pass in
`/tmp/ic-jobs-consumer-integration.log`. That invocation then found committed
metadata export omitted app members, retaining its failed fixture at
`/tmp/jobs-release-tooling.EIJYOK` and selected metadata at
`/tmp/jobs-committed-metadata.dDcXLw`. Exact-commit archive now supplies all
workspace members without a second directory roster. Corrected real local Git
release/publication substitute fixtures pass with Bash 5/GNU Make 4.3 and genuine
Linux Bash 3.2.57/GNU Make 3.81 in `/tmp/ic-jobs-consumer-final-tools.log` and
`/tmp/ic-jobs-consumer-release-bash32.log`. Snapshot, pins, formatting, ShellCheck,
documentation links and diff checks pass. The selected Testkit CLI is not yet
installed: its offline admission correctly refuses, without setup effects, in
`/tmp/ic-jobs-consumer-cli-admission.log`.

The full `make ci` gate remains unrun and explicitly selected under the
maintainer's supplied overlay. Five live harness tests are implemented but
not executed: management/timer upgrade recovery, synchronous write/timer trap
rollback, lost sender-result/receipt disposition, invalid restore with controller
snapshot recovery, and overdue policies. Live PocketIC and native macOS evidence
for this working tree remain pending. No server installation, live gate, commit,
release, push or publication has been performed during this batch.

Shared 0.3.0's complete common toolset remains separate adoption work in
[#15](https://github.com/dragginzgame/ic-jobs/issues/15), awaiting reviewed committed
producer source at the earlier observation; no dirty upstream files were adopted.

## Earlier 0.4.6 adoption evidence before delivery

IC Jobs package version **0.4.5** is pushed at
`c253b3aab8ce8c52ff93b69e1ae14f5dd86b74c9`. Public main matches local HEAD;
the annotated `v0.4.5` object `063d7d0d7682a64513bc754ff9831261d68b1b9e`
matches GitHub's tag reference, and the retained release plan is complete.
The maintainer reports it live; registry publication was not independently checked.
Release-source [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38038868582)
has passed Linux (`114174945921`), Apple Silicon (`114174945776`) and Intel
(`114174945917`) at the exact released source. The configured failure
collector was skipped on the successful Linux gate, so this does not establish
hosted failure-log upload. The released lock changes only the local Jobs version
row from 0.4.4, with SHA-256
`036531dc76445e1c562787a5c77dd8436b6b930d0cd4d460f4c3d1075c90c482`.

Pending compatible **0.4.6** implements the requested upstream adoption. Shared
Tooling **0.2.13** at `5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e`, matching
public main, is adopted through the canonical exporter from clean detached
`/tmp/ic-jobs-adoption.4pCu8K/shared` (78 files). Source/candidate admission and
locked fetch precede selected cargo-edit setup/check. Versioned tool roots retain
older installations; valid selections are checked and reused without replacement,
and invalid existing roots refuse rather than being silently repaired. Ordinary
CI checks tools before dependent dispatch, including under parallel Make.
This adopts the full-delivery-suite baseline; release and live IC execution
retain separate authorization. Jobs has no selected Testkit CLI caller.
[#14](https://github.com/dragginzgame/ic-jobs/issues/14) owns consumer delivery.
Shared's [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38039035514)
passes Linux portable regression, both native macOS hosts and lint/security.

IC Memory **0.34.0** is delivered at
`958080df899ebfa7bb9c2d4c93bb8664bd23575d`, verified as public main and reviewed
from `/tmp/ic-jobs-adoption.4pCu8K/memory`, excluding dirty sibling files. The
maintainer's incoming catalog already selects 0.34; its entire locked selection
is preserved, including Timers 0.16.5 and Metrics 0.3.7, at SHA-256
`b6343c8d97d217ae33c6b0718a075d1f99c58658970268c66df4b0d004a279ee`.
Consumers use the replacement request-only declaration and key-open APIs directly.
One native host owns the shared pool, grants namespaces and bootstraps before
component verification/open. Real stable cells on VectorMemory persist existing
snapshot bytes; cold reopens retain IDs, payloads, deadlines and blocked Running/
Uncertain attempts. A pending cancellation survives another reopen. Foreign
claims fail without opening capability or changing retained backing bytes.
Fault-injection byte stores remain separate native test models, not fallback
readers for another allocation contract. No Job format or installation is reset.
Memory stays a development dependency; normal core/timers trees have no Memory.
The Jobs test graph has one Memory/stable-structures (0.7.2)/Timers identity.
This is native consumer persistence evidence, not installed Jobs canister upgrade,
timer reconstruction, IC rollback or final Canic/IcyDB composition; those remain
with [#11](https://github.com/dragginzgame/ic-jobs/issues/11) and
[Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).

All 20 consumer tests pass separately at Rust 1.88 for core and timers.
The complete `make ci` passes on final code, including native/Wasm Rust 1.88
checks for both feature selections, strict Clippy, doctests and verified packaging.
Current metadata is still 0.4.5; packaging uploads nothing. Maintained actual
release/admission/recovery/publication substitute fixtures pass on Linux Bash 5.2/
GNU Make 4.3 and genuine Bash 3.2.57/GNU Make 3.81. They cover selected-root
absence/invalidity/reuse, refusal before parallel builds, admitted setup/check
ordering and failure, offline/network cache failures, dirty-source refusal and
saved-release resume without reinstallation. Canonical Rust-tool installer
fixtures also pass on both Linux shell profiles. Selected cargo-edit 0.13.13 was
installed into its new root; corrected offline reuse preserves executable inode,
bytes and lockfile. Snapshot, pin, formatting, ShellCheck, workflow, docs and diff
checks pass. Logs are retained under `/tmp/ic-jobs-adoption.4pCu8K/`.
An initial exporter invocation refused its temporary clone's local remote; the
remote identity was corrected before adoption. The first reuse probe revealed
Cargo replacing a byte-identical executable, so setup now explicitly checks and
reuses valid existing roots; the earlier log remains separate. Initial lint
source annotations were corrected before final lint. No sibling edits, commit,
push, release, publication, hook activation or live qualification occurred.
Native macOS results above bind released 0.4.5, not this working-tree adoption.
The patch classification follows unchanged Jobs public API, record and Rust floor:
the Memory hard cut affects private test consumers only.

The preceding **0.4.4** was pushed at
`7470aebe60e10f84a5dd105eb96ee144865ac509`;
the annotated `v0.4.4` object `07743ef1f17f86fcddc930f27796b165bf28cd4f`
matches GitHub's tag reference, and the retained release plan is complete.
Release-source [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38037161776)
has passed Linux (`114169891050`); Apple Silicon (`114169890892`) and Intel
(`114169891111`) were queued at the latest observation. These results qualify
0.4.4 only. No crates.io observation was made for this release during continuation.

Delivered **0.4.5** adopts committed Shared Tooling **0.2.12** at
`a8ba9b461b831846eacf64452e6ddcd2acd000f1`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-0212.gyuP1R/source`.
The clean detached source excludes sibling worktree edits; the selected snapshot
contains 78 files. Canonical admission now checks independently retained Make
invocation evidence and refuses overwritten `MFLAGS`, allowing removal of Jobs'
temporary `_jobs_make_execution_flags` parser. This follows the delivered repair
in [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).
The subsequent committed 0.2.12 repair rejects LF/CR in supplied and resolved
directory paths before snapshot export/verification can select a trimmed neighbor
([Shared #95](https://github.com/dragginzgame/shared-tooling/issues/95)).
The adopted baseline also forbids these operational directory names, preserving
existing artifacts and allowing deliberate negative fixtures. This workspace's
ordinary directory identity needs no rename or reset.

Formatting now reports compact results and retains full failed command output.
The configured CI failure collector archives formatting and tool evidence after
the selected gate. The new `test-formatting-evidence` target checks the actual
wrapper's exact failure status and full log. It now uses Jobs' actual Makefile
and selected includes with a Cargo substitute to prove sorter failure stops
rustfmt, successful checks report one line, and both complete failed logs reach
the local archive. It does not invoke hosted upload. Maintained release fixtures additionally refuse
unsafe modes with both `MAKEFLAGS` and `MFLAGS` erased, as well as harmless-looking
command-line and file assignments to `MFLAGS`, before release/publication effects.

The original 0.2.11 adoption's focused Jobs release, actual formatter/hook,
formatting-retention and canonical
Shared release/formatter fixtures pass on Linux with Bash 5.2/GNU Make 4.3 and
genuine Bash 3.2.57/GNU Make 3.81. Logs remain under
`/tmp/ic-jobs-shared-0211.zDs1gB/`. Those selected Make/formatter payloads are
unchanged by 0.2.12; the earlier runs retain their original source binding.
The 0.2.12 snapshot-distribution fixture and extended Jobs formatting/archive
fixture pass separately on Linux Bash 5.2/GNU Make 4.3 and genuine Bash 3.2.57/
GNU Make 3.81. Logs remain under `/tmp/ic-jobs-shared-0212.gyuP1R/`.
Current snapshot integrity, pin admission, actual release adapters, formatting,
ShellCheck, workflow lint, documentation links and whitespace checks pass.
An initial lint attempt stopped on an unresolved source-file annotation in the
new fixture; the annotation was corrected before final lint qualification.
The 0.4.4 lock remained unchanged during this pre-release batch at SHA-256
`682325851ac31ad8e4ea1a0a6abd1501223d45dec059c9e70d98113089df259b`.
This tooling-only patch changes no Rust API, record, dependency selection or
compiler floor. No full local CI/release gate, Rust requalification, native macOS,
live IC or hosted upload qualification was run for this source. No commit, push,
release, publication or real hook activation was performed by the agent for
the local batch. The maintainer subsequently delivered 0.4.5; matching-source
Linux and both macOS results now complete acceptance for
[#12](https://github.com/dragginzgame/ic-jobs/issues/12) and
[#13](https://github.com/dragginzgame/ic-jobs/issues/13).

The preceding IC Jobs 0.4.3 was released at
`7939c8a53add5ab019cc26e97bd93274b5ed3a43` and pushed by the maintainer.
The exact local annotated `v0.4.3` object
`0a81bc12e25993a995df326827bd6bf74a0cedec` matches GitHub's tag reference;
the retained local release plan is complete. The maintainer reports it live,
and an exact crates.io observation confirms 0.4.3 present without binding
registry bytes to a local artifact.
Release-commit [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37954681664)
has passed Linux, native macOS Intel and Apple Silicon, including five-tool
setup, offline admission and the configured selected-graph gate. Jobs
`113902068497`, `113902069034` and `113902068936` qualify released 0.4.3,
not the subsequent management fixture or incoming Metrics graph.
The preceding 0.4.2
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37951742672)
has passed Linux; both native macOS jobs have since been cancelled without
passing. The preceding 0.4.1
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37941395642)
has passed Linux; both native macOS jobs have since been cancelled. The preceding 0.4.0
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37936803386)
has passed Linux and macOS Intel; Apple Silicon has since been cancelled.
The preceding 0.3.1
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37925974097)
passed Linux and macOS Intel; Apple Silicon has since been cancelled. These older
source-bound native gaps are not relabelled by the later all-host 0.4.3 result.
The preceding 0.3.0
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37918180627)
has passed Linux, macOS Intel and Apple Silicon at its exact release source,
including explicit five-tool installation, offline admission and the configured
selected-graph gate. The preceding
0.2.3 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37909918403)
has passed Linux, macOS Apple Silicon and macOS Intel at its exact release SHA.
Its original native job identities survived subsequent pushes and completed
successfully. These results do not qualify later source or live IC recovery.
The preceding 0.2.2 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37903041952)
has passed Linux, macOS Apple Silicon and macOS Intel at its exact release SHA.
Those earlier native jobs retain their original identities after the 0.2.3 push:
queued retention is observed, and both retained native jobs have completed successfully.
Running retention is now observed separately: the original 0.3.1 Intel job's
validation step ran from 13:25:40Z to 13:28:07Z on 2026-10-09, crossing creation
of the subsequent 0.4.0 push run at 13:26:03Z, and completed successfully.
See [the hosted retention evidence](https://github.com/dragginzgame/ic-jobs/issues/3#issuecomment-6082354943).
Release 0.2.1 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37899775906)
passed all three hosts. Its earlier native results do not qualify later source
or live IC recovery. Owning issues retain the outstanding native/hosted acceptance.
The library owns job policy and checked metadata transitions. Applications own
persistence and effects; the optional adapter uses the consumer-selected IC Timers
dependency graph.

The maintainer requested notes for an application-owned canister management API,
IC Memory-governed stable storage and timer reconstruction across upgrades.
The [consumer composition](../test-consumers.md#proposed-durable-canister-consumer)
records ownership, mutation/rollback ordering, startup and blocked-effect rules;
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) owns implementation and
IC Testkit qualification. The 0.4.4 continuation implements the native notification
management fixture described below; actual endpoints, stable storage and upgrade
evidence remain pending the reviewed replacement allocation contract in
[IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).

Delivered **0.4.4** extends the existing application-owned notification consumer
with manager-authorized create, inspect, bounded job-ID pagination, pending
cancellation and due dispatch. Admission precedes payload/storage access; all
restoration uses `Job::restore`. Replies carry the earliest committed deadline,
and failed writes retain the previous queue/deadline without exposing delivery.
Cancellation cannot undo Running/Uncertain effects; reconstruction and management
dispatch preserve blocked intent until destination evidence permits disposition.
No library API, record, scheduling policy or compiler-floor change is made.
This is compatible test-consumer work delivered as a patch.
The replacement upstream cut was not available for that adoption; no provisional 0.33 range adapter,
duplicate allocator or lifecycle export is introduced in Jobs.
Focused Linux Rust 1.88 tests pass for all 17 consumers in each core/all-feature
selection, including the six new management boundary checks. Optional timer
coverage projects committed deadlines without registering a platform timer.
Warnings-denied development Clippy, formatting, documentation links and whitespace
checks pass. Native caller policy and byte-store reconstruction are not evidence
of Candid authentication, real stable-memory persistence, message rollback or IC
upgrades. Native macOS and live IC qualification remain separate. No full local
CI, commit, release, push, upload or live qualification gate is run for this batch.

The authorized IC Memory addition declares `ic-memory = "0.33"` in the root
catalog and inherits it only as a Jobs development dependency, locked to 0.33.4.
Reviewed public main is `9137192d4b3425a229fa21df5f343883f56d80e8` (0.33.4);
its delivered contract predates the host-wide pool hard cut. Both fixtures contribute
key-only requests for their combined record/payload or maintenance state stores.
Native composition checks reject conflicting consumer identities and preserve
job bytes/deadlines; request order does not change the sealed declaration meaning.
Fresh-runtime opens refuse before bootstrap. No host range adapter, global
registration, storage trait, library dependency or lifecycle export is added.
The byte-replacement stores remain authoritative until the final host integration
can adopt [IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).
All 19 consumer tests pass separately at Rust 1.88 for core and optional timers
on the current Memory/Timers/Metrics graph. Library-only normal dependency trees
confirm Memory is absent from both feature selections. Rust 1.88 native and Wasm
library checks pass separately for core and timers. Warnings-denied consumer
Clippy, locked metadata, declaration checks, formatting, snapshot integrity,
documentation links and whitespace checks also pass. No complete CI/release gate
or native macOS/live IC qualification was run for this dependency addition.
This native declaration
evidence does not establish IC stable-store persistence, allocation recovery,
Canic/IcyDB composition, message rollback or upgrades. The addition extends
compatible 0.4.4, subsequently delivered by the maintainer.
The [Memory owner initially reported local implementation](https://github.com/dragginzgame/ic-memory/issues/44#issuecomment-6095440378)
of the 0.34.0 shared pool, namespace grants and unmanaged exclusions. That work
was then uncommitted; Jobs still selected 0.33.4. Delivery and native Jobs adoption
are now recorded for pending 0.4.6 above. Final Canic/IcyDB/Jobs coordination and
real IC recovery remain at #44/#11; earlier reports do not establish that evidence.

The authorized [Make repair](https://github.com/dragginzgame/ic-jobs/issues/12)
adopts committed Shared Tooling **0.2.9** at
`f8a70ba348e9975a6eb5b337860b00bc8a0b36d1`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-029-fix.HZfYBT/source`.
The 74-file selection recorded the exact committed version. The admission
helper follows the selected include rather than an ambient runtime root and
uses `MAKE_COMMAND` without importing argument-bearing recursive `MAKE` files.
Fleet reports and hosted artifact transport are outside Jobs' selection.

Shared's hidden-mode gap was tracked at
[Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).
Before repair, `-i release-patch MAKEFLAGS=` dispatched a substitute runner
exiting 23 and returned false success; inputs/logs remain at
`/tmp/ic-jobs-make-flags-review.amyFpk/`. The extended maintained Jobs fixture
also failed its new cleared-flags case before implementation, retained at
`/tmp/jobs-release-tooling.DhtoP6/`. Jobs 0.4.4 checked invocation modes retained
in `MFLAGS` at its own Makefile boundary, before the shared includes. This
explicit consumer-owned adapter is permitted by the baseline; no vendored
guard was patched. The supplementary parser is now retired by the committed
canonical repair and actual-consumer regressions delivered in 0.4.5 above.
Shared #30 is closed; source-bound consumer acceptance for #12 now passes.

Focused release/publication, actual adapter, root/recursive-Make and formatting
fixtures pass on Linux with Bash 5.2.21/GNU Make 4.3 and genuine
Bash 3.2.57/GNU Make 3.81. The maintained Jobs cases cover all four unsafe modes,
long forms and a cluster, direct/inherited selections and cleared/replaced
`MAKEFLAGS`, refusing release/resume, formatting and publication before fetch,
gate or source mutation. Ordinary keep-going/silent flags, quoted selections and
parallel recursive Make still work without jobserver warnings. Real local Git
release/recovery uses substituted gate/cache/registry/upload effects; this is
not live release or complete-gate evidence. Logs remain in the adoption directory.
Snapshot integrity, declaration checks, formatting, ShellCheck, documentation
links and whitespace checks pass. The first post-repair fixture attempt stopped
on an uncached incoming Timers 0.16.4 at `/tmp/jobs-release-tooling.WDnSXs/`;
`cargo fetch --locked` then prepared that exact graph without altering its bytes.
The compatible repair was delivered in 0.4.4 without library/API/storage/MSRV
changes. No sibling edits, commit, push, real release/publication, hook activation,
full local CI or native macOS/live IC qualification were performed.

The maintainer requires Rust 1.88 for the complete package. Manifest metadata,
native/Wasm minimum checks, CI setup and current support documentation now use
that floor for both core and timers. Release 0.2.0 drops the previously advertised
1.85 floor and changes the consumer toolchain contract; consumers must upgrade
to 1.88. The compatible publication-path and release-cache fixes are delivered
in 0.2.3. The developer IC bundle hard cut, shared-tooling fixes and locked
dependency refresh are delivered in 0.3.0. The final-pin-record admission and
application-owned test consumers are delivered in 0.3.1. The optional IC Timers
package-identity change is delivered in 0.4.0. Test-consumer recovery coverage
and shared hook fixes are delivered in 0.4.1. The locked Timers refresh and shared
Make adoption are delivered in 0.4.2. The compatible Shared Tooling 0.2.7 Make
admission and fixture isolation fixes are delivered in 0.4.3; package metadata
is 0.4.4; the compatible management fixture, Memory declaration and Make repair
are delivered in 0.4.4. The next canonical Make and formatting-evidence repair is
collected under the undated 0.4.5 draft. The tooling refresh makes no library or dependency
contract change.
Job APIs, stored fields, scheduling policy and the Rust floor are unchanged.
Earlier 1.85 qualification below records historical evidence only.

The released 0.4.2 lock selects Timers 0.16.3 and Metrics 0.3.2, at SHA-256
`5f1e57f836b5265a545e0f3161b59419f6e2b76503c72bd06b4a11341825f496`.
Its finalized changelog names the earlier Timers 0.16.2 working selection; the
released lock owns the actual graph. The published entry is preserved.
The pre-release maintainer lock independently advanced Metrics to 0.3.3, retaining
Timers 0.16.3; its tested tooling-only bytes were preserved at SHA-256
`c5204f0b9bc6f8dc54fec4786541f7c08f4dda8012d179f87fd49064c672ebd1`.
Release 0.4.3 retains that dependency selection, updating the local package row;
its lock SHA-256 is
`5c4f56be2c0abbc1cd137ec841c4c3d77dcd87e70416b2fa9f73e07fd02e7322`.
The local tooling fixtures did not qualify that Rust graph. Exact-source 0.4.3
Linux and native macOS CI now qualify the released graph separately; live IC
evidence remains pending.

The management batch's tested maintainer lock selected Metrics 0.3.4 while
retaining Timers 0.16.3. Its bytes were preserved throughout that batch at SHA-256
`7f7086e572d3476482c5ce4f1e94f9295f12a35c0d5498d3190f936a351987d4`.
The focused 0.4.4 native consumer checks above use that graph; they do
not relabel released 0.4.3 CI or establish a complete dependency/MSRV/Wasm gate.
An intervening maintainer lock selected Metrics 0.3.5, still with Timers 0.16.3,
at SHA-256 `8a3246b1c611e5d5f7842e514fcbe1c64c86f90ea474c973e4abb736da6e744d`.
The earlier incoming maintainer lock selected Metrics 0.3.5 and Timers 0.16.4 at
SHA-256 `af774a3bdd96463c2dc7662e3174a0c4b89ef10c9686bc839fc923282a54cfec`.
That graph is cache-prepared and preserved byte-for-byte during the Make repair;
metadata fixtures do not establish Rust/MSRV/Wasm qualification for it.
The pre-release 0.4.4 tested lock also selects development IC Memory 0.33.4 and its exact
stable-structures 0.7.2 substrate, retaining Timers 0.16.4 and Metrics 0.3.5,
at SHA-256 `67dfef93be379e92df50716137d2bb5bce1a6effd11ea3a99994115b07aca9ca`.
Locked cache preparation and metadata inspection pass; the consumer tests above
qualify this graph separately from the older management-only results. Native
macOS and live IC evidence remained outstanding for that local batch. Release
0.4.4 retains those dependency selections and updates the local package row;
its lock digest and exact-source CI status are recorded above. Live IC remains pending.

The current [Make follow-up](https://github.com/dragginzgame/ic-jobs/issues/10)
adopts committed Shared Tooling 0.2.7 at
`47d6ae6488b8007323fa7c2e22a6efa11d77ae63`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-027-review.ri8p21h9/source`.
The selection grows to 74 files with `make/execution.mk`; both it and its existing
script companion are included in the release adapter fixture. Outer Make now
refuses ignore-errors, dry-run, touch and question modes before recipe effects.
The maintained Jobs fixture checks direct and inherited modes for all release
entrypoints and formatting, preserving Git identity, source and fetch/gate state.
The canonical release checker isolates its tooling root from inherited selections.
Focused checks pass on Linux Bash 5 and genuine Bash 3.2.57: real-Git release and
publication fixtures with substituted gate/registry/upload effects, actual release
adapter checks with a conflicting inherited root, and actual formatter/hook
installation with sorting and source/index preservation. Normal parallel Make
with a quoted variable and the default help goal also pass. Snapshot integrity,
dependency declarations, documentation links, formatting, ShellCheck and whitespace
checks pass. Logs remain in the adoption directory. The incoming Metrics lock
bytes above are unchanged; tooling fixtures do not qualify that Rust graph.
Native macOS and live IC qualification remain separate. No full CI, release,
publication, commit, push or hook activation was performed by the agent for
this adoption. The maintainer subsequently delivered it in 0.4.3, whose exact
Linux, Intel macOS and Apple Silicon CI now pass as recorded above, completing
native consumer acceptance for #10. Live IC qualification remains separate.

The authorized [Make adoption](https://github.com/dragginzgame/ic-jobs/issues/10)
selects committed Shared Tooling 0.2.6 at
`ce13a5314916891fd239d9b199b4a91b04775054`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-026._zai8h6t/source`.
The snapshot grows from 71 to 73 files by explicitly selecting `make/release.mk`
and `make/rust-format.mk` with their existing companions. The subsequent
Make-execution follow-up was excluded from that adoption. Standard release routing and
conflicting-goal admission now have the shared owner; Jobs retains direct-only
guards, release-tool admission, metadata/gate adapters and explicit setup/check
extensions. Formatting uses the same prepared pins/toolchain and now carries
offline/no-auto-install settings throughout both recipes. The default goal
remains help. No Rust function, type, record or scheduling policy changes.
Focused checks pass on Linux Bash 5 and genuine Bash 3.2.57: actual Make release
argument/failure/conflict checks; direct-only admission and exported environment;
formatter missing/wrong-version refusal and failure isolation; real formatter/hook
sorting, partial-stage rejection and unrelated edit/index/lock preservation; and
real-Git direct release/recovery/publication fixtures with substituted gate,
registry and upload effects. The Make adapter fixture now includes both new files.
Logs and the additional actual-Make probes remain in the adoption directory.
Snapshot, dependency declarations, formatting, docs, ShellCheck and whitespace
checks pass; incoming Cargo.lock remains byte-identical at the digest below.
The maintainer subsequently delivered this adoption in 0.4.2, whose exact Linux
CI passes as recorded above; its macOS jobs were subsequently cancelled. The
later 0.4.3 result qualifies the adopted tooling on all three hosts. No full
CI, real release/publication, commit, push or hook activation was performed by
the agent during that adoption.
The [reviewed follow-up](https://github.com/dragginzgame/ic-jobs/issues/10#issuecomment-6083691125)
reproduced ignore-errors mode bypassing formatter admission in a disposable Jobs
0.2.6 Make copy, then verified refusal before formatter calls with the committed
0.2.7 guard. That follow-up is delivered in 0.4.3; 0.4.2 records the earlier
delivered 0.2.6 baseline.

The released 0.4.1 lock selects IC Timers 0.16.1 and IC Metrics 0.3.2, at SHA-256
`4d27cde30ee4dc43d019e134ef3a55b17a2c34381e4f00db96f8e35b6e4dc1e5`.
The earlier working lock selected Timers 0.16.2, changing only that package's
version/checksum; its tested bytes were preserved at SHA-256
`cb85b8c1879b98aa384e224828dfa225cf37bae09ad296bad08937c34127c015`.
The 15 published Timers library source files match 0.16.1 byte for byte, and the
tagged library comparison is unchanged. No adapter rewrite, dependency identity,
job storage conversion or compiler-floor change is needed. Explicit locked fetch
prepared the missing registry input without changing Cargo.lock. Focused Linux
qualification on this incoming graph passes Rust 1.88 core/all-feature tests
(23/24 job tests and 11 consumer tests in each selection), core/all-feature native
and Wasm checks, warnings-denied development Clippy, rustdoc and both doctests.
This evidence is separate from released 0.4.1/0.4.2 CI and the later incoming graph;
native macOS and live IC recovery remain separate. No full CI, commit, release, push or
upload was performed for this continuation.

The released 0.4.0 catalog/lock select registry IC Timers 0.16.0 and IC Metrics
0.3.2, retaining the maintainer's other refreshed lock selections. Released bytes
are preserved at SHA-256
`f68c66b218cdfcfee9b33b8c93a30db20ccbd7190478e0a6e00ddc55454798d5`.
The earlier working adoption selected Metrics 0.3.1 and retained its lock at
`b2dad5af4c1540a6f4f2f26f01426032ff64f36209ff308e0e846faeea1aee62`.
The optional adapter exposes Timers types, so consumers must align timer owners
to 0.16 and exchanged summaries to Metrics 0.3 or the Timers reexport. This is a
pre-1.0 breaking dependency identity and requires the next minor; Job APIs,
durable fields and scheduling transitions do not change. Tagged upstream source
comparison finds no timer runtime or Metrics arithmetic changes between the
released 0.14.23/0.2.20 and selected 0.16.0/0.3.1 library source. No adapter shim,
database or host dependency is added. The prepared cache supports focused Linux
Rust 1.88 core/all-feature native and Wasm checks, all 24 job tests, all nine
consumer tests in each feature selection, warnings-denied Clippy, rustdoc and
both doctests. Formatting, declarations, documentation links, whitespace checks
and verified all-feature packaging pass; packaging still uses local metadata
0.3.1 and uploaded nothing. Those local adoption results retain their earlier
graph; the maintainer subsequently delivered 0.4.0 with the refreshed lockfile.
Its exact Linux CI passes as recorded above; native macOS and live IC acceptance
remain separate. The agent ran no full local CI, package version bump, release,
push or publication for that adoption.

The 0.4.1 consumer continuation extends only the private test consumers. A separate
destination retains bounded exact-attempt/payload rejection evidence, enabling
explicit retry disposition after a lost rejection reply while absence remains
blocked. Disposition writes must commit before retry admission; stale evidence
is refused and the original retry budget still ends in Failed. Accepted execution
receipts take precedence for duplicate requests. The byte store now supports a
failure after one successful write to qualify partial maintenance-batch recovery:
only the first occurrence survives, and reconstruction resumes at the uncommitted
occurrence. This is native storage-composition evidence, not message-wide IC
rollback, live provider delivery or an actual upgrade. No library API, record,
dependency or compiler-floor changes are made.
Focused Linux qualification passes on the released graph: all 11 consumer tests
in each core/all-feature Rust 1.88 run, warnings-denied development Clippy,
formatting, documentation links and whitespace checks. The 0.4.0 lockfile remains
byte-identical during that batch. No full CI, native macOS/live IC gate, commit,
release, push or publication was performed by the agent for that batch. The
maintainer subsequently delivered it in 0.4.1 with the Timers 0.16.1 lock;
its exact Linux CI passes as recorded above.

The Shared Tooling refresh delivered in 0.4.1 adopts committed 0.2.5 at
`04e07b4bf54e7aeb03eb7804a845cee27b7305df`, confirmed as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-025.rNcuhh/source`. The 71-file
selection stays intact and pending consumer work is preserved. The hook and
installer preserve literal newline-ending paths, refuse existing different hook
selections without modifying configuration, and retain failed Git observations.
See [shared #89](https://github.com/dragginzgame/shared-tooling/issues/89).
The released Jobs hook payload fails the new canonical regression, retaining
`/tmp/git-hooks-test.WsFlSx` and `hooks-before.log` in the adoption directory.
The fixed canonical hook fixture passes on Linux Bash 5 and genuine Bash 3.2.57,
including configured local/inherited hook paths, newline-ending checkout names,
failed Git reads, index preservation and prepared-tool discovery. Jobs' actual
formatter/installer adoption passes with real formatters and its current Make
adapter, including manifest sorting, partial-stage refusal and failure isolation.
Logs remain in the adoption directory. New snapshot companion declarations
concern optional upstream test/LOC suites outside Jobs' selection; no executable
or test-suite addition is needed. CI queue guidance is adopted without scheduling
maintenance or changing CI policy. No further Jobs library repair is identified
by this refresh; native macOS and live IC qualification remain separate.
Snapshot integrity, declarations, documentation links, formatting, standard
release Make adapters, ShellCheck and whitespace checks pass. Cargo.lock remained
byte-identical during that adoption at the 0.4.0 released digest above. No full CI,
commit, release, push, publication, native macOS or live IC run was performed by
the agent for that refresh.
The previously uncommitted Apple Make formatter fixture follow-up is included
in the reviewed 0.2.6 source adopted above. Jobs does not vendor that upstream
fixture; its actual consumer Make/hook checks remain separate from canonical
fixture results. Linux Bash 3.2 does not establish native macOS qualification.

The maintainer requested in-repository test consumers.
[Notification and cache-maintenance fixtures](../test-consumers.md)
now exercise the public API with bounded application-owned byte storage, immutable
notification payloads and a separate destination receipt owner. Every load uses
`Job::restore`. Notification delivery is exposed only after Running intent commits;
lost replies and failed result writes retain blocked intent until explicit receipt
reconciliation. The notification queue now supplies up to four unique jobs to
the scheduler, preserving earliest/tied ordering and allowing other due work to
proceed past blocked intent. Insertion/cancellation rederive the shared wakeup;
excess/duplicate retained records and corrupt future work reject before dispatch.
Interruption before delivery retains blocked intent even when no receipt exists.
Local cache effects commit with their metadata, with at most two
recurring runs per wake. `make test-consumers` qualifies core/timers separately on
Rust 1.88 and is selected by configured CI. Timer tests project actual fixture
deadlines through `complete_batch`; native execution does not qualify live
watchdog registration, provider delivery, stable memory or IC upgrade/rollback.
The consumer batch introduced no dependency, library API or storage-contract change.
Focused Linux checks passed before 0.3.1 delivery: all nine consumer tests in each
core/all-feature Rust 1.88 invocation, warnings-denied development Clippy,
formatting, documentation links and whitespace checks. The released lockfile is
unchanged during that batch. The agent ran no full CI, native macOS or live IC gate,
commit, release or publication for the consumer batch; the maintainer subsequently
delivered it in 0.3.1, whose exact Linux CI passes as recorded above.

The released 0.2.3 lockfile selects IC Timers 0.14.23 and IC Metrics 0.2.18 within the
existing compatible requirements. Explicit `cargo fetch --locked` downloaded the
missing Timers input before delivery; that pre-release lockfile's SHA-256 remained
`3c423922eda86146028b688b0ac37a83ea4cd01490b5149aebc8107d36cfc980`.
Focused Linux checks passed on that graph: Rust 1.88 native and Wasm library
checks for core/all features, all 24 job tests with timers, and warnings-denied
development Clippy. This replaces the earlier missing-cache check limitation;
it does not qualify native macOS or live IC recovery.
The released 0.3.0 lockfile selects IC Metrics 0.2.20 with IC Timers 0.14.23.
Before release, that maintainer selection was preserved byte-for-byte at SHA-256
`be8149670998dccb7516e292f831b43e3c28131ac248917402a99f79bdc8d23e`.
Its cached graph now passes focused Linux Rust 1.88 core/all-feature native and
Wasm checks, all 24 job tests with timers, and warnings-denied development Clippy.
The tagged Metrics 0.2.18-to-0.2.20 arithmetic source and package requirements are
unchanged; the upstream fixes concern host tooling. Native macOS and live IC
recovery remain separate qualification requirements for this working tree.

The initial compatible batch delivered in 0.3.0 adopted committed Shared Tooling revision
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

The hard cut delivered in 0.3.0 adopts committed Shared Tooling revision
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
IC qualification was run by the agent for this cleanup. The maintainer subsequently
delivered it in 0.3.0; its exact CI now passes all three supported hosts as
recorded above.

The adoption delivered in 0.3.1 selects committed Shared Tooling 0.2.2 at
`ee48bb37c98c771e77b92fd891f0757d8c1c8b99` through the canonical exporter
from `/tmp/ic-jobs-pin-final-row.u6skwt/source`, retaining the 71-file selection.
Both IC installer loops now consume the final populated pin record without a
trailing newline; the validator, original pin bytes and per-tool checks stay
authoritative. The released Jobs installer reproduced the missing final tool
in the canonical synthetic fixture, retained at `/tmp/ic-tools-test.xgzxR7`;
before/fixed logs are under `/tmp/ic-jobs-pin-final-row.u6skwt/`.
See [#9](https://github.com/dragginzgame/ic-jobs/issues/9) for consumer acceptance.
Shared's independent CI executable publisher is outside Jobs' selected executable
set; its revised guidance is adopted without adding that helper or a new mode.
The corrected canonical IC fixture passes on Linux Bash 5 and genuine Bash
3.2.57 with synthetic assets for all three host selections, including missing
final newline, wrong final-tool version and original pin/receipt preservation.
Jobs' actual prepared bundle passes offline checks under Bash 3.2; setup reuse
with a failing curl substitute retains its active selection, pins and receipt.
During that tooling batch, the released 0.3.0 Cargo.lock stayed unchanged at SHA-256
`0bad777cf80f3560ef09092f7bc1bb857e6ec7eccbbce09186d7ca63f9f6ea83`.
Snapshot, actual declarations, docs, formatting, Make adapters, ShellCheck and
whitespace checks pass. No full local CI, native macOS or live IC execution was
run by the agent for this patch; no commit, package bump, release, push or upload
was performed by the agent. The maintainer subsequently delivered it in 0.3.1;
its exact Linux CI passes, while native consumer acceptance remains with #9.

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
released/published 0.2.3. Its exact release CI now passes all three supported
hosts as recorded above, completing the native acceptance of the path and cache
repairs. The combined cache/path release fixture
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
matrix, permissions, gates or timeout. Delivery and hosted queued/running
retention across consecutive pushes are qualified, with evidence in
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
