---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-17 10:59Z
status: paused
focus: "PAUSED after a localfold at 10:59Z. **hv RULED ALL THREE OPEN QUESTIONS (`c634c7cb`) AND I AM UNBLOCKED ON EVERYTHING I WAS STANDING ON.** Landed and pushed: AC-04.6's second condition MEASURED (all four `Unbuilt` rows hold canon-authored values nothing can move -- vc was right and my argument against measuring it was wrong); issue 0047 (every status spelling now has a witness that fails on a rename); the `info` exit-code guard's third arm; and **the SELF-LOOP RULING IMPLEMENTED across all four machines** at `61069b16`. Five tests were asserting the retired behaviour and THREE were passing for a reason other than the one their name gives. **A Highlander defect fell out of it: `Guard::GatePass` was declared in the table and enforced BY HAND, so deleting the declaration changed nothing -- in the file that implements AC-04.6.** **MY MID-REFACTOR TREE WAS MEASURED BY hv's OWN SUITE AND READ AS A REGRESSION**; all three peers diagnosed it independently and both committed trees were green. Suite green, fmt clean, clippy silent, lane clean, HEAD == local/main. **BOUNCE: Machine 4 -- `issues add|close|open` -- the block is LIFTED.** Then the two paired `kind` conversions, then `Thread.acceptance` off `Unbuilt`. v3 stays off PATH; push to local only."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6). **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: snapshot = same-schema rollback ONLY. **D36**: `rm intent.db` is not an operation. **D37**: our ST/WP/AC ids never reach Intent's output, including the published schema faces.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

**You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either**. Asking SQLite and then writing the answer is still writing a time you obtained. **The record is stamped BY the write.**

**hv's sharpening, verbatim:** _"intent3 won't have any cli or intentsvcs functions that TAKE a time. There will be cli and intentsvcs functions that RETURN times, but those will have gone end-to-end thru the db where the time was SET BY SQLite, not confected in an LLM hallucination."_ **That is a property of the API surface, not of the call sites.**

**Four things that are NOT exceptions**: a test fixture; "I'm only reading it"; "the value came FROM the database"; "it's just a board label".

**Creating vs restoring is the split that makes it workable**: create -> the DB stamps; restore -> the recorded stamp is carried. **Re-stamping on restore or migration destroys history and every stamp still looks valid.**

## THE SELF-LOOP RULE -- hv ratified 2026-08-17, implemented at `61069b16`

**A self-loop is legal, accepted and reported at exit 0, and it does NOT re-run the guard.** Asking a verb for the state an entity is already in is not a movement, so it is not a transition to declare and not an illegal one to refuse.

- **WHAT DECIDES A SELF-LOOP IS WHETHER THE CURRENT STATE EQUALS THE VERB'S TARGET** -- not whether the verb is declared from the current state. `st.resume` targets `wip`, so asking it of a `wip` thread is a no-op, not an illegal transition. This is the reading that makes 0046's distinction operational.
- **"Without re-running the guard" is structural.** A self-loop must not be able to fail for a reason that did not exist when the state was entered. `wp done` on a done package returns 0 **even where the gate would now BLOCK**, and that is the point rather than a side effect.
- **`AlreadyThere` is a NO-OP, not a repeated write.** An envelope for a non-movement would stamp a second `st.done` at a second time, and under D42 the record is stamped BY the write -- so history would show a unit closed twice.
- **It does NOT license an undeclared edge.** Same state with a different payload still falls through to the machine and is refused, so the ruling opens no reported-success-with-no-effect path.
- **A self-loop is not a transition, so no walk may enumerate `from == edge.to`.** `Edge::leaves` already said so; two walks were filtering on `accepts`.

## DOING -- nothing in flight; folded, committed and pushed

## TODO -- in order

1. **MACHINE 4: `issues add` / `close` / `open`. THE BLOCK IS LIFTED** (hv, `c634c7cb`). States `Open` | `Closed`, entry `Open`, **no guards deliberately** -- v2 has none, the row is `keep`, and inventing one is a parity break wearing a ratification. Self-loops per the rule above, which is exactly v2's `already CLOSED`. The dispatch rows already exist marked unbuilt, so this needs no table move from ic. **`issues_surface.rs::the_mutating_verbs_report_themselves_unbuilt_until_a_machine_is_ratified` is what to delete FIRST, deliberately, in the change that wires them.** **A NEW MUTATING VERB MUST DECLARE `recoverability`** (ic) or `check_vocabularies` refuses at binary load.
2. **THE TWO PAIRED `kind` CONVERSIONS.** `Criterion.kind` folds into Machine 3 as a `(kind, state)` pair; `AcceptanceTest.kind` folds into the **`AcceptanceTest.status`** machine -- **each `kind` folds into the machine of the entity that OWNS it**, which is vc's correction to their own shorthand and would have put a field in the wrong machine if transcribed literally. **Flipping `kind` alone is schema-INVALID** (the pairing is enforced at `model.rs:414-432`, held by `ac_kind_state_invariant.rs`), so the verb moves two fields in one act. ic has no notation for a multi-field atomic move and is recording it as a constraint on the verb rather than inventing a row -- so the spelling needs them.
3. **`Thread.acceptance` OFF `Unbuilt`: immutable after creation, no machine, no edge, owed nothing** (hv). It is `Option<AcceptanceMode>` -- an attribute, not a lifecycle -- so changing it is authoring. Needs a third `Disposition` variant; `Entry::Authored` stays the right measurement for it either way, because it measures a property rather than a classification.
4. **`Table` GETS FOUR DERIVES, AND I TOOK IT FROM ic.** `Serialize` on `Table`, `Family`, `Invariant`, `Vocab` so the key-coverage arms can introspect. ic MEASURED the premise: 24 top-level keys, 10 read, 14 stable structural blocks -- nothing like `Target`'s 40-against-2, so the exemption's premise does not hold at the top level. Six keys are silent AND unwitnessed (`measured_at`, `invariants`, `target_states`, `entry_dispositions`, `flag_dispositions`, `recoverability_values`); deleting `flag_dispositions` reports 95 flags undeclared, **and a wall of offending rows sends its reader to fix the DATA when the fault is one missing key.** My crate, my model change.
5. **AC-06.1's surface tail** (NOT the installer/canon block).
6. **WP-10 PHASE B** -- fixtures and a sacrificial copy only. **Not against this estate, and v3 does not go on PATH.**
7. **WP-03 and WP-05 pass their gates and are still `WIP`** (vc measured). Whether the work is done is mine to say; the contract already says yes.

## Operational -- carry into the next session

- **AC-04.6's SECOND CONDITION IS MEASURED AND THE REMAINING WORK IS IMPLEMENTATION WITH ZERO ADJUDICATION LEFT.** All four `Unbuilt` rows hold values authored canon put there with no verb to move them -- driven through the facade, not reasoned about. `Disposition::Unbuilt` carries `Entry` now, bound in BOTH directions, and a fifth such field with no measuring arm panics. **Measuring the debt is not paying it and the row stays red** -- TODO 1-3 are the payment.
- **ISSUE 0033 HAS A FALSE PREMISE ON MY BOARD AND I HAVE STOPPED CARRYING IT AS MINE.** `at_set` in v3 touches ONLY `status`; the note is a reified field, so **v3 cannot destroy it by construction.** The destruction is v2's row-rewriting, `bin/intent*` is FROZEN, and v2 is default-defer -- so the "commit first" ritual is retired by v3 SHIPPING, not by a fix I can write. **Keep committing before `intent at` while v2 is the tool in hand.** vc avoids it by editing the markdown directly when a status is not changing, which costs nothing.
- **0042 IS dc's AND CLOSED.** They branch on the code plus `[ ! -d ]`, never on the `<not set>` token, and the whiteboard guards deliberately do NOT gate on `info`'s exit code -- **so improving that code cannot disable them.**
- **`intent todo done` PRINTS `ok: <spec> was already done` ON A NO-OP.** New output on a shipped row; dc and ic both told.
- **v3 STAYS OFF PATH.** The door is publication, not migration. **0036 is the only publication hold left.**
- **PUSH TO `local` ONLY.** upstream frozen (hv, CI/CD budget). Compare `local` to HEAD; the remotes diverging is expected. **CI is no longer the watcher for the Linux leg** -- a `set -e` or path-separator break that only shows on Linux now has nothing checking it. That is the class that shipped v2.11.12 broken.

## Watch-outs -- grouped by MECHANISM, because a 65-bullet list stopped being read

### The one that keeps recurring: THE CLAIM IS WIDER THAN THE MEASUREMENT

- **FIVE INSTANCES ACROSS TWO DAYS, IN FIVE MATERIALS.** The measurement is sound every time and **the sentence reporting it is wider**, which is where it can be caught, because the tell is in the reporting step. AC-04.6 (two clauses, I measured one -- caught by vc); 0046 (a classification standing in for behaviour); TornRollback (four constructions enumerated, all of one producer); **`project_state_never_reaches_the_exit_code` named all project state and measured two arms of three -- found by reading the arms of the FUNCTION rather than the arms of the test**; and ic's `Table` exemption, granted on a premise they could have grepped. **Ask what your checks have in common before believing they are independent, and ask which of the producers you enumerated.**
- **AN ASSERTION THAT GOES RED WHEN THE PROPERTY IT NAMES GETS BETTER IS ASSERTING THE WRONG THING** (ic, and it is the best form of this I have). It subsumes "a test measuring the MECHANISM of a refusal fails when the mechanism improves": mine names the symptom, ic's says how to recognise it before it happens, from the direction of the change.
- **A GUARD'S PRECONDITION CAN BE INVALIDATED BY A CHANGE THAT IS CORRECT ON ITS OWN TERMS AND NEVER LOOKS AT THE GUARD -- AND IT RUNS BOTH WAYS.** My `info` rendering broke dc's emptiness test; their fix then falsified a sentence in MY `exit_code_consumers.rs`, within a day, with nothing watching either. **The level under it: parsing another program's human-facing output makes its RENDERING an API nobody declared, and rendering is the layer we change most freely and document least.** The remedy is not vigilance -- it is describing your own GUARANTEE rather than the consumer's internals, because only the first is something your file can check.
- **AN EXEMPTION GRANTED FOR ONE REASON GETS APPLIED TO EVERY REASON**, and the person most likely to extend it is the one who just re-derived it for the neighbouring type (ic, on themselves). Split an exemption into the jobs it is silently doing. **Exempting a type from totality can TIGHTEN what it may read**, because the note list is what makes "read but deliberately unclassified" expressible.
- **REDUNDANT WITNESSES: ONE STOPS SILENTLY AND THE OTHER MASKS IT.** Stubbing half my status-line classifier left all six tests green, because a status renders in TWO places and either alone satisfies every presence check. **0047's own shape, inside the fix for 0047, found by mutation and not by review.** Require each source separately.
- **EXTRACTING A SHARED PRIMITIVE CAN LEAVE IT UNWATCHED BY EVERY EXISTING CONSUMER AT ONCE, when they all assert absence.** Stubbing `exitless` to empty reds my arm and leaves `no_state_can_be_entered_and_not_left` GREEN, because that one asserts a set is empty. The only assertion requiring the computation to FIND something is the canary for the whole primitive.

### Prose that is a build input, or a claim nothing checks

- **A `///` DOC COMMENT IS SHIPPED OUTPUT AND CONSUMER-FACING PROSE.** schemars lifts it into the JSON Schema face, async-graphql into the SDL, and both are COMMITTED and drift-checked. **Plain `//` for reasoning.** A rustdoc intra-doc link reaches a published GraphQL schema where it names nothing.
- **A COMMENT ASSERTING A RELATIONSHIP BETWEEN TWO DOCUMENTS IS THE ONE CLAIM NO COMPILER AND NO TEST IS LOOKING AT**, because the act that changes the subject is not the act that revisits the sentence. **Replacing one count with another restarts the clock.**
- **A TEST NAME IS A COVERAGE CLAIM, and reading the list is how it gets believed.** A guard that names a universal property and covers the two easy instances is worse than one that names its scope.
- **A ROW OR A COMMENT CITING A LINE NUMBER IS A PROMISE THAT THE LINE EXISTS** (ic). Going to the source is necessary and does not tell you WHICH source you reached: `git blame` plus `git show HEAD:` is the two-second version.

### Greps, shells and measurement

- **A GREPPABLE PROXY THAT CANNOT SEPARATE CODE FROM TEXT ABOUT CODE FAILS TOWARD THE CLEAN-LOOKING ANSWER.** `contains("rm ")` matching "form"; a retarget guard flagging `grep` patterns as invocations; **a `Completed` COLUMN HEADER reported as a thread's status, and an `acceptance.md` legend sentence reported as a WP's**. A grep for `wb_info_rc` matches the COMMENT naming it, so it passes identically before and after the fix -- **a decorative guard wearing shell.** **The fix is a better discriminator or an authored classification -- never an exemption**, and the discriminator then needs its own two-sided canary.
- **THE SHELL'S ANSWER IS NOT THE ONE YOU THINK, and the Bash tool runs zsh 5.9.** `$?` after a pipe is the last command's. `grep -c` exits 1 on zero. **zsh does NOT word-split an unquoted `$c`** -- use `${=c}`. **`path` is tied to `PATH`**, so `read -r want path` destroys the search path and every subsequent tool is not found -- **a broken instrument reporting maximum alarm** (dc). **A wrong zero certifies absence; a wrong maximum certifies catastrophe, and the second is more persuasive because it looks like diligence rewarded.** The exposure is INLINE only -- every parity tool has a bash shebang. **Prefer a script with a shebang for anything whose result you intend to write down.**
- **`cd` PERSISTS BETWEEN Bash CALLS AND THIS PROJECT HAS TWO ROOTS.** Cargo runs from `native/rust`, the repo from one level up, and a bare relative path is ambiguous across them. Absolute paths for reads. `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift` re-pins the faces.
- **A STALE ARTEFACT IS A SNAPSHOT, AND A FAILURE REPORT NAMES A REVISION.** **A CLAIM ABOUT A POINTER IS A CLAIM ABOUT WHATEVER IT POINTS AT WHEN READ, NOT WHEN WRITTEN** (vc, issue 0049) -- `HEAD` is a pointer and so is `rust`. `FAILED: rust` is what sent hv hunting a regression that did not exist. **A record names the commit the measurement TOOK, never the one its author read** (dc, correcting themselves and then PROVING the drift harmless with an empty `git log A..B -- <files>`).
- **A PARSER REPORTS ITS OWN LIMITS AS THE ESTATE'S DEFECTS, and it sounds authoritative.** **ABSENT IS NOT INVALID.** A vocabulary is what the tool ACCEPTS, not what it prints.

### Git in a four-node clone

- **AN UNCOMMITTED TREE IN A SHARED CLONE WILL BE MEASURED AND READ AS SHIPPED BEHAVIOUR, INCLUDING BY hv's OWN SUITE.** Mine was, twice in nine minutes; ic wrote my worktree into the register as shipped and had to retract; the tree that was compiled had stopped existing 41 seconds after the leg finished. **The fix is not vigilance -- it is committing at GREEN rather than at "finished".** A file has no author in this clone, exactly like a build artefact and an untracked file.
- **`git checkout -- <path>` REVERTS TO THE INDEX**, so when your baseline is UNCOMMITTED work the revert destroys the thing you were testing. Snapshot with `cp`, restore with `cp`, and `diff -q` both back afterwards.
- **`--only` COMMITS WHAT YOU NAME, and a hand-typed pathspec is a roster that is wrong where its author was distracted.** Check `git status` for leftovers IN YOUR OWN LANE afterwards; `git diff --cached` only says what did go in. **`--only` cannot reach an untracked file** -- `git add` it in the same command. A move is TWO facts; verify at HEAD with `git ls-tree`.
- **STAGE NOTHING UNTIL THE MOMENT YOU COMMIT.** `git stash` is unsafe here; sacrificial `git worktree` only for `bin/intent*`. **A peer measuring HEAD must use a detached worktree with its OWN target dir** -- otherwise it contends for `target/` and corrupts the result it was protecting.
- **A FILE MATCHING HEAD IS NOT PROOF YOUR WRITE WAS LOST -- THE RECIPIENT MAY HAVE CONSUMED IT.** In a single-writer protocol the reader owns the lifecycle; check the peer's commits first.
- **NEVER PUT A `"` INSIDE A BOARD HEADER VALUE** -- measured trigger, cause unknown.

### Mechanics worth keeping

- **A surface built from a table cannot be unbuilt from the renderer**: the TABLE row moves first, the renderer second.
- **A DECLARED GUARD ENFORCED BY HAND AT THE CALL SITE IS A DECORATIVE DECLARATION.** `Guard::GatePass` was in the table and hand-run in two verbs, so deleting it from the table changed nothing -- **in the file that implements AC-04.6.** Enforcing from the declaration also fixed an ORDERING nobody would have had to remember.
- **SQLite refuses `ADD COLUMN` for a NOT NULL column with a non-constant default**, so any DB-stamped column means a table rebuild. **`IF NOT EXISTS` makes a schema change invisible until a query fails** -- any DDL change bumps `SCHEMA_VERSION`. **Version 0 is never "schema zero".**
- **A fallback value can stand in for two different facts, and the second is the one nobody ruled on.** **A migration fixture must be a store that could actually have existed.** **An error swallowed in a fixture is a silent error** -- `expect()` always.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN.** `bin/int` + `bin/.devbin/**` are dc's -- including issue 0049's run-verdict stamping, which I support and am not writing.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread** -- three consumers in ic's `gen_dispatch_table.sh`.
- **`doctor --fix` is WITHDRAWN, not deferred** (hv). A diagnostic that NAMES the remedy beats one that performs it.
- **`install::MARKER` IS A SENTINEL, NOT A BOUNDARY** (cc ruled, on dc's packaging ask). It says nothing about what an install CONTAINS, so the rule library does NOT move under it. **Owed with `critic`: a `rules_dir(home)` accessor beside `hook_script`**, plus a sibling of `every_declared_hook_ships_as_a_script`.
- **`Outcome` is deliberately NOT `#[must_use]`.** It was, and it fired on 65 sites, nearly all tests putting a fixture into a state where ignoring it is right. **65 `let _ =` annotations added to silence a warning is how an annotation stops carrying information.** Where the outcome must be reported is the CLI, and that is held behaviourally.
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** A guard that had downgraded my red would have hidden a genuine finding: two of the five tests I had to change were asserting the behaviour hv's ruling retires.
