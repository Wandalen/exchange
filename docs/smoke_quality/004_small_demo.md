# Smoke Quality: Small demo

### Scope

- **Purpose**: Keep the demo file itself short enough to read in one sitting.
- **Responsibility**: One of the five qualities a `smoke_demo` is judged against.
- **In Scope**: One file, headless, no wallets/sockets/ECS per Prompt 4's own constraint on `smoke_exchange_book`.
- **Out of Scope**: The crates it calls into, which may be arbitrarily large.

### Statement

"Small" is a constraint on the demo file, not on the system it exercises — `smoke_exchange_book` is specified as headless, one file, with no wallets, sockets, or ECS involved, even though the scenario it drives (two ring producers, escrow, partials, TIF, STP, halt) is substantial.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1223-1228` | Prompt 9's `smoke_quality` instance list, item 4 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:732` | "Headless. One file. No wallets, no sockets, no ECS." |
