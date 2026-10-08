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

use exact_kind::{ Money, Quantity };
use exchange_fill::Trade;
use exchange_id::{ AccountId, OrderId };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;
use exchange_types::{ Price, TypeError, notional };

/// The arithmetic a [`Holding`] needs of whatever it holds.
///
/// Exists because two different things are conserved here — currency and the
/// asset — and they are two unrelated concrete types in `exact_kind`. The
/// alternative was to write [`Holding`] twice, which is the same code with two
/// names and two places for a bug to be fixed in one of.
///
/// Deliberately minimal: zero, and the two checked moves. Anything richer
/// would be `exact_kind` re-implemented one crate up.
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
  /// [`EscrowError::Arithmetic`] if the sum does not fit, which can only
  /// happen if the two sides were built by something other than this type.
  pub fn total( &self ) -> Result< T, EscrowError >
  {
    self.available.checked_plus( self.reserved ).ok_or( EscrowError::Arithmetic )
  }

  /// The precondition every edge below shares: an amount moved across the
  /// partition is not negative.
  ///
  /// Unstated until it was violated. The four edges are each written as one
  /// checked subtraction and one checked addition, and their correctness rests
  /// entirely on `amount >= NOTHING` — a negative amount turns every
  /// subtraction into an addition and every addition into a subtraction, so
  /// each edge quietly runs backwards and the guards face the wrong way. For
  /// `Quantity` the precondition holds for free, because the type refuses a
  /// negative in its own constructor; `Money` is signed, so here it is
  /// checked. Expressed against `NOTHING` and `Ord`, which [`Conserved`]
  /// already requires, so it stays one rule for both.
  ///
  /// Fix(a_negative_obligation_is_refused_rather_than_minting_spendable_currency):
  /// Root cause: `Money::checked_minus`'s `difference >= ZERO` guard
  /// constrains the *result* of the subtraction, not the sign of the operand.
  /// Given `-200`, `reserve` computed `available - ( -200 )`, which is larger
  /// and trivially over the floor, then `reserved + ( -200 )`, which had no
  /// floor at all — so 200 units of spendable currency appeared and the
  /// account's `available + reserved` total stayed correct to the minor unit.
  ///
  /// Pitfall: a conservation sum is not a solvency check. Any defect that
  /// moves value between the two halves of a partition — including one moving
  /// it the wrong way — leaves the sum invariant, so the sum can never see it.
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

  /// Add `amount` to available — it arrived from a counterparty.
  fn receive( &mut self, amount : T ) -> Result< (), EscrowError >
  {
    Self::movement( amount )?;
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
  /// A reservation of the wrong shape for what is being settled — cash where
  /// the asset was expected, or the reverse.
  ObligationMismatch,
  /// An edge was asked to move a negative amount across the partition.
  ///
  /// Not an exhaustion error and not a shortfall: the account may hold plenty.
  /// A negative movement is a movement in the wrong direction wearing the
  /// right name, and every one of them creates value on one side of the
  /// partition and destroys it on the other. Reachable from a negative
  /// [`Price`], which is what makes a buy's notional negative — refused here
  /// as well as at each `submit`, because [`Escrow`] is public and a caller
  /// that reaches it directly gets no validation step of its own.
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
      | Self::AlreadyReserved( _ ) | Self::ObligationMismatch | Self::NegativeAmount | Self::Arithmetic => None,
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
  reservations : BTreeMap< OrderId, Obligation >,
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
  /// The deposit edge — the only way the sum of a pair changes. Because it
  /// is documented as one of exactly four edges that ever move value, and
  /// the other three ([`Escrow::reserve`], [`Escrow::release`],
  /// [`Escrow::settle`]) never write `reserved` from outside its own
  /// checked move, a second call for the same `id` must not either — doing
  /// so would be a fifth, undocumented edge.
  ///
  /// # Errors
  ///
  /// [`EscrowError::Arithmetic`] if adding to an existing balance overflows
  /// either side. [`EscrowError::NegativeAmount`] if `cash` is negative —
  /// reachable only on this merge path, since a fresh account's own first
  /// deposit has no prior balance for a negative amount to corrupt.
  ///
  /// Fix(open_wiped_reserved_out_from_under_a_live_reservation): Root cause:
  /// `open` always ran `self.accounts.insert(id, Account { cash :
  /// Holding::new(cash), .. })`, and `BTreeMap::insert` on an existing key
  /// discards the old value outright — `Holding::new` sets `reserved` to
  /// `NOTHING`, so a second `open` for an `id` with a resting order's
  /// reservation zeroed that reservation's backing while
  /// `self.reservations` kept the entry pointing at it, unchanged.
  ///
  /// Pitfall: the account-level `reserved` field and the order-indexed
  /// `self.reservations` map are two views of the same fact (see this
  /// module's own "`reserved` is stored, never derived" section) and
  /// nothing keeps them in sync automatically — any edge that replaces an
  /// `Account` wholesale, rather than moving a checked amount across its
  /// existing `Holding`s, silently reintroduces the exact divergence that
  /// design was meant to make impossible.
  pub fn open( &mut self, id : AccountId, cash : Money, asset : Quantity ) -> Result< (), EscrowError >
  {
    match self.accounts.get( &id )
    {
      Some( existing ) =>
      {
        // Fix(exchange_escrow_open_merge_leg_torn_on_partial_failure): the
        // merge branch used to run `account.cash.receive(cash)?` and
        // `account.asset.receive(asset)?` as two statements against a live
        // `&mut` into `self.accounts` — a failure on the asset leg still left
        // the cash leg's already-successful `receive` committed, so a caller
        // saw `open` return `Err` while the account's cash balance had
        // already changed underneath it.
        // Root cause: `receive` writes `self.available` before returning
        // `Ok`, so sequencing two of them through `?` commits the first the
        // moment it succeeds, regardless of whether the second one goes on to
        // fail — nothing here made the two writes one atomic unit.
        // Pitfall: a merge of two independent fallible edges on the same
        // struct must compute both against a local copy and commit them
        // together — mutate-then-`?` commits eagerly no matter how many more
        // fallible steps follow, the same shape `settle` and `release` were
        // hardened against earlier in this file.
        let mut account = *existing;
        account.cash.receive( cash )?;
        account.asset.receive( asset )?;
        *self.accounts.get_mut( &id ).expect( "looked up above" ) = account;
      },
      None =>
      {
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
    self.reservations.get( &order ).copied()
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

    self.reservations.insert( order.id, obligation );
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
  /// The ledger entry is removed only once the account-level move has already
  /// succeeded — never before. `Escrow` is public and `owner` is supplied by
  /// the caller rather than derived from `order` itself, so a caller error
  /// naming the wrong account must fail without destroying the *correct*
  /// account's reservation record on its way to failing.
  ///
  /// Fix(a_release_deleted_the_ledger_entry_before_the_account_move_could_fail):
  /// Root cause: the ledger entry was removed via `self.reservations.remove`
  /// as the first statement, unconditionally — a `BTreeMap::remove` mutates
  /// on the `Some` path regardless of what the rest of the function goes on
  /// to do. Every following step could still fail (`UnknownAccount`, or the
  /// account-level `Holding::release` itself), and on any of those failures
  /// the function returned `Err` with the entry already gone: the true
  /// owner's reservation was orphaned — nothing else ever reads or re-derives
  /// it — and if the wrongly-named account happened to exist with enough of
  /// its *own*, unrelated `reserved` pool to absorb the move, that account's
  /// partition was silently corrupted too, with no error at all.
  ///
  /// Pitfall: an early, unconditional mutation of a lookup structure — a
  /// `remove` used where a `get` would do — is invisible in a diff and in a
  /// short function; only tracing which of the following lines can still
  /// return `Err` reveals that the removal already happened by then.
  ///
  /// # Errors
  ///
  /// [`EscrowError::NoReservation`] if `order` holds none,
  /// [`EscrowError::UnknownAccount`] if its owner is gone. On either error, or
  /// on the account-level move itself failing, the reservation is left
  /// exactly as it was.
  pub fn release( &mut self, owner : AccountId, order : OrderId ) -> Result< Obligation, EscrowError >
  {
    let obligation = self.reservations.get( &order ).copied().ok_or( EscrowError::NoReservation( order ) )?;
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
  /// `taker_side` says which side aggressed, and `taker_limit` is the price
  /// the aggressor reserved at. The two are needed together for one reason:
  /// **a buyer reserves at its own limit and may execute better**, and the
  /// difference has to go back. Reserving the maximum is what makes the order
  /// executable; returning the unused part is what stops the exchange
  /// quietly accumulating the spread.
  ///
  /// Every step below runs against local copies of the two accounts and the
  /// two reservations before anything is written back into `self`. `Account`
  /// and `Obligation` are both `Copy`, so this costs nothing beyond the copy
  /// itself, and it is what stops a fallible step late in the sequence — an
  /// asset holding already at its representable ceiling, say — from leaving
  /// one party already paid out of a trade that, as a whole, never happened.
  /// See this function's own regression test.
  ///
  /// # Errors
  ///
  /// [`EscrowError`] if either party is unknown, holds no reservation, or
  /// holds one of the wrong shape. On any error, neither account and neither
  /// reservation is changed.
  pub fn settle( &mut self, trade : &Trade, taker_side : Side, taker_limit : Price ) -> Result< (), EscrowError >
  {
    let ( buyer, seller, buyer_limit ) = match taker_side
    {
      Side::Buy => ( trade.taker_account, trade.maker_account, taker_limit ),
      // The maker is the buyer, and a maker always executes at its own price,
      // so there is nothing to give back.
      Side::Sell => ( trade.maker_account, trade.taker_account, trade.price ),
    };

    let paid = notional( trade.price, trade.quantity )?;
    let reserved_for_this_fill = notional( buyer_limit, trade.quantity )?;
    let improvement = reserved_for_this_fill.checked_sub( paid ).map_err( | _ | EscrowError::Arithmetic )?;

    let mut buyer_account = *self.accounts.get( &buyer ).ok_or( EscrowError::UnknownAccount( buyer ) )?;
    buyer_account.cash.deliver( paid )?;
    buyer_account.cash.release( improvement )?;
    buyer_account.asset.receive( trade.quantity )?;

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

    let taker_obligation = Self::reduced_obligation
    (
      self.reservations.get( &trade.taker ).copied(),
      trade.taker,
      taker_side,
      reserved_for_this_fill,
      trade.quantity,
    )?;
    let maker_obligation = Self::reduced_obligation
    (
      self.reservations.get( &trade.maker ).copied(),
      trade.maker,
      taker_side.opposite(),
      paid,
      trade.quantity,
    )?;

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
    Self::commit_reservation( &mut self.reservations, trade.taker, taker_obligation );
    Self::commit_reservation( &mut self.reservations, trade.maker, maker_obligation );

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

  /// Compute what `order`'s reservation becomes after taking out the filled
  /// portion, without committing it — the compute half of `settle`'s
  /// compute-then-commit split. `Ok( None )` means fully discharged.
  ///
  /// Uses [`Conserved::checked_minus`] for both shapes uniformly, the same
  /// abstraction every edge in [`Holding`] already uses — not the raw
  /// per-type `checked_sub`, which for `Money` (signed, unlike `Quantity`)
  /// does not itself refuse to go below zero.
  ///
  /// # Errors
  ///
  /// [`EscrowError::NoReservation`] if `order` holds none,
  /// [`EscrowError::ObligationMismatch`] if the stored shape does not match
  /// `side`, [`EscrowError::NotReserved`] if the amount named exceeds what
  /// remains.
  fn reduced_obligation
  (
    current : Option< Obligation >,
    order : OrderId,
    side : Side,
    cash_spent : Money,
    asset_delivered : Quantity,
  ) -> Result< Option< Obligation >, EscrowError >
  {
    let obligation = current.ok_or( EscrowError::NoReservation( order ) )?;

    match ( side, obligation )
    {
      ( Side::Buy, Obligation::Cash( held ) ) =>
      {
        let remaining = held.checked_minus( cash_spent ).ok_or( EscrowError::NotReserved )?;
        Ok( if remaining == Money::ZERO { None } else { Some( Obligation::Cash( remaining ) ) } )
      },
      ( Side::Sell, Obligation::Asset( held ) ) =>
      {
        let remaining = held.checked_minus( asset_delivered ).ok_or( EscrowError::NotReserved )?;
        Ok( if remaining == Quantity::ZERO { None } else { Some( Obligation::Asset( remaining ) ) } )
      },
      _ => Err( EscrowError::ObligationMismatch ),
    }
  }

  /// Write a previously-computed reservation update, dropping the record
  /// entirely once nothing is left. The commit half of `settle`'s
  /// compute-then-commit split.
  fn commit_reservation( reservations : &mut BTreeMap< OrderId, Obligation >, order : OrderId, updated : Option< Obligation > )
  {
    match updated
    {
      Some( obligation ) => { reservations.insert( order, obligation ); },
      None => { reservations.remove( &order ); },
    }
  }
}
