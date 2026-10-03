# item

The exposed surface of `exchange_side`, as built — a consolidated index
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
| `Side` | enum | `{ Buy, Sell }` |
| `Side::opposite` | method | `(self) -> Self` |
| `side_opposite` | fn | `(Side) -> Side` |
| `side_is_bid` | fn | `(Side) -> bool` |
| `side_is_ask` | fn | `(Side) -> bool` |

### Matches the proposal's full function list; one naming divergence

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:542-544`,
also catalogued centrally at
[`../../../../docs/exposed_item/002_exchange_side_items.md`](../../../../docs/exposed_item/002_exchange_side_items.md))
names `Side { Bid, Ask }` with `side_opposite`/`side_is_bid`/`side_is_ask` —
all three functions are built here exactly as named (the central catalog's
claim that `side_is_bid`/`side_is_ask` don't exist predates their addition
to this crate and is now stale). The one real, deliberate divergence is the
enum's variant names — see
[`../decisions/001_buy_sell_not_bid_ask.md`](../decisions/001_buy_sell_not_bid_ask.md).
