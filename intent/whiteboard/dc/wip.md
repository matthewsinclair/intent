---
node: dc
name: DevX Claude
role: worker
session_id: ff2a3ea4-b800-4f7e-8bcd-8dd01154cb5f
heartbeat_at: 2026-08-29 14:47Z
status: active
focus: "**ON THE KEG FOR THE 3.0.1 CUT (vc assigned), MEASUREMENT ONLY -- REPORTED, AWAITING vc.** FOUND: a v3.0.1 stage REFUSES today -- the copy list misses `intent/plugins/claude/bin/intent_claude_cwi`, the ONLY path `intent claude start`/`ws` resolve, and the SHIPPED whiteboard skill calls `ws new` the provisioner, so ST0068 would document a verb the keg cannot run. The door landed a DAY AFTER vc fixed the list, so this is vc's check working, not their defect repeating. Fix B (ship the one file, not the 7-script dir) driven end to end. Second finding: smoke ARM 3 claims the resolver and iterates the COPY LIST. CORRECTED BY vc: items 4 and 5 were ALREADY DONE and I was replaying a ledger; 0137 demotes (ZERO fiat rows store-wide). WAITING: 0136 after v3.0.1, ST0068 on hv's four decisions, 0141 is ic's."
claims: [ST0056/07, ST0056/11, ST0066]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260829/` -- `wip-fold-1423Z.md` is the full pre-fold board, `watch-outs-and-decisions-full.md` the unabridged rules. This file is the COLD-SESSION MINIMUM.**

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

**KEG MEASUREMENT FOR THE 3.0.1 CUT -- REPORTED TO vc, AWAITING THEIR CALL.** vc assigned it as a measurement of the cut, not a build. Nothing built, nothing staged, no file touched.

**A v3.0.1 STAGE REFUSES TODAY.** v3 resolves FIVE install-root paths; the copy list covers four. Uncovered: `intent/plugins/claude/bin/intent_claude_cwi` (`install.rs:361`). Driven with the REAL `support_paths_coverage` extracted from `cmd/macos`, two-sided (right path -> green, irrelevant path -> still red). **The door landed `68ca70bf` 2026-08-27 14:41Z, a day AFTER vc's copy-list fix `0112b8c1` 2026-08-26 15:28Z** -- so this is vc's check working, not vc's defect repeating. Nobody has staged since, which is why it has never spoken.

**THE DOCS COLLISION IS THE POINT, AND IT OUTRANKS THE INSTALL PAGE.** The uncarried path is the only thing `intent claude start` and `intent claude ws` resolve, and the SHIPPED whiteboard skill calls `intent claude ws new <node>` the provisioner -- **so ST0068 would document a verb the keg cannot run, in a skill that ships inside that same keg.** Not gated by hv's four decisions.

**FIX B RECOMMENDED AND FULLY DRIVEN:** add the single file, not the directory (which holds SEVEN scripts, six of them v2 residue -- shipping them contradicts hv's one-surviving-script ruling at `install.rs:352`). Verified end to end: coverage green, `support_tree_drift` clean with a file-level entry, tar emits the right path, **exec bit survives into the archive**.

**SECOND FINDING:** smoke ARM 3 claims _"every path v3 resolves"_ and iterates the COPY LIST -- it cannot catch a copy-list omission, which is the very class it names. No arm exercises `claude start`/`ws`.

Everything in ST0066/0133 landed earlier today; that reasoning is in the COMMIT MESSAGES and is not restated here. Pre-fold board verbatim at `.history/20260829/wip-fold-1423Z.md`.

### ST0066 / 0133 -- LANDED AND CLOSED, seven commits

- **`04cf6f18`** 0133: `Unsatisfied` carries its OWN payload `note` (NOT `evidence`); `legacy.rs`'s wildcard replaced by three explicit arms. **hv ruled both halves 2026-08-29**; `ac unsatisfy` KEEPS CLEARING and the separate field is why that is free.
- **`4a648831`** 0136: `deny_unknown_fields` is a property of the variant's SHAPE, not the enum. `computed` (the last unit variant) still swallows unknown keys. **Priced at ZERO and measured, not quoted.**
- **`72d716dd`** four stale prose claims of mine, three of them counts in one paragraph. **`9c220c1d`** the GraphQL projection list held to the SDL; the strictness arm says what it guards.
- **`b6a483df`** the fiat census + **`9a9e89b1`** 0137 + **`52d21945`** the composer doc corrected. **`bf8cc1af`** 0140.
- **SUITE: 1482 passed, 0 failed** at `070f8154`. Everything after is issue JSON and board prose.

### HELD -- NOT MINE TO START

- **0136 lands AFTER v3.0.1; vc calls one-commit-or-split at the cut** (leaning split, on my own evidence that a mechanical pass over this symbol corrupted three doc comments).
- **0137 is with hv** -- a vocabulary decision on a shipped surface, stated as two options rather than one smuggled in. **v3.0.0 is published and carries the defect.**
- **ST0068 (docs for v3) waits on hv's four decisions; vc routes.** **0141 is ic's** -- they found the wildcard and the scoping is theirs; I declined it twice.

### PARKED -- LIVE, HELD, NOT MINE TO CLOSE

- **`ac gate ST0057` returns PASS 66/66, 3 withdrawn; hv's board still says `BLOCKED -- 51/53`.** Routed to vc as pen-holder. `0123` independently reported the same 66/66, so the case that hv's line is stale is stronger -- **and it says nothing about where 51/53 came from**, which I am not claiming to explain.
- **AT-07.4's STATED REASON FOR RED HAS EXPIRED.** Set red 2026-08-19 because arm (b), THE REFUSAL, was uncovered; it is covered now -- `critic_arming_census.bats` arm 12 refuses at exit 3, **arm 13 is the present-tool half that makes 12 a test**, arm 14 checks v2 and v3 agree, driven 19/19. **vc set it red adjudicating my own refusal to set it, so vc flips it, not me.**
- **WP-11 CANNOT MOVE WITHOUT A PUBLISHED TAG.** AT-11.1 / 11.2 / 11.4 are all `n/a` pending one, and publish stops with hv by my own 2026-08-26 decision.

## TODO

### HELD BY A RULING, AND THE PATCH BELOW DIES WITH THE SESSION IF IT IS ONLY IN scratchpad

- **hv's STANDING DIRECTIVE (b): THE FIVE-STEP `bin/int` -> `bin/devbin` RENAME GOES AFTER THE SWEEP.** Nothing is deleted and no intermediate state is broken -- hv ratified `bin/int` as the optional shortcut [...]
- **STEP 1 INLINED HERE BECAUSE `scratchpad/` DOES NOT SURVIVE THE SESSION.** It applied cleanly at close-out [...]

  > THE DISPATCHER IS ADDRESSED BY ITS OFFICIAL NAME, `bin/devbin`, AND NEVER BY A PER-PROJECT SHORTCUT. hv ratified the convention 2026-08-27: the shim in a project is always `bin/devbin`; a project MAY add a 2-3 character symlink (`bin/int` here) for humans; **tools and process always use the official name.** This file is tooling, so it takes the official one. `bin/int` keeps working for people.

- **RETIRED FROM THIS SECTION: `decisions-surface.patch` and `whiteboard-clock-guard.BUILT.sh`.** Both landed (`27b13f93` and follow-ups) [...]

### LIVE, MINE, UNSTARTED

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

- **WITHDRAWN: "MAIN IS RED ON `cli_end_to_end`" IS FALSE. THE WORKTREE WAS RED AND IT IS cc MID-EDIT.** Caught by vc [...]
  - **A SUITE RUN IN A SHARED TREE MEASURES THE TREE, NOT HEAD -- and my instinct to check the test file's commit history was correct, careful, and aimed at the wrong object.** [...]
  - **AND `git diff` LOOKED CLEAN WHILE THE TREE WAS DIRTY, because cc's edits are STAGED** -- the first status column [...]

- **ISSUE `0086` (HIGH, ic's find, QUEUED NOT ASSIGNED): `intent --help` rc=0 while `intent help` rc=2 `retired`, with a remedy claiming no replacement exists.** [...]

- **`VIEW_NAMES` (`address.rs:357`) CLAIMS A COUPLING THAT IS NOT IN THE CODE -- ANSWERED, FILED AS ISSUE `0087` (low, vc, `4f9ce518`), NOT MINE TO FIX.** [...]
- **ISSUE `0085` -- THE ADVISORY HOOK FIRES ON EVERY WRITE REGARDLESS OF FINDINGS.** `[ -z "$findings" ]` can never fire: both binaries always put `critic:` header lines and an `ok:` line on STDOUT [...]

- **THE FROZEN-`$INTENT_HOME` MECHANISM: THE DETECTOR HALF IS CLOSED, THE ROUTING IS NOT.** vc landed the ref fix and the CI arm (`a38e884b` [...]
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named, two commands produce it and nothing runs them [...]

## Watch-outs

**ELEVEN STANDING FAMILIES. Every rule is still here; the NARRATIVE around each was cut at the 2026-08-29 fold and is verbatim in `.history/20260829/watch-outs-and-decisions-full.md`. `[...]` marks a trimmed body, never a dropped rule.**

**Standing means NOT ARCHIVED. It does not mean NOT REWRITTEN -- I read it as the second for weeks, and that is why this board reached 105KB while a peer's was 5KB.**

- **AND THE SAME SHAPE IN A FOLD, 2026-08-29: I CUT A SECTION AND LOST THREE LIVE ITEMS WITH IT.** An aggressive fold replaced `## DOING` wholesale [...]

### FAMILY 10 -- HOW TO REPORT (2026-08-29; mine, sharpened by ic, and vc RETIRED THEIR OWN ENTRY in favour of it)

**WRITE THE REASONING AT THE RESOLUTION YOU MEASURED IT, NOT AT THE RESOLUTION YOU BELIEVE IT.** A board of verified conclusions gives the next node nothing to plant a fixture against.

- **Send the REFUTED version alongside the true one.** The zero-resolution case of the rule above: a clean verdict with no reasoning attached is unfalsifiable by a reader. [...]
- **Stopping early is a LIMIT; publishing the stopping point as a confident GENERAL claim is a CHOICE.** I made it twice on one bullet; ic came within 20 minutes of filing a gap three-quarters already closed. **Same error, different surface area.** [...]
- **Every correction on 2026-08-29 landed on a claim whose reasoning was VISIBLE** (ic). The claims with no reasoning attached are still standing, and nobody knows whether that is because they are right. [...]
- **Refusing a flattering explanation of your own error is rarer than finding the error** (vc, on my declining ic's generous account of me). [...]
- **VERIFY A PEER'S CORRECTION YOURSELF BEFORE ACCEPTING IT, INCLUDING WHEN IT MAKES YOUR OWN CLAIM FALSE.** Twice on 2026-08-29: ic's `facade.rs:5181` find and vc's `machine_table_check.sh:314` gate. **That is the direction nobody audits.** [...]
- **A WIDENING CHANGES EVERY `==` ON THE TYPE**, including comparisons written long before the field existed. Refuted against my own 0133 by DRIVING it, not reasoning about it. [...]

### FAMILY 9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE (2026-08-29, mine, three instances in one build)

- **`cargo test` STOPS AT THE FIRST FAILING TEST BINARY.** Every count I gave vc during the ST0066 build came from a halted run reported as a total.
- **THIS IS THE HONEST-AND-BLIND CLASS ARRIVING THROUGH THE TOOL I TRUST MOST.** I spent the day demanding positive controls for greps and getting them right [...]
- **THE GENERAL FORM: an instrument that STOPS EARLY is not the same failure as one that CANNOT VARY (Family 1), and it does not respond to the same control.**
- **TREAT EVERY `N passing` I HAVE REPORTED AS A LOWER BOUND RATHER THAN A COUNT** unless the run carried `--no-fail-fast`.

### FAMILY 8 -- THE ESTATE'S OWN CONFIGURATION IS WHAT HIDES ITS BUGS FROM IT (2026-08-28, twice in one day, both mine)

**A fix applied by hand HERE removes the symptom and removes the ability to see the class.** Both instances were found only because a criterion said _in a consumer repo as well as this one_ [...]

- **`.prettierignore`** is 40 hand-written lines somebody sat down and wrote, so this repo has been immune since 2026-08-19 while every consumer stayed exposed [...]
- **`AGENTS.md` sections** render non-empty here because this project declares five languages that fill them [...]

**THE TELL IS A FIX THAT LIVES IN THE TREE RATHER THAN IN THE TOOL.** From inside, _immune_ and _unaffected_ are indistinguishable -- there is no local symptom to notice and no instrument that fires [...]

### FAMILY 7 -- A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR A FACT (2026-08-28 12:11Z, three instances in one day, two of them mine)

- **THE RULE vc APPLIED, AND IT IS THE KEEPER: WHEN A RECORD CARRIES ONE FACT IN TWO NOTATIONS, THE ONE THAT CAME OFF THE INSTRUMENT BEATS THE ONE A HUMAN RETYPED.** [...]
- **THE REMEDY IS STRUCTURAL AND BETTER THAN CARE: EMIT BOTH NOTATIONS FROM ONE CALL, so they cannot disagree.** devbin-cc's Phase 4 now prints `%Sp` beside the sha [...]
- **THREE INSTANCES TODAY AND THE OTHER TWO ARE MINE.** [...]
- **THE THROUGH-LINE, AND IT IS THE SAME ONE AS THE GATE FIGURE: A FACT WORTH RECORDING IS RECORDED AS THE CALL THAT PRODUCES IT, NEVER AS THE ANSWER IT PRODUCED.** [...]
- **AND A GAP I COULD NOT CLOSE, RECORDED AS UNCLOSED: I never captured the pre-hop carrier MODE** -- sha and size only [...]

### FAMILY 6 -- AN UNMEASURED CLAIM INSIDE A COMPLIMENT (2026-08-28 09:28Z, mine, refuted by cc)

- **I TOLD cc THEIR FIX WAS BETTER THAN THEY CLAIMED, AND THE REASON I GAVE WAS FALSE.** I said v1's `find | wc -l` and v2's NUL count differed such that a newline-in-path could inflate BOTH the file count and the row count together and pass.
- **THE DELIVERY VEHICLE IS THE CLASS, NOT THE ERROR.** The claim sat inside a compliment [...]
- **THE HONEST FORM OF WHAT I WAS REACHING FOR, and it favours cc: THE REDUNDANCY THEY LOST COULD NOT HAVE BEEN KEPT.** v1's second arm existed only because its counter was dishonest [...]
- **cc's FREEZE RULE, THEIRS AND BETTER THAN THE GROUND I OFFERED: ONCE A BASELINE IS TAKEN, THE INSTRUMENT IS FROZEN.** [...]

### FAMILY 0 -- THE SHARED CHECKOUT AND THE BLIND INSTRUMENT (2026-08-27, all measured, most of them on me)

- **`$?` AFTER A PIPELINE IS THE LAST STAGE'S STATUS, AND IT HIT FOUR NODES IN ONE EVENING WITH NO CROSS-TALK.** [...]
- **"INERT" IS DIMENSION-SPECIFIC AND THE DIMENSION YOU MEAN MAY NOT BE THE ONE THAT MATTERS.** [...]
- **DETECTION IS NOT DISCRIMINATION: A GREEN THAT IS INVARIANT UNDER THE THING IT CERTIFIES.** After landing tolerance 0 [...]
- **THE DAY'S CLASS, FOUR INSTANCES, FOUR NODES, ONE DAY -- AND IT SUBSUMES THE THREE ABOVE.** [...]
- **THE COMMIT IS THE ROLLOUT FOR ANYTHING READ LIVE OUT OF `INTENT_HOME`, SO THERE IS NO WINDOW IN WHICH TO ANNOUNCE FIRST.** Tolerance 0 was in force in fifteen estates the instant `3463f784` landed -- no sweep [...]
- **DRIVING AN INSTRUMENT TELLS YOU ABOUT THE WORKTREE AT THIS INSTANT, NEVER ABOUT THE RUN YOU ARE INVESTIGATING.** [...]
- **A TRUE GREEN FROM A BLIND INSTRUMENT IS WORSE THAN A FALSE ONE, BECAUSE IT GETS CITED.** My estate figure "0 of 5 hooks carry the arm" grepped `pre-commit` [...]
- **THE PARITY APPARATUS HAS FOUR ROOT CONTRACTS, TWO MUTUALLY INVERSE.** 7 honour `ROOT=` [...]
- **A CENSUS CANNOT REPORT THAT IT MEASURED THE WRONG PREDICATE.** My estate audit counted STRINGS (`102` in Laksa) and was structurally blind to the **7 sites that EXECUTE** [...]
- **PROBE-BY-EXECUTION IS SAFE ONLY IF THE WRITE TARGET IS REDIRECTED AND THE REDIRECT IS HONOURED -- AND NOTHING VERIFIES THE SECOND HALF.** Three of my gated instruments execute (`$BIN doctor` [...]
- **A RESTORE MUST RESTORE EVERY INPUT THE CONSUMER KEYS ON, AND THE CONSUMER DECIDES WHAT THOSE ARE (ic).** A metadata-preserving restore puts bytes back with the ORIGINAL mtime [...]
- **ADDITIONS MAY LEAD, REMOVALS MUST TRAIL (laksa-cc).** Laksa's suite went red at HEAD with nothing in `lib/` changed [...]
- **AN ANOMALY THAT SURVIVES A GOOD CORRECTION IS NOT NOISE -- IT IS THE PART OF THE SUBJECT THE CORRECTION DID NOT REACH.** cc's correction of my stale-hook claim was better measured than mine and right about the repair.
- **CONTENTION MANUFACTURES STAGED-AND-STRANDED.** The index was locked continuously for ~15 minutes [...]
- **THE AUTHORITATIVE STATEMENT WAS IN THE FILE, ON MY SCREEN, AND A BELIEF ABOUT THE SEAM OUTLIVED IT.** I told devbin-vc that Intent repointing `bin/int` would be undone by devbin on the next re-vendor [...]
- **AN OVERRIDABLE-LOOKING KNOB THAT IS NOT OVERRIDABLE REPORTS A PASS ABOUT THE WRONG SUBJECT.** [...]
- **THE THIRD FACE OF THE SAME FAMILY, AND IT COST THE MOST BECAUSE IT LOOKED THE BEST: A CORRECT INSTRUMENT, CORRECTLY DRIVEN, POINTED AT THE WRONG SUBJECT.** [...]
- **NEVER MUTATE THE LIVE SUBJECT. COPY IT, MUTATE THE COPY, RUN THE COPY, DELETE THE COPY -- SO THERE IS NO RESTORE TO GET WRONG.** [...]
- **A FLAG NAMED FOR SCOPE CAN DECIDE SEVERITY, AND GUESSING WHICH COSTS THE WHOLE ESTATE.** `thread_view_skew_check.sh --changed <paths>` does NOT narrow what is CHECKED -- all 288 views are examined either way and the cost is identical.
- **FIVE CROSS-PROJECT CONSTANTS WERE PROPOSED TODAY AND ALL FIVE FAILED ON MEASUREMENT -- AND THEY ARE NOT PROPERTIES OF CARGO, THEY ARE PROPERTIES OF HOW A TREE HAS BEEN BUILT OVER TIME.** [...]
- **CARRIED FROM vc 2026-08-27, NOT MEASURED BY ME, AND RECORDED BECAUSE vc BELIEVED MY BOARD ALREADY HELD THESE AND IT DID NOT.** Attribution matters here: these are vc's findings and I am the second carrier [...]
- **NEVER DELETE A TREE SOMETHING ON `PATH` RESOLVES THROUGH, AND CHECK BEFORE, NOT AFTER. THIS IS STEP 0 OF ANY BUILD-CACHE CLEANUP.**
- **WHEN THE FAILURE IS "MY BOUND WAS TOO SMALL", A BIGGER BOUND REPRODUCES THE CLASS WITH A DIFFERENT CONSTANT -- THE FIX IS NO BOUND, PLUS A PRUNE FOR SPEED.** [...]
- **INDEPENDENT DERIVATION IS NOT CORROBORATION WHEN THE INSTRUMENTS SHARE A SHAPE (vc's, 2026-08-27, and it is the sharpest thing either of us said).**
- **VERIFY THE THING THAT DEPENDS ON THE CHANGE, NOT ONLY THE THING BEING CHANGED (one entry, both ends, agreed with vc).** [...]
- **ENUMERATE TARGET DIRS FROM THE FILESYSTEM, NEVER FROM THE LAYOUT YOU EXPECT -- AND `.worktrees/` IS THE ONE EVERY SWEEP MISSES.** My estate sweep used `find -maxdepth 4 -name Cargo.toml` and returned SEVEN.
- **THE GUARD'S OWN REFUSAL REMEDY BUILDS AN UNBOUNDED CACHE NOBODY COLLECTS, AND IT IS INVISIBLE TO EVERY `target/debug` MEASUREMENT.** `releasebuild.lib:189` tells a refused caller `CARGO_TARGET_DIR=<dir> int build $verb` [...]
- **THE BUILD-CACHE SWEEP METHOD, WHICH IS THE PART THAT TRANSFERS WHEN NONE OF THE NUMBERS DO.** [...]
- **A ZERO AFTER A DELETE MUST BE CHASED, NOT REPORTED.** `bin/int cli ac list ST0056` returned zero rows straight after my sweep.
- **A COMMENT IS NOT INERT IN SHELL: A LINE OPENING `# shellcheck` IS A DIRECTIVE, AND A MALFORMED ONE MAKES THE LINTER STOP RATHER THAN CONTINUE.** [...]
- **A SURVIVING FILE THAT STILL DEPENDS ON `bin/` FAILS OPEN AFTER THE CUT, AT rc 0, WITH STDOUT THAT CAN BE BYTE-IDENTICAL TO THE CORRECT ANSWER.** [...]
- **THE CLASS I WAS THE COMMON FACTOR IN THREE TIMES IN ONE DAY (2026-08-27): A TRUE ANSWER TO A NARROWER POPULATION THAN THE QUESTION.** (1) `grep 'INTENT_ROOT/bin/'` -> 2 of 7 coupled files.
- **A WRONG-BUT-ADJACENT CORRECTION FROM A PEER IS A SIGNAL TO WIDEN THE CHECK, NOT TO WIN THE POINT (2026-08-27, vc).** [...]
- **A DEFENSIVE BRANCH WRITTEN TO STOP ONE FAILURE BECOMES THE THING THAT HIDES AN ABSENCE, AND THIS IS THE TRANSFERABLE HALF OF 2026-08-27's FINDING -- BETTER THAN THE BUG IT CAME FROM.**
- **THE BLIND GREP THAT RETURNS A TRUE NUMBER FOR A NARROWER QUESTION THAN THE ONE ASKED (vc's framing, banked 2026-08-27 in vc's terms because mine were worse).** [...]
- **`.prettierignore` IS GITIGNORE SYNTAX, WHERE A SINGLE `*` DOES NOT CROSS A `/`, AND A FENCE VERIFIED ONLY BY A GREEN IS NOT VERIFIED.** The obvious narrow list fences less than it looks like.
- **A FENCE VERIFIED ONLY BY A GREEN IS NOT VERIFIED.** [...]
- **A LOG LINE IS NOT A HEAD MOVE, AND ON A FIVE-NODE BRANCH THE NEWEST COMMIT IS USUALLY NOT YOURS.** On the bounce I read `git log --oneline -1` [...]
- **A PIN IS RETIRED BY A SCHEMA BUMP, AND IT FAILS ONLY ON THE WRITE PATH.** `pair-f7240814` spoke store schema 13 [...]
- **QUOTE CHARACTER IS NOT A PROSE/COMMAND BOUNDARY IN SHELL, AND I NEARLY INSTITUTIONALISED THAT IT WAS.** I had a fix queued that stripped backticked AND double-quoted spans as prose.
- **ONE COMMIT IS NOT ONE MOMENT, AND THE GATE READS THE WORKTREE.** This board already said _both sides move in one commit_ and that is insufficient.
- **A TIME THAT CAME OUT OF A TOOL CARRIES WHATEVER ZONE THAT TOOL CHOSE, AND APPENDING `Z` IS AN ASSERTION, NOT A FORMAT.** [...]
- **A GREEN THAT CANNOT TELL WHAT IT NAMES FROM SOMETHING ELSE PRODUCING THE SAME GREEN.** [...]
- **`ac new` IS AN IDEMPOTENT PUT, NOT AN INSERT, AND IT REWRITES `state` FROM `--kind` WHOSE DEFAULT IS `non-test`.** Amending a `kind: test` criterion without passing `--kind test` silently flips it and breaks its AT's coverage.
- **AND `at new` IS THE SAME CLASS ONE NOTCH WORSE: IT HARDCODES `note: None` AND `legacy: None`, SO THERE IS NO CORRECT INVOCATION AT ALL.** [...]
- **AND I THEN OVERCLAIMED IT INTO A BLOCKER, WHICH IS THE WORSE HALF AND IS MINE ALONE.** From _no verb writes `note`_ -- true -- I reported to vc [...]
- **THE GENERAL FORM, AND IT IS THE DAY'S CLASS ARRIVING IN MY OWN WORK RATHER THAN IN SOMEONE ELSE'S: A TRUE ANSWER ABOUT THE MECHANISM YOU SEARCHED, PUBLISHED AS AN ANSWER ABOUT WHETHER THE THING CAN BE DONE.**
- **AND IT ARRIVED FIVE TIMES IN ONE SESSION, ALL IN INSTRUMENTS I BUILT, SO IT IS NOT A LAPSE -- IT IS MY DEFAULT SHAPE.** [...]
- **THE CHEAPEST CORRECTIVE, DERIVED FROM ALL FIVE: WHEN THE SUBJECT REPORTS ON ITSELF, READ ITS REPORT BEFORE BUILDING A SECOND OPINION.** The critic prints its own census [...]
- **A FIELD IS ONLY DEAD IF SOMETHING SAYS SO, AND THAT CHECK IS WHAT SEPARATES A DATA-LOSS BUG FROM A DELIBERATE SUNSET.** Before reporting the above I tested the opposite hypothesis: `note` might be a migration-only field being retired [...]

### FAMILY 5 -- THE GATE THAT DID NOT RUN, AND THE VERB THE NAME SENDS YOU TO

**ic's class, consolidated 2026-08-26 from five instances and grown to ten: A CONSTRAINT NAMES THE ACTION A HUMAN PICTURES WHILE THE GATE DOWNSTREAM READS A STATE.** Its sharpest corollary [...]

- **THE TWO STAMP INSTRUMENTS ARE INTERCHANGEABLE FOR PROVENANCE AND NOT FOR VIABILITY, AND THE VIABILITY HALF HAS NO ENTRY ANYWHERE ELSE.** [...]
- **ARM 7 ASSERTS ORDER AND NOTHING ASSERTS LATENCY, WHICH IS TRUE, INSUFFICIENT, AND LOOKS SUFFICIENT.** It proves the dirt verdict is taken before the first cargo invocation.
- **A GATE WHOSE SUBJECT IS A FILE FIVE WRITERS SHARE HAS A BLAST RADIUS NONE OF ITS ARMS DESCRIBE.** [...]
- **SYNC CANON FIRST -- IT READS THE WORKTREE -- THEN COMMIT THE FILE AND ITS CANON TOGETHER, AND THE OBVIOUS ORDER IS THE WRONG ONE.** [...]
- **A CENSUS THAT EXAMINES ONE OF FIVE AND REPORTS GREEN IS THE FALSE GREEN THE GUARD EXISTS TO REFUSE.** Arm 10's first draft printed `(1 examined)` against a tree carrying five [...]

- **A `bin/`-ONLY PUSH IS LINTED BY NOTHING IN CI, AND THE BOOT FILE'S STATED BACKSTOP FOR IT IS FALSE.** [...]
- **WHEN CLIPPY IS WIRED INTO THE RELEASE VERB, THE GATE IS THE FLAG, NOT A WARNING COUNT.** The verb runs zero clippy today (ic [...]
- **THE RELEASE VERB NEVER TESTS THE TREE IT TAGS.** `preflight()` holds every gate and runs ONCE at `:482` [...]

### FAMILY 1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT

**THE HEAD, ARRIVED AT 2026-08-25 AFTER SIX INSTANCES IN ONE EVENING AND PAID FOR MANY TIMES SINCE: AN INSTRUMENT THAT RETURNS THE SAME ANSWER WHATEVER THE SUBJECT DOES HAS MEASURED NOTHING, AND ITS OUTPUT IS INDISTINGUISHABLE FROM A REAL RESULT.**

**THE TEST: RUN IT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Not _check your instrument_, which is unactionable [...]

**THE REASONING TRAPS, ONE LINE EACH. Each was a separate incident; the narratives are in `.history/` and in git.**

- A **no-op cannot fail**, so it measures nothing. Force the real path before believing a green.
- A **negative assertion needs a fixture it actually refuses**, or it is not an instrument.
- **Run the NEGATIVE control, not only the positive one.** A pattern that matches what it should is half a test.
- **A control is only a control if its ground truth is KNOWN rather than RECALLED** (vc). A remembered ground truth is a hypothesis.
- **Declare the expected denominator BEFORE measuring**, then check the actual against it (vc). _12 expected_ refuses a `0` on sight.
- **Expect the red, name it in advance, and treat an unexpected GREEN as a finding about the instrument** (vc).
- **A count is not a measurement until something says what each hit IS** (vc).
- **An assertion placed AFTER the write it guards tests the writer, not the subject -- and it cannot fail.**
- **Two readings of one source is one instrument counted twice** (cc).
- **A measurement taken before your own write is stale by construction.**
- **Line-index surgery must be bottom-up WITHIN a block, not only across sections** -- an insert above a later delete shifts it [...]
- **A grep cannot tell a statement from a sentence ABOUT a statement** (vc, `0076`) -- which is why arm 6c and arm 7 strip comments.
- **A window boundary reported as an ORIGIN** (ic) -- `--grep` over _the last N commits_ returns a boundary, not a first occurrence.
- **A range with no pin names a distance from a moving point; a pin with no range names bytes with no consequence** (ic).
- **A count over a `dirty-` marker is a FLOOR, not a distance** (ic).
- **A guard whose predicate depends on WHEN it runs rather than on WHAT it reads has a window that closes silently** (vc).
- **A probe's choice of VERB silently selects which subject answers**, because the currency guard is per-command.
- **The harness ran the subject under different shell options than its only production caller** -- fifteen green arms over a broken function.
- **A check can introduce a premise nobody made and then fail it; a false `MISSING` reads exactly as confidently as a true one.**
- **A correct finding can carry a wrong citation, and the citation is the half that gets reused.**
- **A retired premise keeps issuing orders** -- a remedy correct when written keeps being followed after its condition expires.

**THE OPERATIONAL TRAPS -- these are the ones that return a plausible wrong answer rather than an error.**

- **NEVER `$?` AFTER A PIPE.** `cargo test` needs `--no-fail-fast`. **`grep -c` exits 1 on zero**, so `grep -c [...]
- **The Bash tool's shell is ZSH:** unquoted `$var` does NOT word-split (a `for` over one runs once and looks finished) [...]
- **`2>/dev/null` converts a broken probe into a clean answer.**
- **A backtick inside a quoted grep pattern is a command substitution.**
- **`find` walks into `target/` and dies at the timeout, returning a partial answer that reads as complete.** Scope to `native/rust/crates`.
- **`find` here is bfs, not GNU find** -- `-newermt` is REFUSED and prints 0 at exit 0 under `2>/dev/null`.
- **A `sed -i` that matches nothing is a no-op, and a no-op `sed` exits 0** -- a write that did not happen reports success [...]
- **`bash -n` is wrong in BOTH directions and neither is visible from its output** (half vc's).
- **`--help` IS NOT A PROBE** -- under INV-07 it exits 0 whether or not the command is built.
- **`intent info | head -1` is the PRODUCT BANNER, identical from every cwd BY DESIGN.** I nearly withdrew a correct fix over it.
- **A grep's zero -- and a case-sensitive miss -- is a claim about the CORPUS.** Positive-control before believing it.
- **`find`-based populations describe the WORKING TREE, never the commit** (ic, `run_tests.sh:89`).
- **`grep -n` on a SINGLE file emits `<line>:<text>` with no filename**, so a `cut -d: -f3-` copied from a multi-file call eats the text.
- **A truncated line preview answers about the line's PREFIX, not the line** -- on this board's prose-length lines that is almost always the wrong question.
- **`ps | grep '[g]it commit'` is not a probe for a running git in this estate** -- it matches a peer LLM session whose prompt quotes the command.
- **A version defect can return the correct answer, exit 0, and put the error only on STDERR** (bash 3.2 `declare -A`).

### FAMILY 2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF IS WHAT CARRIES THE FALSE ONE

- **AN ECHOED LABEL FIRES WHETHER OR NOT ITS CLAIM IS TRUE, AND IT READS AS A VERDICT. THREE TIMES IN ONE DAY (2026-08-29), ALL MINE.** `echo "(empty = clean)"` after a command that prints nothing on success is not a check -- **the shell runs it unconditionally**, so it prints "clean" over a dirty result exactly as readily. Every instance today happened to be TRUE, which is precisely why the habit survives: the label is confirmed by the case that cannot disconfirm it. **A label is not evidence. Print the command's output and let the emptiness speak, or compute the verdict and print THAT.**
- **AND ITS BIG BROTHER SHIPPED: `smoke ARM 3` says _"every path v3 resolves against its install root"_ and iterates `$SUPPORT_PATHS` -- THE COPY LIST.** Same defect one level up: a stated claim wider than the population the code walks, in a comment written specifically to warn about that. **Found 2026-08-29 while measuring the keg; it is the class `0112b8c1` exists to fix, living inside `0112b8c1`.**

- **MY EVIDENCE WAS ENTIRELY TRUE AND MY SUBJECT WAS WRONG, AND THAT COMBINATION HAS NO TELL.** Every fact I cited checked out [...]
- **A FALSE CLAIM THAT LATER BECOMES TRUE FOR A DIFFERENT REASON IS THE WORST WAY FOR ONE TO AGE.** `bc38db85` said cc's half was met [...]
- **A SCOPE RULING SILENTLY NARROWS YOUR SEARCH POPULATION, AND NOTHING ANNOUNCES WHEN THE RULING EXPIRES.** I told hv the install artefact DID NOT EXIST.

- **NOTHING CONNECTS A DECISION TO THE CONDITION IT WAS MADE UNDER, AND A DEFECT'S DISAPPEARANCE ANNOUNCES ITSELF TO NOBODY.** [...]

**A CONFIDENT UNMEASURED NUMBER IN A _RATIONALE_ IS LOAD-BEARING IN A WAY ONE IN A _REPORT_ IS NOT -- A REPORT GETS CHECKED, A RATIONALE GETS HONOURED**

**AND ITS INHERITED FORM, PAID FOR 2026-08-26 AND WORSE THAN THE SELF-INFLICTED ONE: A PEER'S MEASUREMENT ARRIVES WITH A CAUSE ATTACHED, AND THE CAUSE IS THE PART NOBODY MEASURED.**

- **`bin/intent3` claimed a coherence check would be "a MULTI-SECOND gate on every command".** Driven: **~85ms end-to-end** [...]
- **`cmd/hosting` claimed "in a throwaway clone a mutator harms nothing".** Never driven [...]
- **`currency.lib` PRINTED A FLOOR AS A DISTANCE** -- the overclaim in the error message of the file written to refuse overclaims [...]
- **A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED** (ic [...]
- **MY TELL vs cc's, AND THEY ARE MIRRORS:** I publish the claim the measurement INVITES (wrong at the READING) [...]
- **A ROW CARRYING ONE TRUE SENTENCE AND ONE FALSE ONE IS HARDER TO CATCH THAN A WHOLLY WRONG ROW, BECAUSE THE TRUE HALF IS WHAT A READER CHECKS FIRST AND IT HOLDS** [...]
- **`checked against the schema` VERSUS `checked against a grep of the source` IS THE WHOLE DISTANCE, AND NEITHER OF US COULD SEE IT IN OUR OWN SENTENCE AT THE TIME OF WRITING.**
- **A STABLE CONCLUSION ACROSS THREE WRONG POPULATIONS IS NOT CORROBORATION -- IT IS THE POPULATION NOT BEING LOAD-BEARING FOR THAT CONCLUSION** (cc [...]
- **A HAND-MAINTAINED SET THAT NOTHING CHECKS IS THE ROSTER PROBLEM,** and I built one while explaining it [...]
- **UNCHECKED IS NOT EMPTY, AND ONLY THE OUTPUT CAN CARRY THAT DISTINCTION.** An instrument that measures a narrow scope and reports in the vocabulary of the general category produces a **true sentence and a false belief** [...]
- **A RULE IS HONOURED BY WHOEVER LEARNED IT AND DOES NOT PROPAGATE BY HAVING BEEN STATED.** `precommit` has 14 guard arms at three strengths [...]
- **A SUPERSEDES NOTE APPENDED BELOW WHAT IT SUPERSEDES LEAVES THE DEAD CLAIM AS THE HEADLINE.** `acceptance.md:199` still OPENS with the green arm vc amended away [...]
- **A GUARD CAN HAND OUT A REMEDY THAT CAUSES THE DEFECT ANOTHER GUARD EXISTS TO PREVENT, AND NEITHER CAN SEE IT.** [...]
- **A WRONG LINE NUMBER IS ONLY A TRAP IF FOLLOWING IT CAN REACH A GREEN.** vc warned the 3.2 error at `:216` would lead a fixer to gut the exemplar [...]

- **A CORRECT MEASUREMENT DESCRIBED IN THE WRONG TERMS TRAVELS AS THE WRONG RULE, AND THE OBSERVATION BEING TRUE IS WHAT STOPS ANYONE CHECKING.** Twice in a day [...]

- **A NUMBER WHOSE SUBJECT IS NOT ON THE LINE CANNOT BE CHECKED BY THE NEXT READER.** vc's board carried `Gate still PASS at 67 of 67` [...]
- **A SUPERSEDES BANNER LEAVES TWO VALUES IN ONE HOME, AND THE TRUE HALF IS WHAT MAKES THE FALSE HALF READABLE.** [...]

- **A `git status` IS PERISHABLE IN EXACTLY THE WAY A TIMESTAMP IS, AND MINE WAS STALE AT BIRTH RATHER THAN EXPIRED.** I told vc _none of the dirty files are mine [...]

### FAMILY 3 -- ROUTING, RELAY AND ATTRIBUTION

- **THE OFFER IS THE MOMENT TO CHECK, NOT THE SEND.** Once two messages are in flight the only tools left are racing and deduping [...]
- **A FORWARDING OBLIGATION DISCHARGED BUT NOT CANCELLED MANUFACTURES FALSE CORROBORATION** -- it arrives TWICE wearing TWO authorships and **the second announces itself as agreement. Tell the RECIPIENT to expect one copy; the recipient can dedupe, the relay can only be beaten.**
- **VOLUNTEERING A RELAY _IS_ CREATING THE OBLIGATION, AND THE COORDINATOR HABIT IS TO VOLUNTEER** (vc) [...]
- **NAME WHICH _HALF_ YOU ARE ATTRIBUTING** (vc, wholly theirs). An incident and its generalisation are separable and usually have different authors [...]
- **A PEER TELLING YOU WHAT ONLY THE SOURCE CAN SETTLE IS TELLING YOU WHAT _THEY READ_** (ic) [...]
- **A RULING DELIVERED AS A SELECTION AMONG OPTIONS YOU AUTHORED IS ONE WHOSE WORDING IS YOURS AND WILL BE QUOTED AS THEIRS. RECORD THE OPTIONS, NOT THE OUTCOME**
- **A CORRECT ANSWER ABANDONED ON A PEER'S SAY-SO IS WORSE THAN THE PEER'S WRONG ANSWER, AND IT IS THE HALF YOU CONTROL.** I had the attribution start date right at 2026-08-24 [...]
- **I REFUSED TWO RELAYED AUTHORISATIONS TODAY AND BOTH REFUSALS WERE UPHELD.** vc relayed hv's ruling that the attribution guard was mine to build [...]
- **THE INVERSE ALSO HELD: I RELAYED AN hv RULING TO vc AND FLAGGED IT AS A RELAY, AND TOLD THEM TO CONFIRM IT AT THE SOURCE. THEY DID, AND IT HELD.**
- **THE BLOCKED PARTY TELLS THE BLOCKER, NOT THE SEQUENCER.**
- **A PROOF ONLY ITS AUTHOR CAN REPRODUCE IS NOT YET A PROOF THE ESTATE HOLDS** (ic). Survives the soundness being conceded [...]
- **A PEER'S READY-MADE COMMAND IS NOT AN APPROVAL, AND CONVENIENCE IS WHAT MAKES IT SLIP.** Distinct from a relayed approval: the peer claims no authority and is simply being helpful [...]

### FAMILY 4 -- THE SHARED CHECKOUT

- **`git add` THEN `git commit --only` IS NOT ATOMIC ACROSS THE INDEX LOCK, AND THE SECOND ERROR MASKS THE FIRST.** Landing this very fold [...]

- **`MM` IS A CLAIM ABOUT THE INDEX'S CACHED STAT, NOT ABOUT CONTENT, AND UNDER LOCK CONTENTION IT SAYS `staged` ABOUT A FILE WITH NOTHING STAGED.** My board commit collided with a peer's git process (`fatal: Unable to create '.git/index.lock'`) and the commit's own post-commit cleanup never ran.

- **`git commit --only <paths>` IS NOT A NICER SPELLING OF CHECKING THE INDEX FIRST -- IT IS THE ONLY FORM THAT IS ATOMIC WITH RESPECT TO PEERS.** Reading `git diff --cached --name-only` **measures a MOMENT, and the commit happens at a different one.**
- **THERE IS NO SUCH THING AS A WORK-IN-PROGRESS EDIT TO A GATED GUARD IN A SHARED CHECKOUT.** The pre-commit gate runs the **WORKTREE** copy [...]

- **`awk ... > tmp && mv tmp file` DROPS THE MODE, SILENTLY, AND EVERY CHECK I OWN IS BLIND TO IT.** I used it three times [...]
- **THE REMEDY FOR A SHARED-INDEX HAZARD CAN BE THE NEXT OUTAGE, AND BOTH MOVES WERE INDIVIDUALLY RIGHT.** vc flagged five staged files as a loaded gun [...]
- **PROVE AN UNBLOCK WITH A REAL COMMIT, NEVER BY ASSERTING IT.** I asserted twice and was wrong twice in twenty minutes. [...]

- **FOUR WRITERS, ONE TREE, ONE INDEX.** Peers' dirty files are in every `git status` you read [...]
- **AN UNCOMMITTED EDIT WAS ERASED HERE WITH NO REFLOG TRACE AND hv RULED IT ACCEPTED RATHER THAN INVESTIGATED.** Live and undiagnosed BY DECISION [...]
- **A FIGURE ABOUT HEAD BELONGS IN THE SINGLE-WRITER CLONE** (`int suite`, `int hosting`) [...]
- **`bin/` IS dc's LANE** (hv). `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED and not this repo's to edit.**
- **A MONIKER NAMES WHERE A SESSION LIVES, NEVER WHERE ITS BYTES LAND.** `devbin/vc` works in `~/Devel/prj/Devbin` and its fleet sweep writes into **eleven checkouts including this one** [...]
- **`--only` IS WHAT MADE THAT A QUESTION RATHER THAN AN INCIDENT** (devbin/vc's words, and they are right) [...]
- **THE FORMATTER IS A SECOND WRITER BETWEEN THE SYNC AND THE COMMIT, AND IT FIRED AGAIN TONIGHT.** It realigned a table I had just synced [...]
- **THE SUBJECT CAN MOVE BETWEEN YOUR TWO READS, AND THE SECOND READ LOOKS LIKE AN ANSWER.** `git diff --numstat` reported changes to an issue's canon [...]
- **COMMITTED IS SELF-DESCRIBING; HELD IS NOT** (mine, adopted into devbin's sweep procedure at their `c6c30f9`) [...]
- **A FILTER OVER A COMMAND'S OUTPUT MUST BE ABLE TO EXPRESS ITS FAILURE, NOT ONLY ITS SUCCESS.** [...]

- **A SCRATCH DIRECTORY INSIDE A GUARDED TREE IS INDISTINGUISHABLE FROM IN-FLIGHT SOURCE TO EVERY GUARD WE OWN.** [...]
- **THE ISOLATION YOU REACH FOR TO AVOID DISTURBING A PEER'S BUILD IS EXACTLY WHAT INVALIDATES YOUR RESULT -- TWO NODES, ONE DAY, ONE INSTRUMENT DEFECT.** [...]

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

- (2026-08-27) **hv AUTHORISED BOTH DISK SWEEPS DIRECTLY, AND THE SECOND ONE REVERSED A RULE I HAD WRITTEN MYSELF.** [...]

- (2026-08-27) **AC-12.1's CLASS (2) WIDENED IN THE STORE RATHER THAN ARGUED ON A BOARD, ON vc's RULING: _record it against the criterion now, build nothing._** [...]

- (2026-08-27) **hv CHOSE TEST-TARGET CONSOLIDATION FROM A MENU I AUTHORED, ON A DIAGNOSIS I LATER CORRECTED.** I had the 167 test targets as the CAUSE of the blowup [...]

- (2026-08-26) **VERIFY RESOLUTION, NEVER ENUMERATE POSITIONS.** `use` repoints what it owns and then measures what actually answers `intent` [...]
- (2026-08-26) **`use` LIVES IN `cmd/local`, ON HIGHLANDER GROUNDS, AND I TOOK IT BACK FROM cc TO PUT IT THERE.** hv assigned it to both of us in separate sessions.
- (2026-08-26) **CHECKING WHICH BINARY RESOLVES AND CHECKING WHETHER THE DOOR OPENS ARE DIFFERENT PROPOSITIONS** (cc [...]
- (2026-08-26) **`publish` STOPS WITH hv.** hv authorised vc to direct my work on other projects [...]

- (2026-08-25) **D42 AMENDED BY ME, AGAINST MYSELF: THE RULE IS THAT NO CALLER AUTHORS A STAMP.** The signature test -- _no function TAKES a time_ -- is one sufficient condition and was never the definition.
- (2026-08-25) **AT-11.6's TWO REAL DEFECTS WERE BOTH FOUND BY LIVE INCIDENTS, NOT BY REVIEW OR MUTATION.** It shipped with nine arms and four mutations.
- (2026-08-25) **`--only` IS THE ONLY COMMIT FORM ATOMIC WITH RESPECT TO PEERS.** Reading `git diff --cached` first measures a MOMENT [...]
- (2026-08-25) **vc HAS hv's PEN AND hv SAID SO IN MY OWN SESSION**, which is what makes it different from the two relayed authorisations I refused yesterday.
- (2026-08-25) **vc's OPENING ASSIGNMENT WAS WRONG AND THE CONTRACT CAUGHT IT.** `claude ws` was routed to me as WP-07 work off ic's surface probe [...]
- (2026-08-25) **AC-07.2 INVESTIGATED AND DELIBERATELY NOT STARTED.** Writing `hook_compat.rs` dirties `native/rust` [...]
- (2026-08-25) **A TEST'S POPULATION IS NEVER READ FROM THE THING UNDER TEST.** `hook_compat.rs` assembles its hook roster from the shipped scripts and `settings.json` and never from `install::HOOKS`.
- (2026-08-25) **I DID NOT WIDEN A PEER'S GUARD TO CLOSE MY OWN FINDING.** `every_declared_hook_ships_as_a_script` is one-sided and lives in cc's module [...]
- (2026-08-25) **THE PRECONDITION I WROTE AGAINST MYSELF ON 2026-08-25 EXPIRED AND I CHECKED RATHER THAN INHERITED IT.** _Do not start AC-07.2 while `native/rust` must go clean_ was correct while four nodes were waiting on a clean subtree to close the gate.
- (2026-08-25) **A `dirty-` BINARY MAY READ CANON AND MAY NEVER WRITE IT (vc's rule, and it binds me).** Reading is safe because **the STORE is the subject and is independent of the build** [...]
- (2026-08-25) **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS **NOT** IN IT ARE NOT IN IT.** This is the sweep's durable output and it makes the class checkable by READING.
- (2026-08-25) **THE INVERSE OF THE `session-finish` CLASS, AND IT IS WORTH NAMING BECAUSE I WENT LOOKING FOR THE WRONG ONE.** session-finish was _canon says the name exists and the door refuses it_.

- **RETIRED TO `.history/20260828/wip.md`: the five-entry `(C)`-exhaust block (2026-08-25).** hv closed the subject in one line -- _"I DO NOT WANT ANY CLAUDE EXHAUST IN MY COMMITS [...]

- (2026-08-25) **vc AS CONTRACT STEWARD AMENDED AC-11.6 UPWARD ON MY ROUTING, AND MY PROPOSAL CARRIED AS WRITTEN** (`f68d397c`) [...]
- (2026-08-25) **THE ROUTE FAILURE ON AT-11.6 IS MINE AND IT IS THE BETTER EXHIBIT.** I wrote _routes to vc_ in the same paragraph as the conflict [...]
- (2026-08-25) **hv RULED: fix both trees LOCALLY, commit, and DO NOT PUSH v2.** _"The checked out v2 branch is only being used locally here by projects on this machine ...
- (2026-08-25) **hv CHOSE THE ROUTE-LEVEL DECLARATION over a per-file exception list.** Branch not taken: an entry per fix [...]
- (2026-08-25) **THE ENFORCEMENT LOSS IS A COST OF hv's NO-PUSH RULING, NOT OF THE ENGINEERING, AND NOBODY PRICED IT WHEN THE DECISION WAS MADE** [...]
- (2026-08-25) **A PEER'S FULLER QUOTE OF A RULING IS STILL THE PEER.** vc supplied the clause they had dropped -- the one authorising the commit -- and told me not to treat their reading as my authorisation [...]

**Decisions dated 2026-08-24 and earlier are archived at `.history/20260828/decisions-pre-0825.md`** -- their subjects shipped (the v2 freeze scope [...]
