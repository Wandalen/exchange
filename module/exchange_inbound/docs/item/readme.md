# item

The exposed surface of `exchange_inbound`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice already used
across this family; see this family's own `docs/research/` for why a
fuller `item_des.rulebook.md` per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface, including the `ring_factory`/`ring_handle` plumbing types it re-exports so a caller never has to name those crates directly.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `InboundCmd` | enum | `{ Place(Resting), Cancel { instrument: InstrumentId, id: OrderId }, Replace { instrument: InstrumentId, old_id: OrderId, new_resting: Resting } }` |
| `InboundOutcome` | enum | `{ Crossed(Crossing), Cancelled(Option<Resting>), Replaced(Result<Resting, RestReplaceError>) }` |
| `InboundApplyError` | enum | `{ Match(MatchError), Idem(IdemError) }` — added for `exchange_inbound/BUG-003` |
| `inbound_ring` | fn | `(capacity: usize) -> Result<Split<InboundCmd>, BuildError>` |
| `inbound_flush` | fn | `(producer: &mut Producer<'_, InboundCmd>, cmds: impl IntoIterator<Item = InboundCmd>) -> usize` |
| `inbound_overflow_reject` | fn | `(producer: &mut Producer<'_, InboundCmd>, cmd: InboundCmd) -> Result<(), InboundCmd>` |
| `inbound_drain` | fn | `(consumer: &mut Consumer<'_, InboundCmd>) -> Vec<InboundCmd>` |
| `inbound_apply` | fn | `(book: &mut Book, seen: &mut IdSet, policy: SelfMatchPolicy, cmd: InboundCmd) -> Result<InboundOutcome, InboundApplyError>` — `seen` parameter and `InboundApplyError` return type added for `exchange_inbound/BUG-003` |
| `BuildError`, `RingConfig` | enum / struct (re-export) | `ring_factory`'s own types, unchanged |
| `Consumer`, `Drain`, `Ends`, `Producer`, `Split` | struct (re-export) | `ring_handle`'s own types, unchanged |
| `IdemError`, `IdSet` | enum / struct (re-export) | `exchange_idem`'s own types, unchanged — re-exported so a caller never has to name that crate directly just to construct the `seen` argument |

### Differs from the proposal

Verified directly against `core_exchange.txt:652-661` (Prompt 3's exposed-item
list for `exchange_inbound`) and `core_exchange.txt:482-487` (crate 22 in
Prompt 2) — not against the central
`docs/crate/022_exchange_inbound.md`/`docs/exposed_item/022_exchange_inbound_items.md`
summaries, both of which described the earlier, truly-unbuilt state ("zero
`ring_*` dependency exists anywhere in the family"); that description was
stale and has been corrected in place to point here, per this session's "fix
the Design-status line if stale" allowance.

The proposal names `InboundCmd { Place, Cancel, Replace }`, a standalone
`Inbound` type, `inbound_producer` ("the ring's end facing outward"),
`inbound_flush` ("TLS → ring, barrier"), `inbound_drain` ("full order,
`try_*`"), `inbound_apply` ("drain, then rest or match"),
`inbound_overflow_reject`, a unified `InboundError { RingFull, Closed,
Drain }`, and explicitly "does not export `Claim` or `GatingSet`". The real
build:

- **`InboundCmd` matches exactly** — three variants, same names, same roles.
- **No `Inbound` type exists.** The proposal's standalone handle never
  materializes; its role is played by the re-exported `Split`/`Ends` pair
  instead (below).
- **No `inbound_producer` function exists.** The proposal's "ring's end
  facing outward" accessor is instead reached through `inbound_ring`, which
  returns a `Split<InboundCmd>` — `.ends()` then `.split()` (both
  `ring_handle`'s own API, re-exported here) yield the `Producer`/`Consumer`
  pair. One constructor plus the re-exported plumbing types, not a dedicated
  accessor function.
- **`inbound_flush` exists under that exact name but a thinner shape.**
  Per [`../ring_edge/001_exchange_inbound_ring_edge.md`](../../../../docs/ring_edge/001_exchange_inbound_ring_edge.md),
  `ring_tls` is not even a dependency — there is no thread-local staging
  type to flush *from*. `inbound_flush` takes a plain `impl
  IntoIterator<Item = InboundCmd>` instead, a direct forward to
  `Producer::try_push_batch`; staging, if any, is the caller's own concern.
- **`inbound_drain` and `inbound_apply` match by name and role** — thin
  forwards to `Consumer::drain()` and to `exchange_match::cross`/
  `exchange_rest::{rest_cancel, rest_replace}` respectively (see
  [`../../src/lib.rs`](../../src/lib.rs)'s own module doc, "`inbound_apply`'s
  `Place` reaches into `exchange_match`, not just `exchange_rest`").
- **`inbound_overflow_reject` matches by name and role**, returning
  `Result<(), InboundCmd>` (the rejected command itself, handed back) rather
  than a typed error.
- **No `InboundError` type exists at all.** Every failure this crate's API
  can produce is already owned by the crate whose operation produced it:
  `BuildError` (`ring_factory`, `inbound_ring`'s own construction failures),
  `MatchError` (`exchange_match`, threaded through `inbound_apply`'s `Place`
  path), `RestReplaceError` (`exchange_rest`, carried inside
  `InboundOutcome::Replaced`), and a bare `Result<(), InboundCmd>` for
  overflow — not even a `RingFull`-named variant, since the rejected command
  is already the only payload a caller needs. There is no `Closed`/`Drain`
  analogue at all: nothing in this crate's API can report a ring as
  "closed", and `inbound_drain` cannot fail (`Vec`, not `Result`). A single
  `InboundError` wrapping four already-owned error paths would add a
  conversion step and no new information — the same "thin, not the
  proposal's full orchestration" shape `exchange_rest`'s own module doc
  argues for itself ("this crate does **not** consolidate that orchestration
  out of `exchange_core`").
- **"Does not export `Claim`/`GatingSet`" holds.** Neither name appears
  anywhere in `src/lib.rs` or its re-export list.
- **Two real items the proposal never named:** `InboundOutcome` (the actual
  return type carrying `inbound_apply`'s result) and the six ring-plumbing
  re-exports (`BuildError`, `RingConfig`, `Consumer`, `Drain`, `Ends`,
  `Producer`, `Split`) a caller needs to actually drive `inbound_ring`'s own
  return value — see [`../definition/readme.md`](../definition/readme.md)
  for where each is really declared.

No new `docs/decisions/` entry for the `Inbound`/`InboundError` omissions —
the reasoning above is specific enough to state once, here, without an
alternatives-considered ADR.
[`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)
remains the one divergence that genuinely warranted that heavier treatment
(a real alternative design was weighed against this one and rejected).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:482-487` | Crate 22 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:652-661` | Crate `exchange_inbound`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
| `../../../../../ring/ring_handle/src/lib.rs` | `Producer`/`Consumer`/`Ends`/`Split`/`Drain`'s real declarations (confirms no `try_clone`, exactly one pair per `split()`) |
| `../../../../../ring/ring_factory/src/lib.rs` | `Factory::build`/`BuildError`/`RingConfig`'s real declarations |
