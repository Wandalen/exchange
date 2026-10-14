# Pitfall: Retry inserts a second rest

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting a resubmitted order (a host or UI retry) create a second resting entry instead of being recognized as the same order.
- **In Scope**: `idem_insert`'s own refusal behavior.

### Statement

A host or UI retrying a submission after a dropped acknowledgment is an
expected, routine event, not an edge case — without idempotent handling of
a repeated `OrderId`, each retry rests a new order, doubling the account's
exposure for every retry that happens.

### How this crate avoids it

`idem_insert` refuses a second insert of an already-seen `OrderId`,
returning `Err( IdemError::Duplicate )` rather than silently accepting it —
checkable *before* the id ever reaches `exchange_book::Book::insert`.
Test-verifiable:

```bash
cd module/exchange_idem && cargo test --all-features p13_a_repeat_insert_is_refused_as_a_duplicate 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `test result: ok. 1 passed`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/027_retry_inserts_second_rest.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:928` | Pitfall in the source's Prompt 7 "Identity and cap" list |
