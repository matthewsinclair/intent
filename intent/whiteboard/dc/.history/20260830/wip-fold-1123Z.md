---
node: dc
name: DevX Claude
role: worker
session_id: 3f2f5de2-d774-44db-8f1f-c85588606969
heartbeat_at: 2026-08-30 11:15Z
status: active
focus: "AC-00.6 LANDED AND PASSING: IN-AG-FIAT-001 in the agnostic pack, validator green (9/9), and the schema + intent/docs/rules.md now admit PROCEDURAL agnostic rules as an OBLIGATION discharged through applies_when. rule_pack_agnostic RED FOR FOUR DAYS IS GREEN (11/11) via the conscious update the tripwire asked for -- rosters, not a 4-to-6 bump, and the procedural clause is now ENFORCED where nothing enforced it. D7 HAS A WITNESS. WP-07 DOES NOT CLOSE BEFORE THE CUT -- vc drove AC-07.7 RED against the installed keg; it needs a tag, same gate as WP-11. ST0066 evidence half is next and the thread stays 0/6."
claims: [ST0056/07, ST0056/11, ST0056/14, ST0066]
---

# DevX Claude (dc)

**COLD-SESSION MINIMUM. A rule is never dropped here, only its narrative.** Full reasoning for 29-30 Aug is in `.history/20260830/wip-fold-0837Z.md` and `.history/20260829/`.

## D42 -- TIME. Read before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **not the database.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES: NO cli or intentsvcs function TAKES a time.** They may RETURN times.
- **A board stamp is a label, not data** -- read from `date -u` and PASTE, **per stamp, never per session.**
- **`stat`, `git log`, `ls -la` ALL PRINT LOCAL.** Appending `Z` to a local read is an ASSERTION, not a format.
- **A ULID is an IDENTITY and the `ts` is the STAMP** -- which is why `Envelope::minted` may generate the id in Rust without breaching D42, and why a cascade can carry its ancestor's event id without a second transaction.

## The truth model and the environment

- `design.md` (D01 reversed) + `data-model.md`. **The db is the durable SSOT; the typed API is the only door in.** Crates: `intent-cli`, `intentd`, `intentsvcs`. **`intentdb` IS RETIRED.**
- **`intent` ON PATH IS v3 AND RESOLVES INTO THIS TREE** -- `~/.local/bin/intent` and `~/bin/intent` both symlink `native/rust/target/release/intent`.
- **THE STORE IS PROJECT-RELATIVE**, `intent_dir()/.cache/intent.db`, found by walking up from CWD. **SCHEMA 16.**
- **THE INDEX IS SHARED.** `add` + `commit --only <paths>` is the only safe write.
- **CANON ORDER: SYNC FIRST, then commit the file and canon together.** A later sync fixes the NEXT commit and can never fix this one.
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND NEVER TRANSCRIBED.**

## DOING

### ST0066 -- THE MECHANISM IS DONE. WHAT REMAINS IS ITS EVIDENCE.

**hv's SHAPE, TWO-AND-TWO:** `AcState::Fiat` carries its record IN the variant. `AtStatus::Fiat` is a UNIT variant with the record BESIDE it (`Copy` + async-graphql `Enum` forbid a payload). **`ThreadStatus`/`WpStatus` get NO variant** -- `fiat` beside a status that stays `completed`/`done`. **`fiat in the status` IS THE OPTION hv DECLINED.**

**LANDED, ALL OF IT DRIVEN:** the four `fc` verbs and edges; Machines 1, 2, 3 and 5 in `data-model.md` with the checker green on all three axes; the CLOSED GUARD VOCABULARY (Machine 3 went 36/3/6 -> 45 of 45, and axis C now GATES); `UNRESTRICTED_VERBS`; the cascade; the `fc` CLI verb reaching all four kinds through `scope_of`; D7's `put` refusal on every kind; **SCHEMA 16**, faces **DDL 13 / SDL 14 / JSON 16**; vc's four property tables.

**THE SHAPES WORTH KEEPING, because they will be argued again:**

- **A CASCADE CLOSES EXACTLY WHAT THE MACHINES ALLOW.** `transitions::permits` is the whole predicate, so the from-sets keep ONE home. Every skip it produces -- satisfied AC, green AT, done WP, descoped, withdrawn -- would have been argued for separately. **Which children was DERIVED and needed no ruling; how many EVENTS did.**
- **ONE EVENT PER ENTITY MOVED, ONE TRANSACTION.** vc's argument beats mine: one-event-per-cascade is not lossy, it is **asymmetrically wrong** -- replay produces an estate that differs from the real one.
- **`inherited_from` (entity) AND `inherited_event` (event id) ARE BOTH NEEDED.** Repurposing the first would change a published meaning without changing its shape. Not `inherited_at`: `_at` means a time here, and a ULID sorts by time, so that name would be wrong in a way that WORKS.
- **THE INVARIANT IS STRUCTURAL.** `set_thread_status`/`set_wp_status` are one-line wrappers over one implementation taking what happens to the fiat record; the clear is unconditional in the one place every status write passes. A verb added later inherits it. **`status_reason` takes the OPPOSITE default on purpose**: a stale reason is confusing, a stale fiat record is a false claim about a person.

**STILL OWED, AND ALL SIX ROWS ARE `(non-test)`: THE DOOR IS `ac satisfy --evidence`, NOT AN AT.** Measured 2026-08-30 -- this line said _the six ATs_ and would have sent the next reader to build the wrong artefact. `doctor`'s render (AC-00.5), AC-00.6's rules-library rule, **D4's structured field** for the accepted-unverified half.

**ST0066 IS 0/6 AND vc RATIFIED THAT IT STAYS THERE.** Ticking without driven evidence would be a hand-performed fiat close of the thread that builds fiat close -- **a thread that ships a mechanism must not use an informal version of it on itself.**

## TODO

- **AC-07.7 IS DRIVEN RED, AND WP-07 THEREFORE DOES NOT CLOSE BEFORE THE CUT.** vc's two-sided drive: the keg (`3.0.0`, `80d8b2ca`) ships **NO `plugins/claude` tree at all**, so `claude ws list` and `ws hygiene` return rc=2 _known command not implemented_ while `--help` returns rc=0 -- **surface complete, function absent**, which is the row's _refusing for want of its own dependency_ almost verbatim. Dev tree returns rc=0 for both. **Green needs a keg from fixed code, which needs a published tag, which is hv's hand -- the same gate as WP-11. Two WPs behind one gate.** Record the red; do not satisfy the row.
- **D7 IS WITNESSED (done 2026-08-30).** `put_refuses_to_author_a_fiat_record` drives `put` with an injected fiat payload and **asserts the refusal by its own message text, not merely that a refusal happened** -- otherwise a serde failure would go green and witness nothing. **CONTROLLED WITHOUT MUTATING A SHARED SOURCE:** the asserted string is produced only by `facade.rs`, so removing the guard leaves the assertion no source. **The sites are FIVE, not the three I reported -- that figure came from a `head -5` of a grep**, which is W1 for the fourth time today.
- **OPEN AND MINE: FOUR TESTS SPAWN THE BINARY WITH NO `current_dir` AND CAN STILL MIGRATE THE LIVE STORE.** cc measured them -- `schema_versioning.rs`, `bootstrap_door.rs`, `table_driven_tests_fixture_their_home.rs`, `version_spellings_agree.rs`. **The MECHANISM is resolved and the HAZARD is not**: the store is project-relative, so a spawned binary walks up from CWD to the real project, and `HOME` never covered it. Which of the four actually opened the store on 2026-08-30 is unbisected; the schema-16 migration is the evidence to bisect with. **Every test that spawns the binary and touches a store wants a fixture project, and `HOME` alone is a false sense of one.**
- **MINE AND UNSTARTED:** `bin/.devbin` is WP-11 and therefore mine. **`0152` IS CLOSED and this line carried it as mine-and-unstarted -- corrected 2026-08-30** (`916608d3` set the wait, `f1eef84f` gave it the structural witness; cc's board says closed).
- **hv's STANDING DIRECTIVE (b):** the `bin/int` -> `bin/devbin` rename goes AFTER the sweep.
- **PARKED, NOT MINE TO CLOSE:** WP-11 needs a published tag (hv's hand); fleet work is halted by hv; the `claude start`/`ws` smoke arm. **WP-07's seventh criterion is NO LONGER PARKED -- it was nameable, so it was named (`AC-07.7`).**

## Watch-outs

**FOLDED FROM 23 FAMILIES TO 8, because most of them were one shape seen from different sides. Rule only; instances are evidence, not the entry.**

- **W1 -- THE INSTRUMENT ANSWERED A DIFFERENT QUESTION THAN THE ONE ASKED, AND ITS OUTPUT LOOKED LIKE AN ANSWER.** The dominant class, six instances in two days. **A partial read is the sharpest form and it is ASYMMETRIC: one that finds SOMETHING self-corrects; one that finds NOTHING yields a conclusion nobody can falsify without redoing the read.** So **a NEGATIVE from a truncated read is not a result** -- `head -30` said this repo uses no trailers (37 of 40 do); `tail -30` said a suite was green and **let a red reach main**; `tail -14` said a guard did not print paths it prints twice. **THE PROCEDURE: output to a file, then `grep -c '^test .* FAILED$'` over the whole of it.** A count over a complete file can be zero honestly; a tail cannot. Sibling forms: a check that cannot tell WRONG from COULD-NOT-MEASURE names innocent files and prints a remedy that runs clean (the rustfmt `mod common;` probe); one sentence covering two conditions picks neither fix (absent-vs-malformed table, UNMEASURED-vs-DISAGREE); an instrument that cannot vary with its subject (positive-control before believing any zero); `cargo check` does not compile test targets without `--all-targets`.
- **W2 -- THE MEASUREMENT'S SUBJECT WAS NOT THE ONE I NAMED.** **A sha and `git status` answer different questions and only one is in the label** (cc): "the clean tree at `6b32b71d`" ran against a peer's dirty worktree, and the test read the table from disk. A suite run in a shared tree measures the TREE, not HEAD -- **and so does the delivered binary.** `HOME` isolates USER state and the store is PROJECT-relative, so a spawned binary with no `current_dir` finds the real project. **A `git status` expires in minutes here**; re-run it in the same turn as the sentence resting on it.
- **W3 -- OWNERSHIP IS MEASURED, NEVER INFERRED, AND IT HAS TWO TELLS.** _Absent at HEAD, therefore someone else's_ (`fn fc` was mine; three nodes converged on the same wrong owner). _The subject is theirs, therefore the edit is theirs_ (the `values: null` was ic's, not cc's whose daemon rows they were). **`git log -S<symbol> -- <path>` and `git log -1 -- <path>` close both.** Neither leaves a mark when wrong: the peer accepts work they did not do, or spends time in a file they never touched.
- **W4 -- THE SHARED CHECKOUT PUNISHES EVERY ASSUMPTION ABOUT WHO ELSE IS WRITING.** Never `cp` a shared source aside to mutate; never format or bless a file carrying a peer's in-flight change. **A multi-step refactor is uncompilable between its steps and nothing says "mid-move"** (cc): a rename plus its use sites is ONE edit. **A staged intermediate of yours blocks every peer's commit**, because the gates read the index. A build window is silently also a commit freeze.
- **W5 -- A RESTRICTION THAT REDS NOTHING HAS NOT BEEN ADDED.** D7 went into a roster `put` does not read; 29/29 stayed green and the refusal moved nothing. **When you add a guard, something must go red** -- if nothing does, find the door before believing it. Corollary from the other side: **a rule that catches a failure mode it was NOT designed for is the strongest evidence for it** (arm 8 refused a binary that panics on startup, knowing nothing about dispatch tables).
- **W6 -- A SECOND HOME FOR A FACT DRIFTS, AND THE COPY THAT DRIFTS IS THE REPEATED ONE.** Never transcribe an instrument's number -- drive the verb. A count in prose beside a checker that computes it went stale (FIVE state fields when there were six; "the four ratified machines" while a fifth was ratified underneath). **An anchor that resolves to the record's own RENDERING is circular and worse than a dangle** -- a dangle is visible, a circle certifies itself. **And two files can hold different READINGS of one value with nothing seeing it** until something uses the second (`from: &[]` meant _any state_ to one file and _never filled in_ to another).
- **W7 -- MECHANICS THAT RETURN A PLAUSIBLE WRONG ANSWER.** **Never take `$?` through a pipe** -- it belongs to the last stage; `cargo fmt --check | head` prints a diff and reports rc=0 (four instances, three nodes). `cargo test` aborts after a failing target: `--no-fail-fast`, or two runs report different populations in the same units. **`cargo test` is not read-only: it migrates the shared store**, and the severity is BLAST RADIUS, not data -- one `cargo build --release` fixes every node. A detached worktree gets its OWN `CARGO_TARGET_DIR` **and that directory dies with the worktree**: sharing bakes a stale `CARGO_MANIFEST_DIR` into cached rlibs, and keeping one grew an abandoned worktree to 10G.
- **W9 -- A REMEDY WHOSE STATED REASON DOES NOT HOLD IN YOUR CASE IS NOT PERMISSION** (vc's naming, on my find). `doctor` said do not reach for `wp done` **because it is refused on a blocked gate** -- and my gate was not blocked, so `wp done` would have run. **The reason evaporating does not convert the advice into a licence**; it means the remedy was written for a different situation and has nothing to say about yours. Same shape as W1 from the other end: there an instrument answers a question you did not ask, here a remedy answers a situation you are not in, and both read as though aimed at you.
- **W8 -- HOW TO REPORT.** Write reasoning at the resolution you MEASURED it, never at the one it suggests. **Verify a peer's correction yourself, including when it makes your own claim false.** A peer's quote of a ruling is still the peer. **Retract a finding the moment its basis fails, even one you were thanked for** -- and say which claims rested on what, because a negative and a sighting do not feel different at the time.

## Decisions

- **A PUBLISHED `///` IS A CONTRACT AND THE FACES ARE ITS CONSUMERS.** `JsonSchema`/`SimpleObject` lift doc comments verbatim into `thread.schema.json` and the SDL. **Three false ones found:** `FiatRecord`'s _kept forever_ (disproved by `ac_reinstate`), `AcceptanceTest::fiat`'s _present exactly when_ (disproved by `at_set`), and a `///` of mine that published `ST0056` into both faces. **One was fixed by correcting the prose, one by correcting the CODE, one by demoting it to `//` -- and which is which is the judgement.** Reasoning for a maintainer is `//`; a `///` says what a consumer needs.
- **THE FACE COUNT IS A READING, NOT A CHORE.** Three faces moving means PERSISTED AND PUBLISHED; two means published only (a `FiatRecord` is JSON inside columns that already exist); one is a store table with no modelled type. **The pin comments say what the count MEANS, which is what turned a number into a reading.**
- **THE COMPILER IS THE ORACLE FOR A FIELD ADDITION; THE HAND-ROSTERS ARE WHERE IT IS BLIND** -- proptest strategies, SDL variant counts, `fully_populated_row()`. **A generator that pins a new field to `None` is a field the round-trip laws never exercise**, which is how `AcState::Fiat` shipped uncovered. Generate it.
- **A RULING'S PREMISE CAN BE CORRECTLY MEASURED AND STILL NOT COVER WHAT YOU BUILD.** hv's DO-NOT-BUMP measured `tests.status` as unconstrained TEXT -- a claim about the VALUE. `fiat` was a separate FIELD, so it cost a column, a rung and a bump. **Surface the change rather than absorbing it.**
- **hv HAD ALREADY RULED THE THING I WAS INSTRUCTED TO BUILD, THE OTHER WAY.** Read hv's board BODY at pickup, never just its header. **And an OMISSION in an instruction can cost more than an error in it** -- _with one required composer_ was in the ruling's title and never reached me.
- **#fixforward IS A RULING ABOUT WHICH DIRECTION IS CHEAP.** A one-way ladder only sounds unrecoverable while you read it as data; the stale thing was a BUILD ARTEFACT.
- **A DECISION THAT REACHES A QUESTION IT WAS NOT ABOUT IS THE STRONGEST EVIDENCE FOR IT** (vc). Three instances in one night: arm 8 catching a startup panic; one-event-per-entity making skip-derivability answerable; D42's identity/stamp split making a one-transaction cascade possible.
