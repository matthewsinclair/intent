---
verblock: "2026-09-18:v2.1: vc - rewritten from the unified plan hv approved on the bounce (vc decision 38) after the morning's train landed. DOING and TODO only, each item naming its owner and the condition that discharges it. No state that a command regenerates, and no content that already has a home: the close-out rulings are in intent/history/20260915-hv-rulings.md, the bank and landing rules in intent/restart.md, the cut sequence in intent/docs/releasing.md, every issue in `intent issues show <id>`, and ST0078's proposal in its design.md."
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads, then the fleet

Measure before believing anything below: `.claude/restart.md` carries the commands. **No count appears in this file** -- a count in a document a reader reads is a state with no command beside it, and every count this file used to carry had rotted.

## DOING

- **THE CLOSE-OUT** (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. Rulings: `intent/history/20260915-hv-rulings.md` and hv's board. What is open: `intent issues list`, `intent st list`. One lane lands at a time on vc's word; the bank, train and landing rules live in `intent/restart.md`. The SQLite move (rusqlite 0.40.2 / SQLite 3.53.2, with `fallible_uint` on) and the 0442 detector are on main and the pair carries them.
- **0446**, dc's, blocks the live fleet trawl: `organize`'s dehydration guard and `st sync`'s empty-estate guard read a view an older renderer wrote as a possible hand edit, because they compare whole files. It closes when the guard passes an older renderer's view and still refuses a real hand edit, driven against the refusals dc measured in the trawl rehearsal.
- **ST0078**, hv's: Intent on a multi-person project with a git workflow and PRs. The proposal is `intent/st/ST0078/design.md`, with every worked example driven in two clones and the log banked at `refs/bank/vc/st0078/`. Nothing is built until hv rules on P1 to P4 and on whether it goes into 3.1.0; if its answer is project-wide, the fleet trawl waits for it so no estate is touched twice.
- **The all-estate board trawl** (hv, 2026-09-16), cc's, Part A GO and Part B under vc's disposition (hv decision 24). Plan at `refs/bank/cc/trawl/carry-plan.md`. It is the whiteboard subset of the fleet trawl and runs with it.
- **vc**: judging every bank, train and landing; the 3.1.0 notes read for voice and for anything internal; the menubar app's LaunchServices `-600` at install; ic's verification pass over this file.

## TODO

- **THE 3.1.0 CUT SEQUENCE**, in this order, each on vc's word, the sequence itself in `intent/docs/releasing.md`: cc commits the CHANGELOG section and `docs/releases/3.1.0/RELEASE_NOTES.md` from `refs/bank/cc/release-notes/` (the notes already carry the SQLite move and the detector); ic regenerates both halves of `docs/reference` and commits them BEFORE the driver runs, and the staleness check ic landed now REFUSES `build release` until that commit exists; cc re-drives `docs/known-defects.md` whole against the fresh build (hv decision 25), 0442 gets an entry and 0343 deliberately none; hv cuts in hv's terminal; then gyges.
- **The running intentd is the pre-bump image until hv restarts it**: `intent daemon restart` in hv's terminal, or hv's word to cc. Until then one WAL file has a 3.46.0 writer beside 3.53.2 writers, which is the WAL-reset defect's precondition on the daemon's side.
- **Small issues that close before the cut, on merits** (batch before the tag): 0447 (`Facade::open`'s ingest reports a damaged fts5 read as a canon refusal with no artefact to fix), 0448 (`init` writes neither aggregate view, so the first commit is refused by the gate `claude upgrade --apply` installs), and the broken intra-doc links `cargo doc -p intentsvcs` warns on, to be filed. Each XS to S, routed by vc as a lane frees.
- **The LIVE guards adoption pass**, dc's, HELD. Released when `bin/int hooks` reports a canon-shim-wired hook AS wired; the defect is devbin's (hv Q7). **Never run `int hooks --install`.** Gates nothing in this release.
- **THE FLEET TRAWL** (hv, 2026-09-18, the day's second priority): every project in `~/.config/intent/projects.json` upgraded and pristine under `intent doctor` and `intent organize`, one mechanical commit per estate in its own repo, NO PUSH. Rehearsed in scratch clones and banked at `refs/bank/dc/trawl/` with the estate split between dc and ic in `split.md`; the stops are recorded there, most of them 0446. Runs after 0446 lands, on the rebuilt pair, after hv's ruling on ST0078 (hv Q3), one re-rehearsal first; devbin#0083's sweep after it (hv Q8). The dead registry entry is already pruned.
- **WITH hv**: the ST0078 proposal (P1 the event log travels, P2 a renumber verb, P3 the store after a pull, P4 the team page) and its five open questions, each with context, options and a recommendation in the design; whether ST0078 goes into 3.1.0 with the trawl and the cut waiting for it; the daemon restart.
- **ST0056** closes when hv runs `intent/st/ST0056/gyges-brief.md` on gyges against the PUBLISHED 3.1.0 and hands back `~/intent-clean-install.log`: vc judges it, dc satisfies AC-00.5 and AC-11.1 on it, WP-11 and the thread close on hv's word.
- **ST0060** and **ST0077** stay in Triage as the next line's queue (hv Q4, 2026-09-18); neither is unfinished work.
