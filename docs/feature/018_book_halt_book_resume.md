# Feature: book_halt, book_resume

### Scope

- **Purpose**: Freeze and unfreeze matching on one instrument.
- **Responsibility**: Toggle a book's halted state while its resting orders persist.

### Statement

`book_halt` freezes matching on an instrument without touching what's resting; `book_resume` lifts the freeze — the pair that implements hard problem 18's circuit breaker.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:326` | Feature 18 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1049` | Feature 18's English title, Prompt 9 |
