# phase

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002. A phase is one step of the build-out, each adding exactly one new contract (`../../../../codename_space_sandbox/intake/core_exchange.txt:778`); 30 are named, P01 through P30, with P30 being the wall smoke from Prompt 4.

This phased build-out was not followed in the real implementation — the 5 real crates (`exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_core`) were built directly, not through these 30 discrete phase gates or their smoke/golden-output checks. None of P01–P30 exist as separate milestones or demo binaries in the real build.

A closing note in the source (core_exchange.txt:813) excludes two things from ever becoming phases: i128 arithmetic and a second ring for fills.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_p01_id.md`](001_p01_id.md) | InstrumentId/OrderId unique, raw round-trip |
| [`002_p02_side.md`](002_p02_side.md) | Side: opposite bid ↔ ask |
| [`003_p03_tif.md`](003_p03_tif.md) | TIF: GTC rests, IOC does not, FOK requires full |
| [`004_p04_stp.md`](004_p04_stp.md) | STP: three policies, names and distinction only |
| [`005_p05_spec.md`](005_p05_spec.md) | spec: tick 0.05, lot 1; zero tick is an error |
| [`006_p06_snap.md`](006_p06_snap.md) | price_snap 1.26 → 1.25 via 006 |
| [`007_p07_order.md`](007_p07_order.md) | Order with sequence; empty quantity is an error |
| [`008_p08_seq.md`](008_p08_seq.md) | seq_next monotonic, without wall clock |
| [`009_p09_fifo.md`](009_p09_fifo.md) | Level: two rests at one price, FIFO pop |
| [`010_p10_best.md`](010_p10_best.md) | Book: best bid/best ask after two inserts |
| [`011_p11_walk.md`](011_p11_walk.md) | Price walk from the best, not map iteration |
| [`012_p12_cap.md`](012_p12_cap.md) | Cap rest = 2; third is RestsFull |
| [`013_p13_dup.md`](013_p13_dup.md) | Same OrderId twice is Duplicate |
| [`014_p14_hold.md`](014_p14_hold.md) | Escrow stub: hold, release on cancel, commit on fill |
| [`015_p15_nohold.md`](015_p15_nohold.md) | Hold failure → order not in book |
| [`016_p16_fill.md`](016_p16_fill.md) | Fill names maker and taker |
| [`017_p17_cons.md`](017_p17_cons.md) | conserve_assert: 10/-10 zero, 10/-9 error |
| [`018_p18_cancel.md`](018_p18_cancel.md) | Rest one bid, cancel removes it |
| [`019_p19_repl.md`](019_p19_repl.md) | Replace: old gone, new stands |
| [`020_p20_partial.md`](020_p20_partial.md) | Match 12 vs 10 then 5: fills 10 and 2, remainder 3 |
| [`021_p21_worse.md`](021_p21_worse.md) | Worse price untouched while better is alive |
| [`022_p22_ioc.md`](022_p22_ioc.md) | IOC leaves no rest |
| [`023_p23_fok.md`](023_p23_fok.md) | FOK on a thin book: reject, book unchanged |
| [`024_p24_stp.md`](024_p24_stp.md) | STP CancelOldest: no self-fill |
| [`025_p25_halt.md`](025_p25_halt.md) | Halt blocks place; resume allows |
| [`026_p26_depth.md`](026_p26_depth.md) | depth_top(2) matches the book |
| [`027_p27_snap.md`](027_p27_snap.md) | Snapshot of rows; live cancel doesn't change it |
| [`028_p28_drain.md`](028_p28_drain.md) | Two producers, full-order drain, same checksum twice |
| [`029_p29_over.md`](029_p29_over.md) | Ring cap 2: third publish is Reject Overflow |
| [`030_p30_wall.md`](030_p30_wall.md) | smoke_exchange_book — the wall, the merge gate |
