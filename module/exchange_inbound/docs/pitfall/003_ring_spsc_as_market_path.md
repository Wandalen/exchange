# Pitfall: ring_spsc as the market path (many producers)

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Never letting more than one producer write to a single ring, on a path that genuinely has many concurrent submitters.
- **In Scope**: `inbound_ring`'s backend selection and the one-ring-per-producer discipline it depends on.

### Statement

The exchange's inbound edge has many producers (every account submitting orders concurrently) feeding one drain — picking `ring_spsc` for that path violates its single-producer contract the moment a second submitter appears, with a failure mode that may not surface until concurrent load actually happens.

### How this crate avoids it

**Precision on what "avoids" means here, verified directly against
`ring_config`/`ring_core` source, not assumed:** [`inbound_ring`] calls
`RingConfig::new(capacity)` and only ever chains `.with_overflow(...)` —
never `.with_producers(...)`. `ring_config::RingConfig`'s own default is
`producers: 1`, and `ring_core::Ring::new` selects the `Spsc` backend
whenever `cfg.is_multi_producer()` (`producers > 1`) is `false` (confirmed
directly in both crates' own source — see Sources). So every ring
[`inbound_ring`] builds **is**, concretely, backed by `ring_spsc`. This
crate does not avoid the pitfall by requesting an `ring_mpsc`-backed ring
instead (it never does); it avoids the pitfall's actual *harm* — a single
ring corrupted by a second concurrent writer it was never built for — by
never letting that ring see a second producer at all.

That guarantee is structural, not a runtime check: `ring_handle::Producer`
has no `Clone`/`try_clone`, and `Ends::split` yields exactly one
`Producer`/`Consumer` pair per `Split` (confirmed directly in `ring_handle`'s
own source). "Many producers" is therefore built as many independent
single-producer rings — one [`inbound_ring`] call per producer — never as
one ring shared by two threads.
[`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)
records this choice directly, including why depending on `ring_core`/
`ring_mpsc` for a real cloneable producer was rejected (outside the family's
five-crate export Contract and Stage 8's own resolved dependency set).

**Verified**: `grep -n "with_producers" module/exchange_inbound/src/lib.rs`
returns zero hits, confirming the default (SPSC, one producer) is never
overridden. `smoke_exchange_phases`'s `demo_p28_drain.rs` races two real OS
threads, each against its *own* [`inbound_ring`]-built ring, demonstrating
the one-producer-per-ring discipline under genuine concurrent load rather
than only in prose.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/025_ring_spsc_as_market_path.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:924` | Pitfall in the source's Prompt 7 "Ring" list |
| `../../../../../ring/ring_config/src/lib.rs` | `producers: 1` default, `with_producers`, `is_multi_producer` |
| `../../../../../ring/ring_core/src/lib.rs` | `Ring::new`'s `Spsc`/`Mpsc` backend selection on `is_multi_producer()` |
| `../../../../../ring/ring_handle/src/lib.rs` | `Producer`'s real declaration — no `Clone`, `Ends::split` yields exactly one pair |
| [`../../src/lib.rs`](../../src/lib.rs) | Module doc's "Two producers without two threads on one ring" |
