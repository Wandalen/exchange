# item

The exposed surface of `exchange_match`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface, including the `SelfMatchPolicy` re-export.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Crossing` | struct | `{ trades: Vec<Trade>, remaining: Quantity, cancelled: Vec<SelfMatchCancellation> }` |
| `Crossing::is_complete` | fn | `(&self) -> bool` |
| `Crossing::filled` | fn | `(&self) -> Result<Quantity, KindError>` |
| `SelfMatchCancellation` | struct | `{ order: OrderId, account: AccountId, quantity: Quantity }` (`Copy`) |
| `MatchError` | enum | `{ Quantity(KindError), BookDesynchronized, Conservation(ConserveError), PostOnlyWouldTake }` — plus `Display`, `core::error::Error`, `From<KindError>`, `From<ConserveError>`; the third variant added when `exchange_conserve::conserve_assert` was wired into `cross_inner`, as defense-in-depth; the fourth with `Tif::PostOnly` |
| `SelfMatchPolicy` | enum | `pub use exchange_stp::SelfMatchPolicy;` — `{ CancelResting, CancelIncoming, CancelBoth }`, re-exported unchanged |
| `cross` | fn | `(book: &mut Book, incoming: &Order, policy: SelfMatchPolicy) -> Result<Crossing, MatchError>` |
| `would_take` | fn | `(book: &Book, incoming: &Order) -> bool` |

### Differs from the proposal

Verified directly against `core_exchange.txt:446-450` (crate 16, Prompt 2)
and `core_exchange.txt:619-623` (exposed-item list, Prompt 3) — not against
the central `docs/crate/016_exchange_match.md`/`docs/exposed_item/016_exchange_match_items.md`
summaries; the latter's claim that FOK/TIF is "unimplemented family-wide" is
now stale (see below) and has been corrected in place to point here.

The proposal specifies four phase functions (`match_in`, `match_partial`,
`match_fok_check`, `match_stp_apply`) around a `MatchOut { fills, rest,
rejects }` result and `MatchError { Halted, Escrow, Conserve, Empty }`. The
real build:

- Replaces all four phase functions with **one entry point**, `cross(book,
  incoming, policy)`, doing validate→cross→settle→STP as a single internal
  loop (`cross_inner`). `match_partial` is folded in as ordinary loop
  behavior rather than a separate call.
- Returns `Crossing { trades, remaining, cancelled }` — `trades`/`remaining`
  ≈ `MatchOut`'s `fills`/`rest`; no `rejects` field, since rejection (e.g.
  turning an unfillable FOK into an actual reject code) is a facade-level
  concern this crate only reports the raw outcome for, per its own module
  doc's "Time-in-force" section.
- **`match_fok_check` is folded into `cross` itself, not absent.** FOK *is*
  implemented: `cross` gates on `tif_requires_full(incoming.tif)` (from
  `exchange_tif`) and runs a probe-then-commit sequence — the crossing loop
  executes once against a disposable clone of `book`, and only on a full
  fill does the identical, deterministic sequence run again against the
  real `book`. Covered by this crate's own
  [`docs/pitfall/001_fok_that_fills_part_then_rejects.md`](../pitfall/001_fok_that_fills_part_then_rejects.md)
  and exercised by all 8 cases in
  [`tests/tif_test.rs`](../../tests/tif_test.rs). IOC needs no special
  handling in `cross` at all — this function has never inserted the
  incoming order's own remainder for any TIF, so "drop any remainder" was
  already the behavior; only whether a *caller* later rests that remainder
  differs, which is `exchange_rest`'s/`exchange_core`'s decision.
- **Post-only is refused in `cross`.** A `Tif::PostOnly` order for which
  `would_take` holds returns `MatchError::PostOnlyWouldTake` with `book`
  untouched — the one reachable `MatchError`. `exchange_core` checks
  `would_take` first, so it reports a rejection instead.
- `MatchError { Quantity(KindError), BookDesynchronized }` — 2 variants,
  neither matching the proposal's `Halted`/`Escrow`/`Conserve`/`Empty` by
  name. Each variant's own doc comment states why it is practically
  unreachable through normal use: `Quantity` only because every quantity
  here is a split of the incoming order's own already-valid quantity;
  `BookDesynchronized` only because it would mean this loop and `Book`
  disagree about what rests, which this crate returns as an error rather
  than panics on, so a caller in that state learns about it rather than
  losing the process mid-match.
- `Crossing::is_complete()` and `Crossing::filled()` are real convenience
  methods the proposal never named.
- `SelfMatchPolicy` itself is not declared here — it moved to its own root
  crate, `exchange_stp`, and is re-exported unchanged so existing callers
  are unaffected; see
  [`../../../exchange_stp/docs/item/readme.md`](../../../exchange_stp/docs/item/readme.md)
  for why its three variants (`CancelResting`/`CancelIncoming`/
  `CancelBoth`) differ from the proposal's `Allow`/`CancelOldest`/
  `CancelNewest`. `SelfMatchCancellation` stays in this crate rather than
  moving with the policy — it carries this crate's own
  `AccountId`/`OrderId`/`Quantity`, not pure self-match-policy vocabulary,
  so it belongs with the match outcome type it travels inside
  (`Crossing::cancelled`).

No `docs/decisions/` collection: every divergence above is already argued
at its own site in `src/lib.rs`'s module doc (the "Time-in-force" and
"What this crate does not decide" sections) or in `exchange_stp`'s own
decisions collection for the one item that moved there.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:446-450` | Crate 16 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:619-623` | Crate `exchange_match`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
