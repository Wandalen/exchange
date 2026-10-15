//! Plain id types for instruments, orders and accounts, plus the submitter's
//! own order id.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and no knowledge of books, prices or sides. A dedicated type per
//! identity keeps a raw `u64` from crossing a crate boundary without saying
//! what it identifies.
//!
//! # Not built: `IdError::Zero`
//!
//! The source design rejects a raw id of zero. Every field here is `pub`, so
//! a fallible `*_from_raw` would guard one path while `OrderId( 0 )` bypasses
//! it — the conversions stay infallible instead. See
//! `docs/decisions/001_no_id_error.md`.

/// The identity of an instrument. `exchange_book::Book` keys its storage by it.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct InstrumentId( pub u64 );

/// The identity of an order, fixed at submission and never reassigned.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct OrderId( pub u64 );

/// The submitter's own name for an order — FIX's `ClOrdID`.
///
/// Distinct from [`OrderId`], which the exchange assigns: the submitter picks
/// this one before it knows the [`OrderId`], so a retry can carry the same
/// value and be recognised. Unique per [`AccountId`], not globally.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct ClientOrderId( pub u64 );

/// The identity of a participant — a plain number, never a handle borrowed
/// from an ECS (see `exchange_types`' module documentation).
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct AccountId( pub u64 );

/// `InstrumentId` out of its raw form.
#[ must_use ]
pub const fn instrument_from_raw( raw : u64 ) -> InstrumentId
{
  InstrumentId( raw )
}

/// `InstrumentId` back to its raw form.
#[ must_use ]
pub const fn instrument_raw( id : InstrumentId ) -> u64
{
  id.0
}

/// `OrderId` out of its raw form.
#[ must_use ]
pub const fn order_from_raw( raw : u64 ) -> OrderId
{
  OrderId( raw )
}

/// `OrderId` back to its raw form.
#[ must_use ]
pub const fn order_raw( id : OrderId ) -> u64
{
  id.0
}

/// `ClientOrderId` out of its raw form.
#[ must_use ]
pub const fn client_from_raw( raw : u64 ) -> ClientOrderId
{
  ClientOrderId( raw )
}

/// `ClientOrderId` back to its raw form.
#[ must_use ]
pub const fn client_raw( id : ClientOrderId ) -> u64
{
  id.0
}

/// `AccountId` out of its raw form.
#[ must_use ]
pub const fn account_from_raw( raw : u64 ) -> AccountId
{
  AccountId( raw )
}

/// `AccountId` back to its raw form.
#[ must_use ]
pub const fn account_raw( id : AccountId ) -> u64
{
  id.0
}
