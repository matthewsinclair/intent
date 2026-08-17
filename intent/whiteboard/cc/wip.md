---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-17 09:35Z
status: paused
focus: "PAUSED after a localfold at 09:35Z. **dc's message landed DURING the fold (third consecutive time reading before archiving has paid) -- the packaging hold is CLOSED and I ruled their rule-library ask: `MARKER` is a sentinel, not a boundary, so nothing moves.** **FIFTEEN LANDED AND PUSHED**; suite 473 rust green, clippy+fmt clean, lane clean, all four inboxes at the sentinel. Correctness: `intent info` exits non-zero when it cannot resolve its install (vc VERIFIED); `todo.window_hours` refuses what the data cannot honour with a MEASURED self-retirement; 0044's structural half (retired commands refused BY NAME at 2) plus its consumer list as a CHECK; verb-slot arity to ONE home; vc's 0045 guarded end to end; the marked-legacy form's other instance; one `Remedy` trait over eleven error types; `TornRollback` proven REACHABLE (closes AT-04.1's gap); `Arg.default` literal-vs-described. Feature: **`intent issues` READ half ships.** **THE FOLD'S FINDING: TWO OF TODAY'S THREE SELF-INFLICTED TRAPS WERE ALREADY ON THIS BOARD VERBATIM** -- the `git checkout` one told me to back up with `cp` and I did neither, and the `///`-is-shipped-output one is exactly what ic had to tell me. So the 65-bullet watch-out list has stopped changing behaviour at the margin and is CONSOLIDATED into families here, full text in `.history/20260817/wip.md`. **BOUNCE: `issues add|close|open` are BLOCKED on a ratified Issue machine** -- asked of vc, including that v2's `issues close` is IDEMPOTENT where Machines 1-3 refuse. Then AC-06.1's surface tail, then WP-10 Phase B. **COMMIT BEFORE ANY intent at STATUS CHANGE (0033).** v3 stays off PATH; push to local only."
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

## DOING -- nothing in flight; folded and paused

## TODO -- in order

1. **`issues add` / `close` / `open` -- BLOCKED ON A RATIFICATION, ASKED OF vc, AND DO NOT BUILD IT ROUND.** `transitions.rs` declares `Issue.status` as `Disposition::Unbuilt`; `data-model.md` ratifies Machines 1-3 (thread, work package, criterion) and **no issue machine.** AC-04.6 forbids an undeclared edge, so wiring `close`/`open` means declaring `open <-> closed` on my own authority -- in the same change that would make the addition unobservable. **The edges look obvious, which is exactly when the discipline earns its keep.**
   **The half that actually needs a ruling: v2's `issues close` on an already-closed issue returns 0 with `already CLOSED`, where v3 refuses the analogous `st done` as an illegal transition.** Those cannot both be right and it is not a renderer's call. `issues_surface.rs` holds the three at `unwired` and says in as many words that it is the first thing to delete in whatever change wires them.
   **Also deliberately NOT done: the `Unbuilt` note is now slightly false** (the family is no longer wholly unported). Editing a machine's declaration while asking for it to be ratified is the wrong order; it gets corrected in the same change that adds the edges.
2. **0044's REMAINING HALF, and it needs hv rather than me.** The retired class is answered; `1` still means four things -- missing subcommand, missing argument, genuine runtime refusal, and (when built) findings. **vc's 0045 tables are the binding constraint**: git pre-commit blocks on 1, `UserPromptSubmit` blocks on 2, so **every non-zero code blocks exactly one of the two consumers and a third value blocks neither.** Not a free choice of number.
3. **AC-06.1's surface tail** (NOT the installer/canon block).
4. **WP-10 PHASE B** -- fixtures and a sacrificial copy only. **Not against this estate, and v3 does not go on PATH.**
5. **`intent at` must stop destroying the row's note** (issue 0033). It is v3 and therefore mine, and it is the only fix that retires the standing "commit first" ritual.
6. **WP-03 and WP-05 pass their gates and are still `WIP`** (vc measured). Whether the work is done is mine to say; the contract already says yes.

## Operational -- carry into the next session

- **COMMIT BEFORE ANY `intent at` STATUS CHANGE.** Issue 0033: `at red|green|na` DESTROYS the row's note and `at lint` reports `ok` immediately after, so **the contract's own linter cannot see it leave.** 14,253 characters across 34 noted rows; `AT-10.9` alone carries 3,993. `to-write -> green` is refused, so recording a passing test costs TWO rewrites. A committed note is recoverable with `git show`; one written and moved in the same session is gone.
- **A NEW MUTATING VERB MUST DECLARE `recoverability`** (ic): `reversible` / `idempotent` / `one-way`, or `check_vocabularies` refuses at binary load and the whole workspace reds at once.
- **TWO PUBLICATION HOLDS, BOTH dc's, AND I RAISED THE SECOND BEFORE IT BIT.** 0036 (brew shadowing v2's symlinks) and the PACKAGING hold: the formula stages `lib/templates/` nowhere, so on a published build the walk from `current_exe()` terminates without a marker and all three session hooks silently stop. **My `info` fix makes that loud rather than silent** -- it now exits non-zero instead of resolving to an empty path.
- **ONE ITEM IS OWED TO ME BY NOBODY AND SITS IN dc's LANE.** vc measured that dc's 0042 fix distinguishes total failure from a missing guard by testing whether the resolution came back EMPTY -- and `info` now prints `<not set>`, which is non-empty, **so dc's total-failure branch is unreachable in the one condition it was built for.** Guards silently do not run on a published build. Told dc I will take it if they want it; it is their file and they are paused.
- **v3 STAYS OFF PATH.** The door is publication, not migration.
- **PUSH TO `local` ONLY.** upstream frozen (hv, CI/CD budget). Compare `local` to HEAD; the remotes diverging is expected.
- **AT-10.9 and AT-04.1 both have their stated gaps closed and both statuses are vc's to set**, from a clean tree. My tree is clean and pushed.

## Watch-outs -- CONSOLIDATED, and the consolidation is the finding

**Two of today's three self-inflicted traps were already on this board, verbatim.** `git checkout -- <path>` had a bullet telling me to back up with `cp` and restore with `cp`; I did neither and destroyed my own uncommitted `info` fix mid-mutation. `A `///` DOC COMMENT IS SHIPPED OUTPUT` had a bullet naming schemars and async-graphql by name; ic still had to tell me the SDL face had gone stale. **A list of 65 bullets is not read, so it stops changing behaviour at the margin** -- and the honest response is fewer, grouped by MECHANISM, with the individual instances in `.history/20260817/wip.md`. Adding a 66th would have been the ritual rather than the fix.

### The one that keeps recurring: I CHECKED HOW THIS CAN FAIL

- **THE TWO-WRITERS RULE APPLIES TO ARGUMENTS, NOT JUST CODE, and today it caught me twice.** 0046: I would have inferred v3's behaviour from a `keep`/`as-observed` classification -- **a classification is a claim ABOUT code and only code answers for code.** TornRollback: my own earlier note enumerated four constructions, found none reached the variant, and concluded it was unreachable -- **all four were about `WriteSet::commit`'s unwind, and `Applied::rollback` is the other producer.** Both times the enumeration was sound and the conclusion was scoped to whatever was in view. **Ask what your checks have in common before believing they are independent, and ask which of the producers you enumerated.**
- **A guard's PRECONDITION can be invalidated by a change that is correct on its own terms and never looks at the guard** (vc). dc's 0042 emptiness test vs my `<not set>`: neither of us could see it from our own side.
- **A guard can be named for the exact defect and be incapable of firing.** A disjunction with a constant arm is where to look first -- I wrote one TODAY in `remedy_coverage.rs` and it worked only because the arm doing the work survived.
- **A test measuring the MECHANISM of a refusal fails when the mechanism improves.** `a_retired_rows_alias_does_not_come_back` asserted `unrecognized subcommand`, exact until a third outcome existed. Assert the invariant (is it on the surface?), not the message.

### Prose that is a build input, or a claim nothing checks

- **A `///` DOC COMMENT IS SHIPPED OUTPUT AND CONSUMER-FACING PROSE.** schemars lifts it into the JSON Schema face, async-graphql into the SDL, and both are COMMITTED and drift-checked. **Plain `//` for reasoning.** And write it for the consumer: a rustdoc intra-doc link reaches a published GraphQL schema where it names nothing.
- **A comment asserting a relationship between two documents is the one claim no compiler and no test is looking at**, because the act that changes the subject is not the act that revisits the sentence. **Replacing one count with another restarts the clock** -- describe nothing countable, and point at the mechanism that does enforce something.
- **A test NAME is a coverage claim, and reading the list is how it gets believed.**

### Greps, shells and measurement

- **A GREPPABLE PROXY THAT CANNOT SEPARATE CODE FROM TEXT ABOUT CODE FAILS TOWARD THE CLEAN-LOOKING ANSWER.** Three instances: `contains("rm ")` matching "form"; the retarget guard flagging `grep` patterns as invocations; my consumer roster, where **every hit is CLASSIFIED rather than heuristically filtered** because "is this line prose" has no mechanical answer. **The fix is a better discriminator or an authored classification -- never an exemption**, and dc declined the allowlist their own guard offered.
- **THE SHELL'S ANSWER IS NOT THE ONE YOU THINK.** `$?` after a pipe is the last command's (hit again today, through `| head`). `grep -c` exits 1 on zero. **zsh does NOT word-split an unquoted `$c`** -- use `${=c}`; it made a two-segment probe silently test one argument. And zsh eats a backticked fragment in a `-m` message.
- **A STALE ARTEFACT IS A SNAPSHOT, AND A FAILURE REPORT NAMES A REVISION.** The SHELL.errors handed to me was 40 minutes older than the fix. **Re-run before diagnosing**, including a peer's file.
- **A PARSER REPORTS ITS OWN LIMITS AS THE ESTATE'S DEFECTS, and it sounds authoritative.** 246 findings, 227 of them the reader's own bug. **ABSENT IS NOT INVALID.** A vocabulary is what the tool ACCEPTS, not what it prints.

### Git in a four-node clone

- **`git checkout -- <path>` REVERTS TO THE INDEX, so when your baseline is UNCOMMITTED work the revert destroys the thing you were testing.** Snapshot with `cp` to the scratchpad and restore with `cp`. This bullet existed and I hit it anyway; the sharpening is that the hazard is to your own unstaged work, not to the mutation.
- **`--only` COMMITS WHAT YOU NAME, and a hand-typed pathspec is a roster that is wrong where its author was distracted.** After a multi-path `--only`, check `git status` for leftovers IN YOUR OWN LANE -- `git diff --cached` only says what did go in. A move is TWO facts; verify at HEAD with `git ls-tree`.
- **STAGE NOTHING UNTIL THE MOMENT YOU COMMIT**, and an untracked file has no author in this clone. `MM` with a clean `git diff HEAD` is a stale index entry (0028). **`git stash` is unsafe here**; sacrificial `git worktree` only for `bin/intent*`.
- **A SHARED BUILD ARTEFACT HAS NO AUTHOR EITHER, AND IT READS AS EVIDENCE.** dc rebuilt `target/release` twice while my `render.rs` and `info_exit_code.rs` were uncommitted, so for twenty minutes the shared release binary was built from my in-flight work -- **and they nearly reported my fix as landed on the strength of it before checking `git log`.** The untracked-file trap one layer over: **a binary is evidence about a tree, and only `git log` says which tree.** Push before anyone measures.
- **NEVER PUT A `"` INSIDE A BOARD HEADER VALUE** -- measured trigger, cause unknown.

### Mechanics worth keeping

- **Cargo runs from `native/rust`.** `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift` re-pins the faces.
- **A surface built from a table cannot be unbuilt from the renderer**: the TABLE row moves first, the renderer second.
- **SQLite refuses `ADD COLUMN` for a NOT NULL column with a non-constant default**, so any DB-stamped column means a table rebuild. **`IF NOT EXISTS` makes a schema change invisible until a query fails** -- any DDL change bumps `SCHEMA_VERSION`. **Version 0 is never "schema zero".**
- **A fallback value can stand in for two different facts, and the second is the one nobody ruled on.** When you remove a guess, check what else was reaching it.
- **A migration fixture must be a store that could actually have existed.**
- **An error swallowed in a fixture is a silent error.** `expect()` in fixtures, always.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread** -- three consumers in ic's `gen_dispatch_table.sh`.
- **`doctor --fix` is WITHDRAWN, not deferred** (hv). A diagnostic that NAMES the remedy beats one that performs it. Nothing to remove in `render.rs` -- it was declared and never read; ic removes the row.
- **PUSH TO `local` ONLY -- `upstream` is FROZEN (hv, 2026-08-16 10:20Z, CI/CD budget hit).** Announced to all three peers. **The remotes will diverge and that is expected**, so the "verify both with `git ls-remote`" watch-out is suspended: compare `local` to HEAD only. **And CI is no longer the watcher for the Linux leg** -- the full suite, clippy and fmt run locally in seconds, but a `set -e` or path-separator break that only shows on Linux now has nothing checking it. That is the class that shipped v2.11.12 broken.
- **`install::MARKER` IS A SENTINEL, NOT A BOUNDARY** (cc ruled, 2026-08-17, on dc's packaging ask). It appears in three places -- `is_install`, the `NotFound` message, and `hook_script` -- and says nothing about what an install CONTAINS. **So the rule library does NOT move under it and its meaning does NOT widen**: when `critic` lands it resolves `intent/plugins/claude/rules/` from `install::home()`, exactly as `hook_script` resolves `lib/templates/.claude/scripts/`. Same function, same root, different subpath. **Owed with `critic`: a `rules_dir(home)` accessor beside `hook_script`**, so one module knows where shipped assets sit relative to the root, plus a sibling of `every_declared_hook_ships_as_a_script`. dc's archive is rooted at the INSTALL ROOT, which is right -- rooting it at the marker would have BUILT the boundary the question was asking about.
- **THE PACKAGING HOLD IS CLOSED** (dc, `7a41ff2e`), verified rather than accepted: the only outside reference any shipped script makes is `$INTENT_HOME_RESOLVED`, resolved at runtime back into the tree. **0036 is the only publication hold left.**
