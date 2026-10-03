# workaround

External constraints `exchange_level` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look.

### Overview

**None.**

`exchange_level` depends on four in-workspace crates — `exchange_id`,
`exchange_order`, `exchange_seq`, `exact_arith` — and zero published crates.
The `Vec`-based FIFO (rather than the source proposal's intrusive linked
list) is a design decision, documented in
[`../decisions/001_no_level_error.md`](../decisions/001_no_level_error.md)
and this crate's `readme.md` — not a workaround, since nothing external
forces it; Rust's own safety model is why the alternative was never
attempted, not a constraint this crate works around after the fact. Verify
with:

```bash
cd module/exchange_level && cargo tree --depth 1
```

**Expected:** `exact_arith`, `exchange_id`, `exchange_order`, `exchange_seq`
— all four resolving to a `(path = ...)` source under `module/`, no
third-party crate line.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — four in-workspace crates, no third-party |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_level
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  4
```
