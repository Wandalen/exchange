# item

The exposed surface of `exchange_seq`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Sequence` | struct | `( pub u64 )` |
| `Sequence::ZERO` | const | `Sequence` |
| `seq_next` | fn | `(Sequence) -> Sequence` |

### Differs from the proposal

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:567-570`,
also catalogued centrally at
[`../../../../docs/exposed_item/007_exchange_seq_items.md`](../../../../docs/exposed_item/007_exchange_seq_items.md))
names `Seq(u64)` with `seq_zero`/`seq_next`/`seq_cmp` and a `SeqError`. The
real build renames the type `Sequence` and builds `seq_next` exactly as
named (the central catalog's claim that `seq_next` doesn't exist predates
this crate's real Stage 1 build and is now stale — `exchange_core::emit`
calls it directly today). `seq_zero` is covered by `Sequence::ZERO`, a
const rather than a function — same value, no call needed. No `seq_cmp` and
no `SeqError`: see
[`../decisions/001_no_seq_cmp_or_seq_error.md`](../decisions/001_no_seq_cmp_or_seq_error.md).
