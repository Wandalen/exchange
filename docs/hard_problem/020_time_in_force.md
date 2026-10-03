# Hard Problem: Time In Force

### Scope

- **Purpose**: GTC, IOC, FOK as data.
- **Responsibility**: Carry GTC/IOC/FOK as explicit order data rather than only ever resting forever.

**Design status**: Not implemented — confirmed directly in the real code's own doc comment at `exchange_types/src/lib.rs:218-220`: "Time-in-Force disposition is the other cause the family's design names; it is not implemented, so it is not listed here."

### Statement

Different in-game order types need GTC, IOC, and FOK as explicit data, or every order can only ever sit forever.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:282-285` | Hard problem 20 in the source's Prompt 1 answer for workstream 002 |
