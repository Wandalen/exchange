# Algorithm Doc Definition

### Scope

- **Purpose**: Document the procedures the matching engine commits to, at the semantics grain prior rulings decided — and, where those rulings state a prohibition without a policy, at the grain that makes it implementable — so the spike explores implementation rather than renegotiating behaviour.
- **Responsibility**: Document `exchange_core`'s own algorithms — decided steps in, open representation and policy choices named.
- **In Scope**: Matching semantics — order of matching, fill splitting, TIF disposition, event emission — plus the three decisions that ride on them: self-match resolution, cancel and amend, and where a fee is assessed.
- **Out of Scope**: Book data-structure choices and their performance (spike-first, → [`spike/readme.md`](../../../../spike/readme.md)); the audit that checks the results (→ [`exact_arith/docs/algorithm/`](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Price-Time Priority Matching](001_price_time_priority_matching.md) | The decided matching semantics — validate, reserve, match, split, dispose, emit | 🔄 |
| 002 | [Self-Match Prevention](002_self_match_prevention.md) | Three cancel policies with their leakage and denial-of-service surfaces; the ban is fixed, the policy configurable, the default open | 🔄 |
| 003 | [Cancel and Amend Semantics](003_cancel_and_amend_semantics.md) | Cancel is idempotent with a declared race winner; a quantity decrease keeps time priority, an increase and a price change forfeit it | 🔄 |
| 004 | [Fee Assessment Point](004_fee_assessment_point.md) | One assessment, at trade generation, against traded notional; maker/taker as a classification, with no rate fixed | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/substrate/exchange/exchange_core/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              4
# rows in Overview Table: 4
```
