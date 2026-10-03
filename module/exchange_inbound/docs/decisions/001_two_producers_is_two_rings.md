# ADR-001: Two Producers Is Two Rings, Not One Shared One

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The plan's Stage 8 scope names `inbound_producer` singular and one ring, and
`docs/phase/028_p28_drain.md` asks for drain order to be deterministic across
"two producers publishing concurrently" — real OS-thread concurrency, not a
single thread pushing twice in sequence (that would never exercise the
property the phase names: "not thread-completion-order-dependent").

`ring_handle::Producer` cannot be cloned. Its own module doc names this
directly: `try_clone` is "withheld... refused at compile time here, at
runtime one crate down" — `ring_core::Producer::try_clone` exists and can
succeed on an MPSC backend, but `ring_handle` deliberately never exposes it.
`ring_handle::Ends::split` also returns exactly one `Producer`/`Consumer`
pair per ring, with no second call yielding a second pair (`Ends::split`
borrows `&'a mut self`). Stage 8's own research (this session, confirmed by
reading `ring_handle`/`ring_factory` source directly) resolved the
dependency set to `ring_factory` + `ring_handle` + `ring_types` only — not
`ring_core`/`ring_mpsc` directly, where the real multi-producer claim path
(a compare-exchange) lives. So through this crate's own dependency set,
**one ring has exactly one producer, full stop.**

## Decision

"Two producers" is built as two independent rings (`inbound_ring` called
twice), each with its own exclusively-owned `Producer`, each genuinely
raced from its own OS thread with no shared mutable state and no mutex
between them. The drain side combines both rings' `inbound_drain` results
in a **fixed lane order fixed ahead of time** (e.g. "ring A's full drain,
then ring B's") — never by which thread happened to finish publishing
first. Real concurrency exists at the push side (two threads genuinely
race); the result is insensitive to that race's outcome at the drain side,
which is exactly what "deterministic... not thread-completion-order-
dependent" asks for.

This combining step lives in the P28 phase-smoke itself, not as library API
— how many lanes, and in what order, is a caller/deployment decision this
crate should not guess at, the same reasoning `exchange_conserve`'s own
scope already applies to batch boundaries.

## Alternatives Considered

### Option 1: One shared ring, `Producer` wrapped in a `Mutex`

Rejected: this family's own stated purpose is eliminating "the lock-convoy
jitter" of a shared lock under concurrent producers (`substrate/ring`'s own
root readme). Wrapping the one `Producer` this crate's dependency set can
ever obtain in a `Mutex` to fake a second producer would reintroduce
exactly that lock contention, for a capability (true lock-free MPSC) this
crate was never going to get anyway without depending on `ring_core`/
`ring_mpsc` directly — which Stage 8's own resolved dependency set declined.

### Option 2: Depend on `ring_core`/`ring_mpsc` directly for a real cloneable producer

Rejected: outside the family's own blessed five-crate export Contract
(`ring_types`, `ring_handle`, `ring_tls`, `ring_flush`, `ring_factory`), and
outside Stage 8's already-resolved dependency set. Would also require this
crate to re-implement the broadcast/claim coordination `ring_handle` exists
specifically to narrow away from callers.

## Consequences

**Positive:** no new dependency beyond the three already-sanctioned `ring_*`
crates; genuine OS-thread concurrency at the push side, satisfying P28's own
"not thread-completion-order-dependent" requirement; the combining rule is
visible, fixed, and auditable rather than implicit in whichever backend
happens to be selected.

**Negative:** two rings means two capacities to size and two overflow
surfaces to handle, rather than one. Not revisited here: nothing in the
real call sites needs a single shared ring today, and the lane-count/order
decision stays with the caller, where it belongs.

## Related

- [`docs/phase/028_p28_drain.md`](../../../../docs/phase/028_p28_drain.md) — the phase this decision serves
- [`src/lib.rs`](../../src/lib.rs)'s own module doc, "Two producers without two threads on one ring"
