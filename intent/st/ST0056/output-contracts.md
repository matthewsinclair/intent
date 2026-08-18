# Output contracts -- an artefact must announce its own contract

Authored by dc, 2026-08-18, from a defect that cost four propagated wrong reads in one afternoon. It is a findings document, not contract canon: the criteria it produced are AC-10.11, ST0057 AC-03.5 and ST0057 AC-08.5, and those are the binding text.

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

## The adjacent-proposition error

Three instances in one afternoon, by three different nodes, **every one caught by the person next door rather than by any check**:

- a control calibrated on `lib/cmd/*` offered as evidence about `cmd/*` (dc, caught by vc)
- a superlative placed on a peer's unverified mechanism (dc, caught by ic)
- a measurement of one binary pair applied to an adjacent pair without re-measuring (dc, caught by cc)

The shape is one measurement, one proposition next door, and no step between them. **A false remedy is worse than none, because it terminates the search**: acting on the wrong mechanism produces a figure that looks validated and is not.

## What is closed and what is not

- **CLOSED.** The arm announces its contract (`addd4581`). The guard names both arms and says which one gates (`af7f86d7`). Paired identity requires a content hash (AC-10.11). Canon ingesting uncommitted bytes is reported (ST0057 AC-03.5). A verb may not silently clear a field it was not asked to change (ST0057 AC-08.5).
- **OPEN.** A store that refused a write must not then be read as truth silently (vc's, above -- the only member of this class that destroyed data). The 77 nominations over the 37 instruments this WP does not own, held by their owners. Whether a commit-time control should refuse canon that names bytes not in that same commit -- argued from AC-11.1 and the clock guard's block-only-on-what-the-commit-adds, and the false-positive rate is now MEASURED rather than unknown: over all 46 commits carrying canon with attachments, 23 disagree (50%) but only **5 introduce a new divergence**. A naive form blocks half of all commits and is a guard nobody keeps; the inherited-breakage clause is the whole design.
