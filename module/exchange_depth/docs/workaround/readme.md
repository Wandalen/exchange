# workaround

External constraints `exchange_depth` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_depth` depends only on sibling workspace crates
(`exchange_book`, `exchange_side`) and the family's own `exact_arith`
facade. Verify with:

```bash
cd module/exchange_depth && cargo tree --depth 1
```

**Expected:** `exchange_book`, `exchange_side`, `exact_arith` — all
in-workspace or in-family, no external crate.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding |
