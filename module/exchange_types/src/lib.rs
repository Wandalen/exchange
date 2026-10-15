//! The vocabulary every other `exchange_*` crate
//! speaks. Orders, trades, settlement obligations, and the event stream the
//! Contract names as half the family's output.
//!
//! Two properties of this crate are the family's, not this crate's, and
//! both are enforced here because this is where the types are declared:
//!
//! **No ECS type, anywhere.** The Contract's prohibition is absolute — no
//! `Entity`, no `World`, no `Component`, no `Query`, no system registration.
//! A trader is identified by `AccountId`, a number `exchange_id` owns. The
//! failure this prevents is not a compile error anyone would notice: an
//! exchange holding an `Entity` handle in a private field matches perfectly,
//! passes every behavioural test, and couples the market to the simulation's
//! storage layer. Keeping identity a plain integer is what makes the exchange
//! a library that a simulation may call rather than a part of one.
//!
//! **No floating point, anywhere.** Every price, quantity and amount is an
//! [`exact_arith`] value. The dependency is on the facade rather than on
//! `exact_kind` directly, so the family below it can be re-split without
//! this crate noticing.
//!
//! # Naming — `Trade`, not `Fill`
//!
//! The family's Contract says *trades out*; the matching algorithm talks
//! about *fills*. They are one thing, so there is one type: `Trade` (now
//! declared in `exchange_fill`), the record of one match. "Fill" survives as
//! a verb — an order is *fully filled*
//! or *partially filled* — describing what a Trade did to an order, never a
//! second record of it. Two types here would be two sources of truth for one
//! event, and they would disagree exactly when something had already gone
//! wrong.
//!
//! Design: the family's shared algorithm, protocol, and state-machine
//! documents specify behavior at family grain rather than per crate.
//!
//! # Extraction — retired as a re-export aggregator
//!
//! `Side`, `AccountId`, `OrderId` and `Sequence` moved out to their own
//! root crates — `exchange_side`, `exchange_id` (both ids), `exchange_seq` —
//! as part of aligning this family's crate composition with its source
//! design. `Order` and `Obligation` moved out next, to `exchange_order`,
//! gaining the `instrument`/`tif` fields the real struct was missing — see
//! that crate's own module doc for the full reasoning. `Trade`, `Event`,
//! `EventKind`, `RejectReason` and `CancelCause` moved out next again, to
//! `exchange_fill`, gaining a `taker_side` field on `Trade` — see that
//! crate's own module doc for why. All eleven were re-exported here
//! unchanged for a time, so every `use exchange_types::{ ... }` kept
//! resolving through the move; every call site has since been cut over to
//! the leaf crate directly, and the re-export block is gone — this crate no
//! longer carries any type it does not itself declare.
//!
//! `notional`/`TypeError`/`obligation` stay here: they are the one piece of
//! real logic this crate still owns. `Order`/`Obligation`/`Side` are named in
//! `obligation`'s own signature and body, so `exchange_order`/`exchange_side`
//! remain real dependencies — just no longer re-exported from here.

use exact_arith::{ Backing, MONEY_SCALE, Money, Quantity, pow10 };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;

/// The price of one unit, and the type every amount of currency is expressed
/// in.
///
/// An alias rather than a newtype: the ceiling, the scale and the checked
/// operations all belong to `exact_arith`, and wrapping them here would put
/// this crate in the position of re-deciding them.
pub type Price = Money;

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
/// use exact_arith::{ Money, Quantity };
///
/// let price = Money::parse( "1.25" ).unwrap();
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
