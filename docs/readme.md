# docs

Family-level material for workstream 002 that belongs to no single crate below.

## Source

[`../../../codename_space_sandbox/intake/core_exchange.txt`](../../../codename_space_sandbox/intake/core_exchange.txt) —
a chat transcript (Oct 1–2, 2026) running the same nine-prompt design pass already used
for workstream 006 (exact arithmetic), this time for workstream 002. Prompts 1–9 are
answered in full for 002; the transcript then pivots into workstream 010's own prompt 1
and stops there — 010 is out of scope for this corpus.

## Responsibility Table

| Directory | Responsibility |
|-----------|-----------------|
| [`workstream/`](workstream/readme.md) | The one workstream this corpus documents — 002, Exchange Core (1) |
| [`responsibility/`](responsibility/readme.md) | What 002 owns, in six one-line claims (6) |
| [`hard_problem/`](hard_problem/readme.md) | The 24 hard problems the crate split exists to solve (24) |
| [`feature/`](feature/readme.md) | The 30 capabilities the proposed crates specify (30) |
| [`boundary/`](boundary/readme.md) | What's in, out, and forbidden — three named instances (3) |
| [`neighbor_contract/`](neighbor_contract/readme.md) | What each of six neighboring workstreams supplies or consumes (6) |
| [`crate/`](crate/readme.md) | Per-crate purpose/boundary/dependencies for the 23 proposed crates, plus real-build status (23) |
| [`crate_responsibility/`](crate_responsibility/readme.md) | The one-line responsibility parallel to each of the 23 crates (23) |
| [`exposed_item/`](exposed_item/readme.md) | Per-crate exposed struct/enum/trait/fn/error surface, plus real-build status (23) |
| [`dependency_edge/`](dependency_edge/readme.md) | The 17 crate-to-crate dependency edges among the 23 (17) |
| [`dependency_tree/`](dependency_tree/readme.md) | The full proposed dependency tree, roots to facade (1) |
| [`mvp_subset/`](mvp_subset/readme.md) | The 16-crate build-first subset (1) |
| [`side/`](side/readme.md) | Bid/Ask — the two-value book side (2) |
| [`tif/`](tif/readme.md) | GTC/IOC/FOK — the time-in-force values (3) |
| [`stp_policy/`](stp_policy/readme.md) | Allow/CancelOldest/CancelNewest — the self-trade policies (3) |
| [`escrow_port/`](escrow_port/readme.md) | hold_try/hold_release/hold_commit — the three port operations (3) |
| [`reject_reason/`](reject_reason/readme.md) | The nine named reasons an order can be refused (9) |
| [`inbound_path/`](inbound_path/readme.md) | The four-step sequence from ring to rest-or-match (1) |
| [`ring_edge/`](ring_edge/readme.md) | Which `ring_*` crates `exchange_inbound` depends on, and which it must not (1) |
| [`phase/`](phase/readme.md) | The 30 incremental build phases, P01 through P30 (30) |
| [`smoke_demo/`](smoke_demo/readme.md) | One tiny demo per phase, plus the final wall (30) |
| [`golden_output/`](golden_output/readme.md) | The expected printed line for each phase's smoke (30) |
| [`pitfall/`](pitfall/readme.md) | 35 named mistakes to avoid while implementing (35) |
| [`smoke_quality/`](smoke_quality/readme.md) | The five qualities the wall-smoke demo must have (5) |
| [`decision/`](decision/readme.md) | Six settled design decisions for this workstream (6) |
| [`not_a_002_crate/`](not_a_002_crate/readme.md) | Five things explicitly outside this workstream's crate set (5) |
| [`research/`](research/readme.md) | Why this family builds its own book/match instead of an existing crate (1) |
| [`guide/`](guide/readme.md) | Orientation in the real, as-built crate family — purpose, usage, and dependency tier per crate (1) |

Directory names are the entity catalog's own names, verbatim, from Prompt 8
(`../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) — not the closest
`doc_des.rulebook.md` catalog names, and not the source's own prompt-heading phrasing
either. This is a deliberate departure from how the sibling workstream 006 corpus
(`../../exact/docs/readme.md`) named its collections (`crate/`, `type/`, `wall_smoke/`,
`scene/` — prompt headings): here, every one of the first 26 directories above matches
an entity name Prompt 8 lists, one-to-one, with no renaming. `research/` and `guide/`
are the two exceptions — non-catalog additions with no Prompt 8 counterpart, the same
way the sibling workstream 006 corpus also added its own `research/` on top of its
entity catalog. `research/` answers an external question (could an existing crate
replace this family?); `guide/` answers an internal one (what does this family actually
look like today, and how do I use it?) — see [`guide/readme.md`](guide/readme.md)'s own
"Why this collection exists" section.

## Workstream Charter

The transcript opens workstream 002 at `core_exchange.txt:118-133` as a direct answer to
the question "do we have such a workstream?":

> Yes. That workstream exists. It is 002 — Exchange Core, the matching engine.
>
> Demiurg is 001, the ECS virtual machine. It is not the matcher.
>
> 002 owns the price-time book: rest, cancel, replace, match, partial fills, escrow
> port, fills and rejects. It does not own wallets (that is 010) and it does not own
> decimal math (that is 006).

Its compile-time dependencies, from the same exchange (`core_exchange.txt:128-149`):

- **006 — Exact arithmetic.** Prices and quantities are minor units; without it the book
  becomes `f64`.
- **008 — Ring.** Only `exchange_inbound`: many producers, one drain, full order,
  `try_*`. The book and matcher never import `ring_*` directly.

Explicitly NOT a dependency: 001 (Demiurg — the book may later live in a resource, but
matching itself needs no ECS columns), 004 (time — priority is sequence, never wall
clock), 007 (sockets — orders arrive already drained), 010 (wallets — a *consumer* of
`Fill` and an *implementer* of `EscrowPort`, not something 002 depends on).

## Provenance — a second, more granular pass

The workstream readme at
[`../../../codename_space_sandbox/docs/workstream/002_exchange_core/readme.md`](../../../codename_space_sandbox/docs/workstream/002_exchange_core/readme.md)
already cites an earlier design pass ("message 879") naming a 4-crate breakdown —
`exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow` — and that is
exactly what the real build at [`../module/`](../readme.md) implements, plus the
`exchange_core` facade and the `smoke_exchange_core` demo lane (6 crates total) — see
the family's own [Responsibility Table](../readme.md) for the authoritative list; there
is no separate `module/readme.md` index file.

`core_exchange.txt` is a later, independent, far more granular pass: 23 crates instead
of 4, one responsibility per crate rather than four coarse ones. None of that extra
granularity has been built — the real family is still exactly the 6-crate shape message
879 described. This corpus documents the 23-crate proposal faithfully regardless, the
same way the sibling workstream 006 corpus documents its own superseded 15-crate
proposal in [`../../exact/docs/`](../../exact/docs/readme.md) after that one *was* built
in full. Here the gap is still open.

## Design status summary

Of the 23 proposed crates, 4 were built under their proposed name and shape
(`exchange_book`, `exchange_match`, `exchange_core`), one was built but with an
expanded scope and renamed methods (`exchange_escrow` — see
[`crate/012_exchange_escrow.md`](crate/012_exchange_escrow.md)), 9 were folded into the
real `exchange_types` crate, and 9 were not built at all: `exchange_tif`,
`exchange_spec`, `exchange_cap`, `exchange_idem`, `exchange_depth`, `exchange_halt`,
`exchange_snap`, `exchange_stats`, `exchange_inbound`. Two enum-level naming
corrections surfaced while building this corpus and are recorded in their own files
rather than here: real `Side` is `Buy`/`Sell` not `Bid`/`Ask`
([`side/001_bid.md`](side/001_bid.md)), and real `SelfMatchPolicy` is
`CancelResting`/`CancelIncoming`/`CancelBoth` with no "allow" option at all, hardcoded
to `CancelIncoming`
([`stp_policy/001_allow.md`](stp_policy/001_allow.md)). Full per-crate accounting is in
[`crate/`](crate/readme.md); full per-item accounting is in
[`exposed_item/`](exposed_item/readme.md).

## Relocation note

This corpus was built at `/home/user1/pro/lib/yrd_gamedev/substrate/exchange/docs/` while
this family was mid-relocation out of `codename_space_sandbox/substrate/exchange/` into
its own repository (same pattern as `../../exact/` and `../../ring/`). The family's own
root [`../readme.md`](../readme.md) still links to the workstream readme via a path
(`../../docs/workstream/002_exchange_core/readme.md`) that was correct only while this
repository was nested inside `codename_space_sandbox` — it now resolves two directories
short. Not fixed here since it's outside this corpus and the relocation is still in
progress elsewhere.
