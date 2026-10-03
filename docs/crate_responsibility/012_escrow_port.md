# Crate Responsibility: Escrow Port

### Scope

- **Purpose**: State `exchange_escrow`'s one-line responsibility independent of its full crate writeup.
- **Responsibility**: Escrow port — hold/release/commit, without owning balances.

### Statement

The crate's whole job, in one phrase: escrow port. See `../crate/012_exchange_escrow.md` for this crate's build status — note the real crate's scope has expanded past a pure port.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1117` | Crate-responsibility 12, parallel to crate 12 (`exchange_escrow`) in Prompt 9 |
