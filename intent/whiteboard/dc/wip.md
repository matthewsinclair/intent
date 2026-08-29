---
node: dc
name: DevX Claude
role: worker
session_id: 3f2f5de2-d774-44db-8f1f-c85588606969
heartbeat_at: 2026-08-29 23:55Z
status: active
focus: "ST0066's SERVICE HALF IS LANDED at 587105cb (31 files). NEXT, IN ORDER: `### Machine 5` + `RATIFIED_AT` (hv ruled it, data-model.md is free since 0148 landed at cd1bf7d7), then the Guard column's CLOSED VOCABULARY, then `st.fc`/`wp.fc` edges guarded ReasonRecorded NOT GatePass, then commit `render.rs` once ic's `form.rs` and cc's `lib.rs` land. RE-MEASURE EVERYTHING: `cargo test -p intentsvcs -p intent-cli --no-fail-fast`, EXIT CODE FIRST, and `cargo fmt --check` on BOTH separately. vc HOLDS THE PEN FROM hv AND OWNS SPEC/ADJUDICATION/REVIEW; I MAKE IT WORK."
claims: [ST0056/07, ST0056/11, ST0056/14, ST0066]
---

# DevX Claude (dc)

**COLD-SESSION MINIMUM.** Tonight's reasoning is in `.history/20260830/wip-fold-preAFK.md` and `.history/20260829/`. A rule is never dropped here, only its narrative.

## D42 -- TIME. Read before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **not the database.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES: NO cli or intentsvcs function TAKES a time.** They may RETURN times.
- **A board stamp is a label, not data** -- read from `date -u` and PASTE, **per stamp, never per session.**
- **`stat`, `git log`, `ls -la` ALL PRINT LOCAL.** Appending `Z` to a local read is an ASSERTION, not a format.

## The truth model and the environment

- `design.md` (D01 reversed) + `data-model.md`. **The SQLite db is the durable SSOT; the typed API is the only door in.** Crates are `intent-cli`, `intentd`, `intentsvcs`. **`intentdb` IS RETIRED.**
- **`intent` ON PATH IS v3 AND RESOLVES INTO THIS TREE.** Both `~/.local/bin/intent` and `~/bin/intent` symlink `native/rust/target/release/intent`.
- **THE INDEX IS SHARED.** `git add` puts your file where a peer's bare `git commit` sweeps it; **`add` + `commit --only <paths>` is the only safe write** -- verified tonight: my `--only` commit left ic's staged files untouched.
- **`cargo fmt --check` IS A SEPARATE GATE WITH NO LOCAL ALARM** since hv moved fmt+clippy to CI. Green suite is not a green push.
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND NEVER TRANSCRIBED.**

## DOING

### ST0066 -- SERVICE HALF LANDED (587105cb). CLI HALF AND THE TABLE REMAIN.

**hv's SHAPE, AND IT IS TWO-AND-TWO:** `AcState` has `Fiat` carrying its record IN the variant. `AtStatus` has a UNIT `Fiat` with the record BESIDE it on `AcceptanceTest.fiat` (`Copy` + async-graphql `Enum` forbid a payload). **`ThreadStatus`/`WpStatus` get NO variant** -- `fiat` beside a status that stays `completed`/`done`. **`fiat in the status` IS THE OPTION hv DECLINED.**

**LANDED:** `AtStatus::Fiat`, `AcceptanceTest.fiat`, `Facade::at_fc`, the `at.fc` edge, D42 stamp walk, **SCHEMA 15**, faces **DDL 12 / SDL 12 / JSON 14**, `model::fiat_status` (hv's required composer), `at_set` clearing the record, **D7 `put` refusal on BOTH kinds**.

**NEXT, IN ORDER (vc's sequencing, hv's rulings):**

1. **`### Machine 5` + `RATIFIED_AT`** -- hv ruled it. Remove AcceptanceTest from `RATIFIED_WITHOUT_A_TABLE`; `machine_table_check.sh`'s `MACHINE_MAP` gains `5 AcceptanceTest status` and `UNTABLED` loses it. **PARSER FACTS, MEASURED: entry row is `_(none)_`; an EMPTY code from-set renders `(any)`; `verbdot()` takes the FIRST TWO WORDS and does NOT strip flags; `kebab()` inserts a dash before any non-initial capital, so a state cell must be `ToWrite`, never `To-Write`.** State in it: the fiat/status invariant is enforced by NO type and so must be enforced by every verb touching status.
2. **The Guard column's CLOSED VOCABULARY** -- clears the checker's standing 3 DISAGREE / 6 UNMEASURED. **My fiat rows inherited the pre-existing shape; none of it is a new divergence.**
3. **`st.fc`/`wp.fc` edges** -- from the same from-states as `st.done`/`wp.done`, same landing states, guarded **`ReasonRecorded`, NOT `GatePass`**. Relaxing `st.done` to accept either guard would make `st done` silently fiat-capable.
4. **Commit `render.rs`** -- carries MY `fn fc` + composer wiring, cc's `fn engine`, ic's `fn surface`/`surface_retired`/`browsed`. **vc ruled I take it whole and NAME whose hunks are whose.** Blocked until ic's `form.rs` and cc's `lib.rs` land, or it breaks the build at that revision.
5. **Still owed:** the cascade with inherited marker, `doctor`'s render (AC-00.5), the six ATs, AC-00.6's rules-library rule, **D4's structured field for the accepted-unverified half** (hv ruled: a structured FIELD, not free text in `because` -- doctor cannot detect a convention buried in prose).

**KNOWN RED AND IT IS MINE:** `a_machine_ratified_in_prose_is_actually_trivial` -- closed by item 1. **`every_shipped_mutator_is_accounted_for` is ic's `browse`, not mine.**

**ST0066 IS 0/6 AND THAT FIGURE IS HONEST.** Ticking without ATs would be a hand-performed fiat close of the thread that builds fiat close.

## TODO

- **hv's STANDING DIRECTIVE (b):** the `bin/int` -> `bin/devbin` rename goes AFTER the sweep; tools and process use the official name.
- **PARKED, NOT MINE TO CLOSE:** WP-11 needs a published tag (hv's hand); fleet work is halted by hv and hv lifts it; WP-07's seventh criterion; the `claude start`/`ws` smoke arm (fixture copies the binary, never symlinks).

## Watch-outs

**STANDING FAMILIES, RULE ONLY -- DELIBERATELY UNCOUNTED, because a count here is a second home for a fact the list already states (F7).**

- **F0 -- THE SHARED CHECKOUT AND THE BLIND INSTRUMENT.** A suite run in a shared tree measures the TREE, not HEAD -- **and so does the BINARY**: `target/release/intent` is built from the worktree, so every `intent <verb>` measures four nodes' uncommitted work.
- **F1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT.** Positive-control before believing a zero. `grep AtStatus transitions.rs` returns nothing because that file keys by entity/field STRINGS.
- **F2 -- THE CLAIM EXCEEDS THE MEASUREMENT.** State what you measured, not what it suggests.
- **F3 -- ROUTING, RELAY AND ATTRIBUTION.** A peer's quote of a ruling is still the peer. Say who measured what.
- **F4 -- THE SHARED CHECKOUT, WRITES.** Never `cp` a shared source aside to mutate. **Never format or bless a file carrying a peer's in-flight change** -- ic's untracked `form_declares_layout_not_the_field_set.rs` is fmt-red and I left it alone.
- **F5 -- THE GATE THAT DID NOT RUN.** A commit is an indirect `intent` invocation; a build window is silently also a commit freeze.
- **F6 -- AN UNMEASURED CLAIM INSIDE A COMPLIMENT.**
- **F7 -- A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR A FACT.** Drive the verb; never transcribe its number.
- **F8 -- THE ESTATE'S OWN CONFIGURATION IS WHAT HIDES ITS BUGS FROM IT.**
- **F9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE.** A partial read finding SOMETHING self-corrects; one finding NOTHING yields a conclusion you cannot falsify without redoing it. **Fired tonight: `head -30` over commit bodies said "this repo uses no trailers"; the full corpus says 37 of 40 DO.** Also: `cargo check` does not compile test targets -- `--all-targets` or it is not a compile.
- **F10 -- HOW TO REPORT.** Write reasoning at the resolution you MEASURED it. **Verify a peer's correction yourself, including when it makes your own claim false.**
- **F11 -- NEVER TAKE `$?` THROUGH A PIPE.** It belongs to the last stage. `cargo fmt --check | head` prints a diff and reports rc=0. **Four instances across three nodes.** `cargo test` also ABORTS after a failing target, so use `--no-fail-fast` or two runs report different POPULATIONS in the same units.
- **F12 -- `cargo test` IS NOT READ-ONLY: IT MIGRATES THE SHARED STORE, ONE-WAY.** Sixteen files open the LIVE estate. **RESOLVED and the resolution corrects the entry: "no downgrade" is NOT "unrecoverable".** `cargo build --release` is 58s, the store never moves, one rebuild fixes every node. **The severity was entirely BLAST RADIUS, not data** -- I filed it under the vocabulary of data loss and the vocabulary did the reasoning.
- **F13 -- A `git status` IS A MEASUREMENT WITH A SHELF LIFE, AND IN A FOUR-NODE TREE IT EXPIRES IN MINUTES.** True when measured, false when sent, with nothing in the wording saying which. **Four instances today; one was vc observing a staged file of MINE that I had already cleared.** Re-run the status in the same turn as the sentence resting on it.
- **F14 -- NEW. A GUARD IN A ROSTER THE DOOR DOES NOT READ IS NOT A GUARD, AND THE SUITE STAYING GREEN IS THE TELL.** hv's D7 went into `Unsettable` first; that roster is read by `set` and the form layer's editability, and **`put` consults neither and writes the whole row.** 29/29 passed and the refusal moved nothing. **A refusal that reds nothing has not been added** -- when you add a restriction, something must go red, and if nothing does, find the door before believing the guard.
- **F15 -- NEW. "NEW IN THE WORKTREE" IS NOT "SOMEONE ELSE'S", AND OWNERSHIP IS MEASURED, NEVER INFERRED.** I read `fn fc` as cc's because it was absent at HEAD, and told cc so. **It is MINE** -- `git log -S'fn fc(' --` returns no commit at all. **Three of us independently converged on the same wrong owner; cc is the only one who measured.** That is not carelessness three times, it is a property of a file nobody can see whole. Use `git log -S<symbol> -- <path>`.
- **F16 -- NEW. CANON ORDER: SYNC FIRST, THEN COMMIT THE FILE AND CANON TOGETHER.** `canon_commit_check` refuses the obvious order in its own words: _a later sync fixes the NEXT commit and can never fix this one; the criterion is a property of every commit, not of HEAD._ **And a BLOCKED commit still runs the formatters** (vc's finding), which rewrite files AFTER the sync recorded their bytes -- so a retry is not idempotent and needs a re-sync first.

## Decisions

- **2026-08-29 -- #fixforward IS A RULING ABOUT WHICH DIRECTION IS CHEAP.** A one-way ladder only sounds unrecoverable while you read it as data; the stale thing was a BUILD ARTEFACT.
- **2026-08-29 -- hv HAD ALREADY RULED THE THING I WAS INSTRUCTED TO BUILD, THE OTHER WAY.** I quoted `hv/wip.md:66` back rather than building; vc verified and withdrew. **The corrective: read hv's board BODY at pickup, never just its header.**
- **2026-08-29 -- AN OMISSION IN AN INSTRUCTION CAN COST MORE THAN AN ERROR IN IT.** _WITH ONE REQUIRED COMPOSER_ was in the ruling's title and never reached me.
- **2026-08-29 -- A RULING'S PREMISE CAN BE CORRECTLY MEASURED AND STILL NOT COVER WHAT YOU BUILD.** hv's DO-NOT-BUMP measured `tests.status` as unconstrained TEXT -- a claim about the VALUE. `fiat` is a separate FIELD, so the AT kind cost a column, a rung and a bump.
- **2026-08-29 -- A PUBLISHED `///` IS A CONTRACT, AND TWO OF THEM WERE FALSE.** `FiatRecord`'s _kept forever_ was disproved by `ac_reinstate`; `AcceptanceTest::fiat`'s _present exactly when_ was disproved by `at_set`. Both derive `JsonSchema`/`SimpleObject`, so both were published verbatim into the committed faces. **One was fixed by correcting the prose and one by correcting the CODE, and which is which is the judgement.**
- **2026-08-29 -- THE COMPILER IS THE ORACLE FOR A FIELD ADDITION; THE HAND-ROSTERS ARE WHERE IT IS BLIND.** It could not see `model_laws.rs`'s proptest strategy, the SDL variant COUNT, or `fully_populated_row()`, **whose contract is that no field is measured against a `None` serde dropped -- and whose own doc names that trap for two earlier fields.**
- **2026-08-29 -- TWO COMPOSERS IS THE CORRECTION, NOT A COMPROMISE.** hv said _one required composer_; the count that was wrong was never the NUMBER, it was the UNIT -- one per VOCABULARY (generated view vs list surface), not one per call site.
