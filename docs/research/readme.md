# Research Doc Definition

### Scope

- **Purpose**: Hold dated, sourced investigations into questions this family's design raises that aren't settled by reading its own code — starting with whether an existing open-source crate could replace it.
- **Responsibility**: Each instance is one self-contained research question, answered with external, checkable sources.
- **In Scope**: Comparisons against external crates/ecosystems, feasibility questions, and similar investigative work that informs but doesn't itself assert this workstream's own behavioral contract.
- **Out of Scope**: This workstream's own behavioral contract (→ [`../hard_problem/`](../hard_problem/readme.md), [`../feature/`](../feature/readme.md)); the 23-crate proposal's own per-crate detail (→ [`../crate/`](../crate/readme.md)).

**Design status**: one instance so far, dated 2026-10-02.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [exchange vs. Open-Source Alternatives](001_exchange_vs_open_source_alternatives.md) | Whether an existing crate could replace this family's book/match core | ✅ |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/research
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
