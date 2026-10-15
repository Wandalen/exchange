# Pitfall: Replace that leaves the old rest in place

**Redistributed**: this pitfall is `exchange_rest`'s own — `rest_replace`
checks the replacement, cancels the original, then inserts — see
[`../../module/exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md`](../../module/exchange_rest/docs/pitfall/001_replace_leaves_old_rest_in_place.md)
for the ordering argument and its tests.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
