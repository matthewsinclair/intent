---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 14:14Z
status: paused
focus: "**PROBE FIXED AND LANDED (`d3dbeafa`) -- vc authorised it to jump the hold.** 9/30 all-pass before, **30/30 after**. Three fixes: the %ct coin-flip control (vc), and Lamplight's two. **AND A CLASSIFIER I BUILT, DROVE AND WITHDREW rather than shipped.** Everything else PARKED: no narrative until hv instructs. **v3.0.0 SHIPS THE 0133 DEFECT, with hv.** ON THE BOUNCE: hold for hv via vc."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. Nothing of mine dirty. Everything below is parked by vc pending hv's release ruling.**

## **v3.0.0 IS PUBLISHED AND SHIPS THE `0133` DEFECT**

**Four measurements, all re-driven independently by vc before it went to hv** -- their words: relaying is authoring, and this one changes a release decision.

- `v3.0.0` -> `80d8b2ca`, same sha on **both** remotes
- **`04cf6f18` (dc's fix) is NOT an ancestor of the tag**
- `model.rs` in the **TAGGED tree**, line 1070: `Unsatisfied,` -- a bare unit variant
- `gh release view`: **not a draft, not a prerelease**, published **2026-08-26T13:49:37Z**, three assets, **`downloadCount` 3 each**

**The tag cannot be moved. The fix is v3.0.1; sequencing is hv's.** Escalated live to vc AND durably to `hv/inbox.ic.md` -- releases are hv's, and a durable surface is what that inbox is for.

**UNRESOLVED AND NOT MINE TO RESOLVE: `AC-12.4` is recorded UNSATISFIED while its first two clauses are measurably DONE.** Either the criterion is stale, or **the release went out ahead of the criterion meant to gate it** -- the second reading means the gate did not gate. Recorded as unseparated rather than picked.

**HOW IT WAS FOUND, because the method is the transferable part:** I went to update my method doc after dc's fix landed, **expected the tag to be unpushed, and checked instead of assuming.**

## THE PROBE IS FIXED AND LANDED (`d3dbeafa`) -- vc RULED IT JUMPS THE HOLD

**vc's sequencing, and the reason is the keeper: the hold is on the RELEASE and the NARRATIVE, not a freeze on the estate.** A probe is a whiteboard instrument, not a release artefact. **The deciding argument was that the harm is LIVE rather than pending.** vc also **declined my cheap alternative on principle** -- I offered "tell the three estates not to run it yet" and they refused it as **a social mitigation where a mechanical one is available** (their rule 20). A warning protects only the estates you remember to warn, and expires the moment someone runs the tool unwarned.

**MEASURED BEFORE AND AFTER, 30 RUNS EACH: the committed blob is 9 all-pass / 21 FAILURE. The new one is 30 of 30.** vc's 4-of-12 and my 9-of-30 are the same 30%, sampled independently.

**1. THE CONTROL WAS A COIN FLIP AND HAD BEEN SINCE I WROTE IT (vc's find).** `%ct` is **whole seconds**; the old code sorted `(%ct, sha, path)` descending, so same-second commits fell through to comparing **SHA strings**. Every failing arm was a moved-thread arm, every stable one a flat filter. **I found the tie independently ~20 min earlier and found it the WEAK way** -- one failure while editing, traced, fixed. **vc ran it N times and discovered it had been flaky ALL ALONG**, including every run I had cited as evidence the probe discriminates.

**vc's BISECT NOTE IS THE TRANSFERABLE HALF: four refs each showed 8/8 on ONE SAMPLE, and they nearly filed "no regression, all refs clean".** **Bisect ASSUMES determinism, so aimed at a flaky subject it returns a confident, well-formed, wrong answer -- and nothing about the bisect can say so.** Repetition was the right instrument.

**FIX: ordering comes from `git log --all --topo-order`, not a sort** -- deterministic, no ties. The fixture **plants explicit dates** (v2 era -> bucket collapse -> hop) instead of committing in a burst, so it also models the timeline it claims to.

**2. LAMPLIGHT'S FINDING 1 -- the exit-3 guard was BINARY**, firing only at zero recovered, so 152/358 exited 0 with 206 threads unaccounted. **Accounting is now per-thread, the partition is ASSERTED not assumed, and the exit code is GRADED because the quantity is** (3 not-measured, 4 measured-but-incomplete).

**3. LAMPLIGHT'S FINDING 2 -- `*/acceptance.md` is a filename glob, not a thread predicate.** Now cross-checked against the estate's own canon; **where no canon exists the check SAYS SO rather than passing everything.** On Intent it excluded `lib/templates/prj/st/ST####/acceptance.md`, and **mutating the check off leaks that row straight into `EXPOSED`.**

### AND A CLASSIFIER I BUILT, DROVE, AND WITHDREW RATHER THAN SHIPPED

I split unrecovered threads by whether their oldest blob carried the `GENERATED VIEW` banner. **Driven on Intent it put 44 of 67 threads in "residue". I opened ONE instead of reporting it:** `ST0001`'s acceptance file is created **BY the v3 hoist** (`0ec2ac79`) and carries no banner because the early renderer emitted none. **Timestamps failed too -- the hoist wrote acceptance files ~25h BEFORE the first `.canon` record**, so canon arrival is not the port instant it looks like.

**UNDERNEATH BOTH, AND IT IS THE WIDER CRACK: v3's GENERATED `acceptance.md` USES THE SAME `-- satisfied:` SYNTAX AS v2 AUTHORED.** So the banner is **SUFFICIENT and NOT NECESSARY** and I have no reliable discriminator. Split withdrawn to one `UNCLASSIFIED` bucket naming both causes.

**THE CONSEQUENCE I FLAGGED TO vc RATHER THAN LEAVING IMPLICIT: if v3-generated files can satisfy my v2-authored test, the `recovered` population on ANY estate may be contaminated, and the exposure figure is computed over it.** Intent reads 0 exposed so nothing is wrong here; **I do not know that for Baize, Laksa or Prolix.** Not a reason to hold the fixed probe -- exit 4 makes it visible where the old build printed one confident number -- but **a reason not to call any estate measured on the strength of a green.**

**vc HAS WITHDRAWN THE WARRANT FOR LAMPLIGHT'S 25, NOT THE NUMBER** -- it rested on a single 8/8 from a control now known to pass ~30% of the time. Re-run N times requested.

## hv's RULINGS (14:00Z, first-hand in vc's session) -- AND MY UNIT MOVED ONTO THE CRITICAL PATH

1. **v3.0.1 IS THE NEXT RELEASE ACT.**
2. **WARN THE FLEET NOW, remedy or not, naming the PROPERTY to test (does the build carry `04cf6f18`) rather than a version to trust.** hv chose my method doc's own form; **`01afa12f` is the shape the warning goes out in.**
3. **I TAKE `AC-12.2` + `AC-12.3`. `AC-14.10` IS PARKED** on exactly the reasoning I gave -- documenting a file protocol `AC-14.12` rules for deletion is work done twice.

**AND THEN hv MOVED IT: THE DOCUMENTATION COMES BEFORE THE RELEASE, and the new docs ship WITH v3.0.1.** The release is HELD. **My unit is no longer downstream of the release -- it is the thing gating it.**

**DO NOT BEGIN THE NARRATIVE.** hv is writing the instructions; **their shape may not be the one vc and I scoped.** vc routes them the moment they land.

**WHAT THIS DOES TO `AC-12.3`, which I had flagged as unsatisfiable by construction if the docs waited:** hv's sequencing **removes the contradiction rather than working around it** -- the docs now precede the cut, which is what "release docs written BEFORE the cut" always said.

## `AC-12.4` IS THE WORSE READING, CONFIRMED

I gave two readings and said I could not separate them from where I stood. **vc measured the clause I could not reach: the tap formula is LIVE and pins `version "3.0.0"`.** So **all three clauses are measurably done -- tagged both remotes, release published, formula live -- and the criterion is STILL recorded UNSATISFIED.**

**It is not that only formula-live remained. THE RELEASE WENT OUT COMPLETE, PAST THE CRITERION THAT RECORDS IT, AND NOTHING NOTICED.**

**And it makes the defect worse than "the tarball is downloadable": `brew install intent` HANDS YOU THE DESTROYING BUILD.**

## LAMPLIGHT: 25 BLIND AGAINST PREDICTED 25 -- THE FIRST CONTROL THIS ESTIMATE HAS EVER HAD

Exact, independent route, **no number in hand.** **The first control on this fleet estimate that was NOT blind by construction** -- unlike Conflab, whose agreement was perfect and worthless for the axis that was actually wrong.

### AND THEY HANDED BACK TWO DEFECTS IN MY SHIPPED PROBE (credited to Lamplight, not paraphrased)

1. **THE EXIT-3 GUARD IS BINARY AND CANNOT SEE A _PARTIAL_ NON-MEASUREMENT.** It fires only at `files == 0`, so **at 152/358 it exits 0 and prints a confident figure with 206 threads unaccounted for.** They did the split by hand -- 197 v3-created, 9 genuine residue, **152+197+9 = 358, complete.** **The probe ALREADY HOLDS THE DATA to do it** (earliest commit time per acceptance path vs the port commit). **This is my own doctrine surviving one level down: the unit is PER-THREAD, not per-estate** -- I built the refusal at estate granularity and the subject is threads.
2. **`*/acceptance.md` IS NOT A THREAD PREDICATE.** It caught `design/system/handoff/intent/st/ST0334/acceptance.md` and `_inbox/` paths, **and `thread_key` extracted a PLAUSIBLE id from both** -- 9 non-threads, 5 with no canon record. **Cross-checking the key against the store settles it.**

**NOT YET FIXED, AND FLAGGED TO vc AS POSSIBLY NEEDING TO JUMP THE HOLD:** Baize (28), Laksa (10) and Prolix (3) have not reported, so **an estate running this instrument today can still get a confident, incomplete figure.** Not self-authorising past hv's sequencing; vc decides.

## vc's RULINGS ON EVERYTHING ELSE OF MINE -- ALL PARKED

1. **DO NOT START THE 55KB NARRATIVE.** `AC-14.10` is entangled with `AC-14.12` (the file-based `claude ws` family is ruled for deletion) and goes to hv with the release question. **A doc rewrite ahead of a mechanism change gets done twice.**
2. **ST0065's EMPTY CONTRACT + THE COSTED PROPOSAL GO UP AS ONE BATCH.** vc confirmed the channel had **no write**, not a write with no reader; nothing is owed by me for it.
3. **THE TWO PASSING-BUT-WIP THREADS WAIT** (ST0057 66/66, ST0061 7/7). Closing threads while the shipped tag is defective is an ordering question hv should see whole.
4. **THE PRE-`Fiat` RETRACTION IS ACCEPTED.** Zero fiat rows anywhere in the store, on a pattern **demonstrated to over-match** -- a `LIKE` that also caught a `satisfied` row merely mentioning "withdrawn" in prose. **A zero from a pattern proven to over-match is a sound zero**, and it cost nothing next to the rebuild it replaced.

**ASSIGNMENT 2 (surface-review cc's `ac new`/`at new`) STAYS PARKED.** vc pings me when cc lands; I do not watch for it. Reviewing an unshipped package reviews my expectations, which is the failure mode I would be reviewing FOR.

## THE TWO OPEN ITEMS ON MY OWN CLAIMS

**ST0065 HAS ZERO ACCEPTANCE CRITERIA.** Catalogued across two WPs and argued in a costed proposal, with **no ratified boundary behind any of it.** Not minting ACs unilaterally -- ratification is the open-gate. **I lean `acceptance: exempt`** and will argue it.

**THE COSTED PROPOSAL (`_proposal-agents-md.md`) RECOMMENDS OPTION 2: TESTED DUPLICATION.** The 575-byte four-rule index in both `_CLAUDE.md` and `_AGENTS.md`, delivered by `claude upgrade --apply` which already writes both, plus one arm asserting byte-identity. **A test, not a generator** -- the template engine has **no include form** (`rootfiles.rs:436-447` refuses an unknown token). **Stated limit: `usage-rules.md` cannot join** (seeded-once, user-owned, `canon.rs:316`), so it is three uncounted homes to two tested plus one declared exception, **not zero.**

## THE CANON NARRATIVE HAS HAD **ZERO** v3 CONVERGENCE (`AC-12.2` names it first)

Instrument positive-controlled first, because a bare zero from a grep is what I got wrong yesterday.

- **`working-with-llms.md`** 55272 bytes: controls 34 / 0. **`v3` 0, `3.0.0` 0, `v2` 15, `database`/`the store`/`SSOT` 0.**
- **`README.md`** 17207 bytes: controls 40 / 0. **`v3` 0, `database` 0, `SQLite` 0, `brew` 0** -- its only version reference is a v2.10.0 migration-guide link.
- **CHANGELOG is the outlier the GOOD way:** a written `[3.0.0]` entry stating DB-as-SSOT correctly. **The release note knows; the narrative does not.**

**THE TWELVE ACTUALLY REMAINING ON ST0056** (the gate's 66 mixes stored-unsatisfied with test-backed-not-green): `AC-00.5` brew clean machine, `AC-00.6` prune `bin/`, `AC-06.3` deviation register, `AC-10.6` rollback on canary, `AC-11.1` tap formula, `AC-11.4` checksum matches downloaded bytes, `AC-12.2`, `AC-12.3`, `AC-12.4`, `AC-13.9` T3+T4 staged, `AC-14.10`, `AC-14.12`. **Three are interface-shaped and offered: `AC-12.2`, `AC-12.3`, `AC-14.10`** -- all `n-a` ATs, so **the evidence quality IS the deliverable.**

## THE FLEET

**80, not 257. ELEVEN estates MEASURED, FIVE NOT MEASURED.** Baize **28**, Lamplight **25**, Conflab **14** (the only CONFIRMED one), Laksa **10**, Prolix **3**. Real measured zeros: Riffle, Courses, Devbin, Cdsync, Utilz, Intent. **NOT MEASURED:** Anvil, MicroGPTEx, Molt, Molt-flynn, Molt-matts. **Everything except Conflab is PREDICTED-UNCONFIRMED, in those words.**

## THE FOUR THINGS THAT MUST SURVIVE A COMPACT

1. **`0133`'s MODEL HALF IS CLOSED (`04cf6f18`) AND IS NOT IN THE SHIPPED TAG.** `Unsatisfied { note }` is a struct variant now and the ingest wildcard is three explicit arms. **This REPLACES the old entry, which said the field is unrecoverable -- that is now a fact about WHICH BUILD YOU ARE ON.** On v3.0.0 it still destroys; on a build carrying `04cf6f18` a re-ingest from the v2 source becomes a real recovery route. `method-ingest-damage.md` corrected at `01afa12f` to hand over the property, not the verdict.
2. **`ac new` DESTROYS FOUR PAYLOAD-CARRYING VARIANTS**, not just evidence: `Satisfied{e}`, `Descoped`, `Withdrawn`, `Fiat`. **A new payload variant silently enlarges every verb that rebuilds a row from scratch.** vc's narrowing, taken: **it dies at the CLI door, NOT at `put`** -- `put` keeps PUT semantics by design.
3. **THE v2 COMPARISON SOURCE IS IN GIT HISTORY at the ingest's own input path** (`legacy.rs:1273`). This is what made the survey possible at all.
4. **INTENT IS THE CORPUS THE PARSER WAS FITTED TO.** Never a fleet baseline or calibration control.

## Watch-outs

**BORN STALE IS NOT "A CLAIM OUTLIVING ITS BASIS", AND THAT RULE CANNOT REACH IT.** `89cdaffc`: my module header claimed the fixture "ALREADY carries L2 findings", false **by construction** (`write_thread` creates every cited file and writes the row id into it). **`git log -S` puts the false claim and its own refutation in the SAME COMMIT (`90988faf`)**, over a mechanism predating both by months. **Nothing decayed; there was never a moment it was true.** "Re-check when the subject moves" cannot catch it -- the subject never moved. **What caught it: cc wrote a precondition on the sentence they were relying on**, and it fired. I did the archaeology to separate born-stale from ordinary carelessness before calling it a class.

**`--only` SEPARATES FILES, NOT FACTS** (vc's phrasing, my instance). Two forms today, failing opposite ways: **the roster guard CAUGHT it and named the WRONG CAUSE** ("fix the row or fix the runner, whichever is lying" -- neither was; a node believing it deletes vc's row); **`bin/devbin` + its manifest are caught by NOTHING.** Caught-with-wrong-attribution and not-caught-at-all are the same defect. **A per-pair guard would be a roster of known atoms -- a lagging indicator by construction.**

**A CENSUS TAKES ITS UNIT FROM THE SUBJECT'S IDENTITY, NEVER THE FILESYSTEM'S.** 678 paths for 358 threads. The duplication half is an inflation; **the frozen-snapshot half is WRONG VERDICTS.** Reconciling the count without opening the rows would have fixed the number and left the class wrong.

**A CORRECT IMPOSSIBILITY ARGUMENT AIMED AT THE WRONG PROPERTY FEELS LIKE A SAFETY PROOF.** An argument about what cannot CHANGE says nothing about what a value MEANS.

**"NOTHING MEASURED" AND "NOTHING EXPOSED" ARE THE SAME OUTPUT UNLESS YOU MAKE THEM DIFFERENT.** A zero you cannot trust must not exit 0.

**A CONTROL CAN BE GENUINE, UNARRANGED, CROSS-NODE, CROSS-METHOD -- AND BLIND BY CONSTRUCTION.** The defence is asking what the control CANNOT see.

**A STAMP TYPED BY FEEL IS FABRICATION, AND THE MESSAGE CHANNEL HAS NO GUARD.** I wrote `14:0xZ` to vc today -- the same placeholder shape I was corrected for yesterday. Both commit guards run at commit time and **a SendMessage never reaches them** (issue `0099`). **Cite the commit, never invent a better-looking minute.**

**THE ARTEFACT IN FRONT OF YOU IS NOT THE ARTEFACT THAT SHIPS.** Read source with `git show HEAD:`; `--only` takes the WORKTREE. **VERIFY AN EXIT CODE BY CAPTURING TO A FILE.** **zsh: an unquoted `--include=*.md` ABORTS the whole command** and prints a plausible 0.

**MEASURE ONLY: nothing repaired, no estate written to, nothing about the release touched.**

## Decisions

- **(hv, via vc) FOLD+COMPACT FIRST**, ahead of everything else.
- **(vc) LAMPLIGHT GOES TO ITS OWN NODES; ic PACKAGES THE METHOD** -- a number produced from here cannot be checked from here. vc has recorded that the "56% of the fleet" half of their reason was my inflation.
- **(ic) EXPOSURE IS NOT DAMAGE, AND A PREDICTOR IS NOT A CONFIRMATION.** Say **predicted-unconfirmed**.
- **(hv, via vc) ALL ST0065 VERDICTS ADOPTED**; retirement and doc/skill legs complete.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING** -- the template engine has no include form.
- **(all nodes) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
