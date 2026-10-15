# Phase: P21 — Worse price stays untouched

### Scope

- **Purpose**: Prove price priority holds even when a better level still has quantity.
- **Responsibility**: A worse-priced resting order is never touched while a better-priced one is still alive.

### Statement

The twenty-first phase's one new contract: a worse price (e.g. 0.95) is left completely untouched as long as a better price (e.g. 1.00) still has quantity to give — price priority is never skipped ahead of.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:802` | Phase P21 in the source's Prompt 5 answer for workstream 002 |
