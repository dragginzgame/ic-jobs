# Current handoff

## Source and pending work

Delivered Jobs **0.9.3**: `8cdf0ace712e8346e55e7f7baaea414578e3dfc8`.
The working tree prepares compatible **0.9.4** for the remaining artifact-owner
work in [#42](https://github.com/dragginzgame/ic-jobs/issues/42).
It also tightens restore admission for impossible elapsed retry histories in
[#44](https://github.com/dragginzgame/ic-jobs/issues/44), without changing the
stored shape or permitting unresolved effects to run.
Own release metadata, staging and Git refs remain maintainer-owned.
Shared Tooling is now **0.12.0** at
`f7fe31bf80a2b445bf37a470527f001210241aea`, canonically exported from a clean
exact-source checkout, excluding sibling dirty work. The 92-file snapshot retains
existing roots and explicitly adds the publication guide referenced by the new
release contract. Its initial missing-link failure is retained and reported at
[Shared #163](https://github.com/dragginzgame/shared-tooling/issues/163).
Common tool versions/digests and incoming Cargo graph/source are unchanged.

Shared now requires numbered top-position release candidates. Actual Jobs
preflight fixtures reject Draft/misplaced candidates before cache/tool/gate,
metadata, intent or Git effects. The additive publication-attempt runner is not
selected; Jobs retains its existing admitted publication/upload contract.

The unpublished host harness now depends on published IC Build **0.2.5** and
provides `jobs-capture-artifact`. It delegates exact capture and explicit retention
to IC Build; Jobs owns filenames, optimizer flags, source/tool evidence and
custody after retention. Failed final renames print and retain the exact capture.
Successful finalization removes only the empty owned staging child. Recovery
qualifies the single frozen original and captures each optimized producer before
consumption. The wrapper records the capture executable's digest.

`build-recovery-tools` builds the helper with Rust 1.88 after live server admission
in the explicit recovery route. Tooling fixtures build it independently and
exercise the actual executable with substitute optimizer/server effects. Capture
unit tests run under `check-consumer`; there is no second copy engine or server
startup in these focused checks. Library/canister dependencies, durable records,
restore admission and lifecycle/effect authority are unchanged by capture;
the separate retry correction tightens only malformed-record admission.

Incoming Testkit **0.38.0**, Timers **0.22.2** and the other pre-existing manifest/
lock selections are preserved. Only IC Build and its additional dependency rows
are added, plus the host harness's dependency edge. Canonical setup and offline
admission now pass for the lock-selected Testkit 0.38 CLI; the retained selection
receipt binds published version 0.38.0 and its exact source checksum. Receipt,
setup/check logs and unchanged graph proof are retained
under `target/qualification/issue42-cli/`. The admitted shared build is reused;
PocketIC setup, server launch and live qualification remain explicitly selected.

## Focused proof

The maintainer's complete-gate attempt failed Jobs' formatting fixture before
hook qualification: it assumed `candid` and `ic-cdk` remained adjacent after
adding `ic-build`. The fixture now swaps the first two adjacent workspace
dependencies without package-name assumptions. Focused hook qualification and
ShellCheck evidence live in `target/qualification/formatting-adjacency/`;
the failed full gate is not treated as passing evidence.

Retry timing evidence: `target/qualification/issue44-fix/`. Rust 1.88 core and
timers checks retain valid transition histories and all 354 frozen records;
regressions reject too-early uncertain retries, shortened prior completion
evidence and forged rescheduled histories. Exponential/capped delay sums and
wide arithmetic bound work for maximum untrusted counters. This changes product
restore admission, so the preceding capture proof alone does not qualify it.
Native/Wasm minimum checks, strict Clippy, rustdoc, consumer tests, release Wasm
and harness compilation also pass on the repaired source. Full CI and live
qualification remain unselected under the maintainer's explicit gate scope.

Evidence: `target/qualification/issue42-capture/`. Capture tests prove retained
final bytes survive producer removal and failed renames preserve exact bytes.
Actual consumer Wasm and O3/Os/Oz outputs compare byte-for-byte after capture.
Bash 5 and genuine Bash 3.2 wrapper fixtures use the real capture executable:
producer replacement cannot affect frozen inputs; hashes/logs/flags remain;
invalid Wasm/helper paths refuse before attempt/server creation; changed/missing
CI inputs block subsequent validation. Optimizer/server effects are substituted.

Rust 1.88 core/timers tests pass separately, including frozen-record roundtrips
and 21 notification consumer tests each. Native/Wasm minimum checks, all 16
stable-consumer tests, release Wasm, capture tests, harness compilation and strict
native/Wasm Clippy pass. These are local Linux and portability observations,
not full CI or live IC recovery qualification. ShellCheck, formatting, pins,
documentation links and the real disposable release/publication fixture pass;
its gate/registry/upload effects are substituted.

## Qualification boundaries

Routine issue acceptance uses the latest delivered maintained implementation
and complete required matrix under Shared 0.12. For delivered Jobs 0.9.3,
[CI 38522672701](https://github.com/dragginzgame/ic-jobs/actions/runs/38522672701)
passes Linux and fails both macOS architectures. Both exact failed job logs show
Host 0.17.7 retained permissions receiving u32 where Darwin requires u16.
[Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84) owns the still
unpublished repair. No prior cancelled run is an additional active gate and no
native success is inferred from local drafts.

Current follow-up lives only in [#41](https://github.com/dragginzgame/ic-jobs/issues/41),
[#42](https://github.com/dragginzgame/ic-jobs/issues/42), and
[#43](https://github.com/dragginzgame/ic-jobs/issues/43), and
[#44](https://github.com/dragginzgame/ic-jobs/issues/44). Upstream capture evidence
stays with [Build #14](https://github.com/dragginzgame/ic-build/issues/14).

Shared adoption evidence: `target/qualification/shared-0120/`, including canonical
preview/refresh, complete common setup/admission, release/publication and
formatting/evidence/input-drift fixtures. The final release fixture runs on
Bash 5 and genuine Bash 3.2 including child shells. Snapshot/pins/formatting/link
checks and ShellCheck pass. The first missing-doc attempt remains retained.
The snapshot adoption itself did not change product Rust source or dependency
selections; the subsequent retry repair has separate evidence above. No complete gate,
release, upload or live workflow dispatch is selected by this adoption.

Last inspected live recovery remains Jobs 0.8.1 source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.
It does not qualify this graph. No live gate or workflow dispatch ran.

See [frozen history](history-through-0.9.1.md), [architecture](../../README.md),
[safety](../../SAFETY.md), [consumer qualification](../../apps/job-consumer/README.md),
and [delivery procedure](../releasing.md).
