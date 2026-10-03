# Smoke Quality: Hard work in crates

### Scope

- **Purpose**: Keep the demo itself thin by requiring every real mechanism to live in a crate, not in the demo binary.
- **Responsibility**: One of the five qualities a `smoke_demo` is judged against.
- **In Scope**: The demo only calls crate APIs and prints results.
- **Out of Scope**: Any matching, escrow, or book logic written inline in the demo.

### Statement

The smoke demo is meant to be a thin caller over the real crates, never a place where matching, escrow, or book logic gets reimplemented "just for the demo" — the same discipline the proposal's pitfall tape applies elsewhere (e.g. never reimplementing tick snap instead of calling 006). If a smoke needs logic that doesn't already exist in a crate, that is a sign the crate is missing something, not a reason to write it in the demo.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1223-1228` | Prompt 9's `smoke_quality` instance list, item 3 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:14` | Prompt 1's own framing: "Hard work stays in the crates. The demo stays small." |
