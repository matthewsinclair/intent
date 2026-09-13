---
verblock: "2026-09-13:v1.50: vc - run 6 no panic, dtruss decides; 0366 ordered, ST0069 reopened"
intent_version: 3.0.1
---

# Work In Progress -- v3.0.2

## DOING

- 0354 FIRST, hv's order 2026-09-13: fix the spin, then rebuild everything, then Laksa migrates on the fresh build. The site is settled (the store thread's park at store.rs:289, no sender, named by stack on the live daemon); run 6 (a release pair with the parker's assert armed) reproduces WITHOUT a panic, so the wait returns 0 and hv's `sudo dtruss -t psynch_cvwait -p 58837` while hot decides between EINTR (an interrupter, the whole fix) and an immediate 0 (a psynch condvar fault: cc's S1, a private block_on on std's Darwin parker at the two sites); dc verifies the fix with run 1's recipe cold for ten minutes after the index completes, then `bin/devbin build all`, app-install, daemon restart, doctor 0.
- 0366, same window under hv's "sans defects": the daemon never builds its index at open (cc's reading, matched by runs 4 and 5); ST0069 reopened for it; cc builds the index-at-open reconcile on WP-22 with its AC/AT (a daemon-suite test, red on e70c3528a) as one commit.
- The cut, hv at the terminal, nothing batched: `bin/devbin build release --patch`, `build all`, `int macos prepare`, `build formula`, `build publish`, `build smoke --reinstall`.

## TODO

- After the tag: ic's reference regeneration, both halves `--rev v3.0.2 --baseline v3.0.1`; `intent claude skills sync` from the installed 3.0.2.
- Laksa's boards migrate on the fresh build carrying the 0354 fix, named to laksa-vc by hash with hv's go (restore the `.prettierignore` fence lines, retire the `:95` guard arm).
- hv rules or overrules the calls in `intent/history/20260913-calls-under-the-pen.md`; AC-24.6, AC-24.7 and `INTENT_NODE` are hv's alone.
- Hover the menubar identity row on a failed `intent version` read: ic could not confirm a disabled item shows its tooltip (hv or ic, after the cut).
- ST0056: a `brew install` on a clean Mac, then AC-00.5 and AC-11.1 by evidence; WP-11 closes with them.
- hv rules whether 0355 (the index watch's tree walk), 0356 to 0361 (search's answers, found in the ST0069 demonstration) and 0363 to 0365 (Courses' migration findings: upgrade still calls the whiteboard not carried) go before the tag; a ruled one gets a lane.
- Open defects: `intent issues list`; a ruled one gets a lane. Out of 3.0.x by ruling: ST0057, ST0060, ST0070.
