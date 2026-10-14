# workaround

External constraints `exchange_snap` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_snap` depends on `exchange_book`, `exchange_id`,
`exchange_side` and `exact_arith`, and no published crate. Verify with:

```bash
cd module/exchange_snap && cargo tree --depth 1
```

**Expected:** `exchange_book`, `exchange_id` and `exchange_side` from
`module/`, and `exact_arith` from its pinned git revision.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding |
