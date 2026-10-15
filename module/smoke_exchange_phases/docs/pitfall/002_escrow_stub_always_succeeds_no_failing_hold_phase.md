# Pitfall: Escrow stub that always succeeds left in the wall without a failing-hold phase still green

### Scope

- **Purpose**: Name a specific mistake this crate's own existence as a separate suite is meant to prevent.
- **Responsibility**: Shipping the wall smoke's always-succeeding escrow stub without also keeping a dedicated failing-hold phase (P15) passing.
- **In Scope**: `demo_p15_nohold`'s own coverage.

### Statement

The wall smoke's escrow is explicitly a stub that always succeeds, which
means it never exercises the "hold fails" path on its own — that path only
has coverage through P15, so if P15 is allowed to regress while the wall
stays green, the entire insufficient-funds rejection path can silently
break with nothing left to catch it.

### How this crate avoids it — partially, by existing; not yet by enforcement

`demo_p15_nohold.rs` exists as its own binary, independent of whatever the
P30 wall's escrow stub does:

```bash
cd module/smoke_exchange_phases && test -f src/bin/demo_p15_nohold.rs && echo present
```

**Expected:** `present`.

Same limit as [`001`](001_p30_green_while_earlier_phases_red.md): the
binary's existence gives P15 somewhere to be run, but nothing here forces
it to be run on every change, or fails loudly if it's skipped while the
wall (once `smoke_exchange_book` exists, Stage 10) stays green.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/034_escrow_stub_always_succeeds_no_failing_hold_phase.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:939` | Pitfall in the source's Prompt 7 "Process" list |
