//! v0.1 trades one asset pair.
//!
//! Escrow keeps one cash and one asset holding per account, and
//! [`PostingAsset`](exchange_core::PostingAsset) audits those two balances,
//! not each instrument's assets. A second pair would share both holdings with
//! the first, so the audit could balance while one asset is created and
//! another destroyed. `spec_register` refuses it instead.

use exchange_core::{ AssetId, Exchange, ExchangeError, InstrumentId, Price, Quantity, SpecError };

fn grid() -> ( Price, Quantity )
{
  ( Price::parse( "0.01" ).unwrap(), Quantity::from_int( 1 ).unwrap() )
}

/// A second instrument on the registered pair is accepted.
#[ test ]
fn a_second_instrument_on_the_same_pair_is_registered()
{
  let mut exchange = Exchange::new();
  let ( tick, lot ) = grid();
  exchange.spec_register( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), tick, lot ).unwrap();

  assert_eq!( exchange.spec_register( InstrumentId( 2 ), AssetId( 1 ), AssetId( 2 ), tick, lot ), Ok( () ) );
}

/// An instrument on another pair is refused, whichever asset differs, and
/// the refusal registers nothing.
#[ test ]
fn an_instrument_on_another_pair_is_refused()
{
  let mut exchange = Exchange::new();
  let ( tick, lot ) = grid();
  exchange.spec_register( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), tick, lot ).unwrap();

  for ( base, quote ) in [ ( 3, 2 ), ( 1, 3 ), ( 2, 1 ) ]
  {
    let other = InstrumentId( 2 );
    let refused = exchange.spec_register( other, AssetId( base ), AssetId( quote ), tick, lot );
    let asked = ( AssetId( base ), AssetId( quote ) );
    assert_eq!( refused, Err( ExchangeError::SecondAssetPair { id : other, held : ( AssetId( 1 ), AssetId( 2 ) ), asked } ) );
    assert_eq!( exchange.halt_is( other ), Err( ExchangeError::UnknownInstrument( other ) ) );
  }
}

/// The refusal names both pairs, so a swapped base and quote reads as one.
#[ test ]
fn a_second_pair_refusal_names_the_held_and_the_asked_pair()
{
  let mut exchange = Exchange::new();
  let ( tick, lot ) = grid();
  exchange.spec_register( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), tick, lot ).unwrap();

  let refused = exchange.spec_register( InstrumentId( 2 ), AssetId( 2 ), AssetId( 1 ), tick, lot ).unwrap_err();

  assert_eq!( refused.to_string(), "instrument 2 trades asset 2 for asset 1; this exchange trades asset 1 for asset 2" );
}

/// An instrument whose base is its quote would audit one asset as two
/// balances, cash and asset; it is refused and registers nothing.
#[ test ]
fn an_instrument_whose_base_is_its_quote_is_refused()
{
  let mut exchange = Exchange::new();
  let ( tick, lot ) = grid();
  let id = InstrumentId( 1 );

  assert_eq!( exchange.spec_register( id, AssetId( 1 ), AssetId( 1 ), tick, lot ), Err( ExchangeError::BaseIsQuote( id ) ) );
  assert_eq!( exchange.halt_is( id ), Err( ExchangeError::UnknownInstrument( id ) ) );
}

/// When several refusals apply, the first in this order wins: the id is
/// already registered, the grid is degenerate, the base is the quote, the
/// pair differs from the one registered.
#[ test ]
fn spec_register_refuses_in_a_fixed_order()
{
  let mut exchange = Exchange::new();
  let ( tick, lot ) = grid();
  let held = InstrumentId( 1 );
  exchange.spec_register( held, AssetId( 1 ), AssetId( 2 ), tick, lot ).unwrap();
  let other = InstrumentId( 2 );

  let duplicate = exchange.spec_register( held, AssetId( 3 ), AssetId( 3 ), Price::ZERO, lot );
  assert_eq!( duplicate, Err( ExchangeError::SpecAlreadyRegistered( held ) ) );

  let degenerate = exchange.spec_register( other, AssetId( 3 ), AssetId( 3 ), Price::ZERO, lot );
  assert_eq!( degenerate, Err( ExchangeError::Spec( SpecError::ZeroTick ) ) );

  let same_asset = exchange.spec_register( other, AssetId( 3 ), AssetId( 3 ), tick, lot );
  assert_eq!( same_asset, Err( ExchangeError::BaseIsQuote( other ) ) );
}
