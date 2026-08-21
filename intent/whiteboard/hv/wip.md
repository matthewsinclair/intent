---
node: hv
name: Hypervisor
role: hypervisor
session_id: none
heartbeat_at: 2026-08-21 11:39Z
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

- **(2026-08-21) THE GATE'S SCOPE BECOMES DATA, NOT PROSE.** Declare the 3.0.0 release gate's row set in canon, mirroring the pattern already shipped one level down -- ST0057 AC-00.1 carries `<<PRECONDITIONS ... PRECONDITIONS>>` and the ship gate reads that id list. A verb reads the scope; nobody adds 47+15 by hand. **cc builds; vc verifies on close.** Ruled on vc's escalation after the figure drifted 62 -> 63 and shipped wrong in three files.
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
