# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface, including the six `ring_factory`/`ring_handle` types re-exported here (`BuildError`, `RingConfig`, `Consumer`, `Drain`, `Ends`, `Producer`, `Split`) rather than declared.
- **Out of Scope**: Items declared elsewhere and merely used here (`Book`/`Resting` from `exchange_book`; `InstrumentId`/`OrderId` from `exchange_id`; `Crossing`/`MatchError`/`SelfMatchPolicy` from `exchange_match`; `RestReplaceError` from `exchange_rest`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `InboundCmd` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `InboundOutcome` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `inbound_ring` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `inbound_flush` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `inbound_overflow_reject` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `inbound_drain` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `inbound_apply` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `BuildError` | enum (re-export) | `ring_factory` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `RingConfig` | struct (re-export) | `ring_config` crate, re-exported by `ring_factory` and again at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Consumer` | struct (re-export) | `ring_handle` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Drain` | struct (re-export) | `ring_handle` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Ends` | struct (re-export) | `ring_handle` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Producer` | struct (re-export) | `ring_handle` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Split` | struct (re-export) | `ring_handle` crate, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

No decisions-collection pointer for the `Inbound`/`InboundError` omissions —
see [`../item/readme.md`](../item/readme.md)'s "Differs from the proposal"
for that reasoning.
[`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)
covers the one divergence that warranted a full ADR (a real alternative was
weighed and rejected, not just a shape that didn't materialize).
