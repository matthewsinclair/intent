---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 21:22Z
status: active
focus: "ic's evidence defect CLOSED at 74ae7324, and the cause was a layer below either of us: the model was DOCUMENTED as making empty evidence unconstructible, and a required String does not. Both fixes on offer would have left an EMPTY --evidence open. Found a second declared-and-unenforced guard beside it. 350 tests. NEXT: EXP-07's value field at 35 rows."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **Migrations are NORMAL, and the ladder now exists.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6). **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: snapshot = same-schema rollback ONLY; the recovery path for an outdated store is the EXTRACT. **D36**: `rm intent.db` is not an operation. **D37**: our ST/WP/AC ids never reach Intent's output, including the published schema faces.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

**You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either**. Asking SQLite and then writing the answer is still writing a time you obtained: the read and the write are two acts with a gap, and a write retried or deferred inside it is stamped when it was PREPARED. **The record is stamped BY the write.**

**hv's sharpening, verbatim and the most testable form of the rule:** _"intent3 won't have any cli or intentsvcs functions that TAKE a time. There will be cli and intentsvcs functions that RETURN times, but those will have gone end-to-end thru the db where the time was SET BY SQLite, not confected in an LLM hallucination."_ **That is a property of the API surface, not of the call sites** -- and it is a sharper guard than `one_clock.rs`, which bans `::now` and would not catch a time-shaped parameter.

**Four things that are NOT exceptions**, each already used by one of us to reintroduce a clock: a test fixture; "I'm only reading it"; "the value came FROM the database"; "it's just a board label".

**Creating vs restoring is the split that makes it workable**: create -> the DB stamps, no caller supplies anything; restore -> the recorded stamp is carried. **Re-stamping on restore or migration destroys history and every stamp still looks valid.**

**Why load-bearing**: under D34 two machines MERGE event logs. A merge needs a time nobody could have typed.

## DOING -- nothing; picking up next from TODO

## TODO -- in order

1. **EXP-07 / issue 0035 -- `Flag` drops four declared fields.** `required` (3 rows), `value` (35), `default` (6), `accepts` (4) never deserialize. **`value` at 35 is now the one that matters**: it renders the `<fmt>` placeholder in a usage line, so every value-taking flag may be showing clap's fallback instead of the authored one. **`required` got SMALLER, not done** -- with the facade guarding, clap enforcing it is belt-and-braces for an earlier message rather than the only thing standing there.
2. **Wire `intent ingest [PATH]`** -- ic landed the row at `3280b43d` on my ruling (`0..1`; recovery defaults to this project's tree, a migrator names another).
3. **AC-06.6 export**, then **AC-06.1 surface tail**. **AC-04.1's `TornRollback` arm.**
4. **D-numbers: vc RULED THEM IN (38, sweep all) on D37's contracted text, which says "decision number" explicitly.** The faces are swept and guarded. **My measured counter-example post-dates their ruling and is with them**: `D2-D11` in shipped help text is the READER's STZero deliverables, identical in shape to our `D15`, with no blessable value -- so the class is not machine-decidable in prose. Enforced in the faces, review-only elsewhere.

## Waiting

- **vc**: **one decision is theirs and I have deliberately not made it** (21:21Z) -- whether the WP-10 migrator should REFUSE a carried v2 satisfaction with blank evidence, or carry it with `doctor` reporting. It is where the ruled policy's "lossless-by-carrying" and "blocked-until-clean" pull opposite ways. Also with them: Machine 3 now declares `ac.satisfy` as `[NonTestOnly, EvidenceRecorded]` -- implemented ahead of ratification, reversible, and the transcription check holds me to whichever they rule. And the D-number prose exception.
- **ic**: **two error-message changes to re-pin in their surface check** (sent 21:21Z, no rows moved). Their `[PATH]` row is in and still waiting on my wiring -- item 2.
- **dc**: blocked on hv for the tap, offering capacity. Offers 1 and 3 are already done (AC-02.8 landed; `no_function_takes_a_time.rs` covers time-typed parameters). Taken instead: the `repo_root()` triplication needs a dev-dependency crate, which is a workspace change and theirs.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread** -- three consumers in ic's `gen_dispatch_table.sh`.
- **`doctor --fix` is WITHDRAWN, not deferred** (hv). A diagnostic that NAMES the remedy beats one that performs it. Nothing to remove in `render.rs` -- it was declared and never read; ic removes the row.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

- **NEVER PUT A `"` INSIDE A BOARD HEADER VALUE.** A formatter in the pre-commit chain treats the header as YAML: an embedded double quote makes the scalar invalid, so it re-quotes the line in single quotes and DOUBLES every apostrophe, which `ws list` then displays literally. **It landed in `ddd074af` and did not self-repair**, and the hook reported the file as formatted with no sign it had rewritten a value. One occurrence across five boards; the trigger is exactly the embedded quote. Reported to dc and vc 21:35Z.
- **A COMMENT ASSERTING A PROPERTY GETS CITED AS THE REASON NOT TO BUILD THE THING THAT WOULD MAKE IT TRUE.** The model said `Satisfied`'s evidence was "unconstructible without evidence" (hv-attributed, in the PUBLISHED face); `evidence: String` makes the FIELD mandatory, not the value present. **Three separate pieces of reasoning then rested on it and every one was correct given the premise** -- no guard was written, the renderer defaulted the flag, the gate destructured past it. **When a comment says a property is structural, go and look at the structure.** And the `///`-is-published rule has a second edge: the risk is not only that a private note ships, it is that a shipped note is BELIEVED by the next author.
- **TWO PROPOSED FIXES CAN BOTH LEAVE THE HOLE OPEN, and agreeing which is "the right one" is how you ship neither.** ic offered the renderer `?` or clap-enforced `required`; **both refuse an ABSENT flag and both accept `--evidence ""`.** "The flag is present" and "the value is there" are different questions and the second is the one that was asked. **Drive the empty case, not just the missing one.**
- **A TABLE THAT CANNOT EXPRESS A REQUIREMENT IS A TABLE NOTHING CAN CHECK AGAINST IT.** `Edge.guard` held ONE value; `ac.satisfy` needed two; the one that fitted got enforced and the other was never written down -- so the conformance test, the transcription check and the guard test all passed over a rule that had no place to be stated. **When a declaration looks complete, ask what it has no room to say.**
- **A TEST NAME IS A COVERAGE CLAIM, and reading the list is how it gets believed.** `withdraw_requires_a_reason_and_records_it` passed a reason and checked it reached the view. It never withheld one. **The REQUIRES half was asserted by the name**, so the missing guard looked covered from the test list for as long as anyone read the list instead of the body.
- **A LESSON WRITTEN DOWN IS NOT A LESSON APPLIED, AND THE SECOND INSTANCE CAN BE HOURS LATER IN A SIBLING GUARD.** `store_schema_version.rs` hashes the DDL with comments STRIPPED, and says why in its own doc: a guard that cries wolf on a comment gets re-pinned without reading. I wrote that, then built the face-version hash the same afternoon covering every byte -- so rewording a comment demanded three contract-version bumps. **The fix does not generalise itself; grep the siblings when you write the note.**
- **A REFERENT TRAP CAN LIVE IN A CLASS WITH NO BLESSABLE VALUE.** `ST0000` vs `ST0056` is soluble because one id is universal. `D2-D11` (STZero deliverables, the reader's) vs `D15` (a design decision, ours) is NOT: same shape, no carve-out. **When a class cannot be decided, decide the SURFACE instead** -- counted in the faces where the ambiguity cannot arise, review-only in prose. A guard that fires on correct help text gets switched off.
- **A FIX CAN BE INVISIBLE TO THE ENTIRE SUITE WHILE AN EXTERNAL SCRIPT CATCHES IT.** Removing the flag-disposition skip left all 339 Rust tests green; only ic's `surface_check.sh` noticed, and it is not in CI. **A property whose sole witness lives outside CI regresses on the next refactor.** Mutation-test asks "does anything catch this", and "something outside the suite" is the wrong answer.
- **`intent schema` PRINTS THE FACES, so the `///`-is-published rule reaches further than the repo.** It is not that a doc comment ends up in a committed artefact -- it ends up on a stranger's terminal. The comment exemption in a criterion about OUTPUT does not cover a comment a generator publishes.
- **A PEER PUSHING BETWEEN YOUR TWO PUSHES CAN EMPTY A HOOK'S RANGE.** `@{upstream}` names ONE remote; a hook computing `@{upstream}...HEAD` answers "unpushed to the tracked remote", not "in this push". With two remotes those diverge silently and the gate opens.
- **A GUARD CAN BE COARSER THAN THE DEFECT IT NAMES, AND THEN IT PASSES ON IT.** My `created_at` guard survived reverting `threads` to delete-and-reinsert, because both writes landed inside one second and second-granularity stamps compared equal. **The fix was the STAMP, not the assertion** -- the same collision is load-bearing in the product, since D34's cross-machine merge orders by exactly this value. Found by mutation test; unreachable by reading.
- **"DOES THIS THING HAVE ONE" IS THE WRONG QUESTION WHEN IT HAS SEVERAL.** The completeness check asked whether a table had A stamp with a DEFAULT, so stripping `file_index.created_at`'s left `updated_at` to answer for it and the check passed. **Report per column, never per container.**
- **A DEFECT THAT SURFACES SOMEWHERE ELSE IS A DEFECT THIS GUARD DOES NOT COVER.** That same mutation DID break the build -- through three unrelated snapshot tests hitting a NOT NULL violation. A loud failure elsewhere is not coverage here, and it reads like coverage.
- **A MIGRATION FIXTURE MUST BE A STORE THAT COULD ACTUALLY HAVE EXISTED.** The v1 fixture laid down `event_log` ALONE; it passed rung 2 and then met rung 3, which rebuilds seven tables it had never created. **The ladder growing a second rung exposed it, not any reasoning about the fixture.**
- **SQLite REFUSES `ADD COLUMN` FOR A NOT NULL COLUMN WITH A NON-CONSTANT DEFAULT.** Any DB-stamped column means a table REBUILD, not an ALTER. And **`PRAGMA foreign_keys` inside a transaction is a silent no-op** -- set it outside, or the guard looks applied and does nothing.
- **zsh EATS A BACKTICKED FRAGMENT IN A `-m` COMMIT MESSAGE.** `SELECT strftime(` vanished from a commit body and left "the needle set gained :". **Use `-F -` with a heredoc for any message containing backticks or parens.** `--only` protects a commit and NOT an `--amend`, so check `git diff --cached` is EMPTY before amending to repair one.
- **BETTER PROVENANCE IS NOT THE ABSENCE OF A CONFECTION.** I built `Store::now()` to collapse three process clocks, and it was the same defect one layer up. **Three of us landed on "one well-sourced clock" independently, which means the wrong shape is the intuitive one** -- so the enforcement must be structural, never a rule to remember.
- **A SUFFICIENT-LOOKING FIELD ANSWERS A NARROWER QUESTION THAN THE ONE BEING ASKED.** Eight tables shipped with no record timestamp because three columns look like one. **Fifth instance: a required `evidence: String` answers "is there a field", where the ruling asked "is there evidence".** The type is always the narrower answer; say which question you needed.
- **A SWEEP DOES NOT MOVE A FILE, IT SPLITS A CHANGE.** ic's unqualified `--amend` took my test file without the `store.rs` methods it called; HEAD did not build and every file in it looked finished. **After a sweep the check is "does it still build", not "whose file is this".** `--only` protects a commit and NOT an amend.
- **STAGE NOTHING UNTIL THE MOMENT YOU COMMIT.** Staging early to get past a block is what made my file sweepable.
- **`MM` WITH A CLEAN `git diff HEAD` IS A STALE INDEX ENTRY** (issue 0028) -- invisible to the diff, visible only as the left column of `git status --short`. `git reset -- <path>` clears it and touches no byte of the worktree.
- **A DISCARDED `ArgMatches` DROPS EVERY FLAG SILENTLY.** `Some(("doctor", _))` -- clap accepts a declared flag whether or not anything reads it, so `--help` advertises what the renderer denies.
- **A CENSUS BY WHOLE-FILE GREP UNDER-REPORTS.** Missed `st new -s` because its long spelling is `start`. **Check per-ARM.**
- **A FIXTURE CAN PUT A TEST IN THE WRONG WORLD.** `no_match_is_exit_zero_and_silent` used a bare `st new`, so its index was empty: it proved "never searched anything" while claiming "searched and found nothing", and passed either way.
- **A LANE BOUNDARY YOU ASSERT CAN BE BACKWARDS** -- worse than a stale premise, because it moves your work onto someone else's list. **Before naming a peer's block, run the query that proves it.**
- **A TEST THAT ASSERTS A REFUSAL CANNOT TELL WHICH REFUSAL.** `unwired` refuses too.
- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face and async-graphql into the SDL. **Plain `//` for reasoning.**
- **A TEST CAN ASSERT THE DEFECT, and it looks like diligence.** When a ruling lands, grep the tests for what now asserts the old behaviour.
- **A HAND-KEPT ROSTER INSIDE AN INSTRUMENT IS THE DEFECT THE INSTRUMENT LOOKS FOR, AND IT CAN BE WRONG IN EXACTLY THE PLACE ITS SUBJECT IS WRONG.** The blank-reason guard test enumerated `Thread` and `WorkPackage` by hand -- and `Criterion` was both the one entity whose declared guards nothing enforced AND the one entity the check did not visit. Green, twice over, on two live defects. **Discover structurally; never enumerate by hand.**
- **`git checkout -- <path>` REVERTS TO HEAD, NOT TO BEFORE YOUR MUTATION.** Back up with `cp`, restore with `cp`.
- **AN ERROR SWALLOWED IN A FIXTURE IS A SILENT ERROR.** `expect()` in fixtures, always.
- **`IF NOT EXISTS` MAKES A SCHEMA CHANGE INVISIBLE UNTIL A QUERY FAILS.** Any DDL change bumps `SCHEMA_VERSION`, re-pins `store_schema_version.rs`, and writes the migration rung -- in the same commit. **The guard earned its existence within hours: it forced the first rung on the first schema change after it was built.**
- **VERSION 0 IS NEVER "SCHEMA ZERO".** SQLite defaults `user_version` to 0, so 0 permanently means unstamped.
- **SCHEMARS RENDERS A DOCUMENTED ENUM AS `oneOf` OF `const`s AND AN UNDOCUMENTED ONE AS A FLAT `enum`.** A reader that knows one shape returns an EMPTY roster the day someone edits a comment, and every test over it passes vacuously.
- **`--only` COMMITS WHAT YOU NAME, and a move is TWO facts.** Verify at HEAD (`git ls-tree`), never on disk.
- **TWO symlinks point INTO this repo.** Sacrificial `git worktree` only for `bin/intent*`. **`git stash` is unsafe here.**
- **Cargo runs from `native/rust`.** `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift` is what re-blesses the faces -- not a workspace-wide bless.
- **Two remotes; a peer can push between your two pushes.** Verify both with `git ls-remote`.
