---
verblock: "2026-09-14:v1.64: vc - v3.0.3 shipped; the post-ship list"
intent_version: 3.0.3
---

# Work In Progress -- after v3.0.3

## DOING

- ic's Intent.app Console thread (hv approved it in ic's session: a Console copied from the Gtools console, `intent daemon logs` behind it, Run Doctor and a Rebuild Search Index item streaming into it): `intent st new`, the objective in hv's words, a one-page design attached, then the WPs in a private worktree; its release is hv's line (vc's lean: 3.1.0).
- hv's as-written against as-built pass over ST0056, ST0057 and ST0069 and their WPs (2026-09-13): ST0056's remaining attachments by owner (ic tui-design.md and parity/, dc install.md and migration.md now that 3.0.3 is released); every fix through the CLI doors, never the code.

## TODO

- hv: `brew unlink intent` and `brew pin intent` (`smoke --reinstall` left the keg linked, so `intent` on PATH is the keg and hook bodies serve from its libexec, not this tree); annotate the v3.0.2 GitHub release as superseded by v3.0.3, or delete it.
- ST0056: a `brew install matthewsinclair/intent/intent` on a stranger machine, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- ic's explorer TUI list from hv: the `/issues` seam shipped in 3.0.3; the rest of the list is hv's to hand over.
- hv rules the open batch: 0356 to 0361 (search's answers), 0363 to 0365 (Courses' migration findings), 0369 to 0373 (ST0069's as-built gaps), 0374 to 0377 and 0379 to 0389 (the board cutover's register and migrate defects, the locked-store false failure, the canon watcher's cache walk, the footer-only overwrite warning, the release script's abort and its missing view re-render, the release gate under host load); 0382 is a ruling on the carried stamp; a ruled one gets a lane.
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- hv rules the provisional items ST0056's deferred list still carries: D43, D46, the `new-surface` scope line in parity.md, the withheld-13 field, and `ac gate`'s ratification (issue 0032).
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic).
- hv rules whether `intent claude upgrade --apply` regenerates the views it already knows are stale: the fleet sweep left four estates refused at their own gate on a pre-3.0.1 renderer change until dc ran `intent sync --to-disk` in each.
- Open defects: `intent issues list`; a ruled one gets a lane. Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
