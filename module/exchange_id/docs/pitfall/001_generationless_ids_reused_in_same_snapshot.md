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

### Design status — not structurally enforced

`OrderId( pub u64 )` (`src/lib.rs`) is a bare newtype — its own doc comment
states the intended invariant ("fixed at submission and never reassigned")
but nothing in the type itself stops a caller from constructing the same
raw value twice. This differs from
[`../decisions/001_no_id_error.md`](../decisions/001_no_id_error.md)'s
`IdError::Zero` case: that pitfall is avoided by a deliberate choice not to
add a check; this one isn't yet avoided at all, because no id allocator
exists anywhere in the real build for it to guard — every caller today
constructs `OrderId` literals directly (`OrderId( 1 )`, test fixtures,
etc.), so reissue hasn't been possible to trigger yet, not because it's
prevented. Whichever crate eventually owns id allocation (none does today)
is where a generation field or a seen-ids set would need to live; recorded
here, against `exchange_id`'s own type, because that's where the field
itself would be added if this is ever closed.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/029_generationless_ids_reused_in_same_snapshot.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:930` | Pitfall in the source's Prompt 7 "Identity and cap" list |
