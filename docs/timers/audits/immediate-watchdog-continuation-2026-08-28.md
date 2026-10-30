![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Immediate Watchdog continuation evidence

Date: 2026-08-28. Status: historical pre-release evidence for 0.7.0.

## Subject

This report covers the progress-sensitive Watchdog extension in
[the design record](../design/immediate-watchdog-continuation.md). It uses Rust
1.88.0, exact `ic-cdk-timers` 1.0.0, and the pinned PocketIC 15.0.0 binary with
SHA-256
`29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb`.

The native suite exercises the complete immediate transition/arbitration
matrix and injected provider faults. PocketIC exercises the real provider and
replicated-message boundary. The existing trap and 40-billion-instruction
exhaustion cases remain the recovery evidence for the pre-armed cadence
successor.

## Focused behavior evidence

The pinned PocketIC Watchdog matrix passes eight tests. The two new subjects
prove:

- an inactive zero-delay initial scheduler completes one scheduler/work pair
  without advancing the one-second cadence time; and
- a successful work callback replaces its cadence successor at deadline now,
  after which a second scheduler/work pair completes without another time
  advance.

The same matrix still passes explicit work trap, actual instruction exhaustion,
independent timer isolation, upgrade reconstruction, insufficient-cycle
deferral/top-up, overdue coalescing, stop/resume, scheduler/work-gap
cancellation, bounded inventory, and private executor ingress.

Native fault injection separately proves that an internal completion fault
before replacement traps with the cadence successor still armed, that an
installation failure during callback replacement traps for replicated
rollback, and that a public replacement failure retires false scheduled state
to `ProviderBindingFailed`.

## Instruction and cycle observation

One fresh immediate-continuation subject used identical successful callback
work on consecutive attempts. The first returned `ContinueImmediately`; the
second returned cadence `Continue`.

| Attempt | Scheduler interval | Work interval | Scheduler/work cycle delta |
| --- | ---: | ---: | ---: |
| Immediate replacement | 21,596 | 27,811 | 30,742,889 |
| Cadence retention | 21,006 | 19,451 | 30,732,414 |
| Observed difference | +590 | +8,360 | +10,475 |

The work-interval difference contains the additional checked generation/state
transition plus one provider clear/set replacement. The scheduler itself does
not perform that replacement; its difference is retained as observed run
variation rather than attributed to the feature. PocketIC 15 does not expose
complete per-message instruction totals, so neither row is a full IC-message
cost or an instruction-limit proof.

The unchanged cadence Watchdog cohort reports 20,944 scheduler-interval
instructions, 18,548 work-interval instructions, and 30,732,422 cycles for its
one scheduler/work dispatch. The minimal calibration reports 20,941 scheduler
and 18,321 work instructions with 30,731,039 cycles; the representative work
reports 20,973 scheduler and 1,918,336 work instructions with 32,626,885
cycles.

## Linked Wasm size

The maintained mutually exclusive size probes were rebuilt with the release
profile. Final Wasm uses Binaryen 108 `wasm-opt -Oz --enable-bulk-memory
--enable-sign-ext`; gzip is deterministic `gzip -n -9`.

| Cohort | Compiler Wasm | Final Wasm | Deterministic gzip |
| --- | ---: | ---: | ---: |
| Baseline | 263,831 | 214,353 | 85,828 |
| Once | 321,442 | 261,599 | 105,157 |
| After-completion | 321,450 | 261,914 | 105,210 |
| Watchdog | 322,759 | 262,791 | 105,538 |

In this controlled recorded build, Watchdog exceeds after-completion by
1,309 compiler bytes (0.407%), 877 final raw bytes (0.335%), and 328 gzip bytes
(0.312%). The final raw size is authoritative; gzip is secondary. This is a
cross-policy linked-code delta, not a downstream IcyDB Wasm measurement.

## Complexity and scope

The implementation adds no runtime state variant, snapshot field, persisted
value, provider path, registration, handle slot, callback role, or retry-policy
parameter. It adds one public decision, one public Watchdog reconciliation
state, one private pending-command variant, and one private request selector.
The registry remains bounded at 64 entries and at most two owned provider
handles per Watchdog.
