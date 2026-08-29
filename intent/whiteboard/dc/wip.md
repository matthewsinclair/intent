---
node: dc
name: DevX Claude
role: worker
session_id: ff2a3ea4-b800-4f7e-8bcd-8dd01154cb5f
heartbeat_at: 2026-08-29 13:55Z
status: active
focus: "**0133 IS ON MAIN (`04cf6f18`), and everything it owed is closed.** hv ruled both halves: `Unsatisfied` carries its OWN payload `note`, NOT `evidence`; `ac unsatisfy` KEEPS CLEARING. **THE REBASE IS THE LESSON: it reported NO CONFLICT and the compiler then produced four E0533s** in a file cc wrote while 0133 sat on a branch -- a clean merge is not a compiling tree, and the ports went in at `{ note: None }` NOT `{ .. }`, because the weaker port compiles, passes, and silently spends a peer's assertion strength. **0136 FILED AND PRICED: `deny_unknown_fields` is a property of the variant SHAPE, and `Computed` is the last unit variant, so it still accepts unknown keys.** The price is ZERO -- bytes identical, canon accepted, schema identical -- so it is my implementation detail, and my first comparison reported a FALSE difference it manufactured from its own type names. **Delivered binary emits JSON_VER=11/SDL_VER=9 against a repo at 13/11: two generations stale, worse than ic reported.** NEXT: implement 0136 on vc's sequencing; the composer as a GUARD."
claims: [ST0056/07, ST0056/11, ST0066]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260824/wip.md`. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.
- **`stat` PRINTS LOCAL. `git log` PRINTS LOCAL.** Convert at the SOURCE and keep the local value beside it, or a reader appends a `Z` and lands an hour out. I did exactly that today and vc caught it.

- **CORRECTED, BY MY OWN EXPERIMENT: `cargo test` DOES NOT WRITE `~/.intent/home`. THE WRITER IS CLOSED (cc `9c2ba9ed`) AND MY CAUSAL CLAIM WAS A SPURIOUS CORRELATION.** I reported watching the pointer go from a deleted worktree to healthy seven seconds after running the suite. **vc pointed out hv had repointed it BY HAND with an `echo` in the same window** -- a third party writing the same file between my before-state and my after-state. Re-ran ic's experiment across the crate pair under a decoy `HOME`: **nothing under `.intent` at all, only cargo's own cache.** And `9c2ba9ed` was already an ancestor of HEAD at 22:50, so my run could not have written it. **The shim still resolves through that pointer, so a stale value still means every commit in that estate refuses -- but it is not republished by testing, and (B) is not blocked the way I said.**
  - **THE SHAPE IS THE ONE I SPENT THE DAY CORRECTING IN OTHERS: before-state, my action, after-state, with an unmeasured third party in between.** laksa-cc did it twice tonight attributing peer writes to their own commands, and I relayed their retraction hours before making the same claim. **An alternative explanation adjacent to the evidence, in the sentence rather than in the command.**

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **`intent` ON PATH IS v3.0.0 AND RESOLVES INTO THIS TREE. `intent3` IS NOT ON PATH AT ALL.** Measured 2026-08-27: both `~/.local/bin/intent` and `~/bin/intent` are symlinks to `Intent/native/rust/target/release/intent`, repointed **26 Aug 23:53**, and `intent info` reports `INTENT_HOME: /Users/matts/Devel/prj/Intent`. **THIS ENTRY PREVIOUSLY SAID THE OPPOSITE IN BOTH DIRECTIONS** -- v2.19.0 routing to the frozen `Intentv2`, with v3 present under the distinct name `intent3` -- and it was stale for a day and a half while I read it every session. **IT WAS NOT CAUGHT BY USING IT; IT WAS CAUGHT BY MEASURING SOMETHING ELSE.** A board line stating an environment fact has no natural corrective, because the environment does not argue back. Consequence worth holding: every estate whose hook shells out to `intent critic` now gets the **v3** binary.
- **`intent3` NOW REFUSES A BINARY THAT CANNOT BE SHOWN TO DESCRIBE THIS TREE** (hv ruled 2026-08-24). Matrix and reasoning: `bin/.devbin/cmd/shared/currency.lib`, one home. **It keys on DECIDABILITY, never on dirt.**
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED** -- `intent ac status ST0057`, `intent ac status ST0056/03`, `intent ac gate ST0057`. It lived in THREE homes at THREE values on 2026-08-24, one document disagreeing with itself twice. **Do not put the number on this board; put the calls.**
- **hv's FREEZE SCOPE (2026-08-24): Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.** A v3-only defect is a v3-only fix; a shipped-surface one lands in BOTH trees.
- **THE INDEX IS SHARED IN THIS CHECKOUT.** `git add` puts your file where a peer's bare `git commit` sweeps it. Always `git commit --only <paths>` -- **but the refusal is a property of naming a FILE, not of `--only`, and I had it as unconditional.** Driven both arms: `--only <untracked FILE>` exits 1 with `pathspec did not match any file(s) known to git`; **`--only <DIRECTORY>` exits 0, commits the tracked edits and leaves the untracked file behind in silence** (ic). **The success output enumerates what it TOOK, so the omitted file has no line to be missing from** -- this board's own absence rule, arriving at a commit instead of a grep. **`git status --porcelain -- <paths>` after every commit; `??` is the entire signal.** My own `6096e14c` used the directory form and was clean, which was luck rather than care.
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** To ask about another clone, run ITS `bin/int`.
- **A DEVBIN COMMAND RESOLVES ITS PROJECT FROM SOMETHING OTHER THAN YOUR CWD -- BUT THE v3 BINARY RESOLVES FROM CWD.** Both are true and confusing them cost a live incident today.

## DOING

**NOTHING IN FLIGHT. 0133 landed and its whole owed list is closed.** Days verbatim in `.history/20260828/` and `.history/20260829/`; NOT restated here.

### ST0066 -- `intent fc`. AC half `b7a3e771`. **0133 LANDED `04cf6f18`; docs `72d716dd`; coverage `9c220c1d`; issue 0136 `4a648831`**

- **hv RULED BOTH HALVES 2026-08-29.** `Unsatisfied` gets its OWN payload, `note`, **NOT `evidence`** -- evidence means proof a criterion was MET, on `Satisfied` and nowhere else. And **`ac unsatisfy` KEEPS CLEARING**. Ruling 1 made ruling 2 cheap: **nothing to orphan**, because the note is a different field from the one the verb clears.
- **A CLEAN REBASE IS NOT A COMPILING TREE, AND THIS IS THE FOURTH SHAPE OF THE SAME FAMILY.** cc's `8aebe2ce` shipped `a_create_refuses_a_child_id_that_is_taken.rs` while 0133 sat on a branch; it builds the OLD shape in four places. Git reported no conflict; the compiler produced four E0533s. **Code written on main during a branch's life references the old shape and the merge cannot see it.**
- **THE PORT IS THE PART WORTH KEEPING, NOT THE BREAK.** Those four could go to `{ .. }` and everything compiles and passes. **The unit variant FULLY constrained the value, so `{ .. }` is the port that looks mechanical and quietly weakens a peer's assertion** -- one of them guarding the HTTP/GraphQL PUT contract by cc's own comment. Ported at `{ note: None }`, same strength cc wrote. **A widening must not spend a peer's test strength to buy its own convenience, and nothing mechanical flags the weaker port.**
- **0136 FILED BY ME AND PRICED BEFORE ESCALATION (vc's instruction, and it corrected me).** `deny_unknown_fields` is a property of the variant's SHAPE, not of the enum. Census measured post-0133: `computed` (unit) does NOT refuse; the four structs and the newtype `fiat` all do. **`unsatisfied` was closed as a SIDE EFFECT of the widening -- nobody decided to fix strictness, so it could regress the same way.**
- **THE PRICE IS ZERO AND THAT IS MEASURED, NOT QUOTED.** `Computed {}` vs `Computed`: bytes identical (`{"is":"computed"}`), existing canon accepted by both, unknown field refused only by the proposed shape, generated JSON Schema identical. **So nothing published moves -- no face bump, no canon rewrite -- and it is my implementation detail rather than hv's ruling.**
- **MY OWN PRICING INSTRUMENT LIED FIRST, AND IT IS THE SAME FAMILY AS EVERYTHING ELSE HERE.** The schema comparison reported a difference that did not exist: the two mirror types must have different names to coexist, so `title` differed for a reason having nothing to do with the change. **An instrument manufacturing its own finding.** Fixed by excluding `title` and adding a third enum with a real extra field as the positive control that the comparison can still see a genuine difference.
- **COVERAGE: ONE OF THE TWO GAPS DID NOT EXIST, AND I FOUND THAT BY KILLING IT RATHER THAN READING THE COMMENT THAT SAID SO.** `ac_kind_state_invariant.rs` claims its hand-written samples list is safe because `every_declared_state_has_a_sample` discovers the roster from the schema. Dropping `Unsatisfied` fails that arm BY NAME. **The claim is true; nothing added.**
- **THE REAL GAP WAS `graphql_face_agrees.rs`: dropping `Unsatisfied` from the projection cases left ALL THREE tests green.** `count("AcStateName")` next door does not close it and looks like it should -- it compares the SDL to a literal and never looks at what the file exercises. Cases now live in `projection_cases()`, one home, and the new arm DISCOVERS the roster from the published SDL both directions. Driven red: names `["UNSATISFIED"]` while the other three stay green.
- **THE STRICTNESS ARM NOW SAYS WHAT IT GUARDS** (vc: otherwise the next reader deletes it as a duplicate of serde behaviour they assume is automatic). It records that it is NOT automatic, cites 0136, and says `unsatisfied` gained the protection by side effect.
- **FOUR STALE PROSE CLAIMS OF MINE, FIXED (`72d716dd`) -- THREE OF THEM COUNTS IN ONE PARAGRAPH.** `AcState::name`'s doc said "three variants carry payloads", "these five words", "a sixth variant"; measured today: five, six, seventh. **It went stale twice and read perfectly well both times.** Replaced by the PROPERTY, with a line saying why the numbers are gone so nobody helpfully restores them. `enum_str`'s doc claimed a panic that "cannot happen" while `AcState` was already a counter-example. `transitions.rs` said the reversibility ruling was awaited two lines below the edges implementing it -- **refuted by its own diff on the day it landed.**
- **STILL UNBUILT AND THE PIECE vc MOST WANTS: the composer as a GUARD.** `fiat_marker` is the one rendering; **"one composer" and "one composer nobody can go around" are different deliverables.** Owed as a CENSUS over render sites with a two-sided drive on planted fixtures -- `shared-artefact-guard` arms 10 and 11 are the shape, and arm 11 is why a census alone is decoration.
- **ALSO UNBUILT: `ac.fc` itself, the cascade, doctor.** `fiat` is still a declared ORPHAN because no verb reaches it. **AC-00.4 is being REWRITTEN by vc -- do not build to its letter.**

### THE DELIVERED BINARY IS TWO SCHEMA GENERATIONS STALE -- MEASURED BY PROPERTY, NOT BY LABEL

- **`intent schema --versions` is the currency probe that works; the version string is NOT.** The delivered `native/rust/target/release/intent` reports `SCHEMA_JSON_VER=11`, `SCHEMA_SDL_VER=9`; the repo's committed faces are **13** and **11**. ic reported it predates `Fiat`; it is worse than that, and it was already a generation behind before 0133 landed.
- **A BINARY'S EMBEDDED COMMIT HASH IS NOT A CURRENCY CHECK.** My own worktree build reported a hash that was not its HEAD while its CODE was current -- cargo had recompiled the source and the embedded string was stale. **Ask the binary for a property that the change moves, never for its label.**
- **I DID NOT REBUILD THE SHARED ARTEFACT AND IT IS NOT MINE TO.** `native/rust` is clean so the guard would permit it; ic left the sequencing with vc and so do I. **The door being open is not the same as it being my door.** I filed 0136 through a binary I built in my OWN worktree (private target dir) and confirmed the write touched only `0136.json`.

### PARKED -- LIVE, HELD, NOT MINE TO CLOSE

- **`ac gate ST0057` returns PASS 66/66, 3 withdrawn; hv's board still says `BLOCKED -- 51/53`.** Routed to vc as pen-holder. `0123` independently reported the same 66/66, so the case that hv's line is stale is stronger -- **and it says nothing about where 51/53 came from**, which I am not claiming to explain.
- **AT-07.4's STATED REASON FOR RED HAS EXPIRED.** Set red 2026-08-19 because arm (b), THE REFUSAL, was uncovered; it is covered now -- `critic_arming_census.bats` arm 12 refuses at exit 3, **arm 13 is the present-tool half that makes 12 a test**, arm 14 checks v2 and v3 agree, driven 19/19. **vc set it red adjudicating my own refusal to set it, so vc flips it, not me.**
- **WP-11 CANNOT MOVE WITHOUT A PUBLISHED TAG.** AT-11.1 / 11.2 / 11.4 are all `n/a` pending one, and publish stops with hv by my own 2026-08-26 decision.

## TODO

### HELD BY A RULING, AND THE PATCH BELOW DIES WITH THE SESSION IF IT IS ONLY IN scratchpad

- **hv's STANDING DIRECTIVE (b): THE FIVE-STEP `bin/int` -> `bin/devbin` RENAME GOES AFTER THE SWEEP.** Nothing is deleted and no intermediate state is broken -- hv ratified `bin/int` as the optional shortcut, so both names work throughout. **Step 1 is behaviour-neutral (`bin/int` IS a symlink to `bin/devbin`) and I am still not landing it, because the hold is hv's and "it is harmless" is my judgement rather than their ruling.**
- **STEP 1 INLINED HERE BECAUSE `scratchpad/` DOES NOT SURVIVE THE SESSION.** It applied cleanly at close-out. One file, `intent/st/ST0056/parity/tools/runner_roster_check.sh`: `DISPATCH="$ROOT/bin/int"` becomes `DISPATCH="$ROOT/bin/devbin"`, above it the comment that is the actual content --

  > THE DISPATCHER IS ADDRESSED BY ITS OFFICIAL NAME, `bin/devbin`, AND NEVER BY A PER-PROJECT SHORTCUT. hv ratified the convention 2026-08-27: the shim in a project is always `bin/devbin`; a project MAY add a 2-3 character symlink (`bin/int` here) for humans; **tools and process always use the official name.** This file is tooling, so it takes the official one. `bin/int` keeps working for people.

- **RETIRED FROM THIS SECTION: `decisions-surface.patch` and `whiteboard-clock-guard.BUILT.sh`.** Both landed (`27b13f93` and follow-ups), so the scratchpad copies are now redundant rather than at risk.

### LIVE, MINE, UNSTARTED

- **THE KEG SHIPS NO RULE LIBRARY AND NO SKILLS. THE CODE FIX IS DONE (`0112b8c1`) AND IT IS NOW NAMED IN vc's 3.0.1 CUT LIST, SO THE SHIPPING HAS A DATE.** `SUPPORT_PATHS` carries the rules and skills trees; `support_paths_coverage` refuses at `stage` if v3 resolves an install-root path the tarball would not carry; `int macos smoke` is driven RED against the live keg and GREEN against a fixture at `total: 6 rule(s)`. **UNTIL THE RE-CUT PUBLISHES, THE ESTATE'S KEG STILL ARMS 0 OF 0 ON EVERY LANGUAGE** -- the fix and the shipping are different things and only the second one reaches a user. Cause found by devbin/vc as a PACKAGING gap, located to `:116` by vc, resolver enumerated by me.

- **FLEET WORK IS LIVE AND HALTED BY hv (2026-08-26 11:53Z, relayed devbin-cc -> vc). MY FOUR ARE DIRTY AND UNCOMMITTED -- see DOING for the exact table. hv LIFTS IT, NOT vc.**
- **FLEET WORK UNDER vc's RUNBOOK IS THE TOP ITEM AND hv AUTHORISED vc TO DIRECT IT.** `intent/whiteboard/vc/cutover-runbook.md`, plus `verify-canonical.sh` (self-test re-driven by me after vc's fix: **11 failures, threshold 11**). **THE CUTOVER IS 16 PROJECTS IN TWO SHAPES, NOT 9** -- Form B projects wire `session-context` and `require-in-session` but NOT `session-finish`, so the third hook is missing fleet-wide. **`Intentv2` IS NOT A MEMBER AND MUST NEVER BE MIGRATED** (hv ruled it unprompted; the tool warns unprompted too).

- **WP-07's `cwi` PORT IS DONE (`1ad284b3`) AND IT DOES NOT DISCHARGE `AC-14.12` -- WRITTEN HERE SO NO LATER READER COLLAPSES THEM.** WP-14 retires the VERBS (served from the store per `AC-14.7`); WP-07 retired the `bin/` COUPLING. Two different things, two different closing conditions, and hv's ruling gave the `claude ws` family an expiry the gate enforces (`97cc09ca`).

- **THE `bin/` COUPLING HAS AN EDGE NO SWEEP FOR `source` CAN SEE, AND IT NOW LIVES IN CANON RATHER THAN ON THIS BOARD.** `intent/plugins/claude/lib/claude_plugin_helpers.sh:84` CALLS `ext_root_dir()`, defined only at `bin/intent_helpers:367`, and never sources it -- so the file is coupled to the tree `AC-12.1` prunes while every sweep spelled _sources `bin/intent_helpers`_ reports it CLEAN. Post-cut it fails at **`unknown`, rc=0**, with `command not found` on STDERR only, because `plugin_root_tag:85` short-circuits an empty `ext_base` deliberately. **Full two-sided drive, both arms, and the widened class (2) are in `AC-12.1`'s text (`434dded6`) -- read it there.** **REPORTED, NOT BUILT:** shipped plugin canon inside the cut scope, and vc runs the order.

- **THE hv QUEUE: FIVE LIVE, NONE BUILT, ALL TOUCHING INSTRUMENTS THAT GATE OR DESCRIBE THE BUILD.** (1) **roster symmetry** -- `runner_roster_check.sh` reads PRESENT from the COMMIT and ROSTERED from the WORKTREE; four occurrences 2026-08-25, the fourth PREDICTED from the write-up before it happened. (2) **the staging-dir build** -- build into staging, verify, move into place: closes ic's absent-window and vc's concurrency gap with no lock, so no stale-lock failure after a SIGPIPE; **measured at 252 real refusals during one 66-second rebuild.** (3) **`verify_pair` compares the binary against HEAD when the binary's subject is `native/rust`** (ic, confirmed on my own build, which redded because a peer's canon commit landed during my 54-second compile) -- the remedy it implies is a rebuild that races the same board traffic. (4) **D42 amended** -- the rule is NO CALLER AUTHORS A STAMP; the signature test is one sufficient condition, never the definition, and a generic setter reaching `created` evades the proxy while producing the forbidden behaviour. (5) **arm 7 asserts ORDER and nothing asserts LATENCY** (vc's framing, sharpened onto my own arm): cargo WAITS on a locked target rather than failing, so a verdict taken at T can be followed by an unbounded block -- mechanism in Watch-outs, not restated here. **RETIRED FROM THIS QUEUE: the marker's scope blind spot, shipped as `d6ddb874` and now held by arm 6c.** Routed, not reached into.
- **THE ROSTER SYMMETRY FIX -- FOUND BY ME, QUEUED FOR hv, AND EXPLICITLY NOT MINE TO BUILD AT SPEED.** Read ROSTERED from the COMMIT as PRESENT already is. **It gates every commit in the estate and I caused two estate-wide blocks the day I found it**; vc endorsed the hold in those terms. It is the same conversation as the roster-charter ruling already with hv, because both are about WHAT POPULATION the roster is entitled to read.

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` (AT-11.7) has a green to reach. **STILL DECLINED ON SCOPE** -- WP-11 is RELEASE and hv asked for local usability. **TRAP: `codesign --force` REWRITES THE BINARY IN PLACE**, so nothing may hash until `verify_notarised` passes; and `cmd/macos:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one.
- **THE ATTRIBUTION GUARD -- RULED IN BY hv IN vc's SESSION, AND I AM HOLDING FOR hv's WORD IN MINE.** One arm, gates, assert `^Claude-Session:` ABSENT, nothing about `(C)`. **Three conditions, none optional:** (1) key on the COMMIT, never the `wb|board|localfold` subject convention -- 29 of 90 carriers were topic-prefixed real work (ic); (2) ANCHOR it -- the bare string matches a message that merely DISCUSSES the trailer, and it matched two of ours (ic); (3) **a positive control the guard actually FAILS -- mine, and it is the one that makes it an instrument** (see Family 1). **ic's fourth, found by hitting it: the success condition is a `grep -c` INVERSION -- `grep -c` exits 1 when the count is ZERO, which is the PASSING case, so `grep -c ... && ok` blocks every clean commit. COMPARE THE NUMBER, NEVER THE EXIT STATUS.**
- **NEW, AND IT IS MINE BECAUSE I FOUND IT IN MY OWN FILE: NOTHING VERIFIES THAT A ROSTER ROW DESCRIBES WHAT ITS RUNNER DOES.** `runner_roster_check.sh` verifies row-to-file EXISTENCE in both directions and is structurally blind to the row's CLAIM. cc asked that a mechanism, if one is built, be mine. **No mechanism proposed yet, and naming it is not building it.** **AND vc HAS PUT A GATE IN FRONT OF BUILDING IT, WHICH I AM RECORDING BECAUSE IT WOULD OTHERWISE DIE WITH THIS SESSION: DO NOT BUILD BEFORE hv HAS RULED WHETHER A DESCRIPTION IS IN THE ROSTER CHARTER AT ALL.** vc framing: this is a guard whose POPULATION IS FILES where the CLAIM IS BEHAVIOURS, and **widening a guard contract silently is how a roster becomes the mechanism.** So the next step is a ruling, never a checker.

### ROUTED, MEASURED, NOT MINE TO TAKE

- **WITHDRAWN: "MAIN IS RED ON `cli_end_to_end`" IS FALSE. THE WORKTREE WAS RED AND IT IS cc MID-EDIT.** Caught by vc. I ran the suite in a five-node shared checkout and reported it as a property of HEAD. **`git log` names COMMITS; the thing that broke my run was never in the commit history** -- cc has 7 paths STAGED in the shared index right now, including `render.rs` renaming the `st list` column `Slug` -> `Title` with `cli_end_to_end.rs` only half-updated to match. **Three of my four failures are `st list` table tests. That is the whole explanation and it needed no bisect.**
  - **A SUITE RUN IN A SHARED TREE MEASURES THE TREE, NOT HEAD -- and my instinct to check the test file's commit history was correct, careful, and aimed at the wrong object.** Same distinction as `git status` naming paths rather than authors, one level over. **My hedges are what made it cost nothing: I called it a lead not an accusation and said I had not bisected, so cc was told not to chase it before they had seen the red themselves.**
  - **AND `git diff` LOOKED CLEAN WHILE THE TREE WAS DIRTY, because cc's edits are STAGED** -- the first status column, not the second. A check for uncommitted work that reads only the unstaged diff reports a clean tree over seven staged paths.

- **ISSUE `0086` (HIGH, ic's find, QUEUED NOT ASSIGNED): `intent --help` rc=0 while `intent help` rc=2 `retired`, with a remedy claiming no replacement exists.** **Its general defect is this board's own class from a third direction: an ABSENT `spelling` field is RENDERED as a confident negative, and a passing test REQUIRES it to be** -- absence-as-meaning, forbidden by the preamble of the very file it lives in. Not mine unless assigned.

- **`VIEW_NAMES` (`address.rs:357`) CLAIMS A COUPLING THAT IS NOT IN THE CODE -- ANSWERED, FILED AS ISSUE `0087` (low, vc, `4f9ce518`), NOT MINE TO FIX.** Its doc says `Project::classify` _is the single answer_; `classify` keeps its own inline list and never reads it. **No harm driven and none claimed** -- they may be legitimately different populations, in which case the DOC is the defect. **The row survives only because the sentence is un-greppable and reads as an architecture guarantee.**
- **ISSUE `0085` -- THE ADVISORY HOOK FIRES ON EVERY WRITE REGARDLESS OF FINDINGS.** `[ -z "$findings" ]` can never fire: both binaries always put `critic:` header lines and an `ok:` line on STDOUT, the stream the hook captures. **Violates `session-finish.sh:46` -- _a hook that fires identically every time carries no signal_ -- three files from where that sentence is written.** v2 does it identically, so **INHERITED: a `corrected`-class contract decision in `AC-07.3(a)`'s shape, with the contract and not with me.** Opt-in twice. **OPEN QUESTION I DELIBERATELY DID NOT DRIVE (23:16Z, holding): `0085`'s premise rests on what the hook's FLAGGED invocation emits -- `--files <f> --severity-min warning --format text` -- and my drive used plain `--files` only.** vc verified independently and it is their filed issue, so re-driving it unasked is scope creep. **One command if anyone wants it, and worth knowing the premise has not been driven at the flags the hook actually passes.**

- **THE FROZEN-`$INTENT_HOME` MECHANISM: THE DETECTOR HALF IS CLOSED, THE ROUTING IS NOT.** vc landed the ref fix and the CI arm (`a38e884b`, `18ccfbbc`), measured in CI's own log rather than in simulation. **vc states plainly that their reason for `not discharged` expired but the ROUTING did not, and a guard cleared by a peer saying the ruling happened is not a guard.** It sits with hv. **What the guard should ASSERT under the freeze scope is still the unsettled half.**
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named, two commands produce it and nothing runs them. **Needs an edit to `tests/run_tests.sh`, which matts runs externally -- scope, not doubt.**

## Watch-outs

**TEN STANDING FAMILIES. Every rule is still here; the NARRATIVE around each was cut at the 2026-08-29 fold and is verbatim in `.history/20260829/watch-outs-and-decisions-full.md`. `[...]` marks a trimmed body, never a dropped rule.**

**Standing means NOT ARCHIVED. It does not mean NOT REWRITTEN -- I read it as the second for weeks, and that is why this board reached 105KB while a peer's was 5KB.**

- **AND THE SAME SHAPE IN A FOLD, 2026-08-29: I CUT A SECTION AND LOST THREE LIVE ITEMS WITH IT.** An aggressive fold replaced `## DOING` wholesale, and the PARKED block lived INSIDE it -- so `AT-07.4` and `WP-11`, both live and both held for a peer to close, went out with the archived content. [...]

### FAMILY 9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE (2026-08-29, mine, three instances in one build)

- **`cargo test` STOPS AT THE FIRST FAILING TEST BINARY.** Every count I gave vc during the ST0066 build came from a halted run reported as a total. [...]
- **THIS IS THE HONEST-AND-BLIND CLASS ARRIVING THROUGH THE TOOL I TRUST MOST.** I spent the day demanding positive controls for greps and getting them right, then read a test RUNNER's summary as a population without once asking whether it had walked the whole thing. [...]
- **THE GENERAL FORM: an instrument that STOPS EARLY is not the same failure as one that CANNOT VARY (Family 1), and it does not respond to the same control.** [...]
- **TREAT EVERY `N passing` I HAVE REPORTED AS A LOWER BOUND RATHER THAN A COUNT** unless the run carried `--no-fail-fast`.

### FAMILY 8 -- THE ESTATE'S OWN CONFIGURATION IS WHAT HIDES ITS BUGS FROM IT (2026-08-28, twice in one day, both mine)

**A fix applied by hand HERE removes the symptom and removes the ability to see the class.** Both instances were found only because a criterion said _in a consumer repo as well as this one_, and both fixes are BYTE-LEVEL NO-OPS in this tree.

- **`.prettierignore`** is 40 hand-written lines somebody sat down and wrote, so this repo has been immune since 2026-08-19 while every consumer stayed exposed. `intent init` laid down no exclusion at all.
- **`AGENTS.md` sections** render non-empty here because this project declares five languages that fill them. A project declaring only `shell` gets a bare `### Building`; one declaring only `author` gets two bare sections. **4 of the 7 packs.**

**THE TELL IS A FIX THAT LIVES IN THE TREE RATHER THAN IN THE TOOL.** From inside, _immune_ and _unaffected_ are indistinguishable -- there is no local symptom to notice and no instrument that fires. **So the question to ask of any hand-applied repair is: what lays this down for someone who does not have us?** `.gitignore` is the next instance already: `converge_gitignore` runs on migrate and NOT on init, so a project BORN on v3 never gets its store ignored. Reported, banked under hv's 3.0.1 scope, four lines beside the AC-07.6 fix.

### FAMILY 7 -- A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR A FACT (2026-08-28 12:11Z, three instances in one day, two of them mine)

- **THE RULE vc APPLIED, AND IT IS THE KEEPER: WHEN A RECORD CARRIES ONE FACT IN TWO NOTATIONS, THE ONE THAT CAME OFF THE INSTRUMENT BEATS THE ONE A HUMAN RETYPED.** devbin-cc's pre-hop record held the carrier's mode as `-rwx--x--x` (read by `stat`) AND as `751` in prose. [...]
- **THE REMEDY IS STRUCTURAL AND BETTER THAN CARE: EMIT BOTH NOTATIONS FROM ONE CALL, so they cannot disagree.** devbin-cc's Phase 4 now prints `%Sp` beside the sha. **A second notation derived from one read is not a second home; a second notation typed from a first is.** [...]
- **THREE INSTANCES TODAY AND THE OTHER TWO ARE MINE.** [...]
- **THE THROUGH-LINE, AND IT IS THE SAME ONE AS THE GATE FIGURE: A FACT WORTH RECORDING IS RECORDED AS THE CALL THAT PRODUCES IT, NEVER AS THE ANSWER IT PRODUCED.** `restart.md` already says do not transcribe the gate figure, run the three verb calls. [...]
- **AND A GAP I COULD NOT CLOSE, RECORDED AS UNCLOSED: I never captured the pre-hop carrier MODE** -- sha and size only. **The column was filled from devbin-cc's and conflab-ic's independent pre-hop banks, not from anything I held.** [...]

### FAMILY 6 -- AN UNMEASURED CLAIM INSIDE A COMPLIMENT (2026-08-28 09:28Z, mine, refuted by cc)

- **I TOLD cc THEIR FIX WAS BETTER THAN THEY CLAIMED, AND THE REASON I GAVE WAS FALSE.** I said v1's `find | wc -l` and v2's NUL count differed such that a newline-in-path could inflate BOTH the file count and the row count together and pass. [...]
- **THE DELIVERY VEHICLE IS THE CLASS, NOT THE ERROR.** The claim sat inside a compliment, in a message where every other claim was driven. [...]
- **THE HONEST FORM OF WHAT I WAS REACHING FOR, and it favours cc: THE REDUNDANCY THEY LOST COULD NOT HAVE BEEN KEPT.** v1's second arm existed only because its counter was dishonest, and **on a `-print0` list the NUL count is the only correct counter there is.** [...]
- **cc's FREEZE RULE, THEIRS AND BETTER THAN THE GROUND I OFFERED: ONCE A BASELINE IS TAKEN, THE INSTRUMENT IS FROZEN.** I argued stability against churn. [...]

### FAMILY 0 -- THE SHARED CHECKOUT AND THE BLIND INSTRUMENT (2026-08-27, all measured, most of them on me)

- **`$?` AFTER A PIPELINE IS THE LAST STAGE'S STATUS, AND IT HIT FOUR NODES IN ONE EVENING WITH NO CROSS-TALK.** [...]
- **"INERT" IS DIMENSION-SPECIFIC AND THE DIMENSION YOU MEAN MAY NOT BE THE ONE THAT MATTERS.** [...]
- **DETECTION IS NOT DISCRIMINATION: A GREEN THAT IS INVARIANT UNDER THE THING IT CERTIFIES.** After landing tolerance 0, `whiteboard_clock_guard.bats` returned 14/14 -- and the suite HAS a positive control that fires.
- **THE DAY'S CLASS, FOUR INSTANCES, FOUR NODES, ONE DAY -- AND IT SUBSUMES THE THREE ABOVE.** [...]
- **THE COMMIT IS THE ROLLOUT FOR ANYTHING READ LIVE OUT OF `INTENT_HOME`, SO THERE IS NO WINDOW IN WHICH TO ANNOUNCE FIRST.** Tolerance 0 was in force in fifteen estates the instant `3463f784` landed -- no sweep, no upgrade, no gap. [...]
- **DRIVING AN INSTRUMENT TELLS YOU ABOUT THE WORKTREE AT THIS INSTANT, NEVER ABOUT THE RUN YOU ARE INVESTIGATING.** [...]
- **A TRUE GREEN FROM A BLIND INSTRUMENT IS WORSE THAN A FALSE ONE, BECAUSE IT GETS CITED.** My estate figure "0 of 5 hooks carry the arm" grepped `pre-commit`, which **carries the gate in no estate** -- that grep could not have returned anything else under any state of the world.
- **THE PARITY APPARATUS HAS FOUR ROOT CONTRACTS, TWO MUTUALLY INVERSE.** 7 honour `ROOT=`; 6 resolve unconditionally from the script's own location; **2 derive from CWD's git toplevel, where `cd` works and `ROOT=` is IGNORED**; 2 take a positional.
- **A CENSUS CANNOT REPORT THAT IT MEASURED THE WRONG PREDICATE.** My estate audit counted STRINGS (`102` in Laksa) and was structurally blind to the **7 sites that EXECUTE**, which is what turned it into a red suite.
- **PROBE-BY-EXECUTION IS SAFE ONLY IF THE WRITE TARGET IS REDIRECTED AND THE REDIRECT IS HONOURED -- AND NOTHING VERIFIES THE SECOND HALF.** Three of my gated instruments execute (`$BIN doctor`; generators under `OUT="$tmp"`; `$DISPATCH precommit --list-guards`).
- **A RESTORE MUST RESTORE EVERY INPUT THE CONSUMER KEYS ON, AND THE CONSUMER DECIDES WHAT THOSE ARE (ic).** A metadata-preserving restore puts bytes back with the ORIGINAL mtime; cargo keys on mtime, so the next run links the MUTATED artefact while `git diff` says clean.
- **ADDITIONS MAY LEAD, REMOVALS MUST TRAIL (laksa-cc).** Laksa's suite went red at HEAD with nothing in `lib/` changed, because a removal landed ahead of its references.
- **AN ANOMALY THAT SURVIVES A GOOD CORRECTION IS NOT NOISE -- IT IS THE PART OF THE SUBJECT THE CORRECTION DID NOT REACH.** cc's correction of my stale-hook claim was better measured than mine and right about the repair.
- **CONTENTION MANUFACTURES STAGED-AND-STRANDED.** The index was locked continuously for ~15 minutes; `git add` succeeds and the commit then loses the race, so files sit staged in the shared index for as long as the queue lasts.
- **THE AUTHORITATIVE STATEMENT WAS IN THE FILE, ON MY SCREEN, AND A BELIEF ABOUT THE SEAM OUTLIVED IT.** I told devbin-vc that Intent repointing `bin/int` would be undone by devbin on the next re-vendor, because `bin/.devbin/cmd/hooks` emits it 14 times.
- **AN OVERRIDABLE-LOOKING KNOB THAT IS NOT OVERRIDABLE REPORTS A PASS ABOUT THE WRONG SUBJECT.** [...]
- **THE THIRD FACE OF THE SAME FAMILY, AND IT COST THE MOST BECAUSE IT LOOKED THE BEST: A CORRECT INSTRUMENT, CORRECTLY DRIVEN, POINTED AT THE WRONG SUBJECT.** I drove `lib/templates/hooks/pre-commit.sh` two-sided and reported `rc=1` as a fact about estates.
- **NEVER MUTATE THE LIVE SUBJECT. COPY IT, MUTATE THE COPY, RUN THE COPY, DELETE THE COPY -- SO THERE IS NO RESTORE TO GET WRONG.** [...]
- **A FLAG NAMED FOR SCOPE CAN DECIDE SEVERITY, AND GUESSING WHICH COSTS THE WHOLE ESTATE.** `thread_view_skew_check.sh --changed <paths>` does NOT narrow what is CHECKED -- all 288 views are examined either way and the cost is identical.
- **FIVE CROSS-PROJECT CONSTANTS WERE PROPOSED TODAY AND ALL FIVE FAILED ON MEASUREMENT -- AND THEY ARE NOT PROPERTIES OF CARGO, THEY ARE PROPERTIES OF HOW A TREE HAS BEEN BUILT OVER TIME.** (1) phantom-contamination of a duplication metric: Conflab 46%, Intent 11%.
- **CARRIED FROM vc 2026-08-27, NOT MEASURED BY ME, AND RECORDED BECAUSE vc BELIEVED MY BOARD ALREADY HELD THESE AND IT DID NOT.** Attribution matters here: these are vc's findings and I am the second carrier, not a witness. [...]
- **NEVER DELETE A TREE SOMETHING ON `PATH` RESOLVES THROUGH, AND CHECK BEFORE, NOT AFTER. THIS IS STEP 0 OF ANY BUILD-CACHE CLEANUP.** [...]
- **WHEN THE FAILURE IS "MY BOUND WAS TOO SMALL", A BIGGER BOUND REPRODUCES THE CLASS WITH A DIFFERENT CONSTANT -- THE FIX IS NO BOUND, PLUS A PRUNE FOR SPEED.** I went `-maxdepth 4` (found 7), widened to 6 (found 11), verified at 7 and **declared ALL COVERED.
- **INDEPENDENT DERIVATION IS NOT CORROBORATION WHEN THE INSTRUMENTS SHARE A SHAPE (vc's, 2026-08-27, and it is the sharpest thing either of us said).** [...]
- **VERIFY THE THING THAT DEPENDS ON THE CHANGE, NOT ONLY THE THING BEING CHANGED (one entry, both ends, agreed with vc).** Mine: I checked `target/release` was intact and never asked what depended on `target/debug` -- which is how the Conflab recipe went out.
- **ENUMERATE TARGET DIRS FROM THE FILESYSTEM, NEVER FROM THE LAYOUT YOU EXPECT -- AND `.worktrees/` IS THE ONE EVERY SWEEP MISSES.** My estate sweep used `find -maxdepth 4 -name Cargo.toml` and returned SEVEN.
- **THE GUARD'S OWN REFUSAL REMEDY BUILDS AN UNBOUNDED CACHE NOBODY COLLECTS, AND IT IS INVISIBLE TO EVERY `target/debug` MEASUREMENT.** `releasebuild.lib:189` tells a refused caller `CARGO_TARGET_DIR=<dir> int build $verb`; each node pointed it at `target/<node>/`.
- **THE BUILD-CACHE SWEEP METHOD, WHICH IS THE PART THAT TRANSFERS WHEN NONE OF THE NUMBERS DO.** [...]
- **A ZERO AFTER A DELETE MUST BE CHASED, NOT REPORTED.** `bin/int cli ac list ST0056` returned zero rows straight after my sweep.
- **A COMMENT IS NOT INERT IN SHELL: A LINE OPENING `# shellcheck` IS A DIRECTIVE, AND A MALFORMED ONE MAKES THE LINTER STOP RATHER THAN CONTINUE.** [...]
- **A SURVIVING FILE THAT STILL DEPENDS ON `bin/` FAILS OPEN AFTER THE CUT, AT rc 0, WITH STDOUT THAT CAN BE BYTE-IDENTICAL TO THE CORRECT ANSWER.** [...]
- **THE CLASS I WAS THE COMMON FACTOR IN THREE TIMES IN ONE DAY (2026-08-27): A TRUE ANSWER TO A NARROWER POPULATION THAN THE QUESTION.** (1) `grep 'INTENT_ROOT/bin/'` -> 2 of 7 coupled files.
- **A WRONG-BUT-ADJACENT CORRECTION FROM A PEER IS A SIGNAL TO WIDEN THE CHECK, NOT TO WIN THE POINT (2026-08-27, vc).** vc reported `Lamplight/pdf_extractor` unmarked.
- **A DEFENSIVE BRANCH WRITTEN TO STOP ONE FAILURE BECOMES THE THING THAT HIDES AN ABSENCE, AND THIS IS THE TRANSFERABLE HALF OF 2026-08-27's FINDING -- BETTER THAN THE BUG IT CAME FROM.** [...]
- **THE BLIND GREP THAT RETURNS A TRUE NUMBER FOR A NARROWER QUESTION THAN THE ONE ASKED (vc's framing, banked 2026-08-27 in vc's terms because mine were worse).** Sweeping the plugin family for its `bin/` coupling, `grep 'INTENT_ROOT/bin/'` returned **TWO files and read as a complete answer**.
- **`.prettierignore` IS GITIGNORE SYNTAX, WHERE A SINGLE `*` DOES NOT CROSS A `/`, AND A FENCE VERIFIED ONLY BY A GREEN IS NOT VERIFIED.** The obvious narrow list fences less than it looks like.
- **A FENCE VERIFIED ONLY BY A GREEN IS NOT VERIFIED.** [...]
- **A LOG LINE IS NOT A HEAD MOVE, AND ON A FIVE-NODE BRANCH THE NEWEST COMMIT IS USUALLY NOT YOURS.** On the bounce I read `git log --oneline -1`, saw ic's sha where I had left my own, and was one step from reporting a reset of a shared HEAD.
- **A PIN IS RETIRED BY A SCHEMA BUMP, AND IT FAILS ONLY ON THE WRITE PATH.** `pair-f7240814` spoke store schema 13; ic's WP-14 took the store to 14 at `9bd6b0a3`.
- **QUOTE CHARACTER IS NOT A PROSE/COMMAND BOUNDARY IN SHELL, AND I NEARLY INSTITUTIONALISED THAT IT WAS.** I had a fix queued that stripped backticked AND double-quoted spans as prose. [...]
- **ONE COMMIT IS NOT ONE MOMENT, AND THE GATE READS THE WORKTREE.** This board already said _both sides move in one commit_ and that is insufficient.
- **A TIME THAT CAME OUT OF A TOOL CARRIES WHATEVER ZONE THAT TOOL CHOSE, AND APPENDING `Z` IS AN ASSERTION, NOT A FORMAT.** `stat -f '%Sm'` prints LOCAL.
- **A GREEN THAT CANNOT TELL WHAT IT NAMES FROM SOMETHING ELSE PRODUCING THE SAME GREEN.** [...]
- **`ac new` IS AN IDEMPOTENT PUT, NOT AN INSERT, AND IT REWRITES `state` FROM `--kind` WHOSE DEFAULT IS `non-test`.** Amending a `kind: test` criterion without passing `--kind test` silently flips it and breaks its AT's coverage.
- **AND `at new` IS THE SAME CLASS ONE NOTCH WORSE: IT HARDCODES `note: None` AND `legacy: None`, SO THERE IS NO CORRECT INVOCATION AT ALL.** `ac new` rebuilt `state` from `--kind` -- dangerous, but a flag existed to get it right, and reading the source first is what saved me.
- **AND I THEN OVERCLAIMED IT INTO A BLOCKER, WHICH IS THE WORSE HALF AND IS MINE ALONE.** From _no verb writes `note`_ -- true -- I reported to vc, cc and ic that the work **could not be done** and that there was _no sequence of today's verbs_.
- **THE GENERAL FORM, AND IT IS THE DAY'S CLASS ARRIVING IN MY OWN WORK RATHER THAN IN SOMEONE ELSE'S: A TRUE ANSWER ABOUT THE MECHANISM YOU SEARCHED, PUBLISHED AS AN ANSWER ABOUT WHETHER THE THING CAN BE DONE.** [...]
- **AND IT ARRIVED FIVE TIMES IN ONE SESSION, ALL IN INSTRUMENTS I BUILT, SO IT IS NOT A LAPSE -- IT IS MY DEFAULT SHAPE.** [...]
- **THE CHEAPEST CORRECTIVE, DERIVED FROM ALL FIVE: WHEN THE SUBJECT REPORTS ON ITSELF, READ ITS REPORT BEFORE BUILDING A SECOND OPINION.** The critic prints its own census; `git` prints what it committed; `shellcheck` prints which check fired.
- **A FIELD IS ONLY DEAD IF SOMETHING SAYS SO, AND THAT CHECK IS WHAT SEPARATES A DATA-LOSS BUG FROM A DELIBERATE SUNSET.** Before reporting the above I tested the opposite hypothesis: `note` might be a migration-only field being retired, in which case nulling it is BY DESIGN.

### FAMILY 5 -- THE GATE THAT DID NOT RUN, AND THE VERB THE NAME SENDS YOU TO

**ic's class, consolidated 2026-08-26 from five instances and grown to ten: A CONSTRAINT NAMES THE ACTION A HUMAN PICTURES WHILE THE GATE DOWNSTREAM READS A STATE.** Its sharpest corollary, which cost a blocked lane today: **a check that did not run is indistinguishable from one that passed** -- and the unguarded path is repeatedly the one the name sends you to. **hv's fix list lives here.**

- **THE TWO STAMP INSTRUMENTS ARE INTERCHANGEABLE FOR PROVENANCE AND NOT FOR VIABILITY, AND THE VIABILITY HALF HAS NO ENTRY ANYWHERE ELSE.** [...]
- **ARM 7 ASSERTS ORDER AND NOTHING ASSERTS LATENCY, WHICH IS TRUE, INSUFFICIENT, AND LOOKS SUFFICIENT.** It proves the dirt verdict is taken before the first cargo invocation. [...]
- **A GATE WHOSE SUBJECT IS A FILE FIVE WRITERS SHARE HAS A BLAST RADIUS NONE OF ITS ARMS DESCRIBE.** [...]
- **SYNC CANON FIRST -- IT READS THE WORKTREE -- THEN COMMIT THE FILE AND ITS CANON TOGETHER, AND THE OBVIOUS ORDER IS THE WRONG ONE.** `canon_commit_check.sh` blocks a commit whose canon names bytes it does not carry, because canon stores an attachment's FULL TEXT rather than a hash. [...]
- **A CENSUS THAT EXAMINES ONE OF FIVE AND REPORTS GREEN IS THE FALSE GREEN THE GUARD EXISTS TO REFUSE.** Arm 10's first draft printed `(1 examined)` against a tree carrying five, and PASSED -- so the count is printed and is the first thing to read. [...]

- **A `bin/`-ONLY PUSH IS LINTED BY NOTHING IN CI, AND THE BOOT FILE'S STATED BACKSTOP FOR IT IS FALSE.** [...]
- **WHEN CLIPPY IS WIRED INTO THE RELEASE VERB, THE GATE IS THE FLAG, NOT A WARNING COUNT.** The verb runs zero clippy today (ic, confirmed). [...]
- **THE RELEASE VERB NEVER TESTS THE TREE IT TAGS.** `preflight()` holds every gate and runs ONCE at `:482`; every mutation -- version stamp `:499`, `agents sync` `:663`, `claude upgrade --apply` `:690` -- happens after it; `:770` tags. [...]

### FAMILY 1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT

**THE HEAD, ARRIVED AT 2026-08-25 AFTER SIX INSTANCES IN ONE EVENING AND PAID FOR MANY TIMES SINCE: AN INSTRUMENT THAT RETURNS THE SAME ANSWER WHATEVER THE SUBJECT DOES HAS MEASURED NOTHING, AND ITS OUTPUT IS INDISTINGUISHABLE FROM A REAL RESULT.**

**THE TEST: RUN IT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Not _check your instrument_, which is unactionable. **Drive the case that MUST come back different.** Corollary paid for 2026-08-26: **a control that exercises only the arm that FIRES says nothing about the arm that must FALL SILENT**, and the silent arm is usually the whole value.

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
- **Line-index surgery must be bottom-up WITHIN a block, not only across sections** -- an insert above a later delete shifts it, and **every pre-mutation assert still passes, because it ran before the shift.** [...]
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

- **NEVER `$?` AFTER A PIPE.** `cargo test` needs `--no-fail-fast`. **`grep -c` exits 1 on zero**, so `grep -c ... && ok` inverts, and `|| echo 0` fires too and yields `0\n0`. **Compare the NUMBER, never the exit status.**
- **The Bash tool's shell is ZSH:** unquoted `$var` does NOT word-split (a `for` over one runs once and looks finished), an **unmatched glob ABORTS the whole command**, and an apostrophe inside a single-quoted `perl -e` runs nothing.
- **`2>/dev/null` converts a broken probe into a clean answer.**
- **A backtick inside a quoted grep pattern is a command substitution.**
- **`find` walks into `target/` and dies at the timeout, returning a partial answer that reads as complete.** Scope to `native/rust/crates`.
- **`find` here is bfs, not GNU find** -- `-newermt` is REFUSED and prints 0 at exit 0 under `2>/dev/null`.
- **A `sed -i` that matches nothing is a no-op, and a no-op `sed` exits 0** -- a write that did not happen reports success. **The address is half the match**, and a bare line number is a second silent predicate that decays the moment a line is inserted above it.
- **`bash -n` is wrong in BOTH directions and neither is visible from its output** (half vc's).
- **`--help` IS NOT A PROBE** -- under INV-07 it exits 0 whether or not the command is built.
- **`intent info | head -1` is the PRODUCT BANNER, identical from every cwd BY DESIGN.** I nearly withdrew a correct fix over it.
- **A grep's zero -- and a case-sensitive miss -- is a claim about the CORPUS.** Positive-control before believing it.
- **`find`-based populations describe the WORKING TREE, never the commit** (ic, `run_tests.sh:89`).
- **`grep -n` on a SINGLE file emits `<line>:<text>` with no filename**, so a `cut -d: -f3-` copied from a multi-file call eats the text.
- **A truncated line preview answers about the line's PREFIX, not the line** -- on this board's prose-length lines that is almost always the wrong question.
- **`ps | grep '[g]it commit'` is not a probe for a running git in this estate** -- it matches a peer LLM session whose prompt quotes the command. [...]
- **A version defect can return the correct answer, exit 0, and put the error only on STDERR** (bash 3.2 `declare -A`).

### FAMILY 2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF IS WHAT CARRIES THE FALSE ONE

- **MY EVIDENCE WAS ENTIRELY TRUE AND MY SUBJECT WAS WRONG, AND THAT COMBINATION HAS NO TELL.** Every fact I cited checked out; the thing they were facts ABOUT was not the thing under discussion. **A reader verifying the evidence confirms it and learns nothing about the error.** [...]
- **A FALSE CLAIM THAT LATER BECOMES TRUE FOR A DIFFERENT REASON IS THE WORST WAY FOR ONE TO AGE.** `bc38db85` said cc's half was met; it was wrong when written and true an hour later, for `1583d1ad`. **There is then nothing left to correct and no trace it was ever wrong.** [...]
- **A SCOPE RULING SILENTLY NARROWS YOUR SEARCH POPULATION, AND NOTHING ANNOUNCES WHEN THE RULING EXPIRES.** I told hv the install artefact DID NOT EXIST. [...]

- **NOTHING CONNECTS A DECISION TO THE CONDITION IT WAS MADE UNDER, AND A DEFECT'S DISAPPEARANCE ANNOUNCES ITSELF TO NOBODY.** [...]

**A CONFIDENT UNMEASURED NUMBER IN A _RATIONALE_ IS LOAD-BEARING IN A WAY ONE IN A _REPORT_ IS NOT -- A REPORT GETS CHECKED, A RATIONALE GETS HONOURED** (vc's formulation). All three of my shipped defects are instances.

**AND ITS INHERITED FORM, PAID FOR 2026-08-26 AND WORSE THAN THE SELF-INFLICTED ONE: A PEER'S MEASUREMENT ARRIVES WITH A CAUSE ATTACHED, AND THE CAUSE IS THE PART NOBODY MEASURED.** vc's _167 tracked views rewritten_ was real. _By read verbs_ was the attribution, and it was the load-bearing half. I put the whole sentence into this board as established fact and built a release-note recommendation on it. **Worse, I then read my OWN result through the borrowed premise instead of testing it.** My fixture ran the PAIR and never once ran the 3.0.0 keg, so I held exactly zero evidence about the keg -- and I wrote vc the sentence _YOU MEASURED THE SHIPPED KEG; THE PAIR DOES NOT DO IT_, which asserts a CONTRAST whose other half I had not measured. The half that was mine (_the pair does not rewrite on read_) was sound and helped kill the claim. The half that made it a finding was borrowed. **A measurement handed to you is evidence; the cause attached to it is a hypothesis wearing the measurement's authority, and quoting the number is what launders the second into the first.** Restate a peer's figure with its cause marked as theirs and unverified, or measure the cause yourself before it becomes a premise in your own file.

- **`bin/intent3` claimed a coherence check would be "a MULTI-SECOND gate on every command".** Driven: **~85ms end-to-end**, against a component sum measured separately at ~110ms. **The two do not reconcile and I have not resolved them; neither is within two orders of magnitude of the claim.** [...]
- **`cmd/hosting` claimed "in a throwaway clone a mutator harms nothing".** Never driven. Every verb ran from the LIVE tree's cwd and it emptied the durable store.
- **`currency.lib` PRINTED A FLOOR AS A DISTANCE** -- the overclaim in the error message of the file written to refuse overclaims, **and the fix for it added an arm nothing drove** (ic). **A fix that adds an arm adds a thing to drive.**
- **A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED** (ic, against themselves).
- **MY TELL vs cc's, AND THEY ARE MIRRORS:** I publish the claim the measurement INVITES (wrong at the READING); cc drives the measurement and publishes a stronger claim about what it measured (wrong at the WRITING). **Both rest on a real number, which is why neither gets challenged.**
- **A ROW CARRYING ONE TRUE SENTENCE AND ONE FALSE ONE IS HARDER TO CATCH THAN A WHOLLY WRONG ROW, BECAUSE THE TRUE HALF IS WHAT A READER CHECKS FIRST AND IT HOLDS** (cc, on my false roster row).
- **`checked against the schema` VERSUS `checked against a grep of the source` IS THE WHOLE DISTANCE, AND NEITHER OF US COULD SEE IT IN OUR OWN SENTENCE AT THE TIME OF WRITING.**
- **A STABLE CONCLUSION ACROSS THREE WRONG POPULATIONS IS NOT CORROBORATION -- IT IS THE POPULATION NOT BEING LOAD-BEARING FOR THAT CONCLUSION** (cc, on vc's withdrawal).
- **A HAND-MAINTAINED SET THAT NOTHING CHECKS IS THE ROSTER PROBLEM,** and I built one while explaining it. **Manifest plus a drift check, and a path in NEITHER the manifest nor a declared-exclusions list is an ERROR rather than a judgement call** (vc).
- **UNCHECKED IS NOT EMPTY, AND ONLY THE OUTPUT CAN CARRY THAT DISTINCTION.** An instrument that measures a narrow scope and reports in the vocabulary of the general category produces a **true sentence and a false belief**, so nothing it can check is wrong. [...]
- **A RULE IS HONOURED BY WHOEVER LEARNED IT AND DOES NOT PROPAGATE BY HAVING BEEN STATED.** `precommit` has 14 guard arms at three strengths; **7 assert a repository finding on ANY non-zero exit.** [...]
- **A SUPERSEDES NOTE APPENDED BELOW WHAT IT SUPERSEDES LEAVES THE DEAD CLAIM AS THE HEADLINE.** `acceptance.md:199` still OPENS with the green arm vc amended away, with the correction far below it. [...]
- **A GUARD CAN HAND OUT A REMEDY THAT CAUSES THE DEFECT ANOTHER GUARD EXISTS TO PREVENT, AND NEITHER CAN SEE IT.** [...]
- **A WRONG LINE NUMBER IS ONLY A TRAP IF FOLLOWING IT CAN REACH A GREEN.** vc warned the 3.2 error at `:216` would lead a fixer to gut the exemplar. Built it: the error MARCHES to `:225` and stays there. **Misleading, not trapping -- and the marching is what tells you to stop chasing and look up.**

- **A CORRECT MEASUREMENT DESCRIBED IN THE WRONG TERMS TRAVELS AS THE WRONG RULE, AND THE OBSERVATION BEING TRUE IS WHAT STOPS ANYONE CHECKING.** Twice in a day, both mine. [...]

- **A NUMBER WHOSE SUBJECT IS NOT ON THE LINE CANNOT BE CHECKED BY THE NEXT READER.** vc's board carried `Gate still PASS at 67 of 67`; **no thread gate is 67** (64/134, 51/53, 2/6). It was `ST0057` 51/51 **plus** `ST0056/03` 16/16 -- **two thread gates summed, with nothing on the line saying so.** [...]
- **A SUPERSEDES BANNER LEAVES TWO VALUES IN ONE HOME, AND THE TRUE HALF IS WHAT MAKES THE FALSE HALF READABLE.** [...]

- **A `git status` IS PERISHABLE IN EXACTLY THE WAY A TIMESTAMP IS, AND MINE WAS STALE AT BIRTH RATHER THAN EXPIRED.** I told vc _none of the dirty files are mine, my paths are clean_ while quoting the BOOT-TIME status -- having edited my own board two calls earlier. [...]

### FAMILY 3 -- ROUTING, RELAY AND ATTRIBUTION

- **THE OFFER IS THE MOMENT TO CHECK, NOT THE SEND.** Once two messages are in flight the only tools left are racing and deduping, and only one works. Three instances, one mechanism.
- **A FORWARDING OBLIGATION DISCHARGED BUT NOT CANCELLED MANUFACTURES FALSE CORROBORATION** -- it arrives TWICE wearing TWO authorships and **the second announces itself as agreement. Tell the RECIPIENT to expect one copy; the recipient can dedupe, the relay can only be beaten.**
- **VOLUNTEERING A RELAY _IS_ CREATING THE OBLIGATION, AND THE COORDINATOR HABIT IS TO VOLUNTEER** (vc). A rule only the receiver can apply arrives one step too late.
- **NAME WHICH _HALF_ YOU ARE ATTRIBUTING** (vc, wholly theirs). An incident and its generalisation are separable and usually have different authors. **Nobody audits an attribution.**
- **A PEER TELLING YOU WHAT ONLY THE SOURCE CAN SETTLE IS TELLING YOU WHAT _THEY READ_** (ic). Fold instructions, quoted rows, relayed rulings alike.
- **A RULING DELIVERED AS A SELECTION AMONG OPTIONS YOU AUTHORED IS ONE WHOSE WORDING IS YOURS AND WILL BE QUOTED AS THEIRS. RECORD THE OPTIONS, NOT THE OUTCOME**, in an inbox where it survives the session.
- **A CORRECT ANSWER ABANDONED ON A PEER'S SAY-SO IS WORSE THAN THE PEER'S WRONG ANSWER, AND IT IS THE HALF YOU CONTROL.** I had the attribution start date right at 2026-08-24. vc said 08-21. [...]
- **I REFUSED TWO RELAYED AUTHORISATIONS TODAY AND BOTH REFUSALS WERE UPHELD.** vc relayed hv's ruling that the attribution guard was mine to build; hv had ruled it in **vc's** session answering **vc's** question, and hv has me holding in mine. [...]
- **THE INVERSE ALSO HELD: I RELAYED AN hv RULING TO vc AND FLAGGED IT AS A RELAY, AND TOLD THEM TO CONFIRM IT AT THE SOURCE. THEY DID, AND IT HELD.** Quote the ruling VERBATIM rather than characterising it -- a ruling paraphrased in your words gets quoted back as theirs.
- **THE BLOCKED PARTY TELLS THE BLOCKER, NOT THE SEQUENCER.**
- **A PROOF ONLY ITS AUTHOR CAN REPRODUCE IS NOT YET A PROOF THE ESTATE HOLDS** (ic). Survives the soundness being conceded. **Script the repro.**
- **A PEER'S READY-MADE COMMAND IS NOT AN APPROVAL, AND CONVENIENCE IS WHAT MAKES IT SLIP.** Distinct from a relayed approval: the peer claims no authority and is simply being helpful, so there is nothing to challenge and nothing that reads as a demand. [...]

### FAMILY 4 -- THE SHARED CHECKOUT

- **`git add` THEN `git commit --only` IS NOT ATOMIC ACROSS THE INDEX LOCK, AND THE SECOND ERROR MASKS THE FIRST.** Landing this very fold, five nodes folded at once on hv's instruction and ic's `git commit` held `.git/index.lock` through its whole pre-commit gate. [...]

- **`MM` IS A CLAIM ABOUT THE INDEX'S CACHED STAT, NOT ABOUT CONTENT, AND UNDER LOCK CONTENTION IT SAYS `staged` ABOUT A FILE WITH NOTHING STAGED.** My board commit collided with a peer's git process (`fatal: Unable to create '.git/index.lock'`) and the commit's own post-commit cleanup never ran. [...]

- **`git commit --only <paths>` IS NOT A NICER SPELLING OF CHECKING THE INDEX FIRST -- IT IS THE ONLY FORM THAT IS ATOMIC WITH RESPECT TO PEERS.** Reading `git diff --cached --name-only` **measures a MOMENT, and the commit happens at a different one.** [...]
- **THERE IS NO SUCH THING AS A WORK-IN-PROGRESS EDIT TO A GATED GUARD IN A SHARED CHECKOUT.** The pre-commit gate runs the **WORKTREE** copy, so every intermediate save is instantly live for four other nodes. [...]

- **`awk ... > tmp && mv tmp file` DROPS THE MODE, SILENTLY, AND EVERY CHECK I OWN IS BLIND TO IT.** I used it three times, chmod'd two by reflex, and left `bin/.devbin/cmd/precommit` at 644 -- **every `git commit` in the repo failed until ic drove it to the cause.** [...]
- **THE REMEDY FOR A SHARED-INDEX HAZARD CAN BE THE NEXT OUTAGE, AND BOTH MOVES WERE INDIVIDUALLY RIGHT.** vc flagged five staged files as a loaded gun; I agreed and unstaged; **unstaging left a live roster row naming an untracked file and refused every node's commit.** [...]
- **PROVE AN UNBLOCK WITH A REAL COMMIT, NEVER BY ASSERTING IT.** I asserted twice and was wrong twice in twenty minutes. [...]

- **FOUR WRITERS, ONE TREE, ONE INDEX.** Peers' dirty files are in every `git status` you read. **Name yours; `--only` is PATH-scoped, never hunk-scoped.** Peers commit concurrently -- **wait a lock out, never clear it.**
- **AN UNCOMMITTED EDIT WAS ERASED HERE WITH NO REFLOG TRACE AND hv RULED IT ACCEPTED RATHER THAN INVESTIGATED.** Live and undiagnosed BY DECISION. **A protective copy OUTSIDE the tree costs nothing; staging is NOT the remedy, because the index is shared.**
- **A FIGURE ABOUT HEAD BELONGS IN THE SINGLE-WRITER CLONE** (`int suite`, `int hosting`). A figure about the WORKING TREE must be defended by attribution instead.
- **`bin/` IS dc's LANE** (hv). `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED and not this repo's to edit.**
- **A MONIKER NAMES WHERE A SESSION LIVES, NEVER WHERE ITS BYTES LAND.** `devbin/vc` works in `~/Devel/prj/Devbin` and its fleet sweep writes into **eleven checkouts including this one**. Five paths here went dirty mid-commit with no announcement, and **nothing on the wire marks the crossing.** [...]
- **`--only` IS WHAT MADE THAT A QUESTION RATHER THAN AN INCIDENT** (devbin/vc's words, and they are right). A bare `git add -A` sweeps a fleet vendor into your commit **silently**, and neither party learns until the log reads strangely.
- **THE FORMATTER IS A SECOND WRITER BETWEEN THE SYNC AND THE COMMIT, AND IT FIRED AGAIN TONIGHT.** It realigned a table I had just synced, so canon named bytes the file no longer held and the gate refused at `ADDS 1`. **Order: write, LET THE FORMATTER SETTLE, sync, commit.** [...]
- **THE SUBJECT CAN MOVE BETWEEN YOUR TWO READS, AND THE SECOND READ LOOKS LIKE AN ANSWER.** `git diff --numstat` reported changes to an issue's canon; `git diff` seconds later printed nothing. **Not a broken instrument -- a peer committed in between.** [...]
- **COMMITTED IS SELF-DESCRIBING; HELD IS NOT** (mine, adopted into devbin's sweep procedure at their `c6c30f9`). A committed change carries an author, a message and a log entry. [...]
- **A FILTER OVER A COMMAND'S OUTPUT MUST BE ABLE TO EXPRESS ITS FAILURE, NOT ONLY ITS SUCCESS.** [...]

- **A SCRATCH DIRECTORY INSIDE A GUARDED TREE IS INDISTINGUISHABLE FROM IN-FLIGHT SOURCE TO EVERY GUARD WE OWN.** [...]
- **THE ISOLATION YOU REACH FOR TO AVOID DISTURBING A PEER'S BUILD IS EXACTLY WHAT INVALIDATES YOUR RESULT -- TWO NODES, ONE DAY, ONE INSTRUMENT DEFECT.** ic manufactured 13 false failures in the morning and cc 2 in the afternoon, both from a `CARGO_TARGET_DIR` in scratch. [...]

## Decisions

- **2026-08-29 -- CORRECT YOUR OWN ESCALATION BEFORE IT REACHES THE PRINCIPAL, ESPECIALLY WHEN THE ERROR MADE YOUR ASK LOOK BIGGER.** I escalated the fiat exit as "hv ruled one verb and the invariant demands a second". [...]
- **2026-08-29 -- WHEN AN INSTRUCTION ASKS FOR SOMETHING AN INVARIANT FORBIDS, MEASURE IT AND SAY SO; DO NOT BUILD THE HALF THAT PASSES TODAY.** vc asked me to shape the entry edge so the exit could follow. It cannot: entry alone makes a trap state. [...]
- **2026-08-29 -- READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT.** I noted the `///`-is-published rule, then shipped `D42` into a published doc twenty minutes later, then shipped a thread id into a shipped string literal an hour after that. [...]

- **2026-08-28 -- A DEFECT'S FILED WIDTH IS NOT ITS REAL WIDTH, AND THE OBVIOUS FIX FOR THE FILED WIDTH CAN BE WRONG RATHER THAN MERELY SHORT.** `(k)` was filed on `languages: []`, for which the renderer ALREADY had `[[#nolang]]` -- so "two template blocks" looked like the whole job. [...]

- **2026-08-28 -- A COST ESTIMATE IS A CLAIM AND IS OWED THE SAME MEASUREMENT AS ANY OTHER.** I told vc the AC-07.6 migration arm "needs a v2 estate fixture that does not exist yet" **without opening the file.** [...]

- **2026-08-28 -- A PARITY TOOL UNDER `intent/st/ST0056/parity/tools/` IS AN INLINE CANON ATTACHMENT, AND EDITING ONE HAS A COMMIT ORDER THE OBVIOUS SEQUENCE GETS WRONG.** [...]
- **2026-08-28 -- NOT EVERY NEW TEST ARM SHOULD FAIL THE CONTROL, AND CLAIMING OTHERWISE WOULD BE THE FLATTERING LIE.** Driving the pre-fix code red 4 of 5 new arms; the fifth asserts the OLD check survives the change, so it must pass under both. [...]
- **2026-08-28 -- ZSH BIT THREE TIMES IN ONE SESSION AND EVERY ONE WAS ALREADY WRITTEN DOWN.** [...]
- **2026-08-28 -- ~~I CHECKED WHOSE THE RED WAS BEFORE REPORTING IT~~ STRUCK. I banked my own error as the day's lesson learned; cc refuted it and I verified the refutation myself.** Full arc archived; records `0120e8a5`, `92570169`, `e9e71246`. **Two classes survive it and both are the keepers:**
  - **EXONERATION AND ATTRIBUTION ARE TWO CLAIMS AND I MEASURED ONE.** "My diff does not render the subject" establishes it is NOT MINE and says nothing about whose it is. [...]
  - **AN EXPECTATION WHOSE PREMISE A LANDED DECISION RETIRED, WITH NOTHING CONNECTING THE DECISION TO THE EXPECTATION.** Three costumes: cc's `Super_Seded` control (stale GREEN), my `view_skew_check` arm (stale RED for a day), `dispatch_ssot`'s bootstrap probe. [...]

- (2026-08-28) **vc CORRECTED MY ROLLBACK FINDING AT THE RIGHT LEVEL AND IT GENERALISES: THE CARRIER IS DERIVED FROM A TRACKED TEMPLATE, SO THE GENERATOR IS WHAT YOU PRESERVE AND THE OUTPUT NEVER WAS.** I reported the gitignored carrier as having no rollback. [...]
- (2026-08-28) **I FILED THREE ISSUES WITHOUT RUNNING `intent issues list`. ONE WAS A DUPLICATE OF AN OPEN ISSUE; ONE WAS REFUTED BY MY OWN COMMIT OF THE DAY BEFORE, RECORDED IN THIS BOARD'S OWN DOING.** cc's reading is the keeper: **the board is a place I WRITE rather than a place I READ.** [...]
- (2026-08-28) **A TRUE POSITIVE DISCARDED AS FALSE COSTS AS MUCH AS A FALSE ONE BELIEVED.** `int hooks` called this tree's carrier STALE and was RIGHT; I filed the REMEDY's blast radius as a defect in the DETECTION. The carrier stayed stale six more hours. [...]
- (2026-08-28) **THE REHEARSAL'S VALUE WAS IN THE THREE INSTRUMENTS I CHECKED, NOT IN THE STEPS I RAN.** A clone would have been blind to the hook-door question by construction; cc's manifest would have written into my decoy; vc's cold-clippy warning was inverted. [...]

- (2026-08-27) **hv AUTHORISED BOTH DISK SWEEPS DIRECTLY, AND THE SECOND ONE REVERSED A RULE I HAD WRITTEN MYSELF.** [...]

- (2026-08-27) **AC-12.1's CLASS (2) WIDENED IN THE STORE RATHER THAN ARGUED ON A BOARD, ON vc's RULING: _record it against the criterion now, build nothing._** The criterion as written could be discharged green while its subject stayed broken, **and a criterion that can be satisfied while its subje** [...]

- (2026-08-27) **hv CHOSE TEST-TARGET CONSOLIDATION FROM A MENU I AUTHORED, ON A DIAGNOSIS I LATER CORRECTED.** I had the 167 test targets as the CAUSE of the blowup; the cause is **builds retained x size per build**, and consolidation addresses one term of the second factor. [...]

- (2026-08-26) **VERIFY RESOLUTION, NEVER ENUMERATE POSITIONS.** `use` repoints what it owns and then measures what actually answers `intent`, dying if it is not the flavour asked for. **That catches PATH 19, and any position nobody has found yet, WITHOUT NEEDING TO KNOW IT EXISTS.** [...]
- (2026-08-26) **`use` LIVES IN `cmd/local`, ON HIGHLANDER GROUNDS, AND I TOOK IT BACK FROM cc TO PUT IT THERE.** hv assigned it to both of us in separate sessions. [...]
- (2026-08-26) **CHECKING WHICH BINARY RESOLVES AND CHECKING WHETHER THE DOOR OPENS ARE DIFFERENT PROPOSITIONS** (cc, and it is the thing my design was missing). [...]
- (2026-08-26) **`publish` STOPS WITH hv.** hv authorised vc to direct my work on other projects, and I take that as covering fleet work and NOT a public push to a tap. **Stated by me rather than waited for.**

- (2026-08-25) **D42 AMENDED BY ME, AGAINST MYSELF: THE RULE IS THAT NO CALLER AUTHORS A STAMP.** The signature test -- _no function TAKES a time_ -- is one sufficient condition and was never the definition. [...]
- (2026-08-25) **AT-11.6's TWO REAL DEFECTS WERE BOTH FOUND BY LIVE INCIDENTS, NOT BY REVIEW OR MUTATION.** It shipped with nine arms and four mutations. [...]
- (2026-08-25) **`--only` IS THE ONLY COMMIT FORM ATOMIC WITH RESPECT TO PEERS.** Reading `git diff --cached` first measures a MOMENT; the commit happens at another. I read exactly three files and committed four. **The pathspec is the control; the reading never was.**
- (2026-08-25) **vc HAS hv's PEN AND hv SAID SO IN MY OWN SESSION**, which is what makes it different from the two relayed authorisations I refused yesterday. [...]
- (2026-08-25) **vc's OPENING ASSIGNMENT WAS WRONG AND THE CONTRACT CAUGHT IT.** `claude ws` was routed to me as WP-07 work off ic's surface probe; **it is WP-14's (`AC-14.10`), and `AC-14.7` says every `/in-whiteboard` verb is served by `intent wb` FROM THE STORE.** [...]
- (2026-08-25) **AC-07.2 INVESTIGATED AND DELIBERATELY NOT STARTED.** Writing `hook_compat.rs` dirties `native/rust`, which is the exact subtree four nodes are waiting to see clean and the only route back to a current shared binary. **My own guard's predicate, applied to my own next task.** [...]
- (2026-08-25) **A TEST'S POPULATION IS NEVER READ FROM THE THING UNDER TEST.** `hook_compat.rs` assembles its hook roster from the shipped scripts and `settings.json` and never from `install::HOOKS`. [...]
- (2026-08-25) **I DID NOT WIDEN A PEER'S GUARD TO CLOSE MY OWN FINDING.** `every_declared_hook_ships_as_a_script` is one-sided and lives in cc's module. My file covers that direction from the SURFACE, so the class closes either way. [...]
- (2026-08-25) **THE PRECONDITION I WROTE AGAINST MYSELF ON 2026-08-25 EXPIRED AND I CHECKED RATHER THAN INHERITED IT.** _Do not start AC-07.2 while `native/rust` must go clean_ was correct while four nodes were waiting on a clean subtree to close the gate. [...]
- (2026-08-25) **A `dirty-` BINARY MAY READ CANON AND MAY NEVER WRITE IT (vc's rule, and it binds me).** Reading is safe because **the STORE is the subject and is independent of the build**; writing or certifying a criterion is not, because the write is performed by the code that is mid-edit. [...]
- (2026-08-25) **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS **NOT** IN IT ARE NOT IN IT.** This is the sweep's durable output and it makes the class checkable by READING. [...]
- (2026-08-25) **THE INVERSE OF THE `session-finish` CLASS, AND IT IS WORTH NAMING BECAUSE I WENT LOOKING FOR THE WRONG ONE.** session-finish was _canon says the name exists and the door refuses it_. [...]

- **RETIRED TO `.history/20260828/wip.md`: the five-entry `(C)`-exhaust block (2026-08-25).** hv closed the subject in one line -- _"I DO NOT WANT ANY CLAUDE EXHAUST IN MY COMMITS. EVER."_ -- and the guard it implies is already specified in TODO with its four conditions. [...]

- (2026-08-25) **vc AS CONTRACT STEWARD AMENDED AC-11.6 UPWARD ON MY ROUTING, AND MY PROPOSAL CARRIED AS WRITTEN** (`f68d397c`). **Their green arm was the cause of the row being unbuildable and they said so.** [...]
- (2026-08-25) **THE ROUTE FAILURE ON AT-11.6 IS MINE AND IT IS THE BETTER EXHIBIT.** I wrote _routes to vc_ in the same paragraph as the conflict, **on a board that archives at the fold, and then did not route it.** Four days invisible. [...]
- (2026-08-25) **hv RULED: fix both trees LOCALLY, commit, and DO NOT PUSH v2.** _"The checked out v2 branch is only being used locally here by projects on this machine ... [...]
- (2026-08-25) **hv CHOSE THE ROUTE-LEVEL DECLARATION over a per-file exception list.** Branch not taken: an entry per fix, which the guard's own text forbids and which would be FALSE of every file it named -- not PENDING (the v2 landing is done) and not V3-ONLY (v2 received it).
- (2026-08-25) **THE ENFORCEMENT LOSS IS A COST OF hv's NO-PUSH RULING, NOT OF THE ENGINEERING, AND NOBODY PRICED IT WHEN THE DECISION WAS MADE** (vc's framing). Routed to hv rather than absorbed in a test file. [...]
- (2026-08-25) **A PEER'S FULLER QUOTE OF A RULING IS STILL THE PEER.** vc supplied the clause they had dropped -- the one authorising the commit -- and told me not to treat their reading as my authorisation. **I held for hv's word in my own session and vc was right that I should.**

**Decisions dated 2026-08-24 and earlier are archived at `.history/20260828/decisions-pre-0825.md`** -- their subjects shipped (the v2 freeze scope, `intent3`'s refusal, the `v2-maintenance` deletion, the two-mechanism convergence). **Retired because each named its condition and the condition died.**
