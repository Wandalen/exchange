# Decisions

### Scope

- **Purpose**: Record architecture decisions for `exchange_core` that passed the Decision Gate — genuine open trade-offs with real switching cost — rather than being fixed in place directly.
- **Responsibility**: Index this crate's Architecture Decision Records (ADRs).
- **In Scope**: Proposed, accepted, and superseded ADRs for this crate's design.
- **Out of Scope**: The crate's existence and production-home role, which is `task/decisions.md` Q-10's decision; the hosting substrate, which is Q-07's; the matching-engine architecture rulings already made at corpus grain.

Not a doc-definition collection — `docs/decisions/` is a non-doc-definition directory; ADRs here follow a standard Architecture Decision Record (ADR) format and are indexed only in this file, not in
`definition/readme.md` or `graph.yml`.

### Index

| ADR | Decision |
|-----|----------|
| [`001`](001_submit_replaced_by_ring_fed_exchange_step.md) | `submit` is deleted and replaced by a ring-fed `exchange_step`, not kept alongside it; `exchange_step` owns id/arrival sequencing; `Cancel` delegates to the existing `Exchange::cancel` unchanged; `Replace` drains but is not yet applied |
| [`002`](002_escrow_reached_through_escrow_port.md) | Escrow is reached through an `EscrowPort` trait defined in this crate, `Clone` as a supertrait for the dry run; `Exchange< E = Escrow >`, with `new`/`open_account` on `Exchange< Escrow >` only |

The intake concurrency mechanism this crate's trade-offs used to call "still
an open question" is resolved as of ADR-001: `exchange_inbound`'s ring, via
`exchange_step`. The hosting substrate beyond that ring remains Q-07's own
open question, not this crate's.

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_core/docs/decisions
printf 'ADR instances:            '; ls [0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
# ADR instances:            2
```
