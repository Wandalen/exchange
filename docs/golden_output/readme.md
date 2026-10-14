# golden_output

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002. A golden_output is the exact printed line a phase's smoke must produce to count as passing — Prompt 6 (`../../../../codename_space_sandbox/intake/core_exchange.txt:819`) embeds one per phase, consolidated again in Prompt 9 (`../../../../codename_space_sandbox/intake/core_exchange.txt:1210-1218`).

This phased build-out was not followed in the real implementation — the 5 real crates (`exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_core`) were built directly, not through these 30 discrete phase gates or their smoke/golden-output checks. None of P01–P30 exist as separate milestones or demo binaries in the real build.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_p01_golden.md`](001_p01_golden.md) | `eq=1 → ok` |
| [`002_p02_golden.md`](002_p02_golden.md) | `opp=ask → ok` |
| [`003_p03_golden.md`](003_p03_golden.md) | `gtc=1 ioc=0 fok=1 → ok` |
| [`004_p04_golden.md`](004_p04_golden.md) | `n=3 → ok` |
| [`005_p05_golden.md`](005_p05_golden.md) | `ok=1 zt=1 → ok` |
| [`006_p06_golden.md`](006_p06_golden.md) | `p=1.25 → ok` |
| [`007_p07_golden.md`](007_p07_golden.md) | `seq=1 zq=1 → ok` |
| [`008_p08_golden.md`](008_p08_golden.md) | `s=1,2,3 → ok` |
| [`009_p09_golden.md`](009_p09_golden.md) | `first=a → ok` |
| [`010_p10_golden.md`](010_p10_golden.md) | `best=1.00 → ok` |
| [`011_p11_golden.md`](011_p11_golden.md) | `w=1.00,0.95 → ok` |
| [`012_p12_golden.md`](012_p12_golden.md) | `ok_full` |
| [`013_p13_golden.md`](013_p13_golden.md) | `dup=1 → ok` |
| [`014_p14_golden.md`](014_p14_golden.md) | `rel=1 com=1 → ok` |
| [`015_p15_golden.md`](015_p15_golden.md) | `n=0 → ok` |
| [`016_p16_golden.md`](016_p16_golden.md) | `maker=a taker=b → ok` |
| [`017_p17_golden.md`](017_p17_golden.md) | `z=1 nz=1 → ok` |
| [`018_p18_golden.md`](018_p18_golden.md) | `n=0 → ok` |
| [`019_p19_golden.md`](019_p19_golden.md) | `old=0 new=1 → ok` |
| [`020_p20_golden.md`](020_p20_golden.md) | `fills=10,2 rest=3 → ok` |
| [`021_p21_golden.md`](021_p21_golden.md) | `untouched=1 → ok` |
| [`022_p22_golden.md`](022_p22_golden.md) | `rest=0 → ok` |
| [`023_p23_golden.md`](023_p23_golden.md) | `rej=1 book=same → ok` |
| [`024_p24_golden.md`](024_p24_golden.md) | `self=0 → ok` |
| [`025_p25_golden.md`](025_p25_golden.md) | `halt=1 resume=1 → ok` |
| [`026_p26_golden.md`](026_p26_golden.md) | `d=1.00:3,0.95:4 → ok` |
| [`027_p27_golden.md`](027_p27_golden.md) | `snap=1 live=0 → ok` |
| [`028_p28_golden.md`](028_p28_golden.md) | `a=0x… b=0x… → ok if a==b` |
| [`029_p29_golden.md`](029_p29_golden.md) | `overflow=1 → ok` |
| [`030_p30_wall_golden.md`](030_p30_wall_golden.md) | The full wall block, the merge gate |
