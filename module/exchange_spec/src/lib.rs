//! Tick, lot, the asset pair, and the halt flag on one instrument.
//!
//! A single-tier dependent: `exchange_id` for `InstrumentId`, `exact_kind`
//! for the decimal grid itself, `exact_snap`/`exact_round` for the snap
//! machinery. This crate does not implement snapping —
//! [`price_snap`]/[`qty_snap`] are thin wrappers over `exact_snap`'s own
//! `price_snap_tick`/`qty_snap_lot`, which already refuse a zero-sized grid
//! and already pick the rounding. Reimplementing either here would be a
//! second source of truth for one piece of arithmetic, which is exactly what
//! this family's own module documentation warns against for `Trade`/`Fill`.
//!
//! Before this crate, an illegal price could enter the book with no grid to
//! reject it against, and nothing distinguished one instrument from another —
//! closes hard problems 1 (one book per instrument), 8 (tick and lot), 18
//! (halt), and 19 (more than one asset).

use exact_kind::{ Price, Quantity };
use exact_round::rounding_default;
use exact_snap::{ Lot, SnapError, Tick, price_snap_tick, qty_snap_lot };
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
/// Uses `exact_round::rounding_default()` (ties-to-even) rather than a
/// direction this crate invents on its own — `exchange_spec` has no side to
/// reason from (a buy and a sell would want opposite directions), so picking
/// `Down` or `Up` here would be an arbitrary bias dressed up as a default.
/// A caller that needs a directional snap applies its own `Rounding` via
/// `exact_snap::price_snap_tick` directly, bypassing this wrapper.
///
/// # Errors
///
/// [`SpecError::Overflow`] if the snapped result leaves the representable
/// range.
///
/// ```rust
/// use exact_kind::{ Price, Quantity };
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

