# Implementation - ST0056: Intent v3.0.0

## Implementation

v3 is built and released as 3.0.0 and then 3.0.1: the `intent` and `intentd` binaries in the Rust workspace at `native/rust/`, and the menubar app at `native/macos/`. Each spec (`design.md`, `data-model.md`, `migration.md` and the rest) states the as-built behaviour against its own design; `design.md` carries the stack as built, with provenance (Lamplight / Conflab). This document keeps the reference implementations the build drew on and the records below.

## Technical Details

Reference implementations trawled 2026-08-14:

- **Lamplight `native/cli`**: dispatch-spine SSOT (verb table feeding clap + daemon + generated agent guide), typed transport errors rendering remedies with full cause chains, `serde_ignored` inbound refusal, MCP stdio bridge with per-request target resolution.
- **Conflab `native/daemon`** (conflabd): async-graphql + axum + rmcp streamable HTTP, rusqlite index with SHA-256 skip-unchanged file sync (`db_sync.rs`), CLI-owned launchd lifecycle (`daemon_cmd/lifecycle.rs`), mgmt plane split from domain API, policy-stamp self-healing on boot, `notify-debouncer-full` + `ignore` watching, `serial_test` file_locks for HOME-mutating tests, Apple TN3171 cert policy (395-day leaves).

## Challenges & Solutions

(recorded as encountered)

## Rollback exercised on the canary -- AC-10.6 (cc, 2026-08-31 18:52Z)

**EXERCISED, NOT DOCUMENTED.** `AC-00.8` covers the documented procedure; this row is the one that required it run for real, and this section is the record `AT-10.6` asks for.

**Method, and why it was a clone.** vc ruled 2026-08-31 that _for real_ is opposed to SIMULATED, not to CLONED: a clone of this repository at its actual pre-migration revision, migrated by the actual migrator and reverted by actual `git`, exercises every real thing the criterion names. What a clone omits is three live peer sessions on the tree, and that is not evidence the criterion wants -- reverting the migration commit on the live estate would take it out from under them to prove something a clone proves better.

Revision read off `estate_corpus.sh list`, not chosen: the `canary` pin is `42fb5269`, recorded there as the last committed revision at which `intent/issues/` populates BOTH the OPEN and the CLOSED arm.

**What was driven.** Clone at `42fb5269` -- declares `2.19.0`, clean tree, a genuine v2 estate. `intent upgrade` (the tree binary) exited 0. Committed as one commit, then `git revert`.

| stage         | tree object                                |
| ------------- | ------------------------------------------ |
| pre-migration | `94620a034a679eee91a00bf8a258c1f74ec9a14c` |
| migrated      | `7473e639b6e9c33d6ef2357c7e6e267dfc4454dd` |
| post-revert   | `94620a034a679eee91a00bf8a258c1f74ec9a14c` |

**TREE-IDENTICAL, AND NOT VACUOUSLY.** The migrated tree differs from both, so the revert did real work rather than the assertion passing over a no-op. Working tree clean after the revert; the estate declares `2.19.0` again.

**The spec's other two claims, checked while the apparatus was standing.** `intent/.cache/intent.db` SURVIVES the rollback (present after the revert) -- the v3 store is left in place on rollback, and nothing deletes it. And `intent/.cache/` is gitignored, so the DB never entered history at all (D34).

**THE FORMULA REINSTALL WAS DELIBERATELY NOT EXECUTED, AND THAT IS A DECLARED EXCLUSION CARRYING ITS REASON** (vc ruled it explicitly, 2026-08-31). The documented rollback is `git revert` PLUS putting v2 back on PATH. Driven that day: brew carried `intent 3.0.0_1`, PINNED, and `intent` on PATH was `~/.local/bin/intent` symlinked to the tree's release binary, so restoring v2 would have taken the toolchain out from under every live session and the human's shell. The reinstall is about continuing to USE v2 afterwards; the tree-identical assertion this row asks for does not need it.

### THE FINDING THIS EXERCISE PRODUCED: there is no migration commit to revert

**`intent upgrade` WRITES THE MIGRATION AND DOES NOT COMMIT IT.** Measured: after a successful `rc=0` migration, `HEAD` was unchanged, the tree object was unchanged, and the migration's paths sat dirty, modified and untracked. Its closing line is `ok: this project is now Intent v<version> -- commit the canon and the generated views`, and 3.0.1 still behaves this way (`render.rs`, the `upgrade` arm).

That contradicted `migration.md`'s Phase B, which then promised one commit from the tool (it now says the operator commits), and it still contradicts `AC-00.8`, which says a project _migrates in one visible commit_.

**THE CONSEQUENCE IS PRECISE: the documented rollback has no subject.** `git revert <migration-commit>` presumes a commit the migrator does not create. The exercise above only has one because cc made it by hand, which is what an operator does -- so _one visible commit_ is an OPERATOR CONVENTION, not a migrator guarantee, and nothing enforces that the commit is one, or that it contains only the migration. It reads as satisfied because operators have happened to do it correctly.

Not changed through 3.0.1: whether the migrator should commit is a behaviour change and is hv's call.
