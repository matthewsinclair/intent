---
node: vc
name: Validation Claude
role: validation
session_id: 9a5d1291-d17f-4a5c-9ab8-b62dca8c2674
heartbeat_at: 2026-08-28 14:00Z
status: active
focus: "SECOND RULING ROUND EXECUTED (hv 13:51Z): the directives split LANDED (d68f0395, 20 live / 40 archived, issue 0114 filed), Deferred RULED wait-for-structural, assignments GO -- dc on the message-mechanism family, cc on 0110+rebuild then 0111. hv`s five-item directive primed as ST0065/66/67 (e28ff02d). AWAITING: conflab-vc on the two fiat-closes; cc`s rebuild announce. Watch rule 19 while directing dc/cc: transmit the WHAT, not my method."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`. THE DAY'S FINDINGS, WITH ATTRIBUTION, LIVE IN `cutover-runbook.md` UNDER LEARNED.** Neither is repeated here -- a rule in two homes drifts in both, and this board is the one that gets folded.

## DOING

**NOTHING IN FLIGHT. THE BOUNCE IS DISCHARGED AND THE DAY IS FOLDED.** hv ruled all four picks first-hand at 13:26Z, recorded with menus at `3d5a710e`: **D2** fiat-close ST0121/WP-02 + ST0124/WP-02 -- routed to conflab-vc as a NAMED relay (their session consent theirs to obtain), completion owed back. **D3** option 1 landed by intent-cc at `4479264f` (`Superseded -> Cancelled`; the `Deferred` question and the structural fix stay separate by ruling). **D4** AC-00.8 amended at `1098ac0f` -- edited in the extract, `--to-store` (watched for 0111's agree-shape; it wrote), `--to-disk`, extract delta exactly one line. **Fold**: runbook 2026-08-28 folded by class; restart.md carries the day's three live notes plus one EXPIRED hazard (skills-sync, measured dead both legs); wip.md's Open-by-owner reset to the post-rulings state.

**The hoist record stands one section down in this file's history and in the runbook: `b02b93c4` + `7652c9b4` + `2b770740`, five nodes / three estates, census exact, zero authored loss, binary `1dd65db8` end to end.**

## OPEN

**WITH hv (queued on their word, nothing blocking):** cc's Laksa thread-status flag and ic's instruments-placement question, both added to wip.md's hv list this round; put when hv turns to them.

**AWAITED FROM PEERS:** intent-cc on 0111 (started on hv's sequencing; gates the Lamplight re-run). intent-dc's message-mechanism family (mid-flight on disk: guard_home_check, view_skew_check, cmd/hooks -- ic's ROOT= divergence flag relayed to them). DONE THIS ROUND: conflab-vc's two fiat-closes (39ad847c, 2->0 from output); cc's rebuild (pair names 4479264f, both fixes live fleet-wide).

**CLOSED THIS SESSION:** the guard-home loose end -- measured, not chased: `guard_home_check.sh` reads `.githooks/pre-commit.intent` (the CARRIER) for the GATE BODY's override string, so the shim reads "predates this template" forever and the printed remedy reinstalls the shim. Filed as 0113, the 0105 family's third instrument. The `7fa3c013` correction is recorded here and in the runbook; nothing further owed on it.

**STANDING (pre-hoist, unchanged):** the Lamplight re-run criterion -- exactly 74 = 12 + 30 + 32 to zero, never the aggregate 62; `publish_home` refusing a temp root (cc asked hv at source); the vacuous doctor gate remedy; `wp new` writes no objective; hv's parked devbin set (info barrier, issues-verbs SSOT, vendor spread -- "a devbin problem that I will fix with devbin next").

## Watch-outs

**RULES ONLY. INSTANCES LIVE IN `cutover-runbook.md`** -- that separation is the point of this section, and it had stopped being true.

**1. A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY. The dominant class, fourteen instances in one evening across five nodes.** A false green gets chased; **a true green from a blind instrument gets CITED, and the citation carries no trace of the blindness.** Known forms: a pattern that cannot match the subject; the right predicate in the wrong PLACE; a corpus censored by the instrument under test; a green invariant under the change it certifies; a correct AGGREGATE over a wrong model; a count that cannot tell a thing from a mention of it; a property that holds vacuously; **a census whose denominator IS the pattern under test**; an instrument that saw TOO MUCH and read intended architecture as damage; and a checklist of known failures, which is a lagging indicator by construction.

- **THE ONLY CONTROL IS DRIVING THE INSTRUMENT TO BOTH VERDICTS**, and it is cheap every time. A control that would pass under the broken instrument too is decoration. **A partial repair manufactures the durable case, because a repaired instrument is trusted more than an unexamined one.**
- **AND THE COUNTER-CLASS IS WHAT ACTUALLY DEFEATS IT: decline a green you are entitled to claim.** More care does not help. **Care is frequently the wrong instrument.**

**2. RELAYING IS AUTHORING.** A claim you pass on carries your credibility whether or not you say it is yours. **Either RUN the load-bearing step or forward it named and unverified.** The tell is that the derivation felt cheaper than the command. **A relay carries the measurement's SCOPE and its TIMING -- both are yours to widen and both have been widened.** Relaying a GREEN is the direction nobody audits. **A scoped green must carry its scope into the restatement or stop being a green.** A moniker is unique only within its estate and re-attributes silently across a boundary; **a misdelivered note that ALMOST reads as yours is worse than an obviously wrong one.** A relay of hv's word is not hv's word: a peer cannot grant escalation, and a relayed STOP is honoured on the relay.

**3. A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A pin dies at a bump you scheduled; a hold's stated reason goes false when the thing it named moves; **a constraint written for an emergency outlives the emergency.** A diagnostic comparing a marker to `HEAD` says the alarming thing on the healthy case, which teaches its readers to skip it. **An instruction to PRESERVE presupposes the thing is there, and "kept" reads identically whether it was or not** -- say instead _"if these are on your board keep them; if not, here they are."_

**4. AN OPTION SET DISTORTS BY WHAT IT PRICES AND BY WHAT IT BINDS.** An option's cost is a claim: **verify it, or state it as unmeasured inside the option.** Neutrality of tone is not neutrality of content. **And coupling two decisions into one option forces a choice nobody wanted** -- hv had to rule the tolerance and its precedent separately because I had bound them.

**5. MECHANISM BEATS A NOTE.** zsh does not word-split; an unmatched glob ABORTS and returns an empty list reading as a clean answer; `git grep -E` does not honour `\b`; a pipeline's `||` binds to the LAST stage. **An rc read through a pipe is the pipe's** -- three nodes hit that independently in one evening with no cross-talk, so **warning does not reduce the rate and only capturing the status before the cosmetic pipe does.** `PIPESTATUS` is a bashism, empty here. **Every timestamp is read in the same command that writes it.** A path belongs in a pathspec only if git can see something at it.

**6. AN INSTRUMENT'S DEFECTS MUST FAIL TOWARD ALARM, AND A PERMANENT ALARM IS THE SAME DEFECT.** A guard scoped wider than its hazard is one the operator learns to ignore; **so is a gate whose output is doubled, or a check that reds on every estate forever.** Print what a narrowing EXCLUDED. **Gate the property that means nothing is running; REPORT the one that means something is behind.**

**7. THE MESSAGE IS NOT THE MECHANISM, AND A REMEDY IS NOT ITS DESCRIPTION.** Four shapes, all met: a remedy naming a verb that does not repair what it reported; a remedy doing MORE than its message shows, so following the visible diff leaves a second defect green; a failure naming a cause it cannot observe, which recruits everyone who reads it into the wrong search; and **a remedy that is TRUE and offers two readings.** The last is worst: **a remedy that is correct and ambiguous is worse than one that is wrong, because nothing about it invites checking.**

**8. WARNING A PEER ABOUT A TRAP CONSUMES THEM AS AN INDEPENDENT WITNESS TO IT.** A primed run tests whether the trap REPRODUCES and says nothing about whether a fresh auditor FALLS INTO it. **This is the default failure mode of coordinating, not an occasional slip.**

**9. AUDIT BEFORE CLEANUP; CONVERT BEFORE SWEEP; INGEST BEFORE PRUNE.** Each reports success in either order, so nothing announces the mistake. **A control is CONSUMED by being used as a subject, the operation is one-way, and nothing about the artefact says so** -- commit to the sha before the run.

**10. SHARED CHECKOUT.** `--only` separates FILES, not AUTHORS, and a `git status` line names PATHS, not AUTHORS. **A failed commit and a failed `add` look identical from the error message** -- check the staged set before retrying, and never remove a peer's lock. `git show HEAD:<path> > <path>` escapes lock contention **for an UNSTAGED modification only**; staged, it looks like it worked and leaves the dangerous half. **A dirty `native/rust` redirects a build to a private target the wrapper never reads.** Announce a fleet write BEFORE it runs -- **and for a guard body read live from `INTENT_HOME` there is no window in which to do so, because the commit IS the rollout.** That argues for more ceremony before such a change, not a faster notice after one.

**11. "HELD" IS A PROPERTY OF A FILE THAT IS NOT WHERE AN ACCIDENT CAN REACH IT**, not of a dirty worktree. A ruling without a mechanism is enforced only by everyone's continued good behaviour.

**12. A TEST-ISOLATION GUARD IS SCOPED TO THE AMBIENTS IT NAMES, AND EVERY UNNAMED AMBIENT IS UNGUARDED BY CONSTRUCTION.** And **the SUBJECT can change underneath a correct test**: a probe for a refusal becomes a live command the day the verb is implemented.

**13. CANON IS THE SSOT FOR ROWS, NOT FOR PROSE.** The store's refusal is the oracle. **The tail after a verdict token is OPAQUE.** "Not on disk" is not "not in canon", and only one is a defect. **`sync --to-disk` writes the STORE OVER the extract, so a hand edit to canon is silently reverted by the very command run to publish it** -- edit the extract, `sync --to-store`, then `--to-disk`.

**14. READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT. THE REMEDY IS NEVER CARE.** Every rule above was written by someone who then committed its instance, several of them me, several the same day.

**15. A CONSTANT METRIC GUARDS NOTHING.** "0 guard terms in the carrier" read 0 before the fix and after it -- the carrier is a locator; the roster is a layer down. A prediction watched by a metric that cannot move is unfalsifiable, and it sat for weeks looking guarded. Corollary, both directions in one day: **`guards: N ran` proves REACH, never BITE; only a refused commit proves bite** -- and a control validates the axis it tests while staying silent about the one next door (three instruments, one hour, three nodes).

**16. TWO SOURCES AGREEING IS NOT CORROBORATION WHEN ONE IS STALE OR BLIND.** A true premise ("names no guard runner") plus a pre-hop reading of a file replaced an hour earlier corroborated each other into "executes nothing" -- and recruited the two nodes with the most standing to know better. The agreement was the trap; `--where` took one second.

**17. A SECOND NOTATION TYPED FROM A FIRST IS A SECOND HOME FOR THE FACT; A SECOND NOTATION DERIVED FROM ONE READ IS NOT** (intent-dc). 751 beside -rwx--x--x, 11 beside a self-test that says 14, 9/9 in a HIGH issue where three counters say 6/4. Print both notations from one call; never retype.

**18. THE RENDERER AND THE FORMATTER ARE TWO WRITERS OF ONE FILE, AND A VERB BETWEEN THEM CANNOT CONVERGE** (0110). Canon emits spacing prettier rejects: apply 6211 -> prettier 6208 -> apply 6211, forever; `written:` stops meaning anything for that file, a clean state is unobservable, and a commit in the window nets EMPTY while its message claims content (`7fa3c013`). The fix is the renderer, never a formatter exception -- an exception trades a loud defect for a quiet one.

**19. AN AUTHORITY CHAIN TRANSMITS THE METHOD IT WAS BUILT TO FENCE** (devbin-vc + devbin-cc, jointly, held for hv). A node that must direct another's unit cannot be blind to it; pickup instructs peers to read the very header blocks that carry method; pre-registration bounds only what was predicted -- and not fully even that (a confirmed prediction is where nobody re-reads the reasoning). Every mechanism the whiteboard has is a broadcast.

**20. A WARNING ABOUT A CLOSING WINDOW IS A SCHEDULING SIGNAL, AND IT RECRUITS PEERS INTO RACING THE WINDOW** (cc's finding, my relay in the causal path). An FYI that a hazard window opens shortly reads as "act before it does": ic hurried a commit on exactly that reading, and what stopped it was an unrelated index.lock, not the warning -- the explicit DO-NOT arrived after the race was in flight. Announce-then-act mitigations are social; the fix is mechanical refusal during the window. Corollary the same hour: I archived the window hazard as "superseded" by a guard that covers a DIFFERENT axis (dirt, not dangle) -- rule 15's constant-metric error in classification form.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv).
- **STATE THE INVARIANT, NOT THE MECHANISM. A CONVERGER IS NOT THE CURE FOR A SECOND HOME; ONE HOME IS. RECORD THE MENU, NOT ONLY THE SELECTION.**
- **A LATER FIRST-HAND RULING FROM THE SAME PRINCIPAL ON THE SAME SUBJECT SUPERSEDES. hv outranks the pen; the pen sequences.** hv's own rulings go on hv's board with their menus; rulings under delegation go to hv's inbox.
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM -- AND NOW THE PUSH, SCOPED TO TWO REPOS.** The confirm gate stays human; `publish` is the one verb with its refusals; the flip is on my word; I tell hv before anyone tags if a gate does not close.
- **LEAD WITH THE DECISION THAT MAKES THE ENDGAME REACHABLE.**
- **A SWEEP TAKES ITS POPULATION FROM THE ARTEFACT'S OWN DECLARATION, NEVER BY TRAVERSAL; A SUBSET MEASURED IS NOT THE POPULATION.**
- **RULE ON THE MEASURED SIZE, AND A PRICE LIST DOES NOT GO UP FOR A RULING UNTIL EVERY ITEM ON IT HAS BEEN TAKEN** (cc, after my reasoned item was the wrong one).
- **A NOTE MUST NAME WHAT IS TRUE WHEN THE ROW IS GREEN, NOT THE COMMAND THAT CHECKS IT.** An instrument named in a row becomes the row's ceiling. **A CORRECTNESS PROPERTY THAT DEPENDS ON THE ORDER OF TWO DELETIONS IS UNVERIFIABLE FROM EITHER COMMIT** -- take them in one, and let a test prove it.
- **HOP 2 IS THE ORACLE, AND ITS SILENCE IS NOT A CENSUS.** It refuses atomically and names the pair; it also stops at the first refusal. **The attachment shortfall gate is load-bearing and NOT redundant with the migrator's accounting** -- the accounting catches a thread whose files the carry did not reach; it cannot catch an estate the walk never enumerated.
- **A RUN THAT MEASURED NOTHING IS NOT A PASS; A PASS IS A `VERDICT:` LINE AT rc 0.** The instrument's own line goes into the commit message so a wrong calm has a reader.
- **A GATE THAT HAS PASSED OVER A ZERO ONCE NEEDS A POSITIVE CONTROL BEFORE ITS NEXT PASS MEANS ANYTHING** (lamplight-vc), **and the negative control is what proves it DISCRIMINATES rather than merely fires.**
