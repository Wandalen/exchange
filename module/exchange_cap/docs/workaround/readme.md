# workaround

External constraints `exchange_cap` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_cap` is a dependency-free root of the family's tree — one
struct, one error enum, two pure checks. Verify with:

```bash
cd module/exchange_cap && cargo tree --depth 1
```

**Expected:** `exchange_cap` alone, no dependency line beneath it.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — empty |
