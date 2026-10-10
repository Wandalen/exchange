# workaround

External constraints `exchange_types` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.** `exchange_types` depends on three in-workspace crates —
`exact_arith`, `exchange_order`, `exchange_side` — plus dev-only
`exchange_id`/`exchange_tif`, and no published crate. Verify with:

```bash
cd module/exchange_types && cargo tree --depth 1 --edges normal
```

**Expected:** `exact_arith`, `exchange_order`, `exchange_side` — all path
dependencies, no third-party crate line.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_types
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  3
```
