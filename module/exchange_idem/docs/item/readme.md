# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `IdSet`, `IdemError`, `idem_seen`, `idem_insert`, `idem_remove`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the central summary (→ [`../../../../docs/crate/011_exchange_idem.md`](../../../../docs/crate/011_exchange_idem.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `IdSet` | `pub struct IdSet< K = OrderId > { /* private */ }` with `pub fn new() -> Self` | The set of keys already seen — `OrderId`s per book by default |
| `IdemError` | `pub enum IdemError { Duplicate }` | Why an id could not be inserted |
| `idem_seen` | `pub fn idem_seen<K: Hash + Eq>(set: &IdSet<K>, id: K) -> bool` | Whether `id` has already been seen |
| `idem_insert` | `pub fn idem_insert<K: Hash + Eq>(set: &mut IdSet<K>, id: K) -> Result<(), IdemError>` | Record `id` as seen, refusing a repeat |
| `idem_remove` | `pub fn idem_remove<K: Hash + Eq>(set: &mut IdSet<K>, id: K) -> bool` | Forget `id`, so a future resubmission is accepted again |

### Matches the proposal, plus a generic key

The source design's exposed-item list
(`core_exchange.txt:416-421` and `:592-595`, catalogued at
[`../../../../docs/exposed_item/011_exchange_idem_items.md`](../../../../docs/exposed_item/011_exchange_idem_items.md))
names `IdSet`, `idem_seen`/`idem_insert`/`idem_remove`, and
`IdemError { Duplicate }` — all built as named. `IdSet` is opaque (a private
`HashSet< K >`, built via `IdSet::new()`); the proposal names no fields for it.
The one addition is the generic key, defaulting to `OrderId`, so a caller that
assigns `OrderId` itself can key by `( AccountId, ClientOrderId )`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:416-421` | Crate 11 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:592-595` | Crate `exchange_idem`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
