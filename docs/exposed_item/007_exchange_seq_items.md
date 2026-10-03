# Exposed Item: exchange_seq

**Design status**: Built as its own real crate — see
[`../../module/exchange_seq/docs/item/readme.md`](../../module/exchange_seq/docs/item/readme.md)
for the full as-built-vs-proposed listing.

(Superseded note: this file previously claimed "Folded into `exchange_types`"
with "No `seq_zero`/`seq_next`/`seq_cmp` functions exist" — both false now.
`exchange_seq` is a standalone crate with a real public `seq_next` function;
only `seq_cmp` and `SeqError` were actually declined, see
[`../../module/exchange_seq/docs/decisions/001_no_seq_cmp_or_seq_error.md`](../../module/exchange_seq/docs/decisions/001_no_seq_cmp_or_seq_error.md).)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:567-570` | Crate `exchange_seq`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
