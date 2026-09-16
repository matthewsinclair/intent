---
verblock: "2026-09-16:v1.71: vc - the close-out resumed; devbin's whiteboard defects join it"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md` and on hv's board (decision 18 adds devbin's whiteboard defects to this release); what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Each lane resumes when hv lifts its hold, at its own board's IN FLIGHT todo.
- cc: bank 1, 0411, 0415 and 0414 (whiteboard writes logged and stamped, and `sync --to-store` applying the whiteboard difference by natural key), under vc decisions 21 (2) and 22; then bank 2, 0410, 0417 and 0413 (the identity door), under vc decision 21 (1); then 0402, the trailing newline a store filled from the tree drops.
- dc: `wb migrate` made loud under vc decision 20, 0403, 0404 and 0406-0409; then 0412 and 0416 (the pickup gap: a peer's board reaches pickup whole).
- ic: ST0075 WP-02 with the Theme.swift comments Decision A made false, rebanked and its dry landing re-run before the bank report; then O4 (ruling 21) and 0400 (ruling 20), whose two test files are shared with the landed 0331 (b), so it re-runs the whole suite before its bank report; the `/projects` cursor starting on the open project, when hv says.
- vc: judging every bank, train and landing; the delivered pair is behind HEAD's code until the quiet window's rebuild.

## TODO

- The quiet window, on hv's word: `bin/devbin build all`, `intent daemon restart`, `bin/int macos app-install`, ic drives the Console once with close-and-reopen, one mechanical view commit, then `st done` for ST0057 and ST0075 on gate PASS.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344: held until Intent installs and works properly on this machine, then gyges runs dc's runbook (hv, 2026-09-15, low priority).
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
