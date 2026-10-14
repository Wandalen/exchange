# item

The exposed surface of `exchange_inbound`, as built.

### Scope

- **Purpose**: What this crate exports, and how it compares to the source design's exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface, including the `ring_factory` and `ring_handle` types it re-exports.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `InboundCmd` | enum | `{ Place(Resting), Cancel { instrument: InstrumentId, account: AccountId, order: OrderRef }, Replace { instrument: InstrumentId, old_id: OrderId, new_resting: Resting } }` |
| `OrderRef` | enum | `{ Id(OrderId), Client(ClientOrderId) }`; `fn resting_in(self, &Book, InstrumentId, AccountId) -> Option<&Resting>`; `Display`, `From<OrderId>`, `From<ClientOrderId>` |
| `InboundOutcome` | enum | `{ Crossed(Crossing), Cancelled(Option<Resting>), Replaced(Result<Resting, RestReplaceError>) }` |
| `InboundApplyError` | enum | `{ Match(MatchError), OrderIdRests, DuplicateClientId, ZeroQuantity, NotNew }`, with `Display`, `Error`, `From<MatchError>` |
| `Claims` | struct | `{ /* private */ }`, `Claims::new()` — the `( AccountId, ClientOrderId )`s `inbound_apply` has accepted |
| `inbound_ring` | fn | `(capacity: usize) -> Result<Split<InboundCmd>, BuildError>` |
| `inbound_flush` | fn | `(producer: &mut Producer<'_, InboundCmd>, cmds: &[InboundCmd]) -> usize` — `#[must_use]`; publishes a prefix, `cmds[n..]` stays the caller's |
| `inbound_overflow_reject` | fn | `(producer: &mut Producer<'_, InboundCmd>, cmd: InboundCmd) -> Result<(), InboundCmd>` |
| `inbound_drain` | fn | `(consumer: &mut Consumer<'_, InboundCmd>) -> Vec<InboundCmd>` |
| `inbound_apply` | fn | `(book: &mut Book, claims: &mut Claims, policy: SelfMatchPolicy, cmd: InboundCmd) -> Result<InboundOutcome, InboundApplyError>` |
| `BuildError`, `RingConfig` | re-export | `ring_factory`'s |
| `Consumer`, `Drain`, `Ends`, `Producer`, `Split` | re-export | `ring_handle`'s |

### Differs from the proposal

The source design (`core_exchange.txt:482-487`, `652-661`) names
`InboundCmd { Place, Cancel, Replace }`, an `Inbound` type,
`inbound_producer`, `inbound_flush` ("TLS → ring, barrier"),
`inbound_drain`, `inbound_apply`, `inbound_overflow_reject` and
`InboundError { RingFull, Closed, Drain }`, and exports no `Claim` or
`GatingSet`. The build:

- **`InboundCmd`, `inbound_drain`, `inbound_apply` and
  `inbound_overflow_reject`** match by name and role. The last hands the
  refused command back instead of a typed error.
- **No `Inbound` or `inbound_producer`.** `inbound_ring` returns a
  `Split<InboundCmd>`; `.ends().split()` gives the producer and consumer.
- **`inbound_flush` takes a slice.** `ring_tls` is not a dependency, so
  there is no thread-local stage to flush from; it forwards to
  `Producer::try_push_batch`, and what does not fit stays in the caller's
  slice.
- **No `InboundError`.** Each failure belongs to the crate that produces it:
  `BuildError`, `MatchError`, `RestReplaceError` inside
  `InboundOutcome::Replaced`, the refused command on overflow. Nothing can
  report a ring closed, and a drain cannot fail. `InboundApplyError` holds
  only what `inbound_apply` adds: the cross refusal, a resting order id, a
  used client id and the order checks.
- **`InboundOutcome`, `Claims` and the re-exports** are not in the proposal.
  No `Claim` or `GatingSet` is exported.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:482-487` | Crate 22 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:652-661` | Crate `exchange_inbound`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The built shape this table describes |
