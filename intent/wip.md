---
verblock: "26 Nov 2026:v4.37: vc - 3.5.6 READY; folded for hv's estate compact"
intent_version: 3.5.5
---

# Work In Progress

## DOING

- Intent 3.5.6 is READY for hv's cut (hv decisions 118 and 119). Each fix is landed and closed: Intent.app takes ArcaLaunch (I5169c5b, af157d266), app-run and app-install wait for the old copy to exit (I8b74f2a, fc33a251a), the cut refuses a known-defects page titled for another release (If7e7665, 9fc66c4ac), and `intent app stop` fails at once on a failed quit (I47c94cf, 1a9bc8702). Step 2 is at de4261bb1. gate all and test shell are green at de4261bb1, and the dry run is green at e85cfd709. Main past e85cfd709 carries board, event, issue-record and WIP files only. intentd is stopped.
- The cut: hv types `bin/devbin release v3.5.6` in hv's own terminal in `~/Devel/prj/Intent` and answers its confirm (hv decision 3). The cut's pre-flight refuses any dirt in the tree, so every seat commits its board files before vc gives hv the HEAD.

## TODO

- After the cut, from `intent/docs/releasing.md`: `intent daemon start`; `brew upgrade intent`, then `intent --version` and `intent doctor`; the app swap on this Mac; step 5, the site's `db/release.json`, on hv's word (a push to another repo).
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 with the three parity-tool fixes hv parked there (ST0084; hv decisions 104, 105, 106 and 110), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Each takes an Intent upgrade and nothing else (hv decision 114). Their text is `git show f16dce4a7:intent/wip.md`.
