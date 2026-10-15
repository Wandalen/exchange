# Smoke Quality: Easy to validate

### Scope

- **Purpose**: Judge a wall-smoke demo by how cheaply a human or CI can confirm pass/fail.
- **Responsibility**: One of the five qualities a `smoke_demo` is judged against.
- **In Scope**: A short, literal golden print that a diff can check mechanically.
- **Out of Scope**: Breadth of coverage (that is `001_coverage.md`'s concern).

### Statement

A smoke's result must be checkable by eye or by a short diff against a fixed golden block, not by re-deriving whether the behavior was correct from first principles. Prompt 4's golden print for `smoke_exchange_book` is exactly this: a dozen short lines of `key=value` pairs plus an `ok`, nothing the reader has to interpret.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1223-1228` | Prompt 9's `smoke_quality` instance list, item 2 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:754-770` | The golden print this quality is judged against |
