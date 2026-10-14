# Pitfall Doc Definition

### Scope

- **Purpose**: Record the ring pitfalls this crate's ring construction and drain must avoid.
- **Responsibility**: Document each pitfall, how `exchange_inbound` avoids it, and the test or check that verifies it.
- **In Scope**: Central pitfalls 022, 024 and 025.
- **Out of Scope**: Central pitfall 009 (replace leaves the old rest in place), which is `exchange_rest`'s ([`../../../exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md`](../../../exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md)); the other ring pitfalls, about `exchange_book` and `exchange_match` (020, 021), `exchange_step` (023) and `exchange_event` (026).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Drain Order From Thread Completion](001_drain_order_from_thread_completion.md) | Why many rings drain in a lane order fixed ahead of time, not by which thread finished first | 🔄 |
| 002 | [Ring Overflow As Silent Drop](002_ring_overflow_as_silent_drop.md) | Why `inbound_ring` always configures `OverflowPolicy::Fail` | 🔄 |
| 003 | [Ring SPSC As Market Path](003_ring_spsc_as_market_path.md) | Why many producers are many single-producer rings, never one shared ring | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_inbound/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              3
# rows in Overview Table: 3
```
