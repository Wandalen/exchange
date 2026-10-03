# Invariant Doc Definition

### Scope

- **Purpose**: State the properties the matching engine must hold under every code path, so a violation is a recognised defect rather than absorbed behaviour.
- **Responsibility**: Document `exchange_core`'s own invariants and how the crate enforces each.
- **In Scope**: Properties of the book and ledger the engine owns — what backs an order, when reservations release, the order arrivals are processed in, and the shape a holding has.
- **Out of Scope**: Conservation of the value types themselves (→ [`exact_conserve`](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)); determinism of the hosting substrate, not yet integrated.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Escrow Covers Resting Orders](001_escrow_covers_resting_orders.md) | Every resting order's maximum obligation is fully reserved; fill and cancel release instantly | 🔄 |
| 002 | [Arrival Sequence Is Total and Replayable](002_arrival_sequence_total_and_replayable.md) | One total arrival order gives "time priority" a meaning; the same accepted sequence replays to byte-identical fills | 🔄 |
| 003 | [Available and Reserved Balance Separation](003_available_and_reserved_balance_separation.md) | A holding is a conserved `( available, reserved )` pair, so over-commitment is unrepresentable rather than checked | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/exchange_core/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              3
# rows in Overview Table: 3
```
