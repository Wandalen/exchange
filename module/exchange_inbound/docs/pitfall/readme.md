# Pitfall Doc Definition

### Scope

- **Purpose**: Record the "Ring" pitfalls named for workstream 002 that are cleanly this crate's own ring-construction and drain-combination choices to avoid, so this crate's own code and tests can show each is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_inbound` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The three "Ring" pitfalls that are this crate's own `inbound_ring`/`inbound_drain`/`inbound_overflow_reject` construction and configuration choices.
- **Out of Scope**: Central pitfall `009` (replace leaves the old rest in place) — verified this session to be `exchange_rest`'s own concern, not this crate's (see Related, below). The remaining "Ring"-category central pitfalls (`020`, `021`, `023`, `026`) were outside this session's assigned scope and were not evaluated — they stay central pending their own look.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Drain Order From Thread Completion](001_drain_order_from_thread_completion.md) | Why the fixed two-ring combine order never depends on which thread finished first | 🔄 |
| 002 | [Ring Overflow As Silent Drop](002_ring_overflow_as_silent_drop.md) | Why `inbound_ring` always configures `OverflowPolicy::Fail` | 🔄 |
| 003 | [Ring SPSC As Market Path](003_ring_spsc_as_market_path.md) | Why many producers are many single-producer rings, never one shared ring | 🔄 |

### Related

- [`../../../exchange_rest/src/lib.rs`](../../../exchange_rest/src/lib.rs) — `rest_replace` is where pitfall `009` (replace leaves the old rest in place) is actually decided and implemented (cancel old, insert new, roll back on refusal); `inbound_apply`'s `Replace` arm is a one-line, logic-free delegation to it. `009` has since been redistributed to [`../../../exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md`](../../../exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md), `exchange_rest`'s own per-crate copy — the central catalog's `docs/pitfall/009_replace_leaves_old_rest_in_place.md` now points there too.

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_inbound/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              3
# rows in Overview Table: 3
```
