<!-- task_system_metadata
type: local
version: 1.0
crate: exchange_core
last_sync: 2026-08-26T00:00:00Z
-->

# Task Management - exchange_core

Local Task System for this crate — an index of this crate's own task files, scoped to this crate's own directory.

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | UnitType | Unit | Task | Purpose |
|-------|-----|--------------|-------|----------|--------|----------|-------|----------|----------|------|------|---------|
| 1 | [082](unverified/082_implement_exchange_core.md) | 378 | 7 | 3 | 9 | 2 | ❓ (Unverified) | any | module | substrate/exchange/exchange_core | Implement `exchange_core` — order/ledger types, self-match, cancel/amend, fee, escrow, event stream, and order lifecycle semantics | Leaf crate (`Cargo.toml` `[dependencies]` empty, zero real Cargo dependents confirmed via direct grep of every `module/*/Cargo.toml`); designed future edges to `demiurg` (ECS host) and `exact_arithmetic` (every price/balance), neither yet in any Cargo manifest; broadest doc corpus of any leaf crate filed so far — 11 doc instances across 6 doc-definition categories (algorithm×4, feature×1, invariant×3, pitfall×1, protocol×1, state_machine×1) driving 10 test files; unlike `exact_arithmetic`, no `type/` or `format/` doc instance exists and no `spike/exchange_core` directory exists in the workspace — the matching algorithm's own book representation is named Open/spike-first in every algorithm doc instance, so this task scopes to the semantics-level mechanisms (self-match, cancel/amend, fee, escrow, event stream, order lifecycle) that ARE decided, using an explicitly-flagged placeholder book and a simulated arrival sequence rather than inventing the deferred production design; ID 081 was found already claimed by a concurrent session's `shader_cache` task at write time, so this task allocated fresh 082 after re-reading the table immediately before filing, corroborated by a full-repository filesystem search finding no `082_*` or higher task file anywhere |
