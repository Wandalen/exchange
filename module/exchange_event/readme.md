# exchange_event

The event stream's push/drain/len/clear surface — for workstream 010 to read
fills, rejects, and cancel-acks without writing wallets directly. Depends on
`exchange_types` only.

```rust
use exchange_event::{ Event, event_drain, event_push };

let mut events : Vec< Event > = Vec::new();
let drained = event_drain( &mut events );
assert!( drained.is_empty() && events.is_empty() );
```

## Why this crate exists despite being marked "Folded" centrally

`docs/crate/019_exchange_event.md` records `Event`/`EventKind` as already
folded into `exchange_types`, with `exchange_core::events()` as the
accessor — true, and not re-litigated here. But `events()` returns a
borrowed `&[Event]`, never an owned drain: this crate adds the genuinely
missing pieces — `event_push`/`event_drain`/`event_len`/`event_clear`
([`src/lib.rs`](src/lib.rs)) — without a second definition of the types they
operate on.

## Diverges from the proposal

No `EventDrain` type, no `EventError` — see
[`docs/decisions/001_no_event_drain_type.md`](docs/decisions/001_no_event_drain_type.md)
and
[`docs/decisions/002_no_event_error.md`](docs/decisions/002_no_event_error.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exchange_types` only |
| [`src/lib.rs`](src/lib.rs) | Re-exports `Event`/`EventKind`; adds `event_push`/`event_drain`/`event_len`/`event_clear` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `EventDrain` and `EventError` are not part of this build |
| `docs/pitfall/` | The 1 "Ring" pitfall this crate's storage choice bears on |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_event_test.rs`](tests/exchange_event_test.rs) | Test Matrix T01 — push/drain/len/clear |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |
