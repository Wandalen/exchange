# smoke_exchange_phases

The P01–P29 phase-smoke ladder — one tiny, golden-line-checkable binary per
phase, graded independently of the P30 wall smoke (`smoke_exchange_book`).
Each phase proves exactly one new contract; earlier phases stay green as
later ones are added.

```bash
cargo run -q -p smoke_exchange_phases --bin demo_p01_id
# eq=1
# ok
```

## Why a separate crate from the wall smoke

`smoke_exchange_book` grades the whole path end to end in one scenario; this
crate grades each new contract in isolation, the moment it lands, before it
is wired into anything bigger. One crate, one job each — see the root
readme's own Responsibility Table.

## Adding a phase

1. Add the dependency the phase needs (if not already present) to
   [`Cargo.toml`](Cargo.toml).
2. Add `src/bin/demo_pNN_name.rs` — Cargo auto-discovers it, no manifest entry
   needed.
3. Match the exact golden line in `docs/golden_output/0NN_*.md` — this is
   what CI diffs against.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — one path dependency per phase crate in use |
| [`src/bin/demo_p01_id.rs`](src/bin/demo_p01_id.rs) | P01 — id round-trip |
| [`src/bin/demo_p02_side.rs`](src/bin/demo_p02_side.rs) | P02 — side opposite |
| [`src/bin/demo_p03_tif.rs`](src/bin/demo_p03_tif.rs) | P03 — TIF disposition |
| [`src/bin/demo_p04_stp.rs`](src/bin/demo_p04_stp.rs) | P04 — STP policy count |
| [`src/bin/demo_p05_spec.rs`](src/bin/demo_p05_spec.rs) | P05 — spec validity (tick/lot, zero tick refused) |
| [`src/bin/demo_p06_snap.rs`](src/bin/demo_p06_snap.rs) | P06 — price snap to the tick grid |
| [`src/bin/demo_p07_order.rs`](src/bin/demo_p07_order.rs) | P07 — order claims a sequence; zero-qty refused |
| [`src/bin/demo_p08_seq.rs`](src/bin/demo_p08_seq.rs) | P08 — sequence monotonicity |
| [`src/bin/demo_p09_fifo.rs`](src/bin/demo_p09_fifo.rs) | P09 — same-price FIFO pop |
| [`src/bin/demo_p10_best.rs`](src/bin/demo_p10_best.rs) | P10 — best bid/ask after inserts |
| [`src/bin/demo_p11_walk.rs`](src/bin/demo_p11_walk.rs) | P11 — price walk visits sorted order |
| [`src/bin/demo_p12_cap.rs`](src/bin/demo_p12_cap.rs) | P12 — rest cap refusal |
| [`src/bin/demo_p13_dup.rs`](src/bin/demo_p13_dup.rs) | P13 — duplicate `OrderId` refused |
| [`src/bin/demo_p14_hold.rs`](src/bin/demo_p14_hold.rs) | P14 — escrow hold releases on cancel, commits on fill |
| [`src/bin/demo_p15_nohold.rs`](src/bin/demo_p15_nohold.rs) | P15 — a failed hold never reaches the book |
| [`src/bin/demo_p16_fill.rs`](src/bin/demo_p16_fill.rs) | P16 — a cross's `Trade` names both maker and taker |
| [`src/bin/demo_p17_cons.rs`](src/bin/demo_p17_cons.rs) | P17 — `conserve_assert` distinguishes balanced from unbalanced |
| [`src/bin/demo_p18_cancel.rs`](src/bin/demo_p18_cancel.rs) | P18 — cancel actually removes the resting order |
| [`src/bin/demo_p19_repl.rs`](src/bin/demo_p19_repl.rs) | P19 — replace is atomic: old gone, new stands |
| [`src/bin/demo_p20_partial.rs`](src/bin/demo_p20_partial.rs) | P20 — a taker splits correctly across two makers |
| [`src/bin/demo_p21_worse.rs`](src/bin/demo_p21_worse.rs) | P21 — a worse price level stays untouched while a better one suffices |
| [`src/bin/demo_p22_ioc.rs`](src/bin/demo_p22_ioc.rs) | P22 — an IOC order never rests its remainder |
| [`src/bin/demo_p23_fok.rs`](src/bin/demo_p23_fok.rs) | P23 — a FOK order rejects whole, with zero side effects |
| [`src/bin/demo_p24_stp.rs`](src/bin/demo_p24_stp.rs) | P24 — `CancelResting` prevents a same-account self-fill |
| [`src/bin/demo_p25_halt.rs`](src/bin/demo_p25_halt.rs) | P25 — halt/resume round-trip |
| [`src/bin/demo_p26_depth.rs`](src/bin/demo_p26_depth.rs) | P26 — `depth_top(2)` matches the book |
| [`src/bin/demo_p27_snap.rs`](src/bin/demo_p27_snap.rs) | P27 — snapshot survives a live cancel taken after it |
| [`src/bin/demo_p28_drain.rs`](src/bin/demo_p28_drain.rs) | P28 — two producers drain in one deterministic order |
| [`src/bin/demo_p29_over.rs`](src/bin/demo_p29_over.rs) | P29 — a full ring rejects rather than silently drops |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 2 "Process" pitfalls this suite's own structure bears on |

## Related

- [`docs/phase/`](../../docs/phase/readme.md) — the phase catalog each binary here grades
- [`docs/golden_output/`](../../docs/golden_output/readme.md) — the exact line each binary must print
