# exposed_item

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:958-964`) names for workstream 002 — "API names belong only under exposed_item" (`core_exchange.txt:992`). Prompt 3 gives, per crate, every exposed struct/enum/trait/function/error; Prompt 9 then dumps the same content as one flat list. This collection follows the sibling workstream 006 family's precedent (`/home/user1/pro/lib/yrd_gamedev/substrate/exact/docs/type/`) of one file per crate rather than one file per individual item — 23 files, matching the 23 crates in [`../crate/`](../crate/readme.md).

Every file also carries a verified **Design status** comparing the proposed surface against the real crate at `/home/user1/pro/lib/yrd_gamedev/substrate/exchange/module/` (where one exists) — read directly from each real crate's `src/lib.rs`, not inferred.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_exchange_id_items.md`](001_exchange_id_items.md) | Plain identity types — folded into `exchange_types`, no `InstrumentId` |
| [`002_exchange_side_items.md`](002_exchange_side_items.md) | Bid/ask — real enum is `Buy`/`Sell` |
| [`003_exchange_tif_items.md`](003_exchange_tif_items.md) | Time-in-force — not built |
| [`004_exchange_stp_items.md`](004_exchange_stp_items.md) | Self-trade policy — real variants renamed and narrower |
| [`005_exchange_spec_items.md`](005_exchange_spec_items.md) | Instrument spec and snap — not built |
| [`006_exchange_order_items.md`](006_exchange_order_items.md) | Order record — real struct missing `tif`/`instrument`/`seq` |
| [`007_exchange_seq_items.md`](007_exchange_seq_items.md) | Sequence — folded into `exchange_types`, no public constructor |
| [`008_exchange_level_items.md`](008_exchange_level_items.md) | One price FIFO — no `Level` type exists |
| [`009_exchange_book_items.md`](009_exchange_book_items.md) | Sorted ladders — built, narrower surface, no `BookError` |
| [`010_exchange_cap_items.md`](010_exchange_cap_items.md) | Caps — not built |
| [`011_exchange_idem_items.md`](011_exchange_idem_items.md) | Idempotent id set — not built as a crate, partial guard inside `Book::insert` |
| [`012_exchange_escrow_items.md`](012_exchange_escrow_items.md) | Escrow port — built, scope-expanded to real balances |
| [`013_exchange_fill_items.md`](013_exchange_fill_items.md) | Fill and reject types — folded into `exchange_types` as `Trade`/`EventKind` |
| [`014_exchange_conserve_items.md`](014_exchange_conserve_items.md) | Sum-to-zero — folded into `exchange_escrow`'s balance totals |
| [`015_exchange_rest_items.md`](015_exchange_rest_items.md) | Rest, cancel, replace — no `replace` exists |
| [`016_exchange_match_items.md`](016_exchange_match_items.md) | Match — built as one `cross()` entry point |
| [`017_exchange_depth_items.md`](017_exchange_depth_items.md) | Depth — not built |
| [`018_exchange_halt_items.md`](018_exchange_halt_items.md) | Halt — not built |
| [`019_exchange_event_items.md`](019_exchange_event_items.md) | Event drain — folded into `exchange_types`, richer `EventKind` |
| [`020_exchange_snap_items.md`](020_exchange_snap_items.md) | Snapshot rows — not built |
| [`021_exchange_stats_items.md`](021_exchange_stats_items.md) | Stats — not built |
| [`022_exchange_inbound_items.md`](022_exchange_inbound_items.md) | Ring ingress — not built, zero `ring_*` dependency anywhere |
| [`023_exchange_core_items.md`](023_exchange_core_items.md) | Facade — built over 4 crates, not the proposed 10 |
