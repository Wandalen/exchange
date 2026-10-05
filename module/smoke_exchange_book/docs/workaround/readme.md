# workaround

External constraints `smoke_exchange_book` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints compensated in shared tooling (→ [`docs/workaround/`](../../../../../../docs/workaround/readme.md) at the repository root); constraints a future dependency absorbs on its own behalf, to be documented in that crate's own `docs/workaround/`, not duplicated here.

### Overview

**None.**

`smoke_exchange_book` is the wall scenario — every stage in one run, against the P30 golden block. It depends on three crates (`exchange_core`, `exchange_conserve`, `exchange_idem`) and zero published crates. Its source was read in full
and holds no filesystem, environment-variable, or process-identity access —
nothing here compensates for a toolchain limitation, a platform quirk, or a
published dependency's own gap, because there is no such surface to
compensate against. Verify with:

```bash
# from this crate's root ( module/smoke_exchange_book/ )
cargo tree --depth 1
```

**Expected:** `exchange_core`, `exchange_conserve`, `exchange_idem`, each resolving to a `(path = ...)` source under `module/`.

### Workarounds

| File | Relationship |
|------|--------------|
| [`../../../../../../docs/workaround/readme.md`](../../../../../../docs/workaround/readme.md) | Repo-wide workarounds; none reach this crate |

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — 3 dependencies, all in-workspace path dependencies |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/module/smoke_exchange_book
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  3
```
