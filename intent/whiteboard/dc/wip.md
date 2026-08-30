---
node: dc
name: DevX Claude
role: worker
session_id: 3f2f5de2-d774-44db-8f1f-c85588606969
heartbeat_at: 2026-08-30 00:39Z
status: active
focus: "ST0066: the AT half, MACHINE 5 and the CLOSED GUARD VOCABULARY are all LANDED (587105cb, 213be93f, 628ea602). `render.rs` went in on hv's 0f41dce1 sweep, not by my hand. NEXT AND IT IS THE BIG ONE: `st.fc`/`wp.fc` -- measured, NOTHING of the ST/WP half exists: no `fiat` on Thread or WorkPackage, no `st_fc`/`wp_fc`, no edges, so it is a full build like the AT half was (model, DDL, rung + SCHEMA 16, facade, transitions, Machines 1+2, RATIFIED_THREAD/RATIFIED_WP + driver arms, CLI, faces). Guarded `ReasonRecorded`, NEVER `GatePass`. RE-MEASURE EVERYTHING; EXIT CODE FIRST, NEVER THROUGH A PIPE."
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

### ST0066 -- THE AT HALF, MACHINE 5 AND THE GUARD VOCABULARY ARE ALL LANDED. THE ST/WP HALF IS THE REMAINDER.

**hv's SHAPE, AND IT IS TWO-AND-TWO:** `AcState` has `Fiat` carrying its record IN the variant. `AtStatus` has a UNIT `Fiat` with the record BESIDE it on `AcceptanceTest.fiat` (`Copy` + async-graphql `Enum` forbid a payload). **`ThreadStatus`/`WpStatus` get NO variant** -- `fiat` beside a status that stays `completed`/`done`. **`fiat in the status` IS THE OPTION hv DECLINED.**

**LANDED:** `AtStatus::Fiat`, `AcceptanceTest.fiat`, `Facade::at_fc`, the `at.fc` edge, D42 stamp walk, **SCHEMA 15**, faces DDL 12 / SDL 12 / JSON 14, `model::fiat_status`, `at_set` clearing the record, D7 `put` refusal on BOTH kinds (`587105cb`); **`### Machine 5` + `RATIFIED_AT` + `UNRESTRICTED_VERBS` + the four re-anchored ruling records + the gate's own corrected message (`213be93f`); the CLOSED GUARD VOCABULARY and axis C GATING, Machine 3 from 36/3/6 to 45 of 45 (`628ea602`)**; vc's four property tables.

**NEXT -- `st.fc`/`wp.fc`, AND IT IS A FULL BUILD, NOT AN EDGE.** MEASURED 2026-08-30: `grep "pub fiat" model.rs` returns ONE hit (`AcceptanceTest`); there is no `st_fc`/`wp_fc` in `facade.rs` and no `st.fc`/`wp.fc` in `transitions.rs`. So it is the AT half again across ST and WP: model field, DDL column, a rung and **SCHEMA 16**, the two facade verbs, the two edges, Machine 1 and Machine 2 rows, `RATIFIED_THREAD`/`RATIFIED_WP` plus driver arms in `mutation_completeness.rs`, CLI wiring, faces bump. **Guarded `ReasonRecorded` and NEVER `GatePass` -- a fiat close is BY DEFINITION the case where the gate does not pass, and relaxing `st.done` to accept either would make `st done` silently fiat-capable.**

**STILL OWED AFTER THAT:** the cascade with inherited marker, `doctor`'s render (AC-00.5), the six ATs, AC-00.6's rules-library rule, **D4's structured field** for the accepted-unverified half.

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

- **F17 -- NEW. AN ANCHOR THAT RESOLVES TO THE RECORD'S OWN RENDERING IS CIRCULAR, AND IT IS WORSE THAN THE DANGLE IT REPLACES.** Re-anchoring hv's `surface` spelling ruling, the first grep hit was `surface/dispatch-table.md` -- word-perfect, because it is the GENERATED RENDERING of that very record. It resolves, and a reader following it lands on the citation instead of the ruling. **A dangle is visible; a circle certifies itself.** vc's condition (grep the target BEFORE writing the anchor) is what caught it. The general form: when a check asks "does this reference resolve", a generated copy of the reference always answers yes.
- **F18 -- NEW. A DETACHED WORKTREE GETS ITS OWN `CARGO_TARGET_DIR`, AND THAT DIRECTORY DIES WITH THE WORKTREE.** Both halves, because each alone is a defect I caused tonight. **Sharing the main target dir** bakes `CARGO_MANIFEST_DIR` into cached rlibs; deleting the worktree left four test binaries asserting a path that no longer exists, failing later and pointing at innocent files (cc, read out with `strings`). **A private target dir** is what let my abandoned `wt-0133` -- merged, clean, forgotten -- grow to **10G**. The disk was never the cost of isolation; it was the cost of isolation that OUTLIVED its worktree.
- **F19 -- NEW. ONE SENTENCE COVERING TWO CONDITIONS PICKS NEITHER FIX, AND IT COST TWO SEPARATE INVESTIGATIONS TODAY.** `machine_table_check.sh` printed the same words for a table that was ABSENT and one that was MALFORMED; vc read it and reported a peer as mid-write on a section that had never existed. Axis C would have collapsed UNMEASURED into DISAGREE -- "unreadable here" reported as "wrong here". **Opposite fixes both times.** Split them, and say which in the message.
- **F20 -- NEW. TWO FILES CAN HOLD DIFFERENT READINGS OF ONE VALUE AND NOTHING SEES IT UNTIL SOMETHING USES THE SECOND.** `from: &[]` meant ANY STATE to `transitions.rs` (`Edge::accepts`) and NEVER FILLED IN to `mutation_completeness.rs`, which then demanded `at.set` refuse from `fiat` -- the one state `at.set` is the exit FROM. Both readings defensible, exactly one true, invisible for as long as no ratified machine carried such an edge. Now declared in `UNRESTRICTED_VERBS` and checked in BOTH directions. **ic hit the same shape the same night** in `fn flag`: `try_get_one::<bool>().ok()` swallows a valued flag as ABSENT.

## Decisions

- **2026-08-29 -- #fixforward IS A RULING ABOUT WHICH DIRECTION IS CHEAP.** A one-way ladder only sounds unrecoverable while you read it as data; the stale thing was a BUILD ARTEFACT.
- **2026-08-29 -- hv HAD ALREADY RULED THE THING I WAS INSTRUCTED TO BUILD, THE OTHER WAY.** I quoted `hv/wip.md:66` back rather than building; vc verified and withdrew. **The corrective: read hv's board BODY at pickup, never just its header.**
- **2026-08-29 -- AN OMISSION IN AN INSTRUCTION CAN COST MORE THAN AN ERROR IN IT.** _WITH ONE REQUIRED COMPOSER_ was in the ruling's title and never reached me.
- **2026-08-29 -- A RULING'S PREMISE CAN BE CORRECTLY MEASURED AND STILL NOT COVER WHAT YOU BUILD.** hv's DO-NOT-BUMP measured `tests.status` as unconstrained TEXT -- a claim about the VALUE. `fiat` is a separate FIELD, so the AT kind cost a column, a rung and a bump.
- **2026-08-29 -- A PUBLISHED `///` IS A CONTRACT, AND TWO OF THEM WERE FALSE.** `FiatRecord`'s _kept forever_ was disproved by `ac_reinstate`; `AcceptanceTest::fiat`'s _present exactly when_ was disproved by `at_set`. Both derive `JsonSchema`/`SimpleObject`, so both were published verbatim into the committed faces. **One was fixed by correcting the prose and one by correcting the CODE, and which is which is the judgement.**
- **2026-08-29 -- THE COMPILER IS THE ORACLE FOR A FIELD ADDITION; THE HAND-ROSTERS ARE WHERE IT IS BLIND.** It could not see `model_laws.rs`'s proptest strategy, the SDL variant COUNT, or `fully_populated_row()`, **whose contract is that no field is measured against a `None` serde dropped -- and whose own doc names that trap for two earlier fields.**
- **2026-08-30 -- THE GATE WAS SHUT ON TWO INDEPENDENT COUNTS AND THE SECOND WAS INVISIBLE BEHIND THE FIRST.** Machine 5 was one; `rulings_check.sh` exiting 1 on 4 DANGLING records was the other, and it entered with hv's `0f41dce1` blanket checkin sweeping in uncommitted `dispatch-table.json` rows. **Measured on both sides of that commit rather than reasoned about** (`0f41dce1~1` rc=0/0 dangling, HEAD rc=1/4). **A second closure is invisible behind a first one by construction, so I re-measured the gate against HEAD after landing rather than calling it open because my own commit went through.**
- **2026-08-30 -- A GUARD COLUMN WAS MEASURING THE DOCUMENT'S PROSE HABITS, NOT THE CODE.** Nine Machine 3 cells held EFFECTS (`clears evidence first`) and LANDING RULES (`non-test -- lands on entry state`). Neither can be true or false of a transition being ATTEMPTED, so neither was ever checkable -- 3 DISAGREE and 6 UNMEASURED against a transcription that was correct all along. Both facts survive, stated ONCE in prose; **the copy that drifted was the repeated one.**
- **2026-08-29 -- TWO COMPOSERS IS THE CORRECTION, NOT A COMPROMISE.** hv said _one required composer_; the count that was wrong was never the NUMBER, it was the UNIT -- one per VOCABULARY (generated view vs list surface), not one per call site.
