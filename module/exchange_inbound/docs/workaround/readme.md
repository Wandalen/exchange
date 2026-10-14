# workaround

External constraints `exchange_inbound` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look — see `docs/decisions/`.

### Overview

**None.** `exchange_inbound` depends on six in-workspace crates
(`exchange_book`, `exchange_id`, `exchange_idem`, `exchange_match`,
`exchange_rest`, `exchange_tif`), `exact_arith` from `Wandalen/exact`, and
three from `Wandalen/ring` (`ring_factory`, `ring_handle`, `ring_types`),
the last four pinned by revision in the root `Cargo.toml`. Verify with:

```bash
cd module/exchange_inbound && cargo tree --depth 1 --edges normal
```

**Expected:** those ten, no third-party crate line.

### Workarounds

None. No instance file exists in this directory.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_inbound
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  10
```
