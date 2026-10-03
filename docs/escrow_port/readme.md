# escrow_port

One of the 26 doc entity kinds Prompt 8 (`../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`) names for workstream 002 — the three-method port a resting book calls against an escrow implementation.

**Design status** (collection-level): all three methods exist under different names on the real, concrete `exchange_escrow::Escrow` struct (`reserve`/`release`/`settle`) rather than as a trait named `EscrowPort` — see each instance file's own Design status, and `../boundary/002_out.md` for the related scope question.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_hold_try.md`](001_hold_try.md) | Lock funds before an order rests |
| [`002_hold_release.md`](002_hold_release.md) | Return a lock without a fill |
| [`003_hold_commit.md`](003_hold_commit.md) | Convert a lock into a settled transfer |
