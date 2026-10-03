# workaround

External constraints `exchange_order` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_order` depends only on `exchange_id`, `exchange_side`, and
`exchange_tif` (all zero-dependency, in-workspace), plus `exact_arith`
(in-workspace, for `Price`/`Quantity`) — nothing external to work around.
Verify with:

```bash
cd module/exchange_order && cargo tree --depth 1
```

**Expected:** `exchange_id`, `exchange_side`, `exchange_tif`, and
`exact_arith` only, no third-party crate line.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — four in-workspace crates |
