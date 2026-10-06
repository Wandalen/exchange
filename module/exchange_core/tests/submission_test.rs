//! Test Matrix T11 — one order through the facade, crossing all five crates.
//!
//! Everything here goes through [`Exchange`] and nothing reaches past it into
//! a sibling crate. That is the point of the row: the four crates below each
//! pass their own tests, and this is the only place their composition is
//! exercised. A family whose parts are all green and whose seam was never
//! run is a family nobody has used.

use exchange_core::
{
  AccountId, AssetId, CancelCause, Consumer, EscrowError, Event, EventKind, Exchange, ExchangeError, InboundCmd,
  InstrumentId, Money, Obligation, Order, OrderId, Producer, Quantity, Receipt, RejectReason, Resting,
  SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, inbound_flush, inbound_ring, verify,
};

/// The one instrument every test in this suite submits against —
/// multi-instrument routing through the facade is real now (see
/// `exchange_core`'s own module doc), this suite just never needed a second
/// one to make its points.
const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn money( text : &str ) -> Money
{
  Money::parse( text ).unwrap()
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

/// An exchange with two funded participants.
fn market() -> Exchange
{
  let mut exchange = Exchange::new();
  exchange.open_account( AccountId( 1 ), money( "1000" ), qty( 100 ) ).unwrap();
  exchange.open_account( AccountId( 2 ), money( "1000" ), qty( 100 ) ).unwrap();
  exchange
}

/// The ring-fed equivalent of the deleted `Exchange::submit( account, side,
/// price, quantity )` — see `exchange_core`'s own module doc, "`exchange_step`
/// replaces `submit`". `remaining`/`arrival` on the pushed [`Resting`] are
/// never read by [`Exchange::exchange_step`] (it only extracts `.order` from
/// a drained [`InboundCmd::Place`]), so both are placeholders; [`Tif::Gtc`]
/// and [`SelfMatchPolicy::CancelIncoming`] match the old hardcoded
/// submission semantics exactly, so every existing assertion below keeps its
/// original meaning.
fn submit
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  account : AccountId,
  side : Side,
  price : Money,
  quantity : Quantity,
) -> Result< Receipt, ExchangeError >
{
  let order = Order { id : OrderId( 0 ), instrument : INSTRUMENT, account, side, price, quantity, tif : Tif::Gtc };
  let resting = Resting { order, remaining : quantity, arrival : Sequence( 0 ) };
  let pushed = inbound_flush( producer, [ InboundCmd::Place( resting ) ] );
  assert_eq!( pushed, 1, "the ring must accept a single command with headroom to spare" );

  match exchange.exchange_step( consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
  {
    Some( StepOutcome::Placed( result ) ) => result,
    other => panic!( "submit() only ever pushes Place — got {other:?}" ),
  }
}

/// Same as [`submit`], but taking a caller-built [`Order`] directly instead
/// of separate scalar fields — `submit` itself stays pinned to [`Tif::Gtc`]
/// to keep every pre-existing assertion's meaning unchanged (see its own doc
/// comment); the IOC/FOK regression tests below need the other two values,
/// and threading an eighth scalar `tif` parameter through `submit`'s own
/// shape would cross clippy's `too_many_arguments` threshold for no reason —
/// taking the whole `Order` is both narrower and the same shape
/// `Resting`/`InboundCmd::Place` already wrap it in one line down.
fn submit_tif
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  order : Order,
) -> Result< Receipt, ExchangeError >
{
  let resting = Resting { order, remaining : order.quantity, arrival : Sequence( 0 ) };
  let pushed = inbound_flush( producer, [ InboundCmd::Place( resting ) ] );
  assert_eq!( pushed, 1, "the ring must accept a single command with headroom to spare" );

  match exchange.exchange_step( consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
  {
    Some( StepOutcome::Placed( result ) ) => result,
    other => panic!( "submit_tif() only ever pushes Place — got {other:?}" ),
  }
}

/// T11 — one order in, its trades out, through every crate in the family.
///
/// The assertions walk the whole path deliberately: the types crate shaped the
/// order, escrow reserved it, the book ranked it, matching crossed it, and the
/// facade settled and recorded it. Any one of those five failing to run would
/// leave one of these assertions wrong.
#[ test ]
fn t11_one_order_crosses_the_whole_family()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();
  assert!( resting.trades.is_empty(), "there was nothing to cross" );
  assert_eq!( resting.resting, qty( 10 ), "so all of it rests" );
  assert_eq!( exchange.book().len(), 1 );

  let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.50" ), qty( 4 ) ).unwrap();

  assert_eq!( taking.trades.len(), 1 );
  assert_eq!( taking.trades[ 0 ].quantity, qty( 4 ) );
  assert_eq!( taking.trades[ 0 ].price, money( "2.50" ) );
  assert_eq!( taking.trades[ 0 ].maker, resting.order );
  assert!( taking.is_complete() );

  // Escrow moved the units, not just the numbers.
  let seller = exchange.escrow().account( AccountId( 1 ) ).unwrap();
  let buyer = exchange.escrow().account( AccountId( 2 ) ).unwrap();
  assert_eq!( seller.cash.available(), money( "1010" ) );
  assert_eq!( seller.asset.available(), qty( 90 ) );
  assert_eq!( seller.asset.reserved(), qty( 6 ), "the unfilled remainder is still covered" );
  assert_eq!( buyer.asset.available(), qty( 104 ) );

  // And the book holds exactly the remainder.
  assert_eq!( exchange.book().best( INSTRUMENT, Side::Sell ).unwrap().remaining, qty( 6 ) );
}

/// A submission with nothing to cross is accepted and rests. Not an error.
#[ test ]
fn an_order_that_crosses_nothing_still_rests()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();

  let through = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.49" ), qty( 4 ) ).unwrap();

  assert!( through.trades.is_empty() );
  assert_eq!( through.resting, qty( 4 ) );
  assert_eq!( exchange.book().len(), 2, "two orders, opposite sides, not crossing" );
}

/// Reservation happens before matching, so an unfunded order never reaches the
/// book.
///
/// The order of operations, asserted from outside: if matching ran first, this
/// order would generate a trade and *then* fail to pay for it.
#[ test ]
fn an_unfunded_order_never_reaches_the_book()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();
  let before = exchange.book().len();

  let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "500.00" ), qty( 10 ) );

  assert_eq!( refused, Err( ExchangeError::Rejected( RejectReason::InsufficientFunds ) ) );
  assert_eq!( exchange.book().len(), before, "the book is exactly as it was" );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().asset.reserved(), qty( 10 ) );
}

/// A zero-quantity order is refused by validation, before escrow is consulted.
#[ test ]
fn a_zero_quantity_order_is_refused()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "2.50" ), Quantity::ZERO );

  assert_eq!( refused, Err( ExchangeError::Rejected( RejectReason::ZeroQuantity ) ) );
  assert_eq!( exchange.escrow().reservation_count(), 0 );
}

/// An unknown account is refused, and the refusal says so specifically.
#[ test ]
fn an_unknown_account_is_refused_by_name()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 99 ), Side::Buy, money( "2.50" ), qty( 1 ) );

  assert_eq!( refused, Err( ExchangeError::Rejected( RejectReason::UnknownAccount ) ) );
}

/// Cancelling through the facade releases the reservation.
#[ test ]
fn a_cancel_through_the_facade_returns_the_reservation()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "2.50" ), qty( 10 ) ).unwrap();
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.reserved(), money( "25" ) );

  let withdrawn = exchange.cancel( resting.order ).unwrap();

  assert_eq!( withdrawn, qty( 10 ) );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.available(), money( "1000" ) );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.reserved(), Money::ZERO );
  assert!( exchange.book().is_empty() );
}

/// Cancelling what does not rest is a reported outcome, not a panic.
#[ test ]
fn cancelling_an_absent_order_is_reported()
{
  let mut exchange = market();

  assert_eq!( exchange.cancel( OrderId( 42 ) ), Err( ExchangeError::NotResting( OrderId( 42 ) ) ) );
}

/// A filled order cannot then be cancelled — it is not resting.
#[ test ]
fn a_filled_order_cannot_be_cancelled()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 4 ) ).unwrap();
  let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.50" ), qty( 4 ) ).unwrap();

  assert!( matches!( exchange.cancel( taking.order ), Err( ExchangeError::NotResting( _ ) ) ) );
}

/// Every state change emitted exactly one event, and the positions are a gapless run.
///
/// Gap-freedom is what lets a consumer detect loss by arithmetic alone, with
/// no acknowledgement protocol — so a skipped or reused position is a defect
/// even when every event is otherwise correct.
#[ test ]
fn the_event_stream_numbers_every_event_without_a_gap()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.50" ), qty( 4 ) ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.50" ), Quantity::ZERO ).unwrap_err();
  exchange.cancel( resting.order ).unwrap();

  let positions : Vec< u64 > = exchange.events().iter().map( | event | event.sequence.0 ).collect();
  assert_eq!( positions, ( 0..positions.len() as u64 ).collect::< Vec< _ > >() );

  let kinds : Vec< &str > = exchange.events().iter().map( kind_name ).collect();
  assert_eq!
  (
    kinds,
    vec![ "accepted", "accepted", "trade", "rejected", "cancelled" ],
    "one event per state change, including the rejection",
  );
}

fn kind_name( event : &Event ) -> &'static str
{
  match event.kind
  {
    EventKind::OrderAccepted { .. } => "accepted",
    EventKind::OrderRejected { .. } => "rejected",
    EventKind::Trade( _ ) => "trade",
    EventKind::OrderCancelled { .. } => "cancelled",
  }
}

/// The acceptance event carries what was reserved, so the release is checkable
/// against it from the record alone.
#[ test ]
fn acceptance_records_what_it_reserved_and_cancel_records_what_it_returned()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "2.50" ), qty( 10 ) ).unwrap();
  exchange.cancel( resting.order ).unwrap();

  let reserved = exchange.events().iter().find_map( | event | match event.kind
  {
    EventKind::OrderAccepted { reserved, .. } => Some( reserved ),
    _ => None,
  } );
  let released = exchange.events().iter().find_map( | event | match event.kind
  {
    EventKind::OrderCancelled { released, cause, .. } =>
    {
      assert_eq!( cause, CancelCause::Request );
      Some( released )
    },
    _ => None,
  } );

  assert_eq!( reserved, Some( Obligation::Cash( money( "25" ) ) ) );
  assert_eq!( released, reserved, "the cancel returned exactly what acceptance took" );
}

/// A rejected order leaves no book state and no reservation — only its event.
#[ test ]
fn a_rejection_leaves_only_its_own_event_behind()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "500.00" ), qty( 100 ) ).unwrap_err();

  assert!( exchange.book().is_empty() );
  assert_eq!( exchange.escrow().reservation_count(), 0 );
  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.available(), money( "1000" ) );
  assert_eq!( exchange.events().len(), 1 );
}

/// The postings audit balances — checked by `exact_arith`'s own auditor.
///
/// Worth having because the auditor is machinery this family did not
/// write and that knows nothing about order books. A conservation check the
/// exchange authored itself would agree with the exchange by construction.
#[ test ]
fn the_posting_log_balances_under_the_conservation_auditor()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 4 ) ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "3.00" ), qty( 4 ) ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "3.00" ), qty( 8 ) ).unwrap();

  let postings = exchange.postings().unwrap();
  assert_eq!( postings.len(), 4, "two trades, two postings each" );

  let report = verify( &postings ).unwrap();
  assert!( report.is_balanced(), "{report}" );
  assert_eq!( report.discrepancy_minor(), 0 );
}

/// A sweep across two price levels pays each its own price.
///
/// The end-to-end form of the same rule the matching tests pin: 4 at 2.50 plus
/// 4 at 3.00 is 22, and not 8 x 3.00 = 24 nor 8 x 2.50 = 20. Both wrong
/// answers balance perfectly as postings, so only the amount catches them.
#[ test ]
fn a_sweep_pays_each_level_its_own_price()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 4 ) ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "3.00" ), qty( 4 ) ).unwrap();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "3.00" ), qty( 8 ) ).unwrap();

  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.available(), money( "978" ) );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.available(), money( "1022" ) );
}

/// The same submissions twice produce the same trades and the same book.
///
/// Replay determinism at the grain this slice can state it: a pure function of
/// the accepted sequence against an empty starting book.
#[ test ]
fn replaying_the_same_submissions_from_empty_gives_the_same_result()
{
  let run = ||
  {
    let mut exchange = market();
    let mut ring = inbound_ring( 8 ).unwrap();
    let mut ends = ring.ends();
    let ( mut producer, mut consumer ) = ends.split();

    submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.00" ), qty( 3 ) ).unwrap();
    submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), qty( 3 ) ).unwrap();
    let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2.00" ), qty( 5 ) ).unwrap();
    let resting : Vec< _ > = exchange.book().iter().map( | r | ( r.order.id, r.remaining ) ).collect();
    ( taking.trades, resting, exchange.postings().unwrap() )
  };

  assert_eq!( run(), run() );
}

/// A fill nobody can be paid for exactly is declined, not generated.
///
/// **Root Cause.** A sell's obligation is `Asset(quantity)` — `obligation()`
/// never computes a notional for it — so a resting ask can carry a quantity
/// whose value at its own price needs more decimals than `MONEY_SCALE` holds.
/// `settle` computes that notional and refuses it. But `cross` had already run
/// `consume_best`, and `submit` propagated the error with `?`, so the resting
/// order was gone and nothing had been paid for it.
///
/// **Why Not Caught.** Every prior test priced fills in whole currency units,
/// where a notional is exact for any quantity. The inexact case needs a price
/// with a fractional part *and* a fill quantity small enough that their product
/// falls below the scale — a combination no round-number fixture produces.
/// `submit`'s own doc asserted `ExchangeError::Escrow` was unreachable here, so
/// no test looked for it.
///
/// **Fix Applied.** `exchange_match::settleable` checks both notionals
/// settlement will compute — the trade price, and for a taking buy its own
/// limit as well — before `consume_best`. An unsettleable fill breaks the loop,
/// reaching the already-documented "no cross" outcome.
///
/// **Prevention.** The check sits before the irreversible step, matching the
/// submission path's own reserve-before-match rule. Anything that later widens
/// what `settle` computes has to extend `settleable` in the same change, or
/// this test fails.
///
/// **Pitfall.** A mutation and its justification separated by a crate boundary:
/// `cross` consumed the book, `settle` decided whether that was payable, and
/// nothing could undo the first once the second said no.
#[ test ]
fn a_fill_whose_notional_is_inexact_leaves_the_book_and_escrow_intact()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // One minor unit of asset at 0.5 — worth 0.0000005, which MONEY_SCALE = 6
  // cannot express. Accepted, because a sell reserves the asset, not a price.
  let ask = submit
  ( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "0.5" ), Quantity::from_minor( 1 ).unwrap() )
  .unwrap();
  assert_eq!( exchange.book().len(), 1 );

  let cash_before = exchange.escrow().total_cash().unwrap();
  let asset_before = exchange.escrow().total_asset().unwrap();

  // Crosses on price, and must still decline: the only available fill is that
  // same one minor unit.
  let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "0.5" ), qty( 1 ) ).unwrap();
  assert!( taking.trades.is_empty(), "an unpayable fill must not be generated" );

  // The ask survived. Before the fix it was consumed with nothing paid for it.
  assert_eq!( exchange.book().len(), 2, "the resting ask must still be on the book" );
  assert_eq!( exchange.escrow().total_cash().unwrap(), cash_before );
  assert_eq!( exchange.escrow().total_asset().unwrap(), asset_before );

  // And the seller can still get its reservation back — the part that was
  // unrecoverable before, since `cancel` on a vanished order returns
  // `NotResting` and no other release path exists.
  assert!( matches!( exchange.escrow().reserved_for( ask.order ), Some( Obligation::Asset( _ ) ) ) );
  exchange.cancel( ask.order ).unwrap();
  assert_eq!( exchange.escrow().reserved_for( ask.order ), None );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().asset.available(), qty( 100 ) );
}

/// The second notional settlement computes is checked too.
///
/// A taking buy is handed back the difference between its own limit and the
/// maker's price, which is a notional at the *taker's* price. Checking only the
/// trade price leaves this half reachable: 1.0 × one minor unit is exact, 1.5 ×
/// one minor unit is not, so this fill passes the first check and fails the
/// second.
#[ test ]
fn a_taking_buys_own_limit_is_checked_not_just_the_trade_price()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit
  ( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), Quantity::from_minor( 1 ).unwrap() )
  .unwrap();

  let asset_before = exchange.escrow().total_asset().unwrap();
  let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "1.50" ), qty( 1 ) ).unwrap();

  assert!( taking.trades.is_empty(), "the improvement notional is inexpressible" );
  assert_eq!( exchange.book().len(), 2 );
  assert_eq!( exchange.escrow().total_asset().unwrap(), asset_before );
}

/// A negative price is refused before it can rest, on both sides.
///
/// # Root Cause
///
/// `Price` is `Money`, which is signed, and `submit` validated only the
/// quantity. The two sides then failed differently, which is why one guard in
/// escrow was not enough. A *buy* at a negative price has a negative notional,
/// so escrow sees the sign and (since `EscrowError::NegativeAmount`) refuses
/// it. A *sell*'s obligation is `Obligation::Asset( quantity )` — the price
/// never enters it — so escrow has nothing to object to. The order reserved
/// cleanly and rested on the book at a negative price, waiting.
///
/// # Why Not Caught
///
/// Every price in this suite is one somebody would actually type. The suite
/// tests the *shape* of a limit order thoroughly — crossing, resting, price
/// improvement, self-match, inexact notionals — and never asks whether the
/// price is a price at all. A resting sell is also the one place a field
/// survives an entire matching step without being consumed, so the reachable
/// consequence is separated from the submission by an unbounded number of
/// intervening orders.
///
/// # Fix Applied
///
/// `submit` step 1 refuses `price < Money::ZERO` with
/// `RejectReason::NegativePrice`, before the reservation, so neither side can
/// reach the book. Escrow's own `EscrowError::NegativeAmount` stays as the
/// backstop for callers reaching `Escrow` directly.
///
/// # Prevention
///
/// Validate a field where it *enters*, not where its value is consumed. A
/// guard at the consumption site is invisible to every path that carries the
/// field past that site untouched — which is exactly what resting is.
///
/// # Pitfall
///
/// The failure this replaces was worse than a wrong answer. The negative sell
/// rested, and the first order to cross it returned `Err` from step 4 — after
/// `cross` had already mutated the book and after the aggressor's reservation
/// had been taken, with no unwind and no event emitted. The aggressor's cash
/// stayed locked against a reservation it could not name to cancel, in a crate
/// whose protocol says no state changes without an event.
#[ test ]
fn a_negative_price_is_refused_before_it_can_rest()
{
  for side in [ Side::Buy, Side::Sell ]
  {
    let mut exchange = market();
    let mut ring = inbound_ring( 8 ).unwrap();
    let mut ends = ring.ends();
    let ( mut producer, mut consumer ) = ends.split();

    let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), side, money( "-2.00" ), qty( 10 ) );

    assert_eq!
    (
      refused,
      Err( ExchangeError::Rejected( RejectReason::NegativePrice ) ),
      "a {side:?} at -2.00",
    );
    assert_eq!( exchange.book().len(), 0, "and it did not rest" );
    assert_eq!( exchange.escrow().reservation_count(), 0, "nor hold a reservation" );

    // The rejection is recorded. Before the guard the *sell* was accepted
    // silently, and the order that later crossed it changed state and emitted
    // nothing at all.
    let events = exchange.events();
    assert_eq!( events.len(), 1 );
    assert!
    (
      matches!( events[ 0 ].kind, EventKind::OrderRejected { reason : RejectReason::NegativePrice } ),
      "{:?}", events[ 0 ].kind,
    );
  }

  // Zero is not negative, and is deliberately still accepted — a transfer at
  // zero conserves exactly. Pinned so that widening the guard to `<=` has to
  // be a decision rather than a slip.
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let free = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, Money::ZERO, qty( 10 ) );
  assert!( free.is_ok(), "{free:?}" );
  assert_eq!( exchange.book().len(), 1 );
}

/// An order refused for a genuinely representable-ceiling reason is reported
/// as such, not folded into `RejectReason::InsufficientFunds` when the
/// account's funds are, in fact, ample.
///
/// # Root Cause
///
/// `Exchange::reason_for` mapped every `EscrowError` other than
/// `UnknownAccount`/`Obligation` to `RejectReason::InsufficientFunds`, on the
/// documented claim that nothing else was reachable from `submit`'s call to
/// `Escrow::reserve`. `Holding::reserve` checks `available` and `reserved`
/// independently, and only the first of those is a funds check — the second,
/// `reserved.checked_plus(amount)`, is a ceiling check on the account's
/// *pooled* commitment across every one of its live orders, and it can fail
/// on its own.
///
/// # Why Not Caught
///
/// Every account in the existing suite holds one reservation at a time, so
/// `available` and `reserved` never drift far enough apart to matter. Seeing
/// them diverge needs an account that both (a) already carries one very large
/// reservation, and (b) separately earns fresh, genuinely spendable funds —
/// settling as a seller — before placing a second small order: `available`
/// comfortably covers the second order, but folding it into `reserved`
/// alongside the first commitment does not fit the ceiling.
///
/// # Fix Applied
///
/// `reason_for` now maps `EscrowError::Arithmetic` to its own
/// `RejectReason::ReservationUnrepresentable` and matches every remaining
/// `EscrowError` variant exhaustively (no wildcard), so a future variant
/// reaching `reserve` has to be given a considered reason rather than
/// silently inheriting the funds-shortfall bucket.
///
/// # Prevention
///
/// A catch-all reason is a claim that every variant it swallows is either
/// equivalent or unreachable. That claim has an expiry date the moment a new
/// variant is added to the error enum it swallows from — verify it again
/// rather than assuming the bundle still holds.
///
/// # Pitfall
///
/// The two ceilings look identical from the caller's side — both refuse an
/// order that would otherwise have gone through — but they call for different
/// responses: a real shortfall needs more funds, a pooled-reservation ceiling
/// needs fewer *simultaneous* live orders on the account, and conflating them
/// sends the caller looking for the wrong fix.
#[ test ]
fn a_reservation_ceiling_breach_is_distinguished_from_a_funds_shortfall()
{
  let mut exchange = Exchange::new();
  // Account 1 opens holding the whole-unit ceiling in cash outright, plus
  // enough asset to sell later.
  exchange.open_account( AccountId( 1 ), money( "9000000000" ), qty( 600 ) ).unwrap();
  exchange.open_account( AccountId( 2 ), money( "2000" ), qty( 0 ) ).unwrap();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // Order 1: a huge resting buy at $1 that reserves nearly all of account 1's
  // cash — 500 minor units short of the whole-unit ceiling in minor units.
  let huge = Quantity::from_minor( 8_999_999_999_999_500 ).unwrap();
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "1" ), huge ).unwrap();

  // Order 2: account 1 separately rests a sell for 500 of the asset, priced
  // at $2 — above order 1's own $1 buy, so the two do not cross each other
  // (which would otherwise trigger this account's own self-match policy).
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2" ), qty( 500 ) ).unwrap();

  // Account 2 crosses it at $2, paying account 1 $1000 that account 1's
  // original cash deposit never budgeted for — fresh, genuinely spendable
  // funds landing in `available` on top of the huge reservation already
  // sitting in `reserved`.
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2" ), qty( 500 ) ).unwrap();

  let account_before = *exchange.escrow().account( AccountId( 1 ) ).unwrap();
  assert_eq!( account_before.cash.reserved().minor(), 8_999_999_999_999_500, "sanity: still holding order 1's reservation" );
  assert!
  (
    account_before.cash.available() > money( "0.001" ),
    "far more than order 3 costs is genuinely spendable: {:?}", account_before.cash.available(),
  );

  let events_before = exchange.events().len();

  // Order 3: a vanishingly small buy — $0.001 — well within `available`, but
  // `reserved` has only 500 minor units of headroom left before the ceiling.
  let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "1000" ), Quantity::from_minor( 1 ).unwrap() );

  assert_eq!
  (
    refused,
    Err( ExchangeError::Rejected( RejectReason::ReservationUnrepresentable ) ),
    "funds are sufficient; only the pooled reservation ceiling is breached",
  );
  assert_eq!( *exchange.escrow().account( AccountId( 1 ) ).unwrap(), account_before, "a rejected order moves nothing" );
  assert_eq!( exchange.events().len(), events_before + 1, "exactly the rejection event was recorded" );
  assert!
  (
    matches!
    (
      exchange.events().last().unwrap().kind,
      EventKind::OrderRejected { reason : RejectReason::ReservationUnrepresentable },
    ),
    "{:?}", exchange.events().last().unwrap().kind,
  );
}

/// An order whose notional needs more precision than the currency scale holds
/// is refused as `ObligationUnrepresentable`, not folded into
/// `InsufficientFunds` — the account's funds are irrelevant here; the amount
/// itself has no exact representation at all.
///
/// Coverage gap, not a behavioural fix: `reason_for`'s
/// `EscrowError::Obligation(_) => RejectReason::ObligationUnrepresentable` arm
/// was already correct and already exhaustively matched, but nothing in this
/// crate's own suite had ever driven it through `Exchange::submit` end to
/// end — every other test's price/quantity pair happened to divide evenly.
#[ test ]
fn an_inexact_notional_is_refused_as_unrepresentable_not_as_a_shortfall()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // 0.3 x 0.000001 = 0.0000003 — a seventh decimal digit `Money` (scale 6)
  // cannot hold. The account has ample funds; the amount itself cannot be
  // expressed regardless.
  let refused = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "0.3" ), Quantity::from_minor( 1 ).unwrap() );

  assert_eq!
  (
    refused,
    Err( ExchangeError::Rejected( RejectReason::ObligationUnrepresentable ) ),
    "the notional cannot be expressed at all, regardless of what the account holds",
  );
  assert_eq!( exchange.book().len(), 0, "an obligation that cannot be expressed never reaches the book" );
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.reserved(), Money::ZERO, "nothing was reserved" );
}

/// A cancel whose escrow release fails does not remove the order from the
/// book first — the release has to actually succeed before that mutation
/// commits, not after.
///
/// # Root Cause
///
/// `Exchange::cancel` called the infallible, order-removing
/// `self.book.cancel(id)` one statement before the fallible
/// `self.escrow.release(...)` — the same commit-before-validate shape as
/// `Escrow::release`'s own already-fixed bug, one level higher in the call
/// stack. `escrow.release` can refuse an id `book.cancel` just confirmed
/// rests: concretely `EscrowError::Arithmetic`, when the owning account's
/// `available` has grown — via unrelated trades settling after this order
/// was accepted — close enough to the representable ceiling that returning
/// this order's own reservation would breach it. On that refusal `cancel`
/// returned `Err`, but the order was already gone from the book, with no
/// event and no id left for any future retry to name.
///
/// # Why Not Caught
///
/// Every existing cancel test releases a reservation into an account with
/// ample headroom below the ceiling. Seeing `release` fail needs an account
/// that reserved for one order early, then separately earned fresh,
/// genuinely spendable funds — settling as the maker on an unrelated,
/// higher-priced resting order — until `available` alone sits at the
/// ceiling, with the first order's own reservation still untouched on top of
/// it.
///
/// # Fix Applied
///
/// `cancel` now looks up the resting order's account through `Book::iter`
/// (read-only) first, calls `escrow.release` before the book is touched at
/// all, and only removes the order once that release has actually
/// succeeded — the same validate-before-commit ordering `Escrow::release`
/// itself already uses internally.
///
/// # Prevention
///
/// Whenever one step is infallible and irreversible and the next is
/// fallible, the fallible step has to run first, never the reverse — so a
/// `?` short-circuit never leaves the first step's effect stranded with
/// nothing left to undo it.
///
/// # Pitfall
///
/// The reservation itself was never corrupted by this bug — `Escrow::release`
/// keeps the ledger entry and the account's `reserved` balance untouched on
/// its own failure, per its own already-fixed ordering. What was lost is the
/// *order*: once `book.cancel` had already removed it, no id remained for a
/// future retry to name, so an otherwise perfectly intact reservation became
/// permanently unreachable through the public API the moment the account's
/// balance no longer had room to absorb it.
#[ test ]
fn a_cancel_that_fails_to_release_leaves_the_order_resting()
{
  let mut exchange = Exchange::new();
  // Account 1 starts three cash units short of the whole-unit ceiling, plus
  // enough asset to sell later.
  exchange.open_account( AccountId( 1 ), money( "8999999997" ), qty( 10 ) ).unwrap();
  exchange.open_account( AccountId( 2 ), money( "1000" ), Quantity::ZERO ).unwrap();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // Order 1: a small resting buy that reserves 1 unit of cash — the
  // reservation this test tries, and fails, to release.
  let order_1 = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "1" ), qty( 1 ) ).unwrap();
  assert_eq!( exchange.escrow().account( AccountId( 1 ) ).unwrap().cash.reserved(), money( "1" ) );

  // Order 2: account 1 separately rests a sell for 2 of the asset, priced
  // above order 1's own $1 buy so the two do not cross each other (which
  // would otherwise trigger this account's own self-match policy).
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2" ), qty( 2 ) ).unwrap();

  // Account 2 crosses it at $2, paying account 1 exactly the 4 cash units
  // that bring `available` to precisely the whole-unit ceiling — fresh,
  // genuinely spendable funds landing on top of order 1's reservation, still
  // sitting untouched in `reserved`.
  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "2" ), qty( 2 ) ).unwrap();

  let account = exchange.escrow().account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "9000000000" ), "sanity: available sits exactly at the ceiling" );
  assert_eq!( account.cash.reserved(), money( "1" ), "sanity: order 1's reservation is still untouched" );

  // Cancelling order 1 would return its 1 unit to `available` — one more
  // than the ceiling allows.
  let first = exchange.cancel( order_1.order );
  assert_eq!( first, Err( ExchangeError::Escrow( EscrowError::Arithmetic ) ) );

  // The fix: order 1 is still resting, not silently gone. A second attempt
  // sees the exact same order and fails the exact same way — proof nothing
  // was stranded by the first attempt.
  assert!
  (
    exchange.book().iter().any( | resting | resting.order.id == order_1.order ),
    "order 1 must still rest after a release failure",
  );
  let second = exchange.cancel( order_1.order );
  assert_eq!( second, first, "a retry sees the same order and fails the same way, not `NotResting`" );

  // Escrow itself is untouched by either attempt.
  let account = exchange.escrow().account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "9000000000" ) );
  assert_eq!( account.cash.reserved(), money( "1" ) );
  assert_eq!( exchange.escrow().reserved_for( order_1.order ), Some( Obligation::Cash( money( "1" ) ) ) );

  // No cancellation event was ever recorded for order 1.
  assert!
  (
    !exchange.events().iter().any( | event | matches!( event.kind, EventKind::OrderCancelled { .. } ) ),
    "a failed release must not be reported as a completed cancellation",
  );
}

/// A crossing whose second trade cannot settle commits nothing at all — not
/// the first trade, not the incoming order's own reservation, and not the
/// resting orders `cross` already consumed to produce both trades.
///
/// # Root Cause
///
/// `Exchange::submit` mutated real state one step at a time, each guarded
/// individually by `?`: `cross` withdrew every resting order it matched from
/// the book unconditionally, before either trade was settled, and each
/// trade's `escrow.settle` call committed for real, with its own event
/// emitted, before the next trade in the same crossing was even attempted.
/// `Holding::receive` — the edge `settle` uses to credit a party's proceeds —
/// is an unconditional, non-reservation-bounded credit, so a second trade in
/// one crossing could breach an account's representable ceiling even though
/// the first trade, and the incoming order's own reservation and
/// `OrderAccepted` event, had already committed for real by the time the
/// second trade's `settle` call failed.
///
/// # Why Not Caught
///
/// Every existing crossing test either produces exactly one trade per
/// `submit` call, or settles every trade against accounts with ample
/// headroom below the ceiling. Seeing this needs an incoming order that
/// crosses *two* resting orders in one call, against a taker account whose
/// available balance is exactly one unit of proceeds short of the ceiling —
/// so the first trade lands precisely at the ceiling and the second alone
/// overflows it.
///
/// # Fix Applied
///
/// `submit` now dry-runs the whole operation — reserve, cross, settle every
/// trade, release every self-match cancellation — against clones of the
/// escrow and the book before any of it touches real state. Only once the
/// dry run succeeds in full does `submit` replay the identical sequence for
/// real; a dry-run failure instead becomes a single `OrderRejected` event via
/// the same `reject`/`reason_for` path step 2's own reservation failure
/// already used, with nothing else in `self.escrow` or `self.book` ever
/// touched.
///
/// # Prevention
///
/// Whichever step first commits real, hard-to-reverse state (removing a
/// resting order from the book; crediting a party's proceeds) must not run
/// until every later, still-fallible step in the same operation has already
/// been proven to succeed — a dry run against scratch clones is what proves
/// it, when the steps cannot themselves be reordered.
///
/// # Pitfall
///
/// The incoming order's own step-2 reservation looks like it commits safely
/// before the risky part (crossing/settlement) even begins — it is
/// `?`-guarded and fails cleanly on its own. What is easy to miss is that
/// step 2 committing at all, unconditionally, before step 3 is attempted, is
/// itself the hazard: a later failure has nothing to roll step 2 back with,
/// so the fix has to dry-run reserve through release as one unit, not just
/// the crossing/settlement portion that looks like where the risk lives.
#[ test ]
fn a_crossing_whose_second_trade_cannot_settle_commits_nothing()
{
  let mut exchange = Exchange::new();
  // Account 1 starts one cash unit short of the whole-unit ceiling, plus
  // enough asset to sell into two separate trades.
  exchange.open_account( AccountId( 1 ), money( "8999999999" ), qty( 10 ) ).unwrap();
  exchange.open_account( AccountId( 2 ), money( "10" ), Quantity::ZERO ).unwrap();
  exchange.open_account( AccountId( 3 ), money( "10" ), Quantity::ZERO ).unwrap();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // Two resting buys at the same price, account 2 first — so `cross`
  // matches account 2's trade first, landing account 1's cash exactly at the
  // ceiling, and account 3's trade second, one unit past it.
  let buy_2 = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 2 ), Side::Buy, money( "1" ), qty( 1 ) ).unwrap();
  let buy_3 = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 3 ), Side::Buy, money( "1" ), qty( 1 ) ).unwrap();
  assert_eq!( exchange.book().side( INSTRUMENT, Side::Buy ).count(), 2, "sanity: both rest before the crossing sell arrives" );

  let events_before = exchange.events().len();

  // One sell, crossing both — the crossing this bug needs.
  let result = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1" ), qty( 2 ) );
  assert_eq!
  (
    result,
    Err( ExchangeError::Rejected( RejectReason::ReservationUnrepresentable ) ),
    "the second trade's proceeds would breach account 1's cash ceiling",
  );

  // Nothing committed: account 1's cash and asset are exactly as they
  // started — no real reserve ever ran, because the dry run failed first.
  let account_1 = exchange.escrow().account( AccountId( 1 ) ).unwrap();
  assert_eq!( account_1.cash.available(), money( "8999999999" ), "no proceeds from either trade landed" );
  assert_eq!( account_1.cash.reserved(), money( "0" ), "a sell never reserves cash in the first place" );
  assert_eq!( account_1.asset.available(), qty( 10 ), "the sell's own reservation never ran for real" );
  assert_eq!( account_1.asset.reserved(), Quantity::ZERO );

  // Both resting buys are exactly as `cross`'s own dry run found them —
  // neither was withdrawn from the real book.
  assert_eq!( exchange.book().side( INSTRUMENT, Side::Buy ).count(), 2, "cross's dry run never touched the real book" );
  assert!( exchange.book().side( INSTRUMENT, Side::Buy ).any( | resting | resting.order.id == buy_2.order ) );
  assert!( exchange.book().side( INSTRUMENT, Side::Buy ).any( | resting | resting.order.id == buy_3.order ) );

  // Exactly one event was recorded for the failed sell: its rejection.
  let new_events = &exchange.events()[ events_before .. ];
  assert_eq!( new_events.len(), 1, "no OrderAccepted, no Trade, only the rejection" );
  assert!( matches!( new_events[ 0 ].kind, EventKind::OrderRejected { reason : RejectReason::ReservationUnrepresentable } ) );
  assert!
  (
    !exchange.events().iter().any( | event | matches!( event.kind, EventKind::Trade( _ ) ) ),
    "no trade was ever recorded, including the one that would have fit",
  );

  // A retry sees the identical, still-untouched state and fails the same way.
  let retry = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1" ), qty( 2 ) );
  assert_eq!( retry, result, "a retry fails the same way, not differently, from an already-corrupted state" );
}

/// A self-match-cancelled order traded nothing, and `is_complete` must not
/// say otherwise just because nothing is left to rest either.
///
/// # Root Cause
///
/// `Receipt::is_complete` returned `self.resting == Quantity::ZERO`, but
/// `resting` reaches zero two different ways: every unit found a
/// counterparty, or `submit` step 4a withdrew the incoming order's own
/// remainder because it could only have crossed itself. Both leave `resting`
/// at zero; only the first is a fill. The method's own doc promises
/// "entirely filled", which a self-match-cancelled order — zero trades,
/// nothing rested — is not.
///
/// # Why Not Caught
///
/// No test in this suite ever submitted a crossing order under the same
/// account as the resting order it would cross. Every existing `is_complete`
/// assertion checks a genuine fill, where the two zero-reasons happen to
/// agree, so the conflation had nothing to disturb it.
///
/// # Fix Applied
///
/// `Receipt` gained a `self_match_cancelled` field, set from `submit`'s own
/// `incoming_cancelled` flag, and `is_complete` now returns
/// `self.resting == Quantity::ZERO && !self.self_match_cancelled`.
///
/// # Prevention
///
/// A boolean derived from "is some other quantity zero" is only as correct
/// as the claim that zero has one cause. Adding a second way to reach the
/// same zero — as self-match cancellation did to `resting` — obligates a
/// re-check of every method that reads that zero as a signal.
///
/// # Pitfall
///
/// `resting` itself is not wrong here — nothing does rest, so zero is the
/// right value for the field. The bug is entirely in a *consumer* of that
/// field inferring "filled" from "empty" without accounting for the other
/// way the incoming order's remainder can vanish.
#[ test ]
fn a_self_match_cancelled_order_is_not_reported_complete()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  // Account 1 rests a sell, then crosses its own resting order — the
  // hardcoded `SelfMatchPolicy::CancelIncoming` withdraws the incoming buy's
  // remainder whole, since the only liquidity it could take is its own.
  let resting = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();
  let taking = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Buy, money( "2.50" ), qty( 4 ) ).unwrap();

  assert!( taking.trades.is_empty(), "a self-cross never trades" );
  assert_eq!( taking.resting, Quantity::ZERO, "the remainder was withdrawn, not rested" );
  assert!( taking.self_match_cancelled, "the withdrawal was this exact cause" );
  assert!( !taking.is_complete(), "cancelled whole is not filled — nothing was traded" );

  // The resting sell survives, untouched, per `CancelIncoming`'s own contract.
  assert_eq!( exchange.book().len(), 1 );
  assert!( exchange.book().iter().any( | r | r.order.id == resting.order ) );

  // And the cancellation, not a fill, is what the event stream itself says.
  assert!
  (
    exchange.events().iter().any( | event | matches!
    (
      event.kind,
      EventKind::OrderCancelled { cause : CancelCause::SelfMatch, .. },
    ) ),
    "the self-match cancellation must be on record",
  );
}

/// A resting order withdrawn by self-match prevention counts as a cancel;
/// the incoming one does not, since it never rested.
#[ test ]
fn a_self_match_cancelled_resting_order_counts_as_a_cancel()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 10 ) ).unwrap();

  let order = Order { id : OrderId( 0 ), instrument : INSTRUMENT, account : AccountId( 1 ), side : Side::Buy, price : money( "2.50" ), quantity : qty( 4 ), tif : Tif::Gtc };
  inbound_flush( &mut producer, [ InboundCmd::Place( Resting { order, remaining : order.quantity, arrival : Sequence( 0 ) } ) ] );
  let outcomes = exchange.exchange_step( &mut consumer, SelfMatchPolicy::CancelResting );
  assert!( matches!( outcomes[ 0 ], StepOutcome::Placed( Ok( ref r ) ) if r.trades.is_empty() ) );

  assert_eq!( exchange.book().len(), 1, "the resting sell was withdrawn and the buy rested instead" );
  assert_eq!( exchange.stats_get().cancels, 1 );
}

/// A post-only order that would take is rejected on record and reserves
/// nothing; one that would not take rests.
#[ test ]
fn a_post_only_order_rests_or_is_rejected_but_never_takes()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "2.50" ), qty( 4 ) ).unwrap();
  let reservations = exchange.escrow().reservation_count();

  let post_only = | price | Order { id : OrderId( 0 ), instrument : INSTRUMENT, account : AccountId( 2 ), side : Side::Buy, price : money( price ), quantity : qty( 4 ), tif : Tif::PostOnly };

  let taking = submit_tif( &mut exchange, &mut producer, &mut consumer, post_only( "2.50" ) );
  assert_eq!( taking, Err( ExchangeError::Rejected( RejectReason::PostOnlyWouldTake ) ) );
  assert_eq!( exchange.escrow().reservation_count(), reservations, "a refused order reserves nothing" );
  assert!( matches!
  (
    exchange.events().last().unwrap().kind,
    EventKind::OrderRejected { reason : RejectReason::PostOnlyWouldTake },
  ) );

  let resting = submit_tif( &mut exchange, &mut producer, &mut consumer, post_only( "2.45" ) ).unwrap();
  assert!( resting.trades.is_empty() );
  assert_eq!( resting.resting, qty( 4 ) );
  assert_eq!( exchange.book().len(), 2 );
}

/// An IOC taker that only partially fills must not rest its remainder —
/// `exchange_match::cross` never inserts a remainder for any TIF by its own
/// design (see that crate's module doc); whether a caller rests one is the
/// caller's decision, and `Exchange::step_place` must decline for IOC the
/// same way `demo_p22_ioc`'s own direct `tif_rests` check does.
#[ test ]
fn an_ioc_taker_never_rests_its_remainder_through_the_facade()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), qty( 4 ) ).unwrap();

  let taker = Order
  {
    id : OrderId( 0 ), instrument : INSTRUMENT, account : AccountId( 2 ), side : Side::Buy,
    price : money( "1.00" ), quantity : qty( 10 ), tif : Tif::Ioc,
  };
  let taking = submit_tif( &mut exchange, &mut producer, &mut consumer, taker ).unwrap();

  assert_eq!( taking.trades.len(), 1, "it should still fill against the one resting maker" );
  assert_eq!( taking.resting, Quantity::ZERO, "IOC must report nothing resting" );
  assert!( taking.tif_dropped, "the unfilled 6 units were dropped by Tif, not resolved some other way" );
  assert_eq!( exchange.book().len(), 0, "IOC's own unfilled remainder must never reach the book" );

  // The dropped remainder's reservation must come back, not leak — paid 4.00
  // for the 4 units that filled, nothing held against the other 6.
  assert_eq!( exchange.escrow().reserved_for( taking.order ), None, "no reservation may survive a TIF-dropped remainder" );
  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.available(), money( "996" ) );
  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.reserved(), Money::ZERO );

  assert!
  (
    exchange.events().iter().any( | event | matches!
    (
      event.kind,
      EventKind::OrderCancelled { cause : CancelCause::TimeInForce, quantity, .. } if quantity == qty( 6 ),
    ) ),
    "the dropped remainder's cause and quantity must be on record",
  );
}

/// A FOK taker that cannot be filled in full against a thin book must be
/// rejected whole, with the book left exactly as it was — `cross` reports an
/// unfillable FOK as an ordinary no-cross outcome (empty trades, full
/// remaining, `Ok`, per that crate's own module doc), so `Exchange::step_place`
/// is the one place that must translate "FOK, nothing filled" into "reject,
/// don't rest" rather than resting the untouched full quantity by accident.
#[ test ]
fn a_fok_taker_rejects_whole_against_a_thin_book_through_the_facade()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), qty( 4 ) ).unwrap();
  let before = exchange.book().clone();

  let taker = Order
  {
    id : OrderId( 0 ), instrument : INSTRUMENT, account : AccountId( 2 ), side : Side::Buy,
    price : money( "1.00" ), quantity : qty( 10 ), tif : Tif::Fok,
  };
  let taking = submit_tif( &mut exchange, &mut producer, &mut consumer, taker ).unwrap();

  assert!( taking.trades.is_empty(), "a thin book cannot fill this FOK in full" );
  assert_eq!( taking.resting, Quantity::ZERO, "FOK must never rest when it cannot fill completely" );
  assert!( taking.tif_dropped, "the whole order was dropped by Tif, not resolved some other way" );
  assert_eq!( exchange.book(), &before, "the book must be byte-for-byte unchanged after an unfillable FOK" );

  // Nothing filled, so the full reservation must come all the way back —
  // not stay stranded because the order never got to rest and release it.
  assert_eq!( exchange.escrow().reserved_for( taking.order ), None, "no reservation may survive a rejected FOK" );
  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.available(), money( "1000" ) );
  assert_eq!( exchange.escrow().account( AccountId( 2 ) ).unwrap().cash.reserved(), Money::ZERO );
}

/// A halted instrument refuses a new placement, and resuming it allows one
/// again — `halt_set`/`halt_clear` toggled `InstrumentSpec::halted`
/// correctly from the day they were built, but nothing in `step_place` ever
/// consulted it until now (see that method's own
/// `Fix(halt_set_never_actually_blocked_a_placement)` comment).
#[ test ]
fn a_halted_instrument_refuses_placement_and_resuming_allows_it_again()
{
  let mut exchange = market();
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), money( "0.05" ), qty( 1 ) ).unwrap();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  exchange.halt_set( INSTRUMENT ).unwrap();
  let halted = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), qty( 4 ) );
  assert_eq!( halted, Err( ExchangeError::Rejected( RejectReason::Halted ) ) );
  assert_eq!( exchange.book().len(), 0, "a refused placement must never reach the book" );

  exchange.halt_clear( INSTRUMENT ).unwrap();
  let resumed = submit( &mut exchange, &mut producer, &mut consumer, AccountId( 1 ), Side::Sell, money( "1.00" ), qty( 4 ) ).unwrap();
  assert_eq!( resumed.resting, qty( 4 ), "the identical order must now be accepted and rest" );
}
