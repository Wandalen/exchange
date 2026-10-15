# Pitfall: Self-trade allowed by default with no named policy

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Shipping without an explicit, named self-trade policy, so self-matches happen by accident rather than by rule.
- **In Scope**: `SelfMatchPolicy`'s own variant set.

### Statement

One account is often on both sides of a cross — an NPC and a player, or two
of a player's own orders — and without a named, deliberately chosen policy,
the engine's default behavior (allow, or cancel-oldest, or cancel-newest)
becomes an accident of implementation rather than a decision anyone made.

### How this crate avoids it

`SelfMatchPolicy` has no "allow" variant at all — self-match prevention is
unconditional, not merely a selectable policy as the source design asks
for. Grep-verifiable (scoped to the enum body, not the doc comments
discussing why `Allow` isn't here):

```bash
cd module/exchange_stp && command grep -A 20 "pub enum SelfMatchPolicy" src/lib.rs | command grep -E "^\s*[A-Za-z]+\s*,"
```

**Expected:** exactly three lines — `CancelResting,`, `CancelIncoming,`,
`CancelBoth,` — no `Allow,` among them.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/019_self_trade_allowed_by_default.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:916` | Pitfall in the source's Prompt 7 "Match policy" list |
