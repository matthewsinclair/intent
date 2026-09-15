---
verblock: "2026-09-15:v1.70: vc - the close-out, held overnight at hv's wrap"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md`; what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Held overnight at hv's wrap (2026-09-15); each lane resumes at its own board's IN FLIGHT todo.
- cc: 0331 (b), the counts sweep, banked at `refs/bank/cc/0331-b` and judged green as train 8; it lands on vc's word and 0331 closes with it.
- dc: a devbin twin for IN-RS-CODE-001's CI clippy step, which `tests/unit/devbin_rust_gates.bats` holds red on main until it lands; then Decision A's record (the accent is steel `#35618f`) in `docs/design/design-system.md`.
- ic: ST0075 WP-02, banked green at `refs/bank/ic/st0075-wp02`, its dry landing re-run before the bank report; O4 (ruling 21), banked at `refs/bank/ic/o4` for vc's judgement; 0400 (ruling 20), banked at `refs/bank/ic/0400`, lands after WP-02; then WP-03 and ic's todo 20.
- vc: judging every bank, train and landing; after 0331 (b) lands, one rebuild of the pair, intentd's restart, `organize --apply` and one views commit (hv's board gains its `## Standing directives` section from 0375); issue 0402, the trailing newline a store filled from the tree drops, routed to a lane.

## TODO

- The quiet window, on hv's word: `bin/devbin build all`, `intent daemon restart`, `bin/int macos app-install`, ic drives the Console once with close-and-reopen, one mechanical view commit, then `st done` for ST0057 and ST0075 on gate PASS.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344: held until Intent installs and works properly on this machine, then gyges runs dc's runbook (hv, 2026-09-15, low priority).
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
