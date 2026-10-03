# Feature Doc Definition

### Scope

- **Purpose**: Collect the version scope of the matching engine in one place, so a reader sees what a version covers without reassembling it from scattered design and corpus references.
- **Responsibility**: Navigate to the artifacts defining each version; never restate a fact another doc owns.
- **In Scope**: Version scope, deliberate exclusions, exit criteria, and cross-references to every artifact realising them.
- **Out of Scope**: Matching mechanics themselves (→ [`algorithm/`](../algorithm/readme.md)); the escrow property (→ [`invariant/`](../invariant/readme.md)); program-level tracking, out of this crate's own scope.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Exchange Core v0.1](001_exchange_core_v0_1.md) | First real version — order semantics committed, hosting ruled, exit criteria fixed | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/exchange_core/docs/feature
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
