---
verblock: "2026-09-16:v1.73: vc - the lanes' queues are empty; the quiet window goes to hv"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md` and on hv's board (decision 18 adds devbin's whiteboard defects to this release); what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Each lane resumes when hv lifts its hold, at its own board's IN FLIGHT todo.
- Every lane's queue is empty. Open issues: 0344 (held for the clean install) and 0420 (the store write lock, whose capture runs in the quiet window).
- vc: taking the quiet window to hv; judging every bank, train and landing; the delivered pair is behind HEAD's code until the window's rebuild.

## TODO

- The quiet window, on hv's word: `bin/devbin build all`; with the daemon stopped, `intent sync --to-disk` normalising the thread and work-package sections 0402 made one form, checked by cc's `refs/bank/cc/0402-canon-check` and committed alone; `intent daemon restart`, `bin/int macos app-install`, `intent claude skills sync` (carries 0416's in-whiteboard skill edit), ic drives the Console once with close-and-reopen, one mechanical view commit, `intent/st/ST0056/parity/tools/gen_reference.sh` regenerating `docs/reference/` whole as its own commit (the close-out adds flags, eg `wb register --correct`), ic tells hv the hand test is ready (ST0075 WP-02 and WP-03, 0418, and 0419's cursor), and 0420's pre-write capture runs on the rebuilt pair where whiteboard writes are logged; then `st done` for ST0057 and ST0075 on gate PASS.
- The all-estate board trawl, straight after the quiet window (hv, 2026-09-16): dc runs a plain `intent wb migrate` read-only over every registered, unmigrated node in every estate, Conflab included (on v3 canonical since 2026-08-28), and vc sends hv one worklist of every refused unit; hv rules restructure or drop per unit or by class; dc carries, doctor runs, vc checks each estate, and each estate gets one mechanical commit in its own repo. NO PUSH.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344: held until Intent installs and works properly on this machine, then gyges runs dc's runbook (hv, 2026-09-15, low priority).
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
