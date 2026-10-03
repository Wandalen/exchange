# workaround

External constraints `exchange_core` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints a dependency absorbs on its own behalf, to be documented in that crate's own `docs/workaround/`, not duplicated here. (No family-wide `docs/workaround/` collection exists under `substrate/exchange/docs/` to defer to — confirmed the same way `exchange_escrow`'s own `docs/workaround/readme.md` already found; every crate checked so far reports zero instances, so there is nothing yet for a central collection to hold.)

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

None. No instance file exists in this directory, and no family-wide
`docs/workaround/` collection exists under `substrate/exchange/docs/` to
check against either (see Out of Scope, above). This section previously
linked six directory levels up from here, past the repo root into a
nonexistent sibling `docs/workaround/readme.md` — fixed by removing the
dead link rather than correcting its depth, since no such repo-wide
collection exists at any depth.

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
