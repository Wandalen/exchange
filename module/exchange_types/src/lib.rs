//! What an order commits, priced exactly: [`notional`], [`obligation`] and
//! [`TypeError`].
//!
//! A notional is exact or an error, never rounded: a rounded settlement
//! amount is value created or destroyed one minor unit at a time, and every
//! per-account sum still balances.
//!
//! The family's two prohibitions — no ECS type, no floating point — are
//! checked across every crate by `exchange_core/tests/contract_test.rs`.

use exact_arith::{ Backing, MONEY_SCALE, Money, Price, Quantity, pow10 };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;

/// A quantity or price this crate could not express.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum TypeError
{
  /// A notional whose exact value does not fit the currency type.
  NotionalOutOfRange,
  /// A notional that would need more precision than the currency type has.
  /// Refused rather than rounded: rounding a settlement amount is how value
  /// appears and vanishes one unit at a time.
  NotionalInexact,
}

impl core::fmt::Display for TypeError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::NotionalOutOfRange => write!( f, "the notional is outside the representable range" ),
      Self::NotionalInexact => write!( f, "the notional does not land exactly on the currency scale" ),
    }
  }
}

impl core::error::Error for TypeError {}

/// The exact currency value of `quantity` units at `price`.
///
/// Both operands carry scale [`MONEY_SCALE`], so their product lands at twice
/// that and must come back down. It comes down by exact division only: a
/// product with any remainder at the currency's own scale is **refused**, not
/// rounded. That refusal is the whole point — a settlement amount that has
/// been rounded is value created or destroyed, and it is created or destroyed
/// in units too small for anyone to notice until an audit sums them.
///
/// # Errors
///
/// [`TypeError::NotionalOutOfRange`] if the exact product does not fit the
/// currency type, [`TypeError::NotionalInexact`] if it would need rounding.
///
/// ```rust
/// use exchange_types::notional;
/// use exact_arith::{ Money, Price, Quantity };
///
/// let price = Price::parse( "1.25" ).unwrap();
/// let three = Quantity::from_int( 3 ).unwrap();
/// assert_eq!( notional( price, three ).unwrap(), Money::parse( "3.75" ).unwrap() );
/// ```
pub fn notional( price : Price, quantity : Quantity ) -> Result< Money, TypeError >
{
  let scale = i128::from( pow10( MONEY_SCALE ) );
  let product = i128::from( price.minor() ) * i128::from( quantity.minor() );

  if product % scale != 0
  {
    return Err( TypeError::NotionalInexact );
  }

  let minor = product / scale;
  let minor = Backing::try_from( minor ).map_err( | _ | TypeError::NotionalOutOfRange )?;

  // Every way `from_minor` can refuse — over the ceiling, or over the backing
  // width — is the same fact to a caller here: the exact answer exists and
  // this currency type cannot hold it.
  Money::from_minor( minor ).map_err( | _ | TypeError::NotionalOutOfRange )
}

/// What `order` must commit to be allowed to rest.
///
/// A sell owes the asset it is selling. A buy owes currency at **its own limit
/// price**, which is the most it can ever be asked to pay — reserving at the
/// eventual execution price is impossible, because that price is not known
/// until the match happens, and reserving less than the maximum is what makes
/// a resting order un-executable.
///
/// # Errors
///
/// [`TypeError`] when a buy's notional is not exactly expressible.
pub fn obligation( order : &Order ) -> Result< Obligation, TypeError >
{
  match order.side
  {
    Side::Buy => Ok( Obligation::Cash( notional( order.price, order.quantity )? ) ),
    Side::Sell => Ok( Obligation::Asset( order.quantity ) ),
  }
}
