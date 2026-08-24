---
node: vc
name: Validation Claude
role: validation
session_id: 7ae34f78-5b66-4872-a0b0-152af8cd6132
heartbeat_at: 2026-08-24 19:46Z
status: active
focus: "**BOOTSTRAPPED AFTER THE COMPACT AT 2026-08-24 19:46Z; NOTHING IN FLIGHT; HOLDING FOR INSTRUCTIONS.** Tree clean, all four inboxes empty, all three peers folded. **`session_id` is UNCHANGED across the compact and that is the finding, not an oversight: a `/compact` does not rotate it**, so hv's bounce test (_same id means it resumed_) reads a compacted session as a resumed one -- half a test, and the failing half is the one you read. Project-wide narrative, traps, clock and conventions live in `intent/restart.md`, read at pickup and NOT repeated here. Gate 66 of 67; `AC-08.5` is cc's to build and ic's to cover, untouched by me."
claims: [ST0056, ST0057, ST0058]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md` UNDER "Traps that cost real time" AND "The clock".** They are not repeated here -- **that duplication is the exact defect today's lean removed**, and a rule in two homes drifts in both. This board carries only what is vc's: the fold record, what is routed, and the rules not yet promoted.

**Long-form traffic: `.history/20260824/` -- `wip-fold-pre-compact.md` (morning), `wip-fold-eod.md` (afternoon), `wip-fold-lean.md` (this fold).**

## DOING

**Nothing in flight.** Folded after the document lean; holding for the compact and then for instructions.

## TODO

1. **PUSH DISCIPLINE IS A STANDING PROPERTY, NOT A TASK -- LEAVE THE REFS LEVEL.** Authority granted and standing.
2. **FOUR THINGS SIT WITH hv AND NONE IS MINE TO CLEAR** -- listed in `intent/wip.md` under "Sitting with hv": dc's routing question 2, `intent#0073`'s six swift dispositions, `intent#0074`'s heex-coverage question, `intent#0071`'s v2 CHANGELOG heading. **The escaped-mutator decision was WITHDRAWN: dc committed the one real change and the tree is clean.**
3. **AC-08.5 is ST0057's last gate row.** cc builds, ic covers, untouched by me.
4. **`--force` for `claude skills`** (ruled, queued); **`rulings_check.sh` is named after a field that no longer exists**; **the marker's per-crate staleness is NOT closed** -- both binaries agree today only because one change touched both packages.
5. **The ETXTBSY flake is closed ON THE POPULATION DERIVATION, not on greens** -- `rust.yml` is path-filtered, so the run count is ONE and stays one until someone touches Rust. **Do not read an accumulating green board as accumulating evidence.**

## WATCH-OUTS -- vc's OWN, NOT YET PROMOTED

- **AN ESCALATION IS FINISHED WHEN A NAMED READER HAS IT, NEVER WHEN THE WRITE RETURNS.** The escaped-mutator revert sat on MY board as _"Decision still hv's"_ and in dc's commit message as _"the revert is hv's ruling"_ -- **correctly recorded by both of us, delivered by neither**, and hv's inbox held zero mentions. **Every write returned 0.** The roster names ME as the node obliged to surface hv-channel content TO hv. **The tell: a TODO item whose text describes WHOSE decision it is. That phrasing feels like routing and is a description of routing.** **And I then escalated that same decision THREE TIMES with a wrong population while the verdict never moved.**

- **A WARNING IS NOT DISCHARGED BY BEING TRUE, AND NOT BY BEING CURRENT WHEN OBSERVED EITHER** (ic). **In a five-node checkout the subject MOVES between noticing and sending**, so the measurement that counts is taken at the moment of SENDING. Three accusations against me died on re-measurement in one stretch and **every one was true when ic noticed it.** **Your evidence ages while your confidence does not.**

- **CORROBORATION IS ONLY REAL IF THE INSTRUMENTS COULD HAVE DISAGREED. ASK: COULD EACH HAVE PRODUCED THE OTHER'S FINDING?** `prolix-cc` and `prolix-vc` routed `intent#0073` an hour apart -- **one instrument reported twice**, same estate, same runner; banking them as two estates confirming carries one reading with twice the confidence. **AND THE DISAGREEMENT IS THE PAYOFF:** my elixir control returned 19/0 against their 9/10 and **the disagreement located the bug in MY checker.** **A relay is not a second witness** -- tell the RECIPIENT to expect one copy, because the recipient can dedupe and the relay can only be beaten. **A real one cannot be manufactured:** ic's `for-each-ref` taken at pickup for an unrelated purpose BEFORE the claim existed. **ATTRIBUTION FAILS BOTH WAYS THROUGH THE SAME CHANNEL: NAME WHICH HALF** -- an incident and its generalisation are separable and usually have different authors.

- **A HELPER CAN BE CORRECT IN EVERY RESPECT EXCEPT THE SHELL IT RUNS IN, AND THAT DEFECT REVIEWS CLEAN.** bats implements `skip` and `fail` by unwinding the shell they are called in, so `v2="$(_helper)"` aborts the **COMMAND SUBSTITUTION'S SUBSHELL** and the test continues with an empty path -- 247 files compared against `""`. **Keep the message in a variable and the `skip`/`fail` call in the test body.** **cc's general form, which paid out on my own work: A REFUSAL SURVIVES WHAT A READING DOES NOT.**

- **AN ATTESTATION CLOSES A QUESTION A GAP WOULD HAVE INVITED, AND RIGOUR IS THE DISGUISE.** **A record that CLOSES a question it did not ANSWER is worse than no record**, and it lives in CODE as readily as in prose -- the Thread graft's comment claims _"never a partially-defaulted document"_, **true of the four lines beneath it and false of the nine it does not reach** (ic: scope local, grammar global). **An unmeasured number in a RATIONALE is load-bearing in a way one in a REPORT is not -- a report gets checked, a rationale gets HONOURED.**

- **AN ANSWER SURVIVING A CHANGE OF PREMISE IS NOT THE SAME ANSWER, AND RE-PUTTING IT IS HOW YOU FIND OUT** (dc). Their verdict held while the REASON was replaced entirely, and **the old reason died with the restore while the conclusion did not move, so nothing would have flagged it.** Related: **ONE FIELD CARRYING TWO MEANINGS** is this estate's most common structural defect, whose OUTPUT form is **a message that misdescribes its own scope** -- `intent#0069`, framed as dc's stronger claim: **the confirmation line carries ONE HARDCODED NOUN regardless of what the operation touched.** `intent#0074` is the same family.

- **A GUARD'S FAILURE MODES ARE NOT SYMMETRIC, AND A LOUD FAILURE IS NOT AUTOMATICALLY THE SAFE ONE -- the question is whether the loudness is TRUE.** Noise is what gets dismissed: **the real drift then hides inside the false ones, in an instrument everybody can see working.** **A rule that fires on correct code costs you the rules that ARE sound** (prolix-vc). **DETECTABILITY DECAYS.** **A capability with no consumer is not a gate, and a checkpoint with no failure mode it can catch is a QUEUE** -- which is why mechanical canon edits do NOT route through me and authored ones do: **DERIVED vs AUTHORED**, not mechanical vs interesting.

- **A DEFECT A PEER ROUTINELY REPAIRS AS A SIDE EFFECT OF THEIR NORMAL WORK IS INVISIBLE TO THE NODE THAT PRODUCES IT** (dc). **OFFERING A MENU IS AN ACT OF AUTHORSHIP THAT LOOKS LIKE AN ACT OF SERVICE** -- preserve the branches not taken. **A PEER RELAYING THAT hv HAS NO OBJECTION IS NOT hv AUTHORISING.** **A FOLD INSTRUCTION IS NOT A TRUSTED SOURCE ABOUT YOUR OWN HISTORY** (ic). **THE CORRECT SCOPE OF A RULE IS NOT VISIBLE FROM THE INCIDENT THAT PRODUCED IT.**

- **I TOLD THREE PEERS TO "PULL FIRST" AND IT HAS NO REFERENT** (ic's catch). **Five sessions share ONE working tree**, so a peer's commit is in my HEAD the moment they make it -- verified: both of ic's were already ancestors, no pull. **I was reasoning about the topology as if it were separate clones**, which is the same shape as every population error today: a true instruction applied to the wrong subject. **And the symmetric half is mine and corrects ic: "pushing only my own commits" is equally meaningless** -- one branch, one tree, so a push carries every peer commit since the last one, and it always has.

## DECISIONS -- LIVE ONLY

- (2026-08-24) **hv: THE Intentv2 FREEZE IS SCOPED -- FROZEN FOR FEATURES, LIVE FOR SHIPPED-SURFACE DEFECTS.** Options were vc's, choice hv's; branches not taken are on hv's board.
- (2026-08-24) **hv: `intent3` MAY REFUSE AN INCOHERENT BINARY; dc DRAFTS IT.** **A dirty marker cannot ground a refusal and cannot be discharged -- it can only downgrade one to a permanent unclearable warning** (dc).
- (2026-08-24) **vc: NO CHANGELOG ENTRY FOR A DEFECT INTRODUCED AND FIXED INSIDE ONE UNRELEASED CYCLE.** There is no reader for it, **and the document agrees -- the `3.0.0` block has no `Fixed` section while 35 release headings below it do, so the line would require CREATING the section to hold it.** **`intent#0071` is the counter-example and is hv's: 2.19.0 SHIPPED it.**
- (2026-08-24) **vc: MECHANICAL CANON EDITS DO NOT ROUTE THROUGH ME; AUTHORED ONES DO.** The line is **DERIVED vs AUTHORED**. An attachment hash is derived from a file the builder committed and `canon_commit_check` already gates it; criterion text, `state`, `status`, notes and `covers` are the contract.
- (2026-08-24) **vc: EACH PROJECT-WIDE DOCUMENT GETS ONE JOB.** `.claude/restart.md` is the entry point, `intent/wip.md` is current work, `intent/restart.md` is narrative + traps + conventions. **If you find yourself writing a supersedes banner, delete what it supersedes instead.**
