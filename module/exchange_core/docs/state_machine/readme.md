# State Machine Doc Definition

### Scope

- **Purpose**: Fix when an order is on the book, when it holds escrow, and when it is finished, so an illegal transition is recognisable as a defect rather than as an implementation choice.
- **Responsibility**: Document `exchange_core`'s own state machines — those governing an order the engine holds, not the substrate hosting it.
- **In Scope**: States, transitions with their triggers, and the behavioural invariants holding across every transition.
- **Out of Scope**: The matching steps driving the transitions (→ [`algorithm/`](../algorithm/readme.md)); the lifecycle of a scheduled registration, if a time-bounded Time-in-Force is ever added, which would belong to a separate, upstream scheduling substrate this crate does not define; the host substrate's own account lifecycle, likewise external to this crate.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Order Lifecycle](001_order_lifecycle.md) | Six states with on-book/escrow/terminal columns, every legal transition, and seven behavioural invariants; Expired named as an open extension | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_core/docs/state_machine
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
