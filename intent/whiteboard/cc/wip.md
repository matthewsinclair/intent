---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-16 14:16Z
status: active
focus: "BANK 1 on hv's go (2026-09-16): 0411 and 0415 built in wt-wbstore (one store door, WbWrite: rows, updated_at and one wb.* event per verb in one transaction), wb tests green, whole suite running; 0414 (N) waits on vc's carry-versus-diff answer. Then bank 2 (0410, 0417, 0413). NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-16 localfold before hv's compact; HOLDING on hv's word).** 0331 (b) landed as train 8 at a7a31aa2f and 0331 is closed; vc verified it tree-exact. Next, under vc decision 21, two banks. BANK 1, store and audit: 0411 (J, wb writes absent from event_log), 0415 (O, updated_at kept by no writer), 0414 (N, sync --to-store's false "nothing overwritten"). One store door that stamps updated_at and appends a wb.* event in the same transaction, ops added to KNOWN_OPS (event.rs:105), and wb touch logs. BANK 2, stacked on bank 1: 0410 (H, register reads the found header and refuses a disagreement), 0417 (I, `wb register --correct`, contract in vc decision 21), 0413 (M, intent_claude_cwi:193 shows `--name "Control Claude" --role control` and names --correct). The code read, as at aa80e6917: `Store::append_event` store.rs:4041 and `write_event` :4061 (called inside a transaction at :3084, the pattern to reuse); every wb writer store.rs:4377-4687 sets no updated_at and logs nothing; `replace_boards` store.rs:4688 deletes all three wb tables and re-inserts without id or updated_at; `sync_overwrite` facade.rs:6969 diffs threads and issues only, so `store_restored` sync.rs:314 takes its 0 arm; `wb_register` facade.rs:5718 tests `hand_authored` and reads no header. **OPEN WITH vc, answer before building N:** WbNode, WbItem and WbMessage (model.rs:2233, 2267, 2290) carry no id or updated_at, so "replace_boards carries them through" means either (A) adding both to board.json and bumping BOARD_SCHEMA, or (B) replace_boards applying the difference by natural key, so an unchanged row keeps its id and updated_at and the same diff counts N's board differences; cc recommended (B). Worktree `wt-wbstore` in this session's scratchpad at aa80e6917, NO edits made, nothing to bank; HEAD has since moved, so recreate it at HEAD on resume. dc also touches facade.rs (migrate, doctor's L, pickup fix): the second of us to bank green proves on top of the first. Tell vc as each bank goes green. NO RELEASE, NO PUSH.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded at the 2026-09-15 wrap; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc). `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch. An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note, then sets the note to that text. To add a line, pass the old note plus the line. A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one: 0338 (i)'s landing took ST0057's gate from PASS to BLOCKED on exactly that until a header line named the row. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337); `at green --note` on a row already green is accepted and its text must contain the old note. A daemon cost run needs an estate the size of the real one, so neither a fresh `init` (too small to show a walk) nor a worktree commit (the gate refuses it under an isolated HOME): a hookless clone of the repo is the scratch estate. `git grep -E` has no `\b`: a word-boundary pattern matches nothing and exits like a clean miss, which read as `finish` having no callers until `git grep -F 'finish('` found every one. Search for a literal with `-F`. A script that runs `git -C <worktree> apply <patch>` resolves a relative patch path inside the worktree, not the caller's directory: rebase-0331b.sh reset wt-0331 and then could not open regen/0331b-counts.patch, so its apply ran by hand. Pass the patch path absolute.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
