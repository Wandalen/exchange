# Doc Definitions

## Master Doc Definitions Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `algorithm/` | The decided matching semantics — validate, reserve, match, split, dispose, emit — plus self-match, cancel/amend, and fee assessment | [algorithm/readme.md](../algorithm/readme.md) | 4 |
| `feature/` | v0.1 scope — order semantics committed, hosting ruled, exit criteria fixed | [feature/readme.md](../feature/readme.md) | 1 |
| `invariant/` | Properties the book and ledger must never violate | [invariant/readme.md](../invariant/readme.md) | 3 |
| `pitfall/` | Traps whose failures leave every order-level check passing | [pitfall/readme.md](../pitfall/readme.md) | 1 |
| `protocol/` | The emitted event stream's message contract, and how it may evolve | [protocol/readme.md](../protocol/readme.md) | 1 |
| `state_machine/` | Lifecycles this crate owns — the states one order passes through | [state_machine/readme.md](../state_machine/readme.md) | 1 |

## Master Doc Instances Table

| Definition | ID | Name | File |
|------------|----|------|------|
| algorithm | 001 | Price-Time Priority Matching | [algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) |
| algorithm | 002 | Self-Match Prevention | [algorithm/002_self_match_prevention.md](../algorithm/002_self_match_prevention.md) |
| algorithm | 003 | Cancel and Amend Semantics | [algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) |
| algorithm | 004 | Fee Assessment Point | [algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) |
| feature | 001 | Exchange Core v0.1 | [feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) |
| invariant | 001 | Escrow Covers Resting Orders | [invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) |
| invariant | 002 | Arrival Sequence Is Total and Replayable | [invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) |
| invariant | 003 | Available and Reserved Balance Separation | [invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) |
| pitfall | 001 | Balance Lost Update Under Concurrent Settlement | [pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) |
| protocol | 001 | Trade Event Stream | [protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) |
| state_machine | 001 | Order Lifecycle | [state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) |

Six definitions, eleven instances. `decisions/`, `item/`, and `workaround/`
are directories under `docs/` but are not doc definitions in this table's
sense, and are indexed in [`../readme.md`](../readme.md) rather than here —
`item/` specifically is a single consolidated index (lighter-pass style, not
a numbered-instance collection), covered by its own Module Index immediately
below rather than by a row in either table above.

### Module Index

This crate's own `pub` surface, indexed against
[`../item/readme.md`](../item/readme.md) the same way `exchange_id` and
`exchange_escrow` index theirs:

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Receipt` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `ExchangeError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `StepOutcome` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `PostingAsset` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `postings` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Exchange` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Everything else `exchange_core` exports is a re-export from the thirteen
crates it now faces (`exchange_types`, `exchange_book`, `exchange_match`,
`exchange_escrow`, `exchange_id`, `exchange_depth`, `exchange_halt`,
`exchange_inbound`, `exchange_snap`, `exchange_spec`, `exchange_stats`,
`exchange_tif`, `exact_arith`) — see [`../item/readme.md`](../item/readme.md)'s
own Re-exported surface table rather than duplicating it here.

No `format/` directory exists. The 2026-08-30 implementation chose a sorted
`Vec` per side (→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)),
but a chosen structure is not a frozen layout: it is reversible behind
`insert`/`best`/`consume_best`, and nothing outside `exchange_book` may depend
on its shape. A `format/` instance would be a promise this crate is not yet in a
position to make. The event stream's
message contract lives in `protocol/` instead, which fixes what a consumer
observes and deliberately nothing about bytes; the on-wire encoding belongs to
whatever carries the stream as payload, and the on-disk encoding of a
conserved value belongs to `exact_arith` in principle, though no such
encoding was ever built in the real [exact](https://github.com/Wandalen/exact)
family (→ [`../readme.md`](../readme.md)).

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_core/docs
printf 'rows in Definitions table:  '; grep -E '^\| `[a-z_]+/` \|' definition/readme.md | wc -l
printf 'rows in Instances table:    '; grep -E '^\| [a-z_]+ \| [0-9]{3} \|' definition/readme.md | wc -l
printf 'actual instance files:      '; find . -mindepth 2 -maxdepth 2 -regex '.*/[0-9][0-9][0-9]_[A-Za-z0-9_]*\.md' -not -path './definition/*' | wc -l
# rows in Definitions table:  6
# rows in Instances table:    11
# actual instance files:      11
```
