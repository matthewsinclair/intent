# Implementation - ST0056: Intent v3.0.0

## Implementation

Nothing built yet -- WP-01 (design canon) is the gate before any Rust exists. This document records as-built state and deltas from `design.md` as the ladder executes; the stack shortlist with provenance (Lamplight / Conflab) lives in `design.md`.

## Technical Details

Reference implementations trawled 2026-08-14:

- **Lamplight `native/cli`** (15k lines): dispatch-spine SSOT (verb table feeding clap + daemon + generated agent guide), typed transport errors rendering remedies with full cause chains, `serde_ignored` inbound refusal, MCP stdio bridge with per-request target resolution.
- **Conflab `native/daemon`** (conflabd): async-graphql + axum + rmcp streamable HTTP, rusqlite index with SHA-256 skip-unchanged file sync (`db_sync.rs`), CLI-owned launchd lifecycle (`daemon_cmd/lifecycle.rs`), mgmt plane split from domain API, policy-stamp self-healing on boot, `notify-debouncer-full` + `ignore` watching, `serial_test` file_locks for HOME-mutating tests, Apple TN3171 cert policy (395-day leaves).

## Challenges & Solutions

(recorded as encountered)

## Rollback exercised on the canary -- AC-10.6 (cc, 2026-08-31 18:52Z)

**EXERCISED, NOT DOCUMENTED.** `AC-00.8` covers the documented procedure; this row is the one that required it run for real, and this section is the record `AT-10.6` asks for.

**Method, and why it was a clone.** vc ruled 2026-08-31 that _for real_ is opposed to SIMULATED, not to CLONED: a clone of this repository at its actual pre-migration revision, migrated by the actual migrator and reverted by actual `git`, exercises every real thing the criterion names. What a clone omits is three live peer sessions on the tree, and that is not evidence the criterion wants -- reverting the migration commit on the live estate would take it out from under them to prove something a clone proves better.

Revision read off `estate_corpus.sh list`, not chosen: the `canary` pin is `42fb5269`, recorded there as _the last committed revision at which `intent/issues/` populates BOTH arms (23 OPEN + 38 CLOSED)_.

**What was driven.** Clone at `42fb5269` -- declares `2.19.0`, clean tree, a genuine v2 estate. `intent upgrade` (the tree binary) rc=0: 56 threads, 61 issues, 136 files written. Committed as one commit, then `git revert`.

| stage         | tree object                                |
| ------------- | ------------------------------------------ |
| pre-migration | `94620a034a679eee91a00bf8a258c1f74ec9a14c` |
| migrated      | `7473e639b6e9c33d6ef2357c7e6e267dfc4454dd` |
| post-revert   | `94620a034a679eee91a00bf8a258c1f74ec9a14c` |

**TREE-IDENTICAL, AND NOT VACUOUSLY.** The migrated tree differs from both, so the revert did real work rather than the assertion passing over a no-op. Working tree clean after the revert; the estate declares `2.19.0` again.

**The spec's other two claims, checked while the apparatus was standing.** `intent/.cache/intent.db` SURVIVES the rollback (4.5MB, present after revert) -- `migration.md`'s _the v3 store is left in place on rollback; nothing deletes it_ holds. And `intent/.cache/` is gitignored, so the DB never entered history at all (D34).

**THE FORMULA REINSTALL WAS DELIBERATELY NOT EXECUTED, AND THAT IS A DECLARED EXCLUSION CARRYING ITS REASON** (vc ruled it explicitly, 2026-08-31). `migration.md` documents rollback as `git revert` PLUS reinstall the v2 formula. Driven: brew carries `intent 3.0.0_1` and it is PINNED, and `intent` on PATH is `~/.local/bin/intent` symlinked to the tree's release binary, so a v2 reinstall would take the toolchain out from under every live session and the human's shell. The reinstall is about continuing to USE v2 afterwards; the tree-identical assertion this row asks for does not need it.

### THE FINDING THIS EXERCISE PRODUCED: there is no migration commit to revert

**`intent upgrade` WRITES THE MIGRATION AND DOES NOT COMMIT IT.** Measured: after a successful `rc=0` migration, `HEAD` was unchanged, the tree object was unchanged, and 23 paths sat dirty (19 modified, 4 untracked). Its closing line is `ok: this project is now Intent v3.0.0 -- commit the canon and the generated views`.

That contradicts two places that assert otherwise. `migration.md` Phase B step 7 says _One commit, standard message naming the tool version and the artefact counts_, and `AC-00.8` says a project _migrates in one visible commit_.

**THE CONSEQUENCE IS PRECISE: the documented rollback has no subject.** `git revert <migration-commit>` presumes a commit the migrator does not create. The exercise above only has one because cc made it by hand, which is what an operator does -- so _one visible commit_ is an OPERATOR CONVENTION, not a migrator guarantee, and nothing enforces that the commit is one, or that it contains only the migration. It reads as satisfied because operators have happened to do it correctly.

Not repaired here: whether the migrator should commit is a behaviour change in a tag window and is hv's call.

**STALE, FLAGGED RATHER THAN FIXED:** this document's opening sentence still reads _Nothing built yet -- WP-01 (design canon) is the gate before any Rust exists_, which every paragraph above contradicts. Rewriting the as-built narrative is not this row's work; whoever owns it should know it is untrue at HEAD.
