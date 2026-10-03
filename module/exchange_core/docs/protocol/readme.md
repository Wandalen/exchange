# Protocol Doc Definition

### Scope

- **Purpose**: Fix the message contract of what the engine emits, because a stream that is already archived cannot be renegotiated and recovery, audit, and every downstream consumer read the same records.
- **Responsibility**: Document `exchange_core`'s own protocols — the events a consumer is entitled to observe, and the rules an added field or kind must obey.
- **In Scope**: Message structure, message kinds, and version compatibility of the engine's output stream.
- **Out of Scope**: On-wire framing, which this crate does not own; on-disk encoding of conserved values (no successor doc — the old `exact_arithmetic` crate's transaction-log-encoding design was never carried into the real 16-crate [exact](https://github.com/Wandalen/exact) family); the input-side submission contract, which is a separate, upstream protocol this crate does not define.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Trade Event Stream](001_trade_event_stream.md) | The five emitted event kinds, the four fields common to all of them, and additive-only evolution with unknown-field skip | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/exchange_core/docs/protocol
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
