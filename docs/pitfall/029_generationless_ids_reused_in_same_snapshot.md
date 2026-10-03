# Pitfall: Generation-less ids reused after cancel in the same snapshot

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Reissuing a cancelled order's bare numeric id to a new order within the lifetime of one snapshot.
- **In Scope**: Identity and cap

Redistributed to the single crate its own type lives in:
[`../../module/exchange_id/docs/pitfall/001_generationless_ids_reused_in_same_snapshot.md`](../../module/exchange_id/docs/pitfall/001_generationless_ids_reused_in_same_snapshot.md)
— that writeup records the honest current status (not yet structurally
avoided; no id allocator exists yet to guard). This entry stays here only
as the central catalog's own record.

### Statement

Without a generation component, a reused id can collide with a stale reference to the order it used to name — a snapshot, a log entry, or a UI element holding the old id can suddenly start pointing at an unrelated new order, which looks like data corruption rather than an id-reuse bug.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:930` | Pitfall in the source's Prompt 7 "Identity and cap" list |
