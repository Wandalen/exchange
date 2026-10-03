# Feature: STP Policy

### Scope

- **Purpose**: One named self-trade rule.
- **Responsibility**: Offer allow, cancel-oldest, and cancel-newest as the self-trade policy choices.

**Design status**: Held — `exchange_match::SelfMatchPolicy` exists with policy variants covering this choice (see `module/exchange_match/src/lib.rs:124` for the exact variant names).

### Statement

Self-trade policy is a closed choice of three — allow, cancel-oldest, cancel-newest — configured once per book rather than decided ad hoc at match time.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:323` | Feature 15 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1046` | Feature 15's English title, Prompt 9 |
