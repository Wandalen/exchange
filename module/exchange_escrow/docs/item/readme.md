# item

The exposed surface of `exchange_escrow`, as built.

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Conserved` | trait | `const NOTHING: Self; fn checked_plus(self, rhs: Self) -> Option<Self>; fn checked_minus(self, rhs: Self) -> Option<Self>` |
| `Holding<T: Conserved>` | struct | `fn new(available: T) -> Self; fn available(&self) -> T; fn reserved(&self) -> T; fn total(&self) -> Result<T, EscrowError>` — the four edges that move value (`reserve`/`release`/`deliver`/`receive`) are private to this crate |
| `Account` | struct | `{ pub cash: Holding<Money>, pub asset: Holding<Quantity> }` |
| `EscrowError` | enum | `UnknownAccount(AccountId) \| Insufficient \| NotReserved \| NoReservation(OrderId) \| AlreadyReserved(OrderId) \| NotOwner(OrderId) \| ObligationMismatch \| NegativeAmount \| Arithmetic \| Obligation(TypeError)` |
| `Escrow` | struct | `fn new() -> Self; fn open(&mut self, AccountId, Money, Quantity) -> Result<(), EscrowError>; fn account(&self, AccountId) -> Option<&Account>; fn reserved_for(&self, OrderId) -> Option<Obligation>; fn reservation_count(&self) -> usize; fn reserve(&mut self, &Order) -> Result<Obligation, EscrowError>; fn release(&mut self, AccountId, OrderId) -> Result<Obligation, EscrowError>; fn settle(&mut self, &Trade) -> Result<(), EscrowError>; fn total_cash(&self) -> Result<Money, EscrowError>; fn total_asset(&self) -> Result<Quantity, EscrowError>` |

### Differs from the proposal

The source design (`../../../../../../codename_space_sandbox/intake/core_exchange.txt:597-600`,
catalogued at
[`../../../../docs/exposed_item/012_exchange_escrow_items.md`](../../../../docs/exposed_item/012_exchange_escrow_items.md))
names a thin port — `Hold { account, asset, amount }`,
`trait EscrowPort { hold_try, hold_release, hold_commit }`,
`EscrowError { Insufficient, UnknownHold, AlreadyCommitted }` — implemented by
a workstream 010 crate, with 002 holding no balances.

- **`Escrow` holds real balances** (`open`, `total_cash`, `total_asset`)
  rather than leaving them to 010 — a closed decision, see
  [`../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md`](../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md)
  and [`../../../../docs/boundary/002_out.md`](../../../../docs/boundary/002_out.md).
- **The port is `exchange_core::EscrowPort`**, which `Escrow` implements;
  the facade reaches escrow only through it.
- **Renamed:** `reserve` ≈ `hold_try`, `release` ≈ `hold_release`, `settle`
  ≈ `hold_commit`. `settle` also returns a buyer's price improvement, which
  the proposal does not name.
- **`EscrowError`** has the 10 variants above; only `Insufficient` and
  `AlreadyReserved` (≈ `AlreadyCommitted`) have a proposed counterpart.
