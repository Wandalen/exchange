# dependency_edge

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:958-964`) names for workstream 002. The 17 crate-to-crate compile edges named in Prompt 9's `dependency_edge` list (`../../../../codename_space_sandbox/intake/core_exchange.txt:1136-1153`) — only crates with declared dependencies get an edge; the 6 roots (`exchange_id`, `exchange_side`, `exchange_tif`, `exchange_stp`, `exchange_seq`, `exchange_cap`) have none and are excluded.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_exchange_spec_edge.md`](001_exchange_spec_edge.md) | `exchange_spec` → `exchange_id`, 006 types |
| [`002_exchange_order_edge.md`](002_exchange_order_edge.md) | `exchange_order` → `exchange_id`, `exchange_side`, `exchange_tif` |
| [`003_exchange_idem_edge.md`](003_exchange_idem_edge.md) | `exchange_idem` → `exchange_id` |
| [`004_exchange_level_edge.md`](004_exchange_level_edge.md) | `exchange_level` → `exchange_order`, `exchange_seq` |
| [`005_exchange_book_edge.md`](005_exchange_book_edge.md) | `exchange_book` → `exchange_level`, `exchange_spec`, `exchange_id` |
| [`006_exchange_escrow_edge.md`](006_exchange_escrow_edge.md) | `exchange_escrow` → `exchange_id`, `exchange_order` |
| [`007_exchange_fill_edge.md`](007_exchange_fill_edge.md) | `exchange_fill` → `exchange_id`, `exchange_order` |
| [`008_exchange_conserve_edge.md`](008_exchange_conserve_edge.md) | `exchange_conserve` → `exchange_fill`, 006 types |
| [`009_exchange_rest_edge.md`](009_exchange_rest_edge.md) | `exchange_rest` → book, idem, cap, escrow, spec |
| [`010_exchange_match_edge.md`](010_exchange_match_edge.md) | `exchange_match` → book, escrow, fill, stp, tif, conserve |
| [`011_exchange_depth_edge.md`](011_exchange_depth_edge.md) | `exchange_depth` → `exchange_book` |
| [`012_exchange_halt_edge.md`](012_exchange_halt_edge.md) | `exchange_halt` → `exchange_spec`, `exchange_book` |
| [`013_exchange_event_edge.md`](013_exchange_event_edge.md) | `exchange_event` → `exchange_fill` |
| [`014_exchange_snap_edge.md`](014_exchange_snap_edge.md) | `exchange_snap` → `exchange_order`, `exchange_book` |
| [`015_exchange_stats_edge.md`](015_exchange_stats_edge.md) | `exchange_stats` → `exchange_id` |
| [`016_exchange_inbound_edge.md`](016_exchange_inbound_edge.md) | `exchange_inbound` → order, rest, match, event, ring_* |
| [`017_exchange_core_edge.md`](017_exchange_core_edge.md) | `exchange_core` → spec, book, rest, match, depth, halt, event, snap, stats, inbound |
