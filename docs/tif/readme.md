# tif

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002 — the three-value time-in-force type.

**Redistributed**: all 3 instances below are single-crate-relevant — the
type they catalog, `Tif`, is owned by exactly one crate. See
[`../../module/exchange_tif/docs/tif/readme.md`](../../module/exchange_tif/docs/tif/readme.md)
for the crate-local home (same 3 instances, corrected — the type is now
real and built; see that readme's own table for exactly which real callers
consult which value today).

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_gtc.md`](001_gtc.md) | Rests until filled or cancelled |
| [`002_ioc.md`](002_ioc.md) | Fills what it can now, discards the rest |
| [`003_fok.md`](003_fok.md) | Fills completely or rejects, book unchanged |
