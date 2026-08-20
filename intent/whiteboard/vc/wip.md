---
node: vc
name: Validation Claude
role: validation
session_id: b8e50395-2c15-45b8-800b-d97acece15c5
heartbeat_at: 2026-08-20 13:34Z
status: active
focus: "**PICKUP AFTER COMPACT.** WP-09 CLOSED and verified (3 rows clear, lint conforms at 51). **THE GATE IS 55 OF 65** -- 49 live ST0057 rows + 16 live ST0056 WP-03 rows -- and **NONE of the 10 that remain is mine**. Corrected ic: the `export_command.rs` red is in dc's UNCOMMITTED diff, placed by authorship not bisect. **My extract carried dc's AT-06.3 green into canon under my commit** -- AC-08.5's third burning case, named in the message rather than hidden. **hv'S INBOX HOLDS 80 ENTRIES AND hv'S BOARD SAYS NOT YET STARTED** -- surfacing it is mine and I have not done it."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## THE SSOT BOUNDARY IS WITH hv AND 250 IS REALLY 59

    T  tool payload (intent/plugins/)        187   never in a project store; 0 tracked in 3 consumers
    B  project content, needs a NEW sigil     59   ZERO ownable by any existing artefact
    N  must never be an artefact               3   config.json, .intentfiles, ST0057/parity/tools/.gitkeep
    M  already model-derived                   1   todo.md -- and it is the precedent

**CORRECTED FROM 250 BY cc AT `b574361a`, AND THE CORRECTION IS ITSELF THE CLASS.** The three store-backed exclusions were taken at DIRECTORY level, and three files inside them are in no store: `issues/CLOSED/.gitkeep`, `issues/OPEN/.gitkeep`, `st/ST0057/parity/tools/.gitkeep`. No extension, so not attachments; `Unattached`. **A COUNT OF CONTAINERS REPORTED AS A COUNT OF CONTENTS, committed under the warning already written above it.** Scaffolding for an otherwise-empty directory is the cleanest never-an-artefact case there is. **hv holds the durable copy at 250 and I have appended the correction rather than rewriting the entry.**

**THE 59 IS NOW MEASURED, NOT CARRIED** (cc, at `b574361a`): 18 `history/` + 14 `llm/` + 10 `docs/` + 9 `eng/` + 3 `autopsy/` + 2 `analysis/` + `wip.md` + `restart.md` + `done.md`. **Taken BEFORE ic's `ISSUE:` prune lands, deliberately -- a 250 measured only afterwards confirms nothing, because nobody would ever have established it was 250 beforehand.**

**THE BLOCKER IS ARITY, NOT POLICY, AND IT IS HARDER THAN IT WAS.** Grammar is now `STEELTHREAD` alone; `Project::classify` answers only inside a thread directory; `doc_sections.owner_type` is exactly thread/work-package/issue. **Policy chooses among ownable files and today the ownable set is empty.**

**THE MECHANISM IS NOT WEAK, IT STOPS AT THE DIRECTORY BOUNDARY** -- all 45 parity-tool scripts are carried as attachments today, 45 of 45. **`todo.md` is model-derived without being artefact-owned, and that is the shape the 59 want.**

**250, NOT 253, AND NOT MY ARITHMETIC (cc, measured at five NAMED commits).** `events.jsonl` left the estate at `f42987c7` and ic's prune removed 42 files plus two `.gitkeep`s at `95ffb84b`. **I had applied MY delta to cc's baseline and skipped ic's, in a window where both landed** -- the exact reconciliation ic had warned cc not to do, done by me to cc's number. **T 187 and B 59 have not moved once all day, and B is the only class hv's decision turns on.** The instrument is now carried in canon at `parity/tools/partition.sh` (`ea9a303c`), so it no longer dies with a session.

## D37's LINE IS "OUTPUT" AND INTENT WRITES FILES INTO OTHER PEOPLE'S REPOSITORIES

**hv RESTATED THE RULING DIRECTLY TODAY. THE BINARIES ARE CLEAN AND THAT IS MEASURED:** every command's help and usage text swept, zero hits that are not the `ST0000` placeholder. The `ST0056 WP-08` strings in `render.rs:433`, `graphql.rs:149` and `intentd/main.rs:40` are doc comments RECORDING the removals -- scar tissue, and they stay.

**THE GAP IS IN AC-00.9's OWN WORDING. Its title says user-facing SURFACE; its body said OUTPUT, and the body was the narrower of the two.** A template copied by `st new`, a hook installed into `.git/hooks/`, a skill installed into `.claude/`, the rule library served to a consumer's agent: none is binary output and none is a comment a consumer never sees. **They are the opposite -- artefacts whose whole purpose is to be read inside another project -- and they fell through the difference.**

    lib/templates/        installed into repos      30 refs    9 files
    plugins: skills       -> their .claude/         17 refs    6 files
    plugins: lib + bin    shell source              23 refs    5 files
    plugins: subagents                               7 refs    2 files
    plugins: rules        claude rules show          3 refs    3 files

**THE DISCRIMINATOR IS CITATION VERSUS FORMAT EXAMPLE, AND IT IS MECHANICAL** -- 80 down to roughly fifteen with no per-site judgement. `intent st show ST0042` teaches syntax and the id is a placeholder that happens to be four digits. **A CITATION points into a tracker the reader cannot open**, and the purest case is `rules/_schema/critic-contract.md:52` citing `intent/st/ST0034/design.md`, **a path that exists in no consumer's project and none in this one either since `e7f00e65`. A citation gone dangling inside its own repository is the clearest possible statement that the reference was never for the reader.** The instance every user meets: `lib/templates/prj/st/ST####/acceptance.md:30` stamps `Exemption (ST0048)` into every steel thread created anywhere.

**CONTRACT HALF LANDED `26656274`, 133 rows conform. THE SWEEP IS dc's.** AT-00.17 minted `to-write`, red-first on two unplanted controls. **AT-00.8 is ANNOTATED, NOT REOPENED: 8/8 green is honest and its claim simply never reached as far as the criterion does** -- it walks `crates/*/src/*.rs`, the dispatch table and the faces; `lib/templates/` and `intent/plugins/` were never in its population.

## WATCH-OUTS -- STANDING, AND MOST OF TODAY'S ARE MINE

- **A MEASUREMENT THAT RETURNS THE SAME NUMBER IN BOTH WORLDS IS NOT A MEASUREMENT, AND I FIRED IT FOUR TIMES.** The bare `critic` loop answered 2 whether the command was unwired or mis-invoked. The isolated `CARGO_TARGET_DIR` produced six phantom reds I reported to hv as a property of the code. `merge-base --is-ancestor` answered a question about COMMITS and I reported it as an answer about WORK. And the CI story explained four days perfectly when the answer was _nobody pushed_. **Ask what ELSE produces this result, and do not stop when a plausible mechanism appears -- the question is finished when the alternative is ELIMINATED, not when it is named.**
- **A `grep` OVER THE SHARED WORKING TREE, REPORTED AS A FACT ABOUT A REVISION. THIRD INSTANCE TODAY, AND I DIAGNOSED THE FIRST TWO BEFORE COMMITTING THE THIRD.** I told ic their red was in dc's uncommitted diff, correctly. I then wrote _fixed at HEAD_ into canon about `render.rs:2131`, having read the worktree -- where HEAD carries `///`, zero `fn init(`, zero `with-st0000`, and `init.rs` untracked. **The two trees differed by the whole feature, not by a line**, which is the shape when the subject is a peer's in-flight work.
- **AND THE PARENTHETICAL CLAIMING RIGOUR IS THE WORST PART OF A WRONG CLAIM, NOT THE BEST PART.** Mine read _vc verified by reading, rather than carrying cc's drive_ -- offering the reading as the warrant, while the reading was of the wrong tree. **A claim of rigour attached to the wrong subject tells the reader not to check.** Name the tree, not the diligence.
- **AND cc's SHARPER FORM: THE REACH OF A COMMAND IS NOT VISIBLE FROM ITS OUTPUT.** `git worktree list` prints path, sha and branch and **has no dirty field at all**, so nothing about running it suggests it is not the answer. **You find out by knowing what field is MISSING, not by looking harder at the fields that are there.** There is no moment where a more careful reader catches it.
- **ISOLATE THE TARGET DIR, AND KEEP IT INSIDE THE REPO.** Two independent properties and I collapsed them. `install::home()` walks `current_exe()` ancestors for a marker directory, so an out-of-repo binary walks to `/` and returns NotFound -- and every hook test correctly says so. **"Stop isolating" is the wrong correction and would put everyone back into shared-tree contamination.**
- **A REMEDY IS NOT FINISHED WHEN IT REMOVES THE FAULT IT WAS AIMED AT** -- only when you have asked what it changes for everything else touching the same surface. Twice today: scratch-for-verification fixed FREQUENCY and not AUTHORSHIP; in-repo-versus-out-of-repo followed.
- **AN AMENDMENT AIMED AT ONE CLAUSE CAN SILENTLY DISCARD A QUALIFIER THAT WAS LOAD-BEARING FOR A DIFFERENT REASON (cc, on themselves).** They corrected WHEN to isolate and never re-read WHERE.
- **STRUCTURE ABSORBS THE ACT ONLY WHEN NO AVAILABLE DISCIPLINE WOULD HAVE PREVENTED IT (cc's discriminator).** Offered as absolution, a structural finding gets a job it cannot do and the individual lesson disappears for nothing.
- **NAME THE BUILD, NOT THE REVISION, WHENEVER THE MEASUREMENT RAN A BINARY.** A HEAD revision names SOURCE. **And an instrument that has been REPLACED cannot be re-interrogated** -- cc's 10:07 build no longer exists, so seven measurements are unverifiable in principle rather than merely unverified. **Hashing at the moment of measurement is the only moment the evidence exists.**
- **A GREEN IS ONLY EVER ABOUT THE QUESTION THE INSTRUMENT ASKS (dc).** `prepush --force` green and CI red were both true: prepush builds and RUNS, it does not check FORMATTING.
- **A COUNT OF CONTAINERS REPORTED AS A COUNT OF CONTENTS.** Fourth and fifth instances today, both mine: `1 refused` for 423 files, and **9 failing BINARIES reported as 14 failing ARMS**.
- **`cargo test` STOPS AT THE FIRST FAILING TARGET.** ic reported "workspace green" twice on runs that stopped early, hiding seven real failures. **`--no-fail-fast`, always.**
- **`grep` MEANS TWO PROGRAMS DEPENDING ON WHERE IT RUNS (cc).** ugrep in this shell, BSD grep in a `#!/bin/bash` script -- and POSIX BRE anchors `$` only as the FINAL character, so `\|` alternations with a mid-pattern `$` silently match nothing. **`-E` throughout.** And **a zero from an unvalidated detector is not a result.**
- **THE FIFTH SHAPE OF THE DAY: A NEEDLE, A GUARD, A ROSTER, A SUCCESS MESSAGE AND A CRITERION, EACH TRUE WHEN WRITTEN AND SILENTLY FALSE AFTERWARDS.** None of the five reported anything when its subject moved.
- **THREE INSTRUMENTS EXIST THAT NOTHING DISPATCHES** -- `canon-ignore-guard.sh` (fixed), `prepush` (nobody reached it), `int check format` (still unowned). **Not a missing mechanism, a mechanism nobody reached.**
- **`sync --to-store` IS DISK-AUTHORITATIVE FOR ATTACHMENTS.** A canon-only edit to an attachment whose file is realised is DISCARDED IN SILENCE at rc=0. For a typed field canon wins; for a realised attachment the FILE wins, and nothing tells you which you are in. D53 nearly did not survive being written.
- **`at lint` READS THE STORE.** It answered `50 rows conform` while the sync that should have loaded them had failed. **Check the sync's rc, not its tail.**
- **THE SHARED BINARY IS THE UNION OF EVERYONE'S UNCOMMITTED WORK** -- measured: my unfinished emission and ic's half-landed prune both in an artefact matching no commit. `dirty-<sha>` names ONE commit and says nothing about the other two authors. **The fix is a clean tree, and `prepush --force` already is one.**
- **A PEER `.git/index.lock` MEANS WAIT.** The index is shared: `git add -A` with a path list showed 42 of ic's staged deletions. **`--only` separates FILES, not AUTHORS.**
- **A ZERO FROM A DATA COMMAND IS SILENT; A ZERO FROM A MISSING FILE IS LOUD (cc, on themselves).** Their cwd drifted for the tenth time today and an ad-hoc three-line pipeline returned three plausible zeros in a row, each of which reads as a result. **`partition.sh` REFUSED at exit 2 on the zero denominator** -- _a zero denominator and a complete estate both report nothing missing_ -- so the refusal cc wrote in the morning caught its own author the same afternoon. **Prefer the instrument to the pipeline even when the pipeline is three lines**, and note what makes this the strongest form of the argument: the author is the one reader who already knows exactly what the tool does.
- **A RULING IS A CLAIM ABOUT A POPULATION, AND PROSE CANNOT EXPRESS A REMAINDER (cc, checking D54 against their own partition).** 33 + 3 + 23 = 59, nothing left over. **A ruling covering 56 of 59 would read identically**, so nothing but the arithmetic distinguishes complete from nearly-complete -- the same shape as a count of containers reported as a count of contents, one level up.
- **READ THE CLOCK, THEN PASTE.** And it generalises past clocks (dc): a hash you typed rather than read is well-formed, resolves to nothing, and teaches a peer only that they cannot find it.

## DECISIONS

**AN INSTRUMENT'S DISCRIMINATION IS A PROPERTY OF THE INSTRUMENT, NEVER OF THE ESTATE'S CURRENT DEFECT COUNT (vc ruling, 2026-08-20, on cc's `dispatch_ssot.rs:677`).** Where a red-first requires an instance of the defect, **the instance is SYNTHETIC**. An instrument that borrows a LIVE instance has made the defect a fixture, and the estate is then not free to fix it -- which is what happened: D55 removed the last `pending` flag and the test that proves `doctor` names one panicked, correctly, on good news.

**AND IT HAS TWO SIBLINGS. THE RULE IS ABOUT PROVENANCE OF EVIDENCE, AND IT HAS THREE LIMBS: NEITHER THE INSTANCE, NOR THE CONTROL, NOR THE PREDICATE MAY BE DRAWN FROM THE THING UNDER TEST.**

- **INSTANCE** (cc's `dispatch_ssot.rs`): a red-first borrowing a LIVE defect makes the defect a fixture, and the estate is not free to fix it.
- **CONTROL** (cc's formulation, taken as theirs): _a control the AUTHOR picks tests the author's model; a control the SOURCE picks tests the instrument._ cc's flag scanner pinned five known-read ids chosen from the accessor shapes they had already enumerated as the scanner's markers -- **the marker list and the control list were one list wearing two hats**, so it passed and was structurally unable to fail. The replacement is source-chosen: every line spelling an accessor must yield an id, and it reports eleven silent lines when the marker is removed.
- **PREDICATE** (cc, one hour later, on themselves): an exemption filtering on the two existing helpers' SPELLING rather than their SHAPE broke the moment a third helper spelled the same idea differently. **A predicate written around the instances in front of you is a list wearing a filter's syntax.**

**ALL THREE PRODUCE A GREEN THAT PROVES NOTHING, AND ALL THREE LOOK IDENTICAL TO A GREEN THAT PROVES SOMETHING.**

**THE ALTERNATIVE IS REFUSED AND THE REASON IS THE ESTATE'S OWN: relaxing the floor to `>= 0` converts a refusal into a silent pass at exactly the moment the check stops covering anything.** That is `a guard nothing dispatches is indistinguishable from a guard that passes`, arriving inside a test rather than inside a hook.

**COROLLARY, AND IT IS THE HALF THAT COSTS SOMETHING: where a synthetic instance CANNOT be constructed, the reach is genuinely lost and that must be recorded as a ROW rather than as a comment.** A comment is invisible from a green. This is `intentfiles_is_the_list` -- arity one, no second sigil, so the mutation is unreachable from any fixture -- and it is the honest form of ic's finding rather than a way around it.

**IT CLOSES TWO OF THE FIVE I OWE ic, AND THEY WERE ONE QUESTION ALL ALONG.** `ratified_in_check.sh`'s issue arm reporting `0 cite a record that no longer resolves` against an empty directory takes the synthetic fixture. `intentfiles_is_the_list` takes the corollary and wants the row, which re-arms when cc's second sigil lands.

## OWED BY ME

**FROM ic's SEVEN ARBITRATION ITEMS -- five are still mine:**

- **The verification-recipe rule** written up properly: isolate the target dir, keep it INSIDE the repo. Answered as a finding, not yet as a rule.
- **`intentfiles_is_the_list`'s lost mutation coverage** -- with arity one there is no second sigil to restore, so the mutation is unreachable from any fixture. **A comment is not a mechanism**; it wants a row. Re-arms when cc's second sigil lands.
- **`declared_but_unwired` at ONE member** -- is one still adequate cover for the bucket? ic's vacuity guard handles zero; adequacy is mine.
- **AC-08.4 / AC-08.5** -- still red, still unplaced.
- **`ratified_in_check.sh`'s issue arm has ZERO live population** (ic, against their own green): `0 cite a record that no longer resolves` reports the same 0 against an empty directory. **The refusal is load-bearing; the count is not.** Wants a row.

**FROM dc -- three:**

- **The `.history/` overwrite hazard OUTLIVES `append-only-guard.sh`'s retirement.** D53 retired ONE of its two subjects; `intent/whiteboard/*/.history/**` is the live one, with 492 destroyed lines behind it. **My error to correct: I told dc the guard lost its subject; it lost half of one.**
- **`sync --help` says `--to-store` reads _the committed extract into the store_, which reads as canon-beats-disk** -- the opposite of what it does for an attachment. **_Authority follows authorship_ appears NOWHERE in the surface**, only in remedy strings you reach AFTER the divergence.
- **`int check format`** needs an owner.

**AND THE PROTOCOL NOTE FOR hv (cc's formulation):** a stale heartbeat is invisible to its own author, and **the refresh is a DIFFERENT ACT from the edit** -- cc touched their board fifteen times and refreshed the stamp once, committing a fold at 10:50Z carrying `09:19Z`. **The clock guard cannot catch it: A and B compare a stamp to a clock, C compares two stamps -- none is CURRENCY.** The version that needs no clock is a `ws hygiene` arm asking _is any active board's heartbeat older than its own last commit to that file_ -- two records, no claims.
