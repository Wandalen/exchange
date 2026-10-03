# workaround

External constraints `smoke_exchange_phases` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `smoke_exchange_phases` depends only on this family's own root
crates, zero published crates. Verify with:

```bash
cd module/smoke_exchange_phases && cargo tree --depth 1
```

**Expected:** only `exchange_id`/`exchange_side`/`exchange_tif`/`exchange_stp`/
`exchange_seq`/`exchange_cap`, each resolving to a `(path = ...)` source under
`module/`.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — six in-workspace path dependencies |
