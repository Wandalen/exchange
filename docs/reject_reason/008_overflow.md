# Reject Reason: Overflow

### Scope

- **Purpose**: Name a dropped inbound publish — because the ring was full — as a reject, never a silent loss.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The ring-full case `exchange_inbound` is proposed to turn into a reject rather than a drop.
- **Out of Scope**: Any book-side or escrow-side admission check — this fires before the order ever reaches the book.

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` now only re-exports it) has a completely different variant set and does not include this concept at all. Consistent with there being zero `ring_*` dependency anywhere in the real crate family (verified via grep across all `module/*/Cargo.toml`) — there is no ring to overflow.

### Statement

`Overflow` marks the reject raised when a producer's publish fails because the inbound ring is at capacity — the proposal's own pitfall tape and hard problem 24 both insist this must surface to the caller as an explicit reject, never as a silent drop that leaves the player believing an order was placed. It cannot exist before `exchange_inbound` and a ring dependency exist.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 8 of 9 |
