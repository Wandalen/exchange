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

`test` and `test_only` need `cargo-nextest`; `test` level 4 and up also needs a nightly toolchain with
`cargo-udeps` and `cargo-audit`. Like CI, every verb needs `../exact` and `../ring`
checked out next to this repo, at the revisions `.github/workflows/ci.yml` pins.

### Not carried over from ring and exact

- **`fmt`**: this project's codestyle forbids `cargo fmt`, the same as `exact`.
- **`gate`, `bench`**: this repo has no gate suite and no benchmark crate.
- **`publish_check`**: every member sets `publish = false`.
- **`package_info`**: nothing here reads it yet.
- **Per-crate `module/<crate>/verb/` wrappers and `_crate_dispatch`**: `verb/test_only crate::<name>`
  and `verb/lint crate::<name>` already narrow to one crate.
