![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Code-hygiene audit — 2026-08-14

## Summary

The post-0.3.2 audit found an authority-lifetime defect, a public
provider-effect disposition defect, and a release-helper defect. All are fixed
under the open 0.3.3 line. The provider protocol, watchdog message
boundary, snapshot shape, and downstream repositories remain unchanged.

## Callback-authority finding

`TimerContext` was documented as delegated authority for the executing work,
but it stored only the identity and registration claim generation. That claim
generation intentionally lives until unregistration, so consumer code could
retain a context after callback completion and use it to cancel or reschedule a
later callback generation. This created an unbounded alternate custody path
outside the policy-specific registration capabilities and could defeat an
owner's authority-snapshot quiescence.

The context now carries the exact private `CallbackToken`. Every nested ensure,
reconcile, or cancellation validates, inside the registry borrow:

- the canonical identity and registration claim generation;
- the exact callback generation;
- the ordinary-work or watchdog-work role; and
- the policy-specific running state for that attempt.

Nested mutation during consumer work remains supported. Once completion
commits, the same context returns `TimerError::RegistrationExpired`; its
identity accessor remains inert. Ordinary and watchdog regression tests retain
a context across completion, attempt multiple mutations, prove the scheduled
successor is unchanged, and execute that successor normally.

## Provider-effect disposition finding

The pure registry moves to its requested scheduled state before the runtime
binds the emitted effect to `ic-cdk-timers`. Callback-originated binding errors
already failed the registration or trapped watchdog work. The equivalent
public registration and lifecycle path returned its typed error after clearing
the rejected provider handle, but did not retire the registry state. An
unexpected handle-installation or confirmation failure could therefore leave
a retained declaration observably scheduled with no provider callback; later
idempotent ensure requests would coalesce against that dead state.

All claim-originated transitions now use one completion wrapper. On an
unexpected effect-application error it calls the existing canonical
`fail_registration` transition, clears every handle still owned by the entry,
and records `Inactive(ControlFailure(ProviderBindingFailed))` for retained
declarations. The original registration claim remains usable for an explicit
retry. A fault-injection test starts with a committed arm, forces replacement
installation to fail, proves both provider arms are absent and no false
generation is exposed, checks requested-versus-armed counters, and then
successfully schedules and executes a later retry.

## Release-helper finding

The exact-version release path accepted any three numeric components other
than the current version or an existing tag. A request could therefore move
the workspace backward, and numeric components with leading zeroes were
accepted even though they are not canonical SemVer.

The helper now compares all three components without integer-overflow-prone
numeric conversion, requires a strict increase, rejects leading zeroes, and
checks `refs/tags/vX.Y.Z` exactly. Its release-gate test exercises downgrade and
invalid-version rejection before changelog validation, expensive evidence, or
version mutation.

## Additional review

The adjacent registry lookup paths now share one exact-running-work validator,
removing duplicated ordinary/watchdog token checks. The maintained 0.3
contract also had one stale Canic metric sentence; it now maps the legacy
schedule count to committed `wakeups_armed`, consistently with the 0.3.2
adoption contract. The review found no new direct `ic-cdk-timers` path, public
provider re-export, detached provider-handle owner, second pending-command
machine, unsafe code, or production `unwrap`/`expect`/`todo`/`unimplemented`
path.

## Evidence

- 65 native library tests pass, including both context-expiry regressions and
  the provider-binding replacement fault.
- Release-gate syntax, downgrade rejection, leading-zero rejection, and wiring
  checks pass.
- `cargo check --workspace --all-targets --all-features --locked` passes.
- Normal CI passes, including strict Clippy/rustdoc, Wasm compilation, offline
  packaging, provider-boundary enforcement, and all 65 native tests.
- Rust 1.88 workspace compilation and every supported nested-probe lint
  configuration pass.
- The dependency tree contains no duplicate versions, and `cargo audit`
  reports no vulnerability across 13 locked dependencies and 1,216 current
  RustSec advisories.
- ShellCheck and final diff-whitespace validation pass.

The PocketIC matrix is not required for these provider-neutral authority
checks; it remains mandatory in the release gate.
