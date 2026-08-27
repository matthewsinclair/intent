---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-27 22:45Z
status: active
focus: "EOD, HOLDING, START NOTHING. All four nodes folded and clean; pair `d395a5b5` current-but-8-inputs-stale and a rebuild is owed with a green check I could not get once ic and dc had compacted. Globalfold done: runbook 337KB to 41.6KB, and it caught two documents asserting finished work as pending -- restart.md`s top hazard and the port ledger. hv`s standing directives (70KB, 58 entries, mostly spent) are NOT folded: they are hv`s word and the LIVE/SPENT split is theirs to authorise."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`. THE DAY'S FINDINGS, WITH ATTRIBUTION, LIVE IN `cutover-runbook.md` UNDER LEARNED.** Neither is repeated here -- a rule in two homes drifts in both, and this board is the one that gets folded.

## DOING

**NOTHING IN FLIGHT. hv's ROLLOUT IS DELIVERED AND SIX OF THEIR RULINGS ARE EXECUTED.** Pair `d395a5b5`, CURRENT by property, verified green **at the commit it names** rather than at HEAD (ic: 1374 passed, 0 failed). Two build windows, 54s and 69s, one node building, peers told before and after by properties.

**THE HEADLINE IS STILL A NEGATIVE: the sweep hv reserved does not exist.** No v3 code path wrote `pre-commit.intent`; (B) built that capability into `apply` (`f8a78e05` + `22a75509`), ratified by hv as the right shape. **devbin-vc's framing is what made the menu decidable: the guard BODIES move with no ceremony available, the CARRIER cannot move at all, and both halves are one asymmetry.**

**LANDED TONIGHT:** porter's BOTH citation defects (`e935734d` + `eff618e8`), tolerance 0 (`3463f784`), the third stamp surface (`27b13f93`), AC-11.6 with a pty harness the estate never had (`102af78f`), AC-14.7 as one transaction (`05222011`), the `--to-disk` remedy AND its false premise (`04bc607f`), `--severity` enforcement (`8174de80`), doctor's gate check (`3805f359` + `41c3e3f1`), the doc gate (`6c380e09`), (A2)'s body **inert** (`3b0063f3`), the schema faces (`51c9721e`, `3df927f4`).

**MY OWN ERRORS ARE ON THIS BOARD WITH THE DELIVERIES, because the count matters more than any one:** a tolerance ruling I had told devbin-vc I would not make, which they escalated correctly; two Laksa claims damaged in relay; a peer's narrow green restated as a claim about the tree; a denominator from the wrong arm **while correcting an error of that kind**; a hypothesis built on a failure message that named a cause it could not observe; a positive control that failed twice; a generosity that retired a finding before it was understood; a stale "hv is AFK" repeated to five estates; a hold I ruled without giving it any mechanism; **and a relay to cc that widened a measurement's scope AND its timing at once.**

## OPEN

**WAITING ON A NODE, NOT ON hv:**

1. **`main` IS RED** -- dc's `3b0063f3` trips `every_shipped_consumer_is_declared` (`hooks/critic-guard.sh -> intent critic, not declared in CONSUMERS`). **One line, dc's, and it blocks the next rebuild rather than anything live.** The delivered pair is unaffected.
2. **cc's BOOTSTRAP GUARD** -- ic proved an ordinary `cargo test` of the crate pair WROTE `$HOME/.intent/home`; cc fixed the writer at `9c2ba9ed` and is making the class unreachable at authoring time. **hv repointed the pointer; verified `state: OK` rc=0 with a negative control.**
3. **THE (A2) ROSTER LINE** -- string agreed (`rule violations in staged code are UNCHECKED`), waits on (B) reaching estates so the critic does not run twice. **dc announces to the fleet BEFORE committing it; that is part of hv's ruling, not dc's intention.**
4. **`st list` SHOWS THE TITLE** -- hv's ruling, and it is the SMALL version after they overruled my spec. **The title is the SSOT; the slug is a derived, URI-friendly rendering of it, not stored data.** Step 1 tonight is one render change: today `slug` is the table's ONLY descriptive column, which is why ST0001-ST0021 render blank and the other 43 read as truncated noise. Step 2 is a `--slug` option that GENERATES from the title. **No backfill, no stored slugs, no migration.** The 43 existing stored slugs are now vestigial -- flagged, deliberately not ripped out tonight.

**WITH hv:**

5. **THE LAMPLIGHT RE-RUN.** Criterion now arithmetically complete: **12 + 30 + 32 = 74 is the WHOLE broken population** -- so **exactly those 74 to zero, split three ways, never the 62.** hv saw it live and said wait, which is a LIVE non-release rather than a stale one.
6. **CONFLAB'S SEVEN ST0121 RESIDUE ROWS** -- `intent ingest` refused the migration. Repairing them is hv's, complicated by the remedy naming v2 tooling not installed there.
7. **`publish_home` REFUSING A TEMP ROOT** -- cc is asking hv at source. Cost: `install.rs`'s fixture arms publish temp roots legitimately.
8. **THE VACUOUS GATE** -- a `doctor` remedy offering two exits without discriminating them. **An empty contract is refused outright, so the hole is thin-but-nonempty.** Intent shows 0 findings, so nothing here to mis-remedy.
9. **`wp new` writes canon and no file; `wp done` leaves an existing file untouched** -- two representations, one writer. And `project.json` is TWO different files across estates, not one question.

**THE DAY'S FINDINGS AND THEIR ATTRIBUTION LIVE IN `cutover-runbook.md`. It is 337KB and is the next thing to fold; this board is not its index.**

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
