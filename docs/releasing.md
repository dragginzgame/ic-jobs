# IC Jobs release and publication

IC Jobs uses the reviewed [Shared Tooling release contract](releases.md) with
consumer-owned Cargo metadata and validation. The reviewed
[common Make entrypoint](make-interface.md) owns setup, formatting, hooks,
snapshot verification and release routing. IC Jobs selects direct release
delivery only: remote `origin`, branch `main`, annotated tag `vX.Y.Z`. All three
SemVer commands use the same runner and complete gate. They do not publish to
crates.io.

## Preparation

Install the complete host, IC and Rust toolsets with `make install-tools`.
Shared metadata preparation renders workspace versions directly; cargo-edit is
not required.
Prepare Rust 1.91.0 and its Wasm target as described in the README, alongside
the development toolchain and its Wasm target for consumer Clippy.

Shared Make entrypoints require recipe execution and failure propagation.
Remove `-i`, `--ignore-errors`, `-n`, `-t`, `-q` and inherited equivalents;
these modes are refused before recipes run, including formatting commands.
The shared probe checks invocation modes retained in `MFLAGS` independently
when command-line `MAKEFLAGS` is cleared or replaced. Leave `MFLAGS` generated
by GNU Make; command-line and Makefile assignments to it are refused. Jobs uses
this canonical admission without a supplementary flag parser.

Formatting reports a short success/failure summary. A failure retains the full
formatter output at the reported path; configured CI collects those logs and
tool evidence through the reviewed shared action. `make test-formatting-evidence`
checks local failure propagation and archive contents without a hosted upload.

Before ordinary offline checks, prepare the selected dependency graph explicitly
with `cargo fetch --locked`. Standard release commands include that locked fetch
in preflight, after source/candidate admission and before offline validation or
metadata writes; no separate manual fetch is required. This preparation may use
the registry but does not upgrade dependencies or rewrite the lockfile. Cargo's
explicit offline environment/configuration remains authoritative; missing inputs
then stop the release with its original fetch failure. Version reads, metadata
checks stay offline. After successful admission/fetch, preflight runs
`make install-tools` followed by `make tools-check`; setup reuses
the selected installation and does not replace older roots. Saved-release
reconciliation precedes this preparation. Ordinary `make ci` never installs:
it runs `validation-tools-check` before dispatching its checks/builds, including
under parallel Make. `make test-release-tooling` admits its formatter
prerequisites before starting the fixture. The validation gate remains
offline. CI and release validation checkpoint all workspace manifests, the lock,
toolchain, snapshot, Makefile and selected tool catalogs. A changed, missing or
unreadable input stops later stages and retains input identities and logs. Nested
validation in this checkout keeps the original checkpoint. Prepare the current
inputs and restart the complete gate after a refusal; prior validation cannot be
reused. Commit the intended source through the contribution workflow; release
commands require an existing source commit and refuse unrelated
staged, unstaged or untracked work. The first tagged release, 0.1.1, advanced the
initial 0.1.0 implementation with compatible delivery tooling and packaging fixes.
The changelog records the initial implementation separately from those additions.

The shared source checker reports every refused path, including unusual path
bytes, without changing source or the index. Release metadata permits only
`Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`; publication permits no exceptions.
An initial release-preflight refusal reports that validation and version
preparation have not started for that attempt. Failed Git observations are
reported separately from a dirty checkout.

| Command | Effect |
| --- | --- |
| `make version` | Print the current local workspace version. |
| `make package` | Build and verify the crate tarball with all features, locked and offline. |
| `make publish-check` | Verify a crates.io publication dry run with a locked graph and registry access; no upload. |
| `make release-patch` | Validate and deliver the next patch version. |
| `make release-minor` | Validate and deliver the next minor version. |
| `make release-major` | Validate and deliver the next major version. |
| `make release-resume VERSION=X.Y.Z` | Reconcile only the specified retained release. |
| `make publish` | Upload the clean, tagged `ic-jobs` package to crates.io. |

Package and dry-run checks intentionally permit working edits. The real upload
does not. Package verification is included in configured CI and the complete
release gate. The core and consumer-owned Timers paths are checked separately on Rust
1.91; packaging with the development toolchain does not prove that floor.

## Release recovery

Select one release command whose candidate matches the pending changelog.
An explicitly authorized standard release runs the complete `ci` gate and
retains failed validation logs under `.git/release-state/validation-failures/`.
It updates only `Cargo.toml`, local workspace rows in `Cargo.lock` and
`CHANGELOG.md`, then commits, creates the annotated tag and atomically pushes
the selected branch and that tag. Dependencies are not upgraded.

If validation or preflight fails, correct the source and rerun the normal release
command. At unchanged HEAD, interrupted preparation resumes its exact saved
release. A normal direct-release command can instead restart preparation after a
newer clean descendant commit records its metadata disposition, provided the
saved attempt has no staging evidence and its destination still agrees. The
runner archives the original plan, computes the requested increment from current
metadata and repeats preflight and complete validation. `release-resume` remains
exact-source recovery; staging and later phases retain exact effect reconciliation.
See the [shared restart contract](releases.md#source-changes-after-saved-preparation-intent).

Jobs' preparation adapter changes only `Cargo.toml`, local workspace versions in
`Cargo.lock` and `CHANGELOG.md`; it does not stage or perform external effects.
It copies originals to a temporary backup before mutation and restores them on
handled failure. That backup is not a durable transaction journal: an abrupt kill
can leave partial metadata, and successful preparation removes its own backup.
Inspect any retained backup and reconcile all three metadata files before
committing a disposition for restart. A clean commit must represent the intended
current metadata, not merely silence a dirty-tree check. Preserve logs, plans and
build artifacts, and resolve conflicting remote observations before retrying;
do not force-push or recreate tags.

## crates.io publication

Set up Cargo's crates.io credentials explicitly, using `cargo login` or the
supported `CARGO_REGISTRY_TOKEN` environment variable. Credentials must have
permission to publish `ic-jobs`; GitHub authentication alone is insufficient.

`make publish` checks staged, unstaged and untracked source, requires the local
workspace version's annotated tag to point to HEAD, and verifies that the same
tag object exists at the sole push URL selected from `RELEASE_REMOTE` (default
`origin`). It observes the exact version at crates.io: an existing version or
an unavailable observation blocks a new upload. Only a confirmed absent version
permits Cargo's verified upload, with a locked graph and all features.

An interrupted upload must be reconciled against crates.io before retrying.
An existing version blocks repetition; crates.io versions are immutable. An
unavailable registry response blocks publication. The command does not bump,
commit, tag, push, publish another package or delete artifacts.

## Qualification

Jobs' publication wrapper delegates read-only admission to the snapshot-owned
`scripts/ci/check-publication.sh`; Jobs retains the package, features and Cargo
upload command. This delegation does not authorize publication.

`make test-release-tooling` uses real Git repositories and local bare remotes
to exercise patch/minor/major preparation, exact release commits and annotated
tags, failure before version mutation, and completed-release recovery. Publication
checks cover source admission, local/remote tag identity, Make execution modes,
registry availability and upload failure. The complete gate and registry/upload
effects are substituted; the fixture performs no live release or publication.
One common installer substitute retains Jobs catalog/root routing. Shared owns
exhaustive common-leaf and selected-source metadata cases; Jobs keeps late
parallel admission, setup/check refusal and actual adapter propagation checks
inside its existing patch-release fixture.
Jobs keeps eleven representative outer-entrypoint cases spanning release,
formatting and publication, including inherited, cleared/replaced `MAKEFLAGS`
and erased `MFLAGS`. Each checks refusal before fetch, gate, source/index or
release-state effects. Shared Tooling owns the exhaustive flag/control matrix
in its release-command and formatting-Make fixtures. Harmless external-root selection and parallel
argument-bearing recursive Make preserve quoted caller selections.
The fetch substitute also exercises cold-cache preparation, explicit offline
refusal, fetch failure and prepared-cache reuse, verifying unchanged metadata
and no gate/version effects on preparation failure. Dirty source is refused
before any fetch. Actual registry cache preparation is separate evidence.
Unsupported PR delivery is refused at both the Make and metadata-adapter
boundaries before fetch, gate or source mutation; the PR transport is omitted
from Jobs' snapshot selection.

Release 0.13.0 passed its configured gate on Linux and both native macOS
architectures; see the [current handoff](status/current.md) for exact source and
run identity. Later changes need their own qualification. Linux fixture passes
and Bash 3.2 execution on Linux do not establish native macOS evidence;
live IC recovery remains pending.


## Publication admission ownership

`scripts/release/check-publication.sh REMOTE` owns Jobs' read-only publication
policy for `ic-jobs`. It requires clean source, a stable workspace version, one
push destination, the matching pushed annotated tag and confirmed crates.io
absence. It rechecks source and release identity after network observations;
failed or uncertain observations refuse upload. `publish.sh` invokes this policy
before Cargo. Upload authorization and interrupted-upload reconciliation remain
with the maintainer.

Shared source, version, tag, registry and Make observers remain canonical.
The reviewed snapshot's complete publication checker is unused; retire that entry
through a future reviewed export, preserving current snapshot bytes and receipt.
`make test-release-tooling` selects the local publication fixture. It covers
policy ordering, failed observations, source/tag races and real-Git admission,
with registry transport substituted and no upload. Its exit and selected-assertion
failure retention use the standard Shared probes.
