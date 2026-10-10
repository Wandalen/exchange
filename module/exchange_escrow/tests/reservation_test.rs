//! Test Matrix T08–T10 — the partition, and the two ways units leave it.
//!
//! T10 is the row a fill-only suite cannot supply. Release-on-cancel is
//! invisible from every assertion about trades: the books balance, the trades
//! are right, and a customer's currency is locked away forever. Testing the
//! cancel path is the only thing that sees it.

use exact_arith::{ CEILING_WHOLE_UNITS, Money, Price, Quantity };
use exchange_escrow::{ Escrow, EscrowError };
use exchange_fill::Trade;
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;
use exchange_tif::Tif;

fn money( text : &str ) -> Money
{
  Money::parse( text ).unwrap()
}

fn price( text : &str ) -> Price
{
  Price::parse( text ).unwrap()
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

fn order( id : u64, account : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( account ),
    side,
    price : Price::parse( price ).unwrap(),
    quantity : qty( quantity ),
    tif : Tif::Gtc,
    client : None,
  }
}

/// An escrow with one funded account holding 1000 currency and 100 asset.
fn funded() -> Escrow
{
  let mut escrow = Escrow::new();
  escrow.open( AccountId( 1 ), money( "1000" ), qty( 100 ) ).unwrap();
  escrow
}

/// T08 — an order committing more than its owner holds is refused whole.
///
/// Both halves are asserted: the refusal, and that nothing moved. A partial
/// reservation would leave the account short and the order un-backed, which is
/// worse than either failure alone.
#[ test ]
fn t08_an_unfunded_order_is_rejected_and_nothing_moves()
{
  let mut escrow = funded();

  let too_big = order( 1, 1, Side::Buy, "2.00", 1000 );
  assert_eq!( escrow.reserve( &too_big ), Err( EscrowError::Insufficient ) );

  let account = escrow.account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "1000" ), "the balance is untouched" );
  assert_eq!( account.cash.reserved(), Money::ZERO, "and nothing was partially held" );
  assert_eq!( escrow.reservation_count(), 0, "no reservation was recorded" );
}

/// T08 — a sell short of the asset is refused the same way.
#[ test ]
fn t08_a_sell_without_the_asset_is_rejected()
{
  let mut escrow = funded();

  let short = order( 1, 1, Side::Sell, "2.00", 101 );
  assert_eq!( escrow.reserve( &short ), Err( EscrowError::Insufficient ) );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().asset.available(), qty( 100 ) );
}

/// T08 — an unknown account is refused before any arithmetic happens.
#[ test ]
fn t08_an_unknown_account_cannot_reserve()
{
  let mut escrow = funded();
  let stranger = order( 1, 99, Side::Buy, "2.00", 1 );

  assert_eq!( escrow.reserve( &stranger ), Err( EscrowError::UnknownAccount( AccountId( 99 ) ) ) );
}

/// Reserving moves units across the partition rather than merely noting them.
///
/// The distinction is the whole design: after reserving, `available` is
/// *smaller*. A design that only recorded the reservation would leave
/// `available` unchanged, and a second order would find the same units
/// spendable again.
#[ test ]
fn reserving_moves_units_out_of_available()
{
  let mut escrow = funded();

  let obligation = escrow.reserve( &order( 1, 1, Side::Buy, "2.50", 10 ) ).unwrap();
  assert_eq!( obligation, Obligation::Cash( money( "25" ) ) );

  let account = escrow.account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "975" ) );
  assert_eq!( account.cash.reserved(), money( "25" ) );
  assert_eq!( account.cash.total().unwrap(), money( "1000" ), "the sum did not change" );
}

/// A second order cannot spend what the first already promised.
///
/// The time-of-check-to-time-of-use case, and the reason reservation is a
/// mechanism rather than a check. Both orders individually fit the opening
/// balance; together they do not, and the second one finds out because the
/// first one already took the units.
#[ test ]
fn a_second_order_cannot_spend_the_first_orders_reservation()
{
  let mut escrow = funded();

  escrow.reserve( &order( 1, 1, Side::Buy, "10.00", 60 ) ).unwrap();
  let second = escrow.reserve( &order( 2, 1, Side::Buy, "10.00", 60 ) );

  assert_eq!( second, Err( EscrowError::Insufficient ), "600 + 600 does not fit in 1000" );
  assert_eq!( escrow.reservation_count(), 1 );
}

/// One order cannot reserve twice.
///
/// A second reservation would make the first unreachable, and unreachable
/// reserved units are locked away with no event that could ever return them.
#[ test ]
fn one_order_cannot_hold_two_reservations()
{
  let mut escrow = funded();
  let repeat = order( 1, 1, Side::Buy, "2.50", 10 );

  escrow.reserve( &repeat ).unwrap();
  assert_eq!( escrow.reserve( &repeat ), Err( EscrowError::AlreadyReserved( OrderId( 1 ) ) ) );
}

/// T10 — a cancel returns every reserved unit.
#[ test ]
fn t10_cancel_releases_the_whole_reservation()
{
  let mut escrow = funded();
  escrow.reserve( &order( 1, 1, Side::Buy, "2.50", 10 ) ).unwrap();

  let released = escrow.release( AccountId( 1 ), OrderId( 1 ) ).unwrap();

  assert_eq!( released, Obligation::Cash( money( "25" ) ) );
  let account = escrow.account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "1000" ), "back to where it started" );
  assert_eq!( account.cash.reserved(), Money::ZERO, "with nothing left held" );
  assert_eq!( escrow.reservation_count(), 0, "and the record is gone, not merely zeroed" );
}

/// T10 — a sell's cancel returns the asset, not currency.
#[ test ]
fn t10_cancel_returns_the_asset_a_sell_reserved()
{
  let mut escrow = funded();
  escrow.reserve( &order( 1, 1, Side::Sell, "2.50", 10 ) ).unwrap();

  assert_eq!( escrow.release( AccountId( 1 ), OrderId( 1 ) ).unwrap(), Obligation::Asset( qty( 10 ) ) );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().asset.available(), qty( 100 ) );
}

/// Releasing what was never reserved is refused, not silently credited.
///
/// A release with no matching reservation credits units nobody debited — which
/// is minting, and it balances perfectly in every per-account view.
#[ test ]
fn releasing_without_a_reservation_is_refused()
{
  let mut escrow = funded();

  assert_eq!
  (
    escrow.release( AccountId( 1 ), OrderId( 7 ) ),
    Err( EscrowError::NoReservation( OrderId( 7 ) ) ),
  );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().cash.available(), money( "1000" ) );
}

/// Two accounts, one funded sell and one funded buy, ready to settle.
fn two_sided() -> Escrow
{
  let mut escrow = Escrow::new();
  escrow.open( AccountId( 1 ), money( "1000" ), qty( 100 ) ).unwrap();
  escrow.open( AccountId( 2 ), money( "1000" ), qty( 100 ) ).unwrap();
  escrow
}

/// T09 — a fill releases the matched portion and moves the units.
#[ test ]
fn t09_a_fill_settles_and_leaves_nothing_reserved()
{
  let mut escrow = two_sided();
  let seller = order( 1, 1, Side::Sell, "2.50", 4 );
  let buyer = order( 2, 2, Side::Buy, "2.50", 4 );
  escrow.reserve( &seller ).unwrap();
  escrow.reserve( &buyer ).unwrap();

  let trade = Trade
  {
    taker : buyer.id,
    taker_account : buyer.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "2.50" ),
    quantity : qty( 4 ),
  };
  escrow.settle( &trade, Side::Buy, buyer.price ).unwrap();

  let seller_account = escrow.account( AccountId( 1 ) ).unwrap();
  let buyer_account = escrow.account( AccountId( 2 ) ).unwrap();

  assert_eq!( seller_account.cash.available(), money( "1010" ), "the seller was paid 10" );
  assert_eq!( seller_account.asset.available(), qty( 96 ), "and delivered 4" );
  assert_eq!( seller_account.asset.reserved(), Quantity::ZERO, "with nothing left held" );
  assert_eq!( buyer_account.cash.available(), money( "990" ), "the buyer paid 10" );
  assert_eq!( buyer_account.asset.available(), qty( 104 ), "and received 4" );
  assert_eq!( buyer_account.cash.reserved(), Money::ZERO );
  assert_eq!( escrow.reservation_count(), 0, "both reservations are discharged" );
}

/// A buyer who executes better than its limit gets the difference back.
///
/// The buyer reserved at 3.00 and executed at 2.50, so 4 units cost 10 rather
/// than the 12 committed. Without this, the exchange would quietly keep 2 —
/// a fee nobody agreed to, which every conservation sum over the *accounts*
/// would still balance because the units would simply sit in `reserved`
/// forever.
#[ test ]
fn a_price_improvement_returns_the_unused_reservation()
{
  let mut escrow = two_sided();
  let seller = order( 1, 1, Side::Sell, "2.50", 4 );
  let buyer = order( 2, 2, Side::Buy, "3.00", 4 );
  escrow.reserve( &seller ).unwrap();
  escrow.reserve( &buyer ).unwrap();
  assert_eq!( escrow.account( AccountId( 2 ) ).unwrap().cash.reserved(), money( "12" ) );

  let trade = Trade
  {
    taker : buyer.id,
    taker_account : buyer.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "2.50" ),
    quantity : qty( 4 ),
  };
  escrow.settle( &trade, Side::Buy, buyer.price ).unwrap();

  let buyer_account = escrow.account( AccountId( 2 ) ).unwrap();
  assert_eq!( buyer_account.cash.available(), money( "990" ), "it paid 10, not the 12 it committed" );
  assert_eq!( buyer_account.cash.reserved(), Money::ZERO, "and the other 2 came back rather than sticking" );
}

/// A partial fill leaves the rest of the reservation in place.
///
/// Releasing the whole reservation on the first fill would leave the remainder
/// resting on the book with nothing behind it — an order the book displays and
/// cannot settle.
#[ test ]
fn a_partial_fill_leaves_the_remainder_covered()
{
  let mut escrow = two_sided();
  let seller = order( 1, 1, Side::Sell, "2.50", 10 );
  let buyer = order( 2, 2, Side::Buy, "2.50", 4 );
  escrow.reserve( &seller ).unwrap();
  escrow.reserve( &buyer ).unwrap();

  let trade = Trade
  {
    taker : buyer.id,
    taker_account : buyer.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "2.50" ),
    quantity : qty( 4 ),
  };
  escrow.settle( &trade, Side::Buy, buyer.price ).unwrap();

  assert_eq!
  (
    escrow.reserved_for( seller.id ),
    Some( Obligation::Asset( qty( 6 ) ) ),
    "the unfilled 6 are still committed",
  );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().asset.reserved(), qty( 6 ) );
  assert_eq!( escrow.reserved_for( buyer.id ), None, "the taker's side is fully discharged" );
}

/// Conservation: the totals do not move, whatever happens on the matching path.
///
/// Reserved, released, settled, cancelled — every one of those is a
/// rearrangement inside one pair or a transfer between two, so the sums over
/// all accounts are the same at the end as at the beginning.
#[ test ]
fn the_totals_are_unchanged_by_the_whole_matching_path()
{
  let mut escrow = two_sided();
  let opening_cash = escrow.total_cash().unwrap();
  let opening_asset = escrow.total_asset().unwrap();

  let seller = order( 1, 1, Side::Sell, "2.50", 10 );
  let buyer = order( 2, 2, Side::Buy, "3.00", 4 );
  let idle = order( 3, 2, Side::Buy, "1.00", 5 );
  escrow.reserve( &seller ).unwrap();
  escrow.reserve( &buyer ).unwrap();
  escrow.reserve( &idle ).unwrap();

  let trade = Trade
  {
    taker : buyer.id,
    taker_account : buyer.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "2.50" ),
    quantity : qty( 4 ),
  };
  escrow.settle( &trade, Side::Buy, buyer.price ).unwrap();
  escrow.release( AccountId( 2 ), idle.id ).unwrap();
  escrow.release( AccountId( 1 ), seller.id ).unwrap();

  assert_eq!( escrow.total_cash().unwrap(), opening_cash, "no currency appeared or vanished" );
  assert_eq!( escrow.total_asset().unwrap(), opening_asset, "and no asset did either" );
  assert_eq!( escrow.reservation_count(), 0, "with nothing left committed" );
}

/// The bridge assertion: the stored reserve equals the sum of what rests.
///
/// `reserved` is a stored field, never recomputed from the orders. This is the
/// one place the two are compared — which is what makes the storage safe
/// rather than merely convenient.
#[ test ]
fn the_stored_reserve_agrees_with_the_orders_behind_it()
{
  let mut escrow = funded();
  let orders =
  [
    order( 1, 1, Side::Buy, "2.50", 10 ),
    order( 2, 1, Side::Buy, "1.00", 20 ),
    order( 3, 1, Side::Buy, "0.50", 4 ),
  ];

  let mut expected = Money::ZERO;
  for order in &orders
  {
    escrow.reserve( order ).unwrap();
    let Obligation::Cash( committed ) = escrow.reserved_for( order.id ).unwrap()
    else
    {
      panic!( "a buy commits currency" );
    };
    expected = expected.checked_add( committed ).unwrap();

    assert_eq!
    (
      escrow.account( AccountId( 1 ) ).unwrap().cash.reserved(),
      expected,
      "after reserving order {}",
      order.id.0,
    );
  }
}

/// Every escrow refusal says which one it is, in words.
///
/// A `Display` that panicked or printed nothing would turn a rejection into a
/// mystery at exactly the moment a caller needs to explain itself.
#[ test ]
fn every_refusal_names_itself()
{
  let refusals =
  [
    EscrowError::UnknownAccount( AccountId( 9 ) ),
    EscrowError::Insufficient,
    EscrowError::NotReserved,
    EscrowError::NoReservation( OrderId( 9 ) ),
    EscrowError::AlreadyReserved( OrderId( 9 ) ),
    EscrowError::ObligationMismatch,
    EscrowError::NegativeAmount,
    EscrowError::Arithmetic,
  ];

  for refusal in refusals
  {
    assert!( refusal.to_string().len() > 10, "{refusal:?} explains nothing" );
  }
}

/// A negative obligation is not a reservation, and reserving one mints
/// spendable currency that every conservation sum in this crate agrees never
/// appeared.
///
/// # Root Cause
///
/// `Holding`'s four edges each move `amount` from one side of the partition to
/// the other, and all four assume without checking that `amount` is not
/// negative. For `Quantity` that assumption is free — the type refuses a
/// negative in its own constructor. For `Money` it is not: `Money` is signed,
/// and this crate's readme says so and says the guard is explicit. The guard
/// is `Conserved::checked_minus`'s `difference >= ZERO`, which constrains the
/// *result* of the subtraction and says nothing about the sign of the operand.
/// Handed `-200`, `reserve` computes `available - (-200)` — larger, and
/// trivially over the floor — then `reserved + (-200)`, which has no floor at
/// all because a `checked_plus` was never expected to shrink anything.
///
/// # Why Not Caught
///
/// Every existing conservation assertion here sums `available + reserved` and
/// compares it against the opening balance. That sum is exactly what this
/// preserves: the currency conjured into `available` is the same currency
/// driven out of `reserved`, so `total_cash()` is correct to the minor unit
/// before and after. The suite's strongest property is blind to the defect by
/// construction, and only splitting the pair — asserting on `available` alone
/// — can see it.
///
/// # Fix Applied
///
/// All four edges refuse a negative `amount` up front with
/// `EscrowError::NegativeAmount`, expressed once in `Holding::movement` and
/// stated in terms `Conserved` already supplies (`Ord` against `NOTHING`), so
/// the check is generic rather than a `Money`-shaped special case.
///
/// # Prevention
///
/// An invariant that names a value's sign has to be checked against the value
/// that carries it. Checking a downstream result instead only holds while
/// every operand is non-negative — which is the assumption under test.
///
/// # Pitfall
///
/// A conservation test is not a solvency test. `available + reserved` is held
/// invariant by any bug that moves value between the two halves, including one
/// that moves it the wrong way; the half that can actually be spent is the one
/// worth asserting on.
#[ test ]
fn a_negative_obligation_is_refused_rather_than_minting_spendable_currency()
{
  let mut escrow = funded();
  let opening = escrow.total_cash().unwrap();

  // The control: 1100 is more than this account holds, and is refused.
  let honest = order( 1, 1, Side::Buy, "2.00", 550 );
  assert_eq!( escrow.reserve( &honest ), Err( EscrowError::Insufficient ) );

  // The exploit: a buy at -2.00 for 100 units owes -200.
  let minting = order( 2, 1, Side::Buy, "-2.00", 100 );
  assert_eq!
  (
    escrow.reserve( &minting ),
    Err( EscrowError::NegativeAmount ),
    "a negative obligation is not something an account can commit",
  );

  let account = escrow.account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "1000" ), "no currency was created" );
  assert_eq!( account.cash.reserved(), Money::ZERO, "and none was driven below zero to pay for it" );
  assert_eq!( escrow.reservation_count(), 0, "nothing was recorded" );
  assert_eq!( escrow.total_cash().unwrap(), opening, "the total is unchanged — as it would have been either way" );

  // And the control still refuses, which is the whole point: the refused
  // order above must not have left 1200 spendable behind it.
  assert_eq!( escrow.reserve( &honest ), Err( EscrowError::Insufficient ) );
}

/// A settlement naming a larger amount than the order's own stored
/// reservation is refused, not silently allowed to drive the reservation
/// negative — even when the *account's* total reserved balance easily covers
/// it because a second, unrelated order pads it out.
///
/// # Root Cause
///
/// `reduce_reservation`'s `Side::Buy` branch (now `reduced_obligation`)
/// subtracted `cash_spent` from the order's stored `Obligation::Cash` using
/// the raw `Money::checked_sub`, which only fails on true `i64` overflow —
/// `Money` is signed, so `10 - 100` succeeds and returns `-90`. The parallel
/// `Side::Sell` branch could not make the same mistake: `Quantity::checked_sub`
/// refuses a negative result in its own constructor. The two branches
/// silently diverged on exactly the one property both are supposed to share.
///
/// # Why Not Caught
///
/// Every existing settlement test passes `settle` a `taker_limit` that
/// genuinely matches the order's own reserved price, because that is what
/// `Exchange::submit` always supplies. Nothing exercised a caller reaching
/// `Escrow` directly with a `taker_limit` inflated past what was actually
/// reserved, while the *account's* total reserved balance — padded by a
/// second, unrelated order — still had plenty of room to absorb it. That
/// combination is exactly the condition needed to see the per-order ledger
/// diverge from the account-level one.
///
/// # Fix Applied
///
/// `reduced_obligation` now uses `Conserved::checked_minus` for both shapes
/// uniformly — the same abstraction every edge of `Holding` already uses —
/// so a Cash reservation refuses to go below zero exactly as an Asset
/// reservation always did. `settle` itself was also restructured to compute
/// every account and reservation update against local copies before
/// committing any of them, so this refusal leaves nothing partially applied.
///
/// # Prevention
///
/// Two branches meant to enforce the same invariant should share one
/// implementation of it rather than each reaching for whichever operation on
/// the underlying type happened to be in scope.
///
/// # Pitfall
///
/// A check at the account level (`Holding.reserved`, the sum across every
/// order) does not stand in for a check at the order level
/// (`Escrow.reservations`, one order's own remaining share) — an account with
/// several live orders can easily have enough in the pooled total to mask a
/// single order being over-claimed.
#[ test ]
fn a_settlement_naming_more_than_the_orders_own_reservation_is_refused()
{
  let mut escrow = two_sided();

  // Buyer (account 2) holds two simultaneous reservations: a small one
  // (order 2, this test's target) and an unrelated large one (order 9) that
  // pads the account's pooled `cash.reserved` well past what order 2 alone
  // committed.
  let seller = order( 1, 1, Side::Sell, "2.50", 4 );
  let buyer_small = order( 2, 2, Side::Buy, "2.50", 4 );
  let buyer_padding = order( 9, 2, Side::Buy, "2.50", 200 );
  escrow.reserve( &seller ).unwrap();
  escrow.reserve( &buyer_small ).unwrap();
  escrow.reserve( &buyer_padding ).unwrap();
  assert_eq!( escrow.account( AccountId( 2 ) ).unwrap().cash.reserved(), money( "510" ), "both reservations pooled" );

  let before = escrow.clone();

  // The trade itself is honest — 4 units at 2.50, exactly what the seller
  // reserved, so nothing on the asset side objects. The inconsistency is
  // isolated to the *claimed limit* passed to `settle`: a caller reaching
  // `Escrow` directly names 30.00 as order 2's reserved limit rather than its
  // true 2.50, inflating `reserved_for_this_fill` to 120 — an amount the
  // account's pooled $510 easily absorbs, but order 2 itself only ever
  // committed $10 of.
  let trade = Trade
  {
    taker : buyer_small.id,
    taker_account : buyer_small.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "2.50" ),
    quantity : qty( 4 ),
  };

  assert_eq!
  (
    escrow.settle( &trade, Side::Buy, price( "30.00" ) ),
    Err( EscrowError::NotReserved ),
    "order 2's own reservation cannot cover this, regardless of the account total",
  );
  assert_eq!( escrow, before, "a refused settlement changes nothing at all — not order 2, not order 9, not either account" );
}

/// A settlement that fails on its very last step — crediting the buyer's
/// asset — leaves every earlier step's effect uncommitted too, not applied
/// halfway.
///
/// # Root Cause
///
/// `settle` used to mutate the buyer's account directly, one `Holding` edge
/// at a time (`deliver`, then `release`, then `receive`), each through `?`.
/// A failure on the *last* edge left the first two already written into
/// `self.accounts` — cash delivered out of the buyer's reservation with
/// nothing credited back anywhere, no seller ever paid, and — on the real
/// `Exchange::submit` path — the resting order already gone from the book by
/// the time this function is even called.
///
/// # Why Not Caught
///
/// Every existing settlement test uses account balances comfortably inside
/// the representable ceiling, so `receive` never had a reason to fail after
/// `deliver`/`release` had already succeeded. Reaching the failure needs an
/// account already holding the maximum representable asset quantity — legal,
/// if extreme — one more unit away from overflowing.
///
/// # Fix Applied
///
/// `settle` now computes every resulting balance and every reservation
/// update against local copies first, and writes nothing back into `self`
/// until all four have succeeded.
///
/// # Prevention
///
/// A function with more than one fallible mutation and no rollback is an
/// atomicity bug waiting on whichever step is least likely to fail — "least
/// likely" is not "never," and the failure that does happen finds every
/// earlier step already committed.
///
/// # Pitfall
///
/// The failure this test drives has nothing to do with insufficient funds —
/// the buyer's cash is ample. It is the asset side's own representable
/// ceiling, independent of the trade's price or size, refusing the very last
/// unit of headroom.
#[ test ]
fn a_settlement_failing_on_its_last_step_leaves_nothing_committed()
{
  let mut escrow = Escrow::new();
  // The buyer already holds the maximum representable asset quantity, and
  // just enough cash for one more, vanishingly small trade.
  escrow.open( AccountId( 1 ), money( "1" ), Quantity::from_int( CEILING_WHOLE_UNITS ).unwrap() ).unwrap();
  escrow.open( AccountId( 2 ), Money::ZERO, Quantity::from_minor( 1 ).unwrap() ).unwrap();

  let buyer = Order
  {
    id : OrderId( 10 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ), side : Side::Buy,
    price : price( "1" ), quantity : Quantity::from_minor( 1 ).unwrap(), tif : Tif::Gtc,
    client : None,
  };
  let seller = Order
  {
    id : OrderId( 11 ), instrument : InstrumentId( 1 ), account : AccountId( 2 ), side : Side::Sell,
    price : price( "1" ), quantity : Quantity::from_minor( 1 ).unwrap(), tif : Tif::Gtc,
    client : None,
  };
  escrow.reserve( &buyer ).unwrap();
  escrow.reserve( &seller ).unwrap();

  let before = escrow.clone();

  let trade = Trade
  {
    taker : buyer.id,
    taker_account : buyer.account,
    taker_side : Side::Buy,
    maker : seller.id,
    maker_account : seller.account,
    price : price( "1" ),
    quantity : Quantity::from_minor( 1 ).unwrap(),
  };

  assert_eq!
  (
    escrow.settle( &trade, Side::Buy, buyer.price ),
    Err( EscrowError::Arithmetic ),
    "the buyer's asset holding is already at the representable ceiling",
  );
  assert_eq!
  (
    escrow, before,
    "a settlement failing on its last step changes nothing at all — not even the earlier \
     steps that, in isolation, would have succeeded",
  );
}

/// A release naming the wrong account for an order fails without deleting
/// that order's reservation record — even though the wrong-but-real account
/// exists and its own lookup succeeds; only its own, unrelated balance turns
/// out not to cover the amount.
///
/// # Root Cause
///
/// `release` removed `order`'s entry from `self.reservations` as its first
/// statement, unconditionally — a `BTreeMap::remove` mutates on the `Some`
/// path regardless of what the rest of the function goes on to do. Two more
/// fallible steps followed the removal: the account lookup, and the
/// account-level `Holding::release` itself. Either failing returned `Err`
/// with the ledger entry already gone — the true owner's reservation was
/// orphaned, since nothing else ever reads or re-derives it.
///
/// # Why Not Caught
///
/// Every existing call to `release` in this suite, and both call sites in
/// the exchange engine (`Exchange::cancel` and the self-match cancellation
/// path), name the order's own true owner — each derives `owner` from the very order
/// record that was used to reserve it, so `owner` can never mismatch through
/// either mediated path. Nothing exercised a caller reaching `Escrow` directly
/// with an `owner` naming a different, real account.
///
/// # Fix Applied
///
/// The ledger entry is now removed only after the account-level move has
/// already succeeded — the same compute-then-commit ordering `settle` was
/// given, for the same reason.
///
/// # Prevention
///
/// A `remove` used where a `get` would do mutates whether or not the rest of
/// the function goes on to fail; a lookup that might still need to fail
/// afterward must peek rather than take, and commit the removal last, not
/// first.
///
/// # Pitfall
///
/// The account-level failure here is `NotReserved`, not `UnknownAccount` — the
/// wrong account is perfectly real, just uninvolved with this order. A fix
/// that only special-cased an unknown account would miss this path entirely,
/// since `self.accounts.get_mut(&owner)` succeeds right up until the
/// `Holding::release` call after it.
#[ test ]
fn a_release_naming_the_wrong_account_leaves_the_reservation_intact()
{
  let mut escrow = two_sided();
  let buyer = order( 1, 1, Side::Buy, "2.50", 10 );
  escrow.reserve( &buyer ).unwrap();

  let account_2_before = *escrow.account( AccountId( 2 ) ).unwrap();

  // Account 2 is real, but holds no reservation of its own — the account
  // lookup succeeds, and only the account-level `release` itself fails.
  assert_eq!
  (
    escrow.release( AccountId( 2 ), OrderId( 1 ) ),
    Err( EscrowError::NotReserved ),
    "account 2 exists but never reserved anything for order 1",
  );

  assert_eq!
  (
    escrow.reserved_for( OrderId( 1 ) ),
    Some( Obligation::Cash( money( "25" ) ) ),
    "the failed release must not have deleted order 1's reservation record",
  );
  assert_eq!( escrow.account( AccountId( 2 ) ).unwrap(), &account_2_before, "the wrong account is untouched" );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().cash.reserved(), money( "25" ), "the true owner still holds it" );

  // And the reservation is still genuinely usable afterwards — the earlier
  // failure did not half-consume it either.
  let released = escrow.release( AccountId( 1 ), OrderId( 1 ) ).unwrap();
  assert_eq!( released, Obligation::Cash( money( "25" ) ) );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().cash.available(), money( "1000" ), "correctly released afterwards" );
  assert_eq!( escrow.reservation_count(), 0 );
}

/// A second `open` for an already-open account tops up `available` and
/// leaves a live reservation exactly where it was — not zeroed out from
/// under it.
///
/// # Root Cause
///
/// `open` always ran `self.accounts.insert(id, Account { cash :
/// Holding::new(cash), .. })`, and `BTreeMap::insert` on an existing key
/// discards the old value outright — `Holding::new` sets `reserved` to
/// `NOTHING`, so a second `open` for an `id` with a resting order's
/// reservation zeroed that reservation's backing while `self.reservations`
/// kept its entry pointing at the same, now-hollowed-out account.
///
/// # Why Not Caught
///
/// Every existing call to `open` in this suite, and every production call
/// site, opened a fresh `AccountId` exactly once before ever reserving
/// against it — nothing exercised a second `open` for an `id` that already
/// held a live reservation.
///
/// # Fix Applied
///
/// `open` now looks up `id` first: an existing account merges the new
/// `cash`/`asset` into `available` via the same checked `Holding::receive`
/// `settle` already uses for incoming funds; only a genuinely new `id`
/// still goes through `Holding::new`.
///
/// # Prevention
///
/// Any edge that replaces an `Account` wholesale rather than moving a
/// checked amount across its existing `Holding`s can silently reintroduce
/// this divergence — `reserved` is stored, never derived (see this crate's
/// own module doc), so nothing else notices it was zeroed out from under a
/// live reservation.
///
/// # Pitfall
///
/// The reservation must be shown as still genuinely usable, not merely
/// still counted — this test carries it all the way through a `release`
/// afterward, since a subtler bug could zero the ledger entry while leaving
/// the account-level counter alone, or the reverse.
#[ test ]
fn a_second_open_tops_up_available_and_leaves_a_live_reservation_intact()
{
  let mut escrow = funded();
  let buyer = order( 1, 1, Side::Buy, "2.50", 10 );
  escrow.reserve( &buyer ).unwrap();

  // Account 1 now: available 975, reserved 25 (cash); asset untouched at 100.
  escrow.open( AccountId( 1 ), money( "500" ), Quantity::ZERO ).unwrap();

  let account = escrow.account( AccountId( 1 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "1475" ), "975 + 500 topped up, not replaced" );
  assert_eq!( account.cash.reserved(), money( "25" ), "the live reservation must survive the top-up" );
  assert_eq!( account.asset.available(), qty( 100 ), "an untouched asset deposit changes nothing" );

  assert_eq!
  (
    escrow.reserved_for( OrderId( 1 ) ),
    Some( Obligation::Cash( money( "25" ) ) ),
    "the reservation ledger must still point at a live reservation",
  );

  // And it is still genuinely usable afterwards — not just still counted.
  let released = escrow.release( AccountId( 1 ), OrderId( 1 ) ).unwrap();
  assert_eq!( released, Obligation::Cash( money( "25" ) ) );
  assert_eq!( escrow.account( AccountId( 1 ) ).unwrap().cash.available(), money( "1500" ), "1475 + 25 released back" );
  assert_eq!( escrow.reservation_count(), 0 );
}

/// `open` on a brand-new `id` with a non-negative deposit succeeds — the
/// fresh-account branch, pinned beside the merge path's failures.
#[ test ]
fn opening_a_brand_new_account_still_always_succeeds()
{
  let mut escrow = Escrow::new();
  assert_eq!( escrow.open( AccountId( 7 ), money( "42" ), qty( 3 ) ), Ok( () ) );

  let account = escrow.account( AccountId( 7 ) ).unwrap();
  assert_eq!( account.cash.available(), money( "42" ) );
  assert_eq!( account.asset.available(), qty( 3 ) );
  assert_eq!( account.cash.reserved(), Money::ZERO );
}

/// A second `open` that succeeds on the cash leg but fails on the asset leg
/// leaves the cash leg uncommitted too — not credited while the call as a
/// whole reports `Err`.
///
/// # Root Cause
///
/// The merge branch (`Some(account)`) ran `account.cash.receive(cash)?` and
/// `account.asset.receive(asset)?` as two independent statements, each
/// mutating `account` — which is a live `&mut` into `self.accounts` — directly
/// through `?`. The first call's success is not provisional: `receive` writes
/// `self.available` before returning, so a subsequent failure on the second
/// call still leaves the first call's write in place. `settle` and `release`
/// were both hardened against exactly this shape earlier (see their own Fix
/// comments in this file), but `open`'s merge branch predates that pattern and
/// was never brought in line with it.
///
/// # Why Not Caught
///
/// Every existing `open` test either targets the always-succeeds fresh-account
/// branch, or a merge that keeps both legs comfortably inside the
/// representable ceiling, so `asset.receive` never had a reason to fail after
/// `cash.receive` had already succeeded. Reaching the failure needs a merge
/// where the asset leg is already at its representable ceiling and the cash
/// leg is not — the same shape
/// `a_settlement_failing_on_its_last_step_leaves_nothing_committed` uses for
/// `settle`, applied here to `open`.
///
/// # Fix Applied
///
/// `open`'s merge branch now computes both `receive` results against local
/// copies first, and writes neither back into `self.accounts` until both have
/// succeeded — the same compute-then-commit ordering `settle` and `release`
/// already use.
///
/// # Prevention
///
/// A merge of two independent fallible edges must not commit the first one
/// before the second has also succeeded — `?` alone does not provide that,
/// since it early-returns only after whatever it followed already ran.
///
/// # Pitfall
///
/// The account-level error here is `Arithmetic`, not `Insufficient` — the
/// account is not short of anything, and it is the asset side's own ceiling,
/// independent of the cash amount, that refuses the second leg.
#[ test ]
fn a_second_open_failing_on_the_asset_leg_leaves_the_cash_leg_uncommitted()
{
  let mut escrow = Escrow::new();
  escrow.open( AccountId( 1 ), money( "1" ), Quantity::from_int( CEILING_WHOLE_UNITS ).unwrap() ).unwrap();

  let before = escrow.clone();

  assert_eq!
  (
    escrow.open( AccountId( 1 ), money( "500" ), Quantity::from_minor( 1 ).unwrap() ),
    Err( EscrowError::Arithmetic ),
    "the asset leg is already at the representable ceiling",
  );
  assert_eq!
  (
    escrow, before,
    "a merge failing on its second leg changes nothing at all — not even the first \
     leg, which, in isolation, would have succeeded",
  );
}

/// A fresh account cannot start below zero — the same refusal a top-up gets.
#[ test ]
fn a_negative_first_deposit_is_refused()
{
  let mut escrow = Escrow::new();

  assert_eq!( escrow.open( AccountId( 7 ), money( "-1" ), qty( 0 ) ), Err( EscrowError::NegativeAmount ) );
  assert!( escrow.account( AccountId( 7 ) ).is_none(), "no account was created" );
}

/// A buy at price zero holds `Cash( 0 )` from the start, so its reservation
/// must outlive a partial fill on its quantity, not its amount: the
/// remainder still rests, and has to stay cancellable and settleable.
#[ test ]
fn a_zero_price_buy_keeps_its_reservation_until_its_quantity_is_filled()
{
  let mut escrow = two_sided();
  let buyer = order( 1, 2, Side::Buy, "0", 2 );
  let first = order( 2, 1, Side::Sell, "0", 1 );
  let second = order( 3, 1, Side::Sell, "0", 1 );
  escrow.reserve( &buyer ).unwrap();
  escrow.reserve( &first ).unwrap();
  escrow.reserve( &second ).unwrap();

  let fill = | taker : &Order, maker : &Order | Trade
  {
    taker : taker.id,
    taker_account : taker.account,
    taker_side : taker.side,
    maker : maker.id,
    maker_account : maker.account,
    price : price( "0" ),
    quantity : qty( 1 ),
  };

  escrow.settle( &fill( &buyer, &first ), Side::Buy, buyer.price ).unwrap();
  assert_eq!( escrow.reserved_for( buyer.id ), Some( Obligation::Cash( Money::ZERO ) ), "one unit is still unfilled" );

  let mut cancelled = escrow.clone();
  assert_eq!( cancelled.release( AccountId( 2 ), buyer.id ), Ok( Obligation::Cash( Money::ZERO ) ), "the remainder can be cancelled" );

  escrow.settle( &fill( &second, &buyer ), Side::Sell, second.price ).unwrap();
  assert_eq!( escrow.reserved_for( buyer.id ), None, "fully filled, fully discharged" );
  assert_eq!( escrow.reservation_count(), 0 );
  assert_eq!( escrow.account( AccountId( 2 ) ).unwrap().asset.available(), qty( 102 ) );
}
