//! Wall smoke `smoke_exchange_book` — the one run that exercises every
//! stage of this family at once, replacing `smoke_exchange_core`'s
//! two-arm lane.
//!
//! # Why this replaces `smoke_exchange_core` rather than sitting beside it
//!
//! `smoke_exchange_core` proved the five original crates could cross an
//! order and decline one. That question is now answered many times over by
//! `exchange_core`'s own `submission_test.rs`. This lane's job is the one
//! `smoke_exchange_core` was never built to do: walk the *whole* family —
//! multi-level matching, every `Tif`, halt, self-match prevention, the ring,
//! and the standalone idempotency/conservation auditors — in one scenario,
//! against one fixed golden block. Keeping both would mean two "the facade
//! works" smokes with no second thing either proves alone.
//!
//! # Why this depends on `exchange_core` for almost everything
//!
//! Same reasoning `smoke_exchange_core` already gave for depending on it
//! alone: if the facade stops re-exporting a type this lane needs, this
//! lane does not compile, which is a cheaper failure than a reader having to
//! notice the gap. The two further dependencies below are each a deliberate,
//! documented exception, not a quiet widening of that rule.
//!
//! # The two things `exchange_core` deliberately does not wire in
//!
//! `exchange_idem` and `exchange_conserve` are both real checks with no path
//! through the facade at all:
//!
//! - **Idempotency**: `Exchange::claim_order` always reassigns a submitted
//!   order's id internally, so a caller-chosen duplicate id can never be
//!   observed through [`Exchange::exchange_step`] — `exchange_core` has zero
//!   dependency on `exchange_idem` for exactly this reason. The `dup=1`
//!   field below checks `exchange_idem` directly and standalone, the same
//!   way `smoke_exchange_phases::demo_p13_dup` already does.
//! - **Conservation**: `exchange_match::cross` has zero dependency on
//!   `exchange_conserve` — settlement conservation is an external audit, not
//!   something the matching loop checks on its own behalf. The `conserve=0`
//!   field below runs that audit itself, on the real trades the scene below
//!   produced, rather than trusting that a batch which filled without error
//!   must have conserved.
//!
//! # Why price fields are repadded before printing
//!
//! `exact_kind::Decimal`'s `Display` trims trailing fractional zeros by
//! design (`1.00` renders as `1`) — correct for the type, but it conflicts
//! with the golden block's literal two-decimal text. `two_decimals` is
//! this lane's own concern alone; nothing about the shared type changes.
//!
//! # Why the four probes are isolated from the main scene
//!
//! `duplicate_rejected`, `ring_determinism`, `ring_overflow_rejects`, and
//! `self_trade_fill_count` never touch the [`Exchange`] the main [`scene`]
//! builds — each constructs its own `IdSet`, ring, or `Exchange` from
//! nothing. This mirrors `smoke_exchange_phases`'s own P13/P24/P28/P29
//! precedent of testing each property in isolation. For the first three
//! this is a matter of there being no [`Exchange`] involved at all; for
//! `self_trade_fill_count` it is load-bearing: the scene leaves a resting
//! sell on the book from its own halt round trip, and a self-trade probe
//! sharing that book would need its own buy priced *below* that leftover
//! order to avoid crossing it first — a fragile coincidence to depend on
//! when a fresh `Exchange` costs nothing and removes the risk by
//! construction instead.

use exchange_conserve::conserve_assert;
use exchange_core::
{
  AccountId, AssetId, Consumer, Exchange, ExchangeError, InboundCmd, InstrumentId, LevelView, Money, Order, OrderId,
  Price, Producer, Quantity, Receipt, RejectReason, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, Trade,
  inbound_flush, inbound_overflow_reject, inbound_ring,
};
use exchange_idem::{ IdSet, idem_insert };
use std::thread;

/// The one instrument the main scene trades — this lane never needed a
/// second one to make its point, same as `exchange_core`'s own
/// `submission_test.rs`.
const INSTRUMENT : InstrumentId = InstrumentId( 1 );

/// A [`Money`] from one of this lane's own literals.
///
/// # Panics
///
/// Panics if the literal does not parse, which would be this lane's bug
/// rather than the exchange's.
#[ must_use ]
pub fn money( text : &str ) -> Money
{
  Money::parse( text ).expect( "the lane's own literals are well-formed" )
}

/// A [`Price`] from one of this lane's own literals.
///
/// # Panics
///
/// As [`money`].
#[ must_use ]
pub fn price( text : &str ) -> Price
{
  Price::parse( text ).expect( "the lane's own literals are well-formed" )
}

/// A [`Quantity`] of whole units.
///
/// # Panics
///
/// Panics if the count is out of range, which would be this lane's bug
/// rather than the exchange's.
#[ must_use ]
pub fn units( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).expect( "the lane's own quantities are in range" )
}

/// Build an [`Order`] against [`INSTRUMENT`] with a placeholder id — every
/// real submission reassigns it via `Exchange::claim_order`, so the literal
/// value here is never observed.
fn order( account : AccountId, side : Side, price : Price, quantity : Quantity, tif : Tif ) -> Order
{
  Order { id : OrderId( 0 ), instrument : INSTRUMENT, account, side, price, quantity, tif, client : None }
}

/// The ring-fed equivalent of a direct submission — push one `Place`, step
/// once, return its outcome. Takes a pre-built [`Order`] rather than its
/// five scalar fields plus `tif` so the parameter count stays under
/// clippy's `too_many_arguments` threshold; see [`order`] for the builder.
fn submit
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  order : Order,
  policy : SelfMatchPolicy,
) -> Result< Receipt, ExchangeError >
{
  let quantity = order.quantity;
  let resting = Resting { order, remaining : quantity, arrival : Sequence( 0 ) };
  let pushed = inbound_flush( producer, [ InboundCmd::Place( resting ) ] );
  assert_eq!( pushed, 1, "the ring must accept a single command with headroom to spare" );

  match exchange.exchange_step( consumer, policy ).into_iter().next()
  {
    Some( StepOutcome::Placed( result ) ) => result,
    other => panic!( "submit() only ever pushes Place — got {other:?}" ),
  }
}

/// [`Price`] renders with trailing fractional zeros trimmed
/// (`exact_kind::Decimal`'s own documented behavior) — `1.00` prints as `1`,
/// `0.90` as `0.9`. The golden block's literal two-decimal text needs the
/// zeros put back; see this module's own doc for why that belongs here and
/// not on the shared type.
fn two_decimals( value : Price ) -> String
{
  let rendered = value.to_string();
  let ( whole, frac ) = rendered.split_once( '.' ).unwrap_or( ( rendered.as_str(), "" ) );
  let mut frac = frac.to_string();
  while frac.len() < 2
  {
    frac.push( '0' );
  }
  format!( "{whole}.{frac}" )
}

/// Format a batch of trades as the golden block's own `qty@price,qty@price` list.
fn fills_fmt( trades : &[ Trade ] ) -> String
{
  trades.iter().map( | trade | format!( "{}@{}", trade.quantity, two_decimals( trade.price ) ) ).collect::< Vec< _ > >().join( "," )
}

/// Format bid levels as the golden block's own `price:qty,price:qty` list.
fn depth_fmt( levels : &[ LevelView ] ) -> String
{
  levels.iter().map( | level | format!( "{}:{}", two_decimals( level.price ), level.qty ) ).collect::< Vec< _ > >().join( "," )
}

/// FNV-1a, folded over each drained command's identifying `OrderId` in
/// combine order. Order-sensitive by construction, matching
/// `smoke_exchange_phases::demo_p28_drain`'s own copy of this exact
/// function — duplicated rather than shared, since neither binary depends
/// on the other and the function is six lines.
fn checksum( cmds : &[ InboundCmd ] ) -> u64
{
  let mut hash : u64 = 0xcbf2_9ce4_8422_2325;
  for cmd in cmds
  {
    let id = match cmd
    {
      InboundCmd::Cancel { id, .. } => id.0,
      InboundCmd::Replace { old_id, .. } => old_id.0,
      InboundCmd::Place( resting ) => resting.order.id.0,
    };
    for byte in id.to_le_bytes()
    {
      hash ^= u64::from( byte );
      hash = hash.wrapping_mul( 0x0000_0100_0000_01b3 );
    }
  }
  hash
}

/// What the main sequential scene measured, in golden-block field order.
#[ derive( Debug ) ]
pub struct SceneReport
{
  /// The GTC taker's own two trades — the golden block's `fills`.
  pub gtc_fills : Vec< Trade >,
  /// The IOC taker's own two trades — folded into the conservation check
  /// alongside `gtc_fills`, but not separately printed.
  pub ioc_fills : Vec< Trade >,
  /// What is left resting at the best bid after the GTC taker, and that
  /// level's own price — together the golden block's `rest_bid`.
  pub rest_bid_qty : Quantity,
  /// See `rest_bid_qty`.
  pub rest_bid_price : Price,
  /// What the IOC taker left resting — the golden block's `ioc_rest`, zero
  /// since `exchange_core`'s own `tif_dropped` fix.
  pub ioc_rest : Quantity,
  /// Whether the FOK taker was dropped whole against a thin book, trading
  /// nothing — the golden block's `fok_rej`.
  pub fok_rejected : bool,
  /// Whether halting blocked a placement and clearing allowed the same
  /// placement again — the golden block's `halt`.
  pub halt_then_resume_ok : bool,
  /// The top two bid levels, captured right after the GTC taker and before
  /// the IOC taker depletes them further — the golden block's `depth`.
  pub depth_bids : Vec< LevelView >,
}

/// Run the main sequential scenario: three resting bids, a GTC taker that
/// partially sweeps them, an IOC taker that sweeps the rest and drops its
/// own remainder, a FOK taker rejected whole by the now-empty book, a
/// halt-then-resume round trip, and an isolated self-trade probe.
///
/// # Panics
///
/// Panics on any step that does not match the golden scenario — a resting
/// order that doesn't rest, a fill of the wrong size, an IOC/FOK remainder
/// that rests instead of dropping, a halt that doesn't block, a resumed
/// placement that's still refused, or a self-trade that fills.
#[ must_use ]
pub fn scene() -> SceneReport
{
  let maker = AccountId( 1 );
  let taker = AccountId( 2 );

  let mut exchange = Exchange::new();
  exchange.open_account( maker, money( "1000000" ), units( 100_000 ) ).expect( "fresh account, first deposit cannot overflow" );
  exchange.open_account( taker, money( "1000000" ), units( 100_000 ) ).expect( "fresh account, first deposit cannot overflow" );
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), price( "0.05" ), units( 1 ) )
  .expect( "a fresh instrument with a nonzero tick and lot registers" );

  let mut ring = inbound_ring( 8 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // Three resting bids: 10@1.00, 5@1.00, 4@0.95.
  submit( &mut exchange, &mut producer, &mut consumer, order( maker, Side::Buy, price( "1.00" ), units( 10 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded bid rests" );
  submit( &mut exchange, &mut producer, &mut consumer, order( maker, Side::Buy, price( "1.00" ), units( 5 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded bid rests" );
  submit( &mut exchange, &mut producer, &mut consumer, order( maker, Side::Buy, price( "0.95" ), units( 4 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded bid rests" );

  // GTC taker: ask 12@1.00 — fills 10 then 2, leaving 3@1.00 and 4@0.95 untouched.
  let gtc_taking = submit( &mut exchange, &mut producer, &mut consumer, order( taker, Side::Sell, price( "1.00" ), units( 12 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded ask at the bid crosses" );
  assert_eq!( gtc_taking.trades.len(), 2, "12 against 10-then-5 should take two maker levels at the same price" );
  assert_eq!( gtc_taking.trades[ 0 ].quantity, units( 10 ), "the first fill should exhaust the earlier 10@1.00 bid first" );
  assert_eq!( gtc_taking.trades[ 1 ].quantity, units( 2 ), "the second fill should take only what the order still wants" );
  assert!( gtc_taking.is_complete(), "12 units fully filled, nothing should be left to rest" );

  let depth = exchange.depth_get( INSTRUMENT, 2 ).expect( "two levels are on the book" );
  assert_eq!( depth.bids.len(), 2, "both the 1.00 remainder and the untouched 0.95 bid should show" );
  assert_eq!( depth.bids[ 0 ].price, price( "1.00" ), "the best bid is still 1.00" );
  assert_eq!( depth.bids[ 0 ].qty, units( 3 ), "5 - 2 = 3 should remain of the second bid" );
  assert_eq!( depth.bids[ 1 ].price, price( "0.95" ), "the second level is the untouched 0.95 bid" );
  assert_eq!( depth.bids[ 1 ].qty, units( 4 ), "the 0.95 bid was never touched" );

  // IOC taker: ask 100@0.90 — sweeps the rest (3@1.00, 4@0.95), drops the other 93.
  let ioc_taking = submit( &mut exchange, &mut producer, &mut consumer, order( taker, Side::Sell, price( "0.90" ), units( 100 ), Tif::Ioc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded IOC ask sweeps what it can" );
  assert_eq!( ioc_taking.trades.len(), 2, "the IOC should sweep both remaining levels" );
  assert_eq!( ioc_taking.trades[ 0 ].quantity, units( 3 ), "the 1.00 remainder goes first, best price first" );
  assert_eq!( ioc_taking.trades[ 1 ].quantity, units( 4 ), "then the whole 0.95 bid" );
  assert_eq!( ioc_taking.resting, Quantity::ZERO, "an IOC must never rest its unfilled remainder" );
  assert!( ioc_taking.tif_dropped, "the dropped 93 units should be attributed to tif_dropped, not a cancellation" );
  assert_eq!( exchange.book().len(), 0, "both bids are now gone and nothing new rested" );

  // FOK taker: ask 1000@1.00 against a now-empty book — rejected whole, book unchanged.
  let fok_taking = submit( &mut exchange, &mut producer, &mut consumer, order( taker, Side::Sell, price( "1.00" ), units( 1000 ), Tif::Fok ), SelfMatchPolicy::CancelIncoming )
  .expect( "a FOK that cannot fill in full is reported, not refused outright" );
  assert!( fok_taking.trades.is_empty(), "a FOK against an empty book must trade nothing" );
  assert_eq!( fok_taking.resting, Quantity::ZERO, "a FOK must never rest a partial attempt" );
  assert!( fok_taking.tif_dropped, "the whole 1000 units should be attributed to tif_dropped" );
  assert_eq!( exchange.book().len(), 0, "the book must be unchanged — still empty" );

  // Halt round trip: a new sell is refused while halted, then accepted once resumed.
  exchange.halt_set( INSTRUMENT ).expect( "a freshly registered instrument is not already halted" );
  let blocked = submit( &mut exchange, &mut producer, &mut consumer, order( maker, Side::Sell, price( "1.00" ), units( 1 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming );
  assert_eq!( blocked, Err( ExchangeError::Rejected( RejectReason::Halted ) ), "a halted instrument must refuse a new placement" );
  exchange.halt_clear( INSTRUMENT ).expect( "a halted instrument can be resumed" );
  let resumed = submit( &mut exchange, &mut producer, &mut consumer, order( maker, Side::Sell, price( "1.00" ), units( 1 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "the same placement must succeed once resumed" );
  assert_eq!( resumed.resting, units( 1 ), "nothing crosses it, so it rests in full" );

  SceneReport
  {
    gtc_fills : gtc_taking.trades,
    ioc_fills : ioc_taking.trades,
    rest_bid_qty : depth.bids[ 0 ].qty,
    rest_bid_price : depth.bids[ 0 ].price,
    ioc_rest : ioc_taking.resting,
    fok_rejected : fok_taking.tif_dropped,
    halt_then_resume_ok : blocked.is_err() && resumed.resting == units( 1 ),
    depth_bids : depth.bids,
  }
}

/// An isolated account rests a sell, then crosses it with its own buy under
/// `SelfMatchPolicy::CancelResting` — an account must never trade against
/// its own resting order. Built on its own fresh [`Exchange`], never the
/// main [`scene`]'s, so there is nothing else resting at another price to
/// sweep first; see this module's own doc for why that isolation is
/// load-bearing here. Mirrors
/// `smoke_exchange_phases::demo_p24_stp`'s own precedent.
///
/// Under `CancelResting` the *resting* leg is the one withdrawn, not the
/// incoming one — [`Receipt::self_match_cancelled`] tracks only whether the
/// incoming order's own leg was cancelled (true under
/// `CancelIncoming`/`CancelBoth`), so it stays `false` here even though a
/// self-match was correctly prevented. With nothing else resting behind the
/// withdrawn order, the incoming buy's match loop finds nothing further to
/// take and simply rests in full — see `exchange_match`'s own
/// `t10_cancel_resting_withdraws_the_resting_side_and_the_loop_resumes`
/// (same policy, but a second resting order behind the self-match, which is
/// what actually exercises the "loop resumes" half) for the policy's full
/// shape. `trades.is_empty()` is the one fact this scenario needs: zero
/// fills, which is the golden block's own `stp_fill=0`.
///
/// # Panics
///
/// Panics if the resting sell is itself refused, or if the crossing buy is
/// refused outright instead of reporting an outcome.
#[ must_use ]
pub fn self_trade_fill_count() -> usize
{
  let mut exchange = Exchange::new();
  let trader = AccountId( 1 );
  exchange.open_account( trader, money( "1000000" ), units( 100_000 ) ).expect( "fresh account, first deposit cannot overflow" );
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), price( "0.05" ), units( 1 ) )
  .expect( "a fresh instrument with a nonzero tick and lot registers" );
  let mut ring = inbound_ring( 8 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, order( trader, Side::Sell, price( "2.00" ), units( 2 ), Tif::Gtc ), SelfMatchPolicy::CancelIncoming )
  .expect( "a funded sell rests" );
  let crossing = submit( &mut exchange, &mut producer, &mut consumer, order( trader, Side::Buy, price( "2.00" ), units( 2 ), Tif::Gtc ), SelfMatchPolicy::CancelResting )
  .expect( "self-match prevention reports an outcome, not a refusal" );

  assert_eq!( crossing.resting, units( 2 ), "with the resting leg withdrawn and nothing else behind it, the incoming buy should rest in full" );
  crossing.trades.len()
}

/// Idempotency is checked standalone via `exchange_idem`, never through the
/// facade — see this module's own doc for why. Mirrors
/// `smoke_exchange_phases::demo_p13_dup`'s own precedent.
///
/// # Panics
///
/// Panics if the first sighting of an id is itself refused, which would be
/// an `exchange_idem` bug, not a duplicate.
#[ must_use ]
pub fn duplicate_rejected() -> bool
{
  let mut seen = IdSet::new();
  idem_insert( &mut seen, OrderId( 1 ) ).expect( "the first sighting of an id is never a duplicate" );
  idem_insert( &mut seen, OrderId( 1 ) ).is_err()
}

/// Race two producer threads against two independent rings, then combine in
/// a fixed lane order (ring A fully, then ring B) decided here, never by
/// which thread happened to finish first. "Two producers" is two rings,
/// never one ring with two producers — `Producer` cannot be cloned. See
/// `docs/decisions/001_two_producers_is_two_rings.md` and
/// `smoke_exchange_phases::demo_p28_drain`'s own copy of this scenario.
fn ring_determinism_once() -> u64
{
  let mut ring_a = inbound_ring( 64 ).expect( "a small power-of-two capacity is always valid" );
  let mut ring_b = inbound_ring( 64 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends_a = ring_a.ends();
  let mut ends_b = ring_b.ends();
  let ( mut producer_a, mut consumer_a ) = ends_a.split();
  let ( mut producer_b, mut consumer_b ) = ends_b.split();

  thread::scope( | scope |
  {
    scope.spawn( move ||
    {
      for id in 1..=4u64
      {
        producer_a.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).unwrap();
      }
    } );
    scope.spawn( move ||
    {
      for id in 101..=104u64
      {
        producer_b.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).unwrap();
      }
    } );
  } );

  // Fixed lane order, decided here — never by which thread finished first.
  let mut combined : Vec< InboundCmd > = consumer_a.drain().collect();
  combined.extend( consumer_b.drain() );
  checksum( &combined )
}

/// Two independent runs of the same two-ring race, which must reproduce the
/// same checksum despite genuine thread scheduling — `(a, b)`, printed as
/// `a=0x.. b=0x..` and asserted equal.
#[ must_use ]
pub fn ring_determinism() -> ( u64, u64 )
{
  ( ring_determinism_once(), ring_determinism_once() )
}

/// A ring at capacity 2: the third publish must be refused as an explicit
/// overflow reject, never a silent drop. Mirrors
/// `smoke_exchange_phases::demo_p29_over`'s own precedent.
///
/// # Panics
///
/// Panics if either of the first two publishes is itself refused, which
/// would mean capacity 2 does not even hold two.
#[ must_use ]
pub fn ring_overflow_rejects() -> bool
{
  let mut ring = inbound_ring( 2 ).expect( "capacity 2 is a valid power of two" );
  let mut ends = ring.ends();
  let ( mut producer, _consumer ) = ends.split();

  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } )
  .expect( "the first of two fits in a capacity-2 ring" );
  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 2 ) } )
  .expect( "the second of two fits in a capacity-2 ring" );

  let third = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 3 ) };
  inbound_overflow_reject( &mut producer, third ).is_err()
}

/// Whether a batch of real trades nets to zero across its buy and sell legs
/// — run here because nothing in the matching path checks this on its own
/// behalf; see this module's own doc.
#[ must_use ]
pub fn conserve_ok( fills : &[ Trade ] ) -> bool
{
  conserve_assert( fills ).is_ok()
}

/// Run the whole wall scenario, print the golden block, and assert every
/// field against `docs/golden_output/030_p30_wall_golden.md`'s literal text
/// — the one block that must hold for the whole workstream to be considered
/// closed.
///
/// # Panics
///
/// Panics on any field that does not match the golden block exactly, or on
/// anything [`scene`] itself asserts along the way.
pub fn run()
{
  let report = scene();
  let dup = duplicate_rejected();
  let ( a, b ) = ring_determinism();
  let overflow = ring_overflow_rejects();
  let stp_fill = self_trade_fill_count();

  let all_fills : Vec< Trade > = report.gtc_fills.iter().copied().chain( report.ioc_fills.iter().copied() ).collect();
  let conserve_violations = u8::from( !conserve_ok( &all_fills ) );

  let fills_line = fills_fmt( &report.gtc_fills );
  let rest_bid_line = format!( "{}@{}", report.rest_bid_qty, two_decimals( report.rest_bid_price ) );
  let depth_line = depth_fmt( &report.depth_bids );

  println!( "seed=1" );
  println!( "fills={fills_line}" );
  println!( "rest_bid={rest_bid_line}" );
  println!( "ioc_rest={}", report.ioc_rest );
  println!( "fok_rej={}", u8::from( report.fok_rejected ) );
  println!( "dup={}", u8::from( dup ) );
  println!( "halt={}", u8::from( report.halt_then_resume_ok ) );
  println!( "stp_fill={stp_fill}" );
  println!( "overflow={}", u8::from( overflow ) );
  println!( "conserve={conserve_violations}" );
  println!( "depth={depth_line}" );
  println!( "a=0x{a:x} b=0x{b:x}" );

  assert_eq!( fills_line, "10@1.00,2@1.00", "the GTC taker's own fills must match the golden block exactly" );
  assert_eq!( rest_bid_line, "3@1.00", "the remaining best bid must match the golden block exactly" );
  assert_eq!( report.ioc_rest, Quantity::ZERO, "an IOC must never rest a remainder" );
  assert!( report.fok_rejected, "a FOK against a thin book must be rejected whole" );
  assert!( dup, "a duplicate OrderId must be refused" );
  assert!( report.halt_then_resume_ok, "halting must block a placement and clearing must allow it again" );
  assert_eq!( stp_fill, 0, "an account must never trade against its own resting order" );
  assert!( overflow, "a full ring must reject the third publish, not drop it" );
  assert_eq!( conserve_violations, 0, "a batch of real fills must always conserve" );
  assert_eq!( depth_line, "1.00:3,0.95:4", "the captured depth must match the golden block exactly" );
  assert_eq!( a, b, "the same two producers, drained in the same fixed order, must reproduce the same checksum" );

  println!( "ok" );
}
