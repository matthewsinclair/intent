---
verblock: "2026-09-24:v3.8: vc - globalfold at EOD. DOING is empty. 3.2.1 is ready to push and cut: hv's push, CI, the hold, then the cut"
intent_version: 3.2.0
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

## TODO

- **FIRST, hv's hand: push `main`, then read CI.** Every 3.2.1 fix is landed, closed and deployed, and the pair names the last commit under `native/rust`. So the pre-push hook passes unless something lands there first, and anything that does owes `bin/devbin build all` before the push (decision 30). vc then reads both workflows (`rust`, `Intent Tests`) on both legs, from the job logs rather than the run conclusion.
- **THE CUT'S HOLD, before hv fires anything.** Every node commits its own board render. Then no `intent wb` write of any kind, no commit and no `/in-session` until the `v3.2.1` tag exists (see `intent/restart.md`). hv's other sessions are held too.
- **THE CUT, in hv's terminal, never `--no-confirm`:** `intent daemon stop`; `bin/devbin build release v3.2.1 --dry-run`, then the real run. The release step builds the pair at the tag and checks it with the pre-push verdict before it asks to push (0546). Read the gate lines off the terminal before clearing it. Then `bin/int macos prepare`, `formula`, `publish` and `smoke --reinstall`, and finish with `brew unlink intent && brew pin intent && intent daemon restart`.
- **AFTER THE CUT:** lift the hold. Run vc todo 62, the fleet sweep, in every estate that already has Intent (hv decision 31): `intent claude upgrade --apply`, `intent todo update`, doctor, one commit per estate, no push. `sync --to-disk` only where doctor reads no counted ViewSkew, and back up each store first where no session did. dc's guards adoption pass (dc hold 29) rides the same sweep. Then the 3.2.1 app into `/Applications`, by hv's hand: vc's session is refused that write, and `/Applications` still holds a dev build from 2026-09-17.
- **AWAITING hv: Gtools' request for an `intent wb` verb that corrects or redacts an item's text** (vc todo 61). Its requirement is unruled: an append-only correction leaves an UNCOMMITTED originating event carrying the text, so the verb must supersede a pending event before it lands, and add a correction event only once the original is committed. Reply to `gtools-vc` with the issue number, or with hv's ruling.
- **dc**: Prolix's whiteboard carry. hv's board goes first, then dc's hand; only the placeholder lines of hv's stub may be dropped.
- **WITH hv, not re-measured since 2026-09-22**: the pushes of Prodinfra, Riffle, Baize, Conflab, Laksa, Prolix, Utilz and the two Arca repos from their own checkouts, measured at the moment of pushing; and hv decision 28's ten Conflab and Baize rulings, vc's hand and no push.
- **MACHINE, hv's call**: only a reboot answers whether the host's load floor of about 10 is a floor plus a leak (`fseventsd` and the resident `SetStoreUpdateService` processes grew all day on 2026-09-22).
- **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-23).
