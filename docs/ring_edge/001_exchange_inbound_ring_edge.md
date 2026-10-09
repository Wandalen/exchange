# Ring Edge: `exchange_inbound`'s Permitted Ring Dependencies

### Scope

- **Purpose**: Name exactly which `ring_*` crates 002 may depend on, and which it must not, closing off an otherwise open-ended dependency surface.
- **Responsibility**: Six permitted crates, two explicit exclusions, and the rule that only `exchange_inbound` may hold this edge at all.
- **In Scope**: `ring_core` or `ring_handle`, `ring_tls`, `ring_flush`, `ring_batch` (for `drain_order`), `ring_overflow`, `ring_poll`.
- **Out of Scope**: `ring_spsc` (wrong topology — 002 has many producers, not one) and `ring_bench` (a benchmarking tool, not a runtime dependency). A later restatement (below) also names `ring_claim`, `ring_gating`, `ring_store`, and `ring_mpsc` as out of scope.

**Design status**: Built (`module/exchange_inbound`), but on three of the six named crates, not all six. The real dependency set is `ring_factory` (construction — `Factory.build::<T>(cfg)` returns the `Split<T>` this family's own module doc names as the only way `build` can be written, since `Split::ends`/`Ends::split` borrow and so cannot also be the thing `build` returns), `ring_handle` (the `Producer`/`Consumer` types `build` eventually yields), and `ring_types` (for `OverflowPolicy` — needed to request `Fail` explicitly, since `RingConfig::new`'s own default is `DropNewest`, which would make the P29 phase-smoke's required rejection silently unreachable). Confirmed by direct read of `ring_factory`/`ring_handle`/`ring_config`/`ring_types`' real source, not re-derived from this doc's own six-crate list.

Three of the original six are not depended on, each for a reason discovered while building, not asserted: `ring_tls` (thread-local staging) has no crate-level equivalent here — `inbound_flush` takes a plain `impl IntoIterator<Item = InboundCmd>` at its call boundary instead, so staging is the caller's concern, not a dependency of this crate. `ring_batch`'s `drain_order` guarantee is not depended on directly because `ring_handle::Consumer::drain()` already supplies the bounded, ordered drain this crate needs, one tier higher — `ring_batch` is `ring_handle`'s own dependency, not something a caller of `ring_handle` needs to also name. `ring_poll` is not needed because nothing here blocks or parks — every operation used (`try_push`, `try_push_batch`, `try_recv`, `drain`) is already non-blocking by construction; `ring_poll`'s whole purpose is guarding a tick path against an operation that *could* park, and there is no such operation here to guard. `ring_core` (the "or" alternative to `ring_handle`) is also not depended on — `ring_factory::Factory::build` already returns a `ring_handle::Split`, so there is no separate `ring_core::Ring` construction step this crate performs itself. `ring_overflow`'s turning-a-full-ring-into-a-`Reject` behavior is real and used, but reached through `ring_config::RingConfig::with_overflow(OverflowPolicy::Fail)` plus `ring_handle::Producer::try_push`'s `Err` return — a configuration value and a method result, not a direct `ring_overflow` dependency edge (that crate is `ring_core`'s own internal implementation detail of the policy, not part of the five-crate export Contract `ring_handle`/`ring_factory` sit on).

A later, more granular restatement (supplied directly by the user in this
session, 2026-10-07 — a third source, alongside the two `core_exchange.txt`
citations below) frames the permitted set as a five-crate "public door"
rather than the original six, with its own one-line purpose per crate:
`ring_factory` (build the MPSC ring), `ring_handle` (`try_push`,
`drain_order`, `Full`), `ring_tls` (stage before the barrier), `ring_flush`
(flush policy at the aeon edge), `ring_types` (`RingError`, capacity,
`OverflowPolicy::Reject`). This "door" states what `exchange_inbound` is
*permitted* to use, not a claim that all five are actually used — reconciled
against the verified real dependency set above, `ring_tls` and `ring_flush`
sit inside the permitted door but outside the three crates `exchange_inbound`
actually depends on (`ring_factory`, `ring_handle`, `ring_types`), consistent
with (not contradicting) the already-verified finding above that staging and
flush policy have no crate-level equivalent in the real code.

**The five-crate "door" framing is independently confirmed by the real ring
family's own documentation** (checked directly, 2026-10-07, per the user's
"ring is here — use it" instruction): `ring_factory/readme.md` states
verbatim "It is on the family's export Contract... This crate is **the
door**. A consumer reaches the SPSC and MPSC backends, and the registry,
*through* it rather than by importing `ring_spsc`, `ring_mpsc` or
`ring_registry` directly." `ring_flush/src/lib.rs`'s own module doc names the
same five by role: "`ring_factory` makes things, `ring_handle` and `ring_tls`
are things, `ring_types` is vocabulary" (and `ring_flush` itself, "the only
one that is a *decision*"). This is the real family's own self-description,
not merely the pasted restatement's claim — strong corroboration for the
door framing as a whole.

That direct check also surfaced two precise corrections to the pasted
restatement's per-crate detail, both confirmed by reading the real source
rather than asserted: **(1)** `ring_handle` has no method named `drain_order`
— its real method is `drain(&mut self) -> Drain<'_, 'a, T>`
(`ring_handle/src/lib.rs:227`); `drain_order` is `ring_batch`'s own internal
free function (`ring_batch/src/lib.rs:341`, `fn drain_order(claim:
&BatchClaim, capacity: Capacity) -> impl Iterator<Item = (Seq, SlotIndex)>`),
exactly the lower-tier primitive the paragraph above already says
`ring_handle::Consumer::drain()` supplies the caller-facing equivalent of.
**(2)** The real `ring_types` enum variant is `OverflowPolicy::Fail`, not
`OverflowPolicy::Reject` (`ring_types/src/policy.rs:105`) — "Reject" is
likely a conflation with `exchange_inbound`'s own higher-level vocabulary
("overflow becomes a reject", per `../crate/022_exchange_inbound.md`'s own
Purpose line), not the ring family's own name for the variant. Also
confirmed: `ring_handle::Producer::try_push` itself returns plain
`Result<(), T>` (`ring_handle/src/lib.rs:122`) — handing the record back on
failure — never a `RingError` value directly; `RingError::Full` is a
`ring_types`-level concept that `ring_handle`'s own source never references
by name (grepped directly), so "Full" reaches a caller of `ring_handle` as
an `Err(T)`, not as `RingError::Full` itself.

This restatement also widens the explicit exclusion list beyond
`ring_spsc`/`ring_bench` to name `ring_claim`, `ring_gating`, `ring_store`,
`ring_mpsc`, `ring_core`, `ring_poll`, `ring_batch`, and `ring_overflow`
together, under one unifying reason: each "has to be re-exported by
`ring_handle`" rather than depended on directly — the same conclusion the
paragraph above reaches crate-by-crate (`ring_batch`'s `drain_order` reached
through `ring_handle::Consumer::drain()`; `ring_overflow`'s policy reached
through `ring_handle::Producer::try_push`'s `Err`; `ring_core` never
constructed directly since `ring_factory::Factory::build` already returns a
`ring_handle::Split`), stated here as one general rule instead of four
specific ones. `ring_spsc` is additionally named "not the market path" (many
producers, one drain — matching this doc's own existing exclusion reason
above); `ring_bench`/`bench_harness` are confirmed not runtime dependencies.

Family-wide: no `exchange_*` crate other than `exchange_inbound` depends on
any `ring_*` crate — confirmed by the same `[dependencies]`-scoped Cargo.toml
inspection used throughout this corpus's 2026-10-07 session.
`exchange_core` may call into `exchange_inbound` at runtime (its facade
role — see `../crate/023_exchange_core.md`), but that is a call relationship,
not a Cargo edge: the `ring_*` dependency itself never leaves
`exchange_inbound`.

### Statement

002's entire relationship to workstream 008 is this one bounded edge, held by `exchange_inbound` alone: either `ring_core` or `ring_handle` as the composed ring type, plus `ring_tls` (thread-local staging), `ring_flush` (moving staged entries into the ring), `ring_batch` (specifically for its `drain_order`, the stable total order guarantee), `ring_overflow` (turning a full ring into a `Reject`), and `ring_poll` (non-blocking progress). `ring_spsc` is excluded by name because 002 has many producers, not one; `ring_bench` is excluded because it is a benchmarking harness, not something a runtime crate depends on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1194-1202` | Prompt 9's `ring_edge` instance, six permitted plus two excluded |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:80` | Earlier statement of the same edge: "exchange_inbound: ring_core or ring_handle, plus ring_tls, ring_flush, ring_batch, ring_overflow, ring_poll" |
| User-supplied specification, 2026-10-07 session | Third restatement: five-crate "public door" plus an eight-crate exclusion list unified under a "re-exported by `ring_handle`" rule; verified against every `module/exchange_*/Cargo.toml`'s `[dependencies]` section |
| `../../../ring/ring_factory/readme.md:9-13` | Ring family's own words: "This crate is the door" — independent confirmation of the five-crate door framing |
| `../../../ring/ring_flush/src/lib.rs:1-9` | Ring family's own module doc naming the same five crates by role ("`ring_factory` makes things, `ring_handle` and `ring_tls` are things, `ring_types` is vocabulary") |
| `../../../ring/ring_handle/src/lib.rs:122,227` and `../../../ring/ring_batch/src/lib.rs:341` | Direct source read correcting `drain_order` to `drain` as `ring_handle`'s real method name |
| `../../../ring/ring_types/src/policy.rs:98-106` | Direct source read correcting `OverflowPolicy::Reject` to the real variant, `OverflowPolicy::Fail` |
