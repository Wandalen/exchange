# Feature: conservation_assert

### Scope

- **Purpose**: Check a set of fills sums to zero.
- **Responsibility**: Assert that a batch of fills conserves value exactly.

### Statement

`conservation_assert` checks that a set of fills' legs sum to exactly zero, giving hard problem 5 a function to call rather than leaving it as an unchecked invariant.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:329` | Feature 21 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1052` | Feature 21's English title, Prompt 9 |
