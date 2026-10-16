# script

The timed talk track. **A** presents sections 1–2, **B** sections 3–5; one
handover, at 4:30. Every slide has an id — the deck uses the same ids.

Budget: 8:00. Planned speech ends at 7:40; the last 20 s are buffer.

| Section | Slides | Time | Speaker |
|---------|--------|------|---------|
| 1. Real-world problem | `hook`, `exchange`, `promises` | 0:00–1:30 | A |
| 2. Why this solution | `fair`, `cents`, `lock`, `replay`, `claim` | 1:30–4:30 | A |
| 3. Why not an existing crate | `alternatives` | 4:30–5:10 | B |
| 4. Proof | `demo` | 5:10–6:40 | B |
| 5. Not built yet | `future`, `end` | 6:40–7:40 | B |

## Rules for the whole talk

- One running example: Alice and Carol sell apples, Bob and Dave buy them.
- Four domain words, each introduced once: *sell order*, *buy order*, *order
  book*, *match*. No other trading term reaches the screen or the speech.
- The crate map (see [Crate map](#crate-map)) sits on the side of every
  section 2 slide; each bug lights its boxes.

## 1. Real-world problem

### `hook` — 0:00–0:30 — A

**Screen**: a line chart. The index starts at 1000 in 1982 and drifts down to
524.811 on 25 November 1983; a second, dashed line shows the correct value,
1098.892. No other text than the axis labels and "Vancouver Stock Exchange
index".

**Say**: "In 1982 the Vancouver Stock Exchange started a new index at 1000.
It was recalculated after every trade, and every time, the last digit was cut
off instead of rounded. Twenty-two months later the index read 525. The
correct value was 1099. Nobody stole anything. A rounding rule did it."

### `exchange` — 0:30–1:05 — A

**Screen**: an order book ladder that builds up step by step, sell orders on
top, buy orders below. Steps: Alice's sell order appears; Carol's appears
behind it at the same price; Bob's buy order arrives, 10 move from Alice and 2
from Carol; Carol's 3 stay.

**Say**: "That was an index, not an exchange. But on an exchange the same
kind of bug moves real money. So what is an exchange? Alice has 10 apples and
wants a dollar each. She posts a sell order, and it waits in the order book.
Carol posts 5 more at the same price — she waits behind Alice. Bob wants 12
apples at a dollar. The exchange matches him: 10 from Alice, 2 from Carol.
That's the whole job: orders in, trades out."

### `promises` — 1:05–1:30 — A

**Screen**: four numbered cards.

1. **Fair** — first in line at a price trades first
2. **Exact** — no cent appears or vanishes
3. **Covered** — nobody trades money they don't have
4. **Replayable** — the same day replays the same way

**Say**: "Every exchange makes four promises. Fair: first in line trades
first. Exact: no cent appears or vanishes. Covered: nobody trades money they
don't have. Replayable: run the same day again, get the same trades. Let's
break each one."

## 2. Why this solution

Each bug slide: on the left, the naive code and what it does; on the right,
the fix in one line and the crate names; the crate map lights the fix's
boxes. Cards from `promises` stay visible as a strip on top, the current one
highlighted.

### `fair` — 1:30–2:10 — A

**Screen, naive**:

```rust
let mut level = vec![ alice, carol ];  // Alice arrived first
let maker = level.pop();               // takes the last one
// Carol trades first
```

**Screen, fix**: a first-in-first-out queue at each price, prices ranked best
first — `exchange_level`, `exchange_book`. Map lights **Wait in line**.

**Say**: "Promise one, fair. The obvious home for orders at one price is a
Vec: push when they arrive, pop when they match. But pop takes the last one —
Carol came after Alice and trades first. Fix: a first-in-first-out queue at
every price, and the book ranks prices best first. That's the waiting line on
our map."

### `cents` — 2:10–2:50 — A

**Screen, naive**:

```rust
let paid = 0.1_f64 * 3.0;   // 0.30000000000000004
assert_eq!( paid, 0.3 );     // panics
```

**Screen, fix**: exact decimals, and every trade must sum to zero across its
two sides or it is refused — `exact_arith`, `exchange_conserve`. Map lights
**Pay out**.

**Say**: "Promise two, exact. Put money in an f64 and 0.1 times 3 is
0.30000000000000004. Bob pays one amount, Alice receives another, and the
difference is money from nowhere — Vancouver again. Fix: exact decimal numbers
from our own exact_arith crate, and on every trade a check that what left one
account arrived in the other. If it doesn't sum to zero, the trade is
refused."

### `lock` — 2:50–3:30 — A

**Screen, naive** (labelled "sketch"):

```rust
let trades = book.match_order( bob_order );  // match first
for t in &trades { bob.cash -= t.cost; }     // pay after
// bob.cash == -8.00
```

**Screen, fix**: lock the buyer's money when the order arrives, before it can
match or wait — `exchange_escrow`. Map lights **Lock money**.

**Say**: "Promise three, covered. Bob has two dollars and orders ten apples.
The naive engine matches first and checks his wallet after: Alice's apples
are gone, Bob is at minus eight. Fix: lock Bob's money the moment his order
arrives, before it can match or wait. No money, no order. Remember this one —
we'll come back to it."

### `replay` — 3:30–4:15 — A

**Screen, naive**:

```rust
let arrival = Instant::now();  // who came first, by the clock
// run 1: Alice, Carol    run 2: Carol, Alice
```

**Screen, fix**: one intake that numbers every order on the way in and drains
in a fixed order — `exchange_inbound`, `exchange_seq`. Proof line from the
real smoke run: `a=0xda7a45e0d20c8fed b=0xda7a45e0d20c8fed`. Map lights
**Line up**.

**Say**: "Promise four, replayable. When something goes wrong, you replay the
day and you must get the same trades. Let the clock decide who was first, and
two orders that arrive together come out in a different order on each run.
Fix: every order gets a number at the door, through one intake that always
drains in the same order. We run the same day twice and hash the result: same
hash."

### `claim` — 4:15–4:30 — A

**Screen**: all four cards ticked; the whole crate map lit. One line: "24
crates · 0 dependencies from crates.io".

**Say**: "That's our engine: orders in, trades out, four promises kept and
checked on every trade. Twenty-four crates of Rust, no dependencies from
crates.io. *[B's name]* — why build it at all?"

## 3. Why not an existing crate

### `alternatives` — 4:30–5:10 — B

**Screen**: the table below. The **Covered** row is highlighted. Footnote:
"Surveyed 2026-10-02 — docs/research/001".

| Promise | `matchcore` | `orderbook-rs` | `limitbook` | ours |
|---|---|---|---|---|
| Fair | ✅ | ✅ | ✅ | ✅ |
| Exact | ✅ | ✅ | ✅ | ✅ |
| **Covered** | ❌ | ❌ | ❌ | ✅ |
| Replayable | ✅ | ✅ | ❓ | ✅ |
| *They beat us:* more order types | partial | ✅ | ❌ | ❌ |
| *They beat us:* edit an order in the book | partial | ✅ | ❌ | ❌ |

**Say**: "Fair question — there are open-source order books in Rust. We
compared the three closest. Fair, exact, replayable: mostly solved. Covered:
none of them. A general-purpose order book doesn't know about your wallet, so
there's nowhere to lock money before an order waits. That's the part we had
to build. And to be fair to them, they beat us twice: more kinds of orders,
and editing an order already in the book."

## 4. Proof

### `demo` — 5:10–6:40 — B

**Screen**: the order book ladder from `exchange`, now driven by `trace.js` —
a recording of the real engine's output, not a mockup. A balance panel per
person shows *free* and *locked*. B advances scene by scene with the arrow
key. Corner label: "recorded from exchange_core".

| Scene | What the engine does | Say |
|-------|----------------------|-----|
| `alice_sells` | Alice: sell 10 at 1.00 → waits in the book; her 10 apples move to locked | "This is our real engine, recorded. Alice posts her sell order. Her apples are locked — she can't sell them twice." |
| `carol_sells` | Carol: sell 5 at 1.00 → waits behind Alice | "Carol, same price, behind Alice." |
| `bob_buys` | Bob: buy 12 at 1.00 → matches 10 from Alice, 2 from Carol; Carol has 3 left | "Bob buys twelve. His twelve dollars were locked on arrival; now they move. Alice first, then Carol — fair." |
| `dave_broke` | Dave, holding 2.00: buy 10 at 1.00 → refused, `InsufficientFunds`; the book doesn't change | "Dave has two dollars and wants ten apples. Refused at the door — the book never sees him." |
| `proof` | Conservation: balanced. The whole day run twice: two equal hashes | "Every cent and every apple accounted for. And the whole day, run twice: same hash." |

## 5. Not built yet

### `future` — 6:40–7:30 — B

**Screen**: four rows, the first highlighted.

| What | Status |
|------|--------|
| **Restart after a crash** | The book can't be rebuilt from the log yet — our must-fix |
| Edit an order | Cancel works; edit doesn't yet |
| More markets | Only apples for dollars |
| Fees | We know where they'd be charged; nothing is charged |

**Say**: "What's missing. The big one: if the server restarts, we can't
rebuild the book yet, because setting up accounts and markets doesn't write to
the log. That's our must-fix. Then: you can cancel an order but not edit it;
there's one market, apples for dollars; and fees — we know where to charge
them, we don't yet."

### `end` — 7:30–7:40 — B

**Screen**: "Four promises, kept and checked." and the repository URL.

**Say**: "Four promises, kept and checked on every trade. Thank you."

## Crate map

Six boxes in the order an order passes through `exchange_step`. Plain label
first, crate names small underneath.

| Box | Pipeline step | Crates | Lit by |
|-----|---------------|--------|--------|
| **Line up** | ring intake | `exchange_inbound`, `exchange_seq` | `replay` |
| **Check** | validate | `exchange_spec`, `exchange_idem` | — |
| **Lock money** | reserve | `exchange_escrow` | `lock` |
| **Match** | match | `exchange_match` | `fair` |
| **Pay out** | settle | `exchange_conserve`, `exact_arith` | `cents` |
| **Wait in line** | dispose | `exchange_book`, `exchange_level` | `fair` |

## When over time

Cut in this order, after a timed rehearsal:

1. Merge `carol_sells` into `alice_sells` (saves ~10 s).
2. Drop `replay` and say its one-line fix on `claim` instead (saves ~35 s);
   the **Replayable** row leaves `alternatives` with it.

## Sources

- Vancouver index: [Szpiro, *Truncation and rounding*](https://www.georgeszpiro.com/18-truncation-rounding),
  [Arbogast, *Disasters due to rounding error*](https://web.ma.utexas.edu/users/arbogast/misc/disasters.html),
  [Floyd's 1985 draft](https://saildart.org/A112.TEX[106,RWF]1) — 524.811 vs 1098.892.
- Alternatives: [`../research/001_exchange_vs_open_source_alternatives.md`](../research/001_exchange_vs_open_source_alternatives.md).
- Not built yet: the readme's "Not yet built" table and its cold-recovery note.
