# exchange_order — manual testing plan

No manual mutation check is needed here, for a reason worth recording rather
than leaving silent: every one of `Order`'s eight fields (`id`, `instrument`,
`account`, `side`, `price`, `quantity`, `tif`, `client`) has a distinct type. A Rust
struct literal is matched by field name, not position, so there is no
transposition the compiler would accept and a test would need to catch —
unlike `exchange_spec`'s `base`/`quote`, which are both `AssetId` and so share
exactly the failure mode a mutation test exists to prove is caught.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | None applicable | `Order`'s all-distinct field types make a same-typed-swap mutation impossible to construct; ordinary assertions in `tests/exchange_order_test.rs` cover every field's round-trip instead. |
