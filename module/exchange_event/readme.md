# exchange_event

The owned drain point for the event stream — workstream 010 takes fills,
rejects and cancel-acks out of it, rather than 002 writing wallets. Depends
on `exchange_fill` only. Closes hard problem 12 and feature 26.

```rust
use exchange_event::{ Event, event_drain, event_push };

let mut events : Vec< Event > = Vec::new();
let drained = event_drain( &mut events );
assert!( drained.is_empty() && events.is_empty() );
```

## Where it sits

`exchange_core::Exchange` keeps its events in a `Vec< Event >`. `events()`
lends them as a slice; `event_drain()` calls this crate's `event_drain` and
leaves the stream empty. `Exchange::postings` audits only what is still in
the stream, so a drained batch is audited by passing it to
`exchange_core::postings`.

## Diverges from the proposal

No `EventDrain` type and no `EventError { Full }`: the functions take the
`Vec` itself, and nothing bounds it — see
[`docs/decisions/001_no_event_drain_type.md`](docs/decisions/001_no_event_drain_type.md)
and [`docs/decisions/002_no_event_error.md`](docs/decisions/002_no_event_error.md).

## Not built: more than one consumer

`event_drain` hands the stream to one consumer. A second — an audit beside
workstream 010, say — needs a cursor of its own, and the stream has to keep
events until the slowest has read them. That is a retention policy, and the
family has neither a clock nor a capacity to base one on (ADR-002).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill` only |
| [`src/lib.rs`](src/lib.rs) | Re-exports `Event`/`EventKind`; `event_push`/`event_drain`/`event_len`/`event_clear` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why there is no `EventDrain` or `EventError` |
| `docs/pitfall/` | A second fill ring built before 010 asks for one |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_event_test.rs`](tests/exchange_event_test.rs) | Test Matrix T01 — push/drain/len/clear |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_fill/`](../exchange_fill/readme.md) — `Event`, `EventKind`
- [`exchange_core/`](../exchange_core/readme.md) — `Exchange::event_drain`, the one caller
