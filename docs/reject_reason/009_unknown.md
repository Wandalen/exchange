# Reject Reason: Unknown

### Scope

- **Purpose**: Provide a catch-all reason for a refusal that does not fit any of the other eight named causes.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The fallback arm of the reject-reason closed set.
- **Out of Scope**: Standing in for a cause that should instead get its own named variant once it is understood.

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` now only re-exports it) has a completely different variant set and has no catch-all arm; every real variant names a specific cause.

### Statement

`Unknown` exists so the reject-reason type can stay a closed enum without a caller ever being handed a reject with no reason attached at all — a deliberate last resort, not a reason anyone should expect to see often. The real `RejectReason`'s own doc comment takes the opposite stance explicitly: "a closed set rather than a string... which reasons exist will grow," favoring new named variants over ever reaching for a catch-all.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 9 of 9 |
