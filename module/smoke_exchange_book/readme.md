# smoke_exchange_book

The wall: every stage in one run, against one fixed golden block.

```bash
cargo run -p smoke_exchange_book
```

```text
seed=1
fills=10@1.00,2@1.00
rest_bid=3@1.00
ioc_rest=0
fok_rej=1
dup=1
halt=1
stp_fill=0
overflow=1
conserve=0
depth=1.00:3,0.95:4
a=0x... b=0x...
ok
```

(`a`/`b` are a checksum pair whose hex digits vary between runs — the pass
condition is `a == b`, never a specific literal.)

## One scenario, nine properties

**Multi-level matching.** Three bids rest — 10@1.00, 5@1.00, 4@0.95 — then a
GTC ask for 12 takes the first bid whole and the second partially, in arrival
order, leaving 3@1.00 and the untouched 4@0.95 resting.

**Depth.** `depth_get(2)` is read immediately after the GTC taker, before the
IOC taker below depletes the book further — `1.00:3,0.95:4`, the same two
levels the matching step above left behind.

**IOC.** An ask for 100 sweeps both remaining levels (3 then 4) and drops the
other 93 rather than resting it — `ioc_rest=0`. This is the regression this
scenario was written to catch: `exchange_core::step_place` once rested an
IOC's unfilled remainder because nothing checked `Tif::Ioc` before the rest
step, which also leaked the remainder's own escrow reservation (see
[`exchange_core/readme.md`](../exchange_core/readme.md) for both fixes).

**FOK.** An ask for 1000 against the now-empty book is rejected whole —
`fok_rej=1` — and the book is unchanged. `exchange_match::cross` reports an
unfillable FOK as `Ok` with the full quantity still outstanding, by design
(translating that into a rejection is the facade's job); this scenario proves
the facade actually does it, not just that `cross` reports the right shape.

**Halt.** Halting the instrument refuses a new placement
(`ExchangeError::Rejected(RejectReason::Halted)`); clearing it allows the same
placement again — `halt=1`.

**Self-match prevention.** An isolated account, on its own fresh exchange
separate from the scene above, rests a sell then crosses it with its own buy
under `SelfMatchPolicy::CancelResting` — `stp_fill=0`, and the resting leg is
withdrawn rather than filled. On its own exchange rather than the scene's
because the scene's own halt round trip leaves a resting sell behind; a
probe sharing that book would need its own price chosen to avoid sweeping it
first, which is exactly the kind of fragile coincidence a fresh exchange
removes by construction.

**Idempotency, standalone.** `dup=1` — `exchange_idem::idem_insert` refuses
an `OrderId`'s second sighting. Checked outside the facade on purpose:
`Exchange::claim_order` always reassigns a submitted order's id internally,
so a caller-chosen duplicate can never reach `exchange_step` in the first
place. There is no facade path to this property at all.

**Two-ring determinism.** Two independent rings, each raced from its own OS
thread, drain into one checksum in a fixed lane order (ring A fully, then
ring B) decided by the smoke, never by which thread happened to finish
first. Two independent runs reproduce the same checksum — `a == b` — despite
genuinely racing real threads each time.

**Ring overflow.** A ring built at capacity 2 refuses a third publish as an
explicit `Err`, never a silent drop — `overflow=1`.

**Conservation, standalone.** `conserve=0` — `exchange_conserve::conserve_assert`
audits the GTC and IOC takers' real trades and finds zero violations. Also
checked outside the facade on purpose: `exchange_match::cross` has no
dependency on `exchange_conserve` at all, so nothing in the matching path
checks this on its own behalf.

## Three dependencies, each named for what it buys

[`exchange_core`](../exchange_core/readme.md) alone covers everything above
except idempotency and conservation — the same reasoning this lane's
predecessor, `smoke_exchange_core`, already gave for a single dependency (see
git history, since Stage 10 of this family's refactor replaced that crate
with this one): a re-export the facade stops providing is a build failure
here, not a gap nobody notices.

[`exchange_idem`](../exchange_idem/readme.md) and
[`exchange_conserve`](../exchange_conserve/readme.md) are each a deliberate,
named exception to that rule — not a quiet widening of it. Both check
properties `exchange_core` never wires into the facade at all (see "Nine
properties" above for why each one specifically has no path through
`exchange_step`), so there is no re-export to depend on in the first place;
depending on the leaf crate directly is the only way to check either
property anywhere.

## Why four probes never touch the scene's own `Exchange`

`duplicate_rejected`, `ring_determinism`, `ring_overflow_rejects`, and
`self_trade_fill_count` each build their own `IdSet`, ring, or `Exchange`
from nothing, never reusing the one the main scene constructs. For the
first three this is simply that no `Exchange` is involved at all. For the
self-trade probe it is load-bearing, not just tidy: the scene's own halt
round trip leaves a resting sell on the book, and a self-trade probe
sharing that book would need its own price picked to avoid crossing it
first — a fresh `Exchange` removes that risk by construction instead of by
a price choice nobody would think to re-check later. Matches the same
granularity `smoke_exchange_phases`'s own P13/P24/P28/P29 phases already
established.

## Grading

The lane's exit code is the verdict, and is this crate's own reached-test.
`assert!`/`assert_eq!` are the mechanism; a panic is a failed lane, naming
exactly which of the nine properties above failed to hold.

`tests/lane_test.rs` does not replace that, it runs it, plus each
independently-measurable property on its own — a regression that only
breaks one property should name that one, not force a reader into the whole
golden block to find out which.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`docs/workaround/readme.md`](docs/workaround/readme.md) | The external constraint this crate absorbs — none, verified against its dependency graph and source |
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_core`, `exchange_conserve`, `exchange_idem` |
| [`src/lib.rs`](src/lib.rs) | The scene, the four standalone probes, the formatters, and the golden-block assertion |
| [`src/main.rs`](src/main.rs) | The lane's process entry point, and nothing else |
| [`tests/lane_test.rs`](tests/lane_test.rs) | Runs the lane, and each independent property on its own, from the suite |

## Related

- [`exchange_core/`](../exchange_core/readme.md) — the facade this lane walks end to end
- [`exchange_idem/`](../exchange_idem/readme.md) — the standalone duplicate check
- [`exchange_conserve/`](../exchange_conserve/readme.md) — the standalone conservation audit
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — the per-phase ladder this lane's scenario draws its pieces from
- [`../../docs/golden_output/030_p30_wall_golden.md`](../../docs/golden_output/030_p30_wall_golden.md) — the exact block this lane must print
