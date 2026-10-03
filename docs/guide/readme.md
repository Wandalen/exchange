# Guide Doc Definition

### Scope

- **Purpose**: Orient a new reader in the real, as-built crate family — what each crate does, how to call it, and how the 23 crates relate to each other — without re-deriving what each crate's own readme already says.
- **Responsibility**: Each instance is one self-contained orientation document. This first instance covers the whole family in one pass.
- **In Scope**: The real `module/` crates as they exist today — their actual `Cargo.toml` dependency edges, their actual public surface, their actual wiring (or lack of it) into `exchange_core`.
- **Out of Scope**: The 23-crate proposal's own per-crate analysis (→ [`../crate/`](../crate/readme.md), [`../exposed_item/`](../exposed_item/readme.md) — the proposed-vs-real comparison already lives there); any crate's own design trade-offs in depth (→ that crate's own `readme.md`).

**Design status**: one instance so far, dated 2026-10-03, covering all 25 real crates/lanes under `module/` as of that date, verified directly against every crate's own `Cargo.toml` rather than against readme prose (two readmes were found to understate their own crate's dependencies while writing this). Re-verify the crate count and tier table if the family has grown since.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [The Exchange Crate Family](001_crate_family_guide.md) | Purpose, usage, and dependency tier for all 25 real crates/lanes | ✅ |

### Why this collection exists, alongside `research/`

Same shape as [`../research/`](../research/readme.md): a non-catalog addition, not one of Prompt 8's 26 named entities (`../readme.md` § Responsibility Table lists both exceptions now). `research/` answers an external question ("could an existing crate replace this family?"); this collection answers an internal one ("what does this family actually look like today, and how do I use it?"). Requested directly, not derived from the source transcript.

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/guide
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
