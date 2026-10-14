//! Test Matrix T01 — ring round-trip (flush/drain/overflow) and T02 —
//! `inbound_apply`'s dispatch to `exchange_match`/`exchange_rest`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
use exchange_idem::IdemError;
use exchange_inbound::{ inbound_apply, inbound_drain, inbound_flush, inbound_overflow_reject, inbound_ring, Claims, InboundApplyError, InboundCmd, InboundOutcome };
use exchange_match::MatchError;
use exchange_order::Order;
use exchange_rest::RestReplaceError;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn resting( id : u64, side : Side, price : &str, quantity : i64, tif : Tif ) -> Resting
{
  let quantity = Quantity::from_int( quantity ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ), instrument : INSTRUMENT, account : AccountId( id ),
      side, price : Price::parse( price ).unwrap(), quantity, tif,
      client : None,
    },
    remaining : quantity,
    arrival : Sequence( id ),
  }
}

// ── T01: ring round-trip ────────────────────────────────────────────────

#[ test ]
fn a_pushed_command_drains_in_the_same_order_it_was_pushed()
{
  let mut split = inbound_ring( 8 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let a = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) };
  let b = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 2 ) };
  producer.try_push( a ).unwrap();
  producer.try_push( b ).unwrap();

  assert_eq!( inbound_drain( &mut consumer ), vec![ a, b ], "FIFO — push order must survive the ring" );
}

/// A flush takes as many commands as fit and says how many.
///
/// # Root Cause
///
/// `inbound_flush` returned `producer.try_push_batch( &mut cmds )` directly from
/// a function declared `-> usize`, but `try_push_batch` returns
/// `Result<usize, (usize, T)>` — a type mismatch (`E0308`) that failed the whole
/// workspace build, not a logic error inside a passing build.
///
/// # Why Not Caught
///
/// This test already asserted the right behavior and would have caught a wrong
/// *count*, but a compile error pre-empts every test in the crate from running
/// at all — `cargo test`/`nextest` never reached this assertion until the type
/// mismatch itself was fixed.
///
/// # Fix Applied
///
/// `inbound_flush` now matches on the `Result`, returning the accepted count `n`
/// from both `Ok( n )` and `Err( ( n, _ ) )` — the doc comment's own "partial
/// acceptance is the normal case" was already correct; only the implementation
/// disagreed with it (`module/exchange_inbound/src/lib.rs`).
///
/// # Prevention
///
/// When wrapping another crate's method as a "thin forward," confirm the real
/// signature compiles against the wrapper's own declared return type before
/// trusting the doc comment's description of its behavior.
///
/// # Pitfall
///
/// A doc comment describing intended behavior is not evidence the code compiles
/// against that behavior — a signature mismatch can sit underneath correct prose
/// and a correct test indefinitely, caught only when the crate is actually built.
#[ test ]
fn flush_accepts_as_many_as_fit_and_reports_the_count()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let cmds : Vec< _ > = ( 1..=4u64 ).map( | id | InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).collect();
  let accepted = inbound_flush( &mut producer, &cmds );

  assert_eq!( accepted, 2, "capacity 2 — only the first two of four fit" );
  assert_eq!( inbound_drain( &mut consumer ).len(), 2 );
}

/// What a flush could not publish stays with the caller.
///
/// # Root Cause
///
/// `inbound_flush` took any iterator. `try_push_batch` pulls one command past
/// what a full ring takes and hands it back in its error, and the flush
/// dropped that error's command, so a caller resuming from the same iterator
/// lost one command per partial flush — through a capacity-2 ring, 1, 2, 4
/// arrived and 3 never did.
///
/// # Why Not Caught
///
/// The partial-flush test passed its iterator by value and checked the count
/// only.
///
/// # Fix Applied
///
/// `inbound_flush` takes a slice; what did not fit is `cmds[ n.. ]`.
///
/// # Prevention
///
/// Test a partial batch by publishing the rest of it.
///
/// # Pitfall
///
/// The overflow pitfall this crate guards against at `try_push`, one call
/// over.
#[ test ]
fn a_flush_leaves_what_did_not_fit_with_the_caller()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let cmds : Vec< _ > = ( 1..=4u64 ).map( | id | InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).collect();

  let mut drained = Vec::new();
  let mut sent = 0;
  while sent < cmds.len()
  {
    sent += inbound_flush( &mut producer, &cmds[ sent.. ] );
    drained.extend( inbound_drain( &mut consumer ) );
  }

  assert_eq!( drained, cmds );
}

#[ test ]
fn a_full_ring_rejects_the_next_publish_instead_of_dropping_it()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, _consumer ) = ends.split();

  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 2 ) } ).unwrap();

  let third = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 3 ) };
  let rejected = inbound_overflow_reject( &mut producer, third );

  assert_eq!( rejected, Err( third ), "the ring must hand the rejected command back, not silently discard it" );
}

#[ test ]
fn drain_is_empty_on_a_fresh_ring()
{
  let mut split = inbound_ring( 4 ).unwrap();
  let mut ends = split.ends();
  let ( _producer, mut consumer ) = ends.split();

  assert!( inbound_drain( &mut consumer ).is_empty() );
}

// ── T02: inbound_apply dispatch ─────────────────────────────────────────

#[ test ]
fn a_place_with_nothing_to_cross_rests_on_the_book()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );

  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.trades.is_empty(), "nothing on the book to cross against" );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.id, OrderId( 1 ) );
}

#[ test ]
fn a_place_that_fully_crosses_leaves_nothing_resting()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let cmd = InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) );
  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.is_complete() );
  assert_eq!( crossing.trades.len(), 1 );
  assert!( book.is_empty(), "both sides of the full fill are gone" );
}

#[ test ]
fn an_ioc_place_never_rests_its_remainder()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Ioc ) );

  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert_eq!( crossing.trades.len(), 0 );
  assert!( !crossing.is_complete(), "nothing to cross against, so the whole quantity is unfilled" );
  assert!( book.is_empty(), "IOC must not rest the unfilled remainder" );
}

#[ test ]
fn a_post_only_place_that_would_take_is_refused_and_never_rests()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();

  let cmd = InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::PostOnly ) );
  let error = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap_err();

  assert_eq!( error, InboundApplyError::Match( MatchError::PostOnlyWouldTake ) );
  assert_eq!( book.len(), 1, "only the original ask rests" );
}

#[ test ]
fn cancel_withdraws_a_resting_order_and_reports_none_when_absent()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let found = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( found, InboundOutcome::Cancelled( Some( _ ) ) ) );

  let missing = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( missing, InboundOutcome::Cancelled( None ) ), "already withdrawn — a race result, not an error" );
}

#[ test ]
fn replace_swaps_the_resting_order_atomically()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 4, Tif::Gtc ) ), "fresh id, must succeed" );

  let new_resting = Resting { arrival : Sequence( 2 ), ..resting( 1, Side::Sell, "2.10", 6, Tif::Gtc ) };
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 1 ), new_resting };
  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Replaced( Ok( old ) ) = outcome else { panic!( "Replace must succeed here" ) };
  assert_eq!( old.order.price, Price::parse( "2.00" ).unwrap() );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.price, Price::parse( "2.10" ).unwrap() );
}

#[ test ]
fn replace_of_a_missing_order_reports_missing()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 99 ), new_resting : resting( 99, Side::Sell, "2.10", 6, Tif::Gtc ) };

  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();
  assert!( matches!( outcome, InboundOutcome::Replaced( Err( RestReplaceError::Missing ) ) ) );
}

/// `Replace` carries `instrument` and `new_resting` separately and forwards
/// both; `rest_replace` refuses them disagreeing, here as in its own suite.
#[ test ]
fn replace_refuses_a_new_resting_for_a_different_instrument()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 4, Tif::Gtc ) ) );

  let mismatched = resting( 1, Side::Sell, "2.10", 6, Tif::Gtc );
  let mismatched = Resting { order : Order { instrument : InstrumentId( 2 ), ..mismatched.order }, ..mismatched };
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 1 ), new_resting : mismatched };

  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  assert!( matches!( outcome, InboundOutcome::Replaced( Err( RestReplaceError::InstrumentMismatch ) ) ) );
  assert_eq!( book.best( InstrumentId( 2 ), Side::Sell ), None, "the mismatched replacement must not land on instrument 2" );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 1 ), "order 1 is back on its own instrument, unchanged" );
}

/// An incoming order self-match prevention cancelled does not rest.
///
/// # Root Cause
///
/// `crossing.remaining > 0` is true both for "still unfilled, rest it" and for
/// "self-match-cancelled, do not rest it" — `inbound_apply`'s `Place` arm only
/// checked `!crossing.is_complete()` and `tif_rests(..)`, never
/// `crossing.cancelled`, so it could not tell the two apart
/// (`module/exchange_inbound/src/lib.rs`, pre-fix).
///
/// # Why Not Caught
///
/// Every pre-existing test in this file used `SelfMatchPolicy::CancelResting`
/// exclusively — the one policy where the INCOMING order is never the one
/// cancelled — so no test ever exercised `CancelIncoming`/`CancelBoth` through
/// `InboundCmd::Place`, the only path where this gap is reachable.
///
/// # Fix Applied
///
/// `inbound_apply` looks for the incoming order's id in `crossing.cancelled`
/// and does not rest it when found, as `exchange_core::step_place` does.
///
/// # Prevention
///
/// `Crossing::remaining` staying non-zero means "not resolved as a fill" —
/// never assume that implies "safe to rest" without also checking
/// `Crossing::cancelled` for the incoming order's own id.
///
/// # Pitfall
///
/// A self-match-cancelled order and an ordinary unfilled remainder look
/// identical through `remaining`/`is_complete()` alone — only `cancelled`
/// distinguishes them, and skipping that check silently undoes self-match
/// prevention.
#[ test ]
fn place_does_not_rest_an_order_the_self_match_policy_cancelled()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  let shared_account = AccountId( 7 );

  let resting_sell = Resting { order : Order { account : shared_account, ..resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ).order }, ..resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) };
  assert!( book.insert( resting_sell ) );

  let incoming_buy = Resting { order : Order { account : shared_account, ..resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ).order }, ..resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) };
  let cmd = InboundCmd::Place( incoming_buy );

  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelIncoming, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert_eq!( crossing.cancelled.len(), 1, "the incoming order was self-match-cancelled, not filled" );
  assert_eq!( crossing.cancelled[ 0 ].order, OrderId( 2 ) );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ), None, "a self-match-cancelled incoming order must not end up resting on the book" );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 1 ), "CancelIncoming leaves the resting side untouched" );
}

/// A `Place` naming an id that already rests is refused, not dropped.
///
/// # Root Cause
///
/// `inbound_apply`'s `Place` arm trusted `rest_place`'s `bool` to always be `true`
/// once `!crossing.is_complete()` held, checking it only with `debug_assert!`
/// (`module/exchange_inbound/src/lib.rs`, pre-fix) — compiled to nothing outside a
/// debug build, so a duplicate id's refusal was computed and then never observed.
///
/// # Why Not Caught
///
/// Every test built a fresh `OrderId` per order; none submitted one twice.
///
/// # Fix Applied
///
/// `inbound_apply` refuses a `Place` whose id rests with
/// `InboundApplyError::Idem` — now before it crosses, asking the book.
///
/// # Prevention
///
/// A `bool`/`Result` whose failure case is a *documented, meaningful* refusal must
/// be observed through the caller's own `Result`, never only through
/// `debug_assert!`/`assert!` — reserve those for conditions already made
/// impossible by this same function's own prior logic.
///
/// # Pitfall
///
/// A `debug_assert!` guarding a public function's own precondition is a
/// release-mode no-op with a comment attached, not a check — a precondition a
/// public function depends on must hold for every caller, not just today's one.
#[ test ]
fn duplicate_place_id_is_refused_not_silently_dropped()
{
  let mut book = Book::new();
  let mut claims = Claims::new();

  let first = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, first ).unwrap();
  assert!( book.best( INSTRUMENT, Side::Buy ).is_some(), "the first order rests" );

  // Same id, different account/price/quantity — a retried or malicious
  // resubmission, not a legitimate second order.
  let repeat = InboundCmd::Place( resting( 1, Side::Buy, "0.90", 3, Tif::Gtc ) );
  let error = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, repeat ).unwrap_err();

  assert_eq!( error, InboundApplyError::Idem( IdemError::Duplicate ) );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.price, Price::parse( "1.00" ).unwrap(), "the duplicate must not have touched the book at all" );
  assert_eq!( book.len(), 1, "exactly the first order rests — the repeat neither joined nor replaced it" );
}

#[ test ]
fn cancel_then_resubmit_under_the_same_id_is_accepted()
{
  let mut book = Book::new();
  let mut claims = Claims::new();

  let place = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, place ).unwrap();

  let cancel = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) };
  let cancelled = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cancel ).unwrap();
  assert!( matches!( cancelled, InboundOutcome::Cancelled( Some( _ ) ) ) );

  // The id left the book with its order, so a resubmission under it is a
  // new order.
  let resubmit = InboundCmd::Place( resting( 1, Side::Buy, "1.05", 4, Tif::Gtc ) );
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, resubmit ).unwrap();

  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.price, Price::parse( "1.05" ).unwrap() );
}

/// A repeated id is refused before it crosses. Checked only where the
/// remainder would rest, it traded against the book first — the resting bid
/// below was consumed — and the caller got `Err` without those trades.
#[ test ]
fn a_duplicate_place_id_is_refused_before_it_trades()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 2, Side::Buy, "0.90", 5, Tif::Gtc ) ) ).unwrap();

  let repeat = InboundCmd::Place( resting( 1, Side::Sell, "0.90", 8, Tif::Gtc ) );
  let error = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, repeat ).unwrap_err();

  assert_eq!( error, InboundApplyError::Idem( IdemError::Duplicate ) );
  let bid = book.best( INSTRUMENT, Side::Buy ).expect( "the refused repeat must not have traded the bid away" );
  assert_eq!( ( bid.order.id, bid.remaining ), ( OrderId( 2 ), Quantity::from_int( 5 ).unwrap() ) );
}

/// `resting( .. )` under client id `client`.
fn tagged( id : u64, account : u64, side : Side, price : &str, quantity : i64, tif : Tif, client : u64 ) -> Resting
{
  let base = resting( id, side, price, quantity, tif );
  Resting { order : Order { account : AccountId( account ), client : Some( ClientOrderId( client ) ), ..base.order }, ..base }
}

/// A retry under the same `( AccountId, ClientOrderId )` with a fresh order
/// id is refused before it crosses, as `exchange_core` refuses one. Another
/// account may use the same value.
#[ test ]
fn a_retry_under_the_same_client_id_is_refused()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( tagged( 1, 1, Side::Sell, "1.00", 5, Tif::Gtc, 7 ) ) ).unwrap();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 2, Side::Buy, "0.90", 5, Tif::Gtc ) ) ).unwrap();

  let retry = InboundCmd::Place( tagged( 3, 1, Side::Sell, "0.90", 5, Tif::Gtc, 7 ) );
  let error = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, retry ).unwrap_err();
  assert_eq!( error, InboundApplyError::DuplicateClientId );
  assert_eq!( book.len(), 2, "the retry neither rested nor traded" );

  let other_account = InboundCmd::Place( tagged( 4, 4, Side::Sell, "1.10", 5, Tif::Gtc, 7 ) );
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, other_account ).unwrap();
  assert_eq!( book.len(), 3 );
}

/// A client id stays claimed after its order is cancelled — a retry of a
/// cancelled order is still a retry.
#[ test ]
fn a_client_id_stays_claimed_after_cancel()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( tagged( 1, 1, Side::Buy, "1.00", 5, Tif::Gtc, 7 ) ) ).unwrap();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();

  let retry = InboundCmd::Place( tagged( 2, 1, Side::Buy, "1.00", 5, Tif::Gtc, 7 ) );
  assert_eq!( inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, retry ), Err( InboundApplyError::DuplicateClientId ) );
}

/// A refused place claims nothing: a post-only that would take may be
/// resubmitted under the same client id.
#[ test ]
fn a_refused_place_does_not_claim_its_client_id()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();

  let taking = InboundCmd::Place( tagged( 2, 2, Side::Buy, "1.00", 5, Tif::PostOnly, 7 ) );
  assert_eq!
  (
    inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, taking ),
    Err( InboundApplyError::Match( MatchError::PostOnlyWouldTake ) ),
  );

  let passive = InboundCmd::Place( tagged( 3, 2, Side::Buy, "0.95", 5, Tif::PostOnly, 7 ) );
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, passive ).unwrap();
  assert_eq!( book.len(), 2 );
}

/// A place for a quantity of zero is refused, and claims nothing.
///
/// # Root Cause
///
/// `inbound_apply` never read the quantity. `cross` had nothing to trade,
/// the call returned `Ok` with a complete crossing, and the client id was
/// claimed for good, so the account's real order under it was refused.
///
/// # Why Not Caught
///
/// Every place in the suite had a positive quantity.
///
/// # Fix Applied
///
/// A zero quantity is refused first, with `InboundApplyError::ZeroQuantity`,
/// as `exchange_core` refuses it.
///
/// # Prevention
///
/// A second way onto the book gets the facade's checks that do not need an
/// instrument's spec.
///
/// # Pitfall
///
/// A claim taken for good makes a harmless-looking `Ok` permanent.
#[ test ]
fn a_zero_quantity_place_is_refused_and_claims_nothing()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  let empty = tagged( 1, 1, Side::Buy, "1.00", 0, Tif::Gtc, 7 );

  assert_eq!( inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( empty ) ), Err( InboundApplyError::ZeroQuantity ) );

  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( tagged( 2, 1, Side::Buy, "1.00", 5, Tif::Gtc, 7 ) ) ).unwrap();
  assert_eq!( book.len(), 1, "the client id was still free" );
}

/// A place whose `remaining` is not its quantity is refused before it
/// crosses.
///
/// # Root Cause
///
/// `inbound_apply` crossed `order.quantity` and rested what `cross` left of
/// it, so the `remaining` a place carried was never read: a buy for 10 with
/// 3 remaining took all 10 from the ask, and one with nothing remaining
/// rested whole.
///
/// # Why Not Caught
///
/// The test helper sets `remaining` to the quantity.
///
/// # Fix Applied
///
/// A `Place` is a new order, so anything but its whole quantity remaining is
/// refused with `InboundApplyError::NotNew`.
///
/// # Prevention
///
/// Test a command with each field it carries set apart from the others.
///
/// # Pitfall
///
/// `Resting` is the book's node, reused as a command; its `remaining` means
/// something there and nothing in a new order.
#[ test ]
fn a_place_whose_remaining_is_not_its_quantity_is_refused()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Sell, "1.00", 10, Tif::Gtc ) ) );

  for remaining in [ 3, 0, 12 ]
  {
    let partial = Resting { remaining : Quantity::from_int( remaining ).unwrap(), ..resting( 2, Side::Buy, "1.00", 10, Tif::Gtc ) };
    assert_eq!( inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( partial ) ), Err( InboundApplyError::NotNew ), "{remaining} remaining of 10" );
    assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().remaining, Quantity::from_int( 10 ).unwrap(), "the ask was not touched" );
    assert_eq!( book.len(), 1 );
  }
}

/// A replace may take the id of an order that has filled.
///
/// # Root Cause
///
/// Order ids were claimed in a set of their own, released only by a cancel
/// or a replace. Order 1 rested, claimed its id, and filled; the claim
/// stayed. A replace onto id 1 passed `rest_replace`, which asks the book,
/// changed the book, and then panicked claiming the id — in release too.
///
/// # Why Not Caught
///
/// No test replaced onto an id that had been used before.
///
/// # Fix Applied
///
/// The set is gone: `inbound_apply` asks the book whether an id rests, and
/// a replace claims nothing.
///
/// # Prevention
///
/// Keep one record of what the book holds — the book.
///
/// # Pitfall
///
/// Every way an order leaves the book has to release a mirror of it: a
/// fill, a self-match withdrawal, a cancel, a replace.
#[ test ]
fn a_replace_may_take_the_id_of_an_order_that_filled()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  for place in [ resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ), resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ), resting( 3, Side::Sell, "2.00", 5, Tif::Gtc ) ]
  {
    inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Place( place ) ).unwrap();
  }

  let onto_filled = Resting { order : Order { account : AccountId( 3 ), ..resting( 1, Side::Sell, "2.10", 5, Tif::Gtc ).order }, arrival : Sequence( 4 ), ..resting( 1, Side::Sell, "2.10", 5, Tif::Gtc ) };
  let outcome = inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 3 ), new_resting : onto_filled } );

  assert!( matches!( outcome, Ok( InboundOutcome::Replaced( Ok( _ ) ) ) ) );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 1 ) );
}

/// An order id is free again once its order has left the book, whether it
/// filled or self-match prevention withdrew it — as it is after a cancel,
/// and as it always was for an order that filled on arrival.
#[ test ]
fn an_order_id_is_free_again_once_its_order_left_the_book()
{
  for policy in [ SelfMatchPolicy::CancelResting, SelfMatchPolicy::CancelBoth ]
  {
    let mut book = Book::new();
    let mut claims = Claims::new();
    let shared = | resting : Resting | Resting { order : Order { account : AccountId( 7 ), ..resting.order }, ..resting };
    inbound_apply( &mut book, &mut claims, policy, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();
    inbound_apply( &mut book, &mut claims, policy, InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) ) ).unwrap();
    inbound_apply( &mut book, &mut claims, policy, InboundCmd::Place( shared( resting( 3, Side::Sell, "1.00", 5, Tif::Gtc ) ) ) ).unwrap();
    inbound_apply( &mut book, &mut claims, policy, InboundCmd::Place( shared( resting( 4, Side::Buy, "1.00", 5, Tif::Ioc ) ) ) ).unwrap();
    assert!( book.is_empty(), "{policy:?}: order 1 filled, order 3 was withdrawn" );

    for id in [ 1, 3 ]
    {
      inbound_apply( &mut book, &mut claims, policy, InboundCmd::Place( resting( id, Side::Buy, "0.90", 5, Tif::Gtc ) ) ).unwrap();
    }
    assert_eq!( book.len(), 2, "{policy:?}" );
  }
}

/// A place naming an id that rests is refused before it trades, even when
/// that order reached the book some other way than `inbound_apply`.
///
/// # Root Cause
///
/// The check before crossing read the claim set, which only knew what
/// `inbound_apply` had rested itself. Order 1 inserted directly was not in
/// it, so a place under id 1 traded against order 5, then failed to rest:
/// a `debug_assert!` fired in debug, and in release the remainder vanished
/// behind an `Ok`.
///
/// # Why Not Caught
///
/// The duplicate tests rested the first order through `inbound_apply`.
///
/// # Fix Applied
///
/// The check before crossing asks the book.
///
/// # Prevention
///
/// Check a duplicate against the thing that refuses it.
///
/// # Pitfall
///
/// The tests seed books with `Book::insert` and a fresh `Claims` all the
/// time; a caller restoring a book does the same.
#[ test ]
fn a_place_whose_id_rests_is_refused_before_it_trades()
{
  let mut book = Book::new();
  let mut claims = Claims::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 5, Tif::Gtc ) ) );
  assert!( book.insert( resting( 5, Side::Sell, "1.00", 2, Tif::Gtc ) ) );

  let repeat = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );
  assert_eq!( inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, repeat ), Err( InboundApplyError::Idem( IdemError::Duplicate ) ) );

  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 5 ), "order 5 was not traded" );
  assert_eq!( book.len(), 2 );
}
