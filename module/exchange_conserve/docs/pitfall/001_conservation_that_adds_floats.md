# Pitfall: Conservation that adds floats

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Implementing the sum-to-zero conservation check using floating-point addition.
- **In Scope**: `conserve_assert`'s own arithmetic.

### Statement

Summing the legs in floats brings back the rounding the rest of the system
avoids: the check passes or fails by summation order, not by whether value
was conserved.

### How this crate avoids it

Every leg is an `exact_arith::Money`, summed by
`exact_arith::money_sum_assert_zero` in an `i128` accumulator: the true total
or `ConserveError::Overflow`, never an approximation. There is no `f32`/`f64`
in the crate (`grep -n "f32\|f64" module/exchange_conserve/src/lib.rs`
returns nothing).
[`tests/exchange_conserve_test.rs`](../../tests/exchange_conserve_test.rs)'s
`a_mixed_batch_of_different_magnitudes_still_conserves` exercises a multi-leg
sum of varying magnitude and order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/003_conservation_that_adds_floats.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:894` | Pitfall in the source's Prompt 7 "Money" list |
