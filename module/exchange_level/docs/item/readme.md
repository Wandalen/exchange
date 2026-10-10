# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `LevelNode`, `Level`, and the seven `level_*` functions.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the central summary (→ [`../../../../docs/crate/008_exchange_level.md`](../../../../docs/crate/008_exchange_level.md)).

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
| `level_qty_sum` | `pub fn level_qty_sum(level: &Level) -> Result<Quantity, KindError>` | The sum of every node's `remaining`, or why it does not fit |
| `level_empty_is` | `pub fn level_empty_is(level: &Level) -> bool` | Whether this level has no nodes left |

### Differs from the proposal

The source design's exposed-item list (`core_exchange.txt:572-577`,
catalogued at
[`../../../../docs/exposed_item/008_exchange_level_items.md`](../../../../docs/exposed_item/008_exchange_level_items.md))
names the same seven `level_*` functions, built as named, plus:

- **`Level { price, head, len }` / `LevelNode { order, next }`** — an
  intrusive list. Built as `price` plus a `Vec` of nodes instead, each node
  holding `order`, `remaining` and `arrival`; see the readme.
- **`LevelError { Full, Missing }`** — not built; see
  [`../decisions/001_no_level_error.md`](../decisions/001_no_level_error.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:398-403` | Crate 8 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:572-577` | Crate `exchange_level`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
