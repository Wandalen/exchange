//! Smoke lane `smoke_exchange_core` — one order crossing the whole
//! family, and one that must not cross at all.
//!
//! # Why there are two arms
//!
//! An engine that fills everything passes every test about filling. The
//! interesting half of a matching engine is when it **declines**: an order
//! priced through no liquidity must produce zero trades and rest. So this lane
//! runs both, and the control arm's zero is asserted rather than printed —
//! a printed zero nobody compares is indistinguishable from a broken assertion.
//!
//! # Why it depends on `exchange_core` alone
//!
//! One dependency, not five. That makes an incomplete facade a build failure
//! here rather than something a reader has to notice: if `exchange_core` stops
//! re-exporting a type this lane needs, this lane does not compile.
//!
//! # The conservation check is borrowed, not written here
//!
//! The final arm audits the exchange's own postings with the `verify`
//! re-exported above — machinery that knows nothing about order books. A
//! conservation check written inside this lane would agree with the exchange
//! by construction; one written for a different purpose entirely has no such
//! sympathy.
//!
//! # Why the lane is a library and not `src/main.rs`
//!
//! No test suite can execute a bare `src/main.rs` to raise its coverage, and
//! bounding what may live in that file keeps a coverage exclusion from quietly
//! becoming somewhere to keep logic. The lane was well past that bound, which
//! left every assertion about the exchange in the one file nothing measured.
//! `src/main.rs` is now the process entry point and nothing else, and
//! `tests/lane_test.rs` drives what moved.

use exchange_core::
{
  AccountId, Consumer, Exchange, ExchangeError, InboundCmd, InstrumentId, Money, Order, OrderId, Producer, Quantity,
  Receipt, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, inbound_flush, inbound_ring, verify,
};

/// Both traders start with this much of everything, so the arithmetic in the
/// printed output is easy to follow by hand.
pub const OPENING_CASH : &str = "1000";
/// And this much of the asset.
pub const OPENING_ASSET : i64 = 100;
/// The one instrument every arm trades — this lane never needed a second one
/// to make its point, same as `exchange_core`'s own `submission_test.rs`.
const INSTRUMENT : InstrumentId = InstrumentId( 1 );

/// The ring-fed equivalent of the deleted `Exchange::submit( account, side,
/// price, quantity )` — see `exchange_core`'s own module doc, "`exchange_step`
/// replaces `submit`". `remaining`/`arrival` on the pushed [`Resting`] are
/// never read by [`Exchange::exchange_step`] (it only extracts `.order` from
/// a drained [`InboundCmd::Place`]), so both are placeholders; [`Tif::Gtc`]
/// and [`SelfMatchPolicy::CancelIncoming`] match the old hardcoded
/// submission semantics exactly, so every assertion below keeps its original
/// meaning.
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

/// A [`Money`] from one of this lane's own literals.
///
/// # Panics
///
/// Panics if the literal does not parse, which would be this lane's bug rather
/// than the exchange's.
#[ must_use ]
pub fn money( text : &str ) -> Money
{
  Money::parse( text ).expect( "the lane's own literals are well-formed" )
}

/// A [`Quantity`] of whole units.
///
/// # Panics
///
/// Panics if the count is out of range, which would be this lane's bug rather
/// than the exchange's.
#[ must_use ]
pub fn units( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).expect( "the lane's own quantities are in range" )
}

/// The crossing arm: a resting ask, then a bid that takes it.
///
/// Returns the trade count and both traders' available cash afterwards.
///
/// # Panics
///
/// Panics if the ask does not rest, the bid does not take it in full, the cash
/// did not move by exactly the traded value, or the posting log does not
/// balance.
#[ must_use ]
pub fn crossing_arm() -> ( usize, Money, Money )
{
  let mut exchange = Exchange::new();
  let seller = AccountId( 1 );
  let buyer = AccountId( 2 );
  exchange.open_account( seller, money( OPENING_CASH ), units( OPENING_ASSET ) ).expect( "fresh account, first deposit cannot overflow" );
  exchange.open_account( buyer, money( OPENING_CASH ), units( OPENING_ASSET ) ).expect( "fresh account, first deposit cannot overflow" );
  let mut ring = inbound_ring( 8 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, seller, Side::Sell, money( "2.50" ), units( 10 ) )
  .expect( "a funded sell rests" );
  assert!( resting.trades.is_empty(), "nothing was on the other side yet" );

  let taking = submit( &mut exchange, &mut producer, &mut consumer, buyer, Side::Buy, money( "2.50" ), units( 4 ) )
  .expect( "a funded buy at the ask crosses" );

  assert_eq!( taking.trades.len(), 1, "the bid should have taken the one resting ask" );
  assert_eq!( taking.trades[ 0 ].quantity, units( 4 ), "it should have taken exactly what it asked for" );
  assert!( taking.is_complete(), "and nothing of it should be left to rest" );

  let seller_cash = exchange.escrow().account( seller ).expect( "the seller is open" ).cash;
  let buyer_cash = exchange.escrow().account( buyer ).expect( "the buyer is open" ).cash;

  // 4 units at 2.50 is exactly 10 currency units, moved rather than created.
  assert_eq!( seller_cash.available(), money( "1010" ), "the seller was paid" );
  assert_eq!( buyer_cash.available(), money( "990" ), "the buyer paid" );

  let report = verify( &exchange.postings().expect( "trades are expressible" ) )
  .expect( "the posting log is auditable" );
  assert!( report.is_balanced(), "value moved between accounts, it did not appear or vanish" );

  ( taking.trades.len(), seller_cash.available(), buyer_cash.available() )
}

/// The control arm: a bid priced under the only ask on the book.
///
/// This arm must report **wrong** — zero trades — and its zero is what proves
/// the engine can decline. If this arm ever produces a trade, the crossing arm
/// above proves nothing, because an engine that fills unconditionally would
/// pass it too.
///
/// # Panics
///
/// Panics if the bid crosses, if the whole quantity does not rest, or if both
/// orders are not left on the book.
#[ must_use ]
pub fn control_arm() -> usize
{
  let mut exchange = Exchange::new();
  let seller = AccountId( 1 );
  let buyer = AccountId( 2 );
  exchange.open_account( seller, money( OPENING_CASH ), units( OPENING_ASSET ) ).expect( "fresh account, first deposit cannot overflow" );
  exchange.open_account( buyer, money( OPENING_CASH ), units( OPENING_ASSET ) ).expect( "fresh account, first deposit cannot overflow" );
  let mut ring = inbound_ring( 8 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  submit( &mut exchange, &mut producer, &mut consumer, seller, Side::Sell, money( "2.50" ), units( 10 ) )
  .expect( "a funded sell rests" );

  let through = submit( &mut exchange, &mut producer, &mut consumer, buyer, Side::Buy, money( "2.49" ), units( 4 ) )
  .expect( "a bid below the ask is accepted, it simply does not cross" );

  assert_eq!( through.trades.len(), 0, "a bid one minor unit below the ask must not cross it" );
  assert_eq!( through.resting, units( 4 ), "the whole quantity rests instead" );
  assert_eq!( exchange.book().len(), 2, "both orders are on the book, on opposite sides" );

  through.trades.len()
}

/// The cancel arm: a reservation returned rather than leaked.
///
/// Tested separately from the fill path because a suite that only cancels
/// through fills reports a green exchange with a live reservation leak —
/// currency neither usable by its owner nor settled to a counterparty, which
/// no fill-path assertion can observe.
///
/// # Panics
///
/// Panics if the reservation is not held while the order rests, if the cancel
/// is refused, or if anything is left reserved afterwards.
#[ must_use ]
pub fn cancel_arm() -> Money
{
  let mut exchange = Exchange::new();
  let buyer = AccountId( 1 );
  exchange.open_account( buyer, money( OPENING_CASH ), units( OPENING_ASSET ) ).expect( "fresh account, first deposit cannot overflow" );
  let mut ring = inbound_ring( 8 ).expect( "a small power-of-two capacity is always valid" );
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let resting = submit( &mut exchange, &mut producer, &mut consumer, buyer, Side::Buy, money( "2.50" ), units( 10 ) )
  .expect( "a funded buy rests" );

  let held = exchange.escrow().account( buyer ).expect( "the buyer is open" ).cash;
  assert_eq!( held.available(), money( "975" ), "25 currency units are committed while it rests" );
  assert_eq!( held.reserved(), money( "25" ), "and they are held, not gone" );

  exchange.cancel( resting.order ).expect( "a resting order can be cancelled" );

  let returned = exchange.escrow().account( buyer ).expect( "the buyer is open" ).cash;
  assert_eq!( returned.available(), money( OPENING_CASH ), "the cancel returned every committed unit" );
  assert_eq!( returned.reserved(), Money::ZERO, "and left nothing held" );

  returned.available()
}

/// Run all three arms, print what each found, and assert they disagree.
///
/// # Panics
///
/// Panics on anything any arm asserts, and on the two arms agreeing — which
/// would mean neither has shown anything.
pub fn run()
{
  let ( filled, seller_cash, buyer_cash ) = crossing_arm();
  println!( "crossing arm : {filled} trade(s); seller {seller_cash}, buyer {buyer_cash}" );

  let declined = control_arm();
  println!( "control  arm : {declined} trade(s) — the bid was priced through no liquidity" );

  let returned = cancel_arm();
  println!( "cancel   arm : {returned} available again after cancel — nothing left reserved" );

  assert_ne!( filled, declined, "the two arms must disagree, or neither has shown anything" );

  println!( "smoke_exchange_core: ok" );
}
