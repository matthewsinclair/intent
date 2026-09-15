---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 21:49Z
status: active
focus: "the footer's prose source judging in wt-footer for refs/bank/cc/footer-ticks; 0331 (b) revising in wt-0331b, then onto HEAD; 0331 closes when (b) lands. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-15, after train 5).** 0331 (a) landed at d843f7aea and the footer at 96ee3a4bb, each verified by vc against its bank, and the views re-rendered at 3ee36a6f2. 817fa950b's of_n_population.sh item is struck (vc). **1. The footer's source as prose (vc's XS): `finish` prints its source phrase without backticks**, since every caller passes a phrase (the thread canon, the issue canon, the whiteboard model). Built in wt-footer on 3ee36a6f2 and judged by regen/judge-ticks.sh, which creates refs/bank/cc/footer-ticks only if every step is green; send vc the hash with each step's result. It lands right before dc's 0321 rebuild and shares that views commit; the whiteboard views re-render at each node's next board write. **2. 0331 (b), the counts sweep, in wt-0331b (base 33990b80b).** The subagent's first pass is in (regen/0331b-counts.patch, regen/0331b-decisions.md); cc reviewed every KEEP and sent it back to revise under vc's narrower rule (a figure stays only when the number is the design decision; tallies of a file's own arms, tests or rows, and enumerations named beside them, drop), with its eight questions answered (removed measurement tables go verbatim to regen/0331b-evidence.md for the commit message; placeholders `<n>` and `<n>/<m>`; a quotation keeps its figure). When it reports: review the revised KEEPs, carry the patch onto HEAD (only tests/unit/guard_dispatch.bats also moved on main), judge it whole in a worktree, bank refs/bank/cc/0331-b and send vc the hash with each step's result. 0331 closes when (b) lands: the lander appends the close note, closes the issue and runs `organize --apply`. NO RELEASE, NO PUSH.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded 2026-09-15; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc).
- **TWO MORE TRAPS (cc, 2026-09-15).** `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch.
- **THREE MORE TRAPS (cc, 2026-09-15).** An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note, then sets the note to that text. To add a line, pass the old note plus the line.
- **TRAPS FROM THE EVENING (cc, 2026-09-15).** A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one: 0338 (i)'s landing took ST0057's gate from PASS to BLOCKED on exactly that until a header line named the row. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337); `at green --note` on a row already green is accepted and its text must contain the old note. A daemon cost run needs an estate the size of the real one, so neither a fresh `init` (too small to show a walk) nor a worktree commit (the gate refuses it under an isolated HOME): a hookless clone of the repo is the scratch estate.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
