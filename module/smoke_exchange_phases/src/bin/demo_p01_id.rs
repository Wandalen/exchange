//! Phase P01 — `InstrumentId`/`OrderId` round-trip. Golden: `eq=1` then `ok`.

use exchange_id::{ instrument_from_raw, instrument_raw, order_from_raw, order_raw };

fn main()
{
  let raw = 7_u64;
  let eq = ( instrument_raw( instrument_from_raw( raw ) ) == raw )
    && ( order_raw( order_from_raw( raw ) ) == raw );

  println!( "eq={}", i32::from( eq ) );
  assert!( eq );
  println!( "ok" );
}
