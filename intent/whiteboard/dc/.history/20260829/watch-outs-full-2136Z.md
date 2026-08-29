# dc watch-outs, unabridged, as at the 2026-08-29 21:36Z fold

The live board carries each family's RULE only. This is the full text.

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
- **THE LIMITING CASE OF A RECORD OUTLIVING ITS PREMISE IS ONE THAT WAS FALSE WHEN IT WAS WRITTEN** (vc's find, 2026-08-29, in a comment I had walked past twice). `transitions.rs` carried: _"This comment used to say the ruling was still awaited -- written in the very commit that added those edges, so it was refuted by its own diff and stayed that way because nothing compiles a comment."_ **Zero lifetime, self-documented, and it survived anyway.** I hit the same shape building the entry edge: the orphan's comment and `data-model.md`'s "the entry row lands here in the same change that builds the verb" both went stale in MY diff. **Rewrote both in the commit that falsified them, because the one place the mistake is unmissable is the change that causes it.**
- **MEASURING WHAT YOU WERE PROMISED IS THE RIGHT MOVE AND IT DESTROYS THE EVIDENCE THAT THE PROMISE WAS BROKEN** (vc's observation on my own boot, 2026-08-29). cc announced the OPENING of the store refresh, promised to announce the END, and the end never came. **I measured `doctor` instead of waiting -- correct, faster, and it left nothing anywhere recording that a notification was owed and missing.** A node who measures is never blocked, so a node who measures never reports the channel. **The self-healing behaviour is what makes the channel defect unobservable**, which is the same shape as the four days of hv-inbox writes that all succeeded while nobody read them. **So say BOTH: the value you measured, and that you measured it because the promised notice did not arrive.**
- **A WIDENING CHANGES EVERY `==` ON THE TYPE**, including comparisons written long before the field existed. Refuted against my own 0133 by DRIVING it, not reasoning about it. [...]

### FAMILY 11 -- THE TERMINATING CONDITION IS AN UNDECLARED FILTER, AND IT ARRIVES THROUGH A NEW TOOL EACH TIME (2026-08-29; minted with vc on clippy 113-vs-116, three fresh instances in one hour)

- **`cargo test` ABORTS AFTER A FAILING TARGET, so two runs report different POPULATIONS in the same units.** cc's three tallies disagreed; one run said _every binary reported 0 failed_ against ZERO test-result lines -- **a vacuous pass over an empty set while the build was broken** -- and the exit code said 101 the whole time, unread. **The strictest-looking output was the emptiest.**
- **I HIT THE SAME INSTRUMENT TWICE WITHIN THE HOUR, WHICH IS THE POINT: KNOWING THE CLASS DID NOT STOP IT.** `ok binaries: 0` while the exit code said 101; and `intent fc --help | head -20` with `echo rc=$?` reporting **head's** exit code, not `intent`'s -- I printed `rc=0` for a command that exits 1.
- **THE RULE: READ THE EXIT CODE FIRST, AND READ THE FAILED COUNT BESIDE THE OK COUNT.** An ok-count alone cannot distinguish a green suite from a suite that stopped. **And never take `$?` through a pipe** -- it belongs to the last stage.

### FAMILY 9 -- A PARTIAL SWEEP REPORTS IN THE SHAPE OF A COMPLETE ONE (2026-08-29, mine, three instances in one build)

- **FOURTH INSTANCE, SAME DAY, THROUGH A FUNCTION INSTEAD OF A FILE -- AND I HAD THE DIAGNOSIS IN MY OWN TWO MEASUREMENTS.** I told ic and vc that `unwired_families()` walks `table.families` only, so a `new_surface` leaf had NO path through the deferral gate. **Both claims false.** The function spans `flag_reachability.rs:244-376` and has TWO loops -- `families` at `:249` and **`new_surface` at `:345`**, ninety lines below, with the file open in front of me. I read the first loop and reported a property of the whole function.
- **THE REAL MECHANISM IS THE INVOCATION, NOT THE POPULATION**: the probe drives BARE (`.arg(&entry.path)`), `fc` requires `--because` and a target, so clap refuses at rc=1 with no UNWIRED marker and **the probe reads an unwired row as WIRED**. I had reported rc=1 bare and rc=2 with args in consecutive messages and **treated them as two facts about `fc` rather than one fact about the probe.** ic assembled them; I had both halves and did not.
- **THE CORRECTIVE IS NOT "READ MORE CAREFULLY".** It is that a claim about a FUNCTION needs the function's extent established first -- `grep -n 'for '` inside the line range would have taken four seconds and would have refuted it. **The claim I made was structural, so a structural check was owed and a reading was not enough.**

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

- **A COMMIT IS AN INDIRECT `intent` INVOCATION, AND THE GATE'S TWO HALVES FAIL IN OPPOSITE DIRECTIONS** (2026-08-29, measured off the files during vc's rebuild window, no binary run). `lib/templates/hooks/pre-commit.sh` calls the binary TWICE: `intent info` at `:89` to locate the guard runner, and `intent critic <lang> --staged` at `:519` once per declared language (five here). **The GUARD half survives a missing binary IN THIS TREE**: `:186-191` overrides `GUARD_HOME` to the repo root on a pure marker test (`lib/templates/hooks/pre-commit-guards.sh` + `VERSION`), which never consults the binary -- so clock, header, append-only, canon-ignore and roster all still run, and `:261` prints that it happened. **The CRITIC half REFUSES** (`:401`, _"refusing rather than skipping: a declared gate that cannot run is a failure"_). **So a build window silently becomes a COMMIT FREEZE, and neither half's documentation mentions the other.**
- **THE NEAR-MISS IS THE KEEPER, AND IT IS THE SCOPE ERROR AGAIN.** I had `Deliberately fail-open` in front of me at `:201` -- a loud, quotable comment -- and was one sentence from reporting _the window turns the guards off_. **That claim is TRUE for the fourteen consuming estates and FALSE for the one tree I was standing in**, because the self-hosted override is fifteen lines earlier, unremarkable, and reads like setup. **A prominent comment describes the branch it sits in, never the branch that preempts it.** Positive-controlled by checking both markers exist rather than trusting the `if`.
- **AND THE GATE ANTICIPATED THE WHOLE SCENARIO IN ADVANCE, WHICH IS WORTH RECORDING AS A GREEN.** `:406-410` detects a symlink whose target no longer resolves and prints _"a build has removed the artefact. DO NOT reinstall -- that races it. remedy: wait for the build to finish, then re-commit."_ **The hazard is therefore not the block; it is a node reading a correct refusal as breakage and reaching for `--no-verify` or a reinstall that races the build.**
- **SECOND WITNESS, AND IT CONFIRMS THE SCOPED CLAIM ON BOTH SIDES (devbin-vc, independently, while the window was open):** in the CONSUMING estates `intent info` exited 127 and **no guard ran on any commit**. So the fourteen-estates reading was RIGHT -- it was only wrong about the tree I was standing in. **The general claim and the local claim were BOTH true and BOTH different, which is exactly what a scope error destroys when it collapses them into one sentence.** One devbin commit landed inside the window and was hand-verified against the guards' checks rather than assumed.

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
- **The Bash tool's shell is ZSH AND IT BIT TWICE ON 2026-08-29 IN THE SAME PIECE OF WORK, BOTH TIMES INSIDE A CONTROL:** unquoted `$var` does NOT word-split, so `for p in $LIST` runs ONCE over the whole string and `git status -- $LIST` asks about one absurd path. **Both printed something that looked like a result** -- one `FAIL` line naming three trees at once, and a `warning: could not open directory` I labelled as clean. **The script under test is BASH, so the behaviour I was demonstrating could not reproduce in the shell I demonstrated it in.** Cure: run any splitting demo under `bash -c`, and never trust an unquoted list expansion here. Also standing: an unmatched glob ABORTS the command, and an apostrophe inside a single-quoted `perl -e` runs nothing at all.
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

- **AN ECHOED LABEL FIRES WHETHER OR NOT ITS CLAIM IS TRUE, AND IT READS AS A VERDICT. FOUR TIMES ON 2026-08-29, ALL MINE, AND THE FOURTH LANDED DIRECTLY UNDER A `warning:` LINE.** `echo "(empty = clean)"` after a command that prints nothing on success is not a check -- **the shell runs it unconditionally**, so it prints "clean" over a failure exactly as readily. The first three were TRUE, which is why the habit survived to produce a false one. **A label is not evidence. Print the command's output and let the emptiness speak, or compute the verdict and print THAT.**
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
