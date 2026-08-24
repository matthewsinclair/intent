# cc -- localfold 2026-08-24 17:35Z

Archived from the live board at the end-of-day fold. **These are SETTLED, not summarised:** the
intent#0070 fix is landed at `3f367cf8` and the 2026-08-21 tree notes are dated and superseded.
The DURABLE lessons from both were lifted into Watch-outs on the live board rather than left here,
because a watch-out in history is a watch-out nobody reads.

## intent#0070 -- `upgrade` DESTROYS EVERY ISSUE IN AN ALREADY-MIGRATED v3 PROJECT. **FIXED 2026-08-24 16:33Z, BOTH ARMS DRIVEN, RED-FIRST ON THE TEST ITSELF.**

### THE FIX AS BUILT, AND EVERY NUMBER BELOW IS FROM A COMPLETED RUN

**`migrate::plan` now unions `issues` against committed canon exactly as it unions `threads`.** New `orphaned_issues()` sits beside `dehydrated()` and carries the same three rulings verbatim, because they transfer without amendment: the union belongs at the JOIN and not inside `legacy::scan`; `issue_numbers()` answers empty for a missing directory, so a genuine v2 estate adds nothing and the first-run path is untouched; and an unreadable or invalid canon REFUSES rather than being skipped.

**THE STRICT READER WAS EXTRACTED RATHER THAN COPIED -- `ingest::read_issue`, mirroring `read_thread`.** Inlining a second `parse::<Issue>` at my call site is how this defect's own class arrives a second time: one reader enforcing the schema and the other not. `read_thread`'s doc comment already says it is public precisely so a second strict reader cannot grow; the issue side had no such reader, which is why the copy was the tempting move.

**ONE CHECK IS NEW AND IT IS LOAD-BEARING ONLY AT THE NEW CALL SITE.** `read_issue` refuses a file whose declared `number` disagrees with its stem, because the union asks _which numbers has the scan not produced_ and then trusts the file under each remaining one -- a `0003.json` declaring `"number": 7` would smuggle issue 7 past a `seen` set tested against 3. **Measured before adding it rather than assumed safe: 51 issue canon files here, 0 mismatches.**

| arm                                      | binary                   | before | after | verdict                   |
| ---------------------------------------- | ------------------------ | ------ | ----- | ------------------------- |
| already-migrated, PRE-fix (dc's script)  | release `dirty-69f672d3` | 5      | 0     | REPRODUCED -- still red   |
| already-migrated, POST-fix (dc's script) | debug `dirty-eabdd639`   | 5      | 5     | FIXED                     |
| real v2 estate -> v3, PRE-fix            | release `dirty-69f672d3` | 5      | 5     | carried all 5             |
| real v2 estate -> v3, POST-fix           | debug `dirty-eabdd639`   | 5      | 5     | UNCHANGED, byte-identical |

**THE POSITIVE CONTROL WAS FREE BECAUSE dc WROTE TWO BINARIES INTO ONE SCRIPT.** The pre-fix release build is still red **in the same run** that shows the debug build green, so the green is a property of the fix rather than of the instrument going blind. A single-arm script would have given me a green I could not distinguish from a script that had stopped looking.

### RED-FIRST ON THE TEST, WHICH IS THE STEP A GREEN TEST CANNOT SUBSTITUTE FOR

Regression added to `migrator_population_is_canon.rs` -- **that file and not a new one**, because a second home for _the migrator's population is canon_ is precisely how the two populations drifted apart. Then the union was disabled and the test re-run: **`left: {}` vs `right: {21, 22, 23}`.** The precondition assertion passed FIRST, so the store was genuinely populated and then emptied rather than never filled. Source restored and diffed byte-identical.

### THE REPORT WAS FIXED TOO, BECAUSE THE FIX WOULD OTHERWISE HAVE MADE IT LIE

The union makes `migrated: N issue(s)` count a population read back from committed canon, so on a re-run it would claim as CONVERTED what was merely re-emitted -- the exact disclosure threads already get one line down. **Fixing the loss and leaving the report asymmetric would rebuild this defect's shape in the layer 0070's own body calls the worst part.** `Plan` and `Upgraded` now carry `already_migrated_issues`, and the run prints a second line for it. **Separate line rather than a second clause**, because a v2 estate mid-conversion has already-migrated threads and no issue canon at all, and one sentence would make a zero on either side read as a statement about both.

### THE LOSS SURFACE IS CLOSED -- AND THE FIGURE SURVIVED A CHALLENGE MY DERIVATION DID NOT

`store.rebuild` deletes seven tables -- `tests`, `criteria`, `related`, `attachments`, `wps`, `threads`, `issues`. **Five of the seven are sub-populations of `Thread` and return with the threads; `issues` was the only top-level population without a union.** **dc confirmed that half independently at `store.rs:1591`, and it stands.** The five tables it does NOT delete -- `event_log`, `file_index`, `ingests`, `snapshots`, `doc_sections` -- cannot be lost by a rebuild, so there is no third instance of this asymmetry waiting.

**BUT I FIRST WROTE THAT I HAD CHECKED THE DELETE LIST AGAINST THE SCHEMA, AND THE SCHEMA HALF WAS A GREP WITH TWO SILENT DEFECTS.** `CREATE (VIRTUAL )?TABLE (IF NOT EXISTS )?[a-z_]+` **stops at a digit**, so `attachments_v11` and `attachments_v13` both collapsed into a phantom `attachments_v` that exists nowhere -- and the pattern swept **three DOC-COMMENT lines** in as if they were DDL. It reported 21 names. **It produced the RIGHT ANSWER FOR THE WRONG REASON**: the truncation only damaged the `_vN` family, which I then dropped from the count without saying I had dropped anything.

**THE RE-DERIVATION IS FROM `sqlite_master` ON THE LIVE STORE, WHICH IS NOT A READING OF THE SOURCE AT ALL.** 17 tables: 12 logical plus 5 FTS5 shadows of `doc_sections`. Seven deleted, five not. **Zero `_vN` tables exist at runtime** -- they are create-copy-drop-rename migration scaffolding, which is also why dc's competing figure of 14 was noise rather than a disagreement.

**dc THEN DERIVED IT A SECOND TIME, INDEPENDENTLY, BEFORE READING MY ARITHMETIC BACK (dc, live channel, 2026-08-24 16:56Z -- their stamp, attributed rather than asserted).** Same 17, same 12 logical, same 5 -- **plus a negative control I did not run: `_v` and `_v[0-9]*` both return ZERO at runtime.** So the figure now has TWO instruments behind it where the original had a broken one, and dc's 14 is closed from their end rather than by my explanation of it.

**THIS IS MY OWN PATTERN AGAIN AND IT IS THE THIRD SITTING IN A SAFETY ARGUMENT: THE MEASUREMENT IS NOT THE CLAIM.** The delete-list measurement was sound; the sentence describing how I had checked it was stronger than the instrument that produced it. **dc could not reproduce the figure and REFUSED TO CONTRADICT IT** on the ground that a probe with a proven false positive in its output has no standing to adjudicate the rest -- which is the correct move, and it is what sent me to the database instead of to a better regex. **A better regex would have agreed with me and taught me nothing.**

**AND NEITHER OF US HAS A MECHANISM FOR THE DISTANCE BETWEEN `checked against the schema` AND `checked against a grep of the source`** (dc's framing, and they are right that it is the whole distance). Nothing in this estate reads a sentence and asks what instrument stands behind it -- `at lint` checks rows against files, `doctor` checks views against canon, the roster check verifies a row names a file that exists **and never that the row DESCRIBES what its runner does** (dc found that in their own roster row today). **The gap is the same one in all three: a claim ABOUT an instrument is checked by a reader noticing.** Filed as a class rather than a task; neither of us proposed a mechanism, and inventing one to close a paragraph would be the defect again.

### A READING I NEARLY PUBLISHED AND CORRECTED FIRST -- THE FLATTERING VERSION WAS WRONG

I had it that `migrate_v2_project.rs`'s LIMB 4 was **one binding away** from catching this: it does `let (threads, _issues)` and its own doc says _"the population is the claim"_. **That is false, and the true version is worse.** `converted()`'s v2 estate holds no issues at all, so binding the name would have compared `[]` against `[]` and passed on every build ever made. **A true instrument, correctly aimed, at a population that cannot exhibit the failure** -- vc's dominant class from the same day, and the discarded binding is the symptom rather than the cause. Added LIMB 4b, which brings its own issue-bearing estate and covers the FIRST-RUN path specifically, since that is the path the fix could break.

**dc REPORTED AND DROVE IT; I LOCATED THE MECHANISM; dc THEN DROVE THE PREDICTION MY MECHANISM MAKES.** Neither the diagnosis nor the test shares an author with the other, which was the point of asking.

**THE MECHANISM -- `migrate.rs:329-333`, and the defect is an ASYMMETRY rather than the re-run.**

```
let seen: BTreeSet<String> = threads.iter().map(|t| t.id.clone()).collect();
let (extra, extra_ids) = dehydrated(project, &seen)?;
let mut threads = threads;
threads.extend(extra);                                 // threads topped up from COMMITTED CANON
let mut plan = assemble(project, ctx, threads, issues, carried)?;  // issues straight through
```

**THREADS GET A UNION AGAINST THEIR OWN COMMITTED CANON. ISSUES GET NOTHING.** On an already-migrated project `legacy::scan` finds no v2 estate, both come back empty, the union refills threads, nothing refills issues, `assemble` plans zero, and the commit replaces the store. **v2 short-circuits at target; v3 does not, so the re-run is the TRIGGER and the missing union is the DEFECT.**

**AND THE UNION'S OWN COMMENT STATES THE PRINCIPLE FOR BOTH POPULATIONS IN THE SAME BREATH:** _"THE UNION, AND IT IS WHY THIS VERB STOPPED SHRINKING THE ESTATE... a thread absent HERE is absent from the SSOT and from the index the migration leaves behind."_ **Substitute `issue` and every word stays true. The fix that stopped the verb shrinking the estate was applied to HALF the estate** -- and the half it missed is the half with no canon re-emit arm to make the loss visible.

**dc's DRIVEN SPLIT, WHICH COULD HAVE FALSIFIED THE MECHANISM AND DID NOT:**

| arm                                          | report                                | issues | verdict  |
| -------------------------------------------- | ------------------------------------- | ------ | -------- |
| v3 upgrade on a genuine UNMIGRATED v2 estate | `1 thread(s), 5 issue(s), 11 file(s)` | 5 -> 5 | SURVIVED |
| v3 upgrade on an ALREADY-MIGRATED project    | `1 thread(s), 0 issue(s), 6 file(s)`  | 5 -> 0 | DIED     |

**dc's SELF-CORRECTION IS THE PART THAT CHANGES THE FIX.** They first told vc and me that `0 issue(s)` means LEFT none rather than FOUND none. Too strong: **it means CARRIED none, and the number is HONEST -- it says 5 when it carries 5.** So **the output is a FAITHFUL REPORT OF A DESTRUCTIVE PLAN, which is worse than a wrong number: YOU CANNOT FIX THIS BY CHANGING THE MESSAGE.**

**RED-FIRST ARM, SPECIFIED BY THE SPLIT AND NOT BY EITHER OF US: BOTH ARMS REQUIRED.** With the issues union in place the ALREADY-MIGRATED case must report `5 issue(s)` and survive, **AND the unmigrated case must be UNCHANGED at 5** -- because a fix that makes the already-migrated path work by breaking the scan path passes a single-arm test. **And dc's constraint on the assertion: a regression test for 0070 that asserts via `sync`'s agreement report INHERITS 0069 -- read counts from the STORE directly, never `sync` saying AGREE.**

**0069 AND 0070 ARE TWO ROWS WITH A DEPENDENCY, NOT ONE** (dc's ruling, and it corrects my merge question): 0069 is a REPORTING defect in `sync`, 0070 a DESTROYER in `upgrade`. **0069's guard actually WORKED in the live case** -- `sync --to-disk` refused at rc=1 naming _the store holds no issues and the estate has 47_, **which is how any of this surfaced at all.**

**THE TELL PAIRING, dc's AND MINE, AND BOTH REST ON A REAL NUMBER WHICH IS WHY NEITHER GETS CHALLENGED:** **mine goes wrong at the WRITING** -- I drive the measurement and publish a stronger claim about what it measured; **dc's at the READING** -- they read the measurement as the claim it invites. dc did theirs twice in one hour on this same defect, one message after naming it.

## What changed under the tree on 2026-08-21 -- NOT today; read the dates

- **THE v2 CLI HAS LEFT THIS CHECKOUT.** `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` = main HEAD and **NOT the `v2.19.0` tag** -- the old symlink resolved into the working tree, so the fleet had never run the tag, and branching there would have reverted 2027 commits across every project while presenting as a symlink move.
- **CORRECTED 2026-08-24 -- `DO NOT PUT v3 ON PATH` WAS RETIRED BY ST0058 ON 2026-08-22 AND THIS LINE ASSERTED IT FOR TWO MORE DAYS.** **v3 IS on PATH as `intent3`** -- a DISTINCT name, so the fleet's `intent` gate is untouched by construction. `intent3` -> `bin/intent3` -> `target/release/intent`. **And dc's currency guard now REFUSES that release binary**, correctly: it is dirty AND its committed range touches crate source. Remedy is `int local build`. **`intent` on PATH is still v2.19.0 and answers for the FLEET, not for this tree.** Original line, now false in its second half: **`bin/` is no longer load-bearing for anyone else**, so v2 shell can be pruned here without breaking fifteen projects.
- **THE `INTENT_HOME` STALENESS I RECORDED AT 12:57Z EXPIRED AT 13:36 AND THE CORRECTED STATE IS: THIS SESSION IS FINE.** Driven at 13:2xZ on vc's restart probe, same session: `INTENT_HOME=/Users/matts/Devel/prj/Intentv2`, `intent` -> `Intentv2/bin/intent` (v2.19.0), siblings `intent_st` and `int` -> `Intent/bin` (v3). **That is the correct split and no measurement in this session is suspect.** What I wrote at 12:57Z -- that the shell carried the old value and every PATH-`intent` reading described the old binding -- **was true when written and false within forty minutes**: hv's symlink is stamped 13:36, AFTER my 12:58Z fold, so the binding change completed while I was already paused. **THE LESSON IS NOT ABOUT THIS VARIABLE. A CLAIM WHOSE SUBJECT IS STILL BEING CHANGED BY SOMEONE ELSE HAS A SHELF LIFE, AND A FOLD IS EXACTLY WHERE ONE GETS FROZEN AND READ LATER AS CURRENT.** Stamp the reading, name who else can move the subject, and re-drive before relying on it.
- **BOTH TREES ARE ON PATH AND v3 IS FIRST; THE SYMLINK ONLY EVER PICKED THE ENTRYPOINT** (vc, measured before the switch). `~/.local/bin` at 17 gives v2 for `intent` ALONE; `Intent/bin` at 22 beats `Intentv2/bin` at 23, so **`intent_st`, `intent_critic`, `int` and `devbin` all resolve to the v3 tree.** Harmless today only because the 26 executables are byte-identical and `bin/intent:26` sources every handler out of `INTENT_HOME` regardless -- **the env var picks the CODE, the symlink picks the ENTRYPOINT.** It arms itself the moment v3's `main` diverges. Fix is WP-12's _bin/ (shell) pruned at the cut_, mine, later. **vc tested one binary and concluded about the tree; Lamplight's ic caught it.**
- **THIS REPO'S COMMIT GUARDS NOW RESOLVE OUT OF THE FROZEN v2 CHECKOUT** (`.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`). Identical today; **drifting from the next guard change.** dc holds it as a mechanism -- hv declined direnv and hand-refresh by name.
