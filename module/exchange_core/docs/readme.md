# docs

Design documentation for `exchange_core`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `algorithm/` | The decided matching semantics — validate, reserve, match, split, dispose, emit — plus self-match, cancel/amend, and fee assessment |
| `decisions/` | Architecture Decision Records for open trade-offs this crate could not close unilaterally |
| `definition/` | Module Index — every definition and every instance in this crate, in one place |
| `feature/` | v0.1 scope — order semantics committed, hosting ruled, exit criteria fixed |
| `invariant/` | Properties the book and ledger must never violate — escrow coverage, arrival order, balance partition |
| `item/` | The exposed `pub` surface, as built, against the source design's own proposal |
| `pitfall/` | Traps whose failures leave every order-level check passing — ledger corruption first |
| `protocol/` | The emitted event stream's message contract, and how it may evolve |
| `state_machine/` | Lifecycles this crate owns — the states one order passes through |
| `workaround/` | External constraints this crate absorbs, with costs and deletion conditions |

The instances above document the **semantics grain** already decided by
`task/decisions.md` Q-07/Q-10 and the corpus rulings cited throughout —
price-time priority, TIF disposition, escrow discipline, event-sourced
recovery. The matching logic was implemented on 2026-08-30
without a spike — no `spike/exchange_core` was ever created, and the family was
built directly against these instances. Two of the three questions they named
open were closed by that work and are recorded with their reasoning in
[`algorithm/001`](algorithm/001_price_time_priority_matching.md): the
executed-price rule is the maker's price, and the book is a sorted `Vec` per
side. Stop-trigger mechanics remain open and untouched. No `format/` directory
exists — a chosen structure is not a frozen layout, and nothing outside
`exchange_book` may depend on its shape. The event stream's *shape* is
therefore routed through
`protocol/`, which fixes messages and says nothing about memory or wire
layout — the two encodings belong to the wire-transport layer and
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md), not here.

**Not everything above is inherited.** Where the source material states a
requirement without a mechanism — a self-match ban with no resolution
policy, a two-sided balance present only in its un-integrated message tier —
or omits a topic entirely, as it does for amend and for maker/taker
classification, the owning instance says so in its own text and marks what it
originates. A reader who assumes the corpus settled these will not look for
what it did not settle.

### Related Crates

Not yet a dependency of, or dependent on, any crate — the edges below are
designed but enter manifests only at the first implementation increment, per
`task/decisions.md` Q-10.

| Crate | Relationship |
|-------|--------------|
| [`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) | Prospective dependency — every price and balance is exact arithmetic |
