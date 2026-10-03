# item

The exposed surface of `exchange_escrow`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; same as
`exchange_id`'s own `docs/item/readme.md`).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Conserved` | trait | `const NOTHING: Self; fn checked_plus(self, rhs: Self) -> Option<Self>; fn checked_minus(self, rhs: Self) -> Option<Self>` |
| `Holding<T: Conserved>` | struct | `fn new(available: T) -> Self; fn available(&self) -> T; fn reserved(&self) -> T; fn total(&self) -> Result<T, EscrowError>` — the four edges that move value (`reserve`/`release`/`deliver`/`receive`) are private to this crate |
| `Account` | struct | `{ pub cash: Holding<Money>, pub asset: Holding<Quantity> }` |
| `EscrowError` | enum | `UnknownAccount(AccountId) \| Insufficient \| NotReserved \| NoReservation(OrderId) \| AlreadyReserved(OrderId) \| ObligationMismatch \| NegativeAmount \| Arithmetic \| Obligation(TypeError)` |
| `Escrow` | struct | `fn new() -> Self; fn open(&mut self, AccountId, Money, Quantity) -> Result<(), EscrowError>; fn account(&self, AccountId) -> Option<&Account>; fn reserved_for(&self, OrderId) -> Option<Obligation>; fn reservation_count(&self) -> usize; fn reserve(&mut self, &Order) -> Result<Obligation, EscrowError>; fn release(&mut self, AccountId, OrderId) -> Result<Obligation, EscrowError>; fn settle(&mut self, &Trade, Side, Price) -> Result<(), EscrowError>; fn total_cash(&self) -> Result<Money, EscrowError>; fn total_asset(&self) -> Result<Quantity, EscrowError>` |

### Differs from the proposal

The source design's own exposed-item list for crate 12
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:597-600`,
also catalogued centrally at
[`../../../../docs/exposed_item/012_exchange_escrow_items.md`](../../../../docs/exposed_item/012_exchange_escrow_items.md))
names a thin port: `Hold { account, asset, amount }`, `trait EscrowPort { hold_try, hold_release, hold_commit }`, and a 3-variant
`EscrowError { Insufficient, UnknownHold, AlreadyCommitted }` — implemented by
a future workstream 010 crate, with 002 holding no real balances at all (→
[`../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md`](../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md),
[`../../../../docs/boundary/002_out.md`](../../../../docs/boundary/002_out.md)).

This is a real, already-made, deliberate divergence, not a gap: the real
`Escrow` is a concrete, standalone ledger that holds real `Account` balances
itself (`open`, `total_cash`, `total_asset`) rather than a port something
else implements. `reserve()` ≈ `hold_try`, `release()` ≈ `hold_release`,
`settle()` ≈ `hold_commit` — functionally analogous, three-edge shape intact,
renamed to describe the ledger operation rather than the lock/unlock
metaphor. `EscrowError` grew from 3 proposed variants to the 9 above; only
`Insufficient` and `AlreadyReserved` (≈ `AlreadyCommitted`) have a direct
proposed counterpart. `settle()` additionally returns the buyer's price
improvement — logic the proposal attributes to `exchange_conserve` (→
[`../../../../docs/exposed_item/014_exchange_conserve_items.md`](../../../../docs/exposed_item/014_exchange_conserve_items.md))
— with no counterpart named anywhere in the proposal.

Don't re-litigate this here: the decision is closed centrally, cited above.
