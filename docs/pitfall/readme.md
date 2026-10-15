# pitfall

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`)
names for workstream 002 — a specific mistake to avoid while implementing it, from
Prompt 7's "What to avoid" tape (`../../../../codename_space_sandbox/intake/core_exchange.txt:886-942`),
organized under 8 categories.

## Responsibility Table

### Money

| File | Responsibility |
|------|-----------------|
| [`001_f64_prices_or_quantities.md`](001_f64_prices_or_quantities.md) | Using `f64`/`f32` for any price or quantity |
| [`002_reimplementing_tick_snap.md`](002_reimplementing_tick_snap.md) | A local rounding routine instead of calling 006 |
| [`003_conservation_that_adds_floats.md`](003_conservation_that_adds_floats.md) | Summing conservation legs with float addition |
| [`004_dust_dropped_from_truncated_fill.md`](004_dust_dropped_from_truncated_fill.md) | Losing a sub-unit remainder to truncation |

### Book

| File | Responsibility |
|------|-----------------|
| [`005_hashmap_iteration_as_price_order.md`](005_hashmap_iteration_as_price_order.md) | Treating hash iteration as sorted price order |
| [`006_instant_or_systemtime_as_time_priority.md`](006_instant_or_systemtime_as_time_priority.md) | A wall-clock read instead of a claimed sequence |
| [`007_popping_later_order_at_same_price.md`](007_popping_later_order_at_same_price.md) | Serving a price level out of FIFO order |
| [`008_crossing_worse_level_while_better_has_qty.md`](008_crossing_worse_level_while_better_has_qty.md) | Matching a worse price while a better one has qty |
| [`009_replace_leaves_old_rest_in_place.md`](009_replace_leaves_old_rest_in_place.md) | Cancel-and-replace leaving the old order resting |
| [`010_cancel_frees_id_but_not_level_node.md`](010_cancel_frees_id_but_not_level_node.md) | Freeing an id while its book node still rests |

### Escrow

| File | Responsibility |
|------|-----------------|
| [`011_resting_before_hold_try_succeeds.md`](011_resting_before_hold_try_succeeds.md) | Resting an order before its hold is confirmed |
| [`012_match_that_credits_a_wallet.md`](012_match_that_credits_a_wallet.md) | The match loop writing a balance update itself |
| [`013_release_forgotten_on_cancel_ioc_fok.md`](013_release_forgotten_on_cancel_ioc_fok.md) | A hold left locked after its order no longer needs it |
| [`014_commit_and_release_of_same_hold.md`](014_commit_and_release_of_same_hold.md) | Applying both commit and release to one hold |
| [`015_saturating_add_hiding_insolvent_hold.md`](015_saturating_add_hiding_insolvent_hold.md) | A saturating add masking an insufficient hold |

### Match policy

| File | Responsibility |
|------|-----------------|
| [`016_fok_that_fills_part_then_rejects.md`](016_fok_that_fills_part_then_rejects.md) | An FOK order partially executing before rejection |
| [`017_ioc_that_rests_the_remainder.md`](017_ioc_that_rests_the_remainder.md) | An IOC order's leftover resting instead of cancelling |
| [`018_stp_applied_after_fill_emitted.md`](018_stp_applied_after_fill_emitted.md) | Self-trade check running after the Fill already emitted |
| [`019_self_trade_allowed_by_default.md`](019_self_trade_allowed_by_default.md) | Shipping without an explicit, named self-trade policy |

### Ring

| File | Responsibility |
|------|-----------------|
| [`020_ring_star_imported_by_book_or_match.md`](020_ring_star_imported_by_book_or_match.md) | The book or matcher depending on `ring_*` directly |
| [`021_match_claiming_slots_itself.md`](021_match_claiming_slots_itself.md) | The match loop claiming ring slots itself |
| [`022_drain_order_from_thread_completion.md`](022_drain_order_from_thread_completion.md) | Processing order taken from thread completion |
| [`023_park_recv_condvar_inside_exchange_step.md`](023_park_recv_condvar_inside_exchange_step.md) | Blocking the per-tick step on a wait primitive |
| [`024_ring_overflow_as_silent_drop.md`](024_ring_overflow_as_silent_drop.md) | A full ring silently discarding an order |
| [`025_ring_spsc_as_market_path.md`](025_ring_spsc_as_market_path.md) | A single-producer ring on a many-producer path |
| [`026_second_fill_ring_before_asked.md`](026_second_fill_ring_before_asked.md) | A speculative second ring for fills, built too early |

### Identity and cap

| File | Responsibility |
|------|-----------------|
| [`027_retry_inserts_second_rest.md`](027_retry_inserts_second_rest.md) | A resubmit creating a second resting order |
| [`028_full_drops_order_returns_ok.md`](028_full_drops_order_returns_ok.md) | A full book silently discarding an order |
| [`029_generationless_ids_reused_in_same_snapshot.md`](029_generationless_ids_reused_in_same_snapshot.md) | A cancelled id reissued within one snapshot's lifetime |

### Snapshot

| File | Responsibility |
|------|-----------------|
| [`030_snapshot_aliases_live_level_nodes.md`](030_snapshot_aliases_live_level_nodes.md) | A snapshot sharing storage with the live book |
| [`031_checksum_includes_hashmap_bucket_order.md`](031_checksum_includes_hashmap_bucket_order.md) | A determinism checksum depending on hash layout |
| [`032_tick_taken_from_instant_now.md`](032_tick_taken_from_instant_now.md) | Stamping a snapshot's tick from a live clock read |

### Process

| File | Responsibility |
|------|-----------------|
| [`033_p30_green_while_earlier_phases_red.md`](033_p30_green_while_earlier_phases_red.md) | Trusting the wall while an earlier phase smoke is red |
| [`034_escrow_stub_always_succeeds_no_failing_hold_phase.md`](034_escrow_stub_always_succeeds_no_failing_hold_phase.md) | The wall's always-succeeding stub masking a dead P15 |
| [`035_building_wallet_inside_this_stream.md`](035_building_wallet_inside_this_stream.md) | Building real wallet logic inside workstream 002 |
