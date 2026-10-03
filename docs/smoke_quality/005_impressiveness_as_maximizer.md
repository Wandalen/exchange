# Smoke Quality: Impressiveness as maximizer

### Scope

- **Purpose**: Rank visual/narrative polish last among the five qualities, never ahead of coverage or validatability.
- **Responsibility**: One of the five qualities a `smoke_demo` is judged against.
- **In Scope**: Polish as a tie-breaker once coverage, validatability, and smallness are already satisfied.
- **Out of Scope**: Trading away coverage or a clean golden print for a more impressive-looking run.

### Statement

Impressiveness is explicitly demoted to "a maximizer only" — something to improve once the other four qualities already hold, never a reason to accept a demo that covers less, is harder to validate, embeds real logic, or grows beyond one small file. This ordering is stated identically in Prompt 4's generic framing for every workstream, not something specific to 002.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1223-1228` | Prompt 9's `smoke_quality` instance list, item 5 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:14` | "Impressiveness is a maximizer only." |
