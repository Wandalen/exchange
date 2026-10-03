# Exposed Item: exchange_match

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_match`.
- **Responsibility**: Crossing a taker against the opposite ladder, with partials, TIF, and STP.

**Design status**: Built as the real `exchange_match` crate (`module/exchange_match/src/lib.rs`), with one entry point replacing a decomposed set:
- Proposed: `MatchOut { fills, rest, rejects }`, `match_in`, `match_partial`, `match_fok_check`, `match_stp_apply`, `MatchError { Halted, Escrow, Conserve, Empty }`.
- Real: `Crossing { trades, remaining, cancelled }` (≈ `MatchOut`, minus `rejects` — rejection is a facade-level concern, not this crate's), a single `cross(book, incoming, policy)` function doing the whole crossing loop (≈ `match_in`, with `match_partial` folded in as internal loop behavior rather than a separate call), plus `SelfMatchPolicy`/`SelfMatchCancellation` (≈ `match_stp_apply`'s role, applied inline during `cross` rather than as a separate pass).
- **Superseded note, corrected in this pass**: `match_fok_check` does not exist as a *separate, named function* — but FOK/TIF is no longer unimplemented. It is folded directly into `cross()`: `tif_requires_full(incoming.tif)` (from `exchange_tif`) gates a probe-then-commit sequence, the same way `match_partial` is folded in above rather than left as its own call. See [`../../module/exchange_match/docs/item/readme.md`](../../module/exchange_match/docs/item/readme.md) for the full, verified comparison and this crate's own `docs/pitfall/001_fok_that_fills_part_then_rejects.md`/`tests/tif_test.rs` for the behavior and its tests.
- `MatchError { Quantity(KindError), BookDesynchronized }` — 2 variants, neither matching the proposal's `Halted`/`Escrow`/`Conserve`/`Empty` by name; each variant's own doc comment in `module/exchange_match/src/lib.rs` explains why it is practically unreachable through normal use (not a fixed line range here, since both the enum and its neighbors have moved during this family's active TIF/book rework — see the enum's own doc comments directly).
- `Crossing::is_complete()` and `Crossing::filled()` are real convenience methods the proposal never named.

### Statement

Prompt 3 specifies four separate match-phase functions. The real `cross()` is one function doing validate-cross-settle-STP-TIF as a single internal loop, returning a `Crossing` result type close in shape to the proposed `MatchOut` but with a different, narrower `MatchError` set. FOK is handled (probe-then-commit, gated by `tif_requires_full`); IOC needs no special case since `cross` has never rested any remainder for any TIF.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:619-623` | Crate `exchange_match`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
