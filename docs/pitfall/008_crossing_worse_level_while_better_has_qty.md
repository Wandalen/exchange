# Pitfall: Crossing a worse level while a better one still has qty

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Matching against a worse price while a better price on the same side still has quantity left.
- **In Scope**: Book

### Statement

A taker must exhaust every unit available at the best price before touching the next-best one; skipping ahead to a worse level while the better one is not yet empty hands the taker (or the resting side) a worse execution than the book actually offered.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:901` | Pitfall in the source's Prompt 7 "Book" list |
