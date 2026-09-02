---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-02 08:10Z
status: active
focus: "Cold start (aggressive checkin+localfold+compact on hv's word, 2026-09-02 08:10Z; reconvene on the bounce). ON THE BOUNCE: land AT-00.11 -- fully prepped + green-ACCEPTED by vc, recipe + artefacts in .history/20260902/ (NOT scratchpad, gone post-compact); one tight coherent commit via the vc canon window. DONE this session: ST0064 01.3+01.5 (5/9), ST0056/WP-09 verified+closed, issue 0212 filed, explorer confirmed REAL for hv. WP-17 stood down (17.6 cc's WP-08; 17.1 unsatisfiable-as-written -> hv). Currency: binary 361eff99 behind HEAD (a36f3722); rebuild owed, NOT mine. hv-items routed to vc. RE-MEASURE every figure at pickup."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Cold start 2026-09-02 08:10Z (aggressive checkin+localfold+compact on hv's word; reconvene on the bounce for next steps). Pre-fold verbatim + sha-verified at .history/20260902/wip-fold-0810Z.md (sha 433e0bbf). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree and the delivered binary lags HEAD.**

## DOING

**ON THE BOUNCE: land AT-00.11, then next steps from hv.** AT-00.11 (`of_n_closes_over_examined.sh`, mode 1 of AC-00.11) is FULLY PREPPED and green-ACCEPTED by vc; NOT landed (held for hv's fold/compact). Everything to land it -- the reframed instrument, the `at green` note, the fresh prediction, the sweep output, vc's rulings, and a step-by-step LANDING RECIPE -- is in `.history/20260902/` (README.md + four artefacts), rescued out of scratchpad which does not survive the compact. Land as ONE tight coherent commit through the vc canon window (the file on disk arms declared_kind/runner_roster/stale_at REPO-WIDE until committed -- keep it tight; verify the DISK before commit, 0212 revert risk). Evidence, both matching lodged predictions: positive control 6/6 OVERSTATED (the instrument CATCHES the founding defect) + clean sweep 28 CLOSES @ b4ab069e (2 measured / 10 absent-at-rev / 16 no-literal-ratio). The note carries the frozen-reach limit + the gate-cannot-fail truth (green means the INSTRUMENT works, not that every parity tool is checked). Nothing else of mine in flight.

## TODO

1. **LAND AT-00.11** -- recipe in `.history/20260902/README.md`. vc canon window first.
2. **ST0064 remaining (5/9), not mine to start:** 01.2/01.6 cc-gated (Swift present+correct; needs cc's live daemon + both false-positive states CONSTRUCTED; the shared `Poller` seam lands WITH 01.2 per vc -- parameterise the interval, don't copy-paste); 01.7 signing (dc pipeline + hv's ADC notarize); 01.4 console (cc's `daemon logs`, tail-orphan trap vs SIGTERM/INT/KILL separately).
3. **Explorer (hv):** confirmed REAL + wired + CURRENT in binary 361eff99 (`intent explore`, a ratatui TUI; installed == latest, no tui change since the build). Open Q for hv: does the Lotus menu SELECTION function? Mark Option A RULED+LANDED in the artifact.
4. **`intent app start|stop|restart`** (hv, new user verb): controls the INSTALLED app; new_surface `app` family; coord cc.

## Watch-outs -- mechanisms only

1. **Non-test AC closes via `intent ac satisfy`, NEVER an AT; use the ROW'S STATED INSTRUMENT.** Re-drive + positive-control before satisfying. (AT-backed rows close via `at green --note` carrying the driven command/output/sha.)
2. **Canon on SHARED threads has TWO+ hazards.** (a) 0206 (HIGH): two facade verbs each write the whole record back, second's stale snapshot SILENTLY overwrites first -- announce + commit in one breath. (b) 0210 (HIGH, mine): the `.canon/st/<ST>.json` extract names the STORE's union of every dirty-and-ingested attachment; with intentd running, one node's dirty parity/tools file blocks EVERY node's canon commit on that thread -- coherent-with-git-across-all-nodes or no commit; the guard names the FILE not the culprit. Its SECOND MECHANISM (mine, this session, judge whether it belongs ON 0210): a guard TRIAD -- declared_kind wants the roster to know an artefact, runner_roster refuses a row naming an uncommitted file, stale_at refuses a to-write row whose artefact exists -- so a new parity file has NO passing partial state, and the gate is repo-wide, so one author's intermediate state is every node's outage. **DISCIPLINE (vc): ST0056 attachment writes SERIALISED through vc; gather all evidence OUTSIDE the window, make the window one coherent commit.**
3. **0212 (HIGH, mine, filed 835ec6dd+a345eb0c): a watcher-driven ingest silently REVERTS a completed store write when the on-disk extract lags** (fires only when disk!=store at ingest -- a window, not a constant). CURE, transferable past `ac satisfy` to `at green`/`wp done`: verify the DISK extract (`jq .state.is`) BEFORE committing, never the store. Home intentd/src/watch.rs + intentd/src/store.rs:56-70 (ingest has no client, unattributable BY DESIGN). Third member of silent-write-loss-on-canon (0210 loud, 0206 two sessions, 0212 no client).
4. **The macOS app is `native/macos/Intent/` (Swift, xcodegen, folder-GLOBBED sources).** `bin/devbin macos app-build` = unsigned Debug to ~/.local/state (off-tree; release symlink untouched, no W28). `app-test` = the suite. IntentCLI = the one shell-out home; a NEW swift file regenerates the tracked xcodeproj (clean +N/-0 additions -- commit it). dc owns `int macos`; ic builds. NOT `dvb test rust` (unproven post-consolidation).
5. **A dated RULING RECORD is superseded, never edited.** RUSTFMT before `git add` on rust. cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` == your set. NEVER remove a peer's index.lock (WAIT + retry). `git commit --only <paths>` is the only safe write (`git add` new files first). RE-RUN tools, never quote a peer's count. Every stamp from a `date -u` read THIS turn.
6. **A value/claim carried ONE STEP past what it supports is the day's recurring class, and it is MINE too.** The hold-leg on a peer's stale "binary gone" (817eb93c, retracted 4680810d); "17.6 cleanly buildable" on a shallow read (retracted in 20min); D56 "one contract both transports" -> heard as "web can mutate" (vc corrected -- it equalises DOWNWARD). Hand/take PROPERTIES not values; measure a peer's COUNTER-value too; and a true structural statement is not a licence one inference further. vc's own fold ruling ("A+B need no second tree" -> "so the two-tree method can go") was the same shape, sitting two weeks -- true of RUNNING, not of CATCHING.
7. **`intent --version` (commit the BINARY was built from) vs `bin/devbin --version` (what the CHECKOUT is at NOW, +dirty) print an IDENTICAL `3.0.0 (<sha>)` for DIFFERENT quantities.** Currency = `intent --version` vs `git rev-list -1 HEAD -- native/rust surface docs/design`, NEVER vs devbin's. A display filter over a tool's own diagnostic is a population filter (I truncated of_n_closes's EXCEPTIONS out of my own read -- caught by vc; a negative from a truncated read is not a result).

## Decisions

- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv first-hand 2026-09-01 08:32Z, 3183990a via vc). Everything outstanding goes in; cost not a constraint; no external consumer. SCARCITY-REGISTER shape ("not in this cut" / "after the tag" / "ship it red") retired as a class.
- **TOOLCHAIN CURRENCY: binary 361eff99 BEHIND HEAD** (a36f3722, peer AC-06.11); gate currency arm REFUSING; `bin/devbin build all` rebuild owed, NOT mine (racing sessions is the 0196/ST0058 hazard); routed to vc for hv. The value rotted exactly as the property predicted. The "broken toolchain" leg (817eb93c) stays retracted.
- **ST0064 WP-01 = 5/9 (ic).** 01.1+01.8 (9cad4780), 01.9 (d0cec969), 01.3+01.5 (code 50c56d8a / canon 5deb477c + e8ff6486 correction; bundle-proven @361eff99; app-test 14/14; critic-swift 0/0). 01.5 first reverted by the 0212 ingest race, re-landed disk-verified. project-CWD wiring 7e84538a; vc RULED (a) per-app-instance root, D07 registry UNBUILT. Remaining 01.2/01.4/01.6/01.7 cc/dc/hv-gated.
- **ST0056/WP-09 CLOSED (471cbca3), verified 12/12 mcp tests green** (not a typed string -- re-driven). AC-09.6 closed (337d6451). ST0056/09 = 6/6.
- **WP-17 STOOD DOWN (vc, option 2).** Both realisers EXIST. 17.6 blocked on cc's WP-08 (browser-open stub). 17.1 UNSATISFIABLE AS WRITTEN in 3.0.x (no model-mutation op on any transport; D56 equalises downward) -- hv's to reword/build/descope; vc has it. ST0056/17 = 10/12.
- **AT-00.11 green ACCEPTED by vc (2026-09-02), landing pending** -- see DOING + .history/20260902/. vc rulings: AC-00.11 text UNCHANGED (a requirement isn't narrowed by one test's reach); of_n_population NOT narrowed (would derive population from filter, the row's own defect); the note carries the frozen-reach limit + the 2-measured truth. OWED, separate row: 16 path-shape instruments make ratio claims NO current mode adjudicates (measured, not tonight).
- **ST0065 has ZERO ACs** (empty contract, hv-owned). WP-14 (whiteboard+inboxes in the store, L) MAY return from ST0069 -- ownership hv's, not mine.
- **cc: `intent edit <address> --path` is the AC-01.5 door.** Health is a PROJECTION above route(); STALE never unlinks (AC-08.12).
