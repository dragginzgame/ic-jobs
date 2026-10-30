![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Toko Miner adoption evidence

This page preserves dated adoption evidence, not current dependency or issue
status. Current integration guidance is in [runtime architecture](../architecture.md#consumer-integration)
and [the public API](../../../crates/ic-timers/README.md#runtime-ownership). Consumer owners qualify
their selected graphs and deployed artifacts; historical receipts below qualify
only their named sources. See [the release guide](../development.md)
for this repository's host-evidence limits.

Historical status on 2026-10-03: the inspected dirty graph selected two timer
package identities and could not qualify coordinated adoption. Earlier single-package composition receipts remain historical.

## Current read-only dependency inspection — 2026-10-03

Toko Miner HEAD `dba00bbf3cb8faa6fa007ad23bdf5a67209a6c34` has a dirty lockfile
with SHA-256 `b6b9b99ed412ed8d580bd68e6514fa925bafd9ed58ee3263a50137115423e90a`.
It selects both `ic-timers` 0.8.1 and 0.9.2. Game Shard, Translation and
registry Canic Core 0.110.51 use 0.8.1; IcyDB 0.264.6 uses 0.9.2. The application
manifest still requires timer `0.8.1`. Cached registry manifests confirm Canic
Core's exact `=0.8.1` requirement and IcyDB's `0.9` requirement.

This graph cannot establish one shared timer registry. The earlier aligned
0.8.1 receipts do not qualify it. No fresh resolver, build, timer inventory or
IC scenario was run; this is inspected lock/manifests, not deployment evidence.
[Canic #33](https://github.com/dragginzgame/canic/issues/33) was the recorded tracker for a
published coherent graph and composed qualification. Current local Canic uses
timer `0.9`, but its dirty worktree is not that published Core package.

`apps/toko_miner/game_shard/src/action/checkpoint_timer.rs` still owns a retained
Once deadline (`DEADLINE`) that wakes a retained Watchdog (`WORK`). The callback
alone processes bounded checkpoint/recovery work. `active.rs` refreshes the
deadline after replicated journal/cache changes; failed work uses the pre-armed
retry, successful due debt drains immediately, future debt sleeps and quiescence
stops. Any consolidation must preserve those distinctions, synchronous lifecycle
reconstruction and the rollback boundary. Removing the auxiliary deadline is
application-owned work, after package alignment, with bounded composed evidence.
The two claims are not proof of two providers; the two package identities above
are the separate shared-registry problem.

No sibling file was edited. Concurrent downstream changes were preserved.

## Earlier published runtime and dependency inspection — 2026-10-03

Read-only inspection on 2026-10-03 finds Toko Miner HEAD
`3354dfc6b9fe791884ec69e8dd344de313b36940` with registry Canic 0.110.51,
IcyDB 0.264.4 and one `ic-timers` package at 0.8.1. The inspected Cargo.lock
SHA-256 is `c51b916a0ae31b82e575558df5facd3352c213487a926b961ff1e9d061277b22`.
This identifies the inspected worktree dependency selection, not a frozen
deployed artifact; dirty downstream state was preserved.

The cached registry `ic-timers` 0.8.1 package's `.cargo_vcs_info.json`
identifies release commit `c27f54ae64d1c667e9563925a9d7e4791d9359e3`.
The selected registry checksum is
`3e39ee0103132c90c69818c422ba5aea8f68f5eab96f543cf85bfd4175554405`.
This corroborates publication, consistent with the
[IcyDB consumer report](https://github.com/dragginzgame/ic-timers/issues/6#issuecomment-5948345356).
The later resolver-3 commit `f7a4f27cc786afff1df2098f09c21aef0c291642`
is separate from that published package.

## Owner-reported adoption — 2026-10-02

Toko Miner's `docs/upstream/ic-timers.md` and the Canic 0.110.50 adoption entry
in `docs/upstream/scan-log.md` record an aligned graph of Canic 0.110.50,
IcyDB 0.264.3 and one timer 0.8.1. The former Canic 0.110.49 exact-pin blocker
is resolved in that graph. The owner reports native and application CI gates
passing, with eight managed artifacts and seven bounded scenarios qualifying
composed startup and same-release timer/database recovery.

The later Canic 0.110.51 adoption entry in the same scan log records seven
bounded scenarios against its exact artifacts with IcyDB 0.264.3. It also retains
an unrelated browser failure; full application CI is not a pass for that subject.
These owner-reported results do not qualify the subsequently inspected IcyDB
0.264.4 selection, prove current deployment or establish a matched cost saving.
This repository did not rerun any downstream scenario or change any sibling.

## Earlier dependency inspection — 2026-10-02

The read-only Toko Miner worktree at
`5e7f675dddd5bf2126e5d3068c2fe75776e4a2a9` selects registry `ic-timers`
0.8.0, Canic 0.110.49 and IcyDB 0.262.2. Its inspected lockfile contains
one `ic-timers` package. The downstream owner reports `make timer-check`
passing for Game Shard and Translation; this repository did not rerun it.

The cached registry `ic-timers` 0.8.0 package's `.cargo_vcs_info.json`
identifies commit `fba369df1a31b2c37ce31f4f775f816a7c9de1f9`, matching the
local release tag. The registry lock checksum is
`c1474fd7c9bcc237404173c2689c4175e61ef2c9670bccf539ca10ccd193d132`.
These artifacts corroborate publication of 0.8.0 for that earlier subject.
The subsequent 0.8.1 publication and adoption evidence is recorded above.

Inspection provenance: Toko Miner's `docs/upstream/ic-timers.md` and the
2026-10-02 local ic-timers audit in `docs/upstream/scan-log.md`. The inspected
Cargo.lock SHA-256 is
`d49922b78bcf5959bd209b76d8db860d5d1b8b17d12c7898789f3c61b520bc72`.
Dirty downstream state was retained; this is not a deployed-artifact identity.

## Retained composition receipt — 2026-09-20

The downstream owner's
[Canic 0.110.33 adoption receipt](https://github.com/dragginzgame/toko-miner/blob/5e7f675dddd5bf2126e5d3068c2fe75776e4a2a9/docs/upstream/artifacts/canic-0.110.33-adoption-2026-09-20.json)
identifies application HEAD `ada2c2c634b98692c5b70333e718e7ef9b1f8826`,
Canic 0.110.33 at `914a0507483874506538f5b25b4c6ebce22bc389` and IcyDB
0.261.0 at `c1a5028eebed7b2f04cce00d49a4ea6b9c58512c`.

Every application role resolves one `ic-timers` 0.8.0 package. Two managed
scenarios record shared gameplay timer inventory, two idle windows without
checkpoint callbacks, wake on accepted work, same-release state and timer
recovery, and timer registration-identity/saturation metadata. The old
lifecycle-composition blocker is resolved within that recorded scope.

The receipt qualifies a frozen snapshot that excluded concurrent gameplay
edits. It also records unrelated full-CI, browser and lint failures. It does
not establish current staging deployment, live cost savings or qualification
of all later Canic/IcyDB/application combinations. The receipt SHA-256 is
`2328efc4b1a4f351391618ccfbc2d35e0960537bb562cadaced24c084f2c08ea`.
This repository inspected the owner-supplied evidence without rerunning its
managed scenarios or editing any sibling repository.

## Application work identified on 2026-10-03

- The checkpoint timer still uses an auxiliary `OnceRegistration` to wake its
  Watchdog. The released exact-deadline API supports removing that hand-off;
  Toko Miner owns the scheduling change and its managed proof.
- Game Shard's diagnostic `TimerMetric` omits registration sequence. The
  released Canic counter projection already carries epoch and sequence;
  `TimerSnapshot::registration_id()` supplies the canonical identity.
  Callback generation alone cannot qualify counter subtraction.
- Current deployment, complete transfer coverage and controlled live cost
  comparisons remain unmeasured here. No scheduler defect or cycle saving is
  established by this review.
