---
node: dc
name: DevX Claude
role: worker
session_id: 3f2f5de2-d774-44db-8f1f-c85588606969
heartbeat_at: 2026-08-29 22:32Z
status: active
focus: "ACTIVE, BOUNCED AND HOLDING FOR vc. FIRST ACT ON THE BOUNCE WAS THE RE-MEASURE MY OWN HANDOVER ASKED FOR, AND IT CAUGHT A RED: `intent-cli` rc=101 -- `no_shipped_command_answers_from_an_unmigrated_project`, offender `intent surface retired` answering rc=0 over an unmigrated project. IT IS ic`s, ATTRIBUTED BY THE DIFF`S SUBJECT AND NOT BY PRESENCE IN `git status`: the test file is unmodified at HEAD and `retired_and_unreachable` does not exist there. Reported to ic and vc, touched nothing. `intentsvcs` rc=0. AND THE 346 IS NOT A SURVEY -- cargo aborted, 54 of 58 integration targets ran. MY OWN WORK IS UNCHANGED AND UNCOMMITTED: ST0066`s AC kind ships end-to-end; next and unblocked is the ST/WP/AT unit `Fiat` variant + `fiat: Option<FiatRecord>`, then the cascade."
claims: [ST0056/07, ST0056/11, ST0056/14, ST0066]
---

# DevX Claude (dc)

**THIS FILE IS THE COLD-SESSION MINIMUM.** Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260829/`: `wip-fold-2136Z.md` (the full pre-fold board), `watch-outs-full-2136Z.md` (the unabridged families), `landed-and-decisions-pre-0828.md` (landed work + decisions to 2026-08-27), and the earlier folds beside them.

**I wrote a supersedes banner here at the 22:23Z fold and deleted what it superseded instead**, which is `restart.md`'s standing instruction and the reason that file exists.

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** [...]
- **`stat` PRINTS LOCAL. `git log` PRINTS LOCAL.** Convert at the SOURCE and keep the local value beside it [...]

- **CORRECTED, BY MY OWN EXPERIMENT: `cargo test` DOES NOT WRITE `~/.intent/home`. THE WRITER IS CLOSED (cc `9c2ba9ed`) AND MY CAUSAL CLAIM WAS A SPURIOUS CORRELATION.** I reported watching the pointer go from a deleted worktree to healthy seven seconds after running the suite [...]
  - **THE SHAPE IS THE ONE I SPENT THE DAY CORRECTING IN OTHERS: before-state, my action, after-state, with an unmeasured third party in between.** [...]

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **`intent` ON PATH IS v3.0.0 AND RESOLVES INTO THIS TREE. `intent3` IS NOT ON PATH AT ALL.** Measured 2026-08-27: both `~/.local/bin/intent` and `~/bin/intent` are symlinks to `Intent/native/rust/target/release/intent` [...]
- **`intent3` NOW REFUSES A BINARY THAT CANNOT BE SHOWN TO DESCRIBE THIS TREE** (hv ruled 2026-08-24) [...]
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED** -- `intent ac status ST0057` [...]
- **hv's FREEZE SCOPE (2026-08-24): Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.** A v3-only defect is a v3-only fix [...]
- **THE INDEX IS SHARED IN THIS CHECKOUT.** `git add` puts your file where a peer's bare `git commit` sweeps it [...]
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** To ask about another clone, run ITS `bin/int`.
- **A DEVBIN COMMAND RESOLVES ITS PROJECT FROM SOMETHING OTHER THAN YOUR CWD -- BUT THE v3 BINARY RESOLVES FROM CWD.** Both are true and confusing them cost a live incident today.

## DOING

### ST0066 -- THE AC KIND SHIPS END-TO-END. ST / WP / AT ARE RULED AND UNBUILT.

**RE-MEASURED ON THE BOUNCE, 2026-08-29 22:31Z, AND THE FIGURES MOVED: `intentsvcs` rc=0 (1150 passed / 0 failed); `intent-cli` rc=101 (346 passed / 1 failed). THE RED IS ic`s `surface retired`, NOT MINE -- see ROUTED below. ALL MY WORK IS STILL UNCOMMITTED.** **AND THE `intent-cli` FIGURE IS NOT A SURVEY EITHER WAY: cargo aborted at the failing target, so 54 of 58 integration targets ran and four are UNMEASURED rather than green.** Re-measure both rather than trusting any of this, and read the EXIT CODE first -- an ok-count cannot tell a green suite from one that stopped, which is exactly what the 61 recorded here at the fold would have let me believe an hour later.

**`git status` IN THIS CHECKOUT SHOWS FOUR NODES' WORK AND NAMES NO OWNER, so a cold session cannot read it as a list of its own edits.** Measured at this fold: `intent-cli/src/spine.rs` carries ~146 added lines about RETIRED-row enumeration -- that is **ic's ST0058 AC-00.5**, not mine, and I nearly handed it over as mine. **The discriminator is the diff's SUBJECT, never the file's presence in `git status`.** Mine are the `fiat`/`fc` changes: `facade.rs`, `model.rs`, `store.rs`, `transitions.rs`, `event.rs`, `render.rs`'s `fc` arm, four test files under `intentsvcs/tests`, three under `intent-cli/tests`, and the untracked `fiat_close_is_stamped_by_the_database.rs`.

- **BUILT:** `ac.fc` entry edge (`Guard::ReasonRecorded`, from `computed` + `unsatisfied` ONLY), the `fiat` orphan retired, `Facade::ac_fc`, `FacadeError::AlreadyFiatClosed`, `Invoker::collected()`, `KNOWN_OPS`, the D42 stamp channel, the `fc` CLI arm, and 0137's `ac list` render. `data-model.md` Machine 3 at 16 rows / 16 edges.
- **DRIVEN, not asserted:** `intent fc <ST> <AC> --because "..."` -> `ok: <AC> fiat-closed`, record in canon with `at` DB-stamped to the millisecond; no `--because` refuses rc=1 writing nothing; a second close refuses rc=1 naming `ac reinstate`; `ac gate` reports `0/1 satisfied, 1 fiat-closed -- PASS`.
- **NEXT, AND UNBLOCKED -- vc's RULING ON MY MEASUREMENT:** unit `Fiat` variant on `ThreadStatus` / `WpStatus` / `AtStatus` + `fiat: Option<FiatRecord>` on the entity with `skip_serializing_if`, then the cascade with its inherited marker. **The decider is hv's own DO-NOT-BUMP, priced at _zero of 170 extracts until an FC happens_: a PAYLOAD variant rewrites every entity's status field immediately, so the `AcState` mirror would not cost more -- it would INVALIDATE the ruling that said it was free.** Those three derive `Copy` AND async-graphql `Enum`; `AcState` derives neither. Close the state/evidence consistency both ways with an invariant in `ac_kind_state_invariant.rs`'s shape.
- **STILL OWED:** `doctor`'s render (AC-00.5 -- `doctor.rs` contains no `fiat` in any case), the six ATs, and AC-00.6's rules-library rule (hv to say whether it ships in 3.0.1 -- it is the only one touching consumer canon).
- **OPEN, AND THE BOARD SAYS I BUILD AROUND IT:** minutia 3 -- the accepted-unverified half as free text in `because` or a structured field.
- **THE `ac list` SPELLING IS ic's TO RATIFY.** I used this line's own vocabulary (`fiat-closed: <why>`, cascade marker leading) because its neighbours are `descoped-to:` and `withdrawn:`. The census arm asserts the PROPERTY, so any spelling ic picks passes.

### HELD -- NOT MINE TO START

- **0136 lands AFTER v3.0.1; vc calls one-commit-or-split at the cut** (we both lean split).
- **0137 IS CLOSED (2026-08-29, dc) AND ITS DEMOTION IS THE LESSON.** It was parked on a census of ZERO fiat rows store-wide, watched with that census as its trigger. **The trigger fired the moment `fc` could write one** -- a defect whose only defence is that nothing can reach the state stops being defended by the change that reaches it, so that change owes the fix.
- **OWED AND DELIBERATELY UNWRITTEN: no smoke arm exercises `claude start`/`ws`.** ARM 4 proves the rule library ARMS; there is no equivalent for this door, so **present is the strongest claim the keg fix earns.** **CORRECTED 2026-08-29 BY MY OWN MEASUREMENT: IT DOES NOT NEED A KEG, AND "needs a keg" IS WHAT PARKED IT FOR A WEEK.** `intent-cli/tests/critic_refuses_an_empty_library_end_to_end.rs` already has the shape -- `fixture_install()` builds a keg-SHAPED tree in a tempdir with the binary **COPIED, NEVER SYMLINKED** (`install::resolve()` canonicalises the exe before walking for the `lib/templates` marker, so a symlink resolves back and the fake root is silently ignored). **AND THE TRANSITIVE-CLOSURE HOLE I EXPECTED IS NOT THERE: `intent_claude_cwi:79` SOURCES NOTHING, in its own words**, so the FILE-level `SUPPORT_PATHS` entry is sufficient for this door and `claude_plugin_helpers.sh:84`'s `ext_root_dir()` is a different file off this path. **S, red-first available by omitting the door.** vc sequences it.
- **0141 is ic's.** ST0068 is vc's; hv has answered all four decisions and `docs/` is the v3 set, `docs/v2/` frozen.

### PARKED -- LIVE, HELD, NOT MINE TO CLOSE

- **WP-11 CANNOT MOVE WITHOUT A PUBLISHED TAG.** AT-11.1/11.2/11.4 are `n/a` pending one; publish stops with hv.
- **FLEET WORK IS HALTED BY hv (2026-08-26 11:53Z) AND hv LIFTS IT, NOT vc.** My four are dirty and uncommitted.
- **RETIRED FROM THIS SECTION 2026-08-29, both by vc's measurement, both were me replaying a ledger:** `ac gate ST0057` (hv's board already reads PASS 66/66) and `AT-07.4` (already green, flipped 2026-08-28). **A handover outlives its subject -- re-measure, never replay.**

## TODO

### HELD BY A RULING, AND THE PATCH BELOW DIES WITH THE SESSION IF IT IS ONLY IN scratchpad

- **hv's STANDING DIRECTIVE (b): THE FIVE-STEP `bin/int` -> `bin/devbin` RENAME GOES AFTER THE SWEEP.** Nothing is deleted and no intermediate state is broken -- hv ratified `bin/int` as the optional shortcut [...]
- **STEP 1 INLINED HERE BECAUSE `scratchpad/` DOES NOT SURVIVE THE SESSION.** It applied cleanly at close-out [...]

  > THE DISPATCHER IS ADDRESSED BY ITS OFFICIAL NAME, `bin/devbin`, AND NEVER BY A PER-PROJECT SHORTCUT. hv ratified the convention 2026-08-27: the shim in a project is always `bin/devbin`; a project MAY add a 2-3 character symlink (`bin/int` here) for humans; **tools and process always use the official name.** This file is tooling, so it takes the official one. `bin/int` keeps working for people.

- **RETIRED FROM THIS SECTION: `decisions-surface.patch` and `whiteboard-clock-guard.BUILT.sh`.** Both landed (`27b13f93` and follow-ups) [...]

### LIVE, MINE, UNSTARTED

- **ST0066 IS 0/6 AND THAT FIGURE IS HONEST -- THE SEVEN COMMITS BUILT THE SUBSTRATE AND NOT THE FEATURE. Measured 2026-08-29 on vc's ask, so no later reader re-derives it.** `bf8cc1af`'s own message is the finding: _representable is not the same as reachable_.
  - **AC-00.1 NOT REACHED and it is the load-bearing one:** `intent fc --because x ST0066` -> `rc=1`, `error: unrecognized subcommand 'fc'`. There is no verb.
  - **AC-00.2 model half BUILT, unreachable:** `FiatRecord` at `model.rs:1138` (`because` under `#[schemars(length(min = 1))]`, `by`, `at`, `invoker`), serde covered. **ZERO `AcState::Fiat` constructions in `facade.rs` / `transitions.rs` / `preconditions.rs`** -- which is WHY the census found 0 fiat rows in 416, and the census was measuring an unbuilt writer rather than an unused feature.
  - **AC-00.3 field BUILT, cascade NOT:** `model.rs:1157` / `:1196`, `graphql.rs:116,164`. Nothing cascades because nothing closes.
  - **AC-00.4 substantially covered ON SYNTHETIC DATA:** `views.rs:579` + `fiat_close_is_visible_on_every_surface.rs`, five arms with BOTH controls. 0137 is an open hole in this exact criterion.
  - **AC-00.5 NOT REACHED:** `doctor.rs` contains no `fiat` in any case.
  - **AC-00.6 NOT REACHED:** no rule under `intent/plugins/claude/rules/` or `lib/templates/` mentions fiat close or `fc`; the lone grep hit is coincidental.
  - **AND THERE ARE NO ATs AT ALL** -- `intent at list ST0066` prints nothing at `rc=0`, every `covered-by` empty. **So ticking these would be a hand-performed fiat close of the thread that builds fiat close.** Remaining: verb, writer, cascade, doctor render, rules-library rule, six ATs. **M-to-L.** AC-00.6 touches consumer canon, so hv says whether it ships in 3.0.1 or follows.

- **ST0056/WP-07: MY READ IS THE CONTRACT IS SHORT, NOT THAT THE WORK IS UNFINISHED -- and the gate passes honestly.** All six AC-07.x are satisfied with green ATs and I dispute none. **But all six are canon/parity MECHANISMS and not one names the `claude` subsystem's own doors** (`claude ws new|list|archive|hygiene`, `claude start`) though the WP is titled "Canon and claude subsystem". **This is not theoretical: the 3.0.0 keg shipped the cwi door missing and nothing in WP-07 went red, because no criterion covers it** -- and my smoke gap has no row to attach to for the same reason. **Proposed to vc for hv: a seventh criterion covering those doors, which turns doctor's finding from a status argument into a build item I own.** Not mine to rule; `wp done` is the wrong answer either way.

- **THE KEG SHIPS NO RULE LIBRARY AND NO SKILLS. THE CODE FIX IS DONE (`0112b8c1`) AND IT IS NOW NAMED IN vc's 3.0.1 CUT LIST, SO THE SHIPPING HAS A DATE.** [...]

- **FLEET WORK IS LIVE AND HALTED BY hv (2026-08-26 11:53Z, relayed devbin-cc -> vc). MY FOUR ARE DIRTY AND UNCOMMITTED -- see DOING for the exact table. hv LIFTS IT, NOT vc.**
- **FLEET WORK UNDER vc's RUNBOOK IS THE TOP ITEM AND hv AUTHORISED vc TO DIRECT IT.** `intent/whiteboard/vc/cutover-runbook.md` [...]

- **WP-07's `cwi` PORT IS DONE (`1ad284b3`) AND IT DOES NOT DISCHARGE `AC-14.12` -- WRITTEN HERE SO NO LATER READER COLLAPSES THEM.** WP-14 retires the VERBS (served from the store per `AC-14.7`) [...]

- **THE `bin/` COUPLING HAS AN EDGE NO SWEEP FOR `source` CAN SEE, AND IT NOW LIVES IN CANON RATHER THAN ON THIS BOARD.** `intent/plugins/claude/lib/claude_plugin_helpers.sh:84` CALLS `ext_root_dir()` [...]

- **THE hv QUEUE: FIVE LIVE, NONE BUILT, ALL TOUCHING INSTRUMENTS THAT GATE OR DESCRIBE THE BUILD.** (1) **roster symmetry** -- `runner_roster_check.sh` reads PRESENT from the COMMIT and ROSTERED from the WORKTREE [...]
- **THE ROSTER SYMMETRY FIX -- FOUND BY ME, QUEUED FOR hv, AND EXPLICITLY NOT MINE TO BUILD AT SPEED.** Read ROSTERED from the COMMIT as PRESENT already is [...]

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` (AT-11.7) has a green to reach [...]
- **THE ATTRIBUTION GUARD -- RULED IN BY hv IN vc's SESSION, AND I AM HOLDING FOR hv's WORD IN MINE.** One arm [...]
- **NEW, AND IT IS MINE BECAUSE I FOUND IT IN MY OWN FILE: NOTHING VERIFIES THAT A ROSTER ROW DESCRIBES WHAT ITS RUNNER DOES.** `runner_roster_check.sh` verifies row-to-file EXISTENCE in both directions and is structurally blind to the row's CLAIM.

### ROUTED, MEASURED, NOT MINE TO TAKE

- **THE SHARED TREE CAME BACK RED ON THE BOUNCE AND IT IS ic`s, MEASURED 2026-08-29 22:31Z.** `cargo test -p intent-cli`rc=101, one failure:`no_shipped_command_answers_from_an_unmigrated_project` (`unmigrated_surface.rs:358`), offender `intent surface retired` at exit 0 over an estate it cannot see. **ATTRIBUTION BY THE DIFF`S SUBJECT, WHICH IS THE DISCRIMINATOR THIS BOARD RECORDED AT THE LAST FOLD:** `unmigrated_surface.rs` is unmodified at HEAD and `retired_and_unreachable` is absent from HEAD, so the red arrives with ic`s uncommitted verb. The sweep builds argv from `arg.values.first()`, so ic`s new `values: ["retired"]` on the `surface` row constructs the invocation itself. Likely home is `exempt_from_the_migration_refusal:62`, same category as `schema` / `llm guide` / `rules` -- **but whether exempt is right at all is ic`s contract, not my call, and I said so rather than proposing the patch.** Reported to ic and vc; nothing touched. **AND THE POPULATION IS SHORT: cargo aborts after a failing target, 54 of 58 integration targets ran, and `upgrade_command`/`verbosity_flags`/`version_spellings_agree`/`view_single_writer` are UNMEASURED rather than green -- F11, met in the wild within four minutes of reading it.**

- **WITHDRAWN: "MAIN IS RED ON `cli_end_to_end`" IS FALSE. THE WORKTREE WAS RED AND IT IS cc MID-EDIT.** Caught by vc [...]
  - **A SUITE RUN IN A SHARED TREE MEASURES THE TREE, NOT HEAD -- and my instinct to check the test file's commit history was correct, careful, and aimed at the wrong object.** [...]
  - **AND `git diff` LOOKED CLEAN WHILE THE TREE WAS DIRTY, because cc's edits are STAGED** -- the first status column [...]

- **ISSUE `0086` (HIGH, ic's find, QUEUED NOT ASSIGNED): `intent --help` rc=0 while `intent help` rc=2 `retired`, with a remedy claiming no replacement exists.** [...]

- **`VIEW_NAMES` (`address.rs:357`) CLAIMS A COUPLING THAT IS NOT IN THE CODE -- ANSWERED, FILED AS ISSUE `0087` (low, vc, `4f9ce518`), NOT MINE TO FIX.** [...]
- **ISSUE `0085` -- THE ADVISORY HOOK FIRES ON EVERY WRITE REGARDLESS OF FINDINGS.** `[ -z "$findings" ]` can never fire: both binaries always put `critic:` header lines and an `ok:` line on STDOUT [...]

- **THE FROZEN-`$INTENT_HOME` MECHANISM: THE DETECTOR HALF IS CLOSED, THE ROUTING IS NOT.** vc landed the ref fix and the CI arm (`a38e884b` [...]
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named, two commands produce it and nothing runs them [...]

## Watch-outs

**ELEVEN STANDING FAMILIES, RULE ONLY. The full text -- every instance, every measurement -- is `.history/20260829/watch-outs-full-2136Z.md`. A rule is never dropped here, only its narrative.**

- **F0 -- THE SHARED CHECKOUT AND THE BLIND INSTRUMENT.** The index is shared: `git add` puts your file where a peer's bare commit sweeps it, so `add` + `commit --only <path>` is the only safe write. A suite run in a shared tree measures the TREE, not HEAD. `git diff` reads clean while the tree is dirty when a peer's edits are STAGED.
- **F1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT.** Positive-control the instrument before believing a zero: a sample that cannot exhibit the failure, a pattern that cannot match the subject, a control that would pass under the broken instrument too. Green arms prove nothing about whether your change took effect.
- **F2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF CARRIES THE FALSE ONE.** State what you measured, not what it suggests. Stopping early is a LIMIT; publishing the stopping point as a general claim is a CHOICE.
- **F3 -- ROUTING, RELAY AND ATTRIBUTION.** A peer's quote of a ruling is still the peer. Route by owner, and say who measured what.
- **F4 / F0b -- THE SHARED CHECKOUT, WRITES.** Never `cp` a shared source aside to mutate; only a detached worktree sees a broken published tree; a reflog check precedes any reset of a shared HEAD.
- **F5 -- THE GATE THAT DID NOT RUN, AND THE VERB THE NAME SENDS YOU TO.** A commit is an indirect `intent` invocation whose two halves fail in OPPOSITE directions -- guards survive a missing binary in this self-hosted tree, the critic half refuses -- so a build window is silently also a commit freeze.
- **F6 -- AN UNMEASURED CLAIM INSIDE A COMPLIMENT.** Praise carries assertions; they are owed the same measurement as any other.
- **F7 -- A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR A FACT.** Drive the verb; never transcribe its number.
- **F8 -- THE ESTATE'S OWN CONFIGURATION IS WHAT HIDES ITS BUGS FROM IT.** The setup that makes your estate work is the reason your estate cannot see the defect.
- **F9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE.** Read the whole extent before claiming a property OF the whole: a function's second loop, a struct's lower fields, a board's body under its header. **And the sharpening, 2026-08-29 (ic's): both of my wrong reports concluded an ABSENCE. A partial read that finds SOMETHING is self-correcting -- you have the thing. A partial read that finds NOTHING yields a conclusion you cannot falsify without redoing the read, and it arrives in exactly the shape a complete read would.** A structural claim is owed a structural check, not a reading.
- **F10 -- HOW TO REPORT.** Write reasoning at the resolution you MEASURED it. Send the refuted version beside the true one. **Verify a peer's correction yourself before accepting it, including when it makes your own claim false.** Measuring what you were promised is right AND destroys the evidence the promise was broken -- so say both the value and why you had to go get it. Refusing a flattering account of your own error is rarer than finding the error.
- **F11 -- THE TERMINATING CONDITION IS AN UNDECLARED FILTER, ARRIVING THROUGH A NEW TOOL EACH TIME.** `cargo test` ABORTS after a failing target, so two runs report different POPULATIONS in the same units and an ok-count alone cannot tell a green suite from a stopped one. **Read the exit code FIRST and the FAILED count beside the ok count. Never take `$?` through a pipe** -- it belongs to the last stage.

## Decisions

- **2026-08-29 -- CORRECT YOUR OWN ESCALATION BEFORE IT REACHES THE PRINCIPAL, ESPECIALLY WHEN THE ERROR MADE YOUR ASK LOOK BIGGER.** I escalated the fiat exit as "hv ruled one verb and the invariant demands a second".
- **2026-08-29 -- WHEN AN INSTRUCTION ASKS FOR SOMETHING AN INVARIANT FORBIDS, MEASURE IT AND SAY SO; DO NOT BUILD THE HALF THAT PASSES TODAY.** [...]
- **2026-08-29 -- READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT.** I noted the `///`-is-published rule [...]

- **2026-08-28 -- A DEFECT'S FILED WIDTH IS NOT ITS REAL WIDTH, AND THE OBVIOUS FIX FOR THE FILED WIDTH CAN BE WRONG RATHER THAN MERELY SHORT.** `(k)` was filed on `languages: []` [...]

- **2026-08-28 -- A COST ESTIMATE IS A CLAIM AND IS OWED THE SAME MEASUREMENT AS ANY OTHER.** I told vc the AC-07.6 migration arm "needs a v2 estate fixture that does not exist yet" **without opening the file.** [...]

- **2026-08-28 -- A PARITY TOOL UNDER `intent/st/ST0056/parity/tools/` IS AN INLINE CANON ATTACHMENT, AND EDITING ONE HAS A COMMIT ORDER THE OBVIOUS SEQUENCE GETS WRONG.**
- **2026-08-28 -- NOT EVERY NEW TEST ARM SHOULD FAIL THE CONTROL, AND CLAIMING OTHERWISE WOULD BE THE FLATTERING LIE.** Driving the pre-fix code red 4 of 5 new arms [...]
- **2026-08-28 -- ZSH BIT THREE TIMES IN ONE SESSION AND EVERY ONE WAS ALREADY WRITTEN DOWN.** [...]
- **2026-08-28 -- ~~I CHECKED WHOSE THE RED WAS BEFORE REPORTING IT~~ STRUCK. I banked my own error as the day's lesson learned; cc refuted it and I verified the refutation myself.** [...]
  - **EXONERATION AND ATTRIBUTION ARE TWO CLAIMS AND I MEASURED ONE.** "My diff does not render the subject" establishes it is NOT MINE and says nothing about whose it is.
  - **AN EXPECTATION WHOSE PREMISE A LANDED DECISION RETIRED, WITH NOTHING CONNECTING THE DECISION TO THE EXPECTATION.** Three costumes: cc's `Super_Seded` control (stale GREEN) [...]

- (2026-08-28) **vc CORRECTED MY ROLLBACK FINDING AT THE RIGHT LEVEL AND IT GENERALISES: THE CARRIER IS DERIVED FROM A TRACKED TEMPLATE, SO THE GENERATOR IS WHAT YOU PRESERVE AND THE OUTPUT NEVER WAS.** [...]
- (2026-08-28) **I FILED THREE ISSUES WITHOUT RUNNING `intent issues list`. ONE WAS A DUPLICATE OF AN OPEN ISSUE; ONE WAS REFUTED BY MY OWN COMMIT OF THE DAY BEFORE, RECORDED IN THIS BOARD'S OWN DOING.** [...]
- (2026-08-28) **A TRUE POSITIVE DISCARDED AS FALSE COSTS AS MUCH AS A FALSE ONE BELIEVED.** `int hooks` called this tree's carrier STALE and was RIGHT [...]
- (2026-08-28) **THE REHEARSAL'S VALUE WAS IN THE THREE INSTRUMENTS I CHECKED, NOT IN THE STEPS I RAN.** A clone would have been blind to the hook-door question by construction [...]

**Decisions dated 2026-08-27 and earlier are archived at `.history/20260829/landed-and-decisions-pre-0828.md`; pre-08-25 at `.history/20260828/decisions-pre-0825.md`.**
