# item

The exposed surface of `exchange_id`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `InstrumentId` | struct | `( pub u64 )` |
| `OrderId` | struct | `( pub u64 )` |
| `AccountId` | struct | `( pub u64 )` |
| `instrument_from_raw` | fn | `(u64) -> InstrumentId` |
| `instrument_raw` | fn | `(InstrumentId) -> u64` |
| `order_from_raw` | fn | `(u64) -> OrderId` |
| `order_raw` | fn | `(OrderId) -> u64` |
| `account_from_raw` | fn | `(u64) -> AccountId` |
| `account_raw` | fn | `(AccountId) -> u64` |

### Matches the proposal, except `IdError`

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:535-540`,
also catalogued centrally at
[`../../../../docs/exposed_item/001_exchange_id_items.md`](../../../../docs/exposed_item/001_exchange_id_items.md))
names `InstrumentId(u64)`/`OrderId(u64)`/`AccountId(u64)` plus both
`_from_raw`/`_raw` conversion pairs for each — every one of those nine items
is built here exactly as named (the central catalog's own note that these
were "folded into `exchange_types`" with public-field tuple structs instead
of opaque newtypes predates this crate's real Stage 1 build and is now
stale — see the thinned pointer at
[`../../../../docs/exposed_item/001_exchange_id_items.md`](../../../../docs/exposed_item/001_exchange_id_items.md)).

The one real divergence is `IdError { Zero }`, which does not exist here —
see
[`../decisions/001_no_id_error.md`](../decisions/001_no_id_error.md).
