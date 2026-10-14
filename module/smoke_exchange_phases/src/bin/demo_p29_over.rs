//! Phase P29 — a full ring surfaces the third publish as an explicit
//! rejection, never a silent drop. Golden: `overflow=1` then `ok`.

use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_inbound::{ inbound_overflow_reject, inbound_ring, InboundCmd, OrderRef };

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn main()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, _consumer ) = ends.split();

  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, account : AccountId( 1 ), order : OrderRef::Id( OrderId( 1 ) ) } ).unwrap();
  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, account : AccountId( 1 ), order : OrderRef::Id( OrderId( 2 ) ) } ).unwrap();

  let third = InboundCmd::Cancel { instrument : INSTRUMENT, account : AccountId( 1 ), order : OrderRef::Id( OrderId( 3 ) ) };
  let overflow = i32::from( inbound_overflow_reject( &mut producer, third ).is_err() );

  println!( "overflow={overflow}" );
  assert_eq!( overflow, 1, "a full ring built with OverflowPolicy::Fail must reject, not silently drop" );
  println!( "ok" );
}
