---
verblock: "2026-09-14:v1.66: vc - the aggressive issue pass, second fold"
intent_version: 3.0.3
---

# Work In Progress -- the aggressive issue pass after v3.0.3

## DOING

- The aggressive issue pass (hv, 2026-09-14): every open issue is closed on evidence or fixed by a lane now. Remaining, in landing order, one lane on the tree at a time on vc's word: ic (0307's closure, the critic seam 0350+0329, 0334, 0339, 0396); dc (0393 alone, then 0394, 0315, 0323, 0316, 0320, 0397 as one group with ST0057's AC-04.2 row edit); cc (0338 (ii), 0338 (i), 0377, 0331's comments half; 0321 held for hv). The state at the fold is `intent/restart.md` line 3; the measure of what is open is `intent issues list`.
- ic's Intent.app Console thread ST0075: WP-01 `intent daemon logs` landed; WP-02 (the window) and WP-03 (the streaming items) after ic's issue batch; the app side hands the child a stdin pipe (AC-01.4); its release is hv's line (vc's lean: 3.1.0).
- hv's as-written against as-built pass over ST0056, ST0057 and ST0069 and their WPs (2026-09-13): ST0056's remaining attachments by owner (ic tui-design.md and parity/, dc install.md and migration.md); every fix through the CLI doors, never the code.

## TODO

- After the pass lands, in a quiet window on hv's word: `bin/devbin build all` (schema rung 27 migrates the live store on the new pair's first run; a 3.0.3 binary then refuses it), `intent daemon restart`, `bin/int macos app-install`; Gtools, Laksa and Utilz take today's fixes from the dev tree; the index watcher's event-wait arm gets a matched control by the index's owner (it redded in three of cc's four daemon runs today, more than its family).
- hv rules 0321: the daemon log's stamp collides with the one-clock invariant (a standalone `SELECT strftime` is banned and EXEMPT is hv's); cc's options are an INSERT-RETURNING confection, a real lifecycle table with a schema bump (M), an exemption for the daemon log with a stated reason (vc's lean), or no stamp.
- hv overrules or lets stand the ten rulings vc took under the pen on 2026-09-14: 0303, 0347, 0350, 0374, 0375, 0382, 0389 (the fold's seven), decision 16 (the register's State row transcribes the 2026-08-17 fold for Criterion.kind and AcceptanceTest.kind), decision 17 (Machine 5's green edge is red-first), 0334 (edit drops issue; set refuses Thread.acceptance).
- hv rules the five vc did not dole: 0177 (close, or keep as the constraint of decision 12), 0331's six deletions, 0338's contract rows (ST0057 is out of the line), 0345 (L, not in 3.0.x), 0344 (folds into the stranger-machine install).
- hv: `brew pin intent` (the keg is unlinked and not pinned); annotate the v3.0.2 GitHub release as superseded by v3.0.3, or delete it; the unpushed tap commit 9987a93 is superseded and stays unpushed.
- ST0056: a `brew install matthewsinclair/intent/intent` on a stranger machine, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- ic's explorer TUI list from hv: the `/issues` seam shipped in 3.0.3; the rest of the list is hv's to hand over, after ic's batch.
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- hv rules the provisional items ST0056's deferred list still carries: D43, D46, the `new-surface` scope line in parity.md, the withheld-13 field, and `ac gate`'s ratification (issue 0032).
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic).
- hv rules whether `intent claude upgrade --apply` regenerates the views it already knows are stale: the fleet sweep left four estates refused at their own gate until dc ran `intent sync --to-disk` in each.
- Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
