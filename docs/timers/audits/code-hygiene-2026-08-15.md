![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Code-hygiene and module-hierarchy audit — 2026-08-15

## Summary

The runtime hierarchy is coherent and does not need a safety-sensitive module
split. The audit found one accidental public implementation alias, two small
duplications, and several comments that described retained lifetime or
provider delivery too broadly. The released 0.3.7 patch fixes those issues
without changing timer behavior.

## Module hierarchy

The crate root is the convenience facade. `schedule` and `snapshot` remain
public groupings for provider-neutral values, while `control`, `registry`, the
runtime implementation, and `platform` are private. Internal implementation
code now imports from those defining modules instead of depending on crate-root
re-exports intended for consumers. The registry module is declared private,
and the existing structural CI gate continues to keep every direct
`ic-cdk-timers` reference inside `platform`.

`registry/mod.rs` and `runtime/mod.rs` are large, but their production paths
are organized around atomic policy transitions and provider-effect binding.
Splitting those paths solely by line count would distribute rollback-sensitive
state and handle ownership across more boundaries. Their substantial tests are
already separated into directory-local `tests.rs` files. The audit therefore
keeps those cohesive implementation modules intact and documents the reason.

## Public surface and hard cut

`TimerFuture` exposed the concrete callback-erasure strategy even though all
ordinary registration functions accept a generic future and no maintained
consumer needs the alias. It is removed from the crate facade, and the erased
future now lives beside the private registry callback type. This is a direct
pre-1.0 hard cut: no deprecated alias or compatibility forwarding path is
retained.

Post-release review found that publishing this removal as 0.3.7 was a SemVer
mistake: Cargo-compatible 0.3 requirements may select that patch. The hard cut
remains sound, but a future public removal advances the minor line.

The review found no public provider type, provider function, provider module,
registry mutation path, compatibility shim, deprecated forwarder, or second
timer runtime.

Hosted workflow review found that main and tag push events repeated identical
Rust and MSRV jobs at the same release commit. Full validation now belongs to
pull requests and `main`. The tag-only job is deliberately small but still
rejects a tag whose name differs from the Cargo version or whose commit is not
reachable from `main`, then reruns release truth and exact-tag checks.

Free-form release prose remains outside the enforced post-mutation boundary.
The new pre-bump advisory detects likely stale target-version and next-action
wording, emits a warning, and exits successfully even when its input is
missing. Focused shell tests cover clean, candidate, next-action, and missing
input cases.

## DRY and comment corrections

- `register_once` and `register_after_completion` now share one internal
  ordinary-callback erasure helper.
- Snapshot directive conversion reuses the checked duration conversion owned
  by `schedule` instead of maintaining a duplicate implementation.
- Registration and callback-context rustdoc now distinguishes retained from
  remove-on-stop cancellation and reconciliation.
- Scheduling-mode comments state that the latest mode remains observable while
  inactive and does not prove a callback is armed.
- Provider comments consistently describe an armed owned handle rather than a
  guaranteed future delivery.
- Work-count and callback-result comments no longer claim bounds or legality
  that the value type itself does not enforce; registry validation remains the
  authority.
- A historical implementation-patch reference in registry code was replaced by
  the current provider-binding invariant.

## Evidence

- Normal CI passes: action-pin and shell checks, release wiring and truth,
  provider-boundary enforcement, formatting, workspace check, warning-denied
  Clippy, warning-denied rustdoc, all 73 native tests, Wasm compilation, and
  offline package build/verification.
- Rust 1.88 compiles every workspace target, and all supported nested runtime
  and size-probe configurations pass warning-denied Clippy on Rust 1.88.
- The dependency tree contains no duplicate versions. `cargo audit` reports no
  vulnerability across 13 locked dependencies and 1,216 RustSec advisories.
- The packaged-source inventory contains the intended manifest, lockfile,
  README, and 13 Rust source files; no repository-only audit/design material is
  shipped.
- The exact 0.3.7 changelog and release-truth candidate markers pass their
  non-mutating finalizer checks, and final diff whitespace validation passes.
- Release-script syntax, advisory behavior, release-gate wiring, action pins,
  and the revised workflow pass their focused checks, including `actionlint`.
- Generated public rustdoc contains the intended facade and value modules; it
  contains no `TimerFuture` or internal registry, platform, or provider-handle
  item.
- The production source contains no `unwrap`, `expect`, `panic`, `todo`, or
  `unimplemented` path; test-only panics remain explicit fixture assertions.

The PocketIC matrix is not rerun for this organization-only patch because the
provider protocol and registry transitions are unchanged. It remains mandatory
in the maintainer-owned release gate.
