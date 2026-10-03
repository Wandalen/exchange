# exposed_item

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:958-964`) names for workstream 002 — "API names belong only under exposed_item" (`core_exchange.txt:992`). Prompt 3 gives, per crate, every exposed struct/enum/trait/function/error; Prompt 9 then dumps the same content as one flat list. This collection follows the sibling workstream 006 family's precedent (`/home/user1/pro/lib/yrd_gamedev/substrate/exact/docs/type/`) of one file per crate rather than one file per individual item — 23 files, matching the 23 crates in [`../crate/`](../crate/readme.md).

Every file also carries a verified **Design status** comparing the proposed surface against the real crate at `/home/user1/pro/lib/yrd_gamedev/substrate/exchange/module/` (where one exists) — read directly from each real crate's `src/lib.rs`, not inferred.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_exchange_id_items.md`](001_exchange_id_items.md) | Built, own crate — matches except declined `IdError` |
| [`002_exchange_side_items.md`](002_exchange_side_items.md) | Built, own crate — real enum is `Buy`/`Sell`, not renamed |
| [`003_exchange_tif_items.md`](003_exchange_tif_items.md) | Built, own crate — full match |
| [`004_exchange_stp_items.md`](004_exchange_stp_items.md) | Built, own crate — no `Allow`; real three variants renamed |
| [`005_exchange_spec_items.md`](005_exchange_spec_items.md) | Built, own crate — full structural match |
| [`006_exchange_order_items.md`](006_exchange_order_items.md) | Built, own crate — `instrument`/`tif` both present now |
| [`007_exchange_seq_items.md`](007_exchange_seq_items.md) | Built, own crate — matches except declined `SeqError`/`seq_cmp` |
| [`008_exchange_level_items.md`](008_exchange_level_items.md) | Built, own crate — all 7 `level_*` functions, only `LevelError` declined |
| [`009_exchange_book_items.md`](009_exchange_book_items.md) | Built, multi-instrument now — narrower surface, no `BookError` |
| [`010_exchange_cap_items.md`](010_exchange_cap_items.md) | Built, own crate — full match, zero divergence |
| [`011_exchange_idem_items.md`](011_exchange_idem_items.md) | Built, own crate — matches exactly including `idem_remove` |
| [`012_exchange_escrow_items.md`](012_exchange_escrow_items.md) | Built — scope-expanded to real balances (deliberate decision) |
| [`013_exchange_fill_items.md`](013_exchange_fill_items.md) | Built, own crate — `Trade`/`Event`/`RejectReason`/`CancelCause` extracted from `exchange_types` |
| [`014_exchange_conserve_items.md`](014_exchange_conserve_items.md) | Built, own crate — pure function, not extracted from `exchange_escrow`'s totals (deliberate) |
| [`015_exchange_rest_items.md`](015_exchange_rest_items.md) | Built, own crate — `rest_place`/`rest_cancel` thin wrappers; `rest_replace` new, narrower `RestReplaceError` |
| [`016_exchange_match_items.md`](016_exchange_match_items.md) | Built — `cross()`, now TIF/STP-aware |
| [`017_exchange_depth_items.md`](017_exchange_depth_items.md) | Built, own crate — full match |
| [`018_exchange_halt_items.md`](018_exchange_halt_items.md) | Built, own crate — declined `exchange_book` dependency |
| [`019_exchange_event_items.md`](019_exchange_event_items.md) | Built, own crate — `push`/`drain`/`len`/`clear`; declined `EventDrain`/`EventError` |
| [`020_exchange_snap_items.md`](020_exchange_snap_items.md) | Built, own crate — full field match; declined `exchange_order` dep/`SnapError` |
| [`021_exchange_stats_items.md`](021_exchange_stats_items.md) | Built, own crate — includes `cancels` counter |
| [`022_exchange_inbound_items.md`](022_exchange_inbound_items.md) | Built — `InboundCmd` + flush/drain/apply/overflow_reject; no standalone `Inbound`/`InboundError` types |
| [`023_exchange_core_items.md`](023_exchange_core_items.md) | Built — facade over 8 real crates |
