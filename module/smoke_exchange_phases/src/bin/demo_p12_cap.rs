//! Phase P12 — cap 2, third rest refused. Golden: `ok_full` directly.

use exchange_cap::{ BookCaps, CapError, cap_check_rest };

fn main()
{
  let caps = BookCaps { max_rests : 2, max_levels : 2, max_account_rests : 4 };

  assert_eq!( cap_check_rest( caps, 0 ), Ok( () ) );
  assert_eq!( cap_check_rest( caps, 1 ), Ok( () ) );
  assert_eq!( cap_check_rest( caps, 2 ), Err( CapError::RestsFull ) );

  println!( "ok_full" );
}
