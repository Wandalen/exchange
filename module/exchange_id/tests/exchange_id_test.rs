//! Test Matrix T01 — id round-trips, and Phase P01's own smoke assertion.

use exchange_id::
{
  AccountId, ClientOrderId, InstrumentId, OrderId,
  account_from_raw, account_raw, client_from_raw, client_raw, instrument_from_raw, instrument_raw, order_from_raw,
  order_raw,
};

/// T01 — every id type survives a raw round-trip.
#[ test ]
fn every_id_survives_a_raw_round_trip()
{
  assert_eq!( instrument_raw( instrument_from_raw( 7 ) ), 7 );
  assert_eq!( order_raw( order_from_raw( 7 ) ), 7 );
  assert_eq!( account_raw( account_from_raw( 7 ) ), 7 );
  assert_eq!( client_raw( client_from_raw( 7 ) ), 7 );
}

/// Direct tuple construction still works — extraction did not remove it.
#[ test ]
fn direct_construction_still_round_trips()
{
  assert_eq!( OrderId( 7 ).0, 7 );
  assert_eq!( AccountId( 7 ).0, 7 );
  assert_eq!( InstrumentId( 7 ).0, 7 );
  assert_eq!( ClientOrderId( 7 ).0, 7 );
}

/// Distinct raw values give distinct, ordered ids.
#[ test ]
fn ids_are_ordered_and_distinct()
{
  assert_ne!( OrderId( 1 ), OrderId( 2 ) );
  assert!( OrderId( 1 ) < OrderId( 2 ) );
}

/// Phase P01 — `InstrumentId`/`OrderId` round-trip raw 7 out and back.
///
/// Golden print: `eq=1` then `ok` (`docs/golden_output/001_p01_golden.md`).
#[ test ]
fn p01_id_round_trip()
{
  let raw = 7_u64;
  let eq = ( instrument_raw( instrument_from_raw( raw ) ) == raw )
    && ( order_raw( order_from_raw( raw ) ) == raw );

  assert!( eq );
}
