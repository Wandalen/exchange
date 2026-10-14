# ADR-003: A cancel names its account, its instrument, and the order by either id; the ring is the only way to cancel

**Date**: 2026-10-14
**Status**: Accepted
**Deciders**: User (chose removing the direct cancel, the same answer for a foreign id as for an unknown one, and cancelling by either id), implementing session
**Supersedes**: ADR-001 point 3, and the direct `Exchange::cancel` that ADR-001 point 1 already ruled out

## Context

Any ring producer could cancel any account's order. `InboundCmd::Cancel`
carried only `{ instrument, id }`, and `exchange_step` ignored `instrument`
and found the order by its sequential id across every book (ADR-001 point
3). A misrouted or buggy cancel withdrew another account's order.

`Exchange::cancel(id)` stayed public from before the ring existed. ADR-001
point 1 said every cancel goes through the ring, yet point 3 kept the method
and had the ring delegate to it. It took no account, so no owner check could
apply, and it did not feed stats. Only tests called it.

A cancel by `OrderId` alone also had a gap: the exchange assigns the id, and
the `Receipt` carrying it goes to whoever calls `exchange_step`, not to the
producer that pushed the order. A producer had no reliable way to name its
own order.

## Decision

1. **The ring is the only way to cancel.** `Exchange::cancel` is removed;
   its body is the private `cancel_resting`. One command through a ring is as
   synchronous as the direct call was.
2. **`InboundCmd::Cancel { instrument, account, order : OrderRef }`.**
   `OrderRef` is `Id(OrderId)`, the id the exchange assigned, or
   `Client(ClientOrderId)`, the account's own id. This is the FIX shape: a
   cancel names `OrderID` or `OrigClOrdID`.
3. **An order is withdrawn only when account, instrument and id all match.**
   `OrderRef::resting_in` is the one lookup; `exchange_step` and
   `exchange_inbound::inbound_apply` both use it. Escrow releases under the
   command's account.
4. **Every miss answers the same.** Never placed, already gone, another
   account's, another instrument's: `ExchangeError::NotResting(OrderRef)`
   from the facade, `InboundOutcome::Cancelled(None)` from `inbound_apply`.
   No event, no stats, nothing moves. A producer cannot learn which ids
   another account has live; FIX answers both cases as "unknown order".

## Alternatives Considered

- **A distinct `NotOwner` error.** Clearer to debug, but it tells a producer
  that an id is live. Producers do not see the event stream, so that is new
  information to them.
- **Keep the direct method, gated by account, or routed through the same
  per-command code.** It served no caller outside tests, and a second entry
  point is a second thing to keep in step.
- **Cancel by `OrderId` only.** Leaves a producer unable to name an order it
  placed without reading the event stream.
- **Cancel by client id only.** An order placed without a client id could
  never be cancelled.

## Consequences

- Breaking: `Exchange::cancel` is gone; `InboundCmd::Cancel` gains
  `account` and replaces `id` with `order`; `ExchangeError::NotResting`
  carries an `OrderRef`.
- Every cancel is counted in stats, which cold recovery's "recovered stats
  equal live stats" needs.
- Against a hostile producer in the same process this buys nothing: it can
  name any account, as it already can on `Place`. It stops misrouted and
  buggy cancels.
- The lookup scans the book, as the old one did. An id index (spec PF5)
  makes it logarithmic.
- A client id names at most one order because an account never reuses one:
  `Exchange` and `inbound_apply`'s `Claims` both keep client ids for good.
- A cancel the venue itself starts (an operator, a halt) does not exist. If
  it is added, it is its own path with its own `CancelCause`, on the record.

## Related

- [`001_submit_replaced_by_ring_fed_exchange_step.md`](001_submit_replaced_by_ring_fed_exchange_step.md): points 1 and 3, superseded here
- `../../tests/cancel_test.rs`: the facade's cancel rules
- `../../../exchange_inbound/tests/exchange_inbound_test.rs`: `inbound_apply_leaves_a_foreign_cancel_untouched`, `inbound_apply_cancels_by_client_id`
