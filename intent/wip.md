---
verblock: "2026-09-16:v1.75: vc - 0426 before the rebuild, and the guards adoption pass beside the trawl"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md` and on hv's board (decision 18 adds devbin's whiteboard defects to this release); what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Each lane resumes when hv lifts its hold, at its own board's IN FLIGHT todo.
- dc: 0426 (a project's own hook guards declared as `guards` in config.json and run by the live gate), under vc's rulings of 2026-09-16. Open issues otherwise: 0344 (held for the clean install) and 0420 (the store write lock; its capture continues on every chain's store writes).
- vc: closing the quiet window with hv; judging every bank, train and landing.

## TODO

- The quiet window's remainder, after 0426 lands: hv runs `bin/devbin build all`, `intent daemon restart` and `bin/int macos app-install`, then drives `intent app restart` (0423), the Console's merged order (0425), `intent wb register` (0424) and a declared guard (0426); then `intent/st/ST0056/parity/tools/gen_reference.sh` regenerates `docs/reference/` whole as its own commit.
- The all-estate board trawl, on vc's word once the quiet window closes (hv, 2026-09-16): a plain `intent wb migrate` CARRIES FOR REAL on a board with nothing uncarried, so dc runs it only in scratch clones, one per estate (every registered estate, Conflab included, covering every board not yet in the store: a board with no registered node is registered in the clone first, so the worklist shows the real path, register then migrate): the store copied with `sqlite3 .backup` (never `cp`, every store is WAL), the estate's working `intent/whiteboard/` files copied over the clone so uncommitted board edits count, an isolated HOME and no daemon. dc records each node's exit status and its `uncarried:`, `coerced:` and `left in place:` lines, and names already-migrated nodes as skipped. vc sends hv one worklist; hv rules restructure or drop per unit or by class; dc carries in the live estates, doctor runs, vc checks each estate, and each estate gets one mechanical commit in its own repo. NO PUSH.
- The guards adoption pass, beside the trawl (vc, 2026-09-16, on 0426): per estate, declare the project's own guards in config.json `guards`, remove the calls it duplicates from the hand-wired or tracked hook, and run doctor clean; coordinated with devbin for the repos whose `.githooks/` and `bin/int` it manages (Intent, Gtools, Laksa); one commit per estate. NO PUSH.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344: held until Intent installs and works properly on this machine, then gyges runs dc's runbook (hv, 2026-09-15, low priority).
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
