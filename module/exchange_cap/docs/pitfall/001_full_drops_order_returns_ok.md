# Pitfall: Full drops the order and returns ok

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Never silently discarding an order at capacity — a full
  level or side must report a named error, not a quiet success.
- **In Scope**: `cap_check_rest`/`cap_check_level`'s own return value.

### Statement

Hitting a capacity limit must be a visible, named failure — if a full book
drops the order and reports success anyway, the submitter believes it has a
live order that was never actually accepted, the same silent-loss shape as
the ring-overflow pitfall but at the book's own limit instead of the ring's.

### How this crate avoids it

Both check functions return `Result< (), CapError >`, and the refusal
variants (`CapError::RestsFull`/`CapError::LevelsFull`) are exercised by a
real test — never left to a theoretical "should return Err" claim.
Test-verifiable:

```bash
cd module/exchange_cap && cargo test --quiet a_rest_at_the_cap_is_refused
```

**Expected:** the test passes — `cap_check_rest( CAPS, 2 )` with
`max_rests : 2` returns `Err( CapError::RestsFull )`, never `Ok( () )`, once
`current_rests` has already reached the configured cap.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/028_full_drops_order_returns_ok.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:929` | Pitfall in the source's Prompt 7 "Identity and cap" list |
