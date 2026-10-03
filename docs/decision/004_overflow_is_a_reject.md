# Decision: Overflow is a reject

### Scope

- **Purpose**: Guarantee a full inbound ring never silently loses an order.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: `exchange_inbound`'s overflow handling, surfaced as `RejectReason::Overflow`.
- **Out of Scope**: Any form of the ring silently dropping a publish — named explicitly as a pitfall.

**Design status**: Not yet applicable — no ring dependency or `exchange_inbound` crate exists yet, so there is nothing to overflow. See `../reject_reason/008_overflow.md` and `../hard_problem/024_ring_overflow_is_reject.md`.

### Statement

A full ring must answer a publish with an explicit reject, never a silent drop — stated as hard problem 24, repeated as a `reject_reason` value, and repeated again here as a closing decision, which is the strongest signal in the source that this is considered non-negotiable rather than an implementation detail.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 4 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:302-305` | Hard problem 24, the same rule |
