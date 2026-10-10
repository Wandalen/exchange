# Guide: The Exchange Crate Family

### Scope

- **Purpose**: Give a new reader the whole family's shape in one pass — what each real crate does, how to call it, and which tier it sits at — without requiring 26 separate readme visits first.
- **Responsibility**: Purpose, a runnable usage snippet, and a dependency tier for every crate/lane under `module/`.
- **In Scope**: The real, as-built `module/` tree — 24 `exchange_*` crates plus two smoke lanes — and their actual `Cargo.toml` dependency edges, verified by direct read, not by trusting any single crate's own prose.
- **Out of Scope**: The 23-crate *proposal*'s own per-item analysis (→ [`../crate/`](../crate/readme.md), [`../exposed_item/`](../exposed_item/readme.md)); any crate's full design rationale (→ that crate's own `readme.md`, linked from every entry below).

**Design status**: first guide pass, dated 2026-10-03. Every dependency edge below was re-extracted directly from each crate's own `Cargo.toml` (`awk` over every `[dependencies]` block in `module/*/Cargo.toml`), not copied from readme prose — at least three readmes (`exchange_escrow`, `exchange_core`, `exchange_snap`) understate their own crate's dependency count (`exchange_snap`'s own Responsibility Table omits `exchange_side`, confirmed present in its real `Cargo.toml`), and this guide's table reflects the verified edges, not those descriptions. `exchange_inbound` landed the same day, after this pass's first draft — folded in immediately rather than left stale, confirmed directly against its real `Cargo.toml`/`src/lib.rs` rather than taken on a status report alone. Re-run the extraction below before trusting this table if the family has grown since.

**Re-verified 2026-10-04** after the Stage 9 facade rework: the dependency edges were re-extracted from scratch (not diffed against the table above) and every "standalone" wiring claim was re-checked by grepping `exchange_core/src/lib.rs` for each crate's own name, not assumed to still hold. One tier actually changed — `exchange_core` moved from Tier 6 to Tier 7, because it gained a direct `exchange_inbound` dependency it did not have before. Eight of the Introduction's eleven previously-"standalone" crates are now called from inside `exchange_core`; three are not. See point 2 below and every per-crate bullet it points to.

**Re-verified 2026-10-05** after the `exchange_types` retirement: every `Cargo.toml` dependency edge was re-extracted from scratch and the tier of every crate recomputed mechanically (not by hand) from that graph. `exchange_types` lost its `exchange_fill`/`exchange_id`/`exchange_seq` edges — it now depends only on `exchange_order` and `exchange_side` — so it fell from Tier 3 to Tier 2. Every tier at or above the old Tier 4 shifted down by exactly one as a result: `exchange_book`/`exchange_conserve`/`exchange_escrow`/`exchange_event` (4→3), `exchange_depth`/`exchange_match`/`exchange_rest`/`exchange_snap` (5→4), `exchange_inbound` (6→5), `exchange_core` (7→6). `exchange_core` separately gained three direct dependencies it previously reached only through the `exchange_types` re-export (`exchange_fill`, `exchange_order`, `exchange_side`), raising its own dependency count from fifteen to eighteen. See point 1 below, the Tier Table, and every per-crate bullet the shift touched.

## Introduction

`module/` holds 24 `exchange_*` crates plus two smoke lanes (`smoke_exchange_book`, `smoke_exchange_phases`). Each crate does one thing; nothing here is a kitchen-sink `utils`. The family splits into dependency **tiers** — tier *N* may depend only on tiers below it — which is the one piece of structure this guide adds on top of what each crate's own readme already says. Four things worth knowing before the table:

1. **`exchange_types` was a legacy re-export aggregator — retired as of 2026-10-05.** It originally held everything in this family's original 4-crate build (`Side`, `AccountId`, `OrderId`, `Sequence`, `Order`, `Obligation`, `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause`) plus its own `Amount` alias. All twelve are gone now — every call site depends on the owning leaf crate directly — and `exchange_types` owns only what was always genuinely its own: `TypeError`, `notional()`, and `obligation()`. The four crates that used to route through the aggregator (`exchange_book`, `exchange_conserve`, `exchange_escrow`, `exchange_event`) no longer do either: `exchange_book` and `exchange_event` dropped the dependency entirely, while `exchange_conserve` and `exchange_escrow` still depend on the narrowed crate, now for `TypeError` alone. The tier distortion this point used to warn about is resolved as a side effect, not a separate fix — `exchange_types` fell from Tier 3 to Tier 2, and the four dependent crates fell from Tier 4 to Tier 3, both landing at the tier their conceptual role actually suggests.
2. **Most of the family is wired into `exchange_core` now — one crate still isn't.** The Stage 9 facade rework (2026-10-03) gave `Exchange` real methods calling straight through to `exchange_depth`, `exchange_event`, `exchange_halt`, `exchange_snap`, `exchange_spec`, and `exchange_stats`, plus `exchange_inbound` as the ring `exchange_step` itself drains; `exchange_cap` followed, checked in `step_place`'s dry run, and `exchange_idem`, refusing a retried `ClientOrderId`. Only `exchange_conserve` remains standalone — it appears nowhere in `exchange_core/src/lib.rs` (`exchange_match` calls it). `exchange_rest` is a partial case: `rest_place` is called from inside `step_place`, but `rest_replace` is reachable only directly — `exchange_step` drains an `InboundCmd::Replace` and returns `StepOutcome::ReplaceNotWired` rather than applying it (see `exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md`). This is the family's own established "skeleton-first" pattern, now mostly but not fully closed: build and prove a crate in isolation, wire it into the facade in a later stage. Call the two remaining names directly; don't expect `Exchange` to reach them yet.
3. **24 real crates against a 23-crate proposal — not a 1:1 match.** The proposal names 23 crates; the real build now has 24 `exchange_*` directories, and not the same 23 either. `exchange_types` is not one of the proposed 23 at all — it is the legacy crate everything else used to live in — and `exchange_inbound` (the ring-ingress bridge, the proposal's 23rd) landed only after this guide's first pass, built against `ring_factory`+`ring_handle`+`ring_types` rather than `ring_core` directly.
4. **`Money` and `Price` are distinct types.** `exact_arith::Price` is its own struct, not an alias of `Money`; neither converts into the other. A price is per unit, `Money` (and `exchange_order::Amount`) is a total, and `exchange_types::notional` is the one bridge between them.

## Tier Table

| Tier | Crate | One-line purpose |
|------|-------|-------------------|
| 0 — Roots | [`exchange_cap`](../../module/exchange_cap/readme.md) | A limit on book growth, with a named refusal past it |
| 0 — Roots | [`exchange_id`](../../module/exchange_id/readme.md) | Plain id newtypes — `InstrumentId`, `OrderId`, `ClientOrderId`, `AccountId` |
| 0 — Roots | [`exchange_seq`](../../module/exchange_seq/readme.md) | A monotonic sequence for time priority, no wall clock |
| 0 — Roots | [`exchange_side`](../../module/exchange_side/readme.md) | `Side { Buy, Sell }` and its opposite relation |
| 0 — Roots | [`exchange_stats`](../../module/exchange_stats/readme.md) | Running rest/fill/reject/cancel counters |
| 0 — Roots | [`exchange_stp`](../../module/exchange_stp/readme.md) | The self-trade policy, as a closed set of three |
| 0 — Roots | [`exchange_tif`](../../module/exchange_tif/readme.md) | `Tif { Gtc, Ioc, Fok, PostOnly }` — time-in-force disposition |
| 1 | [`exchange_idem`](../../module/exchange_idem/readme.md) | Refuse a key already seen — an `OrderId`, or `( AccountId, ClientOrderId )` |
| 1 | [`exchange_order`](../../module/exchange_order/readme.md) | One order record — instrument, account, side, tif included |
| 1 | [`exchange_spec`](../../module/exchange_spec/readme.md) | Tick, lot, the asset pair, and the halt flag on one instrument |
| 2 | [`exchange_fill`](../../module/exchange_fill/readme.md) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` |
| 2 | [`exchange_halt`](../../module/exchange_halt/readme.md) | Halt and resume new orders on one instrument |
| 2 | [`exchange_level`](../../module/exchange_level/readme.md) | One price, FIFO rest |
| 2 | [`exchange_types`](../../module/exchange_types/readme.md) | `TypeError`, plus `notional`/`obligation` |
| 3 | [`exchange_book`](../../module/exchange_book/readme.md) | The resting order book, one per instrument, in price-time priority |
| 3 | [`exchange_conserve`](../../module/exchange_conserve/readme.md) | A fill batch nets to zero across its legs, or is refused |
| 3 | [`exchange_escrow`](../../module/exchange_escrow/readme.md) | The reservation ledger — available/reserved partition and settlement |
| 3 | [`exchange_event`](../../module/exchange_event/readme.md) | The one owned drain point for the event stream |
| 4 | [`exchange_depth`](../../module/exchange_depth/readme.md) | Top-N book depth without a full walk |
| 4 | [`exchange_match`](../../module/exchange_match/readme.md) | `cross()` — an incoming order against the book, TIF-aware |
| 4 | [`exchange_rest`](../../module/exchange_rest/readme.md) | Rest, cancel, replace — the non-matching ways an order moves |
| 4 | [`exchange_snap`](../../module/exchange_snap/readme.md) | A plain, copied snapshot of a book's resting rows |
| 5 | [`exchange_inbound`](../../module/exchange_inbound/readme.md) | `InboundCmd` from a ring producer to the book — the family's first concurrency |
| 6 — Facade | [`exchange_core`](../../module/exchange_core/readme.md) | `Exchange` — ring-fed `exchange_step`, the facade's only intake path now |
| — (lane) | [`smoke_exchange_book`](../../module/smoke_exchange_book/readme.md) | The wall smoke, exercising every stage in one run |
| — (lane) | [`smoke_exchange_phases`](../../module/smoke_exchange_phases/readme.md) | The P01–P29 phase-smoke ladder, one binary per phase |

A crate's tier is `1 + max(tier of every exchange_* dependency)`; a root is tier 0. `exact_arith` (the family's shared decimal-arithmetic crate, outside this workstream) is not counted — it sits beneath every tier here and is not part of this family's own layering.

## Roots (Tier 0) — zero dependencies on any other `exchange_*` crate

- **`exchange_cap`** — `BookCaps { max_rests, max_levels, max_account_rests }` plus `cap_check_rest`/`cap_check_level`/`cap_check_account`, each returning `CapError::{RestsFull, LevelsFull, AccountFull}`. `exchange_core` checks all three before an order rests, against caps set per instrument with `Exchange::caps_set`.
  ```rust
  use exchange_cap::{ BookCaps, CapError, cap_check_rest };
  let caps = BookCaps { max_rests : 2, max_levels : 2, max_account_rests : 4 };
  assert_eq!( cap_check_rest( caps, 2 ), Err( CapError::RestsFull ) );
  ```
- **`exchange_id`** — `InstrumentId`/`OrderId`/`ClientOrderId`/`AccountId`, plus `*_from_raw`/`*_raw` accessors. The id vocabulary every other crate builds on.
  ```rust
  use exchange_id::{ OrderId, order_from_raw, order_raw };
  let id = order_from_raw( 7 );
  assert_eq!( order_raw( id ), 7 );
  ```
- **`exchange_seq`** — `Sequence` and `seq_next`, the tie-break `exchange_book` uses instead of a clock. `exchange_core::emit` calls `seq_next` once per event.
  ```rust
  use exchange_seq::{ Sequence, seq_next };
  let first = seq_next( Sequence::ZERO );
  assert!( seq_next( first ) > first );
  ```
- **`exchange_side`** — `Side { Buy, Sell }` (not `Bid`/`Ask` — see its own `docs/decisions/`) and `.opposite()`.
  ```rust
  use exchange_side::Side;
  assert_eq!( Side::Buy.opposite(), Side::Sell );
  ```
- **`exchange_stats`** — `BookStats { rests, fills, rejects, cancels }`, a running counter so a caller doesn't have to re-scan the event log. Wired since Stage 9: `Exchange::stats_get` (exchange-wide) and `stats_get_for` (one instrument) call `stats_snapshot` directly, and `step_place`/`step_one` call `stats_rest_add`/`stats_fill_add`/`stats_reject_add`/`stats_cancel_add` on every step.
  ```rust
  use exchange_stats::{ stats_fill_add, stats_snapshot, stats_zero };
  let mut stats = stats_zero();
  stats_fill_add( &mut stats, 2 );
  assert_eq!( stats_snapshot( &stats ).fills, 2 );
  ```
- **`exchange_stp`** — `SelfMatchPolicy { CancelResting, CancelIncoming, CancelBoth }` — no `Allow` variant; the family refuses every self-match unconditionally. `exchange_match` depends on this directly and re-exports it.
  ```rust
  use exchange_stp::{ SelfMatchPolicy, stp_name };
  assert_eq!( stp_name( SelfMatchPolicy::CancelIncoming ), "cancel_incoming" );
  ```
- **`exchange_tif`** — `Tif { Gtc, Ioc, Fok, PostOnly }`, `tif_rests`, `tif_requires_full`, `tif_takes`. `exchange_match::cross` gates FOK and refuses a taking post-only; `exchange_core` drops a non-resting remainder; see `exchange_match`'s own module doc.
  ```rust
  use exchange_tif::{ Tif, tif_rests };
  assert!( !tif_rests( Tif::Ioc ) );
  ```

## Tier 1 — depends only on roots

- **`exchange_idem`** (→ `exchange_id`) — `IdSet`/`idem_insert`/`idem_seen`/`idem_remove`; refuses a retried id instead of double-resting it. Generic over the key: `exchange_inbound` keys it by `OrderId`, `exchange_core` by `( AccountId, ClientOrderId )`.
  ```rust
  use exchange_id::OrderId;
  use exchange_idem::{ IdSet, idem_insert };
  let mut seen = IdSet::new();
  idem_insert( &mut seen, OrderId( 1 ) ).unwrap();
  assert!( idem_insert( &mut seen, OrderId( 1 ) ).is_err() );
  ```
- **`exchange_order`** (→ `exchange_id`, `exchange_side`, `exchange_tif`) — `Order { id, instrument, account, side, price, quantity, tif, client }` and `Obligation { Cash, Asset }`. Every book-facing crate above this one constructs or reads an `Order`.
  ```rust
  use exact_arith::{ Price, Quantity };
  use exchange_id::{ AccountId, InstrumentId, OrderId };
  use exchange_order::Order;
  use exchange_side::Side;
  use exchange_tif::Tif;
  let order = Order { id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ), side : Side::Buy, price : Price::parse( "1.25" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc, client : None };
  assert_eq!( order.side, Side::Buy );
  ```
  Extracted from `exchange_types` with two fields added (`instrument`, `tif`) against the five the real struct had before. `exchange_types` re-exported `Order` unchanged for a time; that re-export is retired now (2026-10-05) and every call site depends on this crate directly.
- **`exchange_spec`** (→ `exchange_id`) — `InstrumentSpec { id, base, quote, tick, lot, halted }`, `spec_new`, `price_snap`/`qty_snap`, `price_fits`/`qty_fits`. `Exchange::spec_register` stores specs; `step_place` refuses an order whose instrument has none (`RejectReason::UnknownInstrument`), then reads its halt flag and refuses an order off its grid (`RejectReason::PriceOffTick`/`QuantityOffLot`).
  ```rust
  use exact_arith::{ Price, Quantity };
  use exchange_id::InstrumentId;
  use exchange_spec::{ AssetId, price_snap, spec_new };
  let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
  assert_eq!( price_snap( &spec, Price::parse( "1.26" ).unwrap() ).unwrap(), Price::parse( "1.25" ).unwrap() );
  ```

## Tier 2

- **`exchange_fill`** (→ `exchange_id`, `exchange_order`, `exchange_seq`, `exchange_side`) — `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause`. `Trade::executed_price` names the maker's-price rule explicitly.
  ```rust
  use exchange_fill::Trade;
  use exact_arith::Price;
  let maker = Price::parse( "2.50" ).unwrap();
  assert_eq!( Trade::executed_price( maker, Price::parse( "3.00" ).unwrap() ), maker );
  ```
- **`exchange_halt`** (→ `exchange_spec`) — `halt_set`/`halt_clear`/`halt_is` on an `InstrumentSpec`'s own flag. Wired since Stage 9: `Exchange::halt_set`/`halt_clear`/`halt_is` call straight through, one-to-one by name, and `step_place` refuses a new order on a halted instrument (`RejectReason::Halted`).
  ```rust
  use exact_arith::{ Price, Quantity };
  use exchange_halt::{ halt_is, halt_set };
  use exchange_id::InstrumentId;
  use exchange_spec::{ AssetId, spec_new };
  let mut spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
  halt_set( &mut spec ).unwrap();
  assert!( halt_is( &spec ) );
  ```
  The proposal names `exchange_book` as a second dependency too; not taken, since nothing here reads a book (own `docs/decisions/` entry).
- **`exchange_level`** (→ `exchange_id`, `exchange_order`, `exchange_seq`) — `Level`/`LevelNode`, the explicit FIFO queue at one price: `level_new`, `level_push`, `level_pop_front`.
  ```rust
  use exchange_level::{ level_new, level_pop_front };
  use exact_arith::Price;
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  assert!( level_pop_front( &mut level ).is_none() );
  ```
  `exchange_book` keeps one `Level` per price per side, and `Resting` is an alias of `LevelNode`. The nodes sit in a `VecDeque`, so taking the front of a level is O(1).
- **`exchange_types`** (→ `exchange_order`, `exchange_side`) — narrowed down to what was always genuinely its own: `TypeError` and `notional`/`obligation`. See Introduction, point 1 — the eleven re-exports this crate used to carry are gone as of 2026-10-05, which is also why it moved from Tier 3 to Tier 2 here.
  ```rust
  use exchange_types::notional;
  use exact_arith::{ Money, Price, Quantity };
  assert_eq!( notional( Price::parse( "2.50" ).unwrap(), Quantity::from_int( 4 ).unwrap() ).unwrap(), Money::parse( "10" ).unwrap() );
  ```

## Tier 3

- **`exchange_book`** (→ `exchange_id`, `exchange_level`, `exchange_side`) — the resting order book, keyed by `InstrumentId`, one sorted `VecDeque<Level>` per side, best at the front (better price first, earlier arrival breaks a tie — never wall-clock time, only a caller-supplied `Sequence`).
  ```rust
  use exchange_book::Book;
  use exchange_id::{ InstrumentId, OrderId };
  use exchange_side::Side;

  let instrument = InstrumentId( 1 );
  // book.insert( resting ) seats a `Resting` order, refusing a duplicate id outright;
  // book.cancel( instrument, id ) -> Option<Resting> (None if already gone);
  // book.side( instrument, Side::Buy ).next() reads the best bid, published-order first.
  ```
  `exchange_types` doesn't appear here at all — this crate never depended on it for `Side`/`OrderId`, and the brief period (before 2026-10-05) where either arrived via its re-export is over; both now come from their owning leaf crate directly, same as `InstrumentId`.
- **`exchange_conserve`** (→ `exchange_fill`, `exchange_side`, `exchange_types`) — `conserve_assert(fills: &[Trade])`: both legs of every trade (debit the buyer, credit the seller) must sum to zero. Standalone — not called from `exchange_match::cross` yet; see Introduction, point 2.
  ```rust
  use exact_arith::Money;
  use exchange_conserve::fill_legs_sum;
  let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -10 ).unwrap() ];
  assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::ZERO );
  ```
- **`exchange_escrow`** (→ `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_types`) — `Escrow`/`Account`/`Holding<T>`: `reserve`/`release`/`settle`, `available`/`reserved` stored separately (never derived from the book) so the two can be compared, not merely agree by construction. The `exchange_types` edge is real but narrow now — just `TypeError` — not the multi-type re-export it used to be before 2026-10-05.
  ```rust
  use exact_arith::{ Money, Price, Quantity };
  use exchange_escrow::Escrow;
  use exchange_id::{ AccountId, InstrumentId, OrderId };
  use exchange_order::Order;
  use exchange_side::Side;
  use exchange_tif::Tif;
  let mut escrow = Escrow::new();
  let account = AccountId( 1 );
  escrow.open( account, Money::from_int( 100 ).unwrap(), Quantity::ZERO ).unwrap();
  let order = Order { id : OrderId( 1 ), instrument : InstrumentId( 1 ), account, side : Side::Buy, price : Price::parse( "2.50" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc, client : None };
  escrow.reserve( &order ).unwrap();
  assert_eq!( escrow.reservation_count(), 1 );
  ```
  Real-balance-holding, by decision — the proposal narrows this to a thin port with a separate ledger owning the balances, but the one real external consumer already calls this crate's concrete API across its whole test suite with no ledger of its own, so narrowing it now would force an unrequested rebuild there. See `docs/decision/` in the family root corpus for the full reasoning.
- **`exchange_event`** (→ `exchange_fill`) — `event_push`/`event_drain`/`event_len`/`event_clear`: the owned-drain operation `exchange_core::events()`'s borrowed `&[Event]` doesn't provide. No longer routes through `exchange_types` as of 2026-10-05 — `Event` comes from `exchange_fill` directly now. Wired since Stage 9: `Exchange::event_drain` calls `event_drain` directly; `events()` still returns the borrowed slice unchanged, so both forms coexist on the facade now rather than this crate's drain being unreachable from it.
  ```rust
  use exchange_event::{ Event, event_drain };
  let mut events : Vec< Event > = Vec::new();
  let drained = event_drain( &mut events );
  assert!( drained.is_empty() && events.is_empty() );
  ```

## Tier 4

- **`exchange_depth`** (→ `exchange_book`, `exchange_id`, `exchange_side`) — `depth_top(book, instrument, n)`, reading `Book`'s own priority-ordered slices directly rather than walking every rest. Wired since Stage 9: `Exchange::depth_get` calls `depth_top` directly, no registered spec required.
  ```rust
  use exchange_book::Book;
  use exchange_depth::depth_top;
  use exchange_id::InstrumentId;
  let book = Book::new();
  let depth = depth_top( &book, InstrumentId( 1 ), 10 ).unwrap();
  assert!( depth.bids.is_empty() && depth.asks.is_empty() );
  ```
- **`exchange_match`** (→ `exchange_book`, `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_stp`, `exchange_tif`, `exchange_types`) — `cross()`: price-time matching, self-match prevention, and TIF (IOC needs no special handling; FOK probes a cloned book first and only commits for real on a full fill). Fully wired into `exchange_core`. The `exchange_types` edge survived the 2026-10-05 retirement narrowed to `notional` alone — the other four names here (`exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`) were added directly then, having previously arrived only transitively through the old aggregator.
  ```rust
  // cross( book: &mut Book, incoming: &Order, policy: SelfMatchPolicy ) -> Result<Crossing, MatchError>
  // A Fok order that cannot fill completely leaves the book untouched: cross()
  // probes a clone first and returns Crossing{ remaining: incoming.quantity, .. }
  // without ever mutating `book`, rather than resting or partially filling it.
  ```
- **`exchange_rest`** (→ `exchange_book`, `exchange_id`) — `rest_place`/`rest_cancel` (thin wrappers over `Book`) and `rest_replace` (new: atomic cancel-then-reinsert, rolling back on a refused insert). Partially wired since Stage 9: `step_place` calls `rest_place` for every order that has a remainder to rest; `Exchange::cancel` still orchestrates escrow/book cancellation inline rather than calling `rest_cancel`. `rest_replace` is imported nowhere in `exchange_core` — `exchange_step` drains an `InboundCmd::Replace` but returns `StepOutcome::ReplaceNotWired` instead of calling it (see Introduction, point 2, and `exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md`).
  ```rust
  // rest_replace( book: &mut Book, instrument, old_id: OrderId, new_resting: Resting )
  //   -> Result<Resting, RestReplaceError>
  // Cancels `old_id`, inserts `new_resting`; on a refused insert (duplicate id,
  // cap reached) it re-inserts the original and returns Err(Refused) rather
  // than leaving the book with neither order resting.
  ```
- **`exchange_snap`** (→ `exchange_book`, `exchange_id`, `exchange_side`) — `snap_take(book, instrument, tick)`: plain copied rows, independent of the live book the moment they're taken. Wired since Stage 9: `Exchange::snap_take` calls `snap_take` directly, one-to-one by name.
  ```rust
  use exact_arith::Money;
  use exchange_book::Book;
  use exchange_id::InstrumentId;
  use exchange_snap::{ snap_len, snap_take };
  let book = Book::new();
  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  assert_eq!( snap_len( &snap ), 0 );
  ```

## Tier 5

- **`exchange_inbound`** (→ `exchange_book`, `exchange_id`, `exchange_match`, `exchange_rest`, `exchange_tif`, plus `ring_factory`/`ring_handle`/`ring_types`) — `InboundCmd::{Place, Cancel, Replace}` carried from a `ring_factory`/`ring_handle` producer to the book: `inbound_ring`, `inbound_flush`, `inbound_drain`, `inbound_apply`, `inbound_overflow_reject`. The family's first genuinely concurrent code, and — since Stage 9 — the ring `Exchange::exchange_step` (Tier 6) itself drains on every call.
  ```rust
  use exchange_book::Book;
  use exchange_id::{ InstrumentId, OrderId };
  use exchange_inbound::{ Claims, InboundCmd, inbound_apply, inbound_drain, inbound_ring };
  use exchange_stp::SelfMatchPolicy;

  let mut split = inbound_ring( 8 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();
  producer.try_push( InboundCmd::Cancel { instrument : InstrumentId( 1 ), id : OrderId( 1 ) } ).unwrap();

  let mut book = Book::new();
  let mut claims = Claims::new();
  for cmd in inbound_drain( &mut consumer )
  {
    inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();
  }
  ```
  `inbound_apply` (called directly here, against a bare `Book`) and `Exchange::exchange_step` (Tier 6) are two different consumers of the same ring contract — the facade does not call `inbound_apply` itself, it re-implements per-command dispatch as its own `step_one` so a drained `Place` runs the full validate-reserve-match-settle pipeline rather than a bare book insert. `ring_handle::Producer` can't be cloned, so "two producers" is built as two independent rings combined in a fixed lane order on drain, not one shared ring — a deliberate divergence from the proposal's single-ring assumption, not a limitation worked around silently. Genuinely moves data across a real ring, in its own OS threads, with its own two-producer and overflow phase-smokes (P28, P29).

## Tier 6 — Facade

- **`exchange_core`** (→ `exchange_book`, `exchange_cap`, `exchange_depth`, `exchange_escrow`, `exchange_event`, `exchange_fill`, `exchange_halt`, `exchange_id`, `exchange_idem`, `exchange_inbound`, `exchange_match`, `exchange_order`, `exchange_rest`, `exchange_seq`, `exchange_side`, `exchange_snap`, `exchange_spec`, `exchange_stats`, `exchange_tif`, `exchange_types`, plus `exact_arith` directly) — `Exchange`. As of the Stage 9 facade rework (2026-10-03), the old `submit` method is gone entirely: every order now arrives through `exchange_step`, draining an `exchange_inbound` ring and running each `InboundCmd::Place` through the same validate-reserve-match-settle-rest pipeline `submit` used to run directly (see `exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md`).
  ```rust
  use exchange_core::
  {
    AccountId, AssetId, Exchange, InboundCmd, InstrumentId, Money, Order, OrderId,
    Price, Quantity, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif,
    inbound_ring, verify,
  };

  let instrument = InstrumentId( 1 );
  let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
  let price = Price::parse( "2.50" ).unwrap();
  let four = Quantity::from_int( 4 ).unwrap();

  let mut exchange = Exchange::new();
  exchange.open_account( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() ).unwrap();
  exchange.open_account( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO ).unwrap();
  exchange.spec_register( instrument, AssetId( 1 ), AssetId( 2 ), Price::parse( "0.01" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();

  // `id`/`arrival` on each draft are placeholders — `exchange_step` overwrites
  // both with its own counter/sequence before an order ever reaches the book.
  let draft = | account, side | Resting
  {
    order : Order { id : OrderId( 0 ), instrument, account, side, price, quantity : four, tif : Tif::Gtc, client : None },
    remaining : four, arrival : Sequence::ZERO,
  };
  let mut split = inbound_ring( 8 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();
  producer.try_push( InboundCmd::Place( draft( seller, Side::Sell ) ) ).unwrap();
  producer.try_push( InboundCmd::Place( draft( buyer, Side::Buy ) ) ).unwrap();

  let outcomes = exchange.exchange_step( &mut consumer, SelfMatchPolicy::CancelIncoming );
  match &outcomes[ 1 ]
  {
    StepOutcome::Placed( Ok( receipt ) ) => assert_eq!( receipt.trades.len(), 1 ),
    other => panic!( "expected the buy to fill, got {other:?}" ),
  }
  assert!( verify( &exchange.postings().unwrap() ).unwrap().is_balanced() );
  ```
  `price`/`tick` are typed `Price`; `open_account`'s balances are typed `Money`. The two are distinct types, so passing a total where a per-unit price belongs is a compile error.
  Reaches every tier below through twenty direct `exchange_*` Cargo.toml dependencies now — `exchange_cap` for caps, `exchange_idem` for client-id retries, and `exchange_fill`/`exchange_order`/`exchange_side`, which the 2026-10-05 `exchange_types` retirement turned from transitive into direct — the only names absent from what `Exchange`'s own methods actually call are `exchange_conserve` and (within `exchange_rest` specifically) `rest_replace`; see Introduction, point 2.

## Smoke lanes

- **`smoke_exchange_book`** — depends on `exchange_core` for everything except idempotency and conservation, by design: a re-export missing from the facade is a build failure here, not a gap nobody notices. One scenario covering multi-level matching, every `Tif`, halt/resume, self-trade prevention, duplicate rejection, two-ring determinism, ring overflow, and conservation, graded against one fixed golden block.
  ```bash
  cargo run -p smoke_exchange_book
  ```
- **`smoke_exchange_phases`** — one tiny `demo_pNN_*` binary per incremental build phase (P01–P29 so far), each printing an exact golden line checked against `../golden_output/`. Depends on nearly every crate directly, deliberately — it grades each new contract in isolation, the moment it lands.
  ```bash
  cargo run -q -p smoke_exchange_phases --bin demo_p01_id
  ```

## Verify this guide yourself

The tier table is a derived fact, not an assertion — recompute the roots yourself:

```bash
cd module
for d in */; do c="${d}Cargo.toml"; [ -f "$c" ] || continue
  deps=$(awk '/^\[dependencies\]/{f=1;next} /^\[/{f=0} f && NF && /^exchange_/{print}' "$c")
  [ -z "$deps" ] && echo "root: ${d%/}"
done
```

**Expected:** exactly the seven Tier-0 crates named above, nothing else.

The wiring claims (Introduction, point 2; every "Wired since Stage 9"/"Standalone" note below) are a second derived fact, not a status report — recompute which previously-standalone crates `exchange_core` actually calls by grepping its own source for each name, rather than trusting this guide's prose:

```bash
cd module/exchange_core
for c in exchange_cap exchange_conserve exchange_depth exchange_event exchange_halt \
         exchange_idem exchange_inbound exchange_rest exchange_snap exchange_spec exchange_stats; do
  command grep -q "$c" src/lib.rs && echo "referenced: $c" || echo "absent:     $c"
done
```

**Expected:** `absent` for exactly `exchange_conserve`; `referenced` for the other ten. A `referenced` crate still needs a second look to tell a real call (e.g. `exchange_halt::halt_set(...)`) apart from a doc-comment mention alone — `exchange_rest` is the one name in this list where that distinction matters: `rest_place` is called, `rest_replace` is not (see Tier 5's `exchange_rest` entry and Introduction, point 2).

## Sources

| File | Relationship |
|------|--------------|
| `module/*/Cargo.toml` | The `[dependencies]` blocks this guide's tier table was computed from, by direct `awk` extraction, not readme prose |
| [`../../module/readme.md`](../../module/readme.md) | The workspace's own flat crate index — this guide adds the tier/wiring dimension on top |
| [`../../readme.md`](../../readme.md) | The family root readme — orientation at the repo level; this guide is the module level |
| [`../dependency_tree/001_002_dependency_tree.md`](../dependency_tree/001_002_dependency_tree.md) | The *proposed* 23-crate graph (Roots/Trunk/Bridge/Facade) this guide's tiers were cross-checked against, then re-derived from the real build where the two diverge |
