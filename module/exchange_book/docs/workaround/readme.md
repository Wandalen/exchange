# workaround

External constraints `exchange_book` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.**

`exchange_book` depends on `exchange_id`, `exchange_level`,
`exchange_side` and `exact_arith`, and no published crate. Verify with:

```bash
cd module/exchange_book && cargo tree --depth 1
```

**Expected:** `exchange_id`, `exchange_level` and `exchange_side` from
`module/`, and `exact_arith` from its pinned git revision.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — `exchange_id`, `exchange_level`, `exchange_side`, `exact_arith` — `exact_arith` a git dependency pinned in the root `Cargo.toml`, the rest in-workspace path dependencies |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_book
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  4
```
