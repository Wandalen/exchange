# Pitfall: Replace that leaves the old rest in place

**Redistributed**: this pitfall is `exchange_rest`'s own concern (its
`rest_replace` — cancel old, insert new, roll back on refusal — is exactly
the operation this pitfall is about) — see
[`../../module/exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md`](../../module/exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md)
for the full writeup, including the ordering argument and the test/mutation
evidence it rests on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
