//! The available/reserved partition — why an order cannot commit what it does
//! not hold.
//!
//! An account's holding of one thing is a **pair**: `available` and
//! `reserved`. Placing an order moves quantity `available → reserved`. Only a
//! fill or a cancel moves quantity out of `reserved`. Their sum changes only
//! at a deposit, which is not on the matching path.
//!
//! # This is a mechanism, not a check
//!
//! The tempting alternative is to check: read `available`, confirm it covers
//! the order, let the order rest. That design is wrong in a way that unit
//! tests do not show. A second order reads the same `available`, finds it
//! sufficient too, and now two resting orders are backed by one balance. The
//! window between the check and whatever eventually writes the balance down is
//! the bug, and it is open for the whole interval.
//!
//! Moving the units at acceptance closes it by construction: the second
//! order's validation reads an `available` that has already lost them. There
//! is no window because there is no interval — the check and the write are one
//! operation.
//!
//! # `reserved` is stored, never derived
//!
//! It would be possible to compute `reserved` by summing an account's resting
//! orders. That would be a second source of truth for one fact, and the two
//! would disagree exactly when a bug had already occurred — the moment the
//! check is least able to notice. So `reserved` is a field, and the sum over
//! resting orders is what a *test* compares it against.
//! [`Escrow::reserved_for`] exposes the stored figure for exactly that
//! comparison.
//!
//! # Four edges, and there is no fifth
//!
//! `available → reserved` on acceptance ([`Escrow::reserve`]);
//! `reserved → available` on cancel ([`Escrow::release`]);
//! `reserved →` the counterparty on a fill ([`Escrow::settle`]);
//! and deposit, which enters the pair from outside ([`Escrow::open`]).
//! In particular there is no path that debits `available` while an order
//! rests, and no path back from `reserved` to `available` that is not a
//! cancel.

use std::collections::BTreeMap;

use exact_arith::{ Money, Price, Quantity };
use exchange_fill::Trade;
use exchange_id::{ AccountId, OrderId };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;
use exchange_types::{ TypeError, notional };

/// The arithmetic a [`Holding`] needs of whatever it holds.
///
/// Exists because two different things are conserved here — currency and the
/// asset — and they are two unrelated concrete types in `exact_arith`. The
/// alternative was to write [`Holding`] twice, which is the same code with two
/// names and two places for a bug to be fixed in one of.
///
/// Deliberately minimal: zero, and the two checked moves. Anything richer
/// would be `exact_arith` re-implemented one crate up.
pub trait Conserved : Copy + Ord + core::fmt::Debug
{
  /// Nothing held.
  const NOTHING : Self;

  /// `self + rhs`, or [`None`] if it does not fit.
  fn checked_plus( self, rhs : Self ) -> Option< Self >;

  /// `self - rhs`, or [`None`] if it would go below zero or does not fit.
  fn checked_minus( self, rhs : Self ) -> Option< Self >;
}

impl Conserved for Money
{
  const NOTHING : Self = Self::ZERO;

  fn checked_plus( self, rhs : Self ) -> Option< Self >
  {
    self.checked_add( rhs ).ok()
  }

  fn checked_minus( self, rhs : Self ) -> Option< Self >
  {
    // A currency holding is never negative, and `Money` is signed, so the
    // guard is here rather than in the type.
    let difference = self.checked_sub( rhs ).ok()?;
    ( difference >= Self::ZERO ).then_some( difference )
  }
}

impl Conserved for Quantity
{
  const NOTHING : Self = Self::ZERO;

  fn checked_plus( self, rhs : Self ) -> Option< Self >
  {
    self.checked_add( rhs ).ok()
  }

  fn checked_minus( self, rhs : Self ) -> Option< Self >
  {
    // `Quantity` refuses to go below zero itself, so this needs no guard —
    // the refusal *is* the error.
    self.checked_sub( rhs ).ok()
  }
}

/// One account's holding of one thing, split into what it can spend and what
/// it has already promised.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Holding< T >
{
  available : T,
  reserved : T,
}

impl< T : Conserved > Holding< T >
{
  /// A holding with `available` spendable and nothing promised.
  pub fn new( available : T ) -> Self
  {
    Self { available, reserved : T::NOTHING }
  }

  /// What can still be committed.
  pub fn available( &self ) -> T
  {
    self.available
  }

  /// What has been committed and not yet settled or returned.
  pub fn reserved( &self ) -> T
  {
    self.reserved
  }

  /// `available + reserved` — invariant across every operation on the
  /// matching path, which is what a conservation test asserts.
  ///
  /// # Errors
  ///
  /// [`EscrowError::Arithmetic`] if the sum does not fit, which no sequence
  /// of edges reaches: `receive` keeps it within the ceiling, and the other
  /// three never grow it.
  pub fn total( &self ) -> Result< T, EscrowError >
  {
    self.available.checked_plus( self.reserved ).ok_or( EscrowError::Arithmetic )
  }

  /// The precondition every edge below shares: an amount moved across the
  /// partition is not negative.
  ///
  /// A negative amount turns each edge's subtraction into an addition and the
  /// reverse, so it runs backwards past guards facing the wrong way.
  /// `Quantity` refuses a negative itself; `Money` is signed, so it is checked
  /// here, once for both.
  ///
  /// Fix(a_negative_obligation_is_refused_rather_than_minting_spendable_currency):
  /// Root cause: `checked_minus` guards the *result* of a subtraction, not
  /// the operand's sign. Reserving `-200` raised `available` and lowered
  /// `reserved`: 200 spendable units appeared, and the total stayed right.
  /// Pitfall: a conservation sum is not a solvency check — value moved
  /// between the two halves leaves the sum unchanged.
  fn movement( amount : T ) -> Result< (), EscrowError >
  {
    if amount < T::NOTHING
    {
      return Err( EscrowError::NegativeAmount );
    }
    Ok( () )
  }

  /// Move `amount` from available to reserved.
  fn reserve( &mut self, amount : T ) -> Result< (), EscrowError >
  {
    Self::movement( amount )?;
    let available = self.available.checked_minus( amount ).ok_or( EscrowError::Insufficient )?;
    let reserved = self.reserved.checked_plus( amount ).ok_or( EscrowError::Arithmetic )?;
    self.available = available;
    self.reserved = reserved;
    Ok( () )
  }

  /// Move `amount` back from reserved to available.
  fn release( &mut self, amount : T ) -> Result< (), EscrowError >
  {
    Self::movement( amount )?;
    let reserved = self.reserved.checked_minus( amount ).ok_or( EscrowError::NotReserved )?;
    let available = self.available.checked_plus( amount ).ok_or( EscrowError::Arithmetic )?;
    self.reserved = reserved;
    self.available = available;
    Ok( () )
  }

  /// Move `amount` out of reserved entirely — it is going to a counterparty.
  fn deliver( &mut self, amount : T ) -> Result< (), EscrowError >
  {
    Self::movement( amount )?;
    self.reserved = self.reserved.checked_minus( amount ).ok_or( EscrowError::NotReserved )?;
    Ok( () )
  }

  /// Add `amount` to available — it arrived from a counterparty or a
  /// deposit. Refused if the holding's total would pass the ceiling.
  ///
  /// Fix(a_holding_past_the_ceiling_could_not_release_its_reservation):
  /// Root cause: only `available + amount` was checked. Proceeds or a top-up
  /// landing beside a reservation carried `available + reserved` past the
  /// ceiling, and from then on `release` could not return the reservation —
  /// a resting order nobody could cancel — and `total` failed.
  /// Pitfall: each half fitting is not the sum fitting.
  fn receive( &mut self, amount : T ) -> Result< (), EscrowError >
  {
    Self::movement( amount )?;
    self.total()?.checked_plus( amount ).ok_or( EscrowError::Arithmetic )?;
    self.available = self.available.checked_plus( amount ).ok_or( EscrowError::Arithmetic )?;
    Ok( () )
  }
}

/// What one participant holds: currency, and the asset being traded.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Account
{
  /// The currency side of the pair.
  pub cash : Holding< Money >,
  /// The asset side of the pair.
  pub asset : Holding< Quantity >,
}

/// Something escrow refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum EscrowError
{
  /// No such account.
  UnknownAccount( AccountId ),
  /// The account holds less than the order commits. The order is refused
  /// whole: there is no partially-backed rest state.
  Insufficient,
  /// A release or delivery naming more than was reserved.
  NotReserved,
  /// No reservation is recorded for this order.
  NoReservation( OrderId ),
  /// This order already holds a reservation. Reserving twice for one order
  /// would leave the first one unreachable and therefore permanently locked.
  AlreadyReserved( OrderId ),
  /// This order's reservation belongs to another account than the one named.
  NotOwner( OrderId ),
  /// A reservation of the wrong shape for what is being settled — cash where
  /// the asset was expected, or the reverse.
  ObligationMismatch,
  /// An edge was asked to move a negative amount across the partition.
  ///
  /// Not a shortfall: a movement in the wrong direction, creating value on
  /// one side of the partition and destroying it on the other. Reachable from
  /// a negative [`Price`], which makes a buy's notional negative.
  /// `exchange_core` refuses one first; a direct caller is refused here.
  NegativeAmount,
  /// A value step did not fit its type.
  Arithmetic,
  /// The order's obligation could not be expressed at all.
  Obligation( TypeError ),
}

impl core::fmt::Display for EscrowError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::UnknownAccount( id ) => write!( f, "account {} is not known to the exchange", id.0 ),
      Self::Insufficient => write!( f, "the account holds less than the order commits" ),
      Self::NotReserved => write!( f, "the amount named is more than was reserved" ),
      Self::NoReservation( id ) => write!( f, "order {} holds no reservation", id.0 ),
      Self::AlreadyReserved( id ) => write!( f, "order {} already holds a reservation", id.0 ),
      Self::NotOwner( id ) => write!( f, "order {} is reserved by another account", id.0 ),
      Self::ObligationMismatch => write!( f, "the reservation is not of the shape this settlement needs" ),
      Self::NegativeAmount => write!( f, "a negative amount is not something an account can move" ),
      Self::Arithmetic => write!( f, "a value step did not fit its type" ),
      Self::Obligation( error ) => write!( f, "the order's obligation is not expressible: {error}" ),
    }
  }
}

impl core::error::Error for EscrowError
{
  // Fix(escrow_error_source_classification_not_exhaustive): was a `match`
  //   ending in `_ => None`, so a future variant wrapping its own source
  //   error (like `Obligation`) would silently report `None` — no
  //   underlying cause — with nothing forcing a second look.
  // Root cause: a trailing wildcard arm answers every variant it was not
  //   told about with the same default, compiling cleanly however many
  //   variants `EscrowError` gains.
  // Pitfall: an error-chain walker (this crate's own callers or a
  //   downstream `anyhow`-style report) would silently lose one link of
  //   context for the new variant instead of failing loudly or being
  //   updated deliberately.
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Obligation( error ) => Some( error ),
      Self::UnknownAccount( _ ) | Self::Insufficient | Self::NotReserved | Self::NoReservation( _ )
      | Self::AlreadyReserved( _ ) | Self::NotOwner( _ ) | Self::ObligationMismatch | Self::NegativeAmount
      | Self::Arithmetic => None,
    }
  }
}

impl From< TypeError > for EscrowError
{
  fn from( error : TypeError ) -> Self
  {
    Self::Obligation( error )
  }
}

/// One live reservation: whose it is, the limit it was reserved at, what it
/// still holds, and how much of its order is still unfilled.
///
/// The entry lives until both reach zero. A buy at price zero holds
/// `Cash( 0 )` from the start, so the amount alone cannot say when the order
/// is done.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
struct Reservation
{
  account : AccountId,
  limit : Price,
  obligation : Obligation,
  unfilled : Quantity,
}

/// Every account's balances, and every live reservation.
///
/// Both maps are ordered, not hashed. Iteration order is part of the
/// observable behaviour here — a conservation sum walked in hash order would
/// still be correct, but any future decision that consulted the walk would
/// differ between runs, and this is a crate where that class of bug is
/// expensive and silent.
#[ derive( Debug, Clone, Default, PartialEq, Eq ) ]
pub struct Escrow
{
  accounts : BTreeMap< AccountId, Account >,
  reservations : BTreeMap< OrderId, Reservation >,
}

impl Escrow
{
  /// An exchange with no participants.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Give `id` a starting balance. If `id` already holds an account, `cash`
  /// and `asset` are added to what is still `available`; `reserved` — the
  /// backing for whatever is still resting on the book — is untouched.
  ///
  /// The deposit edge — the only way into the pair from outside. A second
  /// call never touches `reserved`; that would be a fifth edge.
  ///
  /// # Errors
  ///
  /// [`EscrowError::Arithmetic`] if either holding's total would pass the
  /// ceiling. [`EscrowError::NegativeAmount`] if `cash` is negative.
  ///
  /// Fix(open_wiped_reserved_out_from_under_a_live_reservation):
  /// Root cause: `open` inserted a fresh `Account` over an existing one,
  /// zeroing `reserved` behind a live reservation the ledger still held.
  /// Pitfall: the account's `reserved` and the order ledger are two views of
  /// one fact; replacing an `Account` wholesale splits them.
  pub fn open( &mut self, id : AccountId, cash : Money, asset : Quantity ) -> Result< (), EscrowError >
  {
    match self.accounts.get( &id )
    {
      Some( existing ) =>
      {
        // Fix(exchange_escrow_open_merge_leg_torn_on_partial_failure):
        // Root cause: the two `receive`s ran against the live account, so an
        // asset leg that failed left the cash leg committed under an `Err`.
        // Pitfall: two fallible edges are one unit only if computed on a copy
        // and committed together.
        let mut account = *existing;
        account.cash.receive( cash )?;
        account.asset.receive( asset )?;
        *self.accounts.get_mut( &id ).expect( "looked up above" ) = account;
      },
      None =>
      {
        // Fix(open_accepted_a_negative_first_deposit): a fresh account could
        // start below zero — a holding `checked_minus` promises never exists.
        // Root cause: only the merge branch ran `receive`, the edge that
        // checks the amount's sign.
        // Pitfall: a "nothing to corrupt yet" branch still has to establish
        // the invariant every other edge assumes.
        Holding::movement( cash )?;
        self.accounts.insert( id, Account { cash : Holding::new( cash ), asset : Holding::new( asset ) } );
      },
    }
    Ok( () )
  }

  /// What `id` holds, or [`None`] if it is unknown.
  #[ must_use ]
  pub fn account( &self, id : AccountId ) -> Option< &Account >
  {
    self.accounts.get( &id )
  }

  /// The stored reservation for `order`, if it has one.
  ///
  /// Stored, not derived — see the module documentation. A test comparing
  /// this against the sum of the account's resting obligations is the bridge
  /// between this partition and the coverage property.
  #[ must_use ]
  pub fn reserved_for( &self, order : OrderId ) -> Option< Obligation >
  {
    self.reservations.get( &order ).map( | reservation | reservation.obligation )
  }

  /// How many reservations are live.
  #[ must_use ]
  pub fn reservation_count( &self ) -> usize
  {
    self.reservations.len()
  }

  /// Reserve `order`'s maximum settlement obligation, before it is visible to
  /// matching.
  ///
  /// All-or-nothing: an order that cannot be fully reserved is refused whole
  /// and nothing moves. A partially-backed resting order would be visible on
  /// the book and unable to settle, which is the one thing a book must never
  /// display.
  ///
  /// # Errors
  ///
  /// [`EscrowError::UnknownAccount`], [`EscrowError::Insufficient`] when the
  /// balance does not cover it, [`EscrowError::AlreadyReserved`] on a repeat,
  /// [`EscrowError::NegativeAmount`] when the obligation is negative — which a
  /// buy's is exactly when its price is — or [`EscrowError::Obligation`] when
  /// the obligation is not expressible.
  pub fn reserve( &mut self, order : &Order ) -> Result< Obligation, EscrowError >
  {
    if self.reservations.contains_key( &order.id )
    {
      return Err( EscrowError::AlreadyReserved( order.id ) );
    }

    let obligation = exchange_types::obligation( order )?;
    let account = self.accounts.get_mut( &order.account ).ok_or( EscrowError::UnknownAccount( order.account ) )?;

    match obligation
    {
      Obligation::Cash( amount ) => account.cash.reserve( amount )?,
      Obligation::Asset( quantity ) => account.asset.reserve( quantity )?,
    }

    self.reservations.insert( order.id, Reservation { account : order.account, limit : order.price, obligation, unfilled : order.quantity } );
    Ok( obligation )
  }

  /// Return everything still reserved for `order` to its owner's available
  /// balance, and forget the reservation.
  ///
  /// The cancel edge. A cancel that does not do this leaks the reservation:
  /// funds neither usable by their owner nor settled to a counterparty, which
  /// is economically a deletion and which nothing on the fill path can
  /// discover.
  ///
  /// `owner` must be the account that reserved `order`. The ledger entry is
  /// removed only once the account-level move has succeeded.
  ///
  /// Fix(a_release_naming_another_account_drained_its_reserve):
  /// Root cause: a reservation did not record its owner, so `release` moved
  /// the amount out of whichever account was named. One holding enough
  /// `reserved` of its own absorbed it: the true owner's funds stayed locked
  /// with no entry left to release them, and the named account's own order
  /// could no longer settle or cancel.
  /// Pitfall: the fix below made a wrong account fail only when it held too
  /// little to absorb the move.
  ///
  /// Fix(a_release_deleted_the_ledger_entry_before_the_account_move_could_fail):
  /// Root cause: the entry was removed first, so a release that then failed
  /// returned `Err` with the reservation already orphaned.
  /// Pitfall: a `remove` where a `get` would do mutates whether or not the
  /// rest of the function succeeds.
  ///
  /// # Errors
  ///
  /// [`EscrowError::NoReservation`] if `order` holds none,
  /// [`EscrowError::NotOwner`] if `owner` did not reserve it,
  /// [`EscrowError::UnknownAccount`] if its owner is gone. On any error the
  /// reservation and every account are left exactly as they were.
  pub fn release( &mut self, owner : AccountId, order : OrderId ) -> Result< Obligation, EscrowError >
  {
    let reservation = self.reservations.get( &order ).ok_or( EscrowError::NoReservation( order ) )?;
    if reservation.account != owner
    {
      return Err( EscrowError::NotOwner( order ) );
    }
    let obligation = reservation.obligation;
    let account = self.accounts.get_mut( &owner ).ok_or( EscrowError::UnknownAccount( owner ) )?;

    match obligation
    {
      Obligation::Cash( amount ) => account.cash.release( amount )?,
      Obligation::Asset( quantity ) => account.asset.release( quantity )?,
    }

    self.reservations.remove( &order );
    Ok( obligation )
  }

  /// Move the units `trade` names between its two parties, drawing from their
  /// reservations.
  ///
  /// **A buyer reserves at its own limit and may execute better**, and the
  /// difference goes back to it. The limit is the one the buyer's
  /// reservation recorded: a taker's improvement is returned, a maker
  /// executing at its own price has none. Reserving the maximum is what makes
  /// the order executable; returning the unused part is what stops the
  /// exchange quietly accumulating the spread.
  ///
  /// Every step below runs against local copies of the two accounts and the
  /// two reservations before anything is written back into `self`, so a
  /// fallible step late in the sequence — an asset holding already at its
  /// representable ceiling, say — cannot leave one party already paid out of
  /// a trade that, as a whole, never happened.
  ///
  /// Fix(settle_trusted_the_callers_side_and_limit):
  /// Root cause: the taker's side and limit were arguments, and a buying
  /// maker's limit was taken to be the trade price. A limit above the one
  /// reserved returned more than the fill freed, leaving the rest of the
  /// order under-backed; one below left a filled order's entry behind; a
  /// maker charged above its limit paid from what backed its remainder.
  /// Pitfall: an argument restating what the ledger already recorded is a
  /// second source of truth, and the two only disagree when a caller is
  /// wrong.
  ///
  /// # Errors
  ///
  /// [`EscrowError`] if either party is unknown, holds no reservation, holds
  /// one of the wrong shape, or is not the account that reserved it
  /// ([`EscrowError::NotOwner`]); [`EscrowError::NotReserved`] if the trade's
  /// price is above the buyer's limit. On any error, neither account and
  /// neither reservation is changed.
  pub fn settle( &mut self, trade : &Trade ) -> Result< (), EscrowError >
  {
    let taker = *self.reservations.get( &trade.taker ).ok_or( EscrowError::NoReservation( trade.taker ) )?;
    let maker = *self.reservations.get( &trade.maker ).ok_or( EscrowError::NoReservation( trade.maker ) )?;
    let ( buyer_limit, buyer, seller ) = match trade.taker_side
    {
      Side::Buy => ( taker.limit, trade.taker_account, trade.maker_account ),
      Side::Sell => ( maker.limit, trade.maker_account, trade.taker_account ),
    };

    let paid = notional( trade.price, trade.quantity )?;
    let reserved_for_this_fill = notional( buyer_limit, trade.quantity )?;
    let improvement = reserved_for_this_fill.checked_sub( paid ).map_err( | _ | EscrowError::Arithmetic )?;
    if improvement < Money::ZERO
    {
      // Above the buyer's limit: the fill would spend what backs the rest of
      // its order.
      return Err( EscrowError::NotReserved );
    }

    let taker_reservation = Self::reduced_obligation
    (
      taker,
      trade.taker,
      trade.taker_account,
      trade.taker_side,
      reserved_for_this_fill,
      trade.quantity,
    )?;
    let maker_reservation = Self::reduced_obligation
    (
      maker,
      trade.maker,
      trade.maker_account,
      trade.taker_side.opposite(),
      reserved_for_this_fill,
      trade.quantity,
    )?;

    let mut buyer_account = *self.accounts.get( &buyer ).ok_or( EscrowError::UnknownAccount( buyer ) )?;
    buyer_account.cash.deliver( paid )?;
    buyer_account.cash.release( improvement )?;

    // A self-trade is never produced by the matching engine's own self-match
    // prevention, but `Escrow` is public and defends against direct misuse
    // independently of it. When buyer and seller are the same account, the
    // seller-side steps below continue on the buyer's already-updated copy
    // rather than a second, independent snapshot — two snapshots taken from
    // the same starting state would let whichever commits last silently
    // discard the other's mutation.
    let mut seller_account = if seller == buyer
    {
      buyer_account
    }
    else
    {
      *self.accounts.get( &seller ).ok_or( EscrowError::UnknownAccount( seller ) )?
    };
    seller_account.asset.deliver( trade.quantity )?;
    seller_account.cash.receive( paid )?;

    // Fix(a_self_trade_at_the_ceiling_was_refused):
    // Root cause: the buyer received the asset before the seller delivered
    // it. On a self-trade both ran on one holding, whose total passed the
    // ceiling in between on a trade that nets to zero.
    // Pitfall: a step order harmless across two holdings is not across one.
    let receiving = if seller == buyer { &mut seller_account } else { &mut buyer_account };
    receiving.asset.receive( trade.quantity )?;

    // Every fallible step above succeeded — commit all four at once.
    if seller == buyer
    {
      *self.accounts.get_mut( &buyer ).expect( "looked up above" ) = seller_account;
    }
    else
    {
      *self.accounts.get_mut( &buyer ).expect( "looked up above" ) = buyer_account;
      *self.accounts.get_mut( &seller ).expect( "looked up above" ) = seller_account;
    }
    Self::commit_reservation( &mut self.reservations, trade.taker, taker_reservation );
    Self::commit_reservation( &mut self.reservations, trade.maker, maker_reservation );

    Ok( () )
  }

  /// Sum of every account's `available + reserved` currency.
  ///
  /// The conservation reading: unchanged by every operation on the matching
  /// path, so a test that takes it before and after a trade is asserting that
  /// the exchange moved value rather than created it.
  ///
  /// # Errors
  ///
  /// [`EscrowError::Arithmetic`] if the sum overflows the currency type.
  pub fn total_cash( &self ) -> Result< Money, EscrowError >
  {
    self.accounts.values().try_fold( Money::ZERO, | sum, account |
    {
      let total = account.cash.total()?;
      sum.checked_add( total ).map_err( | _ | EscrowError::Arithmetic )
    } )
  }

  /// Sum of every account's `available + reserved` asset.
  ///
  /// # Errors
  ///
  /// [`EscrowError::Arithmetic`] if the sum overflows the quantity type.
  pub fn total_asset( &self ) -> Result< Quantity, EscrowError >
  {
    self.accounts.values().try_fold( Quantity::ZERO, | sum, account |
    {
      let total = account.asset.total()?;
      sum.checked_add( total ).map_err( | _ | EscrowError::Arithmetic )
    } )
  }

  /// Compute what `order`'s `reservation` becomes after taking out the filled
  /// portion, without committing it — the compute half of `settle`'s
  /// compute-then-commit split. `Ok( None )` means fully discharged: nothing
  /// held and nothing unfilled.
  ///
  /// Uses [`Conserved::checked_minus`] for both shapes uniformly, the same
  /// abstraction every edge in [`Holding`] already uses — not the raw
  /// per-type `checked_sub`, which for `Money` (signed, unlike `Quantity`)
  /// does not itself refuse to go below zero.
  ///
  /// Fix(a_zero_price_buy_lost_its_reservation_on_a_partial_fill):
  /// Root cause: the entry was dropped once the held amount reached zero, and
  /// a buy at price zero holds `Cash( 0 )` from the start — its first partial
  /// fill dropped the entry while the remainder still rested, so it could no
  /// longer be cancelled or settled.
  /// Pitfall: an amount reaching zero is not the order finishing; only the
  /// unfilled quantity says that.
  ///
  /// # Errors
  ///
  /// [`EscrowError::NotOwner`] if `owner` did not reserve it,
  /// [`EscrowError::ObligationMismatch`] if the stored shape does not match
  /// `side`, [`EscrowError::NotReserved`] if the amount or quantity named
  /// exceeds what remains.
  fn reduced_obligation
  (
    reservation : Reservation,
    order : OrderId,
    owner : AccountId,
    side : Side,
    cash_spent : Money,
    asset_delivered : Quantity,
  ) -> Result< Option< Reservation >, EscrowError >
  {
    if reservation.account != owner
    {
      return Err( EscrowError::NotOwner( order ) );
    }

    let ( obligation, drained ) = match ( side, reservation.obligation )
    {
      ( Side::Buy, Obligation::Cash( held ) ) =>
      {
        let remaining = held.checked_minus( cash_spent ).ok_or( EscrowError::NotReserved )?;
        ( Obligation::Cash( remaining ), remaining == Money::ZERO )
      },
      ( Side::Sell, Obligation::Asset( held ) ) =>
      {
        let remaining = held.checked_minus( asset_delivered ).ok_or( EscrowError::NotReserved )?;
        ( Obligation::Asset( remaining ), remaining == Quantity::ZERO )
      },
      _ => return Err( EscrowError::ObligationMismatch ),
    };
    let unfilled = reservation.unfilled.checked_minus( asset_delivered ).ok_or( EscrowError::NotReserved )?;

    Ok( if drained && unfilled == Quantity::ZERO { None } else { Some( Reservation { obligation, unfilled, ..reservation } ) } )
  }

  /// Write a previously-computed reservation update, dropping the record
  /// entirely once nothing is left. The commit half of `settle`'s
  /// compute-then-commit split.
  fn commit_reservation( reservations : &mut BTreeMap< OrderId, Reservation >, order : OrderId, updated : Option< Reservation > )
  {
    match updated
    {
      Some( reservation ) => { reservations.insert( order, reservation ); },
      None => { reservations.remove( &order ); },
    }
  }
}
