---
verblock: "2026-09-14:v1.63: vc - the cut is ready on 4c172d260; 0355 closed on the verdict"
intent_version: 3.0.1
---

# Work In Progress -- v3.0.2

## DOING

- The cut, hv at the terminal, nothing batched, each its own approval: the delivered set is 4c172d260 (intent 4d3b8fa9, intentd b0db7ab5, Intent.app build 7052, doctor 0, store at schema 26, daemon 16233 under vc's watch); `bin/devbin build release --patch`, `build all`, `int macos prepare`, `build formula`, `build publish`, `build smoke --reinstall`; then ic's reference regeneration and `intent claude skills sync`.
- hv's as-written against as-built pass over ST0056, ST0057 and ST0069 and their WPs (2026-09-13): ST0056's remaining attachments by owner (ic tui-design.md and parity/, dc install.md and migration.md when released); every fix through the CLI doors, never the code.

## TODO

- After the tag: ic's reference regeneration, both halves `--rev v3.0.2 --baseline v3.0.1`; `intent claude skills sync` from the installed 3.0.2.
- ic's explorer TUI list from hv: the `/issues` seam landed (a8e3273d8); the rest of the list is hv's to hand over, after the tag unless hv says before.
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- hv rules the provisional items ST0056's deferred list still carries: D43, D46, the `new-surface` scope line in parity.md, the withheld-13 field, and `ac gate`'s ratification (issue 0032).
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic, after the cut).
- ST0056: a `brew install` on a clean Mac, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- hv rules 0356 to 0361 (search's answers, found in the ST0069 demonstration), 0363 to 0365 (Courses' migration findings), 0369 to 0373 (ST0069's as-built gaps) and 0374 to 0375 (Laksa's board-cutover findings: migrate carries entries as live past the message bound; the hv board's standing directives have no kind); a ruled one gets a lane.
- hv rules whether `intent claude upgrade --apply` regenerates the views it already knows are stale: the fleet sweep left four estates refused at their own gate on a pre-3.0.1 renderer change until dc ran `intent sync --to-disk` in each.
- Open defects: `intent issues list`; a ruled one gets a lane. Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
