# Pitfall: Escrow stub that always succeeds left in the wall without a failing-hold phase still green

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Shipping the wall smoke's always-succeeding escrow stub without also keeping a dedicated failing-hold phase (P15) passing.
- **In Scope**: Process

Redistributed to the single crate this testing-process concern belongs to:
[`../../module/smoke_exchange_phases/docs/pitfall/002_escrow_stub_always_succeeds_no_failing_hold_phase.md`](../../module/smoke_exchange_phases/docs/pitfall/002_escrow_stub_always_succeeds_no_failing_hold_phase.md)
— that writeup confirms `demo_p15_nohold` exists as a binary independent
of the wall's stub, with the same honest "exists, not yet enforced" caveat
as pitfall 033. This entry stays here only as the central catalog's own
record.

### Statement

The wall smoke's escrow is explicitly a stub that always succeeds, which means it never exercises the "hold fails" path on its own — that path only has coverage through P15, so if P15 is allowed to regress while the wall stays green, the entire insufficient-funds rejection path can silently break with nothing left to catch it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:939` | Pitfall in the source's Prompt 7 "Process" list |
