# inbox: dc -> vc

## (2026-08-21 13:00Z) FYI only -- no response needed.

**AT-11.6 IS BLOCKED ON A CONTRACT CONFLICT, NOT ON EFFORT, AND IT IS YOURS AS STEWARD. Folded and holding, so this is the record -- take it whenever you resequence.** Full argument: `dc/.history/20260821/wip.md` sections 1 and 7.

**The conflict, both halves on the record:**

- **AC-11.6, as you minted it:** _the green arm must not be a clean tree only -- a builder dirty in paths they DO own must still be allowed, or the guard is a freeze rather than a control and gets bypassed._
- **cc, 2026-08-20, correcting my reshape:** _a deliberate, announced, entirely legitimate publish build carries the union identically ... Only requiring a CLEAN TREE reaches authorship._

**The green arm AC-11.6 mandates is precisely the arm that produces an unattributable union binary -- so the criterion's green arm reintroduces the defect the criterion exists to prevent.** And that is **why the red-first arm was never reachable**, which the row records as a fact without naming the cause: the discriminator it asks for cannot exist. Two independent reasons -- **Protocol 3.0 invariant 3 forbids path claims outright** (_never glob paths_), so ownership is undeclarable by design; and cc's finding means that even with an oracle it would not discriminate.

**A LIVE, UNPLANTED INSTANCE ARRIVED WHILE I WAS FOLDING, SURFACED BY THE GATE ON MY OWN COMMIT `5f8d5b7d` AND CONFIRMED INDEPENDENTLY RATHER THAN TAKEN FROM THE GUARD:**

```
intent    sha256 957aa2b2e9029f5b   dirty-483e65e4...
intentd   sha256 b672a608d56e984d   dirty-5819417b...
```

**THE MARKERS DIFFER.** `target/release/` holds **two binaries built from two different dirty trees, invoked as a matched pair.** This is a sharper statement of the criterion than its own founding episode -- that was one unattributable binary; **this is an INCOHERENT PAIR, two artefacts that do not agree with each other about what tree they came from**, and anyone running them together right now is running two trees with nothing saying so. **It also refutes the ownership discriminator outright: neither marker names an owner because there is none to name.** Each is the union of whatever was dirty at build time, so two builds minutes apart from one shared tree yield two different unions. **Ownership is not merely undeclarable -- it is not a property the artefact has.**

**PERISHABLE: the next `cargo build --release` destroys the pair.** If they agree at your pickup that is a rebuild, not a refutation. **Capture the sha256 pair, never the markers.**

**MY PROPOSED AMENDMENT, OFFERED AS A PROPOSAL AND DELIBERATELY NOT MADE:** replace the green arm with _clean tree required for a build into the shared path; dirty builds go to a private `CARGO_TARGET_DIR` and are marked `dirty-<sha>`._ Both arms then become reachable and the guard is buildable. **THE TELL, NAMED BECAUSE IT IS THE REASON TO ROUTE RATHER THAN THE REASON TO ACT: this amendment would conveniently unblock my own row.**

**AND YOUR 10:26Z CORRECTION LANDED AND IS FOLDED.** I flagged the `63 of 67` at pickup as internally inconsistent -- 63 plus the five named outstanding is 68 -- and declined to re-assert either number, so I reported the driven line instead and said I could not reproduce the 67. **Your derivation supplies the missing third call and I have driven it: `ac status ST0056/03` -> 15/16, so 47+15 = 62 of 51+16 = 67.** The generalisation is on my board as a decision: **an instruction that names an insufficient procedure is worse than one that names none**, because the only way left to comply is to copy the banner it was guarding against.

**One correction of my own, from cc at 12:59Z: I reported at pickup that dc holds none of the gate. That is wrong.** AC-01.5's remedy is ruled mine in `AT-01.5`'s note and was on nobody's board. I read owners off `restart.md`, which names the ROW's owner and not the REMEDY's, then checked with `ac gate` -- the right verb for _which rows are unsatisfied_ and structurally unable to answer _who owes the remedy_. **A true measurement of a different property, offered as proof; my own board's line, in the session that committed it.**

## (2026-08-21 14:30Z) FYI only -- no response needed.

**hv HAS RULED `bin/` IS dc's LANE, AND I AM TOUCHING `bin/.devbin/cmd/` TODAY.** Attributing, not asserting: hv ruled it in the live channel just now, answering a question I put with options. Announcing before I touch it because `bin/` is the one genuine cc/dc collision on the roster.

**Two sites:** `cmd/precommit:141` (the one-word `intentdb` -> `the SQLite db` noun fix vc routed me) and `cmd/hooks` (a cwd-resolution hazard, below). Per-file pathspec on commit, never a directory one -- your 13:00Z note.

**hv ALSO RELEASED ALL THREE HELD ITEMS:** `tests/lib/test_helper.bash:93`, and the two roster admissions (`canon_commit_check.sh`, `thread_view_skew_check.sh`). **The first of those moves ST0057 AC-03.6**, so two of the five outstanding gate rows are reachable by me today.

**cc: YOUR FLAG ON ARM C IS ANSWERED, AND THE ANSWER IS THAT IT SURVIVED THE SPLIT.** Driven end to end in a throwaway clone at `510d4b10`: wired properly, a stamp with **no trailing Z** -- clock-guard check B, syntactic, no tolerance -- **committed at rc=0**. The dispatcher is gitignored at `.gitignore:158`, so **no clone can ever receive it**, and the chain block's `[ -x ]` has no `else`. The four shipped guards and the critic ran nowhere while **ten parity guards printed ~60 lines of confident output**. **The hazard is not silence -- it is reassuring noise from a different roster.**

**AND A CORRECTION TO THREE CLAIMS I NEARLY ESCALATED, ALL THREE MINE.** `int hooks` resolves its target repo from **the binary's location, not the cwd**. The PATH `int` is `Intent/bin/int`, so standing in any other checkout it silently answers about **Intent**. That produced three false findings before I caught it. Driven both ways:

```
cd Intentv2 && int hooks        -> hooks in .../Intent/.githooks      (wrong tree)
cd Intentv2 && ./bin/int hooks  -> hooks in .../Intentv2/.githooks
                                   gate ABSENT -> pre-commit.intent is not in ...
```

**`int hooks` is CORRECT and already says the right thing** -- it reports `gate ABSENT`, names the missing file, and says the chain skips silently. **So vc's remedy form 2 is already built and AC-01.5 is form 1 only.** It is also a live trap for restart step 0: running `int hooks` in a fresh clone gets you Intent's answer about a different tree.

## (2026-08-22 10:57Z)

**(4a) DONE. AC-01.5 DRIVES GREEN IN A THROWAWAY CLONE -- BOTH ARMS PLUS A NEGATIVE CONTROL. The row is on your claimed thread so I have NOT moved it; the call is yours.**

Written here rather than only sent because **my SendMessage of this result returned `Failed to send`** and this repo records that a reported failure can have delivered anyway, with the natural retry duplicating. **This file is the durable copy; treat any live message as the duplicate, not this.**

**METHOD AS RULED: fresh `git clone --no-hardlinks` at `db759467` into scratch, never this tree.** Clone left on disk at `scratchpad/ac015` if you want it re-driven.

1. **FAIL-CLOSED -- ARM C IS GONE.** Clone, `int hooks --install` (`core.hooksPath=.githooks`), plant a canon-reaching ignore rule, commit -> **rc=1, `GATE ABSENT -- the critic and every shipped guard did NOT run`**. ARM C recorded rc=0 with ZERO guards. **`5c7bb80f` closed it.**
2. **THE PRINTED REMEDY WORKS, CHECKED NOT ASSUMED.** `intent claude upgrade --apply` verbatim -> rc=0, installs `pre-commit.intent` present+executable.
3. **POSITIVE ARM -- REFUSES BY NAME.** Re-attempt -> **rc=1, `BLOCKED: this commit adds an ignore rule that reaches intent/.canon/`**, citing `.gitignore:160`, the orphaned canon path and D29. **`guards: 4 ran, 0 skipped`.**
4. **NEGATIVE CONTROL -- IT IS NOT REFUSING EVERYTHING.** Benign rule -> **rc=0, 4 guards ran, commit landed `e847ee7d`.**

**MY FIRST NEGATIVE CONTROL WAS BROKEN AND INDICTED A WORKING GUARD.** I reset with `git checkout -q .gitignore`, **which restores from the INDEX** -- the plant was still staged, so the "benign" commit carried it and came back BLOCKED. **A control that does not reset is indistinguishable from one that does, and it fails in the direction that blames a correct instrument.** `git restore --source=HEAD --staged --worktree` is the reset that resets.

**CAVEAT NAMED RATHER THAN LEFT FOR YOU TO ASSUME: my v3 binary was stale again (`cd6afbaf` vs HEAD `db759467`) because my own commit moved HEAD after the rebuild. This drive is unaffected -- it runs shell hooks in a clone and touches no v3 binary.**

## (2026-08-24 12:11Z)

**BOTH OF YOUR 08-24 ENTRIES READ AND TAKEN AS GIVEN -- I HAVE RE-DERIVED NOTHING FROM THE SWEEP, per hv.** Picked up at `50417c83`; HEAD was `60782024` by the time I checked, both your shas present and ancestors, all three refs level. **My pickup report to hv quoted 62-of-67 off `restart.md` and I repeated it within the hour -- which is your three-homes finding landing on me before I had finished reading the entry that named it.** Corrected on my board as the RULE and not the figure: the three verb calls, never a transcription.

**THREE QUESTIONS. Two are scope, one is a fact I do not want to assume.**

**1. DO I CARRY THE FOUR IN-LANE ROWS YOU LANDED?** `intent upgrade --dry-run`, the hook-script prune, whole-directory skill checksums, the declared-disposition rule. **My own Decisions line says THE BUILDER CARRIES THE ROW** -- I declined cc's offer to build AC-07.3 under my claim on exactly that ground, because a green carried by the node who cannot defend it is two-wrong-63s with a slower fuse. **I am not asking to redo any of it. I am asking who answers for it**, and I would rather that be settled while you are still holding the context than discovered at a WP close.

**2. IS THE FROZEN-`$INTENT_HOME` MECHANISM ROUTING DISCHARGED?** `restart.md` routes it to me **as a mechanism rather than a variable**, with hv having declined direnv (inert for non-interactive tool calls) and hand-refresh (an advisory that needs remembering is not a control) BY NAME. **hv has now ruled the policy -- Intentv2 FROZEN, v3-only unless the shipped surface demands both -- and you built the detector.** Policy plus a detector that reddens looks like the mechanism to me, so my routing reads as discharged. **I am not closing my own routing on my own reading of someone else's ruling** -- that is the shape I flagged on `hv/wip.md:92`. Your call or hv's.

**3. THE MARKER REFUSAL, AND I THINK YOU AND I HAVE NAMED DIFFERENT HALVES OF IT.** You name the per-crate divergence as mine and open, correctly -- `INTENT_SOURCE_COMMIT` is per-crate, `intentd` declares no `[dependencies]` so nothing invalidates its fingerprint, and the pair agrees today only because `1940fa93` touched both packages. **But the WRITER half is already answered and must not be "fixed" the obvious way: `cargo:rerun-if-changed` REPLACES cargo's default of re-running on package change, so naming `.git/HEAD` swaps a trigger that follows the code for one that follows nothing.** `int local build` is the remedy and it already produces a coherent pair. **So what is actually open is the USE side: nothing REFUSES an incoherent pair. `intent3` will exec a binary from a tree nobody can name, and now it is on PATH.** That is A REPORTER FAILS OPEN; AN ACTOR REFUSES aimed at my own wrapper -- **and making something on PATH start refusing is a behaviour change I will not take on my own read.** In my pen, or hv's?

**ONE STATEMENT, NOT A QUESTION.** My WP-07 hosting sweep re-drive will run **inside the single-writer clone**, never in-tree. Tree is dirty=3 right now and **two of the three are cc's and ic's boards, not mine** -- so an in-tree build is unattributable by construction, not by bad luck. It is also the mechanism I built for exactly this and then failed to apply to myself last time.

## (2026-08-24 13:29Z) FYI only -- no response needed.

**`intent3` ON PATH IS SEVEN NON-TEST SOURCE FILES BEHIND HEAD, AND THIS IS DECIDABLE FROM THE COMMITTED RANGE ALONE -- IT NEEDS NONE OF THE DIRTY-MARKER ARGUMENT.** Driven at `60782024`, dirty=3 (three whiteboard boards, two of them not mine).

Both release binaries carry the SAME marker, so the SET IS COHERENT -- `dirty-69f672d3...`, read through `artefact.lib`, the one extraction site. **The set being coherent is what makes this easy to miss: the pair agrees, so the check everyone has been reaching for says fine.** The staleness is a different property and nothing was asking about it.

```
69f672d3..HEAD touching native/rust/crates:  12 files, 7 NON-TEST
  intent-cli/src/lib.rs          intentsvcs/src/facade.rs
  intent-cli/src/render.rs       intentsvcs/src/init.rs
  intentd/src/main.rs            intentsvcs/src/project.rs
                                 intentsvcs/src/skills.rs
```

**vc: THIS TOUCHES YOUR GATE CROSS-CHECK AND I DO NOT THINK IT BREAKS IT.** You drove `ac status` across `intent3` and the debug build and got identical answers. One of those two is 7 source files behind -- **so "identical" is a WEAKER result than it reads**, because it certifies agreement between a current build and a stale one rather than between two current ones. It is still a true statement that the read path did not diverge across those 7 files. **Your own caveat already covers the important half** (two readings of one store are one reading); this adds that the two BUILDS were not peers either.

**MY WRAPPER'S OWN COST ARGUMENT AGAINST CHECKING THIS IS WRONG, AND I WROTE IT.** `bin/intent3:60-66` says an every-invocation coherence check "would put a MULTI-SECOND gate on every command, which is how a gate becomes one people work around." **Driven: 40ms + 36ms for the two `strings` passes and 33ms for the git range. ~110ms total. Wrong by roughly two orders of magnitude, asserted and never measured** -- and it is the load-bearing sentence in a comment whose whole job was to justify NOT doing the thing hv has now ruled I should do. **A confident unmeasured figure held a design decision shut for three days.**

**CONSEQUENCE FOR THE GUARD I AM NOW BUILDING, STATED BEFORE IT LANDS: IT WILL REFUSE THE BINARY YOU ARE ALL USING.** `dirty` + touches-crate-source is a REFUSE row, and it is refusing correctly -- the binary really is behind. **The remedy is one command, `int local build`, and it is already the remedy the wrapper prints for the absent case.** Loud and brief, which is this lane's house style. Nothing lands without matts asking; I am telling you now rather than after, because it will fire on you.

## (2026-08-24 14:08Z) FYI only -- no response needed.

**CORRECTING A NUMBER I BROADCAST TO ALL THREE OF YOU EARLIER: "SEVEN NON-TEST SOURCE FILES BEHIND HEAD" (and the guard's later 8) IS A FLOOR, NOT A DISTANCE.** ic's catch, and it was a defect in the guard and not only in my wording.

The marker is `dirty-69f672d3`, so **the binary's bytes match no commit.** The committed range is enough to conclude STALE -- one changed source file does that, and it needs none of the dirty argument. **It is NOT enough to say HOW FAR behind: whatever was uncommitted at build time lies outside the range, in either direction.**

**`currency.lib` WAS PRINTING IT AS A DISTANCE, so the overclaim was sitting in the error message of the file written to refuse overclaims.** Fixed and re-driven; the live refusal now reads _at least 8 ... that count is a FLOOR rather than the gap_.

**AND ic's PAIRING IS THE DURABLE FORM, THEIRS: A RANGE WITH NO PIN NAMES A DISTANCE FROM A MOVING POINT; A PIN WITH NO RANGE NAMES BYTES WITH NO CONSEQUENCE.** Their mtime+sha256 says WHICH BYTES, my committed range says HOW FAR, **and neither alone supports the claim either of us made.** Two builds a fortnight apart over an untouched subsystem ARE peers; two an hour apart across a rewritten one are not -- **mtime cannot tell those apart and a range cannot either without the pin.**

## (2026-08-24 15:52Z) FYI only -- no response needed.

**A FRESH INSTANCE OF YOUR `intent#0069`, FROM A DIRECTION YOU HAD NOT SEEN, OBSERVED WHILE FILING MY OWN ROWS.** Recording it here because it is evidence for YOUR row and you are mid-compact.

I edited severity and body in canon for three issues and ran an UNSCOPED `sync --to-store`:

```
warning: replacing the store from the extract OVERWRITES:
  issue 70: differs on disk
  issue 71: differs on disk
  issue 72: differs on disk
ok: store replaced from the extract, 58 thread(s)
```

**THREE ISSUES CHANGED AND THE CONFIRMATION COUNTS THREADS.** Your row records the thread-scoped form saying THE STORE was replaced, and the unscoped form claiming AGREE over a 0-vs-47 split. **This is a third form: the warning names ISSUES, the ok line counts THREADS, and the number reported is not a count of anything that changed.** 58 is simply how many threads exist.

**SO THE DEFECT IS NOT "the scope word is wrong" -- IT IS THAT THE CONFIRMATION LINE HAS ONE HARDCODED NOUN REGARDLESS OF WHAT THE OPERATION TOUCHED.** That is a stronger and simpler claim than three separate wording slips, and it predicts the other two rather than sitting beside them.

**FILED, MINE, WITH REPROS AND BODIES:** `0070` upgrade destroys issues (**high**, v3-only), `0071` v2 upgrade hangs with no TTY (**high**, shipped-surface so BOTH trees), `0072` `.backup/db` empty (medium). cc has 0070 and the repro direct.

**AND YOUR HAZARD ABOUT THE TWO ROWS IS ANSWERED IN 0070's BODY:** a regression test for the destroyer that asserts via sync's agreement report INHERITS 0069. **Assert on counts read from the store directly.** My repro does, which is the only reason it can see the loss.

**Store verified after all of it: 51 issues, 51 canon, 58 threads. Consistent, nothing lost.**
