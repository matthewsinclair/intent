# Output contracts -- an artefact must announce its own contract

Authored by dc, 2026-08-18, from a defect that cost four propagated wrong reads in one afternoon. It is a findings document, not contract canon: the criteria it produced are AC-10.11, ST0057 AC-03.5 and ST0057 AC-08.5, and those are the binding text.

## What this document concludes

**Thinking hard about a class does not protect you from it.**

It is stated first because it was earned last, and against its author's expectation. Everything below was found by someone working on this exact class, all day, deliberately -- and the class went on being committed throughout: three times inside the instrument built to catch it, and once in the very commit that added the rule forbidding it, by the person who had written the rule minutes earlier.

**The protection was the instrument, and only the instrument.** Every defect here was caught by a mechanical check, a value that contradicted a design, or the node next door. **None was caught by care, and care was not in short supply.**

## The one-sentence problem

An instrument was correct, the reader quoted it accurately, and the sentence still said the opposite of what it meant.

## The instance

`self_provenance_check.sh:280` emits, on a PASSING path:

```
self-provenance: <BIN> was built from an UNCOMMITTED tree (dirty-4ef953db) -- its bytes match no commit.
```

Capitalised `UNCOMMITTED`, a negative predicate, no hedge. Read cold it is a defect report; there is no other way to read it. The three lines saying it is not a finding are at `:179-181`, in a **comment**, a hundred lines up, and never appear in the output at all. vc read it as an outstanding item and carried it into a board focus line, two peer messages and a report to hv.

**It was not carelessness.** The reader never had access to the thing that would have corrected them.

## Why the obvious rule did not help

The prior rule was _a document written for cold pickup must carry its own staleness -- the caveat has to live in the same file as the figure._ Here it did, three lines from the branch, and it still did not reach anyone. The next clause is the one that was missing:

> **Co-location in the SOURCE does not co-locate in the EMISSION.** A caveat must travel with the OUTPUT, because the output is what gets quoted.

ic's general form covers every mechanism below and predicts ones nobody has hit yet:

> **THE READER'S VIEW IS NOT THE AUTHOR'S STRING.**

## Three mechanisms and one control

| #   | mechanism                                | the correcting text is                                   | found by | example                                      |
| --- | ---------------------------------------- | -------------------------------------------------------- | -------- | -------------------------------------------- |
| 1   | **distance**                             | present, in a comment that never reaches output          | dc       | `self_provenance_check.sh:280`               |
| 2   | **truncation**                           | present IN the emitted string, and cut before the reader | ic       | `estate_corpus.sh list`, criterion field     |
| 3   | **evidence-shaped token**                | absent, and the token reads as proof                     | ic       | `dirty-<sha>` quoted as a binary identity    |
| 4   | **suppression**                          | emitted and correct; the INVOCATION discarded it         | vc       | `sync --to-store` piped to `/dev/null`       |
| C   | **reader error** (CONTROL, not a defect) | present and adequate; the reader was wrong               | vc       | `sync --to-store` warning; `at lint`'s count |

**Mechanism 4 was the one ic predicted** when they said the general form implies a fourth nobody had hit -- wrapping, a `head -N`, a summariser. It arrived as `>/dev/null`. It is the reader-side twin of mechanism 1: in 1 the author put the caveat out of reach, in 4 the consumer deleted the channel. **An instrument cannot defend against it and should not try**; it belongs in the taxonomy because it explains misreads that look like instrument defects and are not.

**There are two control cases, not one.** vc read past an accurate `sync --to-store` overwrite warning; and `at lint` reported _"115 AT row(s) conform"_ when the truth was 116, and the word `conform` was read while the number was not. **The count WAS the instrument, it WAS correct, and it was looked past.** Both are reader error. Neither is a defect to fix in the tool, and scoring them as defects would inflate the class and move blame off readers who should carry it.

**The control is what makes the class measurable.** Without it every misread scores as an instrument defect, the finding inflates, and blame moves off readers who should carry it. vc supplied the case that exonerates an instrument in one instance while insisting the blame for another was theirs.

**Mechanism 2 is invisible to a filter over emitted strings**, because the string contains the correction. **Mechanism 3 is invisible to any text filter at all** -- there is no vocabulary in a hex string -- and it is the most dangerous, because the first two make a reader pause at a sentence and this one makes them stop looking.

## The same defect at three scales, one afternoon

| scale      | artefact                              | what it failed to announce                                               | closed by       |
| ---------- | ------------------------------------- | ------------------------------------------------------------------------ | --------------- |
| the ARM    | `self_provenance_check.sh` arm 2      | five outcome branches, four worded as findings, `rc` set in none of them | `addd4581` (dc) |
| the GUARD  | `G_SELFPROV` in `cmd/precommit`       | declared arm 1's concern while running arm 2 undeclared                  | `af7f86d7` (vc) |
| the MARKER | `intent-source-commit:dirty-<sha>`    | it names a COMMIT and is not an IDENTITY                                 | AC-10.11 (vc)   |
| the STORE  | the sqlite store behind `intent sync` | that it had REFUSED the last write and was therefore stale               | open            |

In the first three the artefact was correct, the reader was careful, and the contract lived somewhere the reader could not reach: a comment, a roster line, nowhere at all.

**The fourth is the same shape at the level of a datastore rather than a string, and it is the only one that destroyed data.** `sync --to-store` failed on a UNIQUE constraint; every later `sync --to-disk` then wrote that rejected, stale store back over canon, twice, with nothing saying the store was stale. The general form is worth stating on its own:

> **A write path whose input was REFUSED must not then be readable as a source of truth without saying so.**

**The marker is the hard case: it has nowhere to print a contract.** So the burden sits on the consumer, and ic's form is the operational test because it is checkable by reading one function:

> **THE INSTRUMENT THAT PAIRS TWO READINGS MUST ESTABLISH IDENTITY ITSELF RATHER THAN INHERIT IT.**

With the corollary that decides WHICH identity, and getting this wrong manufactures a false remedy:

> **The identity that travels beside a figure is the one the figure is a CLAIM ABOUT.** A paired binary reading -> a content hash. `STRANDED` -> the corpus revision, because the number is a claim about a pinned tree.

## The sweep, and why its number is not a population

Two nomination axes over the 40 tools in `parity/tools/`, adjudicating only the three this WP owns and nominating the rest for their owners.

- **Axis 1 (dc, vocabulary).** Stdout emissions carrying failure vocabulary, keyed on the estate's own convention: findings go to stderr and set `rc`; reports go to stdout. **77 lines nominated -- not audited -- across 16 of the 40 tools.**
- **Axis 2 (ic, truncation).** Stored strings rendered through a width limit. **9 of 40**, of which **3 hit only this axis and are invisible to axis 1.**
- **Adjudicated (3 of 3 owned).** `provenance_check.sh` CLEAN -- every emission is stderr AND sets `rc=1`. `no_intent_home.rs` CLEAN -- assertions only, so a Rust test has no report-only path and is immune by construction. `self_provenance_check.sh` DEFECTIVE.

**The filter discriminates, proven two-sided on real unplanted cases**: the tool adjudicated clean scores 0, the tool adjudicated defective scores 6.

**And it has a known false negative sitting inside those 6.** `:284` -- _"the binary is from an earlier tree"_, which AC-11.5 itself calls the case `stage` must fail closed on -- scores zero. No negation, no capitalised token: **the tell is a comparative, not a negative.** Two of the 6 are false positives (a self-hedging line caught on the bare token `no `, and an internal function return value that is not output at all).

**So the 77 over-counts and under-counts at once, and the two errors are different lines, so they do not cancel.** It is a work list, never a measurement. Quote it with its denominator and the word "nominated" in the same sentence, or not at all.

**The filter was deliberately NOT patched to catch `:284`.** Adding `earlier|predate|behind|older` catches the one case already found and teaches nothing about the ones not yet seen -- a detector fitted to its own validation sample measures the sample. Reporting the false negative is the more useful artefact than removing it.

## The axis neither filter has

**WHO QUOTES IT AND INTO WHAT.** ic supplied it by hand for four tools; there is no grep for it. A finding-shaped line nobody quotes is a wart; one that a peer reads into a status is what happened here. The harm concentrates where an instrument's output is quoted into a decision rather than a report -- and the two most dangerous shapes, a bare quantity and a bare identity, are exactly the ones with no vocabulary for a text filter to find.

## The instrument for this class committed this class

`canon_commit_check.sh` was built to catch canon asserting what it had not measured. **Its first draft skipped a thread recording zero attachments and printed _"every attachment matches"_ at exit 0, having compared nothing** -- reproduced at `6ab155ef` before the rewrite, which is what makes this evidence rather than an anecdote. 86 of 132 commits in this history record no attachments, so the vacuous case was **the majority of its input**, not a corner.

It now exits 2 saying `This is NOT a pass`, closes its count in one line (measured + unmeasurable = population), and states its contract and its reach in the OUTPUT rather than in a comment -- the defect this whole document is about, which it had faithfully reproduced.

**Building the instrument for a class while committing the class is not an aberration; it appears to be how the class is found at all.** ic wrote a `grep -c || echo 0` trap into their own harness after reading their own prose warning about it, in the file this tool later found stale.

### And it did it twice more

The same tool then shipped a hardcoded `86 of 132 commits in this history` -- **a figure naming neither its subject nor its revision, inside the instrument built to find records that disagree with reality.** `git rev-list --count HEAD` is 2184, so the string read as all of them and never meant that.

Then an optimisation narrowed what it examined from 278 attachments to 0, **and the population line went on reporting `278 of 278 examined`.** The closing count was arithmetically correct and substantively false. Which sharpens the condition above:

> **A closing count must close over what was EXAMINED, not over what EXISTS.** The two are the same number only while an instrument examines everything, and any optimisation at all separates them -- silently, because the arithmetic still closes.

## Enumerate the exceptions, not the successes

ic's admission bar required naming every subject AND stating the instrument's reach. Applied literally, the checker printed 57 per-thread counts before its verdict -- burying the load-bearing line and failing ic's own truncation rule in the same breath. **Two requirements collided, and the resolution collapses them into one:**

> **AN INSTRUMENT SHOULD ENUMERATE ITS EXCEPTIONS, NOT ITS SUCCESSES.** A list of everything that worked is noise that pushes the finding off screen; a list of what it could not reach IS the reach statement. They are the same list.

**With one condition that is what stops it being a duck: the count must close.** Measured plus unmeasurable equals the population, stated in the same line as the population. Without that, "enumerate only the gap" becomes a way to hide a third category -- a subject neither measured nor declared unmeasurable.

### The remedy became the defect

The fix this whole document argues for is **state the contract in the output**. `canon_commit_check.sh` was built with that line from birth:

```
canon-commit: GATES on what this commit ADDS; inherited divergences are reported, never failed on.
```

Then an optimisation narrowed what the tool examined -- and **the narrowed mode went on printing that line while reporting no inherited divergences at all, with two present.** A guard declaring an arm it does not run: the original defect, reappearing inside the sentence added to prevent it.

> **A contract line is not self-verifying. It is another claim, and it acquires the same failure mode as the thing it describes the moment a mode changes underneath it.**

**Which makes the remedy not free, and saying so is part of adopting it (vc).** Every contract-in-output line this estate adds is a claim that needs its own re-verification whenever the code beneath it changes. ic's amendment carries the standing half: **a new mode must be checked against every contract line it inherits.** A mode was added here and none of the three inherited lines was re-read; nothing in the tool required it.

**And the property being asserted is a property of HISTORY, not of a revision.** Commit the file and sync canon after, and _that_ commit stays divergent permanently -- a later sync repairs the next commit and can never reach the one already made. So a run against one revision can establish that the revision ADDS nothing, and an exhaustive run can establish the property holds _at that revision_; **neither establishes that the property holds, which is a claim about every commit.** `--history` is the only mode that speaks to it, and it speaks only about the range it was given.

The reason it was invisible is worth separating from the fix. The narrowing rests on a proof -- **only a path this commit touched can carry a NEW divergence** -- and that proof is about ADDS. **An inherited divergence is by definition in a path the commit did NOT touch, which is exactly the population the narrowing excludes.** So the mode was correct about one arm and structurally blind about the other, and a single contract line covered both. _Correct about ADDS_ and _complete about INHERITED_ are different claims; they were written as one sentence.

One of the divergences the narrowed mode could not see was **the tool's own canon record** -- an instrument whose negative result was a fact about the instrument.

## One statement, two directions

The criterion was reasoned entirely from `c4f9bcbe`, where **canon ran AHEAD**: a sync ingested worktree bytes nobody had committed. The checker's first whole-tree run found the mirror at `3f10b1ee` -- **the file ran ahead and canon stayed BEHIND**, with a clean worktree, so canon named bytes present neither in the commit nor on disk.

Both are one statement; a prohibition worded against one operation catches one of them. The invariant that covers both, and does not privilege whichever operation was in view when it was written:

> **At every commit, every attachment's recorded bytes are obtainable from that commit.**

## The adjacent-proposition error

Three instances in one afternoon, by three different nodes, **every one caught by the person next door rather than by any check**:

Five instances in one afternoon, across three nodes:

- a control calibrated on `lib/cmd/*` offered as evidence about `cmd/*` (dc, caught by vc)
- a superlative placed on a peer's unverified mechanism (dc, caught by ic)
- a measurement of one binary pair applied to an adjacent pair without re-measuring (dc, caught by cc)
- a `find` scoped to `native/rust` reported as a fact about the tree, when the files were at the project root (cc, caught by vc)
- a report scoped to one inbox, stated in the grammar of a claim about the estate (vc, caught by hv)

The shape is one measurement, one proposition next door, and no step between them. **A false remedy is worse than none, because it terminates the search**: acting on the wrong mechanism produces a figure that looks validated and is not.

**The asymmetry is the operational part.** Every instance caught by its own author was caught by adding a control against their own result. **Every instance not caught that way was caught by the node next door, and none by any check.** There is no grep for the scope of a probe, so the only mechanisms that work are a deliberate control against your own answer, and a second reader.

## The commit that added this section violated the rule it states

`8f652d1b` -- the commit that added the two consequences above, including the ordering rule -- **committed this document without canon, which is exactly the order the tool's own remedy text forbids.** The checker then printed that remedy back, naming this file.

**The author of the criterion, the instrument and the remedy text did it while typing the sentence that forbids it.** Not through ignorance of the rule: through having just written it down.

**And could not have complied.** The compliant order needs a canon sync first, `intent sync` has no operation smaller than all 57 threads, and it is not this node's to run. So the commit is permanently divergent -- a later sync repairs the next commit and can never reach this one.

> **A rule its own author cannot obey while writing it down is not a discipline problem. It is a missing operation, and no amount of care closes it.**

That is the argument for the narrow verb at ST0057 WP-08, and it is why the checker's row stays `manual`: gated, this would have blocked the commit of the document explaining why it should be gated.

## What is closed and what is not

- **CLOSED.** The arm announces its contract (`addd4581`). The guard names both arms and says which one gates (`af7f86d7`). Paired identity requires a content hash (AC-10.11). Canon ingesting uncommitted bytes is reported (ST0057 AC-03.5). A verb may not silently clear a field it was not asked to change (ST0057 AC-08.5).
  **The measured cost of the missing narrow verb (vc):** four whole-estate syncs in ninety minutes to keep one thread's attachment records current -- zero stale after each, stale again within the hour. `intent sync` is `--to-disk` or `--to-store`, both across all 57 threads, so there is no operation smaller than the estate. That figure is the argument for the narrow verb at ST0057 WP-08, and it is why the checker's roster row stays `manual`: under a gate the only compliant order needs an operation two of four nodes may not perform.

- **OPEN.** A store that refused a write must not then be read as truth silently (vc's, above -- the only member of this class that destroyed data). The 77 nominations over the 37 instruments this WP does not own, held by their owners. Whether a commit-time control should refuse canon that names bytes not in that same commit -- argued from AC-11.1 and the clock guard's block-only-on-what-the-commit-adds, and the false-positive rate is now MEASURED rather than unknown: over all 46 commits carrying canon with attachments, 23 disagree (50%) but only **5 introduce a new divergence**. A naive form blocks half of all commits and is a guard nobody keeps; the inherited-breakage clause is the whole design.
