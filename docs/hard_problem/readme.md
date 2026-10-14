# hard_problem

Workstream 002's "Hard problems" (24) the design exists to solve, from `core_exchange.txt`'s Prompt 1 answer.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_one_book_per_instrument.md`](001_one_book_per_instrument.md) | Keep every instrument's resting orders in its own book |
| [`002_price_time.md`](002_price_time.md) | Rank resting orders by price first, then arrival order |
| [`003_partial_fills.md`](003_partial_fills.md) | Let a taker fill against more than one resting order |
| [`004_escrow_before_rest.md`](004_escrow_before_rest.md) | Reserve funds before an order rests |
| [`005_conservation.md`](005_conservation.md) | Make every fill's two legs sum to exactly zero |
| [`006_deterministic_match.md`](006_deterministic_match.md) | Make the same book plus input always produce the same fills |
| [`007_no_float.md`](007_no_float.md) | Represent price/quantity as 006 minor-unit types, never float |
| [`008_tick_and_lot.md`](008_tick_and_lot.md) | Reject a price or quantity off the instrument's grid |
| [`009_cancel_and_replace.md`](009_cancel_and_replace.md) | Withdraw or change a resting order without a stale remnant |
| [`010_self_trade.md`](010_self_trade.md) | Apply one named policy when an account would trade itself |
| [`011_inbound_at_aeon_edge.md`](011_inbound_at_aeon_edge.md) | Receive orders as an already-drained slice at the aeon edge |
| [`012_events_not_wallets.md`](012_events_not_wallets.md) | Emit Fill/Reject/CancelAck; leave balances to 010 |
| [`013_depth.md`](013_depth.md) | Answer top-N depth without walking every resting order |
| [`014_hot_path.md`](014_hot_path.md) | Keep per-instrument, per-aeon matching cheap |
| [`015_idempotent_order_id.md`](015_idempotent_order_id.md) | Treat a retried OrderId as the same order |
| [`016_closed_types.md`](016_closed_types.md) | Keep ids and the book as plain data, no host pointers |
| [`017_capacity.md`](017_capacity.md) | Enforce a resting-order cap; Full is an error, not a drop |
| [`018_halt.md`](018_halt.md) | Freeze matching while resting orders stay in place |
| [`019_more_than_one_asset.md`](019_more_than_one_asset.md) | Let an instrument name a distinct cash and commodity asset |
| [`020_time_in_force.md`](020_time_in_force.md) | Carry GTC/IOC/FOK as explicit order data |
| [`021_maker_and_taker_on_fill.md`](021_maker_and_taker_on_fill.md) | Name both sides and the price on every fill |
| [`022_snapshot_of_book.md`](022_snapshot_of_book.md) | Represent resting orders as copyable rows |
| [`023_full_order_of_inbound.md`](023_full_order_of_inbound.md) | Give the ring's drain a stable total order before matching |
| [`024_ring_overflow_is_reject.md`](024_ring_overflow_is_reject.md) | Turn a full ring into an explicit Reject, never a silent drop |
