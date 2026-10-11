# Pitfall: Conservation that adds floats

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Implementing the sum-to-zero conservation check using floating-point addition.
- **In Scope**: `conserve_assert`'s own arithmetic.

### Statement

A conservation check exists to prove nothing was created or destroyed across
a set of fills; summing the legs with float addition reintroduces the exact
rounding error the rest of the system was built to avoid, so the check can
pass or fail depending on summation order rather than on whether the trade
actually conserved value.

### How this crate avoids it

Every leg `conserve_assert` builds is an `exact_arith::Money`, summed by
`exact_arith::money_sum_assert_zero` ([`src/lib.rs`](../../src/lib.rs)) in an
`i128` accumulator — there is no `f32`/`f64` anywhere in this crate
(`grep -n "f32\|f64" module/exchange_conserve/src/lib.rs` returns zero hits).
The sum is exact by construction: it either returns the true total or refuses
with `ConserveError::Overflow`, never a rounded approximation, so the result
cannot depend on the order legs were pushed in. Verified directly:
[`tests/exchange_conserve_test.rs`](../../tests/exchange_conserve_test.rs)'s
`a_mixed_batch_of_different_magnitudes_still_conserves` exercises a multi-leg
sum of varying magnitude and order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/003_conservation_that_adds_floats.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:894` | Pitfall in the source's Prompt 7 "Money" list |
