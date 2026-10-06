# tif

One of the 26 doc entity kinds Prompt 8
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`)
names for workstream 002 — the three-value time-in-force type the source
design proposes. Redistributed here from the central catalog since this
crate is the one place `Tif` is owned.

### Scope

- **Purpose**: Catalog the source design's own three named time-in-force values, and how each maps onto the real `Tif` enum.
- **Responsibility**: One instance per proposal-named value.
- **In Scope**: The proposal's three named values, as catalog instances.
- **Out of Scope**: The real enum's own documentation (→ [`../item/readme.md`](../item/readme.md)).

### Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_gtc.md`](001_gtc.md) | Rests until filled or cancelled |
| [`002_ioc.md`](002_ioc.md) | Fills what it can now, discards the rest — dropped by `exchange_core`/`exchange_inbound` |
| [`003_fok.md`](003_fok.md) | Fills completely or rejects, book unchanged — gated by `exchange_match::cross` |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_tif/docs/tif
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Responsibility Table: '; grep -cE '^\| \[`[0-9]{3}_' readme.md
# instances:              3
# rows in Responsibility Table: 3
```
