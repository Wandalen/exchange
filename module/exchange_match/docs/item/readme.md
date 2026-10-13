# item

The exposed surface of `exchange_match`, as built.

### Scope

- **Purpose**: What this crate exports, and how it compares to the source design's exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface, including the `SelfMatchPolicy` re-export.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Crossing` | struct | `{ trades: Vec<Trade>, remaining: Quantity, cancelled: Vec<SelfMatchCancellation> }` |
| `Crossing::is_complete` | fn | `(&self) -> bool` |
| `Crossing::filled` | fn | `(&self) -> Result<Quantity, KindError>` |
| `SelfMatchCancellation` | struct | `{ order: OrderId, account: AccountId, quantity: Quantity }` (`Copy`) |
| `MatchError` | enum | `{ Quantity(KindError), BookDesynchronized, Conservation(ConserveError), PostOnlyWouldTake }`, with `Display`, `Error`, `From<KindError>`, `From<ConserveError>` |
| `SelfMatchPolicy` | enum | `pub use exchange_stp::SelfMatchPolicy;` — `{ CancelResting, CancelIncoming, CancelBoth }` |
| `cross` | fn | `(book: &mut Book, incoming: &Order, policy: SelfMatchPolicy) -> Result<Crossing, MatchError>` |
| `would_take` | fn | `(book: &Book, incoming: &Order) -> bool` |

### Differs from the proposal

The source design (`core_exchange.txt:446-450`, `619-623`) has four phase
functions — `match_in`, `match_partial`, `match_fok_check`,
`match_stp_apply` — around `MatchOut { fills, rest, rejects }` and
`MatchError { Halted, Escrow, Conserve, Empty }`. The build:

- **One entry point.** `cross` runs the whole loop: partial fills, self-match
  prevention and the FOK probe happen inside it. It does not settle — escrow
  settles the trades it returns.
- **`Crossing` instead of `MatchOut`.** `trades`/`remaining` correspond to
  `fills`/`rest`. There is no `rejects`: turning an outcome into a rejection,
  such as an unfilled FOK, is `exchange_core`'s job.
- **A different `MatchError`.** `Halted` and `Escrow` are checked by the
  caller before `cross` runs, and `Empty` is an ordinary no-cross.
  `PostOnlyWouldTake` is the one reachable variant; `Quantity`,
  `BookDesynchronized` and `Conservation` guard states the loop cannot produce
  and are returned rather than panicked on.
- `Crossing::is_complete`, `Crossing::filled` and `would_take` are not in the
  proposal.
- `SelfMatchPolicy` is declared in `exchange_stp` and re-exported; see
  [`../../../exchange_stp/docs/item/readme.md`](../../../exchange_stp/docs/item/readme.md)
  for its variants. `SelfMatchCancellation` stays here, with the outcome it
  is part of.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:446-450` | Crate 16 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:619-623` | Crate `exchange_match`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The built shape this table describes |
