# Smoke Quality: Coverage

### Scope

- **Purpose**: Judge a wall-smoke demo by how much of the workstream's hard problems and features it actually exercises.
- **Responsibility**: One of the five qualities a `smoke_demo` is judged against.
- **In Scope**: Breadth of the scenario — how many distinct mechanisms it touches in one run.
- **Out of Scope**: How easy the result is to check (that is `002_easy_to_validate.md`'s concern).

### Statement

A wall smoke earns its name by covering most of the hard problems and features in one scenario rather than exercising just one path well — Prompt 4's own `smoke_exchange_book` proposal lists price-time, partials, TIF, STP, idempotency, halt, escrow, events, 006 types, ring drain, overflow, determinism, and snapshot rows as what a single scenario is expected to force.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1223-1228` | Prompt 9's `smoke_quality` instance list, item 1 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:749-750` | Prompt 4's "What it forces" list, the concrete coverage this demo claims |
