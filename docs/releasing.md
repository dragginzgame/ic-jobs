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
Standard `make release-patch`, `make release-minor` and `make release-major`
prepare the lock-selected IC Build editor during fresh preflight, after source
and candidate admission. The adapter captures its offline-admitted absolute path
for each preparation process; no shell exports are required.

For standalone offline qualification with `make test-release-tooling`, prepare
the editor explicitly:

```sh
make install-release-tools
```

Build and Testkit 0.63 require their `cli` feature for executable tools. Jobs'
Make adapters and live qualification wrapper select `--features cli` explicitly;
library dependencies keep it disabled. Prepare the selected Testkit executable
with `make install-testkit-tools`, then admit both tools with
`make release-tools-check testkit-tools-check`.

The editor renders candidates without release effects; Shared owns admission,
independent checks and restoration. The 1 MiB per-document allowance covers
Jobs' manifests, workspace context and lockfile. A failed installation or offline
admission stops before validation or metadata writes, even if it printed a path.
Common tool setup does not implicitly install this optional release editor. CI
prepares it explicitly; the release fixture selects the same admitted executable
and allowance. `make release-tools-check` prints the admitted path without setup.

Once preparation intent is saved, recovery selects the editor from the admitted
preparation source's committed lock, never partially written live metadata.
Recovery performs no installation or dependency fetch. If that saved selection
is missing, restore its prepared installation before resuming. Prepared and
committed metadata checks do not render candidates and need no editor.

Shared's default formatter preserves dependency-role groups with
`cargo sort --workspace --grouped`. The metadata adapter selects
`--manifest-order-grouped` for prepared, commit-preparation and committed-source
checks. The release fixture verifies unsorted-group refusal and selection of the
recorded commit even when the working manifest has different ordering. These
ordering checks do not enforce transitive production dependency exclusions; see
the [dependency exposure map](../README.md#dependency-exposure).
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
`make install-tools`, `make tools-check` and `make install-release-tools`, then
admits the editor offline; setup reuses
the selected installation and does not replace older roots. Explicit
`release-resume` selects saved intent before dispatch and bypasses fresh setup.
Ordinary `make ci` never installs:
it runs `validation-tools-check`, then the complete `clippy` gate before
dispatching builds, tests and packaging, including under parallel Make. That
gate retains core all-target/all-feature, native consumer/harness all-target,
and Wasm consumer linting with warnings denied. `make test-release-tooling` admits its formatter
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

The runner requires clean committed source before fresh release preflight. The
standalone metadata adapter's shared source checker reports every refused path,
including unusual path bytes, without changing source or the index. It permits only
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

If validation or preflight fails, correct and commit the source, then rerun the
normal release command. Normal commands always start fresh from current committed
metadata and repeat preflight and complete validation. Historical plans, including
malformed or undelivered attempts, are not read or replayed; missing older tags
do not block the next release. Stale receipts for the newly selected candidate
are discarded under the release lock. Build and validation evidence is retained.

Use `make release-resume VERSION=X.Y.Z` to continue a particular saved attempt,
including interrupted preparation at unchanged HEAD. Recovery remains bound to
that attempt's source, payload and destination. Dirty source/index, occupied
locks and conflicting candidate refs still refuse. Uncommitted preparation needs
explicit recovery or a deliberate committed/restored disposition before a fresh
release; the runner does not reset files or stage them to make a release possible.
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

`make publish` captures HEAD, workspace version and tag object before admission,
then passes the same complete Clippy gate. It requires those identities to remain
unchanged and rechecks source cleanliness before uploading. A clean concurrent
commit or tag replacement therefore also blocks upload. `make publish-check` also
passes Clippy before its dry run. Missing offline dependencies require explicit
`cargo fetch --locked` preparation; neither lint gate updates the lockfile.

An interrupted upload must be reconciled against crates.io before retrying.
An existing version blocks repetition; crates.io versions are immutable. An
unavailable registry response blocks publication. The command does not bump,
commit, tag, push, publish another package or delete artifacts.

## Qualification

Jobs' publication wrapper invokes its local
`scripts/release/check-publication.sh`, which owns package and destination policy
while using Shared's source, version, tag, registry and Make observers. It
rechecks source and release identity after network observations. Failed or
uncertain observations refuse upload; validation never authorizes publication.

`make test-release-tooling` uses real Git repositories and local bare remotes
to exercise patch/minor/major preparation, exact release commits and annotated
tags, failure before version mutation, explicit completed-release recovery and
fresh releases despite undelivered or malformed historical plans. Archived
changelog Draft labels are preserved through the actual preparation path. Publication
checks cover source admission, local/remote tag identity, Make execution modes,
registry availability, release identity changes during lint and upload failure.
The complete gate and registry/upload
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
`ci/release-delivery.txt` is Jobs' single direct-only delivery declaration.
The shared checker admits it at both the Make and standalone metadata boundaries
before fetch, gate or source mutation. The validation checkpoint includes that
declaration, and Make cannot replace it through a caller's policy-path override.
The PR transport is omitted from Jobs' snapshot selection.

Release 0.13.0 passed its configured gate on Linux and both native macOS
architectures; see the [current handoff](status/current.md) for exact source and
run identity. Later changes need their own qualification. Linux fixture passes
and Bash 3.2 execution on Linux do not establish native macOS evidence;
live IC recovery remains pending.
