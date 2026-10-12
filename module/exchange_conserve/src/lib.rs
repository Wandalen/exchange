//! A batch of fills nets to zero across its legs, or is refused. Closes hard
//! problem 5 and feature 21. `exchange_match::cross` calls
//! [`conserve_assert`] on every batch it produces.
//!
//! # Two legs per trade
//!
//! The source design signs one leg per trade by `taker_side`. Every trade one
//! `cross` call produces shares its taker, so those legs would all carry one
//! sign and never net to zero. Each trade here contributes both: the buyer
//! debited the notional, the seller credited it. The two come from one
//! `price`/`quantity`, so they always cancel and
//! [`ConserveError::Unbalanced`] is unreachable today. It stays for the day a
//! fee makes the debit and the credit differ.
//!
//! [`ConserveError::Notional`] is reachable — a trade whose notional is not
//! exact, as in `demo_p17_cons`. [`ConserveError::Overflow`] is not: a
//! notional negates within `Money`'s range, and the sum runs in `i128`.
//!
//! # Not `Escrow::total_cash`/`total_asset`
//!
//! The source design names those as this crate's extraction source. They sum
//! the ledger's own balances, which agree by construction; this checks one
//! batch of trades before escrow or the event stream sees it.
//! `exchange_core::postings` builds the same cash legs, plus the asset legs,
//! for [`exact_arith::verify`] to audit.

use exact_arith::{ ConservationError, Money, money_sum_assert_zero };
use exchange_fill::Trade;
use exchange_side::Side;
use exchange_types::TypeError;

/// Why a batch of fills could not be confirmed to conserve.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConserveError
{
  /// One trade's own `price`/`quantity` could not be expressed as a notional.
  Notional( TypeError ),
  /// A debit or the batch's sum left the representable range.
  Overflow,
  /// Every leg was expressible and summed without overflow, but the total
  /// was not zero.
  Unbalanced
  {
    /// The batch's net, in minor currency units: positive means value
    /// appeared, negative that it vanished.
    got : i128,
  },
}

impl core::fmt::Display for ConserveError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Notional( error ) => write!( f, "a trade's notional is not expressible: {error}" ),
      Self::Overflow => write!( f, "a debit or the batch's total left the representable range" ),
      Self::Unbalanced { got } => write!( f, "the batch's signed legs summed to {got} minor units, not zero" ),
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
      Self::Overflow | Self::Unbalanced { .. } => None,
    }
  }
}

/// Confirm `fills`, taken as one batch, nets to zero: every trade's own
/// notional is expressible, and the running sum fits.
///
/// Both of a trade's legs go in; `taker_side` only labels which is the
/// buyer's. See the module doc's "Two legs per trade".
///
/// # Errors
///
/// [`ConserveError::Notional`] if a trade's own `price`/`quantity` cannot be
/// expressed, [`ConserveError::Overflow`] if a debit or the sum does not fit,
/// [`ConserveError::Unbalanced`] if the batch's legs somehow still sum to
/// anything but zero.
///
/// ```rust
/// use exact_arith::{ Price, Quantity };
/// use exchange_conserve::conserve_assert;
/// use exchange_fill::Trade;
/// use exchange_id::{ AccountId, OrderId };
/// use exchange_side::Side;
///
/// let trade = Trade
/// {
///   taker : OrderId( 1 ), taker_account : AccountId( 1 ), taker_side : Side::Buy,
///   maker : OrderId( 2 ), maker_account : AccountId( 2 ),
///   price : Price::parse( "2.50" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(),
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

  money_sum_assert_zero( &legs ).map_err( | error | match error
  {
    ConservationError::NotZero { got } => ConserveError::Unbalanced { got },
    ConservationError::Overflow => ConserveError::Overflow,
  } )
}

// The readme's example repeats `conserve_assert`'s doctest; compiling it here
// keeps that copy from drifting.
#[ cfg( doctest ) ]
#[ doc = include_str!( "../readme.md" ) ]
struct ReadmeDoctest;
