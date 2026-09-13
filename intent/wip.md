---
verblock: "2026-09-13:v1.59: vc - 0354's cause named; cc fixes; the set moves once more"
intent_version: 3.0.1
---

# Work In Progress -- v3.0.2

## DOING

- 0354: S1 (669cf00ee) is in the delivered daemon (pid 18592) and the bursts continue; the cause is named in the issue from a hot sample (every index refresh loads and sorts every prose section body through `Facade::carried_paths`, and SQLite spills the sort to disk); cc lands the fix as one commit on green (a bodiless DISTINCT-file store door over an `(owner_type, file)` index, and one refresh per index batch); the rebuilt daemon under vc's CPU watch is the verdict, one run; then Laksa migrates on that pair's hash.
- The cut, hv at the terminal, nothing batched: the delivered set (9e60fcfb6) moves once more when cc's 0354 fix and ic's ST0074 WPs land, and dc rebuilds on their landing (suite once, `bin/devbin build all` announced to every lane, app-install, `intent daemon restart`, doctor 0); then `bin/devbin build release --patch`, `build all`, `int macos prepare`, `build formula`, `build publish`, `build smoke --reinstall`.

- hv's as-written against as-built pass over ST0056, ST0057 and ST0069 and their WPs (2026-09-13): ST0056's remaining attachments by owner (ic tui-design.md and parity/, dc install.md and migration.md when released); every fix through the CLI doors, never the code.
- ST0074 (ic's bundle) under vc's validator's eye: WP-01 and WP-02 done on main; hv ruled WP-05 (XDG Base Directory adopted for intent and intentd, `~/.config/intent` is v3's, v2 ignored, XDG_* granted into ALLOWED); hv: it is ALL 3.0.2, no 3.1.0; WP-05 (the XDG layout and its migration), WP-03 (the registry and discover) and WP-04 (the picker) are coded now in ic's worktree and land on main before the tag; 0368's test fix landed (64d272688).

## TODO

- After the tag: ic's reference regeneration, both halves `--rev v3.0.2 --baseline v3.0.1`; `intent claude skills sync` from the installed 3.0.2.
- Laksa's boards migrate on the fresh build carrying the 0354 fix, named to laksa-vc by hash with hv's go (restore the `.prettierignore` fence lines, retire the `:95` guard arm).
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- hv rules the provisional items ST0056's deferred list still carries: D43, D46, the `new-surface` scope line in parity.md, the withheld-13 field, and `ac gate`'s ratification (issue 0032).
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic, after the cut).
- ST0056: a `brew install` on a clean Mac, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- hv rules whether 0355 (the index watch's tree walk), 0356 to 0361 (search's answers, found in the ST0069 demonstration), 0363 to 0365 (Courses' migration findings: upgrade still calls the whiteboard not carried) and 0369 to 0373 (ST0069's as-built gaps: index_state, skipped, name_parts, reconcile in MCP and the explorer, status sizes) go before the tag; a ruled one gets a lane.
- Open defects: `intent issues list`; a ruled one gets a lane. Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
