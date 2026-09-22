---
verblock: "2026-09-22:v3.7: vc - globalfold at EOD. DOING is empty: no thread, work package or issue is open. The first TODO is hv's push of the CI doc-link fix, whose pair is already built"
intent_version: 3.2.0
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

## TODO

- **FIRST, hv's hand: push `cd79407eb` and read CI.** It is dc's one-line fix for a public doc on `renumber::issue` (0511) that linked the private `moved_claims`, which reddened the rust doc step on both legs of CI run 35783294587. The fix is committed and not pushed. The installed pair already names `cd79407eb`, so the pre-push hook will pass unless something lands under `native/rust` first, and anything that does owes `bin/devbin build all` before the push (decision 30). Then read the rust run on the pushed HEAD, both legs, the test step included: CI's test step has not run on any of this line's changes yet (dc todo 37).
- **Restart any `intentd` started before the current pair was installed**, or it runs without 0516's backstop: `intent daemon restart`.
- **AWAITING hv: Gtools' request for an `intent wb` verb that corrects or redacts an item's text** (vc todo 61). The load-bearing requirement is that an append-only correction leaves an UNCOMMITTED originating event carrying the text, so the verb must supersede a pending event before it lands, and add a correction event only once the original is committed. Gtools' `cc` board stays uncommitted until it exists. Reply to `gtools-vc` with the issue number, or with hv's ruling.
- **dc**: Half A of the guards adoption pass, the fourteen estates canon does not reach, waits on hv's ruling on whether wiring an estate supersedes vc's decision-27 in-place repairs (dc hold 27). Prolix's whiteboard carry: hv's board first, then dc's hand; only the placeholder lines of hv's stub may be dropped.
- **WITH hv**: the pushes of Prodinfra, Riffle, Baize, Conflab, Laksa, Prolix, Utilz and the two Arca repos from their own checkouts, measured at the moment of pushing and never from this line; the 3.2.0 app into /Applications (the asset is Developer ID, notarized and stapled; the write needs hv's hand or a permission); and hv decision 28's ten Conflab and Baize rulings, every one owed and none executed, vc's hand and no push.
- **vc**: the Finding C census, read-only across the fleet, with Baize's own gate as the positive control.
- **MACHINE, hv's call**: `fseventsd` (pid 353) was at 14.8 GB RSS and above 120% CPU during the 0516 runs and has grown all day; about 150 `SetStoreUpdateService` and `LegacyImporterHost` processes are resident. Only a reboot answers whether the host's load floor of about 10 is a floor plus a leak.
- **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-18).
