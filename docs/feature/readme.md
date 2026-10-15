# feature

Workstream 002's "Features" (30) the design specifies, from `core_exchange.txt`'s Prompt 1 answer, with titles cross-checked against Prompt 9's English re-listing.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_instrumentid_orderid_accountid.md`](001_instrumentid_orderid_accountid.md) | Plain ids for instrument, order, and account |
| [`002_side.md`](002_side.md) | Bid/ask as one type, not a bool |
| [`003_order.md`](003_order.md) | One record for a resting or taker order |
| [`004_instrumentspec.md`](004_instrumentspec.md) | One record for an instrument's tradeable grid |
| [`005_price_snap_qty_snap.md`](005_price_snap_qty_snap.md) | Snap a price/quantity onto its instrument's grid |
| [`006_book.md`](006_book.md) | One instrument's resting bid and ask ladders |
| [`007_price_levels_fifo.md`](007_price_levels_fifo.md) | Group resting orders by price, FIFO within a level |
| [`008_book_rest_book_cancel.md`](008_book_rest_book_cancel.md) | Insert an order into the book, or remove one |
| [`009_match_in.md`](009_match_in.md) | Cross a taker against the opposite ladder |
| [`010_partial_fill_rest_leftover.md`](010_partial_fill_rest_leftover.md) | Rest whatever a match didn't consume |
| [`011_escrowport.md`](011_escrowport.md) | The hold_try/hold_release/hold_commit trait boundary |
| [`012_fill.md`](012_fill.md) | One record of a match |
| [`013_reject.md`](013_reject.md) | An order refusal that names why |
| [`014_cancelack.md`](014_cancelack.md) | Confirm a cancel took effect |
| [`015_stp_policy.md`](015_stp_policy.md) | allow / cancel-oldest / cancel-newest |
| [`016_tif_gtc_ioc_fok.md`](016_tif_gtc_ioc_fok.md) | GTC, IOC, FOK as explicit order data |
| [`017_depth_top.md`](017_depth_top.md) | Top-N book depth in one call |
| [`018_book_halt_book_resume.md`](018_book_halt_book_resume.md) | Freeze and unfreeze matching on one instrument |
| [`019_unique_orderid_per_book.md`](019_unique_orderid_per_book.md) | Refuse a duplicate OrderId already live |
| [`020_bookcaps_and_full.md`](020_bookcaps_and_full.md) | A named limit, and a named error when hit |
| [`021_conservation_assert.md`](021_conservation_assert.md) | Check a set of fills sums to zero |
| [`022_sorted_price_walk.md`](022_sorted_price_walk.md) | Walk prices in sorted order, not map order |
| [`023_bookstats.md`](023_bookstats.md) | Count rests, fills, rejects for one call |
| [`024_all_price_qty_are_006_types.md`](024_all_price_qty_are_006_types.md) | One arithmetic substrate for every number here |
| [`025_book_replace.md`](025_book_replace.md) | Replace an order's terms atomically |
| [`026_event_drain_for_010.md`](026_event_drain_for_010.md) | Hand 010 fills/rejects/cancel-acks, once |
| [`027_book_snapshot_rows.md`](027_book_snapshot_rows.md) | Plain snapshot rows, tick from the caller |
| [`028_sequence_no_wall_clock.md`](028_sequence_no_wall_clock.md) | Time priority from a counter, not a clock |
| [`029_inbound_flush_drain_then_rest_or_match.md`](029_inbound_flush_drain_then_rest_or_match.md) | TLS flush, ring drain, then rest or match |
| [`030_overflow_to_reject.md`](030_overflow_to_reject.md) | A full ring becomes a named Reject |
