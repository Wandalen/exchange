//! Tick, lot, the asset pair, and the halt flag on one instrument.
//!
//! Snapping is not implemented here: [`price_snap`]/[`qty_snap`] wrap
//! `exact_arith`'s `price_snap_tick`/`qty_snap_lot`, which already refuse a
//! zero grid and pick the rounding — one source of truth for the arithmetic.
//! Closes hard problems 1, 8 and 18, and features 4 and 5. Not 19: `base`
//! and `quote` are recorded, but `exchange_escrow` holds one asset per
//! account.

use exact_arith::{ Lot, Price, Quantity, SnapError, Tick, price_snap_tick, qty_snap_lot, rounding_default };
use exchange_id::InstrumentId;

/// The identity of one of the two assets an instrument trades — the base
/// asset bought and sold, and the quote asset priced in. Distinct from
/// [`InstrumentId`]: an asset is a thing held; an instrument is a market
/// pairing two of them.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct AssetId( pub u32 );

/// One instrument's own grid and halt flag.
///
/// `tick`/`lot` are already-validated [`Tick`]/[`Lot`] values, not raw
/// [`Price`]/[`Quantity`] — the zero-check happens once, at construction via
/// [`spec_new`], rather than being re-checked on every snap call.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct InstrumentSpec
{
  /// This instrument's identity.
  pub id : InstrumentId,
  /// The asset bought and sold.
  pub base : AssetId,
  /// The asset it's priced in.
  pub quote : AssetId,
  /// The smallest meaningful price increment.
  pub tick : Tick,
  /// The smallest meaningful quantity increment.
  pub lot : Lot,
  /// Whether trading on this instrument is currently halted.
  pub halted : bool,
}

/// Why an instrument spec could not be built, or a snap could not complete.
///
/// A thin re-naming of [`SnapError`] rather than a second enum with the same
/// three shapes under different names — `exchange_spec` only ever fails the
/// way its one real dependency, snapping, fails.
pub type SpecError = SnapError;

/// Build an instrument spec, refusing a degenerate tick or lot.
///
/// Starts un-halted — a freshly-registered instrument trades immediately
/// unless something halts it afterward.
///
/// # Errors
///
/// [`SpecError::ZeroTick`] if `tick` is exactly zero, [`SpecError::ZeroLot`]
/// if `lot` is exactly zero.
pub fn spec_new
(
  id : InstrumentId,
  base : AssetId,
  quote : AssetId,
  tick : Price,
  lot : Quantity,
) -> Result< InstrumentSpec, SpecError >
{
  let tick = Tick::new( tick )?;
  let lot = Lot::new( lot )?;

  Ok( InstrumentSpec { id, base, quote, tick, lot, halted : false } )
}

/// Whether `spec`'s instrument is currently halted.
#[ must_use ]
pub const fn spec_halted_is( spec : &InstrumentSpec ) -> bool
{
  spec.halted
}

/// Snap `price` to `spec`'s tick grid.
///
/// Uses `exact_arith::rounding_default()` (ties-to-even) rather than a
/// direction this crate invents on its own — `exchange_spec` has no side to
/// reason from (a buy and a sell would want opposite directions), so picking
/// `Down` or `Up` here would be an arbitrary bias dressed up as a default.
/// A caller that needs a directional snap applies its own `Rounding` via
/// `exact_arith::price_snap_tick` directly, bypassing this wrapper.
///
/// # Errors
///
/// [`SpecError::Overflow`] if the snapped result leaves the representable
/// range.
///
/// ```rust
/// use exact_arith::{ Price, Quantity };
/// use exchange_id::InstrumentId;
/// use exchange_spec::{ AssetId, price_snap, spec_new };
///
/// let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
/// let snapped = price_snap( &spec, Price::parse( "1.26" ).unwrap() ).unwrap();
/// assert_eq!( snapped, Price::parse( "1.25" ).unwrap() );
/// ```
pub fn price_snap( spec : &InstrumentSpec, price : Price ) -> Result< Price, SpecError >
{
  price_snap_tick( price, spec.tick, rounding_default() )
}

/// Snap `qty` to `spec`'s lot grid. See [`price_snap`] for the rounding choice.
///
/// # Errors
///
/// [`SpecError::Overflow`] if the snapped result leaves the representable
/// range.
pub fn qty_snap( spec : &InstrumentSpec, qty : Quantity ) -> Result< Quantity, SpecError >
{
  qty_snap_lot( qty, spec.lot, rounding_default() )
}


/// Whether `price` lies on `spec`'s tick grid — what `exchange_core` checks
/// before accepting an order.
///
/// A point on the grid snaps to itself under any rounding, so this asks
/// [`price_snap`] rather than redoing the arithmetic. A snap that overflows is
/// off the grid.
///
/// ```rust
/// use exact_arith::{ Price, Quantity };
/// use exchange_id::InstrumentId;
/// use exchange_spec::{ AssetId, price_fits, spec_new };
///
/// let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
/// assert!( price_fits( &spec, Price::parse( "1.25" ).unwrap() ) );
/// assert!( !price_fits( &spec, Price::parse( "1.26" ).unwrap() ) );
/// ```
#[ must_use ]
pub fn price_fits( spec : &InstrumentSpec, price : Price ) -> bool
{
  price_snap( spec, price ) == Ok( price )
}

/// Whether `qty` lies on `spec`'s lot grid. See [`price_fits`].
#[ must_use ]
pub fn qty_fits( spec : &InstrumentSpec, qty : Quantity ) -> bool
{
  qty_snap( spec, qty ) == Ok( qty )
}
