//! Phase P13 — a repeated `OrderId` is refused as a duplicate. Golden: `dup=1` then `ok`.

use exchange_id::OrderId;
use exchange_idem::{ IdSet, idem_insert };

fn main()
{
  let mut seen = IdSet::new();
  idem_insert( &mut seen, OrderId( 1 ) ).unwrap();
  let dup = idem_insert( &mut seen, OrderId( 1 ) ).is_err();

  println!( "dup={}", u8::from( dup ) );
  assert!( dup );
  println!( "ok" );
}
