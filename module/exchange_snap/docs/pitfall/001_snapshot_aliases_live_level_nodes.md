# Pitfall: Snapshot that aliases live level nodes

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Taking a snapshot that shares storage with the live book instead of copying its rows.
- **In Scope**: `snap_take` and `RestRow`'s own field types.

### Statement

A snapshot is supposed to be a frozen copy — if it instead aliases the live
book's own nodes, a cancel or a fill happening after the snapshot was taken
mutates the "frozen" copy too, silently defeating the entire reason a
snapshot was taken.

### How this crate avoids it

`RestRow { order : OrderId, price : Money, qty : Quantity }` holds only
`Copy` values taken *out of* each `exchange_book::Resting`, never a reference
into `Book`'s own storage. `snap_take` cannot return anything that aliases
the book by construction — there is no reference-typed field for it to leak
through. Verified directly:
[`tests/exchange_snap_test.rs`](../../tests/exchange_snap_test.rs)'s
`snap_is_a_copy_not_a_view` cancels the only resting order immediately after
taking a snapshot of it and asserts the snapshot still shows it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/030_snapshot_aliases_live_level_nodes.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:933` | Pitfall in the source's Prompt 7 "Snapshot" list |
