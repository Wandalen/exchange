# verb

do-protocol verb scripts for the `exchange` repository, mirroring `../ring/verb/` and
`../exact/verb/`. The repository is one Cargo workspace (26 members), so every verb
here reaches the whole family directly.

**Parameter convention.** Every parameter is `key::val` (`level::3`, `crate::exchange_book`,
`dry::1`), never `--flag val`. Every verb rejects an unrecognized parameter loudly
(exit 2) rather than silently ignoring or mis-forwarding it. `dry::1` (where supported)
prints the command(s) the verb would run without running them.

| File | Responsibility |
|------|-----------------|
| `test` | Leveled full-suite verification, from `level::1` (nextest) through `level::5` (+doctests, clippy, udeps, audit, a clean rustdoc rebuild). Default `level::3`. Final verification only |
| `test_only` | Filtered nextest run by `filter::<substring>`, `crate::<name>`. Ordinary verification during development |
| `lint` | Clippy, warnings as errors. `crate::<name>` narrows to one package |
| `build` | Compile the workspace. `crate::<name>` narrows to one package |
| `doc` | Rebuild rustdoc from a clean slate (`rm -rf target/doc` first, because incremental `cargo doc` hides errors in unchanged crates) |
| `clean` | Remove `target/` and `-NNNN_*.log` scratch logs |
| `verify` | Full pre-push gate, an alias for `test level::5` |
| `verbs` | List all verbs with their purpose line |
| `publish_check` | Dry-run `cargo publish` for every publishable crate, in dependency order. `crate::<name>` narrows to one package |
| `publish` | Really publish every publishable crate to crates.io, in dependency order, retrying on crates.io's own rate limit |

`test` and `test_only` need `cargo-nextest`; `test` level 4 and up also needs a nightly toolchain with
`cargo-udeps` and `cargo-audit`. Like CI, every verb needs `../exact` and `../ring`
checked out next to this repo, at the revisions `.github/workflows/ci.yml` pins.

### Not carried over from ring and exact

- **`fmt`**: this project's codestyle forbids `cargo fmt`, the same as `exact`.
- **`gate`, `bench`**: this repo has no gate suite and no benchmark crate.
- **`package_info`**: nothing here reads it yet.
- **Per-crate `module/<crate>/verb/` wrappers and `_crate_dispatch`**: `verb/test_only crate::<name>`
  and `verb/lint crate::<name>` already narrow to one crate.
- **A shared argument-parsing helper**: each verb parses its own `key::val` parameters, as in `ring`.
  Standalone scripts stay diffable against `../ring/verb/`, which is how drift is caught.

### `publish`/`publish_check`, and the migration that made them possible

Every member used to set `publish = false`, which is why `publish_check` was on the list
above as not-yet-applicable. That line is now gone from the 24 real `exchange_*` crates —
`smoke_exchange_book` and `smoke_exchange_phases` keep it, same as `exact`'s own demo
binary stays private. Two more blockers had to clear first, both confirmed empirically
rather than assumed:

- **Every internal `{ path = "../exchange_x" }` dependency had no `version` requirement.**
  `cargo publish` refuses to publish a crate with an unversioned path dependency — confirmed
  directly: `cargo publish --dry-run -p exchange_inbound` first hit the `publish = false`
  gate, and after clearing that would have hit this one next. All 19 affected `Cargo.toml`
  files now carry `version = "0.1.0"` (every member's real current version) alongside
  every internal path dependency.
- **`ring_factory`/`ring_handle`/`ring_types` were pinned by git `rev` alone.** crates.io
  refuses a published crate with a git-only dependency. Confirmed `ring`'s own crates are
  already live at `0.2.1` (`curl` against `crates.io`'s own API), switched the three
  `[workspace.dependencies]` entries to `{ version = "0.2.1" }`, and reran the full
  workspace test suite to confirm `0.2.1`'s real API still matches what the pinned
  revision expected — 109 `test result: ok` blocks, 0 failures, same as before the switch.
  `exact_arith` stayed git-pinned at first, because `exact` itself was still mid-publish
  when this migration ran, and moved to `{ version = "0.1.0" }` once that release was live.

The pins have since gone back to upstream revisions, this time with the `version` kept
alongside: `{ version, git, rev }`. A local build uses the rev; `cargo package` drops `git`
and `rev` and publishes the `version` — confirmed: `cargo package -p exchange_inbound`
writes bare `version` requirements and its verify build compiles against crates.io. The
catch is that the published crate builds against the registry release, not the rev. Code
that uses an API only present at the rev builds here and fails for anyone installing from
crates.io; `verb/publish_check`'s dry-run verify build is the step that catches it.

Both verbs otherwise match `exact`'s own, including `publish`'s resume mechanism: a bare
retry of `cargo publish --workspace` is not itself resumable (confirmed on `exact`'s own
first real run — it published 5 crates, 429'd on the 6th, and the immediate retry died
re-attempting the 1st with `already exists on ... index`), so `publish` checks crates.io
directly before every attempt and `--exclude`s whatever is already live.
