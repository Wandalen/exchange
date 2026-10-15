# Pitfall: A replacement for another instrument

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Keeping `rest_replace`'s `instrument` argument and `new_resting.order.instrument` in agreement.
- **In Scope**: `rest_replace`'s `instrument`/`new_resting` pair.

### Statement

`rest_replace` looks the original up on `instrument`, but `Book::insert` seats
the replacement on whatever instrument its own order names. If the two
disagree, the original is cancelled on one book and the replacement rests on
another — an order moved between instruments, reported as a success.

### How this crate avoids it

`rest_replace` compares the two before anything else and returns
`RestReplaceError::InstrumentMismatch` with nothing touched. The check is
here, not left to a caller: both values are this function's own arguments,
and `exchange_inbound::inbound_apply` forwards a `Replace` command's two
fields unchecked.

[`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs)'s
`rest_replace_refuses_a_new_resting_for_a_different_instrument` and
`exchange_inbound`'s `replace_refuses_a_new_resting_for_a_different_instrument`
drive the mismatch directly.

### Sources

| File | Relationship |
|------|--------------|
| [`../../src/lib.rs`](../../src/lib.rs) | `rest_replace` — the instrument check comes first |
