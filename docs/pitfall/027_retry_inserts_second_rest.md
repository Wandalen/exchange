# Pitfall: Retry inserts a second rest

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Letting a resubmitted order (a host or UI retry) create a second resting entry instead of being recognized as the same order.
- **In Scope**: Identity and cap

### Statement

A host or UI retrying a submission after a dropped acknowledgment is an expected, routine event, not an edge case — without idempotent handling of a repeated `OrderId`, each retry rests a new order, doubling the account's exposure for every retry that happens.

Redistributed to the single crate it's relevant to: [`../../module/exchange_idem/docs/pitfall/001_retry_inserts_second_rest.md`](../../module/exchange_idem/docs/pitfall/001_retry_inserts_second_rest.md) — this entry stays here only as the central catalog's own record.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:928` | Pitfall in the source's Prompt 7 "Identity and cap" list |
