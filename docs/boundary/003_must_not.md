# Boundary: Must Not

### Scope

- **Purpose**: Enumerate the hard prohibitions for 002's implementation.
- **Responsibility**: Five things the implementation must never do, regardless of expedience.
- **In Scope**: f64; wallet-crediting matches; HashMap-ordered price iteration; blocking calls inside match; ring imports inside book/match.
- **Out of Scope**: N/A.

**Design status**: Verified held. `exchange_core` depends on `exact_arith` (no f64), and zero `ring_*` dependency exists anywhere in the family (grep across all `module/*/Cargo.toml` confirms this) — so the ring-import prohibition is vacuously satisfied (nothing imports ring at all yet, including on the inbound path this boundary permits).

### Statement

Five prohibitions carry over directly from the pitfall tape (`pitfall/`) into a hard boundary: no `f64` anywhere in price or quantity; a match must never itself credit a wallet (that is 010's job from a `Fill` event); price order must never be derived from `HashMap` iteration order; nothing inside the match loop may block (`recv`, `park`); and `ring_*` must never be imported by `exchange_book` or `exchange_match` specifically — only `exchange_inbound` may touch the ring.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1080-1085` | Prompt 9's `boundary (must not)` list, five items |
