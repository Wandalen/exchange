# Exposed Item: exchange_idem

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_idem`.
- **Responsibility**: A seen-id set rejecting a repeated `OrderId` once per book.

**Design status**: Built as listed — `IdSet`, `idem_seen`/`idem_insert`/`idem_remove`, `IdemError { Duplicate }` — plus a generic key. Built-vs-proposed comparison: [`../../module/exchange_idem/docs/item/readme.md`](../../module/exchange_idem/docs/item/readme.md).

### Statement

Prompt 3 specifies a dedicated `IdSet` crate preventing any repeat `OrderId` on a book.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:592-595` | Crate `exchange_idem`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
