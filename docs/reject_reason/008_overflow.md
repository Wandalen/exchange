# Reject Reason: Overflow

### Scope

- **Purpose**: Name a dropped inbound publish — because the ring was full — as a reject, never a silent loss.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The ring-full case `exchange_inbound` turns into a reject rather than a drop.
- **Out of Scope**: Any book-side or escrow-side admission check — this fires before the order ever reaches the book.

**Design status**: Built, but not as a `RejectReason` value. The real `RejectReason` enum (`exchange_fill/src/lib.rs`) has no `Overflow` variant, by [decision 004](../decision/004_overflow_is_a_reject.md): a refused command never reaches the consumer, so no event can carry it. The producer gets the reject instead: `inbound_overflow_reject`'s `Err` hands the command back, and `inbound_flush`'s count short of the batch marks the unsent tail.

### Statement

`Overflow` marks the reject raised when a producer's publish fails because the inbound ring is at capacity — the proposal's own pitfall tape and hard problem 24 both insist this must surface to the caller as an explicit reject, never as a silent drop that leaves the player believing an order was placed.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 8 of 9 |
