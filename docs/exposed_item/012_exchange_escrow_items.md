# Exposed Item: exchange_escrow

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_escrow`.
- **Responsibility**: The hold/release/commit port between the book and a wallet.

**Design status**: Built as the real `exchange_escrow` crate (`module/exchange_escrow/src/lib.rs`), scope-expanded well beyond a thin port:
- Proposed: `Hold { account, asset, amount }`, `trait EscrowPort { hold_try, hold_release, hold_commit }`, `EscrowError { Insufficient, UnknownHold, AlreadyCommitted }`.
- Real: `Conserved` (trait, generic arithmetic contract — not the port itself), `Holding<T> { available, reserved }`, `Account { cash, asset }`, `Escrow { new, open, account, reserved_for, reservation_count, reserve, release, settle, total_cash, total_asset }`, and a 9-variant `EscrowError { UnknownAccount, Insufficient, NotReserved, NoReservation, AlreadyReserved, ObligationMismatch, NegativeAmount, Arithmetic, Obligation }`.
- `EscrowPort` is not a trait a future crate implements — `Escrow` is a concrete struct that **holds real account balances itself** (`open()` deposits cash/asset; `total_cash()`/`total_asset()` sum them). This is the family's clearest boundary divergence — see `../boundary/002_out.md` and `../neighbor_contract/003_010_implements_escrowport.md`.
- Method names differ: `reserve()` ≈ `hold_try`, `release()` ≈ `hold_release`, `settle()` ≈ `hold_commit` — functionally analogous, three-edge shape intact, but renamed to describe the ledger operation rather than the lock/unlock metaphor.
- `settle()` additionally does what the proposal splits into `exchange_conserve`'s job (see `../exposed_item/014_exchange_conserve_items.md`) — the "give back the buyer's price improvement" logic (`exchange_escrow/src/lib.rs:516-607`) has no counterpart named anywhere in the proposal at all.
- `EscrowError::Insufficient` and `AlreadyReserved`(≈`AlreadyCommitted`) are the two variants with a reasonably direct proposed counterpart; the other 7 are real-build additions with no proposed equivalent.

### Statement

Prompt 3 specifies a minimal three-method port a future crate implements. The real `Escrow` is a concrete, standalone ledger that owns real account balances and does its own conservation bookkeeping — the single largest surface expansion in the family relative to its own design proposal.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:597-600` | Crate `exchange_escrow`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
