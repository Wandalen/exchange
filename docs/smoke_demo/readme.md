# smoke_demo

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002. A smoke_demo is the tiny, cheap, golden-line-checkable binary that grades one phase — Prompt 6 (`../../../../codename_space_sandbox/intake/core_exchange.txt:819`) names one per phase, `demo_p01_id` through `demo_p29_over`, plus the wall smoke `smoke_exchange_book` for P30.

This phased build-out was not followed in the real implementation — the 5 real crates (`exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_core`) were built directly, not through these 30 discrete phase gates or their smoke/golden-output checks. None of P01–P30 exist as separate milestones or demo binaries in the real build.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_p01_demo_p01_id.md`](001_p01_demo_p01_id.md) | Round-trip raw 7 out and back |
| [`002_p02_demo_p02_side.md`](002_p02_demo_p02_side.md) | Opposite of bid is ask |
| [`003_p03_demo_p03_tif.md`](003_p03_demo_p03_tif.md) | GTC rests, IOC doesn't, FOK requires full |
| [`004_p04_demo_p04_stp.md`](004_p04_demo_p04_stp.md) | Three STP policies, distinct |
| [`005_p05_demo_p05_spec.md`](005_p05_demo_p05_spec.md) | Tick 0.05, lot 1; zero tick fails |
| [`006_p06_demo_p06_snap.md`](006_p06_demo_p06_snap.md) | 1.26 snaps to tick 0.05 |
| [`007_p07_demo_p07_order.md`](007_p07_demo_p07_order.md) | Order with seq; zero qty rejected |
| [`008_p08_demo_p08_seq.md`](008_p08_demo_p08_seq.md) | Three seq_next calls, monotonic |
| [`009_p09_demo_p09_fifo.md`](009_p09_demo_p09_fifo.md) | Two rests, one price; earliest pops first |
| [`010_p10_demo_p10_best.md`](010_p10_demo_p10_best.md) | Best bid after bids at 1.00 and 0.95 |
| [`011_p11_demo_p11_walk.md`](011_p11_demo_p11_walk.md) | Walk order 1.00 then 0.95, not map order |
| [`012_p12_demo_p12_cap.md`](012_p12_demo_p12_cap.md) | Cap 2, third rest refused |
| [`013_p13_demo_p13_dup.md`](013_p13_demo_p13_dup.md) | Same OrderId twice refused |
| [`014_p14_demo_p14_hold.md`](014_p14_demo_p14_hold.md) | Hold, cancel releases, fill commits |
| [`015_p15_demo_p15_nohold.md`](015_p15_demo_p15_nohold.md) | Hold fails; book count stays 0 |
| [`016_p16_demo_p16_fill.md`](016_p16_demo_p16_fill.md) | One cross names maker and taker |
| [`017_p17_demo_p17_cons.md`](017_p17_demo_p17_cons.md) | 10/-10 zero; 10/-9 fails |
| [`018_p18_demo_p18_cancel.md`](018_p18_demo_p18_cancel.md) | Rest then cancel, count returns to 0 |
| [`019_p19_demo_p19_repl.md`](019_p19_demo_p19_repl.md) | Replace: old missing, new present |
| [`020_p20_demo_p20_partial.md`](020_p20_demo_p20_partial.md) | Ask 12 vs 10 then 5 at 1.00: fills 10, 2 |
| [`021_p21_demo_p21_worse.md`](021_p21_demo_p21_worse.md) | 0.95 untouched while 1.00 lives |
| [`022_p22_demo_p22_ioc.md`](022_p22_demo_p22_ioc.md) | IOC leaves no rest |
| [`023_p23_demo_p23_fok.md`](023_p23_demo_p23_fok.md) | FOK on thin book: reject, book unchanged |
| [`024_p24_demo_p24_stp.md`](024_p24_demo_p24_stp.md) | CancelOldest, same account both sides: no self-fill |
| [`025_p25_demo_p25_halt.md`](025_p25_demo_p25_halt.md) | Halt rejects; resume accepts |
| [`026_p26_demo_p26_depth.md`](026_p26_demo_p26_depth.md) | depth_top(2) matches book |
| [`027_p27_demo_p27_snap.md`](027_p27_demo_p27_snap.md) | Snap, then live cancel; snap unchanged |
| [`028_p28_demo_p28_drain.md`](028_p28_demo_p28_drain.md) | Two producers, two runs, same checksum |
| [`029_p29_demo_p29_over.md`](029_p29_demo_p29_over.md) | Ring cap 2, third publish overflows |
| [`030_p30_smoke_exchange_book.md`](030_p30_smoke_exchange_book.md) | The wall — the merge-gate demo |
