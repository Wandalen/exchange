# Pitfall: Generation-less ids reused after cancel in the same snapshot

### Scope

- **Purpose**: Name a specific mistake this crate's own typing does not yet structurally prevent.
- **Responsibility**: `OrderId`'s own shape — whether it carries enough to detect reissue.
- **In Scope**: `OrderId`'s own fields.

### Statement

Without a generation component, a reused id can collide with a stale
reference to the order it used to name — a snapshot, a log entry, or a UI
element holding the old id can suddenly start pointing at an unrelated new
order, which looks like data corruption rather than an id-reuse bug.

### Design status — guarded by the facade, not by the type

`OrderId( pub u64 )` (`src/lib.rs`) carries no generation, so the type alone
cannot detect reissue. Through `exchange_core` it cannot happen: `step_place`
ignores the submitted id and mints a fresh one from a counter that only goes
up (`claim_order`). A caller driving `exchange_inbound::inbound_apply`
directly picks its own ids, and `exchange_idem::idem_remove` deliberately
frees a cancelled id for resubmission — there, reuse stays possible. A
generation field, if ever needed, would be added to this crate's type.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/029_generationless_ids_reused_in_same_snapshot.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:930` | Pitfall in the source's Prompt 7 "Identity and cap" list |
