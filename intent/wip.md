---
verblock: "2026-09-16:v1.74: vc - the quiet window's remainder, and a trawl that is read-only by construction"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md` and on hv's board (decision 18 adds devbin's whiteboard defects to this release); what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Each lane resumes when hv lifts its hold, at its own board's IN FLIGHT todo.
- Every lane's queue is empty. Open issues: 0344 (held for the clean install) and 0420 (the store write lock; its capture continues on every chain's store writes).
- vc: closing the quiet window with hv; judging every bank, train and landing.

## TODO

- The quiet window's remainder: hv runs `bin/devbin build all`, `intent daemon restart` and `bin/int macos app-install` so the pair carries 0421 and 0422, and drives both (an in-place field edit shifts nothing at its cursor; with the detail pane split, the selected row at the foot of a long list stays painted); then `intent/st/ST0056/parity/tools/gen_reference.sh` regenerates `docs/reference/` whole as its own commit.
- The all-estate board trawl, on vc's word once the quiet window closes (hv, 2026-09-16): a plain `intent wb migrate` CARRIES FOR REAL on a board with nothing uncarried, so dc runs it only in scratch clones, one per estate (every registered estate, Conflab included): the store copied with `sqlite3 .backup` (never `cp`, every store is WAL), the estate's working `intent/whiteboard/` files copied over the clone so uncommitted board edits count, an isolated HOME and no daemon. dc records each node's exit status and its `uncarried:`, `coerced:` and `left in place:` lines, and names already-migrated nodes as skipped. vc sends hv one worklist; hv rules restructure or drop per unit or by class; dc carries in the live estates, doctor runs, vc checks each estate, and each estate gets one mechanical commit in its own repo. NO PUSH.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344: held until Intent installs and works properly on this machine, then gyges runs dc's runbook (hv, 2026-09-15, low priority).
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
