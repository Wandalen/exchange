# Neighbor Contract: 006 Supplies Money, Qty, Price

### Scope

- **Purpose**: State exactly what workstream 006 (exact arithmetic) provides to 002.
- **Responsibility**: `Money`, `Qty`, `Price` types, the snap (tick/lot rounding) operation, and the conservation check.
- **In Scope**: Type and function supply only.
- **Out of Scope**: 002 never reimplements any of these.

**Design status**: Built. Every real `exchange_*` crate that needs 006's
types takes its Cargo edge on the single `exact_arith` facade crate, never on
006's own finer-grained leaf crates (`exact_kind`, `exact_snap`, `exact_cmp`,
`exact_conserve`) directly — confirmed by inspecting the `[dependencies]`
section specifically (not a blanket grep) of every `module/exchange_*/Cargo.toml`.
006 itself offers both shapes; `exact_arith` is documented on 006's own side
as "an optional facade if you would rather have one Cargo edge instead of the
four" leaves, and every one of 002's numeric-dependent crates took that
facade option uniformly.

### Statement

Workstream 006 is 002's only source of numeric types and the two operations that keep them honest: snapping a raw value to a legal tick/lot, and asserting that a set of fill legs conserves to zero. 002 calls these; it does not reimplement tick snapping or conservation logic itself (see `pitfall/` category "Money").

### Per-crate edge, as proposed vs. as built

A later, more granular pass (supplied directly by the user in this session,
2026-10-07; not part of `core_exchange.txt`) names which of 006's leaf crates
each 002 crate would use under the fine-grained route instead of the facade:

| 002 crate | Proposed leaf(s) | Real edge (verified) |
|-----------|-------------------|------------------------|
| `exchange_spec` | `exact_kind`, `exact_snap` | `exact_arith` (facade) |
| `exchange_order` | `exact_kind` | `exact_arith` (facade) |
| `exchange_level` | `exact_kind`, `exact_cmp` | `exact_arith` (facade) |
| `exchange_book` | `exact_cmp` | `exact_arith` (facade) |
| `exchange_fill` | `exact_kind` | `exact_arith` (facade) |
| `exchange_conserve` | `exact_conserve`, `exact_kind` | `exact_arith` (facade) |
| `exchange_depth` | `exact_kind` | `exact_arith` (facade) |
| `exchange_snap` | `exact_kind` | `exact_arith` (facade) |
| `exchange_escrow` | `exact_kind` | `exact_arith` (facade) |
| `exchange_rest` | none directly | none directly, confirmed — `exact_arith` appears only as a `[dev-dependencies]` entry (test fixtures) |
| `exchange_match` | none directly | **`exact_arith` directly, in `[dependencies]`** — the one confirmed divergence from the pasted claim. `module/exchange_match/src/lib.rs:101` reads `use exact_arith::{ KindError, Quantity };`, both re-exported from `exact_kind` beneath the facade (`exact_arith/src/lib.rs:88`) |

Crates confirmed to carry no `exact_*` edge anywhere in `[dependencies]`:
`exchange_id`, `exchange_side`, `exchange_tif`, `exchange_stp`, `exchange_seq`,
`exchange_cap`, `exchange_idem`, `exchange_event`, `exchange_stats`,
`exchange_rest`, `exchange_halt`, `exchange_inbound` — the last two each carry
`exact_arith` as a `[dev-dependencies]`-only entry (test fixtures), with zero
production edge, which still matches the pasted claim that neither imports
006 directly in the build that ships.

006 crates 002 uses nowhere, under either the facade or the granular route:
`exact_parse`, `exact_fmt`, `exact_bytes`, `exact_ratio`, `exact_dust`,
`exact_add`, `exact_sign`, `exact_round`, `exact_minor`, `exact_scale`. `i128`
is a Cargo feature on `exact_minor`, not a dependency — 002 does not enable it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1088` | First bullet of Prompt 9's `neighbor_contract` list |
| User-supplied specification, 2026-10-07 session | Per-crate 006-leaf breakdown; verified against every `module/exchange_*/Cargo.toml`'s `[dependencies]`/`[dev-dependencies]` sections and, for the one divergence, `module/exchange_match/src/lib.rs` directly |
