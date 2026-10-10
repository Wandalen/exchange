# workaround

External constraints `exchange_escrow` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints a future dependency absorbs on its own behalf, to be documented in that crate's own `docs/workaround/`, not duplicated here. (No family-wide `docs/workaround/` collection exists under `substrate/exchange/docs/` to defer to — every crate checked so far reports zero instances, so there is nothing yet for a central collection to hold.)

### Overview

**None.**

`exchange_escrow` depends on `exchange_fill`, `exchange_id`,
`exchange_order`, `exchange_side`, `exchange_types`, and `exact_arith`, and
zero published crates — held-funds accounting is expressed directly over
the shared exchange vocabulary and exact-arithmetic facade rather than a
third-party ledger or wallet library. (Before the 2026-10-05 `exchange_types`
retirement this was just `exchange_types` and `exact_arith` — the other four
arrived only once their types had to be named directly instead of through
the old re-export.) Verify with:

```bash
# from this crate's root ( substrate/exchange/exchange_escrow/ )
cargo tree --depth 1
```

**Expected:** `exchange_fill`, `exchange_id`, `exchange_order`,
`exchange_side`, `exchange_types`, and `exact_arith`, all resolving to
`(path = ...)` sources under `module/`.

### Workarounds

None. No instance file exists in this directory, and no family-wide
`docs/workaround/` collection exists to check against (see Out of Scope,
above).

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_types`, `exact_arith` — `exact_arith` a git dependency pinned in the root `Cargo.toml`, the rest in-workspace path dependencies |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/module/exchange_escrow
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  6
```
