# Guide: The Exchange Crate Family

### Scope

- **Purpose**: Give a new reader the whole family's shape in one pass — what each real crate does, how to call it, and which tier it sits at — without requiring 26 separate readme visits first.
- **Responsibility**: Purpose, a runnable usage snippet, and a dependency tier for every crate/lane under `module/`.
- **In Scope**: The real, as-built `module/` tree — 24 `exchange_*` crates plus two smoke lanes — and their actual `Cargo.toml` dependency edges, verified by direct read, not by trusting any single crate's own prose.
- **Out of Scope**: The 23-crate *proposal*'s own per-item analysis (→ [`../crate/`](../crate/readme.md), [`../exposed_item/`](../exposed_item/readme.md)); any crate's full design rationale (→ that crate's own `readme.md`, linked from every entry below).

**Design status**: first guide pass, dated 2026-10-03. Every dependency edge below was re-extracted directly from each crate's own `Cargo.toml` (`awk` over every `[dependencies]` block in `module/*/Cargo.toml`), not copied from readme prose — at least three readmes (`exchange_escrow`, `exchange_core`, `exchange_snap`) understate their own crate's dependency count (`exchange_snap`'s own Responsibility Table omits `exchange_side`, confirmed present in its real `Cargo.toml`), and this guide's table reflects the verified edges, not those descriptions. `exchange_inbound` landed the same day, after this pass's first draft — folded in immediately rather than left stale, confirmed directly against its real `Cargo.toml`/`src/lib.rs` rather than taken on a status report alone. Re-run the extraction below before trusting this table if the family has grown since.

## Introduction

`module/` holds 24 `exchange_*` crates plus two smoke lanes (`smoke_exchange_core`, `smoke_exchange_phases`). Each crate does one thing; nothing here is a kitchen-sink `utils`. The family splits into dependency **tiers** — tier *N* may depend only on tiers below it — which is the one piece of structure this guide adds on top of what each crate's own readme already says. Four things worth knowing before the table:

1. **`exchange_types` is a legacy re-export aggregator, not a design tier.** It held everything in this family's original 4-crate build (`Side`, `AccountId`, `OrderId`, `Sequence`, `Order`, `Obligation`, `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause`). Every one of those has since moved to its own leaf crate, and `exchange_types` now re-exports all eleven unchanged — so old `use exchange_types::X` call sites still resolve while new code depends on the leaf directly. Four crates (`exchange_book`, `exchange_conserve`, `exchange_escrow`, `exchange_event`) still route through it rather than the leaf, which pushes their *measured* tier one layer deeper than their *conceptual* role would suggest. This is a known, tracked transitional state (see `exchange_types/readme.md`'s own "Extraction — shrinking, not static" section), not a design mistake — watch for it when the number in the table looks higher than you'd expect.
2. **Not every built crate is wired into `exchange_core`.** Eleven crates — `exchange_cap`, `exchange_conserve`, `exchange_depth`, `exchange_event`, `exchange_halt`, `exchange_idem`, `exchange_inbound`, `exchange_rest`, `exchange_snap`, `exchange_spec`, `exchange_stats` — are real, tested, and standalone, but `Exchange::submit`/`cancel` does not call them yet. This is the family's own established "skeleton-first" pattern: build and prove the crate in isolation, wire it into the facade in a later stage. Call these directly; don't expect `Exchange` to reach them yet.
3. **24 real crates against a 23-crate proposal — not a 1:1 match.** The proposal names 23 crates; the real build now has 24 `exchange_*` directories, and not the same 23 either. `exchange_types` is not one of the proposed 23 at all — it is the legacy crate everything else used to live in — and `exchange_inbound` (the ring-ingress bridge, the proposal's 23rd) landed only after this guide's first pass, built against `ring_factory`+`ring_handle`+`ring_types` rather than `ring_core` directly.
4. **`Money` and `Price` are the same type wearing two names.** Both resolve to `exact_kind::Decimal<MONEY_SCALE>` via a plain `pub type` alias — not two newtypes with a conversion between them. Either name compiles wherever the other is expected; the split exists only so a signature can signal "a per-unit price" versus "an absolute amount" to a reader, and the compiler enforces none of it. Several crates' own readmes use the two names in ways that look inconsistent at a glance (`Money::parse` feeding a field actually typed `Price`) — that is not a bug in those readmes, it is this fact in action.

## Tier Table

| Tier | Crate | One-line purpose |
|------|-------|-------------------|
| 0 — Roots | [`exchange_cap`](../../module/exchange_cap/readme.md) | A limit on book growth, with a named refusal past it |
| 0 — Roots | [`exchange_id`](../../module/exchange_id/readme.md) | Plain id newtypes — `InstrumentId`, `OrderId`, `AccountId` |
| 0 — Roots | [`exchange_seq`](../../module/exchange_seq/readme.md) | A monotonic sequence for time priority, no wall clock |
| 0 — Roots | [`exchange_side`](../../module/exchange_side/readme.md) | `Side { Buy, Sell }` and its opposite relation |
| 0 — Roots | [`exchange_stats`](../../module/exchange_stats/readme.md) | Running rest/fill/reject/cancel counters |
| 0 — Roots | [`exchange_stp`](../../module/exchange_stp/readme.md) | The self-trade policy, as a closed set of three |
| 0 — Roots | [`exchange_tif`](../../module/exchange_tif/readme.md) | `Tif { Gtc, Ioc, Fok }` — time-in-force disposition |
| 1 | [`exchange_idem`](../../module/exchange_idem/readme.md) | Refuse an `OrderId` the book has already seen |
| 1 | [`exchange_order`](../../module/exchange_order/readme.md) | One order record — instrument, account, side, tif included |
| 1 | [`exchange_spec`](../../module/exchange_spec/readme.md) | Tick, lot, the asset pair, and the halt flag on one instrument |
| 2 | [`exchange_fill`](../../module/exchange_fill/readme.md) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` |
| 2 | [`exchange_halt`](../../module/exchange_halt/readme.md) | An on/off switch for matching alone |
| 2 | [`exchange_level`](../../module/exchange_level/readme.md) | One price, FIFO rest |
| 3 | [`exchange_types`](../../module/exchange_types/readme.md) | Legacy re-export aggregator + `notional`/`obligation` |
| 4 | [`exchange_book`](../../module/exchange_book/readme.md) | The resting order book, one per instrument, in price-time priority |
| 4 | [`exchange_conserve`](../../module/exchange_conserve/readme.md) | A fill batch nets to zero across its legs, or is refused |
| 4 | [`exchange_escrow`](../../module/exchange_escrow/readme.md) | The reservation ledger — available/reserved partition and settlement |
| 4 | [`exchange_event`](../../module/exchange_event/readme.md) | The one owned drain point for the event stream |
| 5 | [`exchange_depth`](../../module/exchange_depth/readme.md) | Top-N book depth without a full walk |
| 5 | [`exchange_match`](../../module/exchange_match/readme.md) | `cross()` — an incoming order against the book, TIF-aware |
| 5 | [`exchange_rest`](../../module/exchange_rest/readme.md) | Rest, cancel, replace — the non-matching ways an order moves |
| 5 | [`exchange_snap`](../../module/exchange_snap/readme.md) | A plain, copied snapshot of a book's resting rows |
| 6 — Facade | [`exchange_core`](../../module/exchange_core/readme.md) | `Exchange` — the five-step submission path |
| 6 | [`exchange_inbound`](../../module/exchange_inbound/readme.md) | `InboundCmd` from a ring producer to the book — the family's first concurrency |
| — (lane) | [`smoke_exchange_core`](../../module/smoke_exchange_core/readme.md) | One order crossing five crates, watched from outside |
| — (lane) | [`smoke_exchange_phases`](../../module/smoke_exchange_phases/readme.md) | The P01–P29 phase-smoke ladder, one binary per phase |

A crate's tier is `1 + max(tier of every exchange_* dependency)`; a root is tier 0. `exact_arith` (the family's shared decimal-arithmetic crate, outside this workstream) is not counted — it sits beneath every tier here and is not part of this family's own layering.

## Roots (Tier 0) — zero dependencies on any other `exchange_*` crate

- **`exchange_cap`** — `BookCaps { max_rests, max_levels }` plus `cap_check_rest`/`cap_check_level`, each returning `CapError::{RestsFull, LevelsFull}`. Nothing calls it yet (see Introduction, point 2).
  ```rust
  use exchange_cap::{ BookCaps, CapError, cap_check_rest };
  let caps = BookCaps { max_rests : 2, max_levels : 2 };
  assert_eq!( cap_check_rest( caps, 2 ), Err( CapError::RestsFull ) );
  ```
- **`exchange_id`** — `InstrumentId`/`OrderId`/`AccountId`, plus `*_from_raw`/`*_raw` accessors. The id vocabulary every other crate builds on.
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
- **`exchange_stats`** — `BookStats { rests, fills, rejects, cancels }`, a running counter so a caller doesn't have to re-scan the event log. Standalone; see Introduction, point 2.
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
- **`exchange_tif`** — `Tif { Gtc, Ioc, Fok }`, `tif_rests`, `tif_requires_full`. Read by `exchange_match::cross` in exactly one place each; see that crate's own module doc.
  ```rust
  use exchange_tif::{ Tif, tif_rests };
  assert!( !tif_rests( Tif::Ioc ) );
  ```

## Tier 1 — depends only on roots

- **`exchange_idem`** (→ `exchange_id`) — `IdSet`/`idem_insert`/`idem_seen`/`idem_remove`; refuses a retried `OrderId` instead of double-resting it. Standalone; see Introduction, point 2.
  ```rust
  use exchange_id::OrderId;
  use exchange_idem::{ IdSet, idem_insert };
  let mut seen = IdSet::new();
  idem_insert( &mut seen, OrderId( 1 ) ).unwrap();
  assert!( idem_insert( &mut seen, OrderId( 1 ) ).is_err() );
  ```
- **`exchange_order`** (→ `exchange_id`, `exchange_side`, `exchange_tif`) — `Order { id, instrument, account, side, price, quantity, tif }` and `Obligation { Cash, Asset }`. Every book-facing crate above this one constructs or reads an `Order`.
  ```rust
  use exact_arith::{ Money, Quantity };
  use exchange_id::{ AccountId, InstrumentId, OrderId };
  use exchange_order::Order;
  use exchange_side::Side;
  use exchange_tif::Tif;
  let order = Order { id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ), side : Side::Buy, price : Money::parse( "1.25" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc };
  assert_eq!( order.side, Side::Buy );
  ```
  Extracted from `exchange_types` with two fields added (`instrument`, `tif`) against the five the real struct had before; `exchange_types` still re-exports `Order` unchanged so existing call sites keep resolving.
- **`exchange_spec`** (→ `exchange_id`) — `InstrumentSpec { id, base, quote, tick, lot, halted }`, `spec_new`, `price_snap`/`qty_snap`. Standalone — `exchange_book` does not yet validate against it; see Introduction, point 2.
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
  use exact_arith::Money;
  let maker = Money::parse( "2.50" ).unwrap();
  assert_eq!( Trade::executed_price( maker, Money::parse( "3.00" ).unwrap() ), maker );
  ```
- **`exchange_halt`** (→ `exchange_spec`) — `halt_set`/`halt_clear`/`halt_is` on an `InstrumentSpec`'s own flag. Standalone; see Introduction, point 2.
  ```rust
  use exact_arith::{ Price, Quantity };
  use exchange_halt::{ halt_is, halt_set };
  use exchange_id::InstrumentId;
  use exchange_spec::{ AssetId, spec_new };
  let mut spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
  halt_set( &mut spec ).unwrap();
  assert!( halt_is( &spec ) );
  ```
  The proposal names `exchange_book` as a second dependency too; dropped here since nothing in the real match loop checks halt yet, so there is no real behavior a book dependency would support (own `docs/decisions/` entry).
- **`exchange_level`** (→ `exchange_id`, `exchange_order`, `exchange_seq`) — `Level`/`LevelNode`, the explicit FIFO queue at one price: `level_new`, `level_push`, `level_pop_front`.
  ```rust
  use exchange_level::{ level_new, level_pop_front };
  use exact_arith::Money;
  let mut level = level_new( Money::parse( "2.50" ).unwrap() );
  assert!( level_pop_front( &mut level ).is_none() );
  ```
  Genuinely new, not yet exercised: `exchange_book` takes this as a Cargo.toml dependency, but per `exchange_book`'s own readme its real storage today is still one flat sorted `Vec<Resting>` per side — the per-price `Vec<Level>` retrofit `exchange_level`'s own readme describes under "Related" is a stated intent, not yet a landed change. A dependency edge in `Cargo.toml` is not proof a type is actually used — the two were cross-checked against each other here, not assumed from either readme alone.

## Tier 3 — the legacy aggregator

- **`exchange_types`** (→ `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_seq`) — re-exports all eleven types that used to live here, plus the one piece of real logic that stayed: `notional` (exact-or-`NotionalInexact`, never rounded) and `obligation`. See Introduction, point 1 — this is the crate to understand before the tier numbers above start looking surprising.
  ```rust
  use exchange_types::{ notional, Side };
  use exact_arith::{ Money, Quantity };
  assert_eq!( notional( Money::parse( "2.50" ).unwrap(), Quantity::from_int( 4 ).unwrap() ).unwrap(), Money::parse( "10" ).unwrap() );
  assert_eq!( Side::Buy.opposite(), Side::Sell );
  ```

## Tier 4

- **`exchange_book`** (→ `exchange_types`, `exchange_level`, `exchange_id`) — the resting order book, keyed by `InstrumentId`, one sorted `Vec<Resting>` per side, best at the front (better price first, earlier arrival breaks a tie — never wall-clock time, only a caller-supplied `Sequence`).
  ```rust
  use exchange_book::Book;
  use exchange_id::InstrumentId;
  use exchange_types::{ OrderId, Side };

  let instrument = InstrumentId( 1 );
  // book.insert( resting ) seats a `Resting` order, refusing a duplicate id outright;
  // book.cancel( instrument, id ) -> Option<Resting> (None if already gone);
  // book.side( instrument, Side::Buy ).next() reads the best bid, published-order first.
  ```
  `Side`/`OrderId` arrive via the `exchange_types` re-export in the crate's own canonical example, even though `exchange_id` is also a direct dependency (used there for `InstrumentId` alone) — not a contradiction, just which path the crate's own code happens to use.
- **`exchange_conserve`** (→ `exchange_fill`, `exchange_side`, `exchange_types`) — `conserve_assert(fills: &[Trade])`: both legs of every trade (debit the buyer, credit the seller) must sum to zero. Standalone — not called from `exchange_match::cross` yet; see Introduction, point 2.
  ```rust
  use exact_arith::Money;
  use exchange_conserve::fill_legs_sum;
  let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -10 ).unwrap() ];
  assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::ZERO );
  ```
- **`exchange_escrow`** (→ `exchange_types`) — `Escrow`/`Account`/`Holding<T>`: `reserve`/`release`/`settle`, `available`/`reserved` stored separately (never derived from the book) so the two can be compared, not merely agree by construction.
  ```rust
  use exact_arith::{ Money, Quantity };
  use exchange_escrow::Escrow;
  use exchange_id::{ AccountId, InstrumentId, OrderId };
  use exchange_order::Order;
  use exchange_side::Side;
  use exchange_tif::Tif;
  let mut escrow = Escrow::new();
  let account = AccountId( 1 );
  escrow.open( account, Money::from_int( 100 ).unwrap(), Quantity::ZERO ).unwrap();
  let order = Order { id : OrderId( 1 ), instrument : InstrumentId( 1 ), account, side : Side::Buy, price : Money::parse( "2.50" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc };
  escrow.reserve( &order ).unwrap();
  assert_eq!( escrow.reservation_count(), 1 );
  ```
  Real-balance-holding, by decision — the proposal narrows this to a thin port with a separate ledger owning the balances, but the one real external consumer already calls this crate's concrete API across its whole test suite with no ledger of its own, so narrowing it now would force an unrequested rebuild there. See `docs/decision/` in the family root corpus for the full reasoning.
- **`exchange_event`** (→ `exchange_types`) — `event_push`/`event_drain`/`event_len`/`event_clear`: the owned-drain operation `exchange_core::events()`'s borrowed `&[Event]` doesn't provide. Standalone; see Introduction, point 2.
  ```rust
  use exchange_event::{ Event, event_drain };
  let mut events : Vec< Event > = Vec::new();
  let drained = event_drain( &mut events );
  assert!( drained.is_empty() && events.is_empty() );
  ```

## Tier 5

- **`exchange_depth`** (→ `exchange_book`, `exchange_id`, `exchange_side`) — `depth_top(book, instrument, n)`, reading `Book`'s own priority-ordered slices directly rather than walking every rest. Standalone; see Introduction, point 2.
  ```rust
  use exchange_book::Book;
  use exchange_depth::depth_top;
  use exchange_id::InstrumentId;
  let book = Book::new();
  let depth = depth_top( &book, InstrumentId( 1 ), 10 ).unwrap();
  assert!( depth.bids.is_empty() && depth.asks.is_empty() );
  ```
- **`exchange_match`** (→ `exchange_types`, `exchange_book`, `exchange_stp`, `exchange_tif`) — `cross()`: price-time matching, self-match prevention, and TIF (IOC needs no special handling; FOK probes a cloned book first and only commits for real on a full fill). Fully wired into `exchange_core`.
  ```rust
  // cross( book: &mut Book, incoming: &Order, policy: SelfMatchPolicy ) -> Result<Crossing, MatchError>
  // A Fok order that cannot fill completely leaves the book untouched: cross()
  // probes a clone first and returns Crossing{ remaining: incoming.quantity, .. }
  // without ever mutating `book`, rather than resting or partially filling it.
  ```
- **`exchange_rest`** (→ `exchange_book`, `exchange_id`) — `rest_place`/`rest_cancel` (thin wrappers over `Book`) and `rest_replace` (new: atomic cancel-then-reinsert, rolling back on a refused insert). Standalone; see Introduction, point 2.
  ```rust
  // rest_replace( book: &mut Book, instrument, old_id: OrderId, new_resting: Resting )
  //   -> Result<Resting, RestReplaceError>
  // Cancels `old_id`, inserts `new_resting`; on a refused insert (duplicate id,
  // cap reached) it re-inserts the original and returns Err(Refused) rather
  // than leaving the book with neither order resting.
  ```
- **`exchange_snap`** (→ `exchange_book`, `exchange_id`, `exchange_side`) — `snap_take(book, instrument, tick)`: plain copied rows, independent of the live book the moment they're taken. Standalone; see Introduction, point 2.
  ```rust
  use exact_arith::Money;
  use exchange_book::Book;
  use exchange_id::InstrumentId;
  use exchange_snap::{ snap_len, snap_take };
  let book = Book::new();
  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  assert_eq!( snap_len( &snap ), 0 );
  ```

## Tier 6

Two crates land here, for different reasons — one because it *is* the facade, one because it happens to depend on two Tier-5 crates at once. Neither depends on the other.

- **`exchange_core`** — the facade. (→ `exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, plus `exchange_seq`/`exchange_id`/`exchange_tif` directly) — `Exchange`, the five-step path (validate → reserve → match → settle → dispose).
  ```rust
  use exchange_core::{ AccountId, Exchange, Money, Price, Quantity, Side, verify };
  let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
  let price = Price::parse( "2.50" ).unwrap();
  let four = Quantity::from_int( 4 ).unwrap();
  let mut exchange = Exchange::new();
  exchange.open_account( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() ).unwrap();
  exchange.open_account( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO ).unwrap();
  exchange.submit( seller, Side::Sell, price, four ).unwrap();
  let receipt = exchange.submit( buyer, Side::Buy, price, four ).unwrap();
  assert_eq!( receipt.trades.len(), 1 );
  assert!( verify( &exchange.postings().unwrap() ).unwrap().is_balanced() );
  ```
  `submit`'s `price` parameter is typed `Price`; `open_account`'s balances are typed `Money`. Not two types that happen to convert easily — `exact_kind::{Money, Price}` are both literally `pub type` aliases for the same `Decimal<MONEY_SCALE>`, so either name compiles in either position. The names exist purely to signal intent (a per-unit price vs. an absolute amount) — the compiler enforces none of it, so a genuine "passed a total where a per-unit price belonged" mistake compiles clean and must be caught by a human or a test, never by `cargo check`.
  Reaches tiers 0–5 through exactly four direct dependencies on real logic (`types`, `book`, `match`, `escrow`) — the eleven standalone crates named in the Introduction are not in this list, so `Exchange` cannot call them yet.
- **`exchange_inbound`** — not the facade; lands at the same tier number only because it depends on two Tier-5 crates at once. (→ `exchange_book`, `exchange_id`, `exchange_match`, `exchange_rest`, `exchange_tif`, plus `ring_factory`/`ring_handle`/`ring_types`) — `InboundCmd::{Place, Cancel, Replace}` carried from a `ring_factory`/`ring_handle` producer to the book: `inbound_ring`, `inbound_flush`, `inbound_drain`, `inbound_apply`, `inbound_overflow_reject`. The family's first genuinely concurrent code.
  ```rust
  use exchange_book::Book;
  use exchange_id::{ InstrumentId, OrderId };
  use exchange_inbound::{ InboundCmd, inbound_apply, inbound_drain, inbound_ring };
  use exchange_stp::SelfMatchPolicy;

  let mut split = inbound_ring( 8 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();
  producer.try_push( InboundCmd::Cancel { instrument : InstrumentId( 1 ), id : OrderId( 1 ) } ).unwrap();

  let mut book = Book::new();
  for cmd in inbound_drain( &mut consumer )
  {
    inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();
  }
  ```
  Joins the Introduction's point 2 list on the one criterion that defines it — `exchange_core::submit` doesn't call it yet — but it is not a thin skeleton like most of that list's other ten: it genuinely moves data across a real ring, in its own OS threads, with its own two-producer and overflow phase-smokes (P28, P29). `ring_handle::Producer` can't be cloned, so "two producers" is built as two independent rings combined in a fixed lane order on drain, not one shared ring — a deliberate divergence from the proposal's single-ring assumption, not a limitation worked around silently.

## Smoke lanes

- **`smoke_exchange_core`** — depends on `exchange_core` alone, by design: a re-export missing from the facade is a build failure here, not a gap nobody notices. Three arms (crossing, control, cancel) graded by exit code.
  ```bash
  cargo run -p smoke_exchange_core
  ```
- **`smoke_exchange_phases`** — one tiny `demo_pNN_*` binary per incremental build phase (P01–P29 so far), each printing an exact golden line checked against `../golden_output/`. Depends on nearly every crate directly, deliberately — it grades each new contract in isolation, the moment it lands.
  ```bash
  cargo run -q -p smoke_exchange_phases --bin demo_p01_id
  ```

## Verify this guide yourself

The tier table is a derived fact, not an assertion — recompute the roots and the inflation point from `exchange_types` directly:

```bash
cd module
for d in */; do c="${d}Cargo.toml"; [ -f "$c" ] || continue
  deps=$(awk '/^\[dependencies\]/{f=1;next} /^\[/{f=0} f && NF && /^exchange_/{print}' "$c")
  [ -z "$deps" ] && echo "root: ${d%/}"
done
```

**Expected:** exactly the seven Tier-0 crates named above, nothing else.

## Sources

| File | Relationship |
|------|--------------|
| `module/*/Cargo.toml` | The `[dependencies]` blocks this guide's tier table was computed from, by direct `awk` extraction, not readme prose |
| [`../../module/readme.md`](../../module/readme.md) | The workspace's own flat crate index — this guide adds the tier/wiring dimension on top |
| [`../../readme.md`](../../readme.md) | The family root readme — orientation at the repo level; this guide is the module level |
| [`../dependency_tree/001_002_dependency_tree.md`](../dependency_tree/001_002_dependency_tree.md) | The *proposed* 23-crate graph (Roots/Trunk/Bridge/Facade) this guide's tiers were cross-checked against, then re-derived from the real build where the two diverge |
