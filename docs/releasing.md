# IC Jobs release and publication

IC Jobs uses the reviewed [Shared Tooling release contract](releases.md) with
consumer-owned Cargo metadata and validation. IC Jobs selects direct release
delivery only: remote `origin`, branch `main`, annotated tag `vX.Y.Z`. All three
SemVer commands use the same runner and complete gate. They do not publish to
crates.io.

## Preparation

Install the required host and Rust tools with `make install-tools`. The
consumer's additional `make install-release-tools` installs pinned cargo-edit
0.13.13 with its set-version feature; `make release-tools-check` checks the
selected executable without downloads. Prepare Rust 1.88.0 and its Wasm target
as described in the README, alongside the development toolchain.

Shared Make entrypoints require recipe execution and failure propagation.
Remove `-i`, `--ignore-errors`, `-n`, `-t`, `-q` and inherited equivalents;
these modes are refused before recipes run, including formatting commands.
Jobs also checks invocation modes retained in `MFLAGS` when command-line
`MAKEFLAGS` is cleared or replaced. This consumer-owned admission supplements
the shared probe until [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)
covers that case; the adopted snapshot stays unchanged.

Before ordinary offline checks, prepare the selected dependency graph explicitly
with `cargo fetch --locked`. Standard release commands include that locked fetch
in preflight, after source/candidate admission and before offline validation or
metadata writes; no separate manual fetch is required. This preparation may use
the registry but does not upgrade dependencies or rewrite the lockfile. Cargo's
explicit offline environment/configuration remains authoritative; missing inputs
then stop the release with its original fetch failure. Version reads, metadata
checks and the validation gate remain offline. Commit the intended source through
the contribution
workflow; release commands require an existing source commit and refuse unrelated
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
release gate. The core and optional timer paths are checked separately on Rust
1.88; packaging with the development toolchain does not prove that floor.

## Release recovery

Select one release command whose candidate matches the pending changelog.
An explicitly authorized standard release runs the complete `ci` gate and
retains failed validation logs under `.git/release-state/validation-failures/`.
It updates only `Cargo.toml`, local workspace rows in `Cargo.lock` and
`CHANGELOG.md`, then commits, creates the annotated tag and atomically pushes
the selected branch and that tag. Dependencies are not upgraded.

If validation or preflight fails, correct the source and rerun the normal release
command. After preparation starts, the runner retains exact release intent and
reconciles it on the next normal invocation. Use `release-resume` to select only
that saved release. Preserve logs, plans and build artifacts, and resolve a
conflicting remote observation before retrying; do not force-push or recreate tags.

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

`make test-release-tooling` uses real Git repositories and local bare remotes
to exercise patch/minor/major preparation, exact release commits and annotated
tags, failure before version mutation, and completed-release recovery. Publication
checks cover source admission, local/remote tag identity, Make execution modes,
registry availability and upload failure. The complete gate and registry/upload
effects are substituted; the fixture performs no live release or publication.
Direct and inherited Make modes are checked at the outer release and formatting
entrypoints, including cleared/replaced `MAKEFLAGS`, verifying refusal before
fetch, gate or source changes. Harmless external-root selection and parallel
argument-bearing recursive Make preserve quoted caller selections.
The fetch substitute also exercises cold-cache preparation, explicit offline
refusal, fetch failure and prepared-cache reuse, verifying unchanged metadata
and no gate/version effects on preparation failure. Dirty source is refused
before any fetch. Actual registry cache preparation is separate evidence.
Unsupported PR delivery is refused at both the Make and metadata-adapter
boundaries before fetch, gate or source mutation; the PR transport is omitted
from Jobs' snapshot selection.

Release 0.2.1 passed its configured gate on Linux and both native macOS
architectures; see the [current handoff](status/current.md) for exact source and
run identity. Later changes need their own qualification. Linux fixture passes
and Bash 3.2 execution on Linux do not establish native macOS evidence;
live IC recovery remains pending.
