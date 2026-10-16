//! The presentation trace: one fixed scenario run through the real engine,
//! printed as the JavaScript file the slide deck loads.
//!
//! Stdout is exactly `docs/presentation/trace.js`: four scenes, each with its
//! outcome, its trades, the book after it, and every balance; then the
//! conservation audit and a replay hash over two fresh runs. Every value is
//! read from `exchange_core` at run time. The scenario's orders and expected
//! answers are the only literals, and each expected answer is an `assert!`,
//! so a run that exits 0 has checked the trace it printed.
//!
//! Regenerate the deck's data from the repo root:
//!
//! ```text
//! cargo run -q -p exchange_core --example presentation_trace > docs/presentation/trace.js
//! ```

use exchange_core::
{
  Account, AccountId, AssetId, Book, Event, Exchange, ExchangeError, InboundCmd, InstrumentId, Money, Order, OrderId, Price, Quantity,
  Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, Trade, inbound_flush, inbound_ring, verify,
};

const INSTRUMENT : InstrumentId = InstrumentId( 1 );
const ALICE : AccountId = AccountId( 1 );
const CAROL : AccountId = AccountId( 2 );
const BOB : AccountId = AccountId( 3 );
const DAVE : AccountId = AccountId( 4 );

/// The four people, in the order the deck lists their balances.
const PEOPLE : [ ( &str, AccountId ) ; 4 ] = [ ( "Alice", ALICE ), ( "Carol", CAROL ), ( "Bob", BOB ), ( "Dave", DAVE ) ];

/// One scene's order, and the answer the engine must give for it.
struct Plan
{
  id : &'static str,
  who : &'static str,
  side : Side,
  qty : i64,
  price : &'static str,
  /// `rested`, `matched` or `refused`.
  outcome : &'static str,
  /// The reject reason's Debug name, when refused.
  reason : Option< &'static str >,
  /// Each trade in execution order, as `( counterparty, whole units )` at the order's price.
  fills : &'static [ ( &'static str, i64 ) ],
  /// The sell queue after the scene, in time priority, as `( owner, whole units )`.
  sell : &'static [ ( &'static str, i64 ) ],
  /// The buy queue after the scene, in time priority, as `( owner, whole units )`.
  buy : &'static [ ( &'static str, i64 ) ],
}

/// The four scenes, in the order the deck plays them.
const PLAN : [ Plan ; 4 ] =
[
  Plan { id : "alice_sells", who : "Alice", side : Side::Sell, qty : 10, price : "1.00", outcome : "rested", reason : None,
    fills : &[], sell : &[ ( "Alice", 10 ) ], buy : &[] },
  Plan { id : "carol_sells", who : "Carol", side : Side::Sell, qty : 5, price : "1.00", outcome : "rested", reason : None,
    fills : &[], sell : &[ ( "Alice", 10 ), ( "Carol", 5 ) ], buy : &[] },
  Plan { id : "bob_buys", who : "Bob", side : Side::Buy, qty : 12, price : "1.00", outcome : "matched", reason : None,
    fills : &[ ( "Alice", 10 ), ( "Carol", 2 ) ], sell : &[ ( "Carol", 3 ) ], buy : &[] },
  Plan { id : "dave_broke", who : "Dave", side : Side::Buy, qty : 10, price : "1.00", outcome : "refused", reason : Some( "InsufficientFunds" ),
    fills : &[], sell : &[ ( "Carol", 3 ) ], buy : &[] },
];

/// One price level: its price, and the queue behind it in time priority.
struct Level
{
  price : Price,
  queue : Vec< ( AccountId, Quantity ) >,
}

/// What the engine answered for one scene, read back after it ran.
struct Scene
{
  id : &'static str,
  who : &'static str,
  side : Side,
  qty : Quantity,
  price : Price,
  outcome : &'static str,
  reason : Option< String >,
  trades : Vec< Trade >,
  sell : Vec< Level >,
  buy : Vec< Level >,
  balances : [ ( &'static str, Account ) ; 4 ],
}

/// Everything one run of the scenario leaves behind.
struct Run
{
  scenes : Vec< Scene >,
  conserved : bool,
  events : Vec< Event >,
}

fn money( literal : &str ) -> Money
{
  Money::parse( literal ).expect( "the scenario's own amounts are well-formed" )
}

fn price( literal : &str ) -> Price
{
  Price::parse( literal ).expect( "the scenario's own prices are well-formed" )
}

fn units( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).expect( "the scenario's own quantities are in range" )
}

fn account_of( name : &str ) -> AccountId
{
  PEOPLE.iter().find( | person | person.0 == name ).map( | person | person.1 ).expect( "the plan names only the four people" )
}

fn name_of( account : AccountId ) -> &'static str
{
  PEOPLE.iter().find( | person | person.1 == account ).map( | person | person.0 ).expect( "every account in the trace is one of the four people" )
}

/// FNV-1a over `bytes`, 64-bit.
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

/// One side of the book as price levels, best price first. `Book::side`
/// yields each level's orders together in time priority, so a new level starts
/// wherever the price changes.
fn levels( book : &Book, side : Side ) -> Vec< Level >
{
  let mut ladder : Vec< Level > = Vec::new();
  for resting in book.side( INSTRUMENT, side )
  {
    let entry = ( resting.order.account, resting.remaining );
    match ladder.last_mut()
    {
      Some( level ) if level.price == resting.order.price => level.queue.push( entry ),
      _ => ladder.push( Level { price : resting.order.price, queue : vec![ entry ] } ),
    }
  }
  ladder
}

/// Every resting order on one side, in book order.
fn queue_of( ladder : &[ Level ] ) -> Vec< ( AccountId, Quantity ) >
{
  ladder.iter().flat_map( | level | level.queue.iter().copied() ).collect()
}

/// A plan's queue, in the engine's own terms.
fn expected_queue( queue : &[ ( &str, i64 ) ] ) -> Vec< ( AccountId, Quantity ) >
{
  queue.iter().map( | &( name, whole ) | ( account_of( name ), units( whole ) ) ).collect()
}

/// Run the scenario once in a fresh `Exchange`, asserting each scene's answer
/// as it comes back.
fn play() -> Result< Run, Box< dyn core::error::Error > >
{
  let mut exchange = Exchange::new();
  exchange.open_account( ALICE, money( "0.00" ), units( 10 ) )?;
  exchange.open_account( CAROL, money( "0.00" ), units( 5 ) )?;
  exchange.open_account( BOB, money( "20.00" ), Quantity::ZERO )?;
  exchange.open_account( DAVE, money( "2.00" ), Quantity::ZERO )?;
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), price( "0.01" ), units( 1 ) )?;

  let mut ring = inbound_ring( 8 )?;
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let mut scenes = Vec::new();
  for ( n, plan ) in PLAN.iter().enumerate()
  {
    // `exchange_step` assigns the real id itself; these only have to be distinct placeholders.
    let order = Order
    {
      id : OrderId( n as u64 + 1 ), instrument : INSTRUMENT, account : account_of( plan.who ), side : plan.side,
      price : price( plan.price ), quantity : units( plan.qty ), tif : Tif::Gtc, client : None,
    };
    let resting = Resting { order, remaining : order.quantity, arrival : Sequence::ZERO };
    assert_eq!( inbound_flush( &mut producer, &[ InboundCmd::Place( resting ) ] ), 1, "{}: the ring takes the command", plan.id );
    let Some( StepOutcome::Placed( result ) ) = exchange.exchange_step( &mut consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
    else
    {
      panic!( "{}: one Place in, one Placed out", plan.id );
    };

    let ( outcome, reason, trades ) = match result
    {
      Ok( receipt ) if !receipt.trades.is_empty() => ( "matched", None, receipt.trades ),
      Ok( receipt ) => ( "rested", None, receipt.trades ),
      Err( ExchangeError::Rejected( reason ) ) => ( "refused", Some( format!( "{reason:?}" ) ), Vec::new() ),
      Err( other ) => panic!( "{}: an unexpected refusal, {other}", plan.id ),
    };
    assert_eq!( outcome, plan.outcome, "{}: outcome", plan.id );
    assert_eq!( reason.as_deref(), plan.reason, "{}: reject reason", plan.id );

    assert_eq!( trades.len(), plan.fills.len(), "{}: number of trades", plan.id );
    for ( trade, &( counterparty, whole ) ) in trades.iter().zip( plan.fills )
    {
      assert_eq!( trade.taker_account, account_of( plan.who ), "{}: taker", plan.id );
      assert_eq!( trade.maker_account, account_of( counterparty ), "{}: counterparty", plan.id );
      assert_eq!( trade.quantity, units( whole ), "{}: fill quantity", plan.id );
      assert_eq!( trade.price, price( plan.price ), "{}: fill price", plan.id );
    }

    let sell = levels( exchange.book(), Side::Sell );
    let buy = levels( exchange.book(), Side::Buy );
    assert_eq!( queue_of( &sell ), expected_queue( plan.sell ), "{}: sell queue", plan.id );
    assert_eq!( queue_of( &buy ), expected_queue( plan.buy ), "{}: buy queue", plan.id );

    scenes.push( Scene
    {
      id : plan.id, who : plan.who, side : plan.side, qty : order.quantity, price : order.price,
      outcome, reason, trades, sell, buy,
      balances : PEOPLE.map( | ( name, id ) | ( name, *exchange.escrow().account( id ).expect( "every person is opened above" ) ) ),
    } );
  }

  let conserved = verify( &exchange.postings()? )?.is_balanced();
  assert!( conserved, "every trade in the run must net to zero across its legs" );
  Ok( Run { scenes, conserved, events : exchange.event_drain() } )
}

/// FNV-1a over the debug text of one fresh run's events.
fn replay_hash() -> Result< u64, Box< dyn core::error::Error > >
{
  let run = play()?;
  Ok( fnv1a( format!( "{:?}", run.events ).as_bytes() ) )
}

/// A value as a JSON string. Names and numbers here are plain text, so nothing needs escaping.
fn text( value : impl core::fmt::Display ) -> String
{
  format!( "\"{value}\"" )
}

/// Money and price text with two decimals, as the deck reads it. Mirrors `two_decimals` in `smoke_exchange_book`; duplicated, not shared.
fn two_decimals( value : impl core::fmt::Display ) -> String
{
  let rendered = value.to_string();
  let ( whole, frac ) = rendered.split_once( '.' ).unwrap_or( ( rendered.as_str(), "" ) );
  let mut frac = frac.to_string();
  while frac.len() < 2
  {
    frac.push( '0' );
  }
  format!( "{whole}.{frac}" )
}

fn side_name( side : Side ) -> &'static str
{
  match side
  {
    Side::Buy => "buy",
    Side::Sell => "sell",
  }
}

/// Items as a one-line JSON array, `[]` when there are none.
fn array_inline< T >( items : &[ T ], render : fn( &T ) -> String ) -> String
{
  if items.is_empty()
  {
    return "[]".to_string();
  }
  format!( "[ {} ]", items.iter().map( render ).collect::< Vec< _ > >().join( ", " ) )
}

/// One trade, with buyer and seller named by side, as `postings` names them.
fn trade_inline( trade : &Trade ) -> String
{
  let ( buyer, seller ) = match trade.taker_side
  {
    Side::Buy => ( trade.taker_account, trade.maker_account ),
    Side::Sell => ( trade.maker_account, trade.taker_account ),
  };
  format!
  (
    "{{ \"buyer\": {}, \"seller\": {}, \"qty\": {}, \"price\": {} }}",
    text( name_of( buyer ) ), text( name_of( seller ) ), text( trade.quantity ), text( two_decimals( trade.price ) )
  )
}

fn level_inline( level : &Level ) -> String
{
  let queue : Vec< String > = level.queue.iter()
  .map( | ( owner, remaining ) | format!( "{{ \"who\": {}, \"qty\": {} }}", text( name_of( *owner ) ), text( remaining ) ) )
  .collect();
  format!( "{{ \"price\": {}, \"queue\": [ {} ] }}", text( two_decimals( level.price ) ), queue.join( ", " ) )
}

/// One person's balances, as an entry of the deck's `balances` object.
fn balance_line( person : &( &str, Account ) ) -> String
{
  let ( name, account ) = person;
  format!
  (
    "        \"{name}\": {{ \"cash_free\": {}, \"cash_locked\": {}, \"apples_free\": {}, \"apples_locked\": {} }}",
    text( two_decimals( account.cash.available() ) ), text( two_decimals( account.cash.reserved() ) ), text( account.asset.available() ), text( account.asset.reserved() )
  )
}

/// One scene as a JSON object, indented to sit inside the `scenes` array.
fn scene_block( scene : &Scene ) -> String
{
  let reason = scene.reason.as_deref().map_or_else( || "null".to_string(), text );
  let balances : Vec< String > = scene.balances.iter().map( balance_line ).collect();
  [
    "    {".to_string(),
    format!( "      \"id\": {},", text( scene.id ) ),
    format!
    (
      "      \"order\": {{ \"who\": {}, \"side\": {}, \"qty\": {}, \"price\": {} }},",
      text( scene.who ), text( side_name( scene.side ) ), text( scene.qty ), text( two_decimals( scene.price ) )
    ),
    format!( "      \"outcome\": {},", text( scene.outcome ) ),
    format!( "      \"reason\": {reason}," ),
    format!( "      \"trades\": {},", array_inline( &scene.trades, trade_inline ) ),
    "      \"book\": {".to_string(),
    format!( "        \"sell\": {},", array_inline( &scene.sell, level_inline ) ),
    format!( "        \"buy\": {}", array_inline( &scene.buy, level_inline ) ),
    "      },".to_string(),
    "      \"balances\": {".to_string(),
    balances.join( ",\n" ),
    "      }".to_string(),
    "    }".to_string(),
  ].join( "\n" )
}

/// The whole trace, in the shape the deck loads.
fn print_trace( run : &Run, replay_a : u64, replay_b : u64 )
{
  let scenes : Vec< String > = run.scenes.iter().map( scene_block ).collect();
  println!( "// Generated by: cargo run -q -p exchange_core --example presentation_trace > docs/presentation/trace.js" );
  println!( "// Do not edit by hand." );
  println!( "const TRACE = {{" );
  println!( "  \"scenes\": [" );
  println!( "{}", scenes.join( ",\n" ) );
  println!( "  ]," );
  println!( "  \"conserved\": {},", run.conserved );
  println!( "  \"replay\": {{ \"a\": \"0x{replay_a:x}\", \"b\": \"0x{replay_b:x}\" }}" );
  println!( "}};" );
}

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  let trace = play()?;
  let replay_a = replay_hash()?;
  let replay_b = replay_hash()?;
  assert_eq!( replay_a, replay_b, "two fresh runs of the same scenario must hash to the same value" );
  print_trace( &trace, replay_a, replay_b );
  Ok( () )
}
