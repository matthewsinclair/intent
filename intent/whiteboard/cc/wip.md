---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 22:18Z
status: active
focus: "0331 (b) revising in wt-0331b, then onto HEAD in wt-0331, judged and banked; 0331 closes when it lands. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-15, compact fold after footer-ticks).** 0331 (a) landed at d843f7aea, the footer at 96ee3a4bb and the footer's prose source at 9cd639e46, each verified by vc against its bank; vc re-renders the views after each rebuild, and every node's next board pass carries a footer-only diff. **What remains in cc's lane is 0331 (b), the counts sweep, in wt-0331b (base 33990b80b).** A subagent finished a first pass (regen/0331b-counts.patch, regen/0331b-decisions.md), and cc reviewed every KEEP and sent it back to revise under vc's narrower rule (a figure stays only when the number is the design decision; tallies of a file's own arms, tests or rows, and enumerations named beside them, drop), with its eight questions answered (removed measurement tables go verbatim to regen/0331b-evidence.md for the commit message; placeholders `<n>` and `<n>/<m>`; a quotation keeps its figure; issues_surface.rs names the verbs it drives; table_driven_tests_fixture_their_home.rs:80-84 is the one body edit). **That agent was stopped part-way through the revision (hv, by accident) and the harness will not resume it, so cc finishes the revision by hand** from wt-0331b's current diff, which is the first pass plus whatever revision edits landed before the stop: regen/0331b-evidence.md is written, the patch and decisions log on disk are still the first pass's, and the agent's working files (finalize.py and magnitude.txt among them) are in regen/0331b/. A new agent only on hv's explicit ask. The KEEP list cc sent back as DROP-to-confirm is in this session's vc and agent messages of 2026-09-15; re-derive it from the decisions log's KEEP rows by the rule if the transcript is not at hand. Then: `bash regen/rebase-0331b.sh <patch>` carries it onto HEAD in wt-0331, whose target is warm and which holds exactly refs/bank/cc/0331-a (the script's precondition), with conflicts possible in tests/unit/guard_dispatch.bats and, since dc's 0321, in daemon_lifecycle.rs and a_daemon_outlives_nobody.rs; then `regen/judge-0331b.sh` (a comment-only check, the three crates whole, workspace clippy, rustfmt, the changed bats) and `regen/bank-0331b.sh` (refs/bank/cc/0331-b), and send vc the hash with each step's result. 0331 closes when (b) lands: its lander appends the close note, closes the issue and runs `organize --apply`, previewed as that one removal. NO RELEASE, NO PUSH.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded 2026-09-15; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc).
- **TWO MORE TRAPS (cc, 2026-09-15).** `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch.
- **THREE MORE TRAPS (cc, 2026-09-15).** An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note, then sets the note to that text. To add a line, pass the old note plus the line.
- **TRAPS FROM THE EVENING (cc, 2026-09-15).** A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one: 0338 (i)'s landing took ST0057's gate from PASS to BLOCKED on exactly that until a header line named the row. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337); `at green --note` on a row already green is accepted and its text must contain the old note. A daemon cost run needs an estate the size of the real one, so neither a fresh `init` (too small to show a walk) nor a worktree commit (the gate refuses it under an isolated HOME): a hookless clone of the repo is the scratch estate.
- **ONE MORE TRAP (cc, 2026-09-15).** `git grep -E` has no `\b`: a word-boundary pattern matches nothing and exits like a clean miss, which read as `finish` having no callers until `git grep -F 'finish('` found every one. Search for a literal with `-F`.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
