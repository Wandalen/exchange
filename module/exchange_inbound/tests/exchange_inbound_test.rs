//! Test Matrix T01 — ring round-trip (flush/drain/overflow) and T02 —
//! `inbound_apply`'s dispatch to `exchange_match`/`exchange_rest`.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_idem::{ IdemError, IdSet };
use exchange_inbound::{ inbound_apply, inbound_drain, inbound_flush, inbound_overflow_reject, inbound_ring, InboundApplyError, InboundCmd, InboundOutcome };
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
      side, price : Money::parse( price ).unwrap(), quantity, tif,
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

// exchange_inbound/BUG-001 substrate/task/exchange_inbound/bug/completed/001_inbound_flush_wrong_return_type.md
// — bug_reproducer: this test already specified the correct partial-acceptance
// behavior (accepted == 2, not a panic) before the fix; it simply could not run
// because inbound_flush did not compile against try_push_batch's real signature.
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

  let cmds = ( 1..=4u64 ).map( | id | InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } );
  let accepted = inbound_flush( &mut producer, cmds );

  assert_eq!( accepted, 2, "capacity 2 — only the first two of four fit" );
  assert_eq!( inbound_drain( &mut consumer ).len(), 2 );
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
  let mut seen = IdSet::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );

  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.trades.is_empty(), "nothing on the book to cross against" );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.id, OrderId( 1 ) );
}

#[ test ]
fn a_place_that_fully_crosses_leaves_nothing_resting()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  assert!( book.insert( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let cmd = InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) );
  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.is_complete() );
  assert_eq!( crossing.trades.len(), 1 );
  assert!( book.is_empty(), "both sides of the full fill are gone" );
}

#[ test ]
fn an_ioc_place_never_rests_its_remainder()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Ioc ) );

  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert_eq!( crossing.trades.len(), 0 );
  assert!( !crossing.is_complete(), "nothing to cross against, so the whole quantity is unfilled" );
  assert!( book.is_empty(), "IOC must not rest the unfilled remainder" );
}

#[ test ]
fn a_post_only_place_that_would_take_is_refused_and_never_rests()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();

  let cmd = InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::PostOnly ) );
  let error = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap_err();

  assert_eq!( error, InboundApplyError::Match( MatchError::PostOnlyWouldTake ) );
  assert_eq!( book.len(), 1, "only the original ask rests" );
}

#[ test ]
fn cancel_withdraws_a_resting_order_and_reports_none_when_absent()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  assert!( book.insert( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let found = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( found, InboundOutcome::Cancelled( Some( _ ) ) ) );

  let missing = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( missing, InboundOutcome::Cancelled( None ) ), "already withdrawn — a race result, not an error" );
}

#[ test ]
fn replace_swaps_the_resting_order_atomically()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 4, Tif::Gtc ) ), "fresh id, must succeed" );

  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 1 ), new_resting : resting( 1, Side::Sell, "2.10", 6, Tif::Gtc ) };
  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Replaced( Ok( old ) ) = outcome else { panic!( "Replace must succeed here" ) };
  assert_eq!( old.order.price, Money::parse( "2.00" ).unwrap() );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.price, Money::parse( "2.10" ).unwrap() );
}

#[ test ]
fn replace_of_a_missing_order_reports_missing()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 99 ), new_resting : resting( 99, Side::Sell, "2.10", 6, Tif::Gtc ) };

  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();
  assert!( matches!( outcome, InboundOutcome::Replaced( Err( RestReplaceError::Missing ) ) ) );
}

// BUG-001 substrate/task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md — bug_reproducer: this crate's own
// `InboundCmd::Replace` carries `instrument` and `new_resting` as two independently-settable
// fields and forwards both straight to `exchange_rest::rest_replace` with no check of its own
// (see that function's own bug_reproducer in `exchange_rest_test.rs`) — a second, real, tested
// call site sharing the exact same gap, found while filing BUG-001's Search More Instances step.
#[ test ]
fn replace_refuses_a_new_resting_for_a_different_instrument()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 4, Tif::Gtc ) ) );

  let mismatched = resting( 1, Side::Sell, "2.10", 6, Tif::Gtc );
  let mismatched = Resting { order : Order { instrument : InstrumentId( 2 ), ..mismatched.order }, ..mismatched };
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 1 ), new_resting : mismatched };

  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  assert!( matches!( outcome, InboundOutcome::Replaced( Err( RestReplaceError::InstrumentMismatch ) ) ) );
  assert_eq!( book.best( InstrumentId( 2 ), Side::Sell ), None, "the mismatched replacement must not land on instrument 2" );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 1 ), "order 1 is back on its own instrument, unchanged" );
}

// exchange_inbound/BUG-002 substrate/task/exchange_inbound/bug/verified/002_place_rests_self_match_cancelled_order.md
// — bug_reproducer: an incoming Place cancelled by self-match prevention
// (CancelIncoming/CancelBoth) was rested on the book anyway, because
// `inbound_apply` never checked `crossing.cancelled` before resting
// `crossing.remaining` — the same class of bug `exchange_core::step_place`
// already found and fixed under its own `incoming_cancelled` check.
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
/// `inbound_apply` now scans `crossing.cancelled` for an entry matching the
/// incoming order's own id (`incoming_cancelled`) and skips resting when true
/// — the same pattern already used by `exchange_core::step_place`'s own
/// `incoming_cancelled` check (`module/exchange_core/src/lib.rs:611-621`).
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
  let mut seen = IdSet::new();
  let shared_account = AccountId( 7 );

  let resting_sell = Resting { order : Order { account : shared_account, ..resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ).order }, ..resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) };
  assert!( book.insert( resting_sell ) );

  let incoming_buy = Resting { order : Order { account : shared_account, ..resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ).order }, ..resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) };
  let cmd = InboundCmd::Place( incoming_buy );

  let outcome = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelIncoming, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert_eq!( crossing.cancelled.len(), 1, "the incoming order was self-match-cancelled, not filled" );
  assert_eq!( crossing.cancelled[ 0 ].order, OrderId( 2 ) );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ), None, "a self-match-cancelled incoming order must not end up resting on the book" );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.id, OrderId( 1 ), "CancelIncoming leaves the resting side untouched" );
}

// exchange_inbound/BUG-003 substrate/task/exchange_inbound/bug/verified/003_duplicate_place_silently_dropped_in_release.md
// — bug_reproducer: a `Place` whose remainder would rest, naming an id already
// claimed by an earlier, still-resting `Place`, was silently dropped — `rest_place`'s
// own `false` was only ever checked by a `debug_assert!`, compiled out in release.
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
/// Every pre-existing test built a fresh, non-colliding `OrderId` per order (see
/// `resting()`'s own per-call `id` argument) — nothing ever submitted the same id
/// twice through a live `Place`.
///
/// # Fix Applied
///
/// `inbound_apply` now takes `seen : &mut IdSet` and calls
/// `exchange_idem::idem_insert` before ever resting a `Place`'s remainder,
/// returning `InboundApplyError::Idem` on a repeat instead of silently dropping it.
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
  let mut seen = IdSet::new();

  let first = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, first ).unwrap();
  assert!( book.best( INSTRUMENT, Side::Buy ).is_some(), "the first order rests" );

  // Same id, different account/price/quantity — a retried or malicious
  // resubmission, not a legitimate second order.
  let repeat = InboundCmd::Place( resting( 1, Side::Buy, "0.90", 3, Tif::Gtc ) );
  let error = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, repeat ).unwrap_err();

  assert_eq!( error, InboundApplyError::Idem( IdemError::Duplicate ) );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.price, Money::parse( "1.00" ).unwrap(), "the duplicate must not have touched the book at all" );
  assert_eq!( book.len(), 1, "exactly the first order rests — the repeat neither joined nor replaced it" );
}

#[ test ]
fn cancel_then_resubmit_under_the_same_id_is_accepted()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();

  let place = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, place ).unwrap();

  let cancel = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) };
  let cancelled = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, cancel ).unwrap();
  assert!( matches!( cancelled, InboundOutcome::Cancelled( Some( _ ) ) ) );

  // A legitimate resubmission under the same, now-cancelled id must be
  // accepted again — the whole reason `idem_remove` exists.
  let resubmit = InboundCmd::Place( resting( 1, Side::Buy, "1.05", 4, Tif::Gtc ) );
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, resubmit ).unwrap();

  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.price, Money::parse( "1.05" ).unwrap() );
}

/// A repeated id is refused before it crosses. Checked only where the
/// remainder would rest, it traded against the book first — the resting bid
/// below was consumed — and the caller got `Err` without those trades.
#[ test ]
fn a_duplicate_place_id_is_refused_before_it_trades()
{
  let mut book = Book::new();
  let mut seen = IdSet::new();
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ) ).unwrap();
  inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, InboundCmd::Place( resting( 2, Side::Buy, "0.90", 5, Tif::Gtc ) ) ).unwrap();

  let repeat = InboundCmd::Place( resting( 1, Side::Sell, "0.90", 8, Tif::Gtc ) );
  let error = inbound_apply( &mut book, &mut seen, SelfMatchPolicy::CancelResting, repeat ).unwrap_err();

  assert_eq!( error, InboundApplyError::Idem( IdemError::Duplicate ) );
  let bid = book.best( INSTRUMENT, Side::Buy ).expect( "the refused repeat must not have traded the bid away" );
  assert_eq!( ( bid.order.id, bid.remaining ), ( OrderId( 2 ), Quantity::from_int( 5 ).unwrap() ) );
}
