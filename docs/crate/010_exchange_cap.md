# Crate: exchange_cap

**Design status**: Built as its own real crate, matching the proposal
exactly — see
[`../../module/exchange_cap/docs/item/readme.md`](../../module/exchange_cap/docs/item/readme.md).

(Superseded note: this file previously said "Not built"; `exchange_cap` was
since built with `BookCaps`, `CapError`, `cap_check_rest`, `cap_check_level`
all present field-for-field as proposed. Nothing in `exchange_book` calls the
checks yet — that caller-side wiring is a later stage, not a gap in this
crate's own surface.)

Purpose/boundary/dependency content moved to
[`../../module/exchange_cap/readme.md`](../../module/exchange_cap/readme.md).
Primary source: `../../../../codename_space_sandbox/intake/core_exchange.txt:410-415`
(Crate 10 in Prompt 2) and `:587-590` (Prompt 3 exposed-item list).
