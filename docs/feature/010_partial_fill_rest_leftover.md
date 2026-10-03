# Feature: Partial Fill, Rest The Leftover

### Scope

- **Purpose**: A fill that doesn't exhaust the taker rests what's left.
- **Responsibility**: Rest whatever quantity a match didn't consume.

### Statement

When a taker's quantity outlasts the resting liquidity it crossed, the unfilled remainder rests in the book exactly as a fresh order would — partial fills feed back into the same resting path rather than a separate one.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:318` | Feature 10 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1041` | Feature 10's English title, Prompt 9 |
