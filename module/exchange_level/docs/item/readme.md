# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `LevelNode`, `Level`, and the seven `level_*` functions.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the superseded central summary (→ [`../../../../docs/crate/008_exchange_level.md`](../../../../docs/crate/008_exchange_level.md), [`../../../../docs/exposed_item/008_exchange_level_items.md`](../../../../docs/exposed_item/008_exchange_level_items.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `LevelNode` | `pub struct LevelNode { order: Order, remaining: Quantity, arrival: Sequence }` | One resting order, what's left of it, and its arrival position |
| `Level` | `pub struct Level { price: Price, nodes: Vec<LevelNode> }` | Every order resting at one price, oldest-first |
| `level_new` | `pub fn level_new(price: Price) -> Level` | An empty level at `price` |
| `level_push` | `pub fn level_push(level: &mut Level, node: LevelNode)` | Add the newest arrival at the back |
| `level_pop_front` | `pub fn level_pop_front(level: &mut Level) -> Option<LevelNode>` | Remove and return the oldest arrival |
| `level_remove` | `pub fn level_remove(level: &mut Level, id: OrderId) -> Option<LevelNode>` | Remove the node with this id, wherever it sits |
| `level_len` | `pub fn level_len(level: &Level) -> usize` | How many nodes rest in this level |
| `level_qty_sum` | `pub fn level_qty_sum(level: &Level) -> Quantity` | The sum of every node's `remaining` |
| `level_empty_is` | `pub fn level_empty_is(level: &Level) -> bool` | Whether this level has no nodes left |

### Differs from the proposal

Verified directly against `core_exchange.txt:398-403` (crate 8, Prompt 2)
and `core_exchange.txt:572-577` (exposed-item list, Prompt 3) — not against
the central `docs/crate/008_exchange_level.md`/`docs/exposed_item/008_exchange_level_items.md`
summaries, which describe a now-superseded state (no `Level` type at all,
folded into `exchange_book`) and are thinned to point here.

The proposal specifies `Level { price, head, len }` / `LevelNode { order, next }`
— an intrusive singly-linked list — plus the same seven `level_*` functions
built here and a `LevelError { Full, Missing }`. The real build:

- Names all seven functions exactly as proposed, with identical signatures
  in substance (`level_remove` taking `OrderId`, `level_qty_sum` returning
  `Quantity`, etc.).
- Represents `Level`/`LevelNode` as `Price`/`Vec<LevelNode>` and
  `Order`/`Quantity`/`Sequence` rather than the proposal's `head`/`len`
  fields and `order`/`next` pointer-chain shape — Rust has no safe way to
  express a self-referential node chain without `unsafe`, `Box`, or
  `Rc<RefCell<_>>`, and nothing else in this family uses any of the three.
- Builds no `LevelError`. `Full` belongs to `exchange_cap::CapError::RestsFull`,
  checked before a node ever reaches a level; `Missing` is handled as
  `Option`, matching `exchange_book::Book::cancel`'s own precedent for "not
  found by id."

Full reasoning: [`../decisions/readme.md`](../decisions/readme.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:398-403` | Crate 8 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:572-577` | Crate `exchange_level`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
