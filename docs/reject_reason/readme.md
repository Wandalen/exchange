# reject_reason

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002 — the closed set of reasons an order can be refused.

**Design status** (collection-level): none of these 9 proposed variants exist by name in the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` now only re-exports it), whose own 6 variants (`ZeroQuantity`, `NegativePrice`, `UnknownAccount`, `InsufficientFunds`, `ObligationUnrepresentable`, `ReservationUnrepresentable`) cover a narrower, currently-reachable slice of this proposed set — see each instance file's own Design status for the specific correspondence, where one exists. This collection stays central rather than redistributing into `exchange_fill`: each variant names a different proposed *detecting* crate (`exchange_idem`, `exchange_cap`, `exchange_halt`, `exchange_spec`, …), and Prompt 9 itself (`../../../../codename_space_sandbox/intake/core_exchange.txt:992`) scopes these instances to the vocabulary only — "API names belong only under exposed_item." The API-ownership fact (that the proposal assigns the full 9-variant `RejectReason` to `exchange_fill` specifically) is recorded there: [`../exposed_item/013_exchange_fill_items.md`](../exposed_item/013_exchange_fill_items.md) and [`../../module/exchange_fill/docs/item/readme.md`](../../module/exchange_fill/docs/item/readme.md).

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_duplicate.md`](001_duplicate.md) | Retry of an already-placed `OrderId` |
| [`002_full.md`](002_full.md) | Book at its capacity cap |
| [`003_halted.md`](003_halted.md) | Instrument's matching frozen |
| [`004_snap.md`](004_snap.md) | Price/quantity off the tick/lot grid |
| [`005_stp.md`](005_stp.md) | Self-trade policy fired |
| [`006_fok.md`](006_fok.md) | Fill-Or-Kill could not fill in full |
| [`007_escrow.md`](007_escrow.md) | Escrow hold failed |
| [`008_overflow.md`](008_overflow.md) | Inbound ring was full |
| [`009_unknown.md`](009_unknown.md) | Catch-all for an unclassified cause |
