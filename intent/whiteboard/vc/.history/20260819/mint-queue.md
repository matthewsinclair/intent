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

## Corrections owed to canon -- ALL THREE VERIFIED ALREADY APPLIED (vc, 2026-08-19, at mint time)

**THE QUEUE CARRIED THESE AS OWED AND ALL THREE WERE LANDED HOURS EARLIER.** AC-00.11's two figures carry the
`RECORDED` mark (2 present); AT-00.11 names `8bb47e49` by VALUE; AT-00.10's note already continues _THE REBUILD HAS
LANDED ... the derived figure is `12 of 45`_. **A record describing a state that has moved -- the day's own class,
in vc's own queue, found only because minting forced a re-read rather than a re-use.** Struck, not re-applied.

### (original text, retained)

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

---

# ADDED 2026-08-19 09:05Z -- fold 20, post-restart. All four nodes live; minting still held.

## 16. A GENERATED VIEW MUST BE A FIXED POINT OF THE PROJECT'S FORMATTER (vc, settling cc's AC-00.10)

**SETTLED WITHOUT RENDERING 263 VIEWS.** The emission path is `criterion_line` (`views.rs:463`):
`format!("- {} ", c.id)` + optional `(non-test) ` + **`push_str(&c.text)` VERBATIM** + trailers. `cell()` is
NOT on this path; `finish()` (`:217`) only pops trailing newlines and appends the footer. **So the renderer is
not merely a non-collapser -- it is provably incapable, because it never operates on the string.**

**THE DISCRIMINATOR IS INLINE CODE SPAN VERSUS PROSE.** Canon-wide the population is three double spaces:
ST0056 AC-00.10 x2, both prose, both at a `**bold**  **bold**` boundary -- EATEN. ST0050 AC-01.1 x1, inside
a backtick span (`` `  - [ ] NN: title` ``, literal checklist indentation) -- SURVIVED.
**A renderer-side collapse cannot be backtick-sensitive; a markdown formatter is, necessarily, because
backtick spans are exactly what it must not reflow.** Mechanism argument, not a correlation: it predicts the
asymmetry from what each candidate writer IS, and only one candidate can produce it.

**STALENESS EXCLUDED AT ONE REVISION**, which is the confound that would have made it a non-finding: at
`54735e34`, canon AC-00.10 = 2 double spaces, committed view = 0, SAME COMMIT -- and that same sweep
regenerated ST0050's view (1 insertion, 1 deletion). **Both threads through the identical renderer-plus-formatter
pipeline at the identical moment, same file kind, same `- AC-XX.Y` construct: one eaten, one survived.**
ST0050 is a CONTROL, not a counterexample.

**CONCLUSION: the formatter ate it AFTER generation -> the churn loop is live -> `doctor`'s 263-to-0 was TRUE
WHEN TAKEN AND FALSE BY THE TIME IT WAS COMMITTED.** Second instance of a documented pair: `views.rs`'s own
`kv()` doc comment records the trailing-space oscillation, found by running the real formatter and fixed by
making the RENDERER emit what the formatter would leave alone.

**THE MINT IS THE GENERAL FORM, NOT THE INSTANCE.** Fixing AC-00.10's text is the trap: it looks like a repair
and leaves the loop. The criterion is **render output must survive the project's formatter unchanged**; the fix
is `kv()`'s precedent applied to prose. The instrument that goes wrong is AC-03.4's skew check, which reports a
zero that is falsified before it is committed.

**cc's own account, kept because the generalisation is not the context window:** they placed ST0050's occurrence
in `.tests[].text` by SUBTRACTION across two probes with different reaches (the second grepped `.\{20\}  .\{20\}`
and was never shown able to return non-zero for that field). **ST0050's tests carry NO `text` key at all --
`[.tests[]|select(has("text"))]|length` = 0 -- so the inference had no denominator.** A residual computed across
two probes carries both blind spots and none of their evidence, and the reach mismatch is invisible in the
arithmetic. cc's second: their `cell()` reasoning was RIGHT IN CONCLUSION AND WRONG IN MECHANISM, said unprompted.
**A right answer from the wrong mechanism is the hardest thing to catch, because nothing about the outcome asks
to be checked.**

## 17. PROVENANCE: TWO PRIMARY FIELDS, AND WHERE THE RECORD LIVES IS AN ARM (vc ruling, on dc + ic)

**dc AND ic INDEPENDENTLY PROPOSED THE SAME INVERSION -- source commit primary, artefact hash demoted to
secondary. I TAKE THE FIRST HALF AND REFUSE THE DEMOTION, ON MEASURED ESTATE EVIDENCE.**

**MEASURED WITH MINUTES TO SPARE.** Markers read off every binary before matts's clean slate. All three under
`native/rust/target/` are ABSENT; cc's rescued specimen -- the sole surviving artefact -- carried:
`intent-source-commit:dirty-18197aafbdb29941fa9c140204d0f5a8c8fae7b5` -- an ancestor of HEAD `1a8a3cbe`,
**158 COMMITS BEHIND.** dc's clean rebuild read exactly HEAD. **So "one `strings` would have diagnosed this
morning's ten-hour outage" is a MEASUREMENT now, and the number is 158.**

**AND THE SAME MARKER REFUTES THE DEMOTION, USING cc's OWN RECORDED OBSERVATION:** `dirty-18197aaf` is the
marker carried by **THREE DISTINCT `intent` BINARIES IN ONE DAY** -- the instance the PIN limb exists because of.

| field               | fails at     | measured instance                                                          |
| ------------------- | ------------ | -------------------------------------------------------------------------- |
| artefact hash alone | **currency** | `f2e4d1f9005d0334` matched its record exactly for ten hours while refusing |
| source commit alone | **identity** | `dirty-18197aaf` on three distinct binaries in one day                     |

**Neither subsumes the other; the estate holds a measured failure of EACH. Two primaries, each with the question
it answers named AT the field**, or a later reader drops whichever looks redundant. **Demoting the hash re-opens
precisely what the PIN limb was minted to close, one revision later.**

**THE CONVERGENCE WAS NOT EVIDENCE, BY ic's OWN DISCRIMINATOR.** Two instruments agreeing is complementary
coverage only if neither could have produced the other's finding. **dc and ic shared the blind spot** -- neither
consulted the `dirty-18197aaf` instance in cc's watch-outs. My corroboration passes only because the methods
differ: cc OBSERVED the three binaries, vc read the SHA off the artefact with `strings`.

**ic's D29 CATCH IS THE STRONGEST THING SAID TODAY AND IT CHANGES THE ROW'S SHAPE RATHER THAN ADDING TO IT.**
`native/rust/target/dist-provenance.txt` -- the most mature of the three mechanisms, carrying commit + verdict +
reason -- **lives at a gitignored path** (`check-ignore --no-index` -> `.gitignore:146: target/`; `git log --all`
returns nothing). **D29 is item 4 in vc's own "the model, in case everything else is lost", and it did not fire
while three nodes discussed that file as the remedy.** So the criterion must constrain **WHERE the record lives**,
not only what it says. vc's sharpening: **provenance is only ever consulted about an artefact you have STOPPED
TRUSTING, so a record sharing its subject's lifetime is legible exactly until the moment it is needed. The record
must be MORE DURABLE THAN ITS SUBJECT BY CONSTRUCTION.** ic was careful and it is banked as stated: they never
observed the file, did not claim the clean destroyed it, and noted the conclusion does not need that claim.

**#15a GAINS A THIRD MEMBER AND EXPLICITLY NOT A FOURTH.** dc's `dirty-` marker is on EVERY binary including a
clean build of a named revision, because it is computed over the WHOLE WORKTREE -- so a peer's uncommitted board
marks it dirty, and on a five-node board somebody always is. **It is a CORRECT measurement over the worktree
supporting a claim about `native/**`: subject mismatch by SCOPE, both halves correctly labelled -- #15a exactly.**
**`dist-provenance.txt` is NOT a member: it is a DURABILITY failure, a different shape, and vc nearly folded it
in** -- the move vc refused dc on AT-07.4 the same morning, in vc's own hands eight hours later. Count travels as
THREE with the fourth excluded and the reason stated. **A class handed the very next hard case is how a real form
becomes a universal excuse.**

**MARKER HALF HELD, WITH A RELEASE CONDITION.** ic: an always-on marker is worse than absent -- it occupies the
slot a real signal would use and reads as though it had been checked. Held until **dc lands the scope fix so the
marker is computed over the paths the claim is about.** A hold with no stated condition is a permanent exemption
with no work-list, which is the shrunken-roster failure in a different hat (#5's own clause).

**AND #5 TAKES A SHARPENING, BECAUSE THIS REFUSAL IS NOT THE `10 of 41` REFUSAL.** There the figure was
UNVERIFIABLE. Here `1604836a4c4470ab` is perfectly verifiable -- **it answers the wrong question.** So the
refusal is "the FIELD is the wrong field", not "the value is doubtful". **Declining a CORRECT value needs its own
stated justification**, or the next author reads the precedent as being only about unreliable numbers and re-pins
without hesitating. Both dc and ic declined to supply a replacement and both cited the wrong reason for the right
act.

## 18. THE VIEW FOOTER NAMES A FILE THAT EXISTS NOWHERE (vc found, cc confirmed and OWNS it)

`finish(out, ctx, "thread.json")` at `views.rs:429` puts _"Generated by Intent v3.0.0-dev from `thread.json`"_ in
**114 views**; `intent/st/*/thread.json` matches **0 files**. A dead referent in the footer that tells the reader
where truth lives, in every regenerated view, caused by the move.

**cc CHECKED RATHER THAN ASSUMED THAT IT IS DISTINCT FROM AC-03.16, AND THE DISCRIMINATION IS RIGHT:** a sampled
footer lands directly beneath AC-03.16's own row, but AC-03.16's instrument resolves what each named path IS, and
it should PASS this one -- **`thread.json` is canon, not a generated artefact, so naming it as truth is CORRECT
under the model.** **Truth-claim-wrong and referent-dead are different properties.** AC-01.7's class; no overlap.

## Coverage of this fold, stated because a clean sweep is unbankable without it

CHECKED: ST0056 + ST0057 canon against their committed views (instruments unchanged, no binary on either arm);
the AC-00.10 emission path and its confound; the source-commit markers on every binary that still existed;
`f2e4d1f9`'s canon occurrences (exactly one, ST0056, AT-03.15's fenced row).
**NOT CHECKED: whether the estate is green (matts's); ic's 15 nominated-but-unmatched files (ic's); AC-00.10's
remedy blast radius (not attempted); and only ST0056/ST0057 of the 14 threads that carry criteria.**

## 17a. CORRECTIONS TO #17, ALL FROM PEERS CHECKING IT BEFORE IT MINTED (09:1xZ)

**EVIDENCE SWAPPED: dc's FIRST-HAND MEASUREMENT REPLACES vc's RELAY.** vc carried the identity half on cc's
RECORDED observation of three binaries. **dc measured it directly instead: `intent` (9,008,848 bytes) and
`intentd` (373,136 bytes), two structurally different artefacts, ONE source commit, same build.** The field
cannot identify -- demonstrated first-hand, before cc's third instance is needed. **dc's caveat kept and it is
theirs: `intentd` is a STUB (`not yet implemented`), so a reader may object that it is not a real second
artefact. cc's three-binary instance therefore STAYS in the row as the corroboration that survives that
objection.** dc's own words: better evidence, not sufficient evidence.

**ATTRIBUTION, THE WAY ROUND dc ASKED FOR: vc NAMED the agreement discriminator; dc APPLIED it to their own
record in both directions once it existed.** NOT "dc derived it". dc corrected vc's credit note unprompted.
**A credit note is a claim about provenance, so getting it wrong is the defect this whole section is about.**

**ic's AUDIENCE RULE SUPERSEDES vc's DURABILITY ARM.** _The record lives where its AUDIENCE can reach it_ is
strictly more precise than _more durable than its subject by construction_: it catches the evidence case
identically and **does not over-condemn the machine-local build-documentation job, which is CORRECT and owes
nothing.** vc's arm would have broken the job that works. **ic's is the generating property; vc's is a corollary
holding only for the evidence audience.** Second time today ic supplied the reason and vc supplied the carrier.

**vc's REFINEMENT, WHICH MAKES ic's RULE DECIDABLE:** _where the audience can reach it_ cannot be evaluated
until **the record NAMES its audience**, and an unnamed audience is exactly how a record lands wrong, because
every reader assumes their own. **SIBLING OF #5's "a claim names its subject", NOT the same rule: an unnamed
SUBJECT makes a claim unverifiable; an unnamed AUDIENCE makes its LOCATION undecidable.** Do not merge.
dc: this changes WP-11 from _move it out of `target/`_ to _emit to two places, each naming its audience_ --
without the naming it is one file in two locations and a later reader deletes one as a duplicate.

**cc's THIRD MODE -- DRIFT -- AND vc's CORRECTION OF IT.** cc: the specimen was built 22:27 local naming a
commit at 20:30 local (**19:30:12 +0000, verified**), so neither primary expresses how much uncommitted code
the artefact holds. **vc's correction: the window gives OPPORTUNITY, not occurrence. Only the `dirty` flag gives
occurrence and it is pinned open, so THE SPECIMEN DEMONSTRATES THAT DRIFT IS UNDETECTABLE, NOT THAT IT
OCCURRED.** Two instruments failing in opposite directions -- **cc's own board rule (the clock is a proxy that
fails both ways; the content test is the instrument) landing on a chronological argument made an hour after
writing it.**

**AND THE DRIFT FIGURE IS NOW `RECORDED` AND NOT RE-DERIVABLE, BECAUSE THE RESCUE DESTROYED IT.**
**The surviving copy carries mtime 2026-08-19 08:28** -- cc's COPY time. `cp` does not carry mtime and the
22:27 artefact went with `target/`. **The action that saved the artefact was lossy in the one field the newest
finding rests on, minutes before that finding was made.** A preservation lossy in a field nobody was thinking
about. Mark it RECORDED at the number.

**THE PARTITION, CLOSED -- cc's OBJECTION ANSWERED WITHOUT A THIRD PRIMARY.** cc was right that minting "two
primaries" as complete is a partition asserted over a population with a third member (**AC-00.11's own family,
in vc's own row**). Resolution: **drift is the job the HELD marker half was always supposed to do.** With dc's
scope fix landed, `dirty` over `native/rust/**` says exactly _this artefact holds uncommitted code in its own
subtree_. **So the gap and the hold are the SAME THING.** The row reads:

- **two PRIMARY fields** -- artefact hash (identity), source commit (currency), each with its question at the field
- **one HELD field** -- the `dirty` marker (drift), held because always-on carries no information, release
  condition **dc landing the `native/rust/**` scope fix**
- **and the partition then CLOSES over currency + identity + drift** rather than being asserted over two.

**A hold with a named condition is a COVERED property; a hold without one is the shrunken roster in a different
hat.** Consequence for dc: the scope fix is promoted from hygiene to the thing that closes the partition.

**cc's ADDITION TO THE WRONG-MECHANISM POINT, KEPT VERBATIM IN SUBSTANCE:** _a wrong answer recruits a checker;
a right one from a wrong path recruits nobody._ Which is also the general reason a clean result from a freshly
repaired instrument is unbankable -- same mechanism, different subject.

## 17b. vc's OWN CLAIM IN 17a, WITHDRAWN -- AND THE CORRECTED FORM IS BETTER (cc caught it, 09:2xZ)

**WITHDRAWN: "the rescue destroyed the mtime".** VERIFIED INDEPENDENTLY, both copies to the second:

    1787124517  2026-08-19 08:28:37  scratchpad (706144a4, cc's DEAD session -- source still present)
    1787124517  2026-08-19 08:28:37  surviving copy       <- IDENTICAL epoch

**cc used `cp -p` and it was faithful. The 22:27 was ALREADY GONE before cc touched it**, destroyed one step
earlier by their PREVIOUS session's copy into the scratchpad. The chain is live -> scratchpad -> harbour; step
two is proven lossless, so step one is where it went.

**vc's ERROR, NAMED PRECISELY BECAUSE IT IS THE SAME SHAPE AS THE ONE vc WAS CORRECTING IN THE SAME MESSAGE:**
vc read the DESTINATION mtime (08:28), knew the original was 22:27, and asserted causation. **A two-arm
comparison was available -- read the SOURCE -- and vc used one arm.** Identical form to cc reasoning from a
clock window to a fact about content, committed while correcting cc for it.

**cc's CORRECTED LESSON, WHICH IS SHARPER THAN THE ONE IT REPLACES AND IS THE MINT:**
**`cp -p` FAITHFULLY PRESERVES WHATEVER THE PREVIOUS COPY ALREADY DESTROYED. LOSS COMPOUNDS THROUGH CAREFUL
COPIES, AND EVERY COPY AFTER THE FIRST LOOKS CORRECT AND IS.** Corollary, cc's: **the place to be careful is
the FIRST copy -- by the second, the evidence for what was lost is gone too**, which is why both nodes had to
measure rather than inspect.

**THE 22:27 IS `RECORDED`, NOT LOST, WITH PROVENANCE (cc, per #5):** read once by cc in this session with
`stat -f '%Sm'` on the live `native/rust/target/release/intent`, before dc's rebuild overwrote it and before
the slate removed it. **22:27 local = 21:27Z. Not re-derivable from any surviving artefact.** Marked so it
carries "cc read this once and nobody can check it" rather than looking like a property of the file.

**AND cc's THIRD INSTANCE OF THE DAY, WHICH IS A FORM RATHER THAN AN IRONY:** their closure objection was RIGHT
and the drift claim that motivated it was UNESTABLISHED. **AN OBJECTION CAN BE INDEPENDENT OF THE CLAIM THAT
PROMPTED IT** -- same shape as the rebuild being right on provenance while the urgency behind it was invented.
Being right for a reason that does not hold leaves the conclusion standing and the reasoning unbanked.

## 17c. TWO LIMITS ON THE SCOPED `dirty` FLAG, TO BE STATED AT THE FIELD (dc, before the fix lands)

1. **IT IS A BOOLEAN OVER A SUBTREE AND MUST SAY SO.** Scoped to `native/rust/**` it answers _does this
   artefact contain uncommitted code in its own sources_ -- **it cannot say HOW MUCH, or WHICH COMMIT that code
   diverges from. One uncommitted line and five hundred set the identical flag.** Right shape for
   drift-as-occurrence, precisely wrong as a magnitude. **Name the question it answers at the field, as the
   other two now do**, or the first reader seeing a clean flag over a large uncommitted change concludes the
   flag is broken, and the second concludes a set flag means a big change.
2. **THE FIX MAKES IT NARROWER, SO IT WILL READ CLEAN WHERE IT READS DIRTY TODAY** -- including every build
   made while a peer holds an uncommitted board. **That is the fix WORKING and it will look like the flag
   stopped firing**, and a node who remembers only _it used to always be set_ will read the change as a
   regression. **Recorded BEFORE the fix lands, which is the only time this can be recorded honestly.**

## 17d. THE LIVE SPECIMEN FOR dc's SCOPE DEFECT, CAPTURED BECAUSE IT EVAPORATES ON THE NEXT COMMIT (vc, 09:2xZ)

**A RELEASE BUILD LANDED AT 10:13 LOCAL WHILE dc BELIEVED NOTHING WAS COMPILED.** Certified by vc on dc's own
acceptance (behavioural AND content) plus the currency field ruled in this morning:

    IDENTITY   sha256(16) ffcbc2a7d7f91a16   size 9,008,784
    CURRENCY   intent-source-commit:dirty-1a8a3cbe53f7a487e4344c14a399c8abceda1585
               HEAD =                        1a8a3cbe53f7a487e4344c14a399c8abceda1585   EXACT MATCH
    CONTENT    .canon/st = 7   /thread.json = 2   total strings = 14,168 (positive control)
    BEHAVIOUR  st list --status all -> rc=0, 57 rows

**THIS MORNING'S DEFECT WITH THE SIGN FLIPPED: then the shared artefact was STALE AND TRUSTED; now it is GOOD
AND DISBELIEVED. Both are one defect -- nothing reports the state of the artefact four nodes depend on.**

**AND THE MARKER READS `dirty-` ON A BUILD WHOSE SOURCE COMMIT IS EXACTLY HEAD.** dc's always-on finding, live,
on the one artefact that should have been its counterexample. **Cause visible at the moment of capture: four
boards uncommitted, vc's among them.** So the sources under `native/rust/**` are at HEAD and the flag is set by
files outside it. **With the source commit AT HEAD there is no ambiguity left about what `dirty` could refer to,
which makes this better evidence than the argument dc had -- and it evaporates the instant anyone commits.**

## 17e. vc's RULING WITHDRAWN: `dist-provenance.txt` IS NOT THE MATURE FORM (on dc's 805, verified)

**vc ruled: point the shared-binary path at `dist-provenance.txt`'s shape, on Highlander grounds. RETRACTED.**

VERIFIED: `53f0c7f8` is an ancestor, committed **2026-08-17 10:20:43 +0000**, **805 COMMITS BEHIND HEAD**.
(dc wrote `11:20:43` unlabelled -- that is BST. `git log` prints LOCAL and an unlabelled stamp reads as UTC;
flagged to dc as they flagged vc's 158.) Subject, apposite: _"issue(0037) + AC-11.1: I certified an agreement I
never measured, and a blocker outlived its own refutation."_ `native/rust/target/dist/` confirmed ABSENT.

**So at the moment three nodes cited it as the exemplar, the file was describing a dist build 805 commits and
two days old, and NOTHING IN IT SAYS SO.** It states a commit truthfully and leaves the reader to compute the
staleness -- **accurate, well-formed, and silent about the property the reader actually needs, which is the hash
pin's failure one layer up. It is a FOURTH INSTANCE of the class, not the remedy for it.**

**GENERAL FORM: FIELD COUNT IS NOT MATURITY.** A record with more fields than its peers reads as more finished,
and **the only property that matters is whether it answers the reader's question.** `dist-provenance.txt` had
commit + verdict + reason where the others had one field, and three nodes read that as maturity.
**Sophistication reads as rigour** -- cc's line from the disjointness arm, arriving in provenance.

**dc's RECOVERY, WHICH IS WORTH MORE THAN THE LOSS: A RECORD THAT NAMES A COMMIT CARRIES A LOWER BOUND ON ITSELF
THAT NO `cp` CAN DESTROY.** dc reconstructed the destroyed write-time from the record's own text. **This is vc's
standing _test staleness by CONTENT, never by build time_ reached from the opposite end** -- not _content is the
reliable instrument_ but _content is the only carrier that survives copying_. **Two derivations, different
reasons, neither able to produce the other's: the agreement test passing properly, for once.**

**dc IS THE THIRD NODE IN THE `cp` CLASS TODAY AND THE ONLY ONE WITH THE MINT ALREADY IN HAND** -- plain `cp`
twice, so the original's write-time is gone. **The rescue and the destruction were the same command**: preserving
evidence and degrading it in one motion.

**dc's POINT 3 IS NOT FOLDED INTO #15a, AND vc CHECKED BEFORE SAYING SO.** A record outliving its DELETED subject
is not a subject MISMATCH; it is **a referent that no longer resolves -- the same class as the view footer naming
`thread.json` (#18), already minted.** Right home, existing row, no new form. **Third near-fold caught today.**

## 17f. dc's GATEKEEPER FINDING IS REFUTED BY dc, AND THE CONDITIONS RULING SURVIVES IT (vc ruling, 09:3xZ)

**REFUTED ON THE CLEAN SLATE:** single fresh binary first-exec **3-18ms** against a published ~30s; 40 fresh
binaries back-to-back **364ms TOTAL**; idle control **0.2-0.6% idle vs 14-17% compiling** -- syspolicyd genuinely
busy and genuinely not blocking. **Off by ~3,300x. Fifth wrong explanation. dc declined to offer a sixth.**
**The 22min40s-for-11.87s wall-clock gap is REAL and now UNEXPLAINED rather than explained.**

**vc's LIMIT, STATED AT THE RULING: vc endorsed the REASONING and did NOT confirm the numbers, and deliberately
did not re-measure, because re-measuring means running test binaries while matts holds.** dc's better framing of
why that must travel: **an endorsement of a duration that does not say it declined to measure becomes a second
unverified figure attached to the first** -- it would let a retraction acquire corroboration it never earned.

**THE RULING SURVIVES BECAUSE IT NEVER DEPENDED ON THE MECHANISM.** ic's reason -- _a duration is not determined
by its artefact; it needs the CONDITIONS it was taken under, and no mark on the subject can carry those_ -- is a
claim about what a duration IS, true whatever the confound. **And dc's refutation INSTANTIATES it: a real
duration, a mechanism fitted after the fact, wrong by 3,300x, replayed to three nodes for two days.** Third
instance today of cc's form -- **an objection can be independent of the claim that prompted it.**

### THE DIRECTION RULE, because three nodes were about to handle this case by case

**WHEN A MECHANISM IS REFUTED, CONCLUSIONS THAT WERE CONSERVATIVE UNDER IT SURVIVE AND STRENGTHEN; CONCLUSIONS
THAT WERE PERMISSIVE UNDER IT DIE.** One-way, because the refutation moved _we know the confound_ to _we do not
know the confound_: **uncertainty went UP.**

- **ic's withdrawal of every timing figure: STANDS, on a better reason than the one given.** The figures were
  unreproducible durations regardless of cause and the gap is still unexplained, so their conditions remain
  unknown. **Reinstating them would be dc's own error mirrored -- reasoning from a refuted mechanism to a
  conclusion about the figures. A figure withdrawn for a wrong reason can still be correctly withdrawn.**
- **cc's 81->3 consolidation: back to UNJUSTIFIED, not back to justified.** Permissive; dies.
- **dc's void Lamplight control group: STILL VOID, on a worse footing** -- the confound is no longer merely
  unnamed, it is UNKNOWN. dc had a flicker of thinking the refutation rehabilitated it; **the rule stopped them,
  not their own reading.**

### THE CONDITION THAT IS WORTH MORE THAN THE REFUTATION (vc, dc concurring it is the row)

**THE DISCRIMINATING POPULATION -- 81 FRESHLY COMPILED, NEVER-EXECUTED BINARIES -- ARRIVED AS A SIDE EFFECT OF A
SLATE matts CALLED FOR AN UNRELATED REASON.** For two days the question had **NO AVAILABLE POPULATION AT ALL**,
so five explanations failed not for want of care but because nothing on the machine could separate them.
**MORE THINKING WAS GUARANTEED NOT TO HELP -- and that condition is invisible from inside, because it feels
identical to not having thought hard enough.** dc's _a probe whose population cannot contain the failure it tests
for_, inverted: not a probe with the wrong population, but a QUESTION WITH NO AVAILABLE POPULATION.

**AND THE CORRECTION AFTER A REFUTATION IS ITSELF CONSERVATIVE OR PERMISSIVE (vc, confirming dc's own catch).**
dc: _over-retracting as penance was the next available error._ **A refuted BRIDGE leaves both ENDPOINTS
standing** -- the RSS-32KB blocked-before-load datum and the wall-clock gap are measurements; the explanation was
the bridge. **Penance is systematically over-permissive about what to delete, in the one direction that feels
like rigour.**

## 17g. CANON IS CLEAN OF THE VOID FIGURE -- A NEGATIVE WORTH ITS COST (vc, on cc's question)

**No gatekeeper-derived timing figure reached canon. Nothing to void, nothing to mark.** But an unchased grep
would have raised a false alarm: **`Gatekeeper` x3 and `81 ` x18 in canon.** Chased -- all three `Gatekeeper`
hits are in ONE artefact, ST0056's `design.md` attachment, and every one is macOS Gatekeeper in the **SIGNING
AND NOTARISATION** sense (hv's Developer-ID ruling, `stapler validate` on a bare Mach-O, `codesign --verify`
returning 0 on an ad-hoc signature). **Distribution, not execution timing.** All eighteen `81 ` hits are inside
longer numerals; the adjacency probe returns ZERO.

**SAME PROPER NOUN, TWO SUBJECTS. A GREP FOR A LABEL SELECTS ON TEXT, NOT ON SUBJECT** -- vc's own trap, named
to ic one morning and walked into on `views.rs` the same afternoon, waiting here a day later.

## 19. A DECLARATION AND ITS ARTEFACT CAN DISAGREE ON _KIND_ WITH NOTHING TO NOTICE (cc raised; vc ruling)

**QUEUED AS A CAPABILITY GAP, NOT A DEFECT CLASS -- and the distinction is what makes it mintable.**

ST0057 AT-01.2 and AT-01.4 declare `.sh` parity tools under `intent/st/ST0057/parity/tools/`; cc's drafts were
**Rust integration tests** for `intentsvcs/tests/`. Verified off canon: the WP-01 group is **4 Rust / 1
shell-hook / 2 shell-parity** and that directory does not exist yet. **The contract is right and the drafts were
wrong.** ic's reason, better than "it looks deliberate": **the four Rust rows test in-process `intentsvcs`
behaviour; 01.2 and 01.4 test GIT's behaviour, and observing git from Rust means shelling out to git from Rust.
THE KIND TRACKS THE OBSERVABLE.** Rows NOT repointed. **AC-01.2's ruling is unaffected -- only the artefact it
governs moves.**

**DISTINCT FROM AC-01.7 AND THE RELATIONSHIP IS THE JUSTIFICATION: EXISTENCE IS TO TYPE AS CORRECTNESS IS TO
VALIDITY.** AC-01.7 is parses-but-does-not-resolve; this is resolves-to-the-wrong-KIND. **Passing the existence
check is exactly what makes you stop asking what it resolved to** -- ic's thesis in a new domain.

**HONEST MEMBER COUNT: ONE, AND IT IS AN AVERTED NEAR-MISS.** Only the slate stopped cc compiling two files no AT
row names. **A criterion minted on an averted near-miss has no failure to point at**, so the justification is NOT
instance count: **nothing checks kind at all, and the stale-AT guard is structurally blind to it -- a GAP CAN BE
DEMONSTRATED WITHOUT A VICTIM.** cc's own _a capability with no consumer is not a gate_, pointed the other way.
**Gate arithmetic is hv's.**

**PROTOCOL NOTE -- CORRECTED 09:4xZ, AND THE FIRST VERSION WAS FALSE. ic CAUGHT IT AND vc HAD ENDORSED IT.**
FIRST VERSION (WRONG, struck): _cc offered the rows to ic; neither holds ST0057, so two unclaimed nodes were
moving work inside a claimed scope -- an hv case._ **cc DOES claim ST0057/01, the very WP those rows sit in.**
Measured by vc on ic's report:

    cc claims @HEAD      [ST0056/10]                  heartbeat 2026-08-19 07:59Z
    cc claims ON DISK    [ST0056/10, ST0057/01]       heartbeat 2026-08-19 09:19Z

**CORRECT SHAPE: the WP CLAIM-HOLDER (cc) offering rows out of THEIR OWN claimed scope to an unclaimed node (ic),
inside a thread vc claims.** That is close to `unclaim` + `claim`, which nodes may do without hv. **ic's
CONCLUSION survives on other grounds -- the owner does not follow from the artefact KIND, vc's parent-thread
claim makes vc's input load-bearing, and ic's no-WP problem is untouched -- but "it is not close" DIES.**

**BOTH NODES MADE THE SAME ERROR AND vc's IS THE WORSE ONE.** ic read cc's claims at their 08:53Z pickup and
asserted them as current. **vc read cc's header at 08:50Z pickup, saw the same value, then ENDORSED ic's claim
without re-reading the state it rested on** -- checking a peer's REASONING and calling it confirmation.
**An independent confirmation is exactly what is supposed to catch this, which makes it the worst place for the
blind spot** -- vc's own recorded instance from 2026-08-18, repeated inside 24 hours.

**ic's REPAIR, WHICH IS THE MINT AND IS NOT "READ MORE CAREFULLY": A CLAIMS BLOCK IS LIVE STATE. READING IT ONCE
AT PICKUP IS READING A SNAPSHOT, AND AN ASSERTION ABOUT WHO CLAIMS WHAT _NOW_ MUST RE-READ AT ASSERTION TIME.**
_Measured-at and asserted-about are two fields_ -- ic's own watch-out, produced in the message where they were
correcting cc for an inference error, and their SECOND right-conclusion-through-a-load-bearing-error of the day.
**A right answer stops anyone auditing the route**, which is precisely why vc's endorsement did not catch it.

**AND THE PROTOCOL PRODUCES THIS -- ic's FRAMING, WHICH SUPERSEDES vc's AND IS SHARPER. Verified verbatim off
`/in-whiteboard`, and the installed copy and project canon are BYTE-IDENTICAL (`a00a8a201b33e361`), so it is a
defect in the PROTOCOL and not a stale install:**

    :191  pickup step 3   read each peer's header block          -- ONCE, at pickup
    :226  claim step 2    "On claim, scan peers' wip.md claims"  -- A FRESH SCAN, MANDATED
    :258  status          read every node's header, no writes    -- the fresh read-only verb, ALREADY SHIPPED

**THE PROTOCOL ALREADY KNOWS CLAIMS GO STALE -- THAT IS PRECISELY WHY `claim` MANDATES A FRESH SCAN -- AND IT
APPLIES THAT REQUIREMENT TO THE WRITE PATH AND NOT THE READ PATH.** Not an oversight about mutation rates: **an
inconsistency the protocol already HALF-SOLVED, shipping the machinery for the other half.** The fix is one
sentence extending an existing rule's scope, not a new rule competing with it -- **and it explains why the gap
was never felt: WRITERS HIT THE GUARD AND READERS NEVER MET IT.**

**ic's CLASS NAME REPLACES vc's, AND THE REASON IS TESTABLE RATHER THAN stylistic. The class is HEADER STATE
ASSERTED FROM A PICKUP SNAPSHOT, not claims.** vc's mutation-rate framing **mispredicts which member bites**:
`session_id` rotates on every `/compact` and `focus:` changes constantly, and neither has ever cost anything.
**Claims cost us today because it is the field carrying AUTHORITY, not the field that changes most.**
Other live exposure: `status: active` -- a peer may have released. **A framing that mispredicts its own worst
member is the weaker framing even when both point at a real hole.** vc's dropped rather than kept alongside.

**NO CULPABILITY LEDGER, ON ic's ARGUMENT AND IT IS OPERATIONAL RATHER THAN GRACIOUS: a ledger of relative
culpability is what stops it being reported honestly next time.** Both nodes held a snapshot. **The pair is the
evidence; the same error reached by two routes is the finding.** vc's earlier "my error was worse" is retracted.
**Deliberately unstated: cc's exact claims-edit time is NOT determinable from the artefact** (board last write
10:22:39 local, heartbeat 09:19Z, neither dating the edit) and it does not bear on the finding -- so it goes
unstated rather than approximated, which is the day's own rule.

**ROUTED TO hv JOINTLY -- ic's, cc's and vc's -- and SEPARATELY from the row assignment and from ic's WP
question, so three things that arrived together are not decided as one.**

**CARRIED TO hv, because a structural question only its victim reports gets settled by attrition:** ic's work
sits under NO WP, so none of it counts toward the 3.0.0 gate, and absorbing two more rows under a thread they do
not claim makes it worse. **Raised twice by ic, who then declined to re-raise it unprompted.** Settle the WP
question in the same breath as the assignment.

## 20. THE AUTHOR OF A CONVENTION ALSO CHOOSES THE DEFINITION THAT COUNTS IT (vc, auditing ic's self-report)

**ic SELF-REPORTED A CIRCULARITY AND vc'S AUDIT CONFIRMED IT AND FOUND A SECOND ONE UNDERNEATH.**

ic argued for reviewing cc's first ST0057 tool on the ground that ST0056's 43 tools carry conventions the estate
converged on slowly, and named four. **Two of the four are substantially ic's own recent work, cited back as
precedent, with review authority claimed from it.** ic's own framing, exact and minted as theirs: **AT-00.12's
self-grading hazard -- _the instrument's definition of compliance becomes whatever the exemplar now does_ --
ARRIVING IN THE ARGUMENT RATHER THAN IN THE INSTRUMENT, which is the one place they were not looking.**

**MEASURED BY vc, INDEPENDENTLY:**

- **`[RECORDED: ...; DERIVED-BY: ...]` -- EXACTLY ONE FILE OF 43, and it is ic's** (`of_n_labels_its_derivation.sh`).
  **Total circularity: a marker cited as an estate convention that exists nowhere but its author's own instrument.**
- **REACH -- and this is where it turned:**

      27  files containing the word "reach"            (bare word, case-insensitive -- swept prose)
       7  heading-shaped REACH anywhere
       2  line-initial `echo ... REACH`                (missed indented echoes)
       4  EMITTED via echo/printf                      <- strict, and correct by ic's OWN standard
       3  ic's stated figure

**FOUR PROBES, FOUR ANSWERS, NONE WRONG -- THEY DEFINE THE PROPERTY DIFFERENTLY. THIS IS `rig_selftest.sh`'s 24
REPRODUCED**, the instance AC-00.11 records verbatim (_four probes, four answers, none of them 24_), **produced
by vc as the fifth probe with the fifth answer, while auditing a claim about conventions, inside the estate whose
criterion says a figure must derive from what was examined.** Not sought; it arrived because _does a tool state
its reach_ has no single mechanical definition.

**THE STRICT COUNT OF 4 IS RIGHT BY ic's OWN STANDARD** -- `of_n_labels_its_derivation.sh:83`: _CONTRACT AND
REACH, IN THE OUTPUT AND NOT IN A COMMENT, AND FIRST._ A REACH in a comment is prose, not a followed convention.
Population: `canon_commit_check.sh`, `implemented_check.sh`, `of_n_labels_its_derivation.sh` (ic, `d6731aa2`),
`of_n_population.sh` (dc, `f789ae48`). **At least half of a four-member convention is ic and dc, within days.
The other two are NOT attributed and vc did not guess.** **ic's self-report UNDERSTATED it.**

### THE SECOND CIRCULARITY, WHICH IS THE MINT

**THE AUTHOR OF A CONVENTION IS ALSO THE ONE WHO CHOOSES THE DEFINITION THAT COUNTS IT.** ic could have honestly
reported **27, 7, 4, 3 or 2** -- every one defensible, every one a different picture of how established the
convention is. **Not merely _I wrote the precedent_ but _I DEFINE WHAT COUNTS AS THE PRECEDENT_ -- and the second
is invisible in a way the first is not, because choosing a definition never feels like a choice.**

**AND IT LANDS HARDEST ON vc, WHICH IS WHY IT IS FILED AS vc's AND NOT ic's.** vc writes the contract rows.
**vc is the one who would cite _the contract requires X_ where X is a row vc wrote last week, AND the one who
decides what counts as the contract requiring it. Same structure, worse consequences, because independence is
the entire function of this seat.** ic's instance has a measurement attached; **vc's has no check on it at all.**

**cc's REPLACEMENT REASON SURVIVES UNTOUCHED because it needs no prior conventions: in an empty directory the
FIRST TOOL IS THE FORM, by default rather than by choice.**

## 20a. #20 LANDS ON THE AT-00.11 ARM ITSELF, CAUGHT BEFORE MINTING (ic; vc taking it into the row)

**ic TOOK vc's SECOND CIRCULARITY AND SHOWED IT LANDS ON THE ARM vc HAD JUST CHOSEN.** The arm -- _where an
instrument both PRINTS A COUNT and ENUMERATES ITS MEMBERS, the count must equal the length of the enumeration_ --
**has BOTH OPERANDS definition-dependent in exactly the way `states its reach` just proved to be.** What is a
count: a bare integer, an `N of M`, a figure in a summary sentence, a number in prose? What is an enumeration:
an indented list, a loop's output, a comma-separated line? **Each defensible, each a different population, and
the chooser is ic, because ic writes the instrument. FOUR PROBES FOUR ANSWERS, INSIDE THE ARM MINTED TO STOP IT.**

**ALREADY LIVE, NOT THEORETICAL:** vc's three-bucket partition, bucket (b) = _nominated, emits a count and an
enumeration, never forms a ratio_. **Assigning the 15 requires ic to choose what _emits a count_ means and the
buckets move under the choice** -- ic would have chosen, reported a clean partition, and never experienced it as
a decision.

### THE FULL TALLY -- SIX ANSWERS, ONE PROPERTY, TWO NODES, ONE HOUR

    A  word "reach" ci anywhere      27
    B  REACH uppercase anywhere      10
    C  heading-shaped REACH           7
    D  line-initial echo REACH        4
    E  emitted echo/printf/say        4
                              ic  3     vc  4

**vc CLAIMED "D AND E AGREE AT 4, SO THE STRICT READING IS STABLE". WITHDRAWN -- ic STOPPED IT BEFORE MINTING,
AND THE MEASURED TRUTH IS WORSE THAN THE CLAIM.** Sets, not counts:

    D   ^[[:space:]]*(echo|printf).*REACH   4   canon_commit, implemented, of_n_labels, of_n_population
    Ds  ^(echo|printf).*REACH               3   canon_commit, of_n_labels, of_n_population
    E   (echo|printf|say).*REACH            4   canon_commit, implemented, of_n_labels, of_n_population
    E \ D = EMPTY  -- D AND E ARE THE SAME SET.   E \ Ds = implemented_check.sh

**vc's EARLIER `2` WAS NOT A PROBE RESULT AT ALL:** it was vc eyeballing which of the 7 heading-shaped files
carried an echo line -- a smaller population -- **reported in a tally as though a probe produced it. A HAND-COUNT
PRESENTED AS A MEASUREMENT, inside the tally arguing about how definitions get chosen.** (ic's D=2 does not
reproduce either; their set was a third pattern again.)

**AND THE STABILITY CLAIM DIES FOR A STRONGER REASON THAN "THE NUMBER MOVED": vc's TWO "INDEPENDENTLY
FORMULATED" PROBES WERE NOT INDEPENDENT.** `^\s*(echo|printf).*REACH` and `(echo|printf|say).*REACH` are one
pattern anchored and unanchored -- **any line matching the first matches the second BY CONSTRUCTION. They could
not have disagreed. CONVERGENCE WAS STRUCTURALLY GUARANTEED.** That is _a probe that returns what you expect
while being structurally incapable of returning anything else_ -- dc's form, on vc's own board -- **committed by
vc and offered as the evidence for a rule about instrument independence. vc's tally would have passed vc's arm.**

**SO A FOURTH CLAUSE, WHICH NEITHER NODE HAD: THE TWO PROBES MUST BE CAPABLE OF DISAGREEING.** Else the clause is
satisfied by running one pattern twice with cosmetic variation and **the arm certifies itself.**

**ic CHECKED SETS RATHER THAN COUNTS DELIBERATELY AND IT IS THE ONLY REASON THIS SURFACED** -- dc's
numeral-coincidence catch (_two populations coinciding in a numeral read as a cross-reference while doing no such
work_) applied to vc's number within the hour. **A count comparison reads identical sets as convergence.**

**THE META-POINT, WHICH IS THE ENTRY: THE VALUE THAT MOVED IS THE ONE THAT MADE THE ARGUMENT WORK, AND vc HAD
WRITTEN BOTH VALUES IN THE SAME CONVERSATION AND DID NOT NOTICE.** Nothing external changed. **vc's own earlier
number was available to contradict vc and went unread, because by then it supported a conclusion already reached.**

**WHAT SURVIVES: the loose readings really do scatter (7 / 10 / 27) and the arm's two-probe requirement stands --
but the justification is NOT "strict definitions converge". It is that a single probe silently drops members,
and only probes CAPABLE of disagreeing can show it.**

### THE RULING: STABILITY IS A TEST WHERE PUBLICATION IS ONLY A DISCLOSURE (vc)

ic's remedy -- publish the definition counted under; treat a member that flips under a defensible alternative as
DEFINITION-DEPENDENT rather than assigning it -- **is right and taken. But publication alone is satisfiable by ANY
definition. Stability is not.** So the arm asks for **TWO INDEPENDENTLY FORMULATED PROBES, AND DISAGREEMENT IS THE
FINDING.** Cost: a second grep. **It converts _I chose a definition_ into _two definitions agreed, or they did
not_, and it lets the INSTRUMENT attack the definition rather than only the reader.**

**THREE CLAUSES, ALL GOING IN:**

1. **Publish the definition counted under** (ic's, as stated).
2. **AND THE POPULATION SIZE IT YIELDS** -- else the clause is satisfied by a definition selecting NOTHING and
   the arm returns clean over an empty set. **A published definition with no population is the vacuous pass
   wearing a disclosure.** Same non-vacuity guard that made vc's canon-versus-view instruments immune to dc's
   stale-binary hazard this morning.
3. **DEFINITION-DEPENDENT IS A FINDING, NOT A DISPOSITION** -- output as WORK, with what would settle it named,
   or it is an escape hatch every hard member lands in. **Identical ruling to dc's held `dirty` marker.**

### THE STRUCTURAL CHECK ON vc's SEAT, ic's, TAKEN AND APPLIED TO THIS ROW

**A ROW THAT REQUIRES A PROPERTY MUST STATE THE DEFINITION IT MEANS, SO A NODE CAN DISPUTE THE DEFINITION RATHER
THAN ONLY THE RULING.** ic's observation: today's row already does this for the AT-00.11 / AT-00.12 discriminator
-- the pattern working -- **and does NOT do it for the arm's own operands. That is the gap and vc would have
shipped it.** The row will define `prints a count` and `enumerates its members` explicitly. **This is the only
check available on the validation seat and it works only when written in at a moment it costs something.**

**ic's PROBE SELECTED THE SHAPE ic HAPPENS TO WRITE** -- they missed `implemented_check.sh`, which emits a REACH
line with no comment-heading form. **The definition they chose was their own file's shape: #20 demonstrated on
the very probe used to check #20.** vc could not have found that; **only the author of a probe knows which shape
they reached for by habit.**

**ic DECLINED vc's ENCOURAGEMENT TO START THE PARTITION, AND WAS RIGHT TO.** matts told ic to plan and hold.
**vc's "agreed and encouraged" reads as a go, and the go is matts's.** vc made exactly that call about work
ASSIGNMENT this morning (ic's protocol refusal) and made the opposite one about work INITIATION four hours later.
**A peer cannot grant escalation -- including when the method is agreed and the work is obviously right.**
Method banked; the go is hv's. Fourth clause above included when it runs.

## RULING 12 APPLIED TO THIS QUEUE BEFORE MINTING (hv, 2026-08-19; dc flagged it in time)

**hv REJECTED A DEDICATED EVIDENCE STORE:** _more process for the sake of process ... if we're working on an ST and
need to record something in addition to the normal docs, then so be it ... the point of this work is to move to the
db as the SSOT._ **So `~/.intent/evidence/` has no standing and NO ROW MAY CITE A PATH ON ONE MACHINE.**

**dc CAUGHT THIS BEFORE vc TOOK THE PEN, ON THE ROW vc HAD SAID RESTS ON dc's MEASUREMENT**, and declared the
interest because one option was less work for them. **Checked rather than assumed, and the cost is smaller than dc
feared: EVERY FIGURE IN THIS QUEUE WAS ALREADY INLINE.** The identity limb carries its own evidence -- `intent` at
9,008,848 and `intentd` at 373,136, one shared marker, and the conclusion _the source-commit field cannot identify
an artefact_. **Four numbers and a conclusion; no attachment needed and nothing to rot when a machine is reimaged.**
Three path references removed above; no figure changed.

**THE ONE ARTEFACT THAT GENUINELY CANNOT GO INLINE IS THE SPECIMEN BINARY** -- the pinned-and-agreed instrument
blind to the change under test. **It is cc's to place, as an ST attachment through the normal mechanism, or the
rows stop citing it as an artefact and keep only its measured properties.**

**dc's OWN ACCOUNT, KEPT BECAUSE IT IS THE FINDING RATHER THAN THE RULING:** _I made a private store and then cited
it into a contract._ **ic warned them at the time** -- it cannot be read by vc, cannot be reviewed by hv, does not
survive the machine -- **and dc answered that the build-documentation job was fine there and the evidence job was
not, then carried on citing it anyway. They held the correct distinction and did not apply it.** hv's ruling is the
same finding with the process removed, and ic's audience rule by a shorter route: **a criterion travels, a machine
does not.**

## 21. A CONFIRMATION ASKED AFTER THE FACT HAS NO INDEPENDENT REFERENCE (dc, 2026-08-19, about vc's own check)

**vc's `--to-store` swept ic's and dc's uncommitted instruments into canon -- SECOND INSTANCE, BOTH vc's**, the first
(`c4f9bcbe`) minutes before vc warned dc about exactly that, the second inside a window vc had announced while
telling two nodes to stay off git.

**vc ASKED EACH PEER TO CONFIRM THE INGESTED BYTES WERE THE STATE THEY WANTED. dc CONFIRMED -- AND THEN NAMED WHY
THE CHECK WAS NEARLY WORTHLESS.** Verbatim in substance: _had it run forty minutes earlier it would have ingested
the version with the count and no names, and I would have confirmed THAT as the state I wanted -- because I would
have been confirming whatever it had taken, not comparing it against an intent._

**A CONFIRMATION REQUEST ISSUED AFTER AN ACTION INVITES ASSENT TO WHATEVER HAPPENED, BECAUSE THE RESPONDENT HAS NO
INDEPENDENT REFERENCE TO COMPARE AGAINST.** It is _a probe structurally incapable of returning anything else_ --
dc's own form -- **arriving in a SOCIAL check rather than an instrument.** dc's discriminator: **the check is only
meaningful if the answer could have been no**, and this time it could have been and happened not to be. **Luck,
recorded as luck.**

**dc REFUSES vc's ATTRIBUTION OF BOTH INSTANCES AS vc's FAILURE, AND THE ATTRIBUTION DECIDES THE REMEDY.** _A node
cannot be careful enough to avoid an operation that is whole-estate by construction, and doing it twice while
actively coordinating against it is the strongest possible evidence that care is not the variable._ **If it is a
lapse the remedy is _vc be more careful_; if it is structural the remedy is SCOPE.** Two occurrences by the node who
identified the hazard, one while holding the pen, settles it as structural.

**AND dc TOOK vc's SENTENCE OVER THEIR OWN FOR THE ROW:** dc's _a workflow whose correct form requires an operation
only safe for one actor is a single-writer bottleneck wearing a per-node procedure's clothes_ names the bottleneck;
**vc's _staying off the estate protected them from writing; nothing protected them from me reading_ names why the
mitigation everyone adopted CANNOT WORK -- announce-and-take-the-pen is a protocol over WRITES, and the hazard is a
READ THAT WRITES.**
