# ADR-001: No `LevelError` — `Full` and `Missing` Are Each Already Handled One Layer Over

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `LevelError { Full, Missing }`
alongside `Level`/`LevelNode` and the seven `level_*` functions
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:572-577`).
The real build declares no such type; `level_remove`/`level_pop_front`
return plain `Option< LevelNode >` instead.

## Decision

No `LevelError` type is declared. A level never refuses an insert on its
own account, and "not found by id" is `None`, not an `Err`.

## Alternatives Considered

### Option 1: Build `LevelError::Full` as a capacity check inside `level_push`

Rejected: a ceiling on how many orders may rest is `exchange_cap`'s concern
(`CapError::RestsFull`), enforced by the caller before a node ever reaches
`level_push`. `Level` itself has no notion of "too many" — it stores
whatever nodes it is given. Adding a capacity check here would
duplicate `exchange_cap`'s own check under a second name, or require this
crate to depend on `exchange_cap` for a limit that is the caller's
responsibility to enforce before calling, not this crate's to re-derive.

### Option 2: Build `LevelError::Missing` for `level_remove`/`level_pop_front`

Rejected: "remove an id that isn't present" is not a caller error in this
family — it is a race result, the same situation `exchange_book::Book::cancel`
already resolved by returning `Option` rather than inventing an error
variant ("the order may have filled or been cancelled already," per that
function's own doc comment). `level_remove` follows the identical
precedent rather than introducing a second vocabulary for the same
situation one crate over.

## Consequences

**Positive:** no error type to keep in sync with `exchange_cap`'s own
capacity errors as that crate's ceiling logic evolves; `level_remove`'s
signature matches `Book::cancel`'s already-established `Option` idiom,
so a caller familiar with one already knows the other.

**Negative:** a caller cannot distinguish "this level never had that id"
from "this level had it and it just got raced away" — both collapse to
`None`. Accepted: no code path in this family has ever needed that
distinction; `Book::cancel`'s own precedent made the same trade first.

## Related

- [`../../../../docs/exposed_item/008_exchange_level_items.md`](../../../../docs/exposed_item/008_exchange_level_items.md) — the source design's own exposed-item entry, naming `LevelError`
- [`../../../exchange_order/docs/decisions/001_no_order_mutation_or_error.md`](../../../exchange_order/docs/decisions/001_no_order_mutation_or_error.md) — the sibling decision on `OrderError`, same "handled one layer over" shape
