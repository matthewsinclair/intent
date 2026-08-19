---
node: cc
name: Control Claude
role: control
session_id: 706144a4-c6ed-49be-8c4d-8835993e7930
heartbeat_at: 2026-08-19 07:59Z
status: active
focus: "**THE MOVE LANDED (`16048f82`) AND SO DID AC-03.16 (`4304d8f4`).** 97 canon files into `intent/.canon/`, byte-identical, vc re-verified every rename blob SHA and fired the of-N trigger. Suite **650/0/0 across 89 suites**. **AT-01.7 red 1-of-8 to green 8-of-8 across the move; AT-03.17 red 205-of-266 to clean** -- both red-firsts TAKEN, one of them in a window that was closing rather than opening. AC-01.5s guard built + mutation-proven but UNWIRED, so vc has it `red`; the roster re-cut is dcs and waits on matts. **HELD: 263 skewed views, pending a clear-to-run, because attachments are disk-FIRST and a sync would eat a peers in-flight edit silently.**"
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon. **ST0057 is INSIDE the 3.0.0 gate** (hv).

**D42 -- TIME, and it has no clauses.** The create door stamps; the restore door carries. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. `one_clock.rs` enforces it structurally and has caught me.

## THE MOVE LANDED -- `16048f82`, and what it cost to get right

**97 files moved, byte-identical, keyed by the id read from CONTENT.** 57 -> `intent/.canon/st/<ID>.json`, 40 -> `intent/.canon/issues/<NNNN>.json` (zero-padded four). Verified AT HEAD with `git ls-tree`, never `git grep` -- **git grep reads the INDEX, so a staged-but-uncommitted move reads as landed** (vc). 42 legacy `{OPEN,CLOSED}` correctly did not move. `check-ignore` rc=1. Suite **648/0/0 across 88 suites**, fmt clean. Required step: 57 rows, 52 Completed / 2 Cancelled / 2 WIP / 1 Not Started, byte-identical to the pre-move listing.

**AT-01.7 CROSSED THE MOVE RED-TO-GREEN, AND THE WINDOW WAS ALMOST GONE.** `f41d6760` repointed `schema/ddl.sql` in the SAME commit as the resolver, so 7 of 8 `carried by` declarations had been dangling since then and **the move REPAIRED them rather than breaking them.** The row prescribed "apply the relocation, require RED" -- applying it now yields GREEN. **A RED-FIRST ARM PHRASED AS _apply X, require RED_ CARRIES AN UNDECLARED EXPIRY: it encodes an assumed starting state, and X landing in two parts puts the red in the gap between them, unobserved and unclaimed.** Nothing in a row says when its prescription stops being executable. Written before the move by one decision; had I moved first the red was unrecoverable without faking it.

## NEXT -- mine

1. **AC-01.5: the guard is BUILT and UNWIRED, and vc has set AT-01.5 `red` on that ground** -- `lib/templates/hooks/canon-ignore-guard.sh` at `91736056`, six arms driven in a real clone, both load-bearing halves mutation-proven (force attribution true -> inherited arm 0->1; drop the synthetic future-probes -> future-only arm 1->0). **Nothing consults it, and a capability with no consumer is not a gate.** Wiring means generalising `WB_GUARDS` past its `intent/whiteboard/` presence test; **dc takes that together with the distinct-exit-code re-cut of the same block -- one change, both ends, one commit -- and is NOT starting it without matts**, because it re-cuts fail-open semantics every consumer inherits on upgrade.
2. **AC-01.1 .. AC-01.4 are measurable for the first time** now the layout is real. AT-01.1/01.3 exist; AT-01.2 (clone completeness) and AT-01.4 (concurrent diff) are unwritten.
3. **AT-01.7's row still says `to-write` in canon while the test is GREEN.** Flipping it is a canon write; vc asked for none while I was mid-move and I have not made it. Resolve who flips it.
4. **ST0057 AT-03.15's ten-verb hold** (vc, `11fdc168`). Start with `todo list` and `at lint`: both classified `mutate` while their names say read.
5. **ST0056 AC-03.16 / AT-03.17 -- LANDED at `4304d8f4`.** Three sites, not the two vc named. Red-first TAKEN by reverting the fix: 205 claims across 266 rendered views, clean with it in place. **263 of 266 views are now SKEWED on disk and the regeneration is HELD pending a clear-to-run from all three peers** -- `sync --to-disk` is a 263-file estate-wide rewrite and attachments are disk-first.
6. **The stale-AT guard's mirror blind side** -- catches a `to-write` row citing a file that EXISTS, cannot catch one citing FICTION. Wants its own red-first.
7. **The cold-warm `doctor` arm stays PARKED** -- "a file on disk canon does not know about" changes meaning under sparseness.
8. **`critic rust` and `critic shell` arm ZERO rules.** A green from either means "nothing asked a question". dc owns Half B.

## THE TWO-ENDED CLASSIFIER -- still live, and MORE dangerous after the move

**The discriminator (canon at `c88a6f06`): WHETHER THE UNMIGRATED END CAN STILL PRODUCE A VALUE.** A missing directory cannot -- `NotFound`. A string operation always can. **The move made the silent kind WORSE, not better: `${var#pat}` returning the string unchanged yields `ST0056.json`, and against a FLAT layout that is now a plausible-looking filename rather than obvious nonsense.**

**AND THE ID AND THE FILENAME AGREE FOR ALL 57 TODAY -- measured, zero mismatches.** So a path-derived identity and a content-derived one are **indistinguishable on the current estate**, and any instrument deriving identity from the path passes every check anyone can run right now. `jq -r '.id'` is correct for a reason nothing in the tree will presently demonstrate.

## Watch-outs -- the mechanisms, distilled

**A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT** (vc). Four limbs, one rule, every one with a live instance from a different node inside two days:

| limb                                                      | the freedom                                                                 | today's instance                                                                  |
| --------------------------------------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| **INSTRUMENT** -- which tool                              | git and mtime give OPPOSITE answers                                         | AC-01.4; the fix was in the criterion, not my reading                             |
| **DEPTH** -- how far in, of SUBJECT and DENOMINATOR alike | an internal subject lets the test reach PAST the thing tested               | `view_determinism.rs` drove `write_all` and stayed green while the estate churned |
| **EXTENT** -- how much of the right KIND                  | a subset of the right kind, **both figures observable, so DEPTH passes it** | 20 of 20 views against a writer population of ~364                                |
| **PIN** -- at which revision                              | measure at a NAMED COMMIT, never `HEAD`                                     | three `intent` binaries carried `dirty-18197aaf` in one day                       |

**IDENTITY AND EXPECTED VALUE MUST COME FROM OUTSIDE THE THING UNDER TEST** (ic's unification). Content-derived identity survives a relocation; path-derived does not. A written-down expected value is a check; one derived from the subject is a tautology. **Take either from the subject and the artefact loses the ability to disagree with it.**

**A PROBE WHOSE POPULATION CANNOT CONTAIN THE FAILURE IT TESTS FOR.** `sync` printing _the store and the extract agree_ over **0 == 0**; `git status` vouching for an mtime-only prediction it is blind to; an all-changed subset vouching for a skip that never fired; **and my own required step, which I minted deliberately after naming the class.**

**A TOOL'S DEFAULT VIEW IS A FILTER, and a command passed on without the flag it was MEASURED with silently inherits that default** (ic). Nothing about the transmitted form looks incomplete, which is why it survived three readers.

**A CHECK THAT VALIDATES THE FORM OF A CLAIM AND NEVER ITS REFERENT.** `openness.rs` required a declaration to start with `carried by ` and never looked at what it named. The drift checker would have blessed `intent/.canon/<ID>/thread.json` -- right prefix, wrong flat shape -- **because the wrongness is in a dimension it does not inspect.**

**MUTATION-TEST EVERY GUARD, AND RE-TEST WHEN THE MECHANISM CHANGES.** **Twice in one day a correct guard sat on a path the failure does not take** -- the AC-03.14 skip on `write_all`, and AT-01.6's first version, which stayed green with the historical bug re-introduced. **Both found by mutating, neither by reading.**

**THIS WORKSPACE HAS TWO CRATES AND ITS MEASUREMENTS KEEP HAVING ONE** (vc). I asserted "the fixtures are fine" from `intentsvcs`'s shared fixture; `intent-cli` has SEVEN private `seed()` copies. ic retracted a claim the same evening whose reach was also one crate of two, opposite direction. **Until an instrument states its crate coverage in its own output, both of us keep paying.**

**`target/release/` IS A SHARED MUTABLE ARTEFACT four nodes read.** Snapshot and sha256 before measuring; **announce before rebuilding**. `int prepush` does it right: clone to a temp dir, build there. **A rig assembled by SYMLINKING into the real tree is NOT isolated** (dc): `cp` onto a symlink follows it and writes through.

**`cd` DRIFT: use absolute paths.** Five times today. **NEVER TRUNCATE A TEST RUN** -- sum every `test result:` line. **`git commit --only` ON AN UNTRACKED PATH STAGES NOTHING** and reports a true count about what it DID commit.

**A PIN FIXES WHICH ARTEFACT YOU MEASURED WITH AND SAYS NOTHING ABOUT WHETHER IT CAN SEE THE CHANGE UNDER TEST -- AND THE TEST FOR THAT IS A CONTENT TEST, NEVER A CHRONOLOGICAL ONE** (mine, vc corrected the argument). The PIN limb exists because three binaries carried `dirty-18197aaf` -- ambiguous identity. **This is the opposite failure: the identity was unambiguous, agreed by three nodes, stable, and being pinned is exactly what made it look trustworthy.** `f2e4d1f9` reported the whole estate unmigrated after the move because it predates the resolver; `strings` gives 0 `.canon/st/` and 9 `/thread.json`. **I had that content evidence and then reached past it for the clock, which condemns the wrong binaries: a DIRTY-TREE BUILD CARRIES CODE YOUNGER THAN ANY COMMIT STAMP**, and both debug binaries predate `f41d6760` by 45 minutes while CONTAINING the repoint. On an estate where every binary this week came off a dirty tree, build-time-vs-commit-time is wrong more often than right. Specimen preserved as `intent-f2e4d1f9-pinned-but-blind-to-canon`.

**AND THE DIRECTIONALITY IS WHAT HID IT, ON THE REQUIRED STEP ITSELF.** Before the move a stale binary and a current one AGREE at 57, because the v2 paths still exist; they diverge only at the instant the change lands. **So the conservation check's before/after pair has DIFFERENT INSTRUMENT VALIDITY AT ITS TWO ENDS, and the "before" reading is precisely what certifies the stale instrument taking it.** The step's whole design is that a matching before/after proves conservation. **And it fails as a false positive that reads as catastrophe**: it names 57 threads unmigrated and prints a remedy which, followed, runs `upgrade` over a correctly-migrated estate. **THE STEP CANNOT TELL "THE ESTATE IS BROKEN" FROM "YOUR BINARY IS OLDER THAN THE ESTATE", AND IT REPORTS THE FIRST.**

**A PARTITION MUST CLOSE OVER THE POPULATION THE CRITERION IS ABOUT, AND CLOSING IT OVER THE WRONG WHOLE IS ITS OWN DEFECT.** Three instances, three nodes, one morning. Mine twice: a `st list` breakdown read **53 Completed and did not close (58 over 57 rows)** because the table's own HEADER ROW carries the literal word `Completed` -- **the header is not a member of the population it labels, and only the failure to close exposed it; at one Completed fewer I would have matched the recorded figure and been WRONG WITH CORROBORATION.** Then, applying that very fix, AT-01.7's partition over `^CREATE TABLE` reads **10 against 11 declarations** because `doc_sections` is `CREATE VIRTUAL TABLE ... USING fts5` -- which would have failed the test for a reason unrelated to its criterion. ic's is the same detector: `EXAMINED 2 of 1 ... the other -1`. **A REMEMBERED FIGURE IS NOT INDEPENDENT CONFIRMATION WHEN THE SAME METHOD PRODUCED IT** -- that is one measurement twice, not agreement.

**A PROBE THAT IS RED FOR A REASON UNRELATED TO ITS CRITERION IS WORSE THAN ONE THAT IS SILENT, BECAUSE IT WILL BE READ AS THE CRITERION.** `declarations_in` deliberately absorbs the comment lines following a declaration -- adjacency is the binding -- so two real declarations arrive with a paragraph attached. Taking the remainder as the path gives a referent that dangles **red before the move AND red after it, indistinguishable from a failed move.** Caught by READING the DDL, not by running.

**ATTACHMENTS ARE DISK-FIRST; VIEWS ARE DISK-DISCARDED. SAME DIRECTORY, SAME APPARENT KIND, OPPOSITE DIRECTIONS, BOTH SILENT** (vc, who lost a commit to it and sent the mechanism rather than just the fix). `acceptance.md` is a GENERATED VIEW -- a row authored there is discarded at the next `--to-disk`. `design.md` / `impl.md` / `tasks.md` are INGESTED ATTACHMENTS -- **the authoring surface is the FILE, and an edit made to the copy inside `thread.json` is discarded at the next `--to-store`.** Edit the file, then sync, then commit. **Only one direction was ever written down and I had internalised only that one.**

**`git diff --stat` CANNOT EXPRESS SIZE ON CANON JSON** (vc). An attachment is ONE LINE, so a 9,740-byte file entering canon and a one-word typo both read as a single insertion. Review a canon commit by comparing STRUCTURES, never by its diffstat.

**A DETECTOR THAT READS A RENDERED VIEW CANNOT TELL THE GENERATOR'S WORDS FROM THE AUTHOR'S, AND REPORTING THE DEFECT BECOMES THE OFFENCE.** My first AC-03.16 checker found 3 of 266 and **all three were authored prose -- one of them AC-03.16's OWN ROW, quoting the sentence it exists to describe**, one a v2-era criterion in ST0044 that is TRUE about v2. This is the whiteboard header guard's "never scans prose" ruling arriving somewhere new. Attribution now excludes anything present in the canon the view was rendered FROM, **per thread and never estate-wide** -- estate-wide, AC-03.16's quotation would excuse every generated occurrence everywhere. **The cost is a STATED hole, not a hidden one:** ST0056 alone cannot be caught by the estate arm for that sentence, so the checker is also asserted against the historical wording as a plain string. Two arms because neither denominator can cover the other.

**`bash -n` AT 3.2 IS BLIND TO EXACTLY THE CLASS IT LOOKS LIKE IT COVERS** (dc, correcting my own published claim). I ran `/bin/bash -n` on the guard and wrote "bash-3.2 clean" into a commit message. **3.2 validates neither parameter-expansion operators nor builtin names at parse time**, so `${v^^}` and `mapfile` both parse at rc=0. **My green meant nothing and I had already published it. A ZERO IS NOT A RESULT UNTIL THE CHECK HAS PRODUCED A NON-ZERO.** And the split that matters is dc's: bash-4 SYNTAX fails LOUD at rc=1, bash-4 BUILTIN fails at **rc=0 handing the caller an empty array and a plausible number** -- and not one of the seven shipped hooks sets `-e`, so the silent form is the one this estate gets.

**A GENERATED FILE THAT ASSERTS ITS OWN CANONICITY IN ITS HEADER WHILE ITS FOOTER SAYS DO NOT EDIT** is one artefact making two incompatible claims, and the reader believes the one they were primed for. `acceptance.md` did both. vc named two emit sites; the third was the preamble itself. **Fixing only the named sites fixes the instance and leaves the class.**

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** Both threads carry an `AC-04.4`; four carry an unrelated `AT-03.6`. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE, which is the one failure mode that stops a reader looking further.**
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work.
- **An uncarried file is NOT a disposition** (vc). **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON -- and re-deriving the reason is how you find out the reason was wrong.**
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`EdgeKind::Incidental` STAYS with no user. `doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`.**
- **`config.json` DOES NOT MOVE WITH `intent_dir`** -- `Project::config_path` always answers `intent/.config/config.json`, because something must be findable before anything is configured.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's; `canon_commit_check.sh` is dc's. **Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one** -- commit and ping vc; it is AC-08.5's missing operation, not my failure.

**dc's 88-binary consolidation: they STOOD DOWN.** Their premise was refuted -- matts measured Lamplight's suite at ~10 minutes against our 1m56s, so **we are the faster project and binary count does not explain the slowness.** Their real signal is that the identical suite ran in 16.12s and 100.10s back to back: **contention across nine sessions on sixteen cores.** `CARGO_TARGET_DIR=native/rust/target/cc` is worth taking **for tests only** -- a release build must keep landing at `native/rust/target/release/` where four nodes read it.
