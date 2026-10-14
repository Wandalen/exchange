# Research: `exchange` vs. Open-Source Alternatives

### Scope

- **Purpose**: Answer, with evidence, whether an existing open-source crate could have replaced workstream 002's exchange core — or a part of it — instead of building from scratch.
- **Responsibility**: A dated, sourced comparison of this family's book-and-match crates against the closest Rust-ecosystem order-matching alternatives, checked against the requirements this family's own docs actually state.
- **In Scope**: Rust order-book and matching-engine crates on crates.io/GitHub, as surveyed 2026-10-02.
- **Out of Scope**: Non-Rust ecosystems (noted, not scored); ring-buffer/Disruptor crates — that is workstream 008's own build-vs-buy question, not this one; wallet/ledger crates — workstream 010's question; this family's own implementation detail (→ [`../crate/`](../crate/readme.md), [`../hard_problem/`](../hard_problem/readme.md)).

**Design status**: first research pass, dated 2026-10-02. Findings are sourced to crates.io/docs.rs/GitHub pages fetched on that date — re-verify before relying on a specific version, star count, or dependency list, since these crates evolve.

## Question

Workstream 002 is building a custom price-time order book and matching engine — 6 of the proposal's 23 crates exist so far (`exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_core`, `smoke_exchange_core`) — rather than depending on an existing crate. Was there an existing crate, or combination, that already solved this?

## Candidates Surveyed

| Crate | Category | Maturity | Link |
|---|---|---|---|
| `matchcore` | Single-threaded, deterministic, in-memory matching state machine | Small (5 deps), framed for exchange simulators/research | [crates.io](https://crates.io/crates/matchcore) · [docs.rs](https://docs.rs/matchcore/latest/matchcore/) |
| `orderbook-rs` (joaquinbejar) | Thread-safe, lock-free limit order book and matching engine | Mature — 544 GitHub stars, 713 commits, MIT, active (v0.15 as of this survey) | [GitHub](https://github.com/joaquinbejar/OrderBook-rs) · [docs.rs](https://docs.rs/orderbook-rs/latest/orderbook_rs/) |
| `limitbook` (solarpx) | In-memory CLOB, nanosecond-level matching | Early-stage — 10 stars, ~3 commits, Apache-2.0 | [GitHub](https://github.com/solarpx/limitbook) · [crates.io](https://crates.io/crates/limitbook) |

Also surveyed, not scored in the table below because each is either a near-duplicate of a scored candidate under a different name, or too thin on public documentation to compare fairly (noted here rather than silently dropped):

- **`landakram/orderbook-rs`**, **`dgtony/orderbook-rs`**, **`TechieBoy/rust-orderbook`** — smaller, earlier limit-order-book-and-matching-engine projects sharing the same name pattern as the scored `orderbook-rs`; TechieBoy's reports ~10 microseconds per match at 1M resting orders (self-reported, not independently re-measured here).
- **`luminengine`** — "lock-free, multi-threaded financial quantitative trading order book matching framework" per its crates.io listing; too little public detail beyond that tagline to score fairly.
- **`fx-core`** — "ultra-low-latency matching engine and lock-free order book for FX trading" per its crates.io listing; its crates.io page did not expose enough rendered detail to verify claims against this family's requirements.
- **`matching_engine`** — matches FIFO or Pro-rata; too little public documentation beyond the algorithm-choice tagline to score fairly.
- **CoinTossX** (arXiv 2102.10925) — an open-source, low-latency, high-throughput matching engine from academic market-microstructure research. Not a Rust crate (not surveyed as an adoption candidate), but confirms this family's problem shape is a recognized research subject outside the Rust ecosystem too.
- **`disruptor-rs`**, **`rusted-ring`** — LMAX-Disruptor-pattern ring-buffer crates. Relevant to workstream 008 (ring), not this one — none of the three scored candidates above use a Disruptor-style ring for ingress, and no crate found combines a Disruptor ring with order-book matching in one package.

## Comparison Against This Family's Actual Requirements

Requirements drawn from this family's own hard problems ([`../hard_problem/`](../hard_problem/readme.md)) and the Workstream Charter ([`../readme.md`](../readme.md) § Workstream Charter):

| Requirement | Source | `matchcore` | `orderbook-rs` | `limitbook` |
|---|---|---|---|---|
| No binary floats, exact price/quantity representation | [HP 007](../hard_problem/007_no_float.md) | ✅ distinct `Price`/`Quantity` structs, not float | ✅ `u128`/`u64`, no floats | ✅ `rust_decimal` |
| Price-time priority with FIFO at a level | [HP 002](../hard_problem/002_price_time.md) | ✅ core design goal | ✅ core design goal | ✅ core design goal |
| Partial fills | [HP 003](../hard_problem/003_partial_fills.md) | ✅ via market-to-limit/iceberg | ✅ preserved under priority | ✅ supported |
| Escrow-before-rest / a balance-reservation hook | [HP 004](../hard_problem/004_escrow_before_rest.md) | ❌ not addressed at all | ❌ book+match only, no wallet hook | ❌ not addressed at all |
| Self-trade policy, named and chosen | [HP 010](../hard_problem/010_self_trade.md) | ❌ not documented | ✅ three modes: `CancelMaker`/`CancelTaker`/`CancelBoth` | ❌ not documented |
| Time-in-force: GTC/IOC/FOK | [HP 020](../hard_problem/020_time_in_force.md) | ✅ GTC/IOC/FOK/GTD | ✅ GTC/IOC/FOK/GTD/DAY, plus more | ❌ limit/market only |
| Deterministic match, replayable | [HP 006](../hard_problem/006_deterministic_match.md) | ✅ explicit design claim, same-input-same-output | ✅ deterministic replay with journaling | ⚠️ not documented either way |
| Cancel and replace | [HP 009](../hard_problem/009_cancel_and_replace.md) | 🟡 amend operations, "replace" not named explicitly | ✅ atomic modify with rollback | 🟡 cancel yes, replace not documented |
| Zero/near-zero external (crates.io) dependencies | Observed convention — not a hard problem | 🟡 5 deps (lightest of the three) | ❌ ~20 deps (tokio, nats, dashmap, serde, …) | 🟡 lightweight plus `rust_decimal` |

`⚠️` = no data found on this requirement in the sources checked. `🟡` = partial match, detailed below.

## Requirement: Escrow-Before-Rest / A Balance-Reservation Hook

**Represented in**: [HP 004](../hard_problem/004_escrow_before_rest.md), [`feature/011_escrowport.md`](../feature/011_escrowport.md).

**Explanation**: the one requirement all three candidates fail identically, and for a structural reason, not an oversight — a general-purpose order-book library has no reason to know about any particular game or exchange's balance model, so none expose so much as a trait seam for "hold funds before resting an order." `orderbook-rs` comes closest in spirit: its separate risk layer enforces per-account notional and open-order limits, which is a *check*, not a *reservation* — it can refuse an order that would exceed a limit, but it never locks funds the way this family's `exchange_escrow` does. Adopting any of the three as-is would mean bolting a reservation layer on from outside, with no cooperation from the library's own match loop to call into it at the right moment (before resting, per [pitfall 011](../pitfall/011_resting_before_hold_try_succeeds.md)) — a timing guarantee an external library cannot give without a patch, since it isn't a config option but a hook that doesn't exist yet.

## Requirement: Self-Trade Policy

**Represented in**: [HP 010](../hard_problem/010_self_trade.md), [`stp_policy/`](../stp_policy/readme.md).

**Explanation**: `orderbook-rs` already ships this almost exactly as this family independently converged on it. Its three modes — `CancelMaker`, `CancelTaker`, `CancelBoth` — map directly onto the real `exchange_match::SelfMatchPolicy`'s own `CancelResting`, `CancelIncoming`, `CancelBoth` (see [`stp_policy/002_canceloldest.md`](../stp_policy/002_canceloldest.md)'s Design status): maker=resting, taker=incoming, same three-way split, different vocabulary. This is a genuine convergent-design finding, not a coincidence of similar names — two independent implementations of price-time matching with self-trade protection landed on the identical three-way partition (withdraw the resting side, withdraw the incoming side, or withdraw both), which suggests it is close to the only sensible partition of the problem. `matchcore` and `limitbook` do not document a self-trade policy at all.

## Requirement: Time-in-Force

**Represented in**: [HP 020](../hard_problem/020_time_in_force.md), [`tif/`](../tif/readme.md).

**Explanation**: solved territory for two of three. `matchcore` and `orderbook-rs` both implement GTC/IOC/FOK (plus GTD, which this family's own proposal does not ask for), and `orderbook-rs` goes considerably further — iceberg, post-only, pegged, trailing-stop, reserve-with-replenishment, and quote-notional market orders, none of which this family's hard problems or features name at all. `limitbook` has no time-in-force concept — every order behaves as GTC. This is the single clearest piece of evidence that TIF itself (the part of workstream 002 this family's own code explicitly has not built yet — see [`crate/003_exchange_tif.md`](../crate/003_exchange_tif.md)) is thoroughly solved ground elsewhere in the Rust ecosystem.

## Requirement: Zero/Near-Zero External Dependencies

**Represented in**: Observed convention — the real `exchange_core`'s own `Cargo.toml` depends on exactly one thing, `exact_arith` (an internal workstream dependency, not a crates.io external one); confirmed via direct inspection, zero `[dependencies]` entries outside this family's own crates across the whole `module/` tree.

**Explanation**: the three candidates span the full range. `matchcore` is close to this family's own convention — 5 dependencies, 2 of them dev-only. `limitbook` adds one substantial dependency, `rust_decimal`, already surveyed and found adequate for decimal representation by the sibling workstream 006 research. `orderbook-rs` is the opposite case: roughly 20 runtime dependencies, including an async runtime (`tokio`), a message bus client (`async-nats`), and a concurrent hash map (`dashmap`) — a materially heavier dependency footprint than this family has adopted anywhere else, for features (NATS publishing, async book management) this family's own hard problems never ask for.

## Conclusion

No single existing crate — and no realistic combination of them — covers this family's actual requirement set. As with the sibling workstream 006 research, the honest reading is narrower than "nothing out there helps":

- **Time-in-force and advanced order types are thoroughly solved ground.** `orderbook-rs` alone already implements more order-type variety (iceberg, pegged, trailing-stop, reserve orders) than this family's own 24 hard problems ask for. If this family builds `exchange_tif` from scratch, it will be re-solving a problem `orderbook-rs` solved more completely two years before this transcript was written.
- **Self-trade policy is a genuine convergent-design result, not a gap.** `orderbook-rs`'s `CancelMaker`/`CancelTaker`/`CancelBoth` and this family's real `CancelResting`/`CancelIncoming`/`CancelBoth` are the same three-way partition under different names — independent evidence that this family's own (currently hardcoded to `CancelIncoming`, not yet caller-configurable — see [`stp_policy/003_cancelnewest.md`](../stp_policy/003_cancelnewest.md)) policy shape is not an idiosyncratic choice.
- **The escrow/reservation hook is the genuinely unsolved part, on or off this list.** All three candidates — including the otherwise-comprehensive `orderbook-rs` — stop at the book-and-match boundary and expose no seam for a caller to hold funds before an order rests. This is this family's own real contribution, not a reinvention.
- **This family's own dependency convention rules out the most feature-complete candidate on its own terms.** `orderbook-rs`'s ~20-dependency footprint (tokio, NATS, dashmap) conflicts with the zero-external-dependency convention this family has followed everywhere else; adopting it would mean either accepting that footprint or stripping features back out — at which point the remaining value over building fresh narrows considerably.
- **Nothing surveyed integrates with this project's own `exact_arith` types or its own `ring_*` ingress crates** — both are this project's own prior inventions, so this was expected rather than a finding, but it means every candidate's price/quantity representation and concurrency model would need an adapter layer regardless of which one was chosen.

Building the book-and-match core from scratch therefore duplicates some already-solved ground (TIF, several order types, self-trade policy) in exchange for exact-arithmetic integration, a zero-dependency footprint, and an escrow seam none of the surveyed candidates offer. That is a real trade-off, most visible in the TIF and order-type gap — it is not the case that nothing out there could have shortened this build.

## Sources

- [matchcore — crates.io](https://crates.io/crates/matchcore)
- [matchcore — docs.rs](https://docs.rs/matchcore/latest/matchcore/)
- [orderbook-rs (joaquinbejar/OrderBook-rs) — GitHub](https://github.com/joaquinbejar/OrderBook-rs)
- [orderbook-rs — docs.rs](https://docs.rs/orderbook-rs/latest/orderbook_rs/)
- [limitbook (solarpx) — GitHub](https://github.com/solarpx/limitbook)
- [limitbook — crates.io](https://crates.io/crates/limitbook)
- [landakram/orderbook-rs — GitHub](https://github.com/landakram/orderbook-rs)
- [dgtony/orderbook-rs — GitHub](https://github.com/dgtony/orderbook-rs)
- [TechieBoy/rust-orderbook — GitHub](https://github.com/TechieBoy/rust-orderbook)
- [luminengine — crates.io](https://crates.io/crates/luminengine/0.2.3)
- [fx-core — crates.io](https://crates.io/crates/fx-core)
- [matching_engine — crates.io](https://crates.io/crates/matching_engine)
- [CoinTossX: An open-source low-latency high-throughput matching engine — arXiv](https://arxiv.org/pdf/2102.10925)
- [disruptor-rs — crates.io](https://crates.io/crates/disruptor-rs)
- [rusted-ring — crates.io](https://crates.io/crates/rusted-ring)
