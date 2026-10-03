//! Phase P04 — three self-trade policies, distinct. Golden: `n=3` then `ok`.

use exchange_stp::SelfMatchPolicy;

fn main()
{
  let policies =
  [ SelfMatchPolicy::CancelResting, SelfMatchPolicy::CancelIncoming, SelfMatchPolicy::CancelBoth ];

  let mut n = 0;
  for ( i, a ) in policies.iter().enumerate()
  {
    if policies[ ..i ].iter().all( | b | b != a )
    {
      n += 1;
    }
  }

  println!( "n={n}" );
  assert_eq!( n, 3 );
  println!( "ok" );
}
