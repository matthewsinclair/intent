---
node: dc
name: DevX Claude
role: worker
session_id: 3f2f5de2-d774-44db-8f1f-c85588606969
heartbeat_at: 2026-08-29 23:12Z
status: active
focus: "HOLDING FOR vc at 2026-08-29 23:12Z. ST0066`s AT KIND IS BUILT AND FIVE TESTS ARE RED -- THREE MINE AND MECHANICAL, TWO vc`s D56 WRITE. RE-MEASURE, DO NOT TRUST THESE FIGURES: `cargo test -p intentsvcs` and `-p intent-cli`, EXIT CODE FIRST, and `cargo fmt --check` on BOTH (it is a SEPARATE GATE with no local alarm since hv moved it to CI). THE CLI IS FIXED AND THE STORE NEVER MOVED -- hv ruled #fixforward, I rebuilt the shared binary in 58s, and F12 now carries the correction that my own panic was misfiled. hv RULED THE ST/WP SHAPE ON 2026-08-28 AND vc`s INSTRUCTION TO ME WAS THE DECLINED OPTION: ST/WP get NO status variant, `fiat` sits BESIDE a status that stays `completed`/`done`, carried by ONE COMPOSER. vc withdrew it; the `st.fc`/`wp.fc`/`at.fc` EDGES are with hv."
claims: [ST0056/07, ST0056/11, ST0056/14, ST0066]
---

# DevX Claude (dc)

**THIS FILE IS THE COLD-SESSION MINIMUM.** Tonight's reasoning is verbatim in `.history/20260829/wip-fold-2303Z.md`; the earlier folds and the unabridged watch-out families are beside it.

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.**
- **`stat` PRINTS LOCAL. `git log` PRINTS LOCAL.** Convert at the SOURCE.
- **`cargo test` DOES NOT WRITE `~/.intent/home`** -- my causal claim there was a spurious correlation, corrected by my own experiment. **But it DOES migrate the runtime store; see F12.**

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is.

## The environment

- **`intent` ON PATH IS v3 AND RESOLVES INTO THIS TREE. `intent3` IS NOT ON PATH.** Both `~/.local/bin/intent` and `~/bin/intent` are symlinks to `native/rust/target/release/intent`.
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED.**
- **hv's FREEZE SCOPE: Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.**
- **THE INDEX IS SHARED IN THIS CHECKOUT.** `git add` puts your file where a peer's bare `git commit` sweeps it; `add` + `commit --only <path>` is the only safe write.
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.**
- **A DEVBIN COMMAND RESOLVES ITS PROJECT FROM SOMETHING OTHER THAN YOUR CWD -- THE v3 BINARY RESOLVES FROM CWD.**
- **`cargo fmt --check` IS A SEPARATE GATE FROM THE SUITE AND HAS NO LOCAL ALARM.** hv moved fmt and clippy out of `prepush` into CI on 2026-08-29, so a green suite is not a green push. Run it before declaring a lane ready. **And never through a pipe** -- see F11.

## DOING

### ST0066 -- THE AT KIND IS BUILT. ST/WP ARE RULED AND THEIR EDGE IS WITH hv.

**hv's RULING OF 2026-08-28 18:09Z (`hv/wip.md:66`) IS THE SPEC, AND IT IS TWO-AND-TWO, NOT THREE-AND-THREE:**

- `AcState` -- has `Fiat`, carrying its record IN the variant. Built and driven.
- `AtStatus` -- **unit `Fiat` variant**, record BESIDE it on `AcceptanceTest.fiat`, because `AtStatus` derives `Copy` AND async-graphql `Enum` and a payload breaks both. **BUILT TONIGHT.**
- `ThreadStatus` / `WpStatus` -- **NO VARIANT.** `fiat: Option<FiatRecord>` beside a status that stays `completed` / `done`. **`fiat in the status` IS THE OPTION hv DECLINED.**
- **"WITH ONE REQUIRED COMPOSER" IS IN THE RULING'S TITLE AND IS THE HALF THAT DOES THE WORK.** With fiat beside the status, nothing structural makes a renderer look, so AC-00.3 is carried by one composer every status render passes through -- the shape `ac list` uses for `AcRow.state`. **UNBUILT. Build it before ST/WP, not after.**

**BUILT TONIGHT, all uncommitted:** `AtStatus::Fiat` + `display() -> "fiat-closed"`; `AcceptanceTest.fiat` with `skip_serializing_if`; `Facade::at_fc` (machine checked through `check_transition`, reason through `check_reason`, NO hand-written from-state check -- issues 0051/0053); the `at.fc` edge in `transitions.rs` from `to-write`/`red` only; `KNOWN_OPS`; the D42 stamp walk extended to AT records; **SCHEMA 15** (`tests.fiat TEXT` + rung 15) with both pins re-cut; the three schema faces blessed; `AlreadyFiatClosed` widened with an `undo` field so its remedy is not hardcoded to one kind.

**FIVE TESTS RED. THREE ARE MINE AND EVERY ONE NAMES ITS OWN FIX:**

- `a_machine_ratified_in_prose_is_actually_trivial` -- **the AT machine has outgrown its prose ratification and now owes a TABLE.** `RATIFIED_WITHOUT_A_TABLE` says one verb, no from-restriction; `at.fc` is a second verb WITH a from-set and a guard. Needs: `### Machine 5` in `data-model.md`, a `RATIFIED_AT` const + a `RATIFIED` row in `mutation_completeness.rs`, the AcceptanceTest row REMOVED from `RATIFIED_WITHOUT_A_TABLE`, and `machine_table_check.sh`'s `MACHINE_MAP` gaining `5 AcceptanceTest status` while `UNTABLED` loses it. **PARSER FACTS, MEASURED, SO THE NEXT SESSION DOES NOT RE-DERIVE THEM: an entry row is `_(none)_` (`:248`), and an EMPTY from-set renders `(any)` (`:314`).**
- `every_declared_field_of_every_model_is_settable_or_refused_by_name` -- `fiat` needs its settable row.
- `a_face_whose_contract_moves_must_bump_that_faces_version` -- all THREE face versions moved and want bumping + re-pinning in `intentsvcs::faces`: DDL 11, SDL 11, JSON 13. The test prints the new hashes.

**TWO ARE NOT MINE AND I GOT THEM WRONG ONCE ALREADY:** `every_realised_attachment_in_the_estate_still_matches_canon` and `every_thread_prose_file_is_carried_and_its_bytes_round_trip` both fail on `ST0056/design.md`, disk 133485 vs canon 129247. **It is vc's uncommitted D56 write reaching canon by the normal path, NOT an estate divergence needing adjudication.** I reported the opposite to cc and vc; cc corrected me and I verified it.

**STILL OWED ON ST0066:** the composer; the ST/WP half once hv rules the edge; the CLI `fc` arm still routes `AT-` to `unwired`, so **the AT kind is BUILT AND NOT REACHABLE** -- `representable is not the same as reachable` is this thread's own lesson and it is currently true again; `doctor`'s render (AC-00.5); the six ATs; AC-00.6's rules-library rule pending hv.

**OPEN, BUILD AROUND IT:** minutia 3 -- the accepted-unverified half as free text in `because` or a structured field.

### HELD -- NOT MINE TO START

- **0136 lands AFTER v3.0.1**; vc calls one-commit-or-split at the cut.
- **0137 IS CLOSED** and its demotion is the lesson: a defect whose only defence is that nothing can reach the state stops being defended by the change that reaches it.
- **THE `claude start`/`ws` SMOKE ARM IS OWED AND NEEDS NO KEG** -- `critic_refuses_an_empty_library_end_to_end.rs`'s `fixture_install()` builds a keg-SHAPED tree with the binary **COPIED, NEVER SYMLINKED** (`install::resolve()` canonicalises before walking for `lib/templates`). S, red-first by omitting the door. vc sequences it.
- **ic OWNS THE `at.fc` REGISTER ROW AND HAS LANDED IT** -- `at.fc -> ["fc"]` in `FANS_OUT`, and the ruling is generalised: a fiat-close edge is spelled `fc` whatever machine it sits on. **`st.fc` and `wp.fc` each need one line there THE MOMENT their edges land**, and ic deliberately did NOT pre-add them, because that file's own guard checks the right-hand side only and a speculative row would pass in silence.

### PARKED -- LIVE, HELD, NOT MINE TO CLOSE

- **WP-11 CANNOT MOVE WITHOUT A PUBLISHED TAG.** Publish stops with hv.
- **FLEET WORK IS HALTED BY hv AND hv LIFTS IT, NOT vc.** My four are dirty and uncommitted.
- **WP-07's SEVENTH CRITERION** -- all six AC-07.x are canon/parity mechanisms and not one names the `claude` subsystem's own doors, which is why the 3.0.0 keg shipped the cwi door missing with nothing going red. Proposed to vc for hv.
- **THE hv QUEUE:** roster symmetry (`runner_roster_check.sh` reads PRESENT from the COMMIT and ROSTERED from the WORKTREE); nothing verifies a roster row DESCRIBES what its runner does; the `cmd/macos` provenance writer; the attribution guard.

## TODO

- **hv's STANDING DIRECTIVE (b): THE `bin/int` -> `bin/devbin` RENAME GOES AFTER THE SWEEP.** The dispatcher is addressed by its official name `bin/devbin`; a project MAY keep a short symlink for humans, but tools and process always use the official one.
- **ST0066 IS 0/6 AND THAT FIGURE IS STILL HONEST.** Verb, writer, cascade, doctor render, rules-library rule, six ATs. Ticking these without ATs would be a hand-performed fiat close of the thread that builds fiat close.

## Watch-outs

**STANDING FAMILIES, RULE ONLY -- AND DELIBERATELY UNCOUNTED, BECAUSE A COUNT HERE IS A SECOND HOME FOR A FACT THE LIST BELOW ALREADY STATES (F7). IT READ `TWELVE` OVER FOURTEEN ENTRIES.** Full text in `.history/20260829/watch-outs-full-2136Z.md`. A rule is never dropped here, only its narrative.

- **F0 -- THE SHARED CHECKOUT AND THE BLIND INSTRUMENT.** The index is shared. A suite run in a shared tree measures the TREE, not HEAD -- **AND SO DOES THE BINARY.** `target/release/intent` is built from the worktree, so every `intent <verb>` any node runs is a measurement of four nodes' uncommitted work. `git diff` reads clean while the tree is dirty when a peer's edits are STAGED.
- **F1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT.** Positive-control before believing a zero. **Tonight: `grep AtStatus transitions.rs` returned nothing and I nearly reported "no machine exists" -- but `ThreadStatus`, `WpStatus` and `IssueStatus` also return nothing, because that file keys by entity/field STRINGS. The control caught a false absence that would have read as a finding.**
- **F2 -- THE CLAIM EXCEEDS THE MEASUREMENT.** State what you measured, not what it suggests.
- **F3 -- ROUTING, RELAY AND ATTRIBUTION.** A peer's quote of a ruling is still the peer. Route by owner; say who measured what.
- **F4 -- THE SHARED CHECKOUT, WRITES.** Never `cp` a shared source aside to mutate; a reflog check precedes any reset of a shared HEAD. **And never bless, format or regenerate a shared artefact that carries a peer's in-flight change** -- ic declined to bless my model change under their hand and was right to; when I blessed, I checked the diff removed ZERO lines before leaving it.
- **F5 -- THE GATE THAT DID NOT RUN.** A commit is an indirect `intent` invocation whose two halves fail in OPPOSITE directions, so a build window is silently also a commit freeze.
- **F6 -- AN UNMEASURED CLAIM INSIDE A COMPLIMENT.**
- **F7 -- A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR A FACT.** Drive the verb; never transcribe its number.
- **F8 -- THE ESTATE'S OWN CONFIGURATION IS WHAT HIDES ITS BUGS FROM IT.**
- **F9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE.** Read the whole extent before claiming a property OF the whole. A partial read that finds SOMETHING is self-correcting; one that finds NOTHING yields a conclusion you cannot falsify without redoing the read. **THE CORRECTIVE WORKED TONIGHT: I read `hv/wip.md`'s BODY before building and found the ruling that made my instruction wrong.** **And its newest instance: `cargo check` DOES NOT COMPILE TEST TARGETS -- a green that could not have seen the failure. `--all-targets` or it is not a compile.**
- **F10 -- HOW TO REPORT.** Write reasoning at the resolution you MEASURED it. **Verify a peer's correction yourself, including when it makes your own claim false** -- done twice tonight, and cc was right both times.
- **F11 -- THE TERMINATING CONDITION IS AN UNDECLARED FILTER.** `cargo test` ABORTS after a failing target, so two runs report different POPULATIONS in the same units. Read the exit code FIRST. **NEVER TAKE `$?` THROUGH A PIPE** -- it belongs to the last stage. **Third instance tonight: `cargo fmt --check | head` printed a diff and my own echo said `rc=0`. The trap does not look like a failure; it looks like a pass sitting beside contradicting output.**
- **F12 -- NEW. `cargo test` IS NOT READ-ONLY: IT MUTATES SHARED DURABLE STATE, ONE-WAY.** Sixteen files in `intentsvcs/tests` call `Project::open(&repo_root())` against the LIVE estate, because reading the real estate is the point of those ATs. Opening a store RUNS THE MIGRATION LADDER. So driving the suite with an unlanded rung in your worktree migrates the shared store, every peer's older binary then refuses, and **the ladder is deliberately one-way with no downgrade.** **Delete-and-rebuild is NOT the escape** -- `store.rs:4` rules it out in as many words: _MIGRATIONS ARE NORMAL, so there is no "rebuild instead of migrating" story_. The only remedy is a binary that speaks the new version. **The vector is the act everyone performs constantly and believes is read-only.** **RESOLVED 2026-08-29 23:12Z, AND THE RESOLUTION CORRECTS THIS ENTRY.** hv's ruling was one word, #fixforward, and it is right: `cargo build --release` is 58 seconds, the store never moves, and both `~/.local/bin/intent` and `~/bin/intent` point at the one `target/release/intent`, so a single rebuild fixes every node. **I had written `one-way with no downgrade` as though it meant UNRECOVERABLE. It does not -- the direction the ladder goes is the direction the work was going anyway.** Keep the distinction the panic lost: what made this severe was breaking PEERS, a shared build artefact going stale, which is loud and cheap. I filed it under the vocabulary of data loss, and the vocabulary did the reasoning.
- **F13 -- NEW. A `git status` IS A MEASUREMENT WITH A SHELF LIFE, AND IN A FOUR-NODE TREE IT EXPIRES IN MINUTES.** I told cc and vc that `design.md` was unmodified in git and the store was therefore stale -- from a `git status` I had run forty minutes earlier, before vc wrote D56. **The claim was true when measured and false when sent, and nothing about its wording said which.** Re-run the status in the same turn as the sentence that rests on it. **Related and distinct: "unmodified at HEAD" and "nobody has touched it" are DIFFERENT CLAIMS** (vc's sharpening), and the first will be read as the second every time.

## Decisions

- **2026-08-29 -- #fixforward IS A RULING ABOUT WHICH DIRECTION IS CHEAP, NOT AN INSTRUCTION TO PRESS ON.** I reported the migrated store as damage; hv answered with one word. The reason it is the whole answer is that a one-way ladder only sounds unrecoverable while you are reading it as data -- the thing that was actually stale was a BUILD ARTEFACT, and it rebuilt in 58 seconds without the store moving a version. **The correction I want to keep is not about stores: I reached for the vocabulary of data loss and then reasoned inside it.**

- **2026-08-29 -- hv HAD ALREADY RULED THE THING I WAS INSTRUCTED TO BUILD, AND THE RULING WAS THE OTHER WAY.** vc told me to put a unit `Fiat` variant on `ThreadStatus`/`WpStatus`; `hv/wip.md:66` records _FIAT BESIDE THE STATUS ... (menu: beside the status + composer -- CHOSEN | fiat in the status)_. I quoted it back rather than building, vc verified first-hand and withdrew. **The corrective that caught it is the one I adopted from my own miss the same morning: read hv's board BODY at pickup, never just its header.** vc's own note: it was their second time reasoning from code where the answer was in the document they maintain.
- **2026-08-29 -- AN OMISSION IN AN INSTRUCTION CAN COST MORE THAN AN ERROR IN IT.** The ruling's title carries _WITH ONE REQUIRED COMPOSER_ and that never reached me. Building the variant and skipping the composer would have produced the DECLINED design AND lost the only thing the chosen one buys.
- **2026-08-29 -- A RULING'S PREMISE CAN BE CORRECTLY MEASURED AND STILL NOT COVER WHAT YOU ARE BUILDING.** hv's DO-NOT-BUMP records _ZERO DDL_ for ATs, measured on `tests.status` being unconstrained TEXT. True, and it is a claim about the STATUS VALUE. `fiat` is a separate FIELD -- forced there by `Copy` + `Enum` -- and the `tests` table had nowhere to put it. **So the AT kind cost a column, a rung and a schema bump.** Measured and reported rather than either building around it or treating the ruling as wrong.
- **2026-08-29 -- THE FIRST RUNG THAT IS AN `ALTER` RATHER THAN A REBUILD BREAKS THE ONE TEST THAT SIMULATES AN OLD STORE.** `a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_refused` builds its fixture from the CURRENT `DDL` and stamps an old version on it, so `ADD COLUMN fiat` hits `duplicate column name`. Every sibling rung is a table rebuild and survives it. **The load-bearing detail in the fix: the rebuild's `SELECT` must NOT name the new column**, which is what makes one statement correct against both a real store that lacks it and a fixture that has it.
- **2026-08-29 -- A PUBLISHED DOC COMMENT WAS FALSE AND HAD BEEN READ MANY TIMES.** `FiatRecord`'s `///` said the record is _kept forever and never cleared by a later transition_. `ac_reinstate` sets the state to `AcState::entry(kind)`, so it is discarded structurally -- and hv's 2026-08-29 exit ruling settles the ground: **the history is the event log, not this field.** It matters more than an ordinary stale comment because the type derives `JsonSchema` and `SimpleObject`, so the sentence was published verbatim into the committed faces as a durability guarantee the model does not offer.
- **2026-08-29 -- THE COMPILER IS THE ORACLE FOR A FIELD ADDITION, AND THE HAND-ROSTERS ARE WHERE IT IS BLIND.** Driving `cargo check --all-targets` in a loop closed fourteen fixture sites without guessing. What it could NOT see: `model_laws.rs`'s proptest strategy (still owes a `Fiat` arm, or the round-trip law never generates one), the SDL variant COUNT, and `mutation_every_writable_field.rs`'s compile-fenced list -- which caught itself, because its destructure is the fence.
