---
node: dc
name: DevX Claude
role: worker
session_id: bf63578d-c34e-4a6c-8c43-0190ae43aec3
heartbeat_at: 2026-08-25 09:15Z
status: active
focus: "**BOOTED 2026-08-25 09:15Z AFTER A COMPACT. hv SAID HOLD, AND A COMPACT DOES NOT END A SESSION.** **THE FOLD`S OWN FOCUS LINE WENT STALE INSIDE NINE MINUTES AND SAID THE OPPOSITE OF THE TRUTH:** it read _four commits land local and unpushed, the push is hv`s_, and **the push had already happened** -- both remotes sit at `851ae74a`, all four of mine are in, three peer commits sit on top. **I TOLD hv A PUSH WAS NEEDED ONE TURN AFTER IT WAS NOT.** What caught it was re-running my own probe WITHOUT `2>/dev/null`: the swallowed form printed `commits ahead of upstream: 0`, which READS as nothing-to-push and MEANT the command errored. **A STDERR REDIRECT TURNS A BROKEN PROBE INTO A CLEAN ANSWER, and it is the fifth instance of that family in one day.** CI run 32830353211 is IN PROGRESS at `9d9d186d` and DOES carry `5bfd5719`, so it is the first run that can exercise the drift fix -- **not green, and I am not calling it.** **MY APPROVED PLAN -- the precommit seven-arm class, then AT-11.6 -- IS STILL UNSTARTED.**"
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260824/wip.md`. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.
- **`stat` PRINTS LOCAL. `git log` PRINTS LOCAL.** Convert at the SOURCE and keep the local value beside it, or a reader appends a `Z` and lands an hour out. I did exactly that today and vc caught it.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **`intent` ON PATH IS v2.19.0 AND RESOLVES THROUGH `$INTENT_HOME` TO THE FROZEN `Intentv2`. v3 IS ALSO ON PATH, AS `intent3`** -- "DO NOT PUT v3 ON PATH" was retired 2026-08-22 by ST0058 and both restart files asserted it for two more days. The DISTINCT NAME is what leaves the fleet's gate untouched, by construction.
- **`intent3` NOW REFUSES A BINARY THAT CANNOT BE SHOWN TO DESCRIBE THIS TREE** (hv ruled 2026-08-24). Matrix and reasoning: `bin/.devbin/cmd/shared/currency.lib`, one home. **It keys on DECIDABILITY, never on dirt.**
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED** -- `intent ac status ST0057`, `intent ac status ST0056/03`, `intent ac gate ST0057`. It lived in THREE homes at THREE values on 2026-08-24, one document disagreeing with itself twice. **Do not put the number on this board; put the calls.**
- **hv's FREEZE SCOPE (2026-08-24): Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.** A v3-only defect is a v3-only fix; a shipped-surface one lands in BOTH trees.
- **THE INDEX IS SHARED IN THIS CHECKOUT.** `git add` puts your file where a peer's bare `git commit` sweeps it. Always `git commit --only <paths>` -- **but the refusal is a property of naming a FILE, not of `--only`, and I had it as unconditional.** Driven both arms: `--only <untracked FILE>` exits 1 with `pathspec did not match any file(s) known to git`; **`--only <DIRECTORY>` exits 0, commits the tracked edits and leaves the untracked file behind in silence** (ic). **The success output enumerates what it TOOK, so the omitted file has no line to be missing from** -- this board's own absence rule, arriving at a commit instead of a grep. **`git status --porcelain -- <paths>` after every commit; `??` is the entire signal.** My own `6096e14c` used the directory form and was clean, which was luck rather than care.
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** To ask about another clone, run ITS `bin/int`.
- **A DEVBIN COMMAND RESOLVES ITS PROJECT FROM SOMETHING OTHER THAN YOUR CWD -- BUT THE v3 BINARY RESOLVES FROM CWD.** Both are true and confusing them cost a live incident today.

## DOING

**Nothing in flight. Tree clean, inboxes at the sentinel, claims intact.** Yesterday's DOING is verbatim in `.history/20260825/wip-fold-0905Z.md`.

- **ALL FOUR OF MY COMMITS ARE PUSHED, AND THE FOLD SAID THEY WERE NOT.** `e28c215b` + `db15e857` (the guard fix, both trees, byte-identical by blob id), `5bfd5719` (drift guard route-aware), `134da1f1` + `14777940` (two corrections to my own claims inside it), `851ae74a` (the fold). **Both remotes sit at `851ae74a`; a peer pushed in the normal course while I was folding, and three peer commits now sit on top.** I reported the opposite to hv one turn earlier and had to correct it. **CI run 32830353211 is IN PROGRESS at `9d9d186d`, which DOES carry `5bfd5719`** -- the first run that can exercise the drift fix. Not green; not claiming it.
- **THE DEFECT, AND ITS SHAPE IS THE POINT.** Both guards reached their `printf | grep -q` filter ONLY after a violation had already been detected. **Sound whenever there was nothing to catch; unsound exactly when there was.** vc analysed it and drove the header guard; **I drove the clock guard, which was the half nobody had**, and reproduced the idiom at 200/200 and 100/100 on the two shells a consumer can actually get, BEFORE touching the fix.
- **MY INHERITED-AT-SCALE ARM IS THE CONTROL vc TOOK OVER THEIR OWN.** Theirs was _clean board stays silent_, which a filter rewritten to always-true PASSES. Mine is _breakage the commit did NOT add stays silent_, which such a filter FAILS.
- **THE EXISTING BATTERY ALREADY HAD A CASE FOR THIS PROPERTY AND COULD NOT SEE THE DEFECT** -- its payload is three lines, below the pipe buffer, so printf never takes SIGPIPE and both forms agree. **That is why the class survived every review, mine included.**
- **`canon-ignore-guard.sh:133` FIXED RATHER THAN COMMENTED, NO TEST, AND vc GAVE A BETTER WARRANT THAN MINE.** I argued a test would be a null control. Theirs: the herestring MOOTS the hazard, so there is no future behaviour left to guard.
- **CI DIAGNOSIS: all three failures in run 32826733499 traced to ONE code commit in a twelve-commit push.** `CANON_INTENT_FOOTER_MARK` was a contiguous substring of the footer's PROSE, so cc's rewording made the tool stop recognising its own output. **It failed as a legitimate message** -- _user-authored, left alone_ is a real state -- so nothing in the output separated the defect from the design. cc had it fixed before I finished reporting it.

## TODO

### LIVE, MINE, UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` (AT-11.7) has a green to reach. **STILL DECLINED ON SCOPE** -- WP-11 is RELEASE and hv asked for local usability. **TRAP: `codesign --force` REWRITES THE BINARY IN PLACE**, so nothing may hash until `verify_notarised` passes; and `cmd/macos:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one.
- **`thread_view_skew_check.sh` roster admission** -- held on a staleness refusal that does not exist. **THE HOLD RESTS ON A MEASUREMENT FROM 2026-08-20 AND MUST BE RE-DERIVED BEFORE IT IS ACTED ON IN EITHER DIRECTION.** Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6** -- blocked on the contract conflict routed to vc.
- **NEW, AND IT IS MINE BECAUSE I FOUND IT IN MY OWN FILE: NOTHING VERIFIES THAT A ROSTER ROW DESCRIBES WHAT ITS RUNNER DOES.** `runner_roster_check.sh` verifies row-to-file EXISTENCE in both directions and is structurally blind to the row's CLAIM. cc asked that a mechanism, if one is built, be mine. **No mechanism proposed yet, and naming it is not building it.** **AND vc HAS PUT A GATE IN FRONT OF BUILDING IT, WHICH I AM RECORDING BECAUSE IT WOULD OTHERWISE DIE WITH THIS SESSION: DO NOT BUILD BEFORE hv HAS RULED WHETHER A DESCRIPTION IS IN THE ROSTER CHARTER AT ALL.** vc framing: this is a guard whose POPULATION IS FILES where the CLAIM IS BEHAVIOURS, and **widening a guard contract silently is how a roster becomes the mechanism.** So the next step is a ruling, never a checker.

### ROUTED, MEASURED, NOT MINE TO TAKE

- **THE FROZEN-`$INTENT_HOME` MECHANISM: THE DETECTOR HALF IS CLOSED, THE ROUTING IS NOT.** vc landed the ref fix and the CI arm (`a38e884b`, `18ccfbbc`), measured in CI's own log rather than in simulation. **vc states plainly that their reason for `not discharged` expired but the ROUTING did not, and a guard cleared by a peer saying the ruling happened is not a guard.** It sits with hv. **What the guard should ASSERT under the freeze scope is still the unsettled half.**
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named, two commands produce it and nothing runs them. **Needs an edit to `tests/run_tests.sh`, which matts runs externally -- scope, not doubt.**
- **THE ESCAPED-MUTATOR REVERT IS SETTLED: KEEP BOTH, and both are committed here.** `AGENTS.md` is a GENERATED view and the escape ran `agents sync` early, so reverting restores a STALE one; `config.json`s `project_id` is LIVE in the store, driven with `.dump` plus both controls. **`MODULES.md` WAS NEVER AN INCIDENT FILE** -- 13:30:54Z, thirteen minutes BEFORE the escape.

## Watch-outs

### FAMILY 1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT

**THE TEST: RUN IT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Not _check your instrument_, which is unactionable. The instances below are evidence FOR the rule, not rules beside it.

- **`intent info | head -1` is the PRODUCT BANNER, identical from every cwd BY DESIGN.** I nearly withdrew a CORRECT fix on it. **Pick a line that CAN vary, or diff the whole output and pick none.**
- **`--help` IS NOT A PROBE.** Under INV-07 it exits 0 whether or not a command is built.
- **A GREP'S ZERO IS A CLAIM ABOUT THE CORPUS, AND SO IS A CASE-SENSITIVE MISS.** Positive-control before believing it. Twice: a db grepped for `0001` returned 1609 hits; a restart.md probe returned 0 because I matched lowercase against an uppercased rule.
- **A NO-OP CANNOT FAIL, SO IT MEASURES NOTHING.** My v2 arm read `5 -> 5` because upgrade SHORT-CIRCUITED. **Force the real path before believing a clean result.**
- **`find`-BASED POPULATIONS DESCRIBE THE WORKING TREE, NEVER THE COMMIT** (ic). `run_tests.sh:89`. A missing test and a passing test are the same observation.
- **A COUNT OVER A `dirty-` MARKER IS A FLOOR, NOT A DISTANCE** (ic). The uncommitted delta at build time lies outside the range in either direction.
- **A RANGE WITH NO PIN NAMES A DISTANCE FROM A MOVING POINT; A PIN WITH NO RANGE NAMES BYTES WITH NO CONSEQUENCE** (ic, theirs whole).
- **A GUARD WHOSE PREDICATE DEPENDS ON WHEN IT RUNS RATHER THAN ON WHAT IT READS HAS A CATCHABLE WINDOW THAT CLOSES SILENTLY** (vc).
- **A MEASUREMENT TAKEN BEFORE YOUR OWN WRITE IS STALE BY CONSTRUCTION.** It cost the attribution on the 2026-08-24 incident.
- **TWO READINGS OF ONE SOURCE IS ONE INSTRUMENT COUNTED TWICE** (cc). I could not reproduce cc's table count and declined to CONTRADICT it, because my own probe had returned `if` as a table name. **That refusal is what sent them to `sqlite_master` rather than to a better regex, and a better regex would have agreed with them and taught us nothing.**
- **THE ` M` RULE LIVES AT `intent/restart.md:69`, ONE HOME, in my name.** What is NOT there and belongs to cc: they binned the same zero-byte file into `peers` twice while reporting scope, inferring AUTHORSHIP from a marker making no claim about content -- **action right, reason wrong, so nothing broke and nothing would ever have surfaced it. A CHARACTERISATION WHOSE ACTION IS CORRECT IS INVISIBLE BY CONSTRUCTION.**
- **THE HARNESS RAN THE SUBJECT UNDER DIFFERENT SHELL OPTIONS THAN ITS ONLY PRODUCTION CALLER**, so fifteen green arms sat over a function returning rc=1 on its healthiest answer. bats sets neither `errexit` nor `pipefail`; `bin/intent3` sets both. **AND A COMMAND SUBSTITUTION DISARMS errexit IN THE SUBSHELL** -- `$-` is `ehuBc` at top level and `huBc` inside `$( )` -- so the one caller survived BY ACCIDENT while the library promised it could not kill a host. **Drive the options production sets, not the ones the harness happens to have.**
- **NEVER `$?` AFTER A PIPE. `cargo test` needs `--no-fail-fast`. `grep -c` exits 1 on zero. The Bash tool's shell is ZSH and does not word-split an unquoted expansion.**
- **`bash -n` IS WRONG IN BOTH DIRECTIONS AND NEITHER IS VISIBLE FROM ITS OUTPUT** (half vc's). It **OVERSTATES BY POSITION** -- bash parses incrementally, so an error the run never reaches is never raised; `-n` reads the whole file, a run reads as far as it gets. It **UNDERSTATES BY CONSTRUCT** -- blind to every version defect that is valid syntax, which is where bash 3.2's gaps all live. **Which way it cuts must be DRIVEN: I built a control at the wrong end of a file, no arm fired, and I briefly held a refutation of a true finding.**
- **A VERSION DEFECT CAN RETURN THE CORRECT ANSWER, EXIT 0, AND PUT THE ERROR ONLY ON STDERR.** Under 3.2 `declare -A` degenerates the subscript to index 0 -- which is what the assignment set -- so the value is right, the mechanism is wrong and rc is 0. **NO RESULT-CHECKING FINDS THAT.** Only stderr or `set -e` separates it, and **9 of the 12 rostered guards do not execute `set -e`**, so it is invisible inside three quarters of the commit gate.
- **A NULL POPULATION WEARS THE SHAPE OF A FINISHED MEASUREMENT.** `for g in $GUARDS` under zsh does not word-split; the loop ran once, matched nothing, and printed `0 / 0`, which reads as _no guard executes `set -e`_ -- **bigger and more alarming than the truth. Caught ONLY because the wrong answer pointed the same way as the right one.** A correct loop over a wrong population prints the identical `0/0` and reads as ALL CLEAR. **Legibility was luck.** Its loud twin (vc): 101 failures of 295, caught in one second, same defect, opposite legibility.
- **RUN THE NEGATIVE CONTROL, NOT ONLY THE POSITIVE ONE.** My bash-4 detector was checked against `declare -a` as well as `-A`. A pattern flagging both would report hits across the tree and **read exactly like a finding**, in a sweep whose whole job is telling those apart.
- **A CONTROL IS ONLY A CONTROL IF ITS GROUND TRUTH IS KNOWN RATHER THAN RECALLED** (vc, theirs whole). **A remembered ground truth is a second guess wearing a control's costume**, and nothing tells them apart by looking.
- **DECLARE THE EXPECTED DENOMINATOR BEFORE MEASURING, THEN CHECK THE ACTUAL AGAINST IT** (vc). _12 expected_ refuses a `0` on sight. **It reaches four of six failure kinds and NOT the other two: not precision (comment matches inflating a count), not a wrong axis.** A control that names what it cannot reach.
- **A GREP CANNOT TELL A STATEMENT FROM A SENTENCE ABOUT A STATEMENT** (vc's `0076`). It landed inside my own verification: I probed the fixed file for the apostrophe, got a hit, and nearly re-opened a closed defect -- **the hit was in an ordinary comment, outside both heredocs. THE PARSE IS THE CONTRACT, NOT THE GREP.**

- **EXPECT THE RED, NAME IT IN ADVANCE, AND TREAT AN UNEXPECTED GREEN AS A FINDING ABOUT THE INSTRUMENT** (vc, theirs whole, and it is the RULE where I had only the episode). **It is the only detector that worked on this class all day, four times.**
- **FOUR INSTANCES IN ONE DAY, ALL MINE, EVERY ONE RETURNING A CLEAN PLAUSIBLE RESULT.** (i) A ~19KB e2e fixture that fit inside the pipe buffer, so the race never armed and OLD agreed with NEW on every arm -- **the agreement was the only tell.** (ii) A run against the WORKING TREE reported as a fact about the COMMIT, in a four-writer tree where a peer was mid-fix in that very file. (iii) A mute control that appended to a file that did not exist, so `>>` CREATED it on the v2 side -- **a fixture added to the wrong side of a ONE-DIRECTIONAL comparison is invisible to it.** (iv) A positive control grepping `tests/` for a filename the file does not contain: **badly chosen rather than broken, and only the population grep finding hits elsewhere separated the two.**
- **`2>/dev/null` CONVERTS A BROKEN PROBE INTO A CLEAN ANSWER, AND IT IS THE FIFTH INSTANCE OF FAMILY 1 IN ONE DAY.** My push check printed `commits ahead of upstream: 0` and I read it as _nothing to push_; the redirect was eating the error and the count was of nothing. **The un-swallowed run said `ahead: 1` and the commits were already in.** The four instances above are all _the instrument could not reach its subject_; **this one is the instrument reporting that it could not, into a hole.** Swallow stderr only where you have already decided that a failure and an empty result mean the same thing.
- **A CORRECT FINDING CAN CARRY A WRONG CITATION, AND THE CITATION IS THE HALF THAT GETS REUSED.** vc was right that a fourth suite caller existed and named `cmd/shared/measure.lib`, which contains no such call; the real one is `bin/.devbin/config.yaml:91`. **The finding survives; the pointer would have been copied forward.**
- **AN UNDER-STATED RESIDUAL IN A RATIONALE IS HONOURED RATHER THAN CHECKED** (vc, correcting me). I wrote _every node here has it_ about a guard **nothing runs on a commit**, in the sentence congratulating myself for stating the cost. **HAVING A ROUTE IS NOT RUNNING IT.**
- **A RETIRED PREMISE KEEPS ISSUING ORDERS.** The drift guard's proxy remedy said _push the branch_, correct while the push was owed. hv retired the owing; left alone it would have gone permanently red on every node while instructing each of them to do the thing hv had just declined.

### FAMILY 2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF IS WHAT CARRIES THE FALSE ONE

**A CONFIDENT UNMEASURED NUMBER IN A _RATIONALE_ IS LOAD-BEARING IN A WAY ONE IN A _REPORT_ IS NOT -- A REPORT GETS CHECKED, A RATIONALE GETS HONOURED** (vc's formulation). All three of my shipped defects are instances.

- **`bin/intent3` claimed a coherence check would be "a MULTI-SECOND gate on every command".** Driven: **~85ms end-to-end**, against a component sum measured separately at ~110ms. **The two do not reconcile and I have not resolved them; neither is within two orders of magnitude of the claim.** It was the entire stated reason for not building the thing hv later ruled I should build, and it held the design shut for three days.
- **`cmd/hosting` claimed "in a throwaway clone a mutator harms nothing".** Never driven. Every verb ran from the LIVE tree's cwd and it emptied the durable store.
- **`currency.lib` PRINTED A FLOOR AS A DISTANCE** -- the overclaim in the error message of the file written to refuse overclaims, **and the fix for it added an arm nothing drove** (ic). **A fix that adds an arm adds a thing to drive.**
- **A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED** (ic, against themselves).
- **MY TELL vs cc's, AND THEY ARE MIRRORS:** I publish the claim the measurement INVITES (wrong at the READING); cc drives the measurement and publishes a stronger claim about what it measured (wrong at the WRITING). **Both rest on a real number, which is why neither gets challenged.**
- **A ROW CARRYING ONE TRUE SENTENCE AND ONE FALSE ONE IS HARDER TO CATCH THAN A WHOLLY WRONG ROW, BECAUSE THE TRUE HALF IS WHAT A READER CHECKS FIRST AND IT HOLDS** (cc, on my false roster row).
- **`checked against the schema` VERSUS `checked against a grep of the source` IS THE WHOLE DISTANCE, AND NEITHER OF US COULD SEE IT IN OUR OWN SENTENCE AT THE TIME OF WRITING.**
- **A STABLE CONCLUSION ACROSS THREE WRONG POPULATIONS IS NOT CORROBORATION -- IT IS THE POPULATION NOT BEING LOAD-BEARING FOR THAT CONCLUSION** (cc, on vc's withdrawal).
- **A HAND-MAINTAINED SET THAT NOTHING CHECKS IS THE ROSTER PROBLEM,** and I built one while explaining it. **Manifest plus a drift check, and a path in NEITHER the manifest nor a declared-exclusions list is an ERROR rather than a judgement call** (vc).
- **UNCHECKED IS NOT EMPTY, AND ONLY THE OUTPUT CAN CARRY THAT DISTINCTION.** An instrument that measures a narrow scope and reports in the vocabulary of the general category produces a **true sentence and a false belief**, so nothing it can check is wrong. Four instruments, one shape: `0069`, `0074`, `0075`, `0076`. **A silent gap gets found; a gap that files a clean report on its own behalf does not.**
- **A RULE IS HONOURED BY WHOEVER LEARNED IT AND DOES NOT PROPAGATE BY HAVING BEEN STATED.** `precommit` has 14 guard arms at three strengths; **7 assert a repository finding on ANY non-zero exit.** The arm BURNED by this class guards the tool being MISSING and is **still exposed to the tool being BROKEN** -- driven, not read. **Fixing one more arm in passing would be the same error a fourth time; the class needs one change across seven sites.**
- **A WRONG LINE NUMBER IS ONLY A TRAP IF FOLLOWING IT CAN REACH A GREEN.** vc warned the 3.2 error at `:216` would lead a fixer to gut the exemplar. Built it: the error MARCHES to `:225` and stays there. **Misleading, not trapping -- and the marching is what tells you to stop chasing and look up.**

### FAMILY 3 -- ROUTING, RELAY AND ATTRIBUTION

- **THE OFFER IS THE MOMENT TO CHECK, NOT THE SEND.** Once two messages are in flight the only tools left are racing and deduping, and only one works. Three instances, one mechanism.
- **A FORWARDING OBLIGATION DISCHARGED BUT NOT CANCELLED MANUFACTURES FALSE CORROBORATION** -- it arrives TWICE wearing TWO authorships and **the second announces itself as agreement. Tell the RECIPIENT to expect one copy; the recipient can dedupe, the relay can only be beaten.**
- **VOLUNTEERING A RELAY _IS_ CREATING THE OBLIGATION, AND THE COORDINATOR HABIT IS TO VOLUNTEER** (vc). A rule only the receiver can apply arrives one step too late.
- **NAME WHICH _HALF_ YOU ARE ATTRIBUTING** (vc, wholly theirs). An incident and its generalisation are separable and usually have different authors. **Nobody audits an attribution.**
- **A PEER TELLING YOU WHAT ONLY THE SOURCE CAN SETTLE IS TELLING YOU WHAT _THEY READ_** (ic). Fold instructions, quoted rows, relayed rulings alike.
- **A RULING DELIVERED AS A SELECTION AMONG OPTIONS YOU AUTHORED IS ONE WHOSE WORDING IS YOURS AND WILL BE QUOTED AS THEIRS. RECORD THE OPTIONS, NOT THE OUTCOME**, in an inbox where it survives the session.
- **THE BLOCKED PARTY TELLS THE BLOCKER, NOT THE SEQUENCER.**
- **A PROOF ONLY ITS AUTHOR CAN REPRODUCE IS NOT YET A PROOF THE ESTATE HOLDS** (ic). Survives the soundness being conceded. **Script the repro.**
- **A PEER'S READY-MADE COMMAND IS NOT AN APPROVAL, AND CONVENIENCE IS WHAT MAKES IT SLIP.** Distinct from a relayed approval: the peer claims no authority and is simply being helpful, so there is nothing to challenge and nothing that reads as a demand. **devbin/vc's own words on it: _that is the failure mode of convenience in this setting._** The test is not whether the change is sound -- theirs was -- **it is who gets to say yes.**

### FAMILY 4 -- THE SHARED CHECKOUT

- **FOUR WRITERS, ONE TREE, ONE INDEX.** Peers' dirty files are in every `git status` you read. **Name yours; `--only` is PATH-scoped, never hunk-scoped.** Peers commit concurrently -- **wait a lock out, never clear it.**
- **AN UNCOMMITTED EDIT WAS ERASED HERE WITH NO REFLOG TRACE AND hv RULED IT ACCEPTED RATHER THAN INVESTIGATED.** Live and undiagnosed BY DECISION. **A protective copy OUTSIDE the tree costs nothing; staging is NOT the remedy, because the index is shared.**
- **A FIGURE ABOUT HEAD BELONGS IN THE SINGLE-WRITER CLONE** (`int suite`, `int hosting`). A figure about the WORKING TREE must be defended by attribution instead.
- **`bin/` IS dc's LANE** (hv). `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED and not this repo's to edit.**
- **A MONIKER NAMES WHERE A SESSION LIVES, NEVER WHERE ITS BYTES LAND.** `devbin/vc` works in `~/Devel/prj/Devbin` and its fleet sweep writes into **eleven checkouts including this one**. Five paths here went dirty mid-commit with no announcement, and **nothing on the wire marks the crossing.** They vendor-and-HOLD where a tree is dirty at pre-flight, so held bytes sit uncommitted and unattributed until someone asks. **ASK, DO NOT INFER: the two answers are `a peer is working` and `something writes to this tree unattributed`, and only one is survivable.**
- **`--only` IS WHAT MADE THAT A QUESTION RATHER THAN AN INCIDENT** (devbin/vc's words, and they are right). A bare `git add -A` sweeps a fleet vendor into your commit **silently**, and neither party learns until the log reads strangely.
- **THE FORMATTER IS A SECOND WRITER BETWEEN THE SYNC AND THE COMMIT, AND IT FIRED AGAIN TONIGHT.** It realigned a table I had just synced, so canon named bytes the file no longer held and the gate refused at `ADDS 1`. **Order: write, LET THE FORMATTER SETTLE, sync, commit.** Second time in this file's history, and this time inside the document about instruments misreporting.
- **THE SUBJECT CAN MOVE BETWEEN YOUR TWO READS, AND THE SECOND READ LOOKS LIKE AN ANSWER.** `git diff --numstat` reported changes to an issue's canon; `git diff` seconds later printed nothing. **Not a broken instrument -- a peer committed in between.** vc hit the identical trap twenty minutes earlier (`d07c94fd`). **In a four-writer tree a two-command measurement has no single subject unless you pin the revision.**
- **COMMITTED IS SELF-DESCRIBING; HELD IS NOT** (mine, adopted into devbin's sweep procedure at their `c6c30f9`). A committed change carries an author, a message and a log entry. **Bytes vendored into your tree and deliberately left uncommitted carry NOTHING**, which is exactly the state that reads as an intruder. **A fleet operation must name its HELD estates, not merely pre-announce itself.**
- **A FILTER OVER A COMMAND'S OUTPUT MUST BE ABLE TO EXPRESS ITS FAILURE, NOT ONLY ITS SUCCESS.** I piped `git commit` through `grep -E 'files changed|^\[main|error|refus'` and it printed NOTHING: a peer took the index lock, git said `fatal: Unable to create '.git/index.lock': File exists`, **and that message matches none of those patterns.** **A FAILED COMMIT PRESENTED AS SILENCE**, and only an independent `git log -1` caught it. **Same family as the whole evening: the instrument could report the outcome I expected and not the one I needed.** Read the rc, or do not filter.

## Decisions

- (2026-08-25) **hv RULED: fix both trees LOCALLY, commit, and DO NOT PUSH v2.** _"The checked out v2 branch is only being used locally here by projects on this machine ... as long as [they] can work on a stable v2 branch and not have their project management functions screwed up while we build v3, I'm good."_ **The public drift is knowingly worn.**
- (2026-08-25) **hv CHOSE THE ROUTE-LEVEL DECLARATION over a per-file exception list.** Branch not taken: an entry per fix, which the guard's own text forbids and which would be FALSE of every file it named -- not PENDING (the v2 landing is done) and not V3-ONLY (v2 received it).
- (2026-08-25) **THE ENFORCEMENT LOSS IS A COST OF hv's NO-PUSH RULING, NOT OF THE ENGINEERING, AND NOBODY PRICED IT WHEN THE DECISION WAS MADE** (vc's framing). Routed to hv rather than absorbed in a test file. **Release is an automatic caller with ground truth, but it is the LAST place you want to first learn a fix reached one tree.**
- (2026-08-25) **A PEER'S FULLER QUOTE OF A RULING IS STILL THE PEER.** vc supplied the clause they had dropped -- the one authorising the commit -- and told me not to treat their reading as my authorisation. **I held for hv's word in my own session and vc was right that I should.**

- (2026-08-24) **THE BUILDER CARRIES THE ROW, IN BOTH DIRECTIONS.** vc built four things in my lane and carries all four (vc, applying my own rule to me); cc then offered me their v2-estate arm to PLACE because it landed in my lane, reading my wall warning as territorial when it was PROCEDURAL. **A row does not transfer because of where it lands.**
- (2026-08-24) **A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL.** cc reported matts had ruled 0070 mine to close; I declined the relay and **cc withdrew the framing themselves** -- it answered a question asked in THEIR session.
- (2026-08-24) **hv RULED: `intent3` MAY REFUSE.** Branch not taken: _keep it a reporter, warn but exec_. **vc preserved the menu, which is the remedy for the `sync`-skip provenance gap.**
- (2026-08-24) **hv RULED: the local `v2-maintenance` branch DELETED.** A silent wrong answer became a loud absent one. **Fast-forwarding LOOKS equivalent and re-arms the moment upstream moves.**
- (2026-08-24) **hv's FREEZE SCOPE: frozen for features, live for shipped-surface defects.** `0070` was v3-only; `0071` is shipped-surface and lands in both.
- (2026-08-24) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE.** `currency.lib` is EXEC-time; `cmd/local`'s `verify_pair` is BUILD-time and demands `sc == HEAD` exactly. **The convergence is RECORDED AND DELIBERATELY NOT DONE.**
- (2026-08-22) **A CLAIM OF UNIQUENESS IS A MEASUREMENT AND MUST BE GREPPED, NOT ASSERTED.**
- (2026-08-22) **SOUND-BUT-UNNECESSARY AND UNSOUND ARE DIFFERENT VERDICTS, AND ONLY ONE IS FAIR TO THE PROPOSER.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** `intent3` is an ACTOR, on PATH.
