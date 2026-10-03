# Pitfall: Self-trade allowed by default with no named policy

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Shipping without an explicit, named self-trade policy, so self-matches happen by accident rather than by rule.
- **In Scope**: Match policy

Redistributed to the single crate it's relevant to: [`../../module/exchange_stp/docs/pitfall/001_self_trade_allowed_by_default.md`](../../module/exchange_stp/docs/pitfall/001_self_trade_allowed_by_default.md) — this entry stays here only as the central catalog's own record.

**Design status**: Avoided, and more strongly than asked — `SelfMatchPolicy` (now in its own `exchange_stp` crate, re-exported by `exchange_match`; not at the stale `module/exchange_match/src/lib.rs:124-132` this note originally cited before the Stage 1 extraction) has no "allow" variant at all (`CancelResting`/`CancelIncoming`/`CancelBoth` only). Self-match prevention is unconditional in the real build, not merely a named, selectable policy as this pitfall asks for — there is no way to configure an account to trade against itself.

### Statement

One account is often on both sides of a cross — an NPC and a player, or two of a player's own orders — and without a named, deliberately chosen policy, the engine's default behavior (allow, or cancel-oldest, or cancel-newest) becomes an accident of implementation rather than a decision anyone made.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:916` | Pitfall in the source's Prompt 7 "Match policy" list |
