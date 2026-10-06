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
| `ClientOrderId` | struct | `( pub u64 )` |
| `instrument_from_raw` | fn | `(u64) -> InstrumentId` |
| `instrument_raw` | fn | `(InstrumentId) -> u64` |
| `order_from_raw` | fn | `(u64) -> OrderId` |
| `order_raw` | fn | `(OrderId) -> u64` |
| `account_from_raw` | fn | `(u64) -> AccountId` |
| `account_raw` | fn | `(AccountId) -> u64` |
| `client_from_raw` | fn | `(u64) -> ClientOrderId` |
| `client_raw` | fn | `(ClientOrderId) -> u64` |

### Matches the proposal, except `IdError`

The source design's exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:535-540`,
catalogued at
[`../../../../docs/exposed_item/001_exchange_id_items.md`](../../../../docs/exposed_item/001_exchange_id_items.md))
names the three id types and a `_from_raw`/`_raw` pair for each — all nine
are built as named. `IdError { Zero }` is not — see
[`../decisions/001_no_id_error.md`](../decisions/001_no_id_error.md).

`ClientOrderId` and its pair are additions: the submitter's own id, so a
retried placement can be recognised.
