# workaround

External constraints `exchange_types` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints a future dependency absorbs on its own behalf, to be documented in that crate's own `docs/workaround/`, not duplicated here. (No family-wide `docs/workaround/` collection exists under `substrate/exchange/docs/` to defer to — every crate checked so far reports zero instances, so there is nothing yet for a central collection to hold.)

### Overview

**None.**

`exchange_types` depends on three in-workspace crates — `exact_arith`,
`exchange_order`, `exchange_side` — plus `exchange_id`/`exchange_tif` as
dev-only dependencies for its own tests, and zero published crates. The
price/notional/obligation vocabulary is expressed directly in terms of the
exact-arithmetic facade's own types and the two leaf crates `obligation`'s
signature names (`Order`, `Side`), privately imported rather than
re-exported — this crate no longer re-exports anything it does not itself
declare. Verify with:

```bash
cd module/exchange_types && cargo tree --depth 1 --edges normal
```

**Expected:** `exact_arith`, `exchange_order`, `exchange_side` — all three
resolving to a `(path = ...)` source under `module/`, no third-party crate
line.

### Workarounds

None. No instance file exists in this directory, and no family-wide
`docs/workaround/` collection exists to check against (see Out of Scope,
above).

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — three in-workspace crates (normal deps), no third-party |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/module/exchange_types
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  3
```
