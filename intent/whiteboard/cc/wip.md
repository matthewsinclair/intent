---
node: cc
name: Control Claude
role: control
session_id: 6551ed66-8c7f-4dbe-b512-eb5b65a0ca60
heartbeat_at: 2026-09-17 12:40Z
status: active
focus: "2026-09-17, localfolded for hv's compact: resume at the RESUME HERE doing item. Claims ST0076/05. The extractor shapes are banked for the train with dc's WP-03 (patch-id 69d6653d); WP-05 is started, and its level-3 core is the next build, in worktree wt-l3core. NO RELEASE, NO PUSH."
claims: [ST0076/05]
---

# Control Claude (cc)

## DOING

- **RESUME HERE (cc, localfold for hv's compact, 2026-09-17).** Claims ST0076/05; vc claims ST0076 as director. (1) SHAPES, banked for the train with dc's WP-03: refs/bank/cc/st0076-extractor-shapes (blob f4b4991d4, patch-id 69d6653d, base c800b8319: @name.base, @self, @alias.as ignored as a row). They land first in the train on vc's word and carry two design.md facts: the row-vocabulary table's Rust cells as WP-02 built them, and the line saying rust-analyzer is not installed. (2) WP-05 LEVEL-3 CORE, the next build, in worktree wt-l3core (created off 90827b777, nothing written yet). vc's rulings Q1 to Q7 (vc decision 25) and hv's two of 2026-09-17 (the verb is `intent index resolve`; one needs_the_toolchain arm per language, ignored in CI, run by bin/devbin test all) are in WP-05's body. Bank the core alone at refs/bank/cc/st0076-l3-core: a `resolution` row per language (state, tool, path, line, detail, resolved_at, matched, unmatched, dropped, ambiguous), resolved rows unique on (path, line, name, target) with target_path and target_line, and per-file sha256, lang and run; rung 29; per-file replace; staleness by content hash; purge; the verb and its register row (exposed_on_mcp false, with an mcp_review note, as `index rebuild`); proven with an in-memory reader; PFIC. Register the module in intent/llm/MODULES.md first. dc stacks WP-06 on it. Then `index.resolution` in the envelope on WP-04's landing (ic's shape: keyed by language beside corpora; state missing, failed or stale; out of `complete`; target, target_path and target_line on SymbolFacts at level 3; symbol_note; the mcp drift test gains level 3). Then the SCIP decoder (pure, fields by number) and the Rust reader (build dir under intent/.cache). The landing that brings the Rust needs_the_toolchain arm also carries dc's devbin test option from refs/bank/dc/devbin-toolchain (dc banks it after the compact), so the arm is never unrun. Scratch drafts and scripts are banked as blobs under refs/bank/cc/st0076-wp05/, recovered with `git cat-file -p <ref> > <file>`. NO RELEASE, NO PUSH.

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, localfold of 2026-09-17; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch. An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon: judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note. `intent issues edit --from` replaces an issue's whole prose too, so a close note is the old body plus the note. A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337). A daemon cost run needs an estate the size of the real one: a hookless clone of the repo is the scratch estate. `git grep -E` has no `\b`: search for a literal with `-F`. A script that runs `git -C <worktree> apply <patch>` resolves a relative patch path inside the worktree: pass it absolute. `sqlite3 -readonly` cannot open a WAL store that has no -shm beside it (error 14): read a store nothing writes as `file:<path>?immutable=1`. A CLI arm run from inside a Claude Code session inherits CLAUDE_CODE_SESSION_ID, and since 0433 a `wb pickup` with no `--session` records it, so an arm that asserts a session removes the variable. `rust-analyzer scip` has no switch that stops it running build scripts and proc macros (its `--config-path` settings change nothing, measured), and without CARGO_TARGET_DIR it builds into the workspace's own target.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
