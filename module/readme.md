# module

Every crate in the `substrate/exchange` workspace.

## Responsibility Table

| Directory | Responsibility |
|-----------|-----------------|
| [`exchange_id/`](exchange_id/readme.md) | `InstrumentId`/`OrderId`/`AccountId` — plain id newtypes |
| [`exchange_side/`](exchange_side/readme.md) | `Side { Buy, Sell }` and its opposite relation |
| [`exchange_seq/`](exchange_seq/readme.md) | `Sequence` and `seq_next` — time priority without a clock |
| [`exchange_cap/`](exchange_cap/readme.md) | `BookCaps` — a configurable limit on book growth |
| [`exchange_stp/`](exchange_stp/readme.md) | `SelfMatchPolicy` — how a self-match resolves |
| [`exchange_tif/`](exchange_tif/readme.md) | `Tif { Gtc, Ioc, Fok }` — time-in-force disposition |
| [`exchange_spec/`](exchange_spec/readme.md) | `InstrumentSpec` — tick, lot, asset pair, halt flag |
| [`exchange_stats/`](exchange_stats/readme.md) | `BookStats` — running rest/fill/reject/cancel counters for the hot path |
| [`exchange_types/`](exchange_types/readme.md) | `notional`/`obligation`/`TypeError`/`Price` — the settlement logic; no longer re-exports `Order`/`Trade` and friends, which now live solely in their own crates |
| [`exchange_order/`](exchange_order/readme.md) | `Order`, `Obligation` — one order record, instrument and tif included |
| [`exchange_idem/`](exchange_idem/readme.md) | `IdSet` — refuse a duplicate `OrderId` before it reaches the book |
| [`exchange_fill/`](exchange_fill/readme.md) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` — the event-stream vocabulary |
| [`exchange_conserve/`](exchange_conserve/readme.md) | `conserve_assert`/`fill_legs_sum` — a fill batch nets to zero across its legs |
| [`exchange_level/`](exchange_level/readme.md) | `Level`/`LevelNode` — one price, FIFO rest |
| [`exchange_book/`](exchange_book/readme.md) | The resting order book and price-time priority |
| [`exchange_depth/`](exchange_depth/readme.md) | `depth_top` — top-N book depth without a full walk |
| [`exchange_halt/`](exchange_halt/readme.md) | `halt_set`/`halt_clear`/`halt_is` — an on/off switch for matching |
| [`exchange_event/`](exchange_event/readme.md) | `event_drain` — the one owned drain point for the event stream |
| [`exchange_snap/`](exchange_snap/readme.md) | `snap_take` — a plain, copied snapshot of a book's resting rows |
| [`exchange_match/`](exchange_match/readme.md) | `cross()` — an incoming order against the book, producing trades |
| [`exchange_rest/`](exchange_rest/readme.md) | `rest_place`/`rest_cancel`/`rest_replace` — the non-matching ways an order moves on the book |
| [`exchange_inbound/`](exchange_inbound/readme.md) | `InboundCmd` over a `ring_factory`/`ring_handle` channel — many producers, one deterministic apply order |
| [`exchange_escrow/`](exchange_escrow/readme.md) | Available/reserved balance partition |
| [`exchange_core/`](exchange_core/readme.md) | `Exchange` — the facade tying book, match and escrow together |
| [`smoke_exchange_book/`](smoke_exchange_book/readme.md) | P30 — the wall smoke exercising every stage in one run |
| [`smoke_exchange_phases/`](smoke_exchange_phases/readme.md) | The P01-P29 phase-smoke ladder, one binary per phase |
