---
id: "0017"
title: The AT reference field has no grammar: at_pathname takes the first backtick span, so a row can cite a selector, a bare filename or nothing at all, and no diagnostic fires
date: 2026-08-11
reporter: matts
status: CLOSED
severity: high
---

# 0017: The AT reference field has no grammar: at_pathname takes the first backtick span, so a row can cite a selector, a bare filename or nothing at all, and no diagnostic fires

## Tags

acceptance, at-coverage, parsing, grammar, silent-failure, root-cause

> **This is the root cause of 0014 and 0015, filed as a parent rather than a duplicate.** 0015 (a green AT whose cited file does not exist) and 0014 (coverage ids dropped by a non-comma separator) are two symptoms of one absence: **the AT row has no specified grammar**, so every field is recovered by a best-effort regex over free prose and each recovery fails differently. Fixing either symptom in isolation leaves the drift that produced it.

## Summary

An AT row is meant to assert: _this named test, in this named file, proves these named criteria, and here is its state._ Three of those four are recovered from free-form markdown by single-shot regexes, and the reference is the weakest:

```bash
at_pathname()   { extract_field "$1" '^[^`]*`([^`]*)`.*'; }   # bin/intent_acceptance:65
```

**The "path" is defined as _whatever sits inside the first pair of backticks on the line_.** It is not required to be a path, not required to contain a directory, not required to exist, and not required to be present at all. A row with no backticks yields the empty string, silently.

Because nothing constrains the field, authors have filled it four incompatible ways, all of which look correct when read by a human:

| Form observed                       | Example                                                       | `at_pathname` yields   |
| ----------------------------------- | ------------------------------------------------------------- | ---------------------- |
| backticked repo-relative path       | `` `native/cli/tests/discovery.rs` ``                         | the path -- **works**  |
| unbackticked path + `::"test name"` | `apps/control/.../narrator_test.exs::"murder corpus"`         | **empty**              |
| unbackticked path + `("test name")` | `apps/llclient/.../acton_foundation_test.exs ("theme sets")`  | **empty**              |
| bare filename, no directory         | `publisher_test.exs`                                          | unresolvable by design |
| backticked NON-path                 | `` `focus_mode` ``, `` `data-banner-row` ``, `` `<header>` `` | a CSS/DOM token        |

The last row is the sharpest: the field happily returns a DOM attribute name as a test path, and every downstream reader treats it as one.

## Reproduction

Measured on Lamplight, 2026-08-11, across all 16 active steel threads -- 314 AT rows, 216 of them `green`.

Per-thread, comparing the record against `intent at list`:

| Thread | AT rows | rows backticked | rows where the tool shows a reference |
| ------ | ------- | --------------- | ------------------------------------- |
| ST0290 | 23      | 23              | 23                                    |
| ST0201 | 22      | 2               | 8                                     |
| ST0264 | 32      | 0               | 6                                     |
| ST0324 | 21      | 0               | 3                                     |
| ST0345 | 20      | 0               | 1                                     |

**Backticks are sufficient and nothing else is reliable.** ST0290 is 23/23 because that thread happens to backtick every row. ST0345 shows 1 of 20 -- and that one is a backticked DOM token, not a test.

A single row, end to end:

```
record:  - AT-02.1 apps/control/test/control/wright/compiler/narrator_test.exs::"murder corpus round-trip" -- covers AC-02.1, AC-02.4 -- status: green
tool:    at: AT-02.1    green
```

The file exists, the test exists, the row is correct, and the tool reports no reference at all.

### A live instance, twenty minutes after this issue was filed

Not constructed, and worth more than the table above because it shows the format still actively drifting. A work package merged into Lamplight's ST0347 introduced a **fifth** reference form -- square brackets, bare filename, quoted name:

```
- AT-02.1 [play_live_test.exs, "ST0347 AT-02.1"] the place button opens the sheet (bottom-sheet classes at base, `md:absolute`/`md:bottom-full` popover ...) -- covers AC-02.2, AC-02.5 -- status: green
```

`at_pathname` takes the first backtick span, which here is a Tailwind utility class buried in the prose:

```
$ intent at list ST0347
at: AT-02.1  md:absolute  green
at: AT-02.2  min-h-11     green
```

**Two green ATs on a live thread now report CSS class names as their test file paths**, and the gate counts both as coverage. Nothing warned. The author did nothing unreasonable -- they wrote a readable row, and the format has no rule to have broken.

### The same instance argues for linking by id rather than by name

The row above cites its test as `"ST0347 AT-02.1"`. **No test in that file is named that.** The deck actually carries:

```elixir
describe "the place sheet opens and never clips a card (ST0347 AT-02.1)" do
  test "AT-02.1: the place button opens the sheet -- bottom sheet at base, popover above the button at md+ -- and a second press closes it", ...
```

So the cited **name** is a paraphrase and a name-match fails -- exactly the failure that defeated every string-matching instrument in the Impact table. But the **id** is present, twice. **The proposed L3 check passes on this row; a name check fails on it.** That asymmetry is the whole argument for the change below, and it turned up on its own within the hour.

**The diagnostic that should catch this does not cover it.** `warn_bad_fields` (`:135`) checks exactly two classes -- `bad_status_lines` and `bad_marker_lines`. There is no `bad_pathname_lines`. The comment at `:55` states that a parse failure is "harmless and audible instead of silent and wrong"; that is true of status and of the non-test marker, and **not true of the reference**, which is the field with no diagnostic at all.

## Root Cause

The AT row was specified as a **human-readable convention** and implemented as **a set of independent recovery regexes**. There is no single anchored pattern that a row must match, so:

- no row can ever be _malformed_ -- only partially recovered;
- each field degrades independently, so a row can lose its reference and keep its status;
- the failure mode of every field is **silent**, because a regex that does not match returns empty rather than raising;
- and the format is therefore free to drift, because nothing has ever rejected a variant.

0014 is this shape in the `covers` field. 0015 is this shape in the reference field, observed at its most damaging (a green AT counted as coverage while its file does not exist). This issue is the shape itself.

## Impact

**An entire class of audit is impossible, and it is the class that verifies the acceptance contract against the tree.**

The concrete case that prompted this: a node found an AT row citing a test deck that could never have carried its claim (async, hand-built maps, no storage setup, while the criterion needed storage). The obvious follow-up -- _do other AT rows cite files incapable of their claim?_ -- was attempted estate-wide and **could not be answered**. Four mechanical instruments were built and every one returned a large number dominated by format variance rather than defects:

| Instrument                                | Raw hits | What they actually were                                                  |
| ----------------------------------------- | -------- | ------------------------------------------------------------------------ |
| match the cited test name inside the file | 15       | paraphrase (`each` vs `every`, `or` vs `+`)                              |
| loosen to a four-word fragment            | 13       | same cause                                                               |
| does the cited path exist                 | 59       | bare filenames -- each resolves to exactly one real file                 |
| does the file carry the AT/AC id          | 40       | decks that simply do not label; the convention is real but not universal |

Every row inspected by hand was correct. **The tooling cannot distinguish a correct row from a phantom one, so a phantom is found only by a human reading it** -- which is how the original was found, and how the next one will be.

Compounding, per 0015: the gate gets _more_ permissive as citations rot, so nothing ever draws attention to it.

## Proposed Fix

Regularise the row into one anchored, closed-vocabulary grammar, and enforce it. **The design goal is that a single regex parses every conforming row and rejects every non-conforming one** -- no best-effort recovery anywhere.

### The grammar

```
- AT-<N.N> `<repo-relative-path>` -- covers <AC-N.N>[, <AC-N.N>...] -- status: <to-write|red|green|n/a>[ -- <free note>]
```

```
^- (AT-[0-9]+\.[0-9]+) `([^`]*/[^`]+)` -- covers (AC-[0-9]+\.[0-9]+(?:, AC-[0-9]+\.[0-9]+)*) -- status: (to-write|red|green|n/a)( -- .*)?$
```

Fully anchored; every field delimited by something that cannot occur inside it; free prose confined to a trailing note that is explicitly never parsed. This subsumes 0014 -- the comma separator becomes part of the grammar rather than an undocumented expectation, so `covers AC-09.2 and AC-04.3` is a hard parse failure instead of a silent single link.

**Note the `[^`]\*/[^`]+` in the path group: it requires at least one directory separator, so a bare filename is a GRAMMAR error rather than something a later existence check has to catch.** The first draft of this proposal used `[^`]+` and accepted `` `publisher_test.exs` `` -- one of the four forms it exists to kill. Caught by running the arms below against the proposal itself.

Verified against every form observed in the wild, both directions:

| Arm                               | Expected | Result |
| --------------------------------- | -------- | ------ |
| canonical, single AC              | match    | match  |
| canonical, multi AC               | match    | match  |
| canonical + trailing note         | match    | match  |
| `to-write`                        | match    | match  |
| Rust `src/` path (not a test dir) | match    | match  |
| unbackticked + `::"name"`         | reject   | reject |
| unbackticked + `("name")`         | reject   | reject |
| bare filename                     | reject   | reject |
| backticked DOM token              | reject   | reject |
| `and` separator (0014)            | reject   | reject |
| `**green**` (markdown emphasis)   | reject   | reject |
| possessive after id (0014)        | reject   | reject |

12/12. The two 0014 cases reject on the grammar alone, with no extra check.

### The one substantive change: drop the free-text test name, link by id

The current row tries to name the test in prose. That is unverifiable -- the measurements above show paraphrase defeating every string match -- and it is the reason the reference field grew three competing shapes (`::"name"`, `("name")`, `::bare_name`).

**Replace it with a bidirectional link: the cited FILE must contain the AT's own id as a literal string.**

```
- AT-03.2 `apps/control/test/control/run/npc/decide_action_test.exs` -- covers AC-03.2 -- status: green
```

```elixir
describe "AT-03.2 / AC-03.2: off-fidelity decision schedules a bounded CharacterActed" do
```

This is already the best convention in practice -- it was observed in use and is what made those decks verifiable when nothing else was. It is strictly better than a prose name because it is **checkable in both directions**: the row names the file, the file names the row, and `rg AT-03.2` finds both ends. It is also what makes the original finding catchable: a deck that cannot carry a claim does not carry its id either.

### The enforcement: `intent at lint <stid>`, folded into `ac gate`

| Check  | Rule                                                                   | Applies to      |
| ------ | ---------------------------------------------------------------------- | --------------- |
| **L1** | the line matches the grammar exactly; a non-match is RED, never a skip | every AT row    |
| **L2** | the cited path resolves from the repo root                             | `green` / `red` |
| **L3** | the cited file contains the literal AT id                              | `green` / `red` |
| **L4** | every id in `covers` exists as an AC row in the same file              | every AT row    |

L2 and L3 exempt `to-write` deliberately -- a missing file is the **correct** state for a test not yet written, which is the nuance 0015 already identified (a naive existence check reds five correct rows).

L2 closes 0015. L1 closes 0014. L3 is new, and it is the one that makes the contract auditable rather than merely parseable.

### Migration

Non-trivial and should be tooled, not hand-run: Lamplight alone carries 314 AT rows across 16 active threads, plus ~96 completed threads. Suggest `intent at lint --fix` for the mechanical part (wrap the path in backticks; expand a bare filename where exactly one file on disk matches -- which was true for every bare filename measured) and a report for the rest, since adding the id back-reference requires touching the test file and is a judgement call per row.

Completed threads should be migrated for grammar but **not** be gated on L3: retrofitting id labels into closed threads' tests is archaeology, and the existing ruling is that closed contracts are exempt rather than backfilled.

## Related

- 0015 -- `ac gate` counts a green AT whose cited test file does not exist (**symptom**: the reference field, at its most damaging)
- 0014 -- AT coverage is comma-separated only (**symptom**: the same absence in the `covers` field)
- Found while sweeping a Lamplight finding one level up: an AT row citing a deck structurally incapable of its claim.

## Resolutions

FIXED + CLOSED (2026-08-14), shipped in v2.19.0, as the centrepiece of a nine-issue release. It also closes 0014 and 0015, which are symptoms of the same missing grammar. Implemented as proposed, with **three holes in the proposed grammar found by running the arms against real rows rather than reading them**.

**Hole 1 -- `path::name` (found by vc's review).** The tool's own doc comment and the shipped `acceptance.md` template both taught `[test path::name]`. The proposed path group ``([^`]*/[^`]+)`` ACCEPTS that form at L1 (it contains a slash) and then L2 fails it as a dangling file, so the commonest legacy shape would have been diagnosed as the wrong defect. `:` is excluded from the path group, `::` forms are a clean L1 reject with a targeted message, and `--fix` strips the suffix.

**Hole 2 -- the parenthetical status note.** The shipped template itself taught `status: to-write (red-first)`, which the proposed one-armed grammar rejects. The form entered the estate because Intent put it there, so the template was corrected in the same commit and a trailing note now requires the spaced `--` delimiter.

**Hole 3 -- non-test rows, which is structural.** `n/a` is documented in every contract preamble as the doc / eyeball status, while the proposed grammar demanded a backticked path on every row. Those rows therefore had **no legal form to migrate to** -- three of them in this repo alone. Hence two arms, and the enforced biconditional: `n/a` if and only if `(non-test)`. This also produced **L5**, which the one-armed grammar could never have expressed: a `(non-test)` AT can never be green, so it can never satisfy a test-backed AC, and being the sole cover for one is a silent hole the grammar would otherwise have blessed.

**What shipped.** Two anchored arms; every field reader is one line over the matching arm, so a non-conforming row yields NO field rather than a plausible wrong one (which is what made the failures silent -- a half-parsed row still looks like a parsed row). `intent at lint <ID>` reports five checks: **L1** grammar, **L2** a `green`/`red` row's cited file exists (`to-write` exempt -- a missing file is the correct state for a test not yet written), **L3** the cited file contains the literal AT id, **L4** every covered id is a real AC, **L5** the non-test sole-cover trap. `--fix` migrates the mechanical half and guesses at nothing. The lint folds into `ac gate`, and `at green|red` refuse a dangling citation at the moment it goes load-bearing.

**The `--fix` migrator shipped broken and was caught only by running it on a real estate.** It compared the repaired path against the _raw_ reference rather than the canonical backticked form, so the single commonest migration -- a correct repo-relative path merely lacking backticks -- was skipped silently. Residual findings across this repo dropped 27 to 13 once fixed. A dry run could not have shown it.

**Own-estate sweep:** 116 AT rows, 103 migrated mechanically, 13 by hand, with the diff touching only AT rows.

**Second `--fix` defect, reported from a consumer estate the same day and fixed before the cut.** The basename resolver ran a **full-tree walk per non-conforming row**, and it ran for references that cannot be filenames at all. On a 65GB repository whose rows carried a `[to-write]` placeholder in the reference slot, that meant scanning the entire tree twelve times -- about four seconds a row on a warm cache, longer cold, which presents as a hang. Two further problems fell out of the same read:

- **`find -name` takes a GLOB.** `[to-write]` is a character class, so it matched any single-character filename; a unique hit in a subdirectory would have rewritten the row to cite a file with nothing to do with it. A migrator that guessed is precisely what the grammar exists to prevent, and this one could have.
- **The exclusions filtered but never pruned.** `-not -path '*/node_modules/*'` still descends into the tree and discards results afterwards, so the exclusion list looked like it bounded the walk while bounding nothing.

Now: the search only runs for a string shaped like a filename (extension, no separator, no whitespace, no glob metacharacter), excluded trees are pruned, and results are memoised for the run. The guard's first test **passed for the wrong reason** -- it planted the decoy file at the repo root, where the path group's required `/` rejects it regardless -- and mutation testing caught that the guard could be deleted with nothing failing. The decoy now sits in a subdirectory, where it genuinely bites.

**Third `--fix` defect, from the same consumer, and the worst of the three: it was LOSSY.** Once the hang was gone they ran it on a real thread, measured the damage and reverted. On ST0264 it turned 32 findings into 31 at the cost of four rows losing a second real cited file (all four existed and carried the cited tests), ~17 rows losing the `::"name"` that was the only statement of which test covered the AC, and three L1 findings becoming L3 ones -- a row that now reads complete and points at nothing.

**The root cause was the SUGGESTION, not the fixer**, which is the consumer's insight and the more valuable half. `at_row_defect` reported everything before the first `::`, so on a row citing two files it printed "cite the FILE only (`...worlds_controller_test.exs`)" and silently discarded the second. **Anyone hand-following that advice lost exactly what `--fix` lost.** A lossy fixer damages the contracts it touches; a lossy suggestion damages every contract a human touches after reading it.

Both ends fixed:

- **The diagnostic names every file the row cites** (`at_ref_files`, splitting on `+` specifically, because the test names themselves contain `+`), states that the grammar admits one, tells the author where the others belong (the trailing note), and says outright that `--fix` will not touch the row. It never tells anyone to drop anything.
- **`--fix` refuses any row whose reference span is not a lone path token.** The stripping of `::"name"` and `("name")` suffixes is gone entirely. The migration is two-ended -- cite the file, put the AT id inside the test -- and a fixer that can only do one end must do neither, or it leaves behind a contract that reads complete and proves nothing.

Verified by simulation against the reporting estate, read-only, before any of it shipped: 1642 AT rows, 268 still migrate mechanically, **zero lose a cited file or a test name**. On ST0264 specifically: zero rows rewritten, all 32 left for a human, all 26 names and all 6 multi-file citations intact.

**The existing test asserted the lossy behaviour as the contract** -- it fed `deck.bats::"a name"` to `--fix` and asserted the name came out. That assertion was mine and it was wrong; it now asserts the row is untouched. My first mutation of the new guard also survived, because it left the whitespace check that already caught the quoted shape; the faithful revert (restoring the stripping `sed`) kills two tests, which is the check that counts.

**What `--fix` correctly does NOT do**, since the same report asked: a retired `path::name` citation cannot be migrated mechanically. The fix is to cite the file and name the AT id inside the test itself, which means editing test files -- owner work, per lane, not a records sweep. `--fix` migrates the delimiter and leaves the citation reported for a human, which is the right division.

**Later correction (vc audit F1).** Each arm has its own status vocabulary and nothing enforced that, so `intent at na` on a test-arm row wrote a status the row's grammar does not admit: the write LANDED, pushed the row out of the grammar, and the strict reader then read nothing back -- so `assert_written` reported "the file was NOT updated" about a file it had just corrupted, and left the invalid row behind. Refused before any write now, using the arm the grammar already computes. The write verifier's message was corrected in the same pass: its own design comment says it verifies the RESULT and not the mechanism, so it may not conclude the file was untouched.

### Accepted consequences, recorded rather than left to be rediscovered

- **L2 applies to COMPLETED threads** (vc O1). A test renamed after a thread closed makes that thread's readers warn and its gate report BLOCKED. Nothing invokes the gate on a completed thread mechanically, so this surfaces only if someone asks. Accepted: the alternative is a contract that stops telling the truth the moment it is closed.
- **The `at_grammar` upgrade step re-runs on a judgement-residue-only estate** (vc O2). Its probe reads "needed" while any residue remains, so every `intent upgrade` re-runs a no-op `--fix` and re-prints the residue note until a human clears it. The nagging is deliberate: residue is rows a machine must not guess at, and a step that went quiet would let them sit forever.
- **The gate blocks unswept estates from the day this ships.** That is the fix working. Every row it names was already contributing no coverage, silently.
- **AT-name traceability (the ST0043 deferral) is superseded.** The id is the traceable token, not the `@test` name, so the deferred work no longer has a subject.
