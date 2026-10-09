//! Test Matrix T13 — a repeat id is refused; a forgotten id is resubmittable.

use exchange_id::OrderId;
use exchange_idem::{ IdSet, IdemError, idem_insert, idem_remove, idem_seen };

/// A fresh set has seen nothing.
#[ test ]
fn a_new_set_has_seen_nothing()
{
  let set = IdSet::new();
  assert!( !idem_seen( &set, OrderId( 1 ) ) );
}

/// T13 — the first insert of an id succeeds, and it is then seen.
#[ test ]
fn the_first_insert_of_an_id_succeeds()
{
  let mut set = IdSet::new();
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );
  assert!( idem_seen( &set, OrderId( 1 ) ) );
}

/// T13 — a second insert of the same id is refused, not silently accepted.
///
/// This is the whole point of the crate: a retry that resubmits the same id
/// must not double-rest the order.
#[ test ]
fn p13_a_repeat_insert_is_refused_as_a_duplicate()
{
  let mut set = IdSet::new();
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Err( IdemError::Duplicate ) );
}

/// Two distinct ids never interfere with each other.
#[ test ]
fn distinct_ids_are_independent()
{
  let mut set = IdSet::new();
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );
  assert_eq!( idem_insert( &mut set, OrderId( 2 ) ), Ok( () ) );
  assert!( idem_seen( &set, OrderId( 1 ) ) );
  assert!( idem_seen( &set, OrderId( 2 ) ) );
}

/// Removing an id forgets it, and reports that it was there.
#[ test ]
fn removing_a_seen_id_forgets_it_and_reports_true()
{
  let mut set = IdSet::new();
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );

  assert!( idem_remove( &mut set, OrderId( 1 ) ) );
  assert!( !idem_seen( &set, OrderId( 1 ) ) );
}

/// Removing an id that was never there is a defined outcome, not a panic —
/// the same "race result, not a caller error" shape `exchange_book::cancel`
/// already establishes for an absent id.
#[ test ]
fn removing_an_absent_id_reports_false_rather_than_panicking()
{
  let mut set = IdSet::new();
  assert!( !idem_remove( &mut set, OrderId( 99 ) ) );
}

/// Forgetting an id makes it resubmittable — a cancel-then-resubmit under the
/// same id is not mistaken for the retry this crate exists to refuse.
#[ test ]
fn a_forgotten_id_can_be_inserted_again()
{
  let mut set = IdSet::new();
  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );
  assert!( idem_remove( &mut set, OrderId( 1 ) ) );

  assert_eq!( idem_insert( &mut set, OrderId( 1 ) ), Ok( () ) );
}

/// A non-default key works the same way — here, an account's own client id.
#[ test ]
fn a_composite_key_is_refused_on_repeat_and_scoped_by_its_parts()
{
  use exchange_id::{ AccountId, ClientOrderId };

  let mut set : IdSet< ( AccountId, ClientOrderId ) > = IdSet::new();
  assert_eq!( idem_insert( &mut set, ( AccountId( 1 ), ClientOrderId( 7 ) ) ), Ok( () ) );
  assert_eq!( idem_insert( &mut set, ( AccountId( 1 ), ClientOrderId( 7 ) ) ), Err( IdemError::Duplicate ) );
  assert_eq!( idem_insert( &mut set, ( AccountId( 2 ), ClientOrderId( 7 ) ) ), Ok( () ), "another account may reuse the value" );
}
