# Golden Output: P30 — the wall

### Scope

- **Purpose**: The exact golden block `smoke_exchange_book` must print.
- **Responsibility**: The one block covering every scenario the wall exercises at once.

### Statement

The wall's golden print, reproduced in full:

```text
seed=1
fills=10@1.00,2@1.00
rest_bid=3@1.00
ioc_rest=0
fok_rej=1
dup=1
halt=1
stp_fill=0
overflow=1
conserve=0
depth=1.00:3,0.95:4
a=0x… b=0x…
ok
```

Pass requires every line above to match exactly, `conserve=0`, `a==b`, and the result equal to the fixture — this is the one block that must hold for the whole workstream to be considered closed.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:756-772` | The wall's golden print, from Prompt 4's English answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1218` | Consolidated in Prompt 9's `golden_output` list |
