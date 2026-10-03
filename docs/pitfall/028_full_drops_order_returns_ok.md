# Pitfall: Full drops the order and returns ok

**Redistributed**: this pitfall is `exchange_cap`'s own concern (its
`cap_check_rest`/`cap_check_level` are the only place a capacity refusal is
decided) — see
[`../../module/exchange_cap/docs/pitfall/001_full_drops_order_returns_ok.md`](../../module/exchange_cap/docs/pitfall/001_full_drops_order_returns_ok.md)
for the full writeup and the test that verifies the avoidance.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:929` | Pitfall in the source's Prompt 7 "Identity and cap" list |
