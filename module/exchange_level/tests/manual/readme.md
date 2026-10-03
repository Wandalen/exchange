# Manual testing plan — exchange_level

### Scope

- **Purpose**: Record the one compiler-invisible mistake class this crate's automated tests must actually catch, and the real, dated test that proved they do.
- **Responsibility**: Manual verification plan for `exchange_level`.
- **In Scope**: Mistakes that compile cleanly and could ship silently.
- **Out of Scope**: Ordinary type/logic errors already caught by `cargo test`/`cargo clippy` on every run.

### The risk: FIFO silently becomes LIFO

`level_push`/`level_pop_front` are a two-function contract: push adds the
newest arrival at the back, pop removes the oldest arrival from the front.
Swap which end each one touches — push at the front, pop from the back, or
vice versa — and the result is a fully valid, type-correct, compiling stack
instead of a queue. Nothing about the types catches this; only an assertion
on actual pop *order* across more than one node does.

### Real mutation test — 2026-10-03

1. Edited `src/lib.rs`'s `level_push` from `level.nodes.push( node )` to
   `level.nodes.insert( 0, node )` — newest arrival now lands at the front,
   turning the queue into a stack.
2. Ran `cargo test -p exchange_level` (`docs/-0022_level_mutation.log`).
   Result: 2 of 9 failed —
   `pushed_nodes_pop_in_the_order_they_arrived` and
   `removing_by_id_finds_a_node_wherever_it_sits` (the latter asserts the two
   survivors keep their original relative order after a middle removal). The
   other 7 — construction, emptiness, qty sum, absent-id removal, in-place
   front mutation — are insensitive to push/pop end and correctly kept
   passing.
3. Reverted to `level.nodes.push( node )`.
4. Re-ran `cargo test -p exchange_level` (`docs/-0023_level_revert_check.log`).
   Result: 9/9 passed again.

Confirms the suite fails loudly, and only on the tests whose job is exactly
this, when FIFO silently inverts to LIFO.

### Why no other mutation test

`LevelNode`'s three fields (`order: Order`, `remaining: Quantity`,
`arrival: Sequence`) and `Level`'s two (`price: Price`, `nodes: Vec<LevelNode>`)
are all distinctly typed — no same-typed-field-pair swap compiles, so there
is no second silent-transposition class to probe (matching `exchange_order`'s
own documented reasoning, [`../../../exchange_order/tests/manual/readme.md`](../../../exchange_order/tests/manual/readme.md)).
