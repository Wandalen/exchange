//! Phase P02 — opposite of bid is ask. Golden: `opp=ask` then `ok`.

use exchange_side::{ Side, side_is_ask, side_opposite };

fn main()
{
  let opposite = side_opposite( Side::Buy );
  let label = if side_is_ask( opposite ) { "ask" } else { "bid" };

  println!( "opp={label}" );
  assert!( side_is_ask( opposite ) );
  println!( "ok" );
}
