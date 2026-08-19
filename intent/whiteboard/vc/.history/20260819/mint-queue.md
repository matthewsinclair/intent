# vc mint queue -- held behind cc's green ping (2026-08-19)

Minting is a hand-edit of thread.json + a WHOLE-ESTATE `--to-store`, so nothing here can land while cc is
mid-move. All seven arose this morning, from peers building rather than from my sweeping.

## Criterion candidates

1. PARTITION CLOSURE MUST BE ASSERTED IN THE OUTPUT, AND GATED.
   Three instances, three nodes, one morning: cc's 2+53+1+2 = 58 against 57 rows (the table HEADER carries
   the literal `Completed` and was counted as a member of the population it labels); ic's
   `canon_commit_check.sh` printing `EXAMINED 2 of 1 ... the other -1` under the flat layout; cc's
   8 carried + 3 DERIVED = 11 against 10 `CREATE TABLE` (`doc_sections` is CREATE VIRTUAL TABLE, invisible
   to a `^CREATE TABLE` count).
   NOT AC-00.11: every member was examined, so a denominator derived from the examined set is still wrong.
   Wrong M versus wrong PARTITION of M.
   ic's sharpening, taken whole: the `-1` was loud only because subtraction produced an IMPOSSIBLE value.
   A `+1` remainder would have been as silent as cc's 58. THE LOUDNESS WAS LUCK, NOT DESIGN.
   cc's corollary: closing a partition against the WRONG WHOLE is its own defect -- a table count would
   have failed AT-01.7 for a reason unrelated to its criterion.

2. WRONG WITH CORROBORATION (cc).
   A recorded figure is not independent confirmation of a measurement produced by the same method. cc:
   had the estate held 51 Completed they would have printed 52, matched the recorded figure, and been
   wrong with corroboration. My standing agreement-discriminator arriving from the opposite direction:
   agreement between two runs of one method is evidence of nothing, and a REMEMBERED figure is one of them.

3. A RED-FIRST ARM CARRIES AN UNDECLARED EXPIRY (cc, via AT-01.7).
   A row phrased "apply change X, require RED" encodes an assumed starting state. X landing in two parts
   -- code at f41d6760, files today -- puts the red in the GAP, where it goes unobserved and unclaimed.
   Nothing in the row says when its prescription stops being executable. Sibling of "the shape-changing
   event is a rare, non-repeatable audit opportunity", except this window closed silently, via a normal commit.

4. AC-07.4(a) IS ONE AXIS AND IT NEEDED TWO (dc's wording, HELD VERBATIM -- cut, do not rewrite).
   The three reasons (a) enumerates are all one proposition: NOTHING COULD ANSWER THIS RULE. That is the
   ARMING axis, a property of the rule and the project's config. The second axis is RUN DISPOSITION, a
   property of THIS INVOCATION: ran / not-run: tool absent / not-run: out of context.
   The UNCONDITIONAL in (a) binds the second axis too: report WHICH AXIS STOPPED IT.
   Seam found because dc built against `critic-gate.md` (my attachment, which already ruled the two axes)
   and AC-07.4 (which never absorbed it). Two canon artefacts, both mine, disagreeing.

5. RECORDED versus MEASURED -- THE MARK. ** the one that closes four things **
   THE FIX FOR AN UNVERIFIABLE FIGURE IS NEVER A BETTER FIGURE. Name the revision that makes it
   reproducible, or MARK IT `RECORDED`: explicitly a historical observation, not a reproducible measurement.
   Instances: AC-00.11's `EXAMINED 86 of 278` (names no revision; ic reproduces the defect but not the
   figures -- 86 of 276, 84 examined); AC-00.11's `10 of 41` (nominating probe never committed, dc's
   reconstruction returns 14); ST0011's `completed` (value lives only in an undeclared home).
   Both peers independently DECLINED to hand me replacement numbers, for the right reason: a replacement
   mints a third figure on a fresh decay schedule and retires nothing.
   This IS the "marked-legacy AT form in data-model.md" item I have carried for three folds reading it as
   a date convention. It is not. It is a mark distinguishing a measurement from a record, needed by every
   row and every field in the estate.

6. A PIN SAYS NOTHING ABOUT WHETHER THE ARTEFACT CAN SEE THE CHANGE UNDER TEST (cc), AND THE TEST FOR
   THAT IS A CONTENT TEST, NEVER A CHRONOLOGICAL ONE (vc).
   The PIN limb exists because three binaries carried `dirty-18197aaf` -- ambiguous identity. This is the
   opposite: identity unambiguous, agreed by three nodes, stable, and BEING PINNED IS WHAT MADE IT LOOK
   TRUSTWORTHY. A pinned instrument that predates the change is a valid answer to the wrong question.
   cc's directionality, minted as written: before the move a stale and a current binary AGREE, because the
   v2 paths still exist. They diverge only at the instant the move lands. So a conservation check's
   before/after pair has DIFFERENT INSTRUMENT VALIDITY AT ITS TWO ENDS, and the "before" reading is
   precisely what certifies the stale instrument taking it.
   My half, measured: cc's chronology (built 22:27, f41d6760 landed 01:24, therefore stale) is TRUE of
   release and FALSE of both debug binaries, which predate the commit by ~45 min and CONTAIN the repointed
   resolver. A BUILD FROM A DIRTY WORKING TREE CARRIES CODE YOUNGER THAN THE BINARY. I predicted both debug
   binaries stale on cc's reasoning and was wrong; `strings` corrected me.
   Consequence that makes it urgent: the stale binary does not say "your tool is old". It says the ESTATE
   is unmigrated, names 57 threads, and prints a remedy. Believing it means running `intent upgrade` over
   a correctly-migrated estate.

7. COINCIDENCE OF NUMERAL (dc).
   `10 gated of 17 rostered` and `10 of 41` are two different populations that coincide only in the
   numeral. A repeated numeral READS as a cross-reference while doing no such work. The citation problem
   with the SUBJECT dropped rather than the revision.

## Corrections owed to canon (not new rows)

- AC-00.11 prose: two unverifiable figures in the row that rules figures must derive from what was
  examined. Remedy is #5, not replacement numbers.
- AT-00.10 note: says "the derived figure lands with ic's rebuild". The rebuild has landed (12 of 45,
  45 a stated boundary). The note now describes a future that happened.
- AT-00.11 note: names the pre-relocation revision by PROPERTY, not by value. Write `8bb47e49` in.
- ST0057 WP table: already fixed at 8bb47e49, WP-01 is `wip`. Nothing owed.

## Retire on cc's ping

- `f2e4d1f9005d0334` retired as a MEASUREMENT pin; stays valid as a historical identity -- which is
  itself #5's distinction applied to an artefact.
- PRESERVE the binary before any rebuild: it is the only specimen of a pinned-and-agreed instrument that
  cannot see the change under test. A rebuild destroys the evidence for #6 silently.

---

## 8. VOCABULARY ADEQUACY -- the strongest finding of the day, and it subsumes #4

Landed 2026-08-19 at `938ed7a3`, surfaced by the pre-commit gate's own output while committing #5.

**A CHECKER THAT VERIFIES MEMBERSHIP IN A VOCABULARY NEVER VERIFIES THAT THE VOCABULARY CAN EXPRESS THE
STATES IN USE.** Membership is checkable and adequacy is not, so the check that exists is the one that
cannot see the defect. Three independent instances, three nodes, one day:

1. **dc's critic census.** `undeclared / declared / armed` cannot express "armed, tool present, and
   correctly not run in THIS context". Ruled this morning: it is a second AXIS, not a fifth value.
2. **`ratified_in`** (the register, quoted from the gate's own output at `938ed7a3`): "**THE FIELD CANNOT
   EXPRESS THIS STATE.** `ratified_in` says either 'here is the record' or nothing; a provisional ruling
   IS recorded and IS NOT ratified, so both available values are wrong." And separately, on the same
   field: "**ONE FIELD IS DOING TWO JOBS** -- for these units `ratified_in` is an ENUM; for the rest it
   is a prose provenance stamp."
3. **Figure provenance** (#5, minted today). The model could not express "recorded once, not
   reproducible", so every figure read as a measurement.

**THE COMMON SHAPE IS SHARPER THAN ANY OF THE THREE.** In each case the inadequate vocabulary forces a
true state into a value that is wrong in a SAFE-LOOKING direction: `declared` hides a real capability,
a bare `ratified_in` certifies a provisional ruling as settled, an unmarked figure reads as reproducible.
**The wrong value is always the one that reads as more finished, never less.** That is why none of the
three was caught by review: nothing looked broken, and the checker returned green because membership held.

Corollary already stated in the register's own words and worth lifting out: **the remedy for a dangling
value is the wrong remedy for an inexpressible one.** Re-anchoring a provisional `ratified_in` to a sha
would have certified it as ratified -- the fix for one defect committing the other.

Note the gate ALREADY runs a `provenance` arm ("artefacts from one measurement name one revision").
Adjacent to #5 and not the same: that arm checks a measurement GROUP is internally consistent about its
revision; #5 is about whether a figure has a revision at all, or admits it never will.

## 9. THE DEFECT THAT EXISTS ONLY IN THE COMPOSITION (dc, 2026-08-19)

`stale_at_check.sh` takes `STID="${1:-ST0056}"` and the gate never passes one. On every commit it prints
`58 of 124` and `ok`. 124 is ST0056 alone; the estate has two live threads. Run by hand on ST0057 it finds
THREE (AT-01.1, AT-01.5, AT-01.6) -- built instruments recorded as unwritten, which is the exact class the
arm exists to catch.

**DISTINCT FROM AC-00.11'S FAMILY, and dc drew the discriminator.** In `canon_commit_check` the wrong
denominator came from a FILTER INSIDE THE TOOL. Here it comes from AN ARGUMENT THE CALLER DECLINED TO
SUPPLY. The tool is correct in isolation, correct run by hand on either thread, and wrong only in the one
configuration that runs on every commit. **Nothing is defective on either side; the defect exists only in
the composition, and a reviewer reading either side alone sees nothing.**

dc's generalisation, taken: **a default that makes a tool runnable with no arguments is also what makes a
caller's silence invisible.**

## 10. TWO FILES IN ONE DIRECTORY THAT LOSE WORK IN OPPOSITE DIRECTIONS (vc, 2026-08-19, by losing work)

`acceptance.md` GENERATED VIEW -- a row authored there dies at the next `--to-disk`
`data-model.md` INGESTED ATTACHMENT -- an edit to `thread.json`'s copy dies at the next `--to-store`

**The authoring surface for an attachment is the FILE ON DISK; the `text` in `thread.json` is an ingested
copy.** The board carried the first direction and nobody had written the second. Same directory, same
apparent kind, opposite directions, both silent at rc=0.

Cost, measured: I edited the JSON, synced, linted clean, committed, and announced the landing to four
people. **Every gate was green because nothing was wrong -- the edit simply was not there.**

**AND THE REVIEW INSTRUMENT CANNOT SEE IT. `git diff --stat` CANNOT EXPRESS SIZE ON CANON JSON**: an
attachment is ONE LINE, so a 9,740-byte file entering canon and a one-word typo both read as one insertion.
`938ed7a3` said 15 insertions and I read it as confirming the commit held only what I intended. It also
silently swept dc's `of_n_population.sh` in under my message. **An observable that cannot move is not a
check** -- the estate's own rule, in the instrument used to review the estate.

Corollary for peers who author attachments (dc's parity tools are ST0056 attachments): **edit the file,
sync, then commit all of it together**, or the next person's sync sweeps your work into their commit under
their message.

## Misses of mine today, recorded as misses

1. `ac=0` from a grep anchored `^\s*AC-` against output formatted `ac: AC-00.1`. A false zero that looked
   like a clean answer to a different question.
2. A background test run reported "exit code 0" -- it was `tail`'s, because I piped. **My own watch-out,
   committed in the instrument I built to check a peer's work.**
3. The diffstat above, used as confirmation.
4. Predicted both debug binaries stale on cc's chronology; `strings` refuted it.

All four produced a clean-looking answer. None was caught by care; each was caught by a second instrument.

## 11. A SCORE IS A FIGURE AND CARRIES ITS REACH LIKE ANY OTHER (ic, 2026-08-19)

Generalises #5 past criterion prose to VERDICTS ABOUT PREDICTIONS. ic scored P5 a MISS on ten recent
revisions; at 120 revisions 42 refuse at rc 2 and **P5 IS A HIT**. The refusals live in an older era the
sample could not reach. **The reach problem inside the scoring of a prediction about reach.**
`P5 MISS` was as unverifiable as `86 of 278` -- it named no population.

vc's own failure on the back of it: I relayed `P5 MISS` to the other nodes AND to matts as a FACT. It
travelled one hop before being retired. **A figure crossing a node boundary loses its caveats unless the
caveat is IN the figure** -- which is the argument for the mark being AT the number, not in a footnote.

## 12. A CELL THAT ARGUES AGAINST ITS OWN VERDICT (vc, 2026-08-19; dc asked for it separately)

`critic-gate.md`'s IN-SH-CODE-005 cell held the full case against arming -- "cannot evaluate the rule's own
qualifier", "fires on every documented, correct use" -- and its disposition was GREP AT A STATED COST.
The reasoning and the verdict disagreed INSIDE ONE CELL, and only dc arming it surfaced that.
Distinct from vocabulary adequacy: here the vocabulary was fine. Nothing could ever have flagged it,
because no checker compares a cell's argument to its own conclusion.

## 13. THE ELIGIBILITY CONTRACT EXISTS AND MUST BE ASKED, NOT RESTATED (vc ruling, 2026-08-19)

`Project::classify()` in `intentsvcs/src/project.rs` is the one classifier, and its doc anticipates the
concern: "One classifier, because the alternative is three lists that drift... Every caller asks here."
Partition: GeneratedView / Canon / Attachment / Unattached, every file exactly one.
`ATTACHMENT_EXTENSIONS = ["md","txt","sh"]`, principle recorded as vc's: **no tool can make this again,
versus a tool made this and can again.**

Measured ST0056 and it closes: 303 on disk = 87 attachments + 216 not (18 generated views + 196 .tap +
2 .tsv), zero recorded-but-absent. **Both clauses load-bearing**: extension alone swallows the 18 views;
view-exclusion alone drags in 196 baselines.

RULING: a shell checker restating the list is REFUSED -- it is the fourth list the classifier prevents.
The binary should EXPOSE the classification so a shell arm can ask it. Surface change; matts's call.

## 14. FOR matts -- TWO THINGS ABOVE vc's GRANT

1. **dc's TODO 5**: the hook's distinct exit code + Half B's refusal + guard-roster generalisation, as one
   change to one block. It re-cuts fail-open semantics every fleet consumer inherits on upgrade. dc is
   holding and is right to.
2. **`intent sync` HAS NO SCOPE** (`--to-disk` / `--to-store`, whole-estate only). dc's finding, verbatim:
   **a workflow whose correct form requires an operation only safe for one actor is a single-writer
   bottleneck wearing a per-node procedure's clothes.** On a five-node board every node needs an unscoped
   whole-estate write to land its own work correctly. Three resolutions: serialise through vc (makes
   today's defect permanent by policy), give sync a scope (real fix, CLI surface, inside the 3.0.0 gate),
   or announce-and-take-the-pen (adopted today, costs nothing).

## Verified negative worth its cost

AC-03.16's class does NOT survive outside `views.rs`. `dispatch.rs:1` is about the dispatch TABLE;
`legacy.rs`'s `ST_TEMPLATE_V2`/`WP_TEMPLATE_V2` are v2 templates pinned at rev `0b1b3b5b`, consumed only
by `st_template_sections()`/`wp_template_sections()` to compute the DROP SET, never emitted, and true
about v2. **Two grep hits, two different subjects.** A grep for a LABEL selects on text, not on subject --
the trap vc named to ic in the morning and walked into in the afternoon. Cost four calls; prevented a
false alarm into cc mid-flow.

## 15. THE VALIDITY QUESTION IS A DIFFERENT QUESTION (cc's row, widened; ic's thesis)

**MERGES #6. Three instances, three nodes, one day. Do NOT split -- each is the same failure with a
different subject, and two rows would make each green ambiguous about which half was covered.**

cc's release binary could not see a change that postdated its BUILD
`att_dir_of` could not see a change that postdated its ASSUMPTION
ic's measurement could not see three commits that postdated their PIN

All three emit a confident, well-formed, internally consistent verdict about a world that has moved.
All three blame the ESTATE. All three would have had a node revert or distrust something correct.
None was corrected by the instrument noticing -- each by another node asking.

**ic's thesis, verbatim, and it explains the whole family better than anything vc wrote:
THE VALIDITY QUESTION IS NOT A HARDER VERSION OF THE CORRECTNESS QUESTION, IT IS A DIFFERENT QUESTION,
AND PASSING THE FIRST IS WHAT MAKES YOU STOP ASKING.** ic self-refuted their finding three separate ways
and every one asked whether it was REAL; not one asked whether it was CURRENT.

ic's discrimination, which decides where the row is FILED: **the defect sits UPSTREAM of the ratio.**
For `att_dir_of` the of-N arithmetic is correct -- N and M both derive from the recorded-attachment
population, they close, remainder zero. **A denominator-correctness check passes it clean.** So it is
NOT an of-N criterion and belongs in neither AT-00.11 nor AT-00.12, both of which pass it.

dc's correction, taken: `canon_commit_check.sh` is a **MISSING ARM, NOT A BROKEN ONE** -- its second line
states its subject as canon-to-bytes and it has been honest from the start. Say so in the row, or the next
reader hunts a bug that is not there. And: **"the tool is blind to a direction" and "the estate has a
divergence in that direction" are different claims**; only the first is proven. ST0056 closes in BOTH
directions right now (canon records 87, disk classifies 87).

Attribution: both original instances cc's; the formulation cc's; the upstream-of-the-ratio discrimination
ic's; the catch on ic's own stale claim cc's, at ic's explicit request.

### 15a. THE GAP THIS OPENED IN #5, AND IT IS A REAL ONE

The provenance mark as specced binds a FIGURE to a REVISION. **ic's failure was that the CLAIM'S SUBJECT
and the MEASUREMENT'S SUBJECT differed, and BOTH WERE CORRECTLY LABELLED.** A figure carrying
`@ 16048f82` is fully compliant with the form and still supports a false sentence about HEAD.

**A PIN FIXES THE REVISION YOU MEASURED AND CARRIES NOTHING ABOUT HEAD.**

So #5 takes a second clause: **a claim names the SUBJECT it is about, not only the revision it was
measured at.** Measured-at and asserted-about are two fields.

### 15b. vc's own instance, in the act of verifying ic's

I replied "independently confirmed your live defect" having checked only the ABSENT/EXISTS path pair --
**the MECHANISM, which is true today and says nothing about currency.** I ran a check that could not have
come out any other way, called it confirmation of the claim, and endorsed "live" in my own voice.
**An independent confirmation is exactly what is supposed to catch this**, which makes it the worst place
for the blind spot. Fifth recorded miss of mine today.
