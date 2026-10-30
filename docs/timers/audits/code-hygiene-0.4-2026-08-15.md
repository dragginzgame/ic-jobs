![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# 0.4 code-hygiene audit — 2026-08-15

## Summary

The 0.3 runtime has one registry, one provider boundary, and no deprecated,
feature-gated, fallback, or alternate timer implementation to preserve. The
0.4 hard cut therefore removes accidental public value paths and redundant
adapters without changing normal registry transitions or the two-message
watchdog protocol. The audit does tighten unreachable and terminal failure
paths where doing so preserves the same fail-closed contract.

## Public facade

The crate root is now the only supported import path. `schedule` and
`snapshot` remain useful source-code ownership boundaries, but exposing their
contents both through those modules and through the crate root created two
public paths for every retained value. Both modules are private in 0.4; their
supported types remain root re-exports.

`TimerLabel` was an independently public intermediate value even though the
runtime accepts only a complete `TimerIdentity`. It duplicated construction,
error, conversion, dereference, and display surface. Identity now stores its
three bounded components directly, validates them through
`TimerIdentity::try_new`, returns them as `&str`, and reports the failed field
and reason through one `TimerIdentityError` enum. The accurately named public
bound is `MAX_TIMER_IDENTITY_COMPONENT_BYTES`.

This is intentionally a 0.4 minor-line break. No aliases, deprecated
forwarders, old constants, dual error shapes, or compatibility modules remain.

## Snapshot authority and DRY

Snapshots remain inert observations. The reverse conversion from
`TimerDirectiveSnapshot` to the executable `TimerDirective` is removed; the
runtime still owns the one-way checked projection from commands to snapshots.

The canonical expected-failure streak remains on `TimerOutcomeSnapshot`.
Forwarders on `TimerObservabilitySnapshot` and `TimerSnapshot` added two more
paths without different semantics, so they are removed. The identity-scoped
`consecutive_expected_failures` runtime query remains because consumers use it
as cheap functional state without constructing a complete inventory.

Provider-handle cleanup had two copies of the same wake-up/work drain. One
private helper now owns that sequence. Entry-local helpers also own callback
role-to-slot selection and exact handle detachment, so install, consume,
failure, and cleanup paths cannot drift between wake-up and work roles.

The deeper pass found one failure-path bug in restoration: when both watchdog
handles were temporarily detached, an early return after the first failed
reinstall skipped processing the second capability. Restoration now attempts
both handles, clearing any handle that cannot be rebound, and returns the first
failure only after both linear capabilities have been consumed. A focused
fault-injection test proves that the failed wake-up is cleared while the work
handle is restored and remains registry-owned.

The watchdog replacement path also used eager `Option::or` evaluation. It
looked up and detached the registry-owned work handle even if the caller had
already supplied that exact detached capability. One lazy selection helper now
serves arm, clear, and watchdog replacement paths. A borrow-guard test proves
the detached case performs no redundant registry access.

The platform handle and both detached handle types are now `must_use`, so
future internal code receives a compiler warning if it ignores the obligation
to bind, restore, or clear a linear provider capability.

Three synchronous public paths detached exact provider handles before asking
the registry to cancel, unregister, or reconcile inactive. Their post-detach
registry failures are unreachable after normal claim validation, but using
`?` there made future invariant regressions drop the detached handles. One
completion helper now restores every capability before returning the original
error and retires the exact claim if restoration cannot re-establish
ownership. A focused fault test injects the internal transition error and
proves the wake-up remains armed and registry-owned.

Lifecycle verification previously rebuilt a complete snapshot—including a
cloned boxed identity and all observations—to check immutable policy and
lifetime. It now validates the exact retained claim directly in the registry.
This keeps snapshots observation-only and rejects a stale claim without an
unnecessary snapshot allocation.

Pure ordinary control also duplicated the full arm/replace mutation for
earliest-deadline scheduling and authoritative exact reconciliation. One
private typed deadline-selection rule now chooses whether the shared mutation
replaces a scheduled generation. Request-sequence and generation exhaustion
remain failure-atomic and retain their focused state-machine tests.

The final watchdog audit found repeated terminal branches that set inactive
state but did not all clear a pending nested command. One private transition
now owns terminal failure cleanup. Scheduler and work generations are checked
as an atomic pair before either changes, and focused pure tests cover
scheduler-generation, work-generation, deadline, and nested-request overflow.
The normal pre-arm/dispatch protocol is unchanged.

The public scheduling error included `MissingCadence`, even though public
cadences are validated at registration and no public schedule request could
produce it. That condition exists only when internal code combines
`RecurAfterCompletion` with a policy that has no cadence. It now uses a private
directive-resolution error and still maps to the same terminal control
failure; the unreachable public variant is removed for the 0.4 hard cut.

Private unregistration previously consumed a claim and forced the runtime to
clone its three bounded identity components solely for cleanup. The registry
now borrows the exact claim while the public sole-owner registration remains
consuming. Delegated callback claims are likewise derived from one complete
callback token rather than separately supplied identity and generation
arguments.

Public observation fragments no longer implement `Default`: the registry is
their only coherent constructor. The public `cadence_ns`, combined
`provider_arms`, completion-invariant checker, nested directive-mode helper,
and nested state-deadline helper were redundant with typed cadence, role-split
counters, owner-local tests, and top-level `TimerSnapshot` projections, so the
0.4 hard cut removes them.

Inactive-state comments now say that no callback *generation* is scheduled or
running. A retained declaration can still own callback authority, so the
documentation no longer describes the declaration itself as absent.

## Module hierarchy decision

The private `control` state machine remains separate from `registry`. It owns
ordinary generation and request-sequence transitions, while the registry owns
the sole pending-command arbitration and all canonical timer entries. Folding
the two would move rollback-sensitive code without eliminating a second
authority, because no second authority exists.

The large `registry` and `runtime` modules also remain cohesive: their long
paths correspond to atomic state transitions and provider-effect binding.
Their tests already live in adjacent `tests.rs` files.

No blanket lint suppression remains. The nine narrow `allow` attributes across
the library and real-canister fixture each carry an owner-local rationale:
linear ownership consumption, canister-local futures, atomic audited
transitions/fixtures, or the registry-only snapshot constructor.

## Migration

- Import retained values from `ic_timers::*`, not
  `ic_timers::schedule::*` or `ic_timers::snapshot::*`.
- Construct identities only with `TimerIdentity::try_new`.
- Use `identity.owner()`, `identity.subsystem()`, and `identity.name()` as
  `&str`; remove a trailing `.as_str()` used for the old label wrapper.
- Match the field-specific `TimerIdentityError` variants directly.
- Replace `MAX_TIMER_LABEL_BYTES` with
  `MAX_TIMER_IDENTITY_COMPONENT_BYTES`.
- Read the expected-failure streak from
  `snapshot.observability().outcomes().consecutive_expected_failures()` or use
  the identity-scoped runtime query.
- Treat `TimerDirectiveSnapshot` as observation only. Build a new
  `TimerDirective` explicitly when scheduling work.
- Replace `policy.cadence_ns()` with
  `policy.cadence().map(TimerCadence::as_nanos)`.
- Keep `wakeups_armed` and `work_dispatched` separate unless an adapter
  deliberately needs their sum.
- Read scheduling mode and next deadline from `TimerSnapshot`, and obtain all
  counter/outcome/performance fragments from canonical snapshots rather than
  constructing defaults.
- Remove any `ScheduleError::MissingCadence` match arm. It was never returned
  by the public API; illegal cadence-free recurrence remains internal.

Maintained Canic and IcyDB sources already use root imports and
`TimerIdentity::try_new`. The inspected Canic worktree has two label-wrapper
`.as_str()` calls that require the mechanical getter migration above. No
downstream repository was changed or requalified by this audit.

## Evidence

The normal local CI gate passes: release and provider checks, formatting,
workspace check, warning-denied Clippy and rustdoc, all 78 native tests, Wasm
compilation, and offline package build/verification. Rust 1.88 compiles every
workspace target, and every supported nested probe configuration passes
warning-denied Clippy on Rust 1.88.

The dependency tree has no duplicates. The package contains the intended 18
files. Fresh generated rustdoc exposes only the crate-root facade and none of
the removed symbols. Panic-like source matches are confined to test fixtures
and the test-only platform fake. Both 0.4 finalizer checks, release-impact
classification, and diff whitespace validation pass.

The 78 native tests include focused coverage of detached two-handle
restoration, synchronous post-detach error restoration, and every watchdog
terminal overflow boundary. PocketIC is not repeated because normal timer
transitions, provider calls, and the scheduler/work protocol did not change.
The maintainer-owned release gate retains the full real-canister evidence
requirement when the package version is advanced.
