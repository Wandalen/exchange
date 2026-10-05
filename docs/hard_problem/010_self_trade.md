# Hard Problem: Self-Trade

### Scope

- **Purpose**: Allow, cancel-oldest, or cancel-newest — one rule.
- **Responsibility**: Apply one named policy whenever an account would trade against itself.

**Design status**: Held — `exchange_match` has `SelfMatchPolicy` and `SelfMatchCancellation`, and `exchange_fill::CancelCause::SelfMatch` covers the resulting cancellation.

### Statement

One account is frequently on both sides of a potential match, so self-trade needs exactly one named policy — allow, cancel-oldest, or cancel-newest. Without it, an accidental self-fill can break escrow's accounting.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:232-235` | Hard problem 10 in the source's Prompt 1 answer for workstream 002 |
