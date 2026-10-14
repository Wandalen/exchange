//! Phase P08 — three successive `seq_next`, monotonic. Golden: `s=1,2,3` then `ok`.

use exchange_seq::{ Sequence, seq_next };

fn main()
{
  let a = seq_next( Sequence::ZERO );
  let b = seq_next( a );
  let c = seq_next( b );

  println!( "s={},{},{}", a.0, b.0, c.0 );
  assert_eq!( ( a.0, b.0, c.0 ), ( 1, 2, 3 ) );
  println!( "ok" );
}
