# Crate Responsibility: Event Drain

### Scope

- **Purpose**: State `exchange_event`'s one-line responsibility independent of its full crate writeup.
- **Responsibility**: Event drain — the one point workstream 010 reads fills/rejects/cancel-acks from.

### Statement

The crate's whole job, in one phrase: event drain. See `../crate/019_exchange_event.md` for this crate's build status.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1124` | Crate-responsibility 19, parallel to crate 19 (`exchange_event`) in Prompt 9 |
