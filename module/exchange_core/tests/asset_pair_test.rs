//! v0.1 trades one asset pair.
//!
//! Escrow keeps one cash and one asset holding per account, and
//! [`PostingAsset`](exchange_core::PostingAsset) audits those two balances,
//! not each instrument's assets. A second pair would share both holdings with
//! the first, so the audit could balance while one asset is created and
//! another destroyed. `spec_register` refuses it instead.

use exchange_core::{ AssetId, Exchange, ExchangeError, InstrumentId, Price, Quantity };

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
    assert_eq!( refused, Err( ExchangeError::SecondAssetPair( other ) ), "pair ( {base}, {quote} )" );
    assert_eq!( exchange.halt_is( other ), Err( ExchangeError::UnknownInstrument( other ) ) );
  }
}
