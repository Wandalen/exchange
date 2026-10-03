# Exposed Item: exchange_idem

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_idem`.
- **Responsibility**: A seen-id set rejecting a repeated `OrderId` once per book.

**Design status**: Built, in its own `exchange_idem` crate — this summary predates that extraction. Matches the proposal exactly: `IdSet`, `idem_seen`/`idem_insert`/`idem_remove`, and `IdemError { Duplicate }` all exist under the same names. Verified built-vs-proposed comparison: [`../../module/exchange_idem/docs/item/readme.md`](../../module/exchange_idem/docs/item/readme.md).

### Statement

Prompt 3 specifies a dedicated `IdSet` crate preventing any repeat `OrderId` on a book. See the per-crate doc linked above — the real build has exactly this, in its own crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:592-595` | Crate `exchange_idem`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
