# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `IdSet`, `IdemError`, `idem_seen`, `idem_insert`, `idem_remove`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the superseded central summary (→ [`../../../../docs/crate/011_exchange_idem.md`](../../../../docs/crate/011_exchange_idem.md), [`../../../../docs/exposed_item/011_exchange_idem_items.md`](../../../../docs/exposed_item/011_exchange_idem_items.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `IdSet` | `pub struct IdSet< K = OrderId > { /* private */ }` with `pub fn new() -> Self` | The set of keys already seen — `OrderId`s per book by default |
| `IdemError` | `pub enum IdemError { Duplicate }` | Why an id could not be inserted |
| `idem_seen` | `pub fn idem_seen<K: Hash + Eq>(set: &IdSet<K>, id: K) -> bool` | Whether `id` has already been seen |
| `idem_insert` | `pub fn idem_insert<K: Hash + Eq>(set: &mut IdSet<K>, id: K) -> Result<(), IdemError>` | Record `id` as seen, refusing a repeat |
| `idem_remove` | `pub fn idem_remove<K: Hash + Eq>(set: &mut IdSet<K>, id: K) -> bool` | Forget `id`, so a future resubmission is accepted again |

### Matches the proposal, plus a generic key

Verified directly against `core_exchange.txt:416-421` (crate 11, Prompt 2)
and `core_exchange.txt:592-595` (exposed-item list, Prompt 3) — not against
the central `docs/crate/011_exchange_idem.md`/`docs/exposed_item/011_exchange_idem_items.md`
summaries, which describe a now-superseded state ("not built as its own
crate") and are thinned to point here.

The proposal specifies `IdSet`, `idem_seen`/`idem_insert`/`idem_remove`, and
`IdemError { Duplicate }`. The real build has every one of them, under the
same names, with no omissions:

- `IdSet` is opaque (its `HashSet< K >` field is private, built via
  `IdSet::new()`) rather than the proposal's unspecified representation —
  not a divergence, since the proposal names no fields for `IdSet` to match
  or differ from.
- `IdSet` is generic over its key, defaulting to `OrderId` — an addition, so
  a caller that assigns `OrderId` itself can key one by
  `( AccountId, ClientOrderId )` instead.
- `idem_remove` exists so a legitimate cancel-then-resubmit under the same
  id is not indistinguishable from the retry this crate exists to refuse —
  named in the proposal's own exposed-item list, not an addition beyond it.

No `docs/decisions/` exists for this crate: there is no rejected
alternative to record when the built surface already matches what was
asked for.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:416-421` | Crate 11 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:592-595` | Crate `exchange_idem`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
