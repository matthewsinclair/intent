---
verblock: "2026-09-13:v1.58: vc - hv: all of ST0074 is 3.0.2"
intent_version: 3.0.1
---

# Work In Progress -- v3.0.2

## DOING

- 0354: the fix is on main (669cf00ee, the store thread on std's thread parker); dc, released by hv, runs the workspace suite once on main, then `bin/devbin build all`, app-install, `intent daemon restart`, doctor 0; the rebuilt daemon under vc's CPU watch is the verdict, hv's `sudo dtruss -t psynch_cvwait -p <pid>` the fallback if it still spins; then Laksa migrates on the delivered pair's hash.
- The rebuild carries 0366 (524f5f868, AT-22.4), ST0074/01 the explorer's /threads and /issues (793984a50) and /02 the menubar status line (133061d7f, app-test 32/0), and the 0354 fix once hv allows cc's commit; dc runs the workspace suite once on main before `bin/devbin build all`.
- The cut, hv at the terminal, nothing batched: `bin/devbin build release --patch`, `build all`, `int macos prepare`, `build formula`, `build publish`, `build smoke --reinstall`.

- hv's as-written against as-built pass over ST0056, ST0057 and ST0069 and their WPs (2026-09-13): cc takes ST0069 and ST0057; ST0056's attachments by owner (ic tui-design.md and parity/, dc install.md and migration.md when released, cc data-model.md, output-contracts.md and realisation.md; vc the cover, design.md, impl.md, tasks.md, deferred.md); every fix through the CLI doors, never the code.
- ST0074 (ic's bundle) under vc's validator's eye: WP-01 and WP-02 done on main; hv ruled WP-05 (XDG Base Directory adopted for intent and intentd, `~/.config/intent` is v3's, v2 ignored, XDG_* granted into ALLOWED); hv: it is ALL 3.0.2, no 3.1.0; WP-05 (the XDG layout and its migration), WP-03 (the registry and discover) and WP-04 (the picker) are coded now in ic's worktree and land on main before the tag; 0368's test fix landed (64d272688).

## TODO

- After the tag: ic's reference regeneration, both halves `--rev v3.0.2 --baseline v3.0.1`; `intent claude skills sync` from the installed 3.0.2.
- Laksa's boards migrate on the fresh build carrying the 0354 fix, named to laksa-vc by hash with hv's go (restore the `.prettierignore` fence lines, retire the `:95` guard arm).
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- hv rules the provisional items ST0056's deferred list still carries: D43, D46, the `new-surface` scope line in parity.md, the withheld-13 field, and `ac gate`'s ratification (issue 0032).
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic, after the cut).
- ST0056: a `brew install` on a clean Mac, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- hv rules whether 0355 (the index watch's tree walk), 0356 to 0361 (search's answers, found in the ST0069 demonstration) and 0363 to 0365 (Courses' migration findings: upgrade still calls the whiteboard not carried) go before the tag; a ruled one gets a lane.
- Open defects: `intent issues list`; a ruled one gets a lane. Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
