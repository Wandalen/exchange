# Decisions

### Scope

- **Purpose**: Record architecture decisions for `exchange_core` that passed the Decision Gate — genuine open trade-offs with real switching cost — rather than being fixed in place directly.
- **Responsibility**: Index this crate's Architecture Decision Records (ADRs).
- **In Scope**: Proposed, accepted, and superseded ADRs for this crate's design.
- **Out of Scope**: The crate's existence and production-home role, which is `task/decisions.md` Q-10's decision; the hosting substrate, which is Q-07's; the intake concurrency mechanism, still an open question; the matching-engine architecture rulings already made at corpus grain.

Not a doc-definition collection — `docs/decisions/` is a non-doc-definition directory; ADRs here follow a standard Architecture Decision Record (ADR) format and are indexed only in this file, not in
`definition/readme.md` or `graph.yml`.

### Index

None yet. This crate's real open trade-offs — the hosting substrate and the
intake concurrency mechanism — are shared with whatever host eventually
integrates this engine, and aren't this crate's alone to decide.

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/exchange_core/docs/decisions
printf 'ADR instances:            '; ls [0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
# ADR instances:            0
```
