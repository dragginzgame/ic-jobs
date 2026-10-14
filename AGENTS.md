# IC Jobs agent instructions

Apply the reviewed [Dragginzgame baseline](DRAGGINZGAME.md) first.
Its exact revision is recorded in [.shared-tooling.snapshot](.shared-tooling.snapshot).
This file is the local overlay. Read [the current handoff](docs/status/current.md).

- Scope is this workspace; other sibling repositories remain read-only.
- Keep job metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- Use focused package checks under the validation cadence below; `make ci` is
  the full release gate. Release effects and live PocketIC qualification retain
  their separate explicit authority.
- Rust floor: 1.88 for both core and optional timers. Qualify both separately.
- Host workflows support Linux and macOS Intel/Apple Silicon. Bind native
  qualification to its tested source; IC recovery evidence remains pending and
  native tests do not establish it.
- Use the common contribution and release authority rules. The public remote is
  `dragginzgame/ic-jobs`; configuring tooling does not authorize releases or
  publication. Direct release delivery uses `origin`/`main`; `make publish` is
  a separate, explicitly authorized crates.io upload.

## Validation cadence during shared-policy adoption

Shared Tooling owns the [validation policy](DRAGGINZGAME.md#validation-and-evidence);
local guidance supplies commands and product qualification requirements only.
Maintainer-approved exception (2026-10-14): until this snapshot adopts Shared
Tooling's focused-handoff policy, replace its full-suite-before-handoff requirement
with focused checks of changed behavior and directly affected callers, including
relevant failure/recovery paths. Reuse unchanged passing evidence. Documentation
changes need link, consistency and diff checks only. Run local full suites only
for an explicitly authorized release or an explicit request for those checks;
implementation, continuation, PR preparation and readiness requests do not qualify.
Report tested scope; an unrun full suite alone is not a handoff blocker. Configured
CI, required branch checks, release gates and separate effect authority remain
unchanged. This exception avoids redundant local testing; it does not claim a
snapshot refresh. On reviewed adoption, remove this exception and retain only the
link to the canonical policy and repository-specific commands.
