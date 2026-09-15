---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 21:24Z
status: active
focus: "0331 (a) banked green at refs/bank/cc/0331-a for train 5; the footer to prove on top of it; 0331 (b) in wt-0331b. 0321 and 0375 are dc's. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-15, after the third bounce).** 0321 and 0375 moved to dc (vc); cc's queue is 0331 (a), the footer and 0331 (b). **1. 0331 (a) is BANKED GREEN at refs/bank/cc/0331-a (fa0ca629d, base 90f482c1f)** for train 5 with ic's ST0075 WP-02, reported to vc with every judge step's result. It carries the dead artefacts with their rows, bats assertions and MODULES.md rows, the `claude rules` register row as-observed with the census and a regenerated view, the rules docs (rules.md's index-generator see-also line included, added after the judge with prettier and docs_completeness.bats), the routed stale comments and todo 13's doc-comment strings, spine.rs's scope sentence, the embed floor tied to DESTINATIONS, and ST0056's canon carrying the coverage_map.sh attachment. Stacking (vc): whichever of 0331 (a), ic's WP-02 (MODULES.md) and dc's 0375 (dispatch-table.json and its view) banks green first stacks first, and the second proves its bank on top, re-applying its rows and re-running prettier or the generator. The landing makes no verb store write: intentd ingests ST0056's canon and attachment, and the lander runs `at lint` and `ac gate` for ST0056 past that ingest. Open with vc: which of_n_population.sh roster sentence 817fa950b meant (read_claim_probe.sh's BIN was already fixed at a24428a79). **2. The footer (todo 13's `finish` strings, vc's ruling (i)): WIP at refs/bank/cc/0331-footer-wip.** Prove it on top of whatever 0331 (a) lands as (it applies on 90f482c1f plus (a)), judge it there, bank refs/bank/cc/0331-footer; it lands right before the rebuild after train 5, then `organize --apply` from the new pair and vc's views commit. **3. 0331 (b), the counts sweep, in wt-0331b (base 33990b80b) with a background subagent** writing regen/0331b-counts.patch and regen/0331b-decisions.md; if its notification is lost, the patch is `git -C <scratchpad>/wt-0331b diff --binary`. Review every KEEP first, carry it onto HEAD, then bank refs/bank/cc/0331-b. 0331 closes when (a) and (b) have landed. Every lander runs `intent at lint <ST>` and `intent ac gate <ST>` after its store writes and before its commit (vc).

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
