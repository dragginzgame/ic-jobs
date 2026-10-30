![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# 0.4.1 code-hygiene audit — 2026-08-15

## Summary

The 0.4.0 public API, snapshot schema, provider boundary, and two-message
Watchdog protocol remain unchanged. This patch audit found one lifetime bug
and several private representation and duplication points; all are corrected
without a public compatibility break.

## Transient cancellation

`DeclarationLifetime::RemoveWhenStopped` promises removal after cancellation.
Scheduled and running declarations already honored that rule, but a fresh
inactive registration returned a no-op transition and remained in the bounded
inventory indefinitely. The ordinary and Watchdog cancellation paths now
derive removal from the lifetime plus canonical stopped state, including a
declaration that never owned a provider handle.

Pure-registry and live-runtime tests cover Once, after-completion, and Watchdog
registrations. They prove cancellation succeeds, no provider handle remains,
the inventory entry disappears, capacity is released, and the sole-owner
registration claim reports `RegistrationExpired` afterward.

## Private DRY cleanup

Explicit Once ensure and ordinary reconciliation duplicated schedule
resolution and scheduling-mode projection. `PendingSchedule::resolve` now owns
that translation once. Recurring ensure now matches its policy directly rather
than constructing an `ordinary` boolean and a tuple containing state that some
branches ignored.

The remaining ordinary request path combined a generic ensure/reconcile enum
with a separate `require_once` boolean and a pass-through scheduling helper.
One closed request kind now distinguishes Once ensure, after-completion ensure,
and authoritative reconciliation, so invalid combinations are not
representable.

The three retained lifecycle helpers duplicated fresh registration, exact
claim extraction, immutable declaration verification, and impossible missing-
slot handling. One generic private reconciliation seam now owns those
invariants while retaining the distinct public policy APIs and callback
types.

Cancellation and explicit unregistration separately detached the exact
provider handles, invoked a transition, and then restored or retired the
claim. One private detached-transition seam now owns that sequence. The
ordinary reconciliation path retains its policy validation before detachment,
so a wrong-policy request cannot disturb live ownership.

Watchdog cancellation previously returned an opaque boolean beside its
transition. The caller used it partly for metrics and partly for removal, even
though canonical state already answers both questions more precisely. The
transition helper now returns only its transition; immediate cancellation is
identified from pre-transition state for accounting, and stopped/removable
state is read after the transition.

## Provider-effect cohesion

The provider helpers accepted selected token and delay fields beside a
reference to the complete effect, even though confirmation consumed the effect
again. That duplicated authority and made mismatched arguments representable
inside the private runtime. Each helper now destructures the one complete
effect it binds.

Effect-shape validation runs before cleanup, platform calls, or counter
confirmation. An ordinary arm must carry an ordinary-work token; an initial
Watchdog arm must carry a scheduler token and cannot be a replacement; and a
Watchdog dispatch must pair scheduler/work roles from the exact same identity
and claim generation. Registry and runtime negative tests mix two independent
Watchdog claims and prove rejection before provider arms or counter mutation.

Initial/replacement arm intent now remains one closed value from ordinary
control through provider binding instead of being split into duplicate action
variants and reconstructed later. The non-empty provider callbacks selected
for clearing are another closed private enum rather than independent booleans.
This removes the representable no-op clear shape and makes call sites state
which owned capabilities they intend to replace or clear.

## Exact handle consumption

Provider-handle consumption looked up the canonical entry by identity and
matched callback generation plus role, but omitted the registration claim
generation. After an identity was removed and registered again, a late old
callback could therefore detach the new claim's handle when both registrations
used the same first callback generation. The stale callback was rejected
later, but cancellation could no longer reach the detached replacement handle.

One entry-local exact-claim predicate now owns claim-generation validation for
callback acceptance, measurements, provider installation, running-context
validation, and handle consumption. A live runtime regression reuses an
identity and proves the old token cannot detach the replacement's armed handle;
the replacement remains observable and cancellation clears the real provider
timer.

No compatibility alias, dual runtime, fallback registry, provider re-export,
or alternate control path was introduced. Compatibility-only exhaustiveness
markers were removed from private control errors rather than preserved as a
pre-1.0 shim.

## Comment truth

The public control comments now distinguish an armed provider wake-up from a
successor request retained while ordinary work is running. Cancellation and
unregistration explicitly state that they do not interrupt consumer work;
nested removal completes through the existing normal-completion arbitration.

## Evidence

The normal CI gate passes with 83 native tests, warning-denied Clippy and
rustdoc, Wasm compilation, and offline packaging. Rust 1.88 checks the
complete workspace and every nested runtime and size-probe configuration. The
dependency tree has no duplicates, both 0.4.1 finalizer checks pass, release
impact is `crate`, and diff whitespace validation passes.

PocketIC is not required for this slice because fresh inactive cancellation
owns no provider handle, malformed effects are rejected before provider calls,
and the stale-token fix only tightens which exact claim may detach an existing
handle. No valid provider call or Watchdog protocol transition changed. The
maintainer-owned version bump retains the full release gate.
