---
node: hv
name: Hypervisor
role: hypervisor
session_id: none
heartbeat_at: 2026-08-21 12:44Z
status: active
focus: "Workstream Zero. Adjudicates scope, sequences work, owns releases and commits-to-main. **This board is maintained BY vc ON hv's BEHALF from 2026-08-21 -- see the provenance rule below. Every directive here is hv's word, dated, and traceable to where hv said it.**"
claims: []
---

# Hypervisor (hv)

## PROVENANCE -- READ THIS BEFORE READING ANYTHING BELOW

**This board is written by `vc` under hv's ruling of 2026-08-21, not by hv directly.** The protocol allows it: the hypervisor node is human-driven and "the human maintains it (or has it maintained on their behalf)". It was a stub from 2026-06-25 to 2026-08-21 while peers were told to read it for directives at pickup, and hv's real directives lived scattered across peers' boards and quoted speech in `README.md`.

**The constraint on the pen, and it is the whole point:** every entry below is hv's word, carrying the date hv said it and a pointer to where. **vc holds the pen hv handed over; vc does not hold hv's authority.** Nothing enters this board because vc thinks it is right. A ruling vc cannot attribute does not go in -- it goes to hv as a question. If you find an unattributed directive here, that is a defect in this board, not a directive.

## DOING

- Sequencing the 3.0.0 gate. Five rows outstanding: ST0057 AC-01.5, AC-03.6, AC-07.7, AC-08.5; ST0056 AC-03.14.

## TODO

- (hv sets)

## Standing directives

- **(2026-08-21) ALL FOUR WORKSTREAMS: AGGRESSIVE LOCALFOLD, THEN HOLD.** hv stops every Claude Code session, opens fresh terminals, relaunches each node, and the estate reconvenes after. **The fold has to be real because `restart.md` CHANGED today and it is what every node's `--append-system-prompt` carries** -- anything held in conversation rather than in a file does not survive the restart, which is precisely how `intentdb` reached all five of us and stayed six days. **Fold the thing you could not reconstruct, not the summary.** Announced to cc, ic and dc with the new state attached; vc folds and holds too.

- **(2026-08-21) THE v2 CLI SPLITS OUT OF THIS CHECKOUT.** `v2-maintenance` branched at `fb45e9ea` (**main HEAD, NOT the `v2.19.0` tag** -- the fleet has never run the tag, so branching there would have silently rolled 2027 commits back across every project on this machine and presented as a symlink move); checked out at `~/Devel/prj/Intentv2` at `d74fb388`, config declared `2.19.0` so it does not refuse itself. **hv makes the two machine-wide moves** (`~/.zshrc:37-38` and the `~/.local/bin/intent` symlink). **The binding was never just the symlink: `bin/intent:26` reads `if [ -z "$INTENT_HOME" ]`, so the exported var beats symlink resolution outright, and `$INTENT_HOME/bin` is a second PATH route.** Once done, **HEAD here is free to be ruthless about v3.**
- **(2026-08-21) THIS REPO'S GUARD RESOLUTION AFTER THE SPLIT IS dc's, AS A MECHANISM.** With `INTENT_HOME` pointing at `Intentv2`, this repo's commit guards resolve out of the frozen v2 checkout -- byte-identical today, slow drift after, and the exact frozen-roster failure already on record. **hv declined both cheap answers**: a `.envrc` covers an interactive prompt and not automation, because git hooks do not reliably inherit direnv; and refreshing the v2 copy by hand is an advisory that requires remembering, which is not a control. **A mechanism, not a variable**, and a wrong answer must be loud.
- **(2026-08-21) THE WORD `intentdb` IS RETIRED CORPUS-WIDE. IT NAMES NO COMPONENT.** hv, verbatim: _"This is absolutely not true. The SQLite db is the durable SSOT. Always has been. The intentd, just like the cli, which itself uses intentsvcs, all talk to the db. The daemon is only there for some other wider features that go beyond the original functionality of the single, per-project intent operations."_ The crates are `intent-cli`, `intentd`, `intentsvcs`; the db is a SQLite file all three talk to. **The SUBSTANCE of D01 is unchanged -- the db is the durable SSOT and the files are re-creatable. Only the term is wrong**, and it mattered because it implied a daemon-owned store. **The word was adopted from hv's own phrasing in two quoted rulings of 2026-08-15, which is why no node ever challenged it.** Corrected at `513642e7` in both restart files, `wip.md`, `.gitignore`, `design.md` and ST0056 canon; quoted rulings corrected in square brackets with an editorial note, never silently. Routed: `intentsvcs` doc comments (cc), `surface/dispatch-table` (ic), `bin/.devbin/cmd/precommit` (dc).
- **(2026-08-21) `runner_roster_check.sh`'s POPULATION WIDENS TO EVERY PARITY INSTRUMENT.** Every instrument under `intent/st/*/parity/tools/` regardless of filename; each declares `gated` or `manual` with a required reason. **cc builds; vc verifies on close.** Ruled after vc measured that the guard -- gated, clean on every commit, whose job is "every parity instrument declares whether anything runs it" -- has a population bounded to ST0056's tools dir AND to `*_check.sh` names, leaving ST0057's whole toolset undeclared and invisible to it.
- **(2026-08-21) A CRITERION RESTING ON AN UNDISPATCHED INSTRUMENT GETS DRIVEN, NOT ARGUED ABOUT.** Applied same day to the five ST0057 greens: **AC-01.2, AC-01.4, AC-02.4, AC-04.6 CONFIRMED by driven measurement at `49be1059`; AC-07.5's arm A refused at exit 2 on a needle defect (`pgrep -f 'intentd'` matching `intentdb` in every MAAC node's system prompt) and its arm B passed, so the green stands and the instrument is what needs fixing.**
- **(2026-08-21) THE GATE'S SCOPE BECOMES DATA, NOT PROSE.** Declare the gate's row set in canon, mirroring the pattern already shipped one level down. **SCOPE CORRECTED BY vc 2026-08-21: this gate is _all ST0057 live rows plus all ST0056 WP-03 rows_ (67), which is ST0057's CLOSURE gate and NOT the 3.0.0 release gate.** vc's first transcription of this ruling called it the release gate, which is a title broader than its body -- the v3.0.0 release is ST0056 WP-12, whose dependency line reads _all prior WPs_, and ST0056 stands at 59/132 with seven WPs Not Started. **The declaration cc builds must name WHICH gate it declares** -- ST0057 AC-00.1 carries `<<PRECONDITIONS ... PRECONDITIONS>>` and the ship gate reads that id list. A verb reads the scope; nobody adds 47+15 by hand. **cc builds; vc verifies on close.** Ruled on vc's escalation after the figure drifted 62 -> 63 and shipped wrong in three files.
- **(2026-08-21) EVERY NODE PRUNES ITS OWN `target/<node>` AT FOLD.** Disposal joins creation in the same ritual: `/in-finish release` removes the per-node target dir the node made. Shared `target/debug` survives. Ruled after ~71G of build artefacts was found, 33G of it duplication produced by correct compliance with the isolate-the-target-dir rule, which said how to create these and nothing about removing them.
- **(2026-08-21) `upstream` IS PUSHABLE.** The freeze lift of 2026-08-20 is standing, not one-shot. Push both remotes.
- **(2026-08-18) ST0057 IS A PRE-RELEASE THREAD, NOT A FOLLOW-ON.** hv's words, quoted at `intent/st/ST0057/info.md:19`: _"Definitely BEFORE the release. We're getting this whole thing feature complete before we release 3.0.0."_
- **(2026-08-15) dc's SCOPE IS hv's FRAMING AND IS QUOTED RATHER THAN ELABORATED**, because the dc/cc boundary is not yet ruled -- `README.md` roster. **`bin/` is the one genuine collision** (v2 bash CLI plus the devbin) and is open for hv rather than assumed by either node.
- **(2026-08-19) THE hv INBOX'S READER IS vc.** hv's words, quoted in `README.md`: _the workstreams can write in the hv channel FOR me, but I need that stuff surfaced TO me by vc._ **A write is the durable half and never the delivery.** Restated by hv 2026-08-21: filing into the inbox does not get a decision -- **hv needs context, a question, and options, in the live channel.**
- **THREE ITEMS ARE HELD ON hv's WORD AND MUST NOT BE STARTED:** dc's two roster admissions (`canon_commit_check.sh`, `thread_view_skew_check.sh`) and `tests/lib/test_helper.bash:93`'s `3.0.0` default. hv instructed dc directly to hold. **vc issued a "land it" on the third and was wrong to; hv's word outranks it.**

## Watch-outs

- **A DIRECTIVE THAT IS NOT REACHABLE FROM THE SITE THAT NEEDS IT IS AN OPEN QUESTION IN PRACTICE.** This board was a stub for eight weeks while peers were told to read it. That is the defect this board now exists to close.
- **ATTRIBUTE hv's LIVE STAMPS, NEVER ASSERT THEM.** The live channel is unguarded -- all three clock-guard checks run at commit. A peer quoting a live stamp into their board launders it into the committed record past a guard posted at the wrong door.

## Decisions

- (2026-08-21) This board's writer is `vc`, on hv's ruling, under the provenance rule above.
