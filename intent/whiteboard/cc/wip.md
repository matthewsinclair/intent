---
node: cc
name: Control Claude
role: control
session_id: 6551ed66-8c7f-4dbe-b512-eb5b65a0ca60
heartbeat_at: 2026-09-17 14:21Z
status: active
focus: "2026-09-17, localfolded for hv's compact (the second of the day): resume at the RESUME HERE doing item. Claims ST0076/05. The extractor shapes and the level-3 core are landed (e302be033, 648f1371a); the envelope is ic's WP-07; next is the Rust reader bank, drafted and banked as blobs under refs/bank/cc/st0076-wp05/. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

- **RESUME HERE (cc, 2026-09-17).** No claim. The Rust reader is in the installed pair (rebuilt by vc at ea81a6c58) and vc's drive of `intent index resolve --lang rust` on this estate came back current. Issue 0437 is closed (landed 1740ddfb6, closed 3da65b697). WATCHING: ic's WP-07 adds to the Resolver trait which manifest a reader resolves from, so a not-applicable language never reads `unresolved`, with Rust's side in rust_analyzer.rs beside cc's code (vc, 2026-09-17; ic tells cc); dc's WP-06 moves OPERATOR and is_operator into resolved.rs (vc decision 26). Next work comes from vc. NO RELEASE, NO PUSH.

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, localfold of 2026-09-17, the second of the day; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch. An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon: judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note. `intent issues edit --from` replaces an issue's whole prose too, so a close note is the old body plus the note. A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337). A daemon cost run needs an estate the size of the real one: a hookless clone of the repo is the scratch estate. `git grep -E` has no `\b`: search for a literal with `-F`. A script that runs `git -C <worktree> apply <patch>` resolves a relative patch path inside the worktree: pass it absolute. `sqlite3 -readonly` cannot open a WAL store that has no -shm beside it (error 14): read a store nothing writes as `file:<path>?immutable=1`. A CLI arm run from inside a Claude Code session inherits CLAUDE_CODE_SESSION_ID, and since 0433 a `wb pickup` with no `--session` records it, so an arm that asserts a session removes the variable. `rust-analyzer scip` has no switch that stops it running build scripts and proc macros (its `--config-path` settings change nothing, measured), without CARGO_TARGET_DIR it builds into the workspace's own target, and its cargo metadata step writes a Cargo.lock into a crate that has none. NEW 2026-09-17: a new verb row and new DDL text need the WHOLE suites, because targeted runs green across 263 arms missed three reds (the shipped-mutator roster in write_moves_only_what_changed.rs, and two no_pm_state_in_output arms); a tracker id (ST, WP, a ruling) never goes inside the DDL literal, which `intent schema ddl.sql` prints, so provenance belongs in the rung's Rust comment. The critic gate's IN-RS-CODE-004 proxy reads `Result<BTreeMap<String, String>, E>` in a turbofish as `Result<T, String>`: type the binding instead. `git reset --hard` in a worktree leaves a bank's NEW files behind as untracked, and `git apply --3way` then refuses them ("does not exist in index") and applies nothing until they are removed. A ref name may not end in `.lock`. cargo prints `Compiling`, not `Checking`, for a crate with a build script, so a gate script counting clippy members must accept both. A peer's git can hold `.git/index.lock` for a moment in the shared checkout: retry the commit once. A JUDGED PATCH-ID IS WHAT LANDS (vc, 2026-09-17): when the gate refuses a judged bank, stop, make the change in the worktree, re-bank, and send vc the new patch-id and the delta BEFORE committing, even for two lines.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
