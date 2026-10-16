//! `exchange_core` behind a one-line text protocol, for the deck in `../index.html`.
//!
//! JS writes a command into the input buffer (`io_input`), calls `io_run`,
//! and reads the JSON answer back from `io_output`. One instrument, four
//! accounts (ids 1..=4), tick 0.01, lot 1.
//!
//! Commands:
//! - `reset c1 a1 c2 a2 c3 a3 c4 a4` — fresh exchange, cash and apples per account
//! - `place acct buy|sell price qty gtc|ioc|fok|post resting|incoming|both`
//! - `cancel acct order_id`
//! - `halt`, `resume`, `state`, `hash`

use std::cell::RefCell;
use std::fmt::Write as _;

use exchange_core::
{
  AccountId, AssetId, CancelCause, Event, EventKind, Exchange, ExchangeError, InboundCmd, InstrumentId, Money, Obligation, Order,
  OrderId, OrderRef, Price, Quantity, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, inbound_flush, inbound_ring, verify,
};

const INSTRUMENT : InstrumentId = InstrumentId( 1 );
const ACCOUNTS : u64 = 4;

struct Sim
{
  exchange : Exchange,
  /// Events already reported to JS.
  seen : usize,
}

thread_local!
{
  static SIM : RefCell< Option< Sim > > = const { RefCell::new( None ) };
  static INPUT : RefCell< Vec< u8 > > = const { RefCell::new( Vec::new() ) };
  static OUTPUT : RefCell< String > = const { RefCell::new( String::new() ) };
}

/// Room for a `len`-byte command; JS writes it at the returned address.
#[ unsafe( no_mangle ) ]
pub extern "C" fn io_input( len : usize ) -> *mut u8
{
  INPUT.with_borrow_mut( | input |
  {
    input.clear();
    input.resize( len, 0 );
    input.as_mut_ptr()
  } )
}

/// Run the command in the input buffer; returns the answer's length in bytes.
#[ unsafe( no_mangle ) ]
pub extern "C" fn io_run() -> usize
{
  let command = INPUT.with_borrow( | input | String::from_utf8_lossy( input ).into_owned() );
  let answer = run( &command );
  OUTPUT.with_borrow_mut( | output |
  {
    *output = answer;
    output.len()
  } )
}

/// The last answer's address.
#[ unsafe( no_mangle ) ]
pub extern "C" fn io_output() -> *const u8
{
  OUTPUT.with_borrow( | output | output.as_ptr() )
}

/// One command in, one JSON answer out.
#[ must_use ]
pub fn run( command : &str ) -> String
{
  let words : Vec< &str > = command.split_whitespace().collect();
  SIM.with_borrow_mut( | sim |
  {
    if words.first() == Some( &"reset" )
    {
      return match reset( &words[ 1.. ] )
      {
        Ok( fresh ) => answer( sim.insert( fresh ), "{\"kind\":\"reset\"}".to_string() ),
        Err( message ) => error( &message ),
      };
    }
    let Some( sim ) = sim.as_mut() else { return error( "reset first" ) };
    let result = match words.as_slice()
    {
      [ "place", acct, side, price, qty, tif, stp ] => place( sim, acct, side, price, qty, tif, stp ),
      [ "cancel", acct, id ] => cancel( sim, acct, id ),
      [ "halt" ] => sim.exchange.halt_set( INSTRUMENT ).map( | () | "{\"kind\":\"halted\"}".to_string() ).map_err( | e | e.to_string() ),
      [ "resume" ] => sim.exchange.halt_clear( INSTRUMENT ).map( | () | "{\"kind\":\"resumed\"}".to_string() ).map_err( | e | e.to_string() ),
      [ "state" ] => Ok( "{\"kind\":\"state\"}".to_string() ),
      [ "hash" ] => Ok( format!( "{{\"kind\":\"hash\",\"hash\":\"0x{:016x}\"}}", fnv1a( format!( "{:?}", sim.exchange.events() ).as_bytes() ) ) ),
      _ => Err( format!( "unknown command: {command}" ) ),
    };
    match result
    {
      Ok( result ) => answer( sim, result ),
      Err( message ) => error( &message ),
    }
  } )
}

fn reset( args : &[ &str ] ) -> Result< Sim, String >
{
  if args.len() != 2 * ACCOUNTS as usize
  {
    return Err( "reset takes cash and apples for four accounts".to_string() );
  }
  let mut exchange = Exchange::new();
  for ( n, pair ) in args.chunks( 2 ).enumerate()
  {
    let cash = Money::parse( pair[ 0 ] ).map_err( | e | format!( "cash: {e:?}" ) )?;
    let apples = Quantity::parse( pair[ 1 ] ).map_err( | e | format!( "apples: {e:?}" ) )?;
    exchange.open_account( AccountId( n as u64 + 1 ), cash, apples ).map_err( | e | e.to_string() )?;
  }
  let tick = Price::parse( "0.01" ).map_err( | e | format!( "{e:?}" ) )?;
  let lot = Quantity::from_int( 1 ).map_err( | e | format!( "{e:?}" ) )?;
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), tick, lot ).map_err( | e | e.to_string() )?;
  Ok( Sim { exchange, seen : 0 } )
}

fn account( word : &str ) -> Result< AccountId, String >
{
  word.parse::< u64 >().ok().filter( | n | ( 1..=ACCOUNTS ).contains( n ) ).map( AccountId ).ok_or_else( || format!( "bad account: {word}" ) )
}

/// Push `cmd` through a fresh one-command ring and apply it.
fn step( sim : &mut Sim, cmd : InboundCmd, policy : SelfMatchPolicy ) -> Result< StepOutcome, String >
{
  let mut ring = inbound_ring( 2 ).map_err( | e | format!( "{e:?}" ) )?;
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  if inbound_flush( &mut producer, &[ cmd ] ) != 1
  {
    return Err( "ring refused the command".to_string() );
  }
  sim.exchange.exchange_step( &mut consumer, policy ).into_iter().next().ok_or_else( || "nothing drained".to_string() )
}

fn place( sim : &mut Sim, acct : &str, side : &str, price : &str, qty : &str, tif : &str, stp : &str ) -> Result< String, String >
{
  let side = match side { "buy" => Side::Buy, "sell" => Side::Sell, _ => return Err( format!( "bad side: {side}" ) ) };
  let tif = match tif
  {
    "gtc" => Tif::Gtc, "ioc" => Tif::Ioc, "fok" => Tif::Fok, "post" => Tif::PostOnly,
    _ => return Err( format!( "bad tif: {tif}" ) ),
  };
  let policy = match stp
  {
    "resting" => SelfMatchPolicy::CancelResting, "incoming" => SelfMatchPolicy::CancelIncoming, "both" => SelfMatchPolicy::CancelBoth,
    _ => return Err( format!( "bad stp: {stp}" ) ),
  };
  let order = Order
  {
    id : OrderId( 0 ), instrument : INSTRUMENT, account : account( acct )?, side,
    price : Price::parse( price ).map_err( | e | format!( "price: {e:?}" ) )?,
    quantity : Quantity::parse( qty ).map_err( | e | format!( "qty: {e:?}" ) )?,
    tif, client : None,
  };
  let resting = Resting { order, remaining : order.quantity, arrival : Sequence::ZERO };
  match step( sim, InboundCmd::Place( resting ), policy )?
  {
    StepOutcome::Placed( Ok( receipt ) ) => Ok( format!
    (
      "{{\"kind\":\"placed\",\"order\":{},\"trades\":{},\"resting\":\"{}\",\"stp\":{},\"tif_dropped\":{}}}",
      receipt.order.0, receipt.trades.len(), receipt.resting, receipt.self_match_cancelled, receipt.tif_dropped,
    ) ),
    StepOutcome::Placed( Err( ExchangeError::Rejected( reason ) ) ) => Ok( format!( "{{\"kind\":\"rejected\",\"reason\":\"{reason:?}\"}}" ) ),
    StepOutcome::Placed( Err( other ) ) => Err( other.to_string() ),
    other => Err( format!( "unexpected outcome: {other:?}" ) ),
  }
}

fn cancel( sim : &mut Sim, acct : &str, id : &str ) -> Result< String, String >
{
  let id = OrderId( id.parse::< u64 >().map_err( | _ | format!( "bad order id: {id}" ) )? );
  let cmd = InboundCmd::Cancel { instrument : INSTRUMENT, account : account( acct )?, order : OrderRef::Id( id ) };
  match step( sim, cmd, SelfMatchPolicy::CancelIncoming )?
  {
    StepOutcome::Cancelled( Ok( qty ) ) => Ok( format!( "{{\"kind\":\"cancelled\",\"qty\":\"{qty}\"}}" ) ),
    StepOutcome::Cancelled( Err( ExchangeError::NotResting( _ ) ) ) => Ok( "{\"kind\":\"not_resting\"}".to_string() ),
    StepOutcome::Cancelled( Err( other ) ) => Err( other.to_string() ),
    other => Err( format!( "unexpected outcome: {other:?}" ) ),
  }
}

fn error( message : &str ) -> String
{
  format!( "{{\"ok\":false,\"error\":\"{}\"}}", message.replace( '\\', "\\\\" ).replace( '"', "\\\"" ) )
}

/// `result`, plus the events it caused and the whole state after it.
fn answer( sim : &mut Sim, result : String ) -> String
{
  let exchange = &sim.exchange;
  let mut out = format!( "{{\"ok\":true,\"result\":{result},\"events\":[" );
  let fresh = &exchange.events()[ sim.seen.. ];
  for ( n, event ) in fresh.iter().enumerate()
  {
    if n > 0 { out.push( ',' ); }
    event_json( &mut out, event );
  }
  sim.seen = exchange.events().len();

  out.push_str( "],\"book\":{" );
  for ( n, ( name, side ) ) in [ ( "sell", Side::Sell ), ( "buy", Side::Buy ) ].into_iter().enumerate()
  {
    if n > 0 { out.push( ',' ); }
    let _ = write!( out, "\"{name}\":[" );
    let mut last : Option< Price > = None;
    for resting in exchange.book().side( INSTRUMENT, side )
    {
      match last
      {
        Some( price ) if price == resting.order.price => out.push( ',' ),
        Some( _ ) => { let _ = write!( out, "]}},{{\"price\":\"{}\",\"orders\":[", resting.order.price ); },
        None => { let _ = write!( out, "{{\"price\":\"{}\",\"orders\":[", resting.order.price ); },
      }
      last = Some( resting.order.price );
      let _ = write!
      (
        out, "{{\"id\":{},\"acct\":{},\"qty\":\"{}\",\"total\":\"{}\",\"tif\":\"{}\"}}",
        resting.order.id.0, resting.order.account.0, resting.remaining, resting.order.quantity, tif_name( resting.order.tif ),
      );
    }
    if last.is_some() { out.push_str( "]}" ); }
    out.push( ']' );
  }

  out.push_str( "},\"accounts\":[" );
  for n in 1..=ACCOUNTS
  {
    if n > 1 { out.push( ',' ); }
    if let Some( account ) = exchange.escrow().account( AccountId( n ) )
    {
      let _ = write!
      (
        out, "{{\"id\":{n},\"cash_free\":\"{}\",\"cash_locked\":\"{}\",\"apples_free\":\"{}\",\"apples_locked\":\"{}\"}}",
        account.cash.available(), account.cash.reserved(), account.asset.available(), account.asset.reserved(),
      );
    }
  }
  let conserved = exchange.postings().ok().and_then( | postings | verify( &postings ).ok() ).is_some_and( | report | report.is_balanced() );
  let halted = exchange.halt_is( INSTRUMENT ).unwrap_or( false );
  let stats = exchange.stats_get();
  let _ = write!
  (
    out, "],\"conserved\":{conserved},\"halted\":{halted},\"stats\":{{\"rests\":{},\"fills\":{},\"rejects\":{},\"cancels\":{}}}}}",
    stats.rests, stats.fills, stats.rejects, stats.cancels,
  );
  out
}

fn tif_name( tif : Tif ) -> &'static str
{
  match tif { Tif::Gtc => "gtc", Tif::Ioc => "ioc", Tif::Fok => "fok", Tif::PostOnly => "post" }
}

fn side_name( side : Side ) -> &'static str
{
  match side { Side::Buy => "buy", Side::Sell => "sell" }
}

fn obligation_json( obligation : Obligation ) -> String
{
  match obligation
  {
    Obligation::Cash( amount ) => format!( "{{\"cash\":\"{amount}\"}}" ),
    Obligation::Asset( quantity ) => format!( "{{\"apples\":\"{quantity}\"}}" ),
  }
}

fn event_json( out : &mut String, event : &Event )
{
  let _ = write!( out, "{{\"seq\":{},\"order\":{},\"acct\":{},", event.sequence.0, event.order.0, event.account.0 );
  let _ = match event.kind
  {
    EventKind::OrderAccepted { side, price, quantity, reserved } => write!
    (
      out, "\"kind\":\"accepted\",\"side\":\"{}\",\"price\":\"{price}\",\"qty\":\"{quantity}\",\"reserved\":{}}}",
      side_name( side ), obligation_json( reserved ),
    ),
    EventKind::OrderRejected { reason } => write!( out, "\"kind\":\"rejected\",\"reason\":\"{reason:?}\"}}" ),
    EventKind::Trade( trade ) => write!
    (
      out, "\"kind\":\"trade\",\"maker\":{},\"maker_acct\":{},\"taker_side\":\"{}\",\"price\":\"{}\",\"qty\":\"{}\"}}",
      trade.maker.0, trade.maker_account.0, side_name( trade.taker_side ), trade.price, trade.quantity,
    ),
    EventKind::OrderCancelled { cause, quantity, released } =>
    {
      let cause = match cause { CancelCause::Request => "request", CancelCause::SelfMatch => "self_match", CancelCause::TimeInForce => "tif" };
      write!( out, "\"kind\":\"cancelled\",\"cause\":\"{cause}\",\"qty\":\"{quantity}\",\"released\":{}}}", obligation_json( released ) )
    },
  };
}

/// FNV-1a over `bytes`, 64-bit — the same hash `presentation_trace` uses.
fn fnv1a( bytes : &[ u8 ] ) -> u64
{
  let mut hash : u64 = 0xcbf2_9ce4_8422_2325;
  for byte in bytes
  {
    hash ^= u64::from( *byte );
    hash = hash.wrapping_mul( 0x0000_0100_0000_01b3 );
  }
  hash
}

#[ cfg( test ) ]
mod tests
{
  use super::run;

  #[ test ]
  fn the_queue_scene_trades_in_price_time_order()
  {
    assert!( run( "reset 0 10 0 5 20 0 2 0" ).starts_with( "{\"ok\":true" ) );
    assert!( run( "place 1 sell 1.00 10 gtc incoming" ).starts_with( "{\"ok\":true" ) );
    assert!( run( "place 2 sell 1.00 5 gtc incoming" ).starts_with( "{\"ok\":true" ) );
    let bob = run( "place 3 buy 1.00 12 gtc incoming" );
    assert!( bob.contains( "\"trades\":2" ), "{bob}" );
    assert!( bob.contains( "\"maker_acct\":1,\"taker_side\":\"buy\",\"price\":\"1\",\"qty\":\"10\"" ), "{bob}" );
    let dave = run( "place 4 buy 1.00 10 gtc incoming" );
    assert!( dave.contains( "\"reason\":\"InsufficientFunds\"" ), "{dave}" );
    assert!( dave.contains( "\"conserved\":true" ), "{dave}" );
  }

  #[ test ]
  fn a_bad_command_answers_an_error()
  {
    assert!( run( "reset 0 0 0 0 0 0 0 0" ).starts_with( "{\"ok\":true" ) );
    assert!( run( "place 9 buy 1 1 gtc incoming" ).starts_with( "{\"ok\":false" ) );
  }
}
