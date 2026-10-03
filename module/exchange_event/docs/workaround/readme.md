# workaround

External constraints `exchange_event` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_event` depends on one sibling workspace crate
(`exchange_types`) only. Verify with:

```bash
cd module/exchange_event && cargo tree --depth 1
```

**Expected:** `exchange_types`, in-workspace, no external crate.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding |
