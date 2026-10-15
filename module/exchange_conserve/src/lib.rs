//! A batch of fills nets to zero across its legs, or is refused.
//!
//! # Not extracted from `Escrow::total_cash`/`total_asset`
//!
//! The source design names those two methods as this crate's extraction
//! source. They check the *ledger's* own internal self-consistency — the sum
//! of every account's `available + reserved`, which is trivially true by
//! construction, since it sums the same state every other `Escrow` method
//! already maintains. That is a different check at a different grain from
//! hard problem 5 / feature 21's actual ask: whether *one batch of fills*,
//! taken on its own with no ledger in sight, nets to zero.
//!
//! # Revision — one signed leg per trade did not survive contact with `cross`
//!
//! The first design (one leg per trade, signed by `taker_side`: negative on a
//! buy, positive on a sell) was built to match the family's proposal and the
//! P17 phase doc's literal "10 and -10" example. It could never be called
//! correctly from `exchange_match::cross`: every trade one call to `cross`
//! produces shares the *same* `taker_side`, because there is exactly one
//! incoming order per call — so every leg in a real batch carries the same
//! sign, and a sum of same-signed non-zero legs can never land on zero. Wired
//! in as the plan originally described, every successful match would have
//! come back `Unbalanced`.
//!
//! The fix is this crate's second design: **both legs, every trade.** Each
//! trade debits whichever side is the buyer and credits whichever is the
//! seller — `taker_side` says which one the taker was, the maker is always
//! its opposite — and both go into the batch together. A trade's own debit
//! and credit are computed from the same `price`/`quantity`, so they always
//! cancel; a batch of any size, any mix of `taker_side`, sums to zero by the
//! same construction `postings()` already relies on below. [`ConserveError::Unbalanced`]
//! is consequently **unreachable through this function today** — recorded
//! here rather than left for a mutation test to discover by surprise (see
//! `tests/manual/readme.md`'s M0). It stays rather than being deleted because
//! the day a fee enters this family's design, a buyer's debit and a seller's
//! credit stop being the same number, and this is exactly the check that
//! starts catching a fee silently miscomputed on one side only.
//!
//! # What this still catches
//!
//! [`ConserveError::Notional`] if any trade's own `price`/`quantity` cannot be
//! expressed exactly, and [`ConserveError::Overflow`] if the running sum
//! leaves the representable range — both real, reachable failures, demoed by
//! [`conserve_assert`]'s own doctest and `demo_p17_cons`.
//!
//! # Relationship to `exchange_core::postings`
//!
//! `exchange_core::postings` builds the same debit-buyer/credit-seller pair
//! per trade, across the exchange's *entire* event log, for
//! [`exact_arith::verify`] to grade with machinery the exchange did not
//! write (see that function's own doc comment). This crate's
//! [`conserve_assert`] is the same shape at a narrower, pre-ledger grain — one
//! batch, straight off `Trade` data, with no event log or account lookup
//! involved — callable the instant `cross` produces a batch, before escrow or
//! the event stream see it at all.

use exact_kind::Money;
use exchange_fill::Trade;
use exchange_side::Side;
use exchange_types::TypeError;

/// Why a batch of fills could not be confirmed to conserve.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConserveError
{
  /// One trade's own `price`/`quantity` could not be expressed as a notional.
  Notional( TypeError ),
  /// A running sum left the representable range.
  Overflow,
  /// Every leg was expressible and summed without overflow, but the total
  /// was not zero.
  Unbalanced,
}

impl core::fmt::Display for ConserveError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Notional( error ) => write!( f, "a trade's notional is not expressible: {error}" ),
      Self::Overflow => write!( f, "the batch's running total left the representable range" ),
      Self::Unbalanced => write!( f, "the batch's signed legs did not sum to zero" ),
    }
  }
}

impl core::error::Error for ConserveError
{
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Notional( error ) => Some( error ),
      Self::Overflow | Self::Unbalanced => None,
    }
  }
}

/// Sum a set of already-signed legs — positive where cash arrived, negative
/// where it left.
///
/// The primitive [`conserve_assert`] builds on. Classifying a batch of
/// `Trade`s into signed legs is the only part that needs `taker_side`;
/// summing a leg set to see whether it nets to zero does not, so that half is
/// kept separate and directly testable against plain values.
///
/// # Errors
///
/// [`ConserveError::Overflow`] if the running sum leaves the representable
/// range.
///
/// ```rust
/// use exact_kind::Money;
/// use exchange_conserve::fill_legs_sum;
///
/// let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -10 ).unwrap() ];
/// assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::ZERO );
/// ```
pub fn fill_legs_sum( legs : &[ Money ] ) -> Result< Money, ConserveError >
{
  legs.iter().try_fold( Money::ZERO, | sum, &leg | sum.checked_add( leg ).map_err( | _ | ConserveError::Overflow ) )
}

/// Confirm `fills`, taken as one batch, nets to zero: every trade's own
/// notional is expressible, and the running sum fits.
///
/// Both of a trade's legs go in: a debit for whichever side bought, a credit
/// for whichever sold. `taker_side` names which one the taker was — the
/// maker is always its opposite — purely to label which amount is whose; the
/// sum this checks is identical either way `taker_side` reads, since addition
/// does not care which order its two terms were pushed in. See the module
/// doc's "Revision" section for why that makes [`ConserveError::Unbalanced`]
/// unreachable today, and why it stays regardless.
///
/// # Errors
///
/// [`ConserveError::Notional`] if a trade's own `price`/`quantity` cannot be
/// expressed, [`ConserveError::Overflow`] if the running sum does not fit,
/// [`ConserveError::Unbalanced`] if the batch's legs somehow still sum to
/// anything but zero.
///
/// ```rust
/// use exact_kind::{ Money, Quantity };
/// use exchange_conserve::conserve_assert;
/// use exchange_fill::Trade;
/// use exchange_id::{ AccountId, OrderId };
/// use exchange_side::Side;
///
/// let trade = Trade
/// {
///   taker : OrderId( 1 ), taker_account : AccountId( 1 ), taker_side : Side::Buy,
///   maker : OrderId( 2 ), maker_account : AccountId( 2 ),
///   price : Money::parse( "2.50" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(),
/// };
/// assert!( conserve_assert( &[ trade ] ).is_ok() );
/// ```
pub fn conserve_assert( fills : &[ Trade ] ) -> Result< (), ConserveError >
{
  let mut legs = Vec::with_capacity( fills.len() * 2 );

  for trade in fills
  {
    let notional = exchange_types::notional( trade.price, trade.quantity ).map_err( ConserveError::Notional )?;
    let debit = Money::ZERO.checked_sub( notional ).map_err( | _ | ConserveError::Overflow )?;

    match trade.taker_side
    {
      Side::Buy => { legs.push( debit ); legs.push( notional ); },
      Side::Sell => { legs.push( notional ); legs.push( debit ); },
    }
  }

  let net = fill_legs_sum( &legs )?;

  if net == Money::ZERO
  {
    Ok( () )
  }
  else
  {
    Err( ConserveError::Unbalanced )
  }
}
