# workaround

External constraints `exchange_core` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — for a crate with no dependencies, that means the language, the toolchain, and the targets.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints compensated in shared tooling (→ [`docs/workaround/`](../../../../../../docs/workaround/readme.md) at the repository root).

### Overview

**None.**

`exchange_core` has 8 real dependencies — `exact_arith`, `exchange_book`,
`exchange_escrow`, `exchange_id`, `exchange_match`, `exchange_seq`,
`exchange_tif`, `exchange_types` — every one of them an in-workspace or
first-party sibling path crate, and zero published (non-workspace) crates.
This list was previously recorded here as empty, which was stale: as the
facade over the rest of the family, a non-trivial dependency list is this
crate's whole point, unlike a leaf crate. None of the eight originates
outside this project, so there is still nothing here for a workaround to
compensate for. Verify the surface with:

```bash
# from this crate's root ( module/exchange_core/ )
cargo tree --depth 1
```

### Workarounds

| File | Relationship |
|------|-----------------|
| [`../../../../../../docs/workaround/readme.md`](../../../../../../docs/workaround/readme.md) | Repo-wide workarounds; none reach this crate, which has no rendering or dev-server path |

### Sources

| File | Relationship |
|------|-----------------|
| `Cargo.toml` | The dependency surface examined for this finding — 8 real path dependencies, zero published crates |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_core
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  8
```
