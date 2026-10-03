# Exposed Item: exchange_rest

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_rest`.
- **Responsibility**: Placing, cancelling, and replacing a resting order.

**Design status**: `exchange_rest` is now built as its own real crate
(verified via direct read of `module/exchange_rest/src/lib.rs`), alongside
— not instead of — the pre-existing equivalents still orchestrated inline
elsewhere:
- `rest_place` is a direct call-through to `Book::insert` (`exchange_book`);
  `Exchange::submit`'s step 5 (`exchange_core/src/lib.rs:348-360`) still
  orchestrates its own idempotency/cap/escrow checks around the same
  book-level move, unchanged.
- `rest_cancel` is a direct call-through to `Book::cancel` (`exchange_book`);
  `Exchange::cancel` (`exchange_core/src/lib.rs:391-407`) likewise still
  performs its own escrow release inline — a step the proposal's
  `exchange_rest` crate (dependent only on `book, idem, cap, escrow, spec`)
  implies but does not spell out as one atomic caller-facing operation the
  way the real `Exchange::cancel` does.
- **`rest_replace` is new, and built**: an atomic cancel-then-reinsert with
  rollback on refusal. The central catalog's own former note that this
  "does not exist anywhere" predates this crate's real build and is now
  stale — see the thinned pointer at
  [`../crate/015_exchange_rest.md`](../crate/015_exchange_rest.md).
- `RestError { Duplicate, Missing, Full, Halted, Escrow, Snap }` still does
  not exist as its own type, and `rest_place`/`rest_cancel` still need no
  error type of their own — they return the same `bool`/`Option<Resting>`
  `Book::insert`/`Book::cancel` already do. `rest_replace` instead carries
  its own, narrower `RestReplaceError { Missing, Refused }` — the two
  outcomes a cancel-then-insert wrapper can itself produce.
  `Duplicate`/`Full`/`Halted`/`Escrow`/`Snap` remain spread across
  `ExchangeError` (`Rejected`, `Escrow`, `Matching`, `NotResting`) and
  `RejectReason` as before (see `../reject_reason/`).

### Statement

Prompt 3 specifies a `rest_place`/`rest_cancel`/`rest_replace` trio with a
dedicated error type. The real build now has all three operations —
`rest_place`/`rest_cancel` as thin, error-type-free wrappers, `rest_replace`
with the narrower `RestReplaceError` — see
[`../../module/exchange_rest/docs/item/readme.md`](../../module/exchange_rest/docs/item/readme.md)
for the full as-built listing. The remaining concrete gap against the
proposal is the error surface's shape, not a missing operation (compare
`../feature/025_book_replace.md`, now closed).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:615-617` | Crate `exchange_rest`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
