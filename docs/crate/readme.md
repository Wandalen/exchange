# crate

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:958-964`) names for workstream 002. Per-crate dependency/boundary specs for the 23 proposed crates, from Prompt 2's "List crates" answer (`../../../../codename_space_sandbox/intake/core_exchange.txt:351-527`).

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_exchange_id.md`](001_exchange_id.md) | Plain id, without pointers |
| [`002_exchange_side.md`](002_exchange_side.md) | Bid and ask as one type |
| [`003_exchange_tif.md`](003_exchange_tif.md) | GTC, IOC, FOK as data |
| [`004_exchange_stp.md`](004_exchange_stp.md) | The self-trade policy |
| [`005_exchange_spec.md`](005_exchange_spec.md) | Tick, lot, asset pair, halt on the instrument |
| [`006_exchange_order.md`](006_exchange_order.md) | One record for a rest or a taker |
| [`007_exchange_seq.md`](007_exchange_seq.md) | Monotonic sequence for time priority, no wall clock |
| [`008_exchange_level.md`](008_exchange_level.md) | One price, FIFO rest |
| [`009_exchange_book.md`](009_exchange_book.md) | Sorted bid and ask ladders for one instrument |
| [`010_exchange_cap.md`](010_exchange_cap.md) | Limit on rests and levels; Full is an error |
| [`011_exchange_idem.md`](011_exchange_idem.md) | OrderId seen once per book |
| [`012_exchange_escrow.md`](012_exchange_escrow.md) | Hold/release/commit port |
| [`013_exchange_fill.md`](013_exchange_fill.md) | Fill, reject, cancel-ack |
| [`014_exchange_conserve.md`](014_exchange_conserve.md) | A set of fills sums to zero |
| [`015_exchange_rest.md`](015_exchange_rest.md) | Rest, cancel, replace |
| [`016_exchange_match.md`](016_exchange_match.md) | Taker against the opposite ladder, partials, TIF, STP |
| [`017_exchange_depth.md`](017_exchange_depth.md) | Top-N without walking every rest |
| [`018_exchange_halt.md`](018_exchange_halt.md) | Freeze the match; rests remain |
| [`019_exchange_event.md`](019_exchange_event.md) | Drain fills, rejects, cancel-acks for 010 |
| [`020_exchange_snap.md`](020_exchange_snap.md) | Plain rows of the rests, and the tick, from the caller |
| [`021_exchange_stats.md`](021_exchange_stats.md) | Rests, fills from this call, rejects |
| [`022_exchange_inbound.md`](022_exchange_inbound.md) | The single bridge to workstream 008 |
| [`023_exchange_core.md`](023_exchange_core.md) | The facade |
