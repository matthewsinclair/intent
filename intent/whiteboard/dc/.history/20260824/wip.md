---
node: dc
name: DevX Claude
role: worker
session_id: 55d5f57e-bc10-4cbf-9959-789541b069dc
heartbeat_at: 2026-08-24 15:52Z
status: active
focus: "**PICKED UP 2026-08-24 -- FRESH SESSION, NOTHING IN FLIGHT, CLAIMS INTACT (ST0056/07, ST0056/11), TREE CLEAN AT `50417c83`.** **The bounce TOOK this time, and that is measured on the one discriminating field rather than inferred: `session_id` moved `d2fad1a7` -> `55d5f57e`. Do NOT use ListAgents' `started` column -- it is socket age, and four nodes agreed off it and were all wrong.** Inbox: vc's 11:13Z five-estate config-sweep briefing, FYI-only, and **hv's instruction is that I TAKE it from that entry and re-derive none of it.** Four of its items are mine and vc landed all four: `intent upgrade --dry-run` (whose first cut wrote `.backup/` while printing `nothing was modified` -- caught only by fingerprinting the tree, never by reading the output), the hook-script PRUNE, whole-directory skill checksums, and the declared-disposition rule. **THE HAZARD I CARRY INTO EVERY EDIT TODAY: the fleet runs the FROZEN Intentv2 via $INTENT_HOME, so a shipped-surface fix landed in ONE tree reaches nobody and presents as done -- land in BOTH checkouts; `tests/unit/shipped_surface_drift.bats` reddens if I forget, and its first catch was vc.** Holding for vc's instructions before starting anything."
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260821/wip.md` -- the afternoon session is appended under its own heading. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.
- **A CONVERTED stamp is one you COMPUTED; a RE-READ stamp is one you MEASURED** (vc's formulation of my rule, `e0e7d2ff`). I derived `local = UTC+1` from one reading of each clock and APPLIED it -- arithmetic on a measurement. `TZ=UTC stat` re-reads the same fact with the clock forced. Both landed on the same figure; only one of them _could not_ have been wrong. **Never leave the clock you measured on unless the question forces you to; if it does, RE-READ rather than convert.** `ls -lT` emits no marker and so cannot fabricate a `Z`; `stat -t` will wear whichever one you hand it.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **A `scratchpad/...` PATH CITED ANYWHERE DURABLE IS BOTH PERISHABLE AND AMBIGUOUS, AND THE AMBIGUITY IS THE WORSE HALF** (vc raised the reaping; the set comparison, the control and the age test are mine, driven 2026-08-23 13:07Z). **CORRECTING MY OWN PUBLISHED VERSION AT `3db3c815`: I wrote that 21 tmp dirs with no transcript showed the two stores are "independently reaped". IT SHOWED NOTHING OF THE KIND** -- vc caught it, and the control settles it: **0 of those 21 have a populated scratchpad, against 18 of 21 for the transcript-backed dirs.** They are agent run dirs that never had a transcript to lose. **A boundary named on a misread is worse than an unnamed one, because it looks surveyed.**
- **WHAT IS ACTUALLY ESTABLISHED, and it is stronger than the count either of us started from: 18 transcripts under `~/.claude/projects/` have no tmp dir, and the split is PURELY BY AGE WITH ZERO INTERLEAVING.** Every no-dir transcript is 30 Jul -- 19 Aug 01:35 local; every paired one is 19 Aug 09:43 -- today. **A sharp cutoff is the signature of time-based removal; never-created would interleave.** Surviving window ~4 days. **So `~/.claude/projects/` is the durable store and tmp is reaped, which is vc's original claim -- reached, finally, by a measurement that could have refuted it.**
- **AND THE LIVE INSTANCE DOES NOT FAIL THE WAY THE RULE PREDICTS.** `vc/inbox.dc.md:57` -- mine -- cites a clone at a bare `scratchpad/ac015`. It is not dead: **four `ac015*` dirs exist, two named exactly `ac015`, under different session UUIDs** (vc's count, and my own session holds one of them). **A bare scratchpad path was never a pointer that goes dead -- it is a pointer plus an unstated assumption about which session is reading.** A dead path errors loudly and someone notices; an ambiguous one resolves silently to whichever it finds first. **Treat that line as uncited; guessing would produce a citation that looks repaired.** Cite a committed path or nothing.
- **v2 LIVES IN `~/Devel/prj/Intentv2`** (`v2-maintenance` at `fb45e9ea`). **Its gate was ARM C LIVE and I repaired it** -- dispatcher was absent, `gate ABSENT` -> `WIRED`, planted violation now `rc=1 / guards: 4 ran, 0 skipped`.
- **`intent` ON PATH IS v2.19.0 AND RESOLVES THROUGH `$INTENT_HOME` TO THE FROZEN `Intentv2`. v3 IS ALSO ON PATH NOW, AS `intent3` -- "DO NOT PUT v3 ON PATH" WAS RETIRED 2026-08-22 BY ST0058, AND BOTH RESTART FILES WENT ON ASSERTING IT FOR TWO MORE DAYS** (vc, their 2026-08-24 12:07Z stamp, attributed not asserted). The DISTINCT NAME is what leaves the fleet's gate untouched, and it is untouched by construction rather than by care. **`intent3` -> `bin/intent3` -> `target/release/intent`, WHICH THE GATE REPORTS AS BUILT FROM AN UNCOMMITTED TREE -- so PIN BY HASH, NEVER BY THE MARKER.** The per-crate divergence underneath that is MINE and OPEN: `INTENT_SOURCE_COMMIT` comes from each crate's own `build.rs`, `intentd` declares no dependencies so nothing invalidates its fingerprint, and the two binaries agree today ONLY because `1940fa93` happened to touch both packages. **They diverge again on the next single-package change, silently, at `Finished`.**
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED** -- `intent ac status ST0057`, `intent ac status ST0056/03`, `intent ac gate ST0057`. vc found it living in THREE homes at THREE values on 2026-08-24, with one document disagreeing with ITSELF twice inside its own text. **Highlander applies to a figure in prose exactly as it applies to code**, and I read the stale one at pickup and repeated it to hv within the hour. **Do not put the number on this board; put the calls.**
- **`.envrc` EXISTS AND IS BLOCKED -- it does NOTHING until hv runs `direnv allow`.** Even then it fires only at an interactive prompt: **every node commits through non-interactive tool calls, so it is inert for us.**
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** It now warns when they differ (`1005ab88`). To ask about another clone, run ITS `bin/int`.

## DOING

### FILED: `0070` / `0071` / `0072`, WITH REPROS (2026-08-24 15:52Z)

```
0070  HIGH    intent upgrade DESTROYS every issue in an already-migrated v3 project   v3-ONLY
0071  HIGH    v2 intent upgrade BLOCKS on an interactive read with no TTY             BOTH trees
0072  medium  intent/.backup/db is empty -- no pre-incident store snapshot exists
```

**THE INCIDENT WAS MINE AND THE BUG IS THE VERB'S.** My escaped `upgrade` is how it reached this repo; **it would have arrived through the front door eventually.** Two scripted repros, self-contained under `mktemp`, so **the finding is not author-locked** -- ic's rule, applied before anyone asked twice.

**THE v2 ARM WAS A FALSE CLEAN AND I CAUGHT IT RATHER THAN SHIPPING IT.** v2 printed _already at 2.19.0_ and short-circuited, so `5 -> 5` measured only that **a no-op destroys nothing.** Backdated the declared version to `2.10.0` to force a real migration; **then** it was clean, and only then did the v3-only bound mean anything.

**THE DIAGNOSIS IS THE ASYMMETRY: v2's upgrade SHORT-CIRCUITS at target, v3's RE-RUNS THE MIGRATION** on an already-migrated project. Threads survive that re-run and issues do not -- **which is the constraint vc set and their own hypothesis could not meet.**

**cc's DEPENDENCY QUESTION IS ANSWERED IN 0070's BODY RATHER THAN BY MERGING THE ROWS: A REGRESSION TEST FOR THE DESTROYER THAT ASSERTS VIA `sync`'s AGREEMENT REPORT INHERITS `0069`.** Assert on counts read from the store DIRECTLY. **My repro does, which is the only reason it can see the loss at all.**

**AND FILING THEM PRODUCED A THIRD INSTANCE OF `0069`, WHICH SHARPENS vc's ROW INTO A SIMPLER CLAIM.** An unscoped `--to-store` that changed THREE ISSUES confirmed with `store replaced from the extract, 58 thread(s)`. **The warning names issues, the ok line counts threads, and 58 is not a count of anything that changed -- it is how many threads exist.** So the defect is not three wording slips: **the confirmation line carries ONE HARDCODED NOUN regardless of what the operation touched.** That predicts the other two instead of sitting beside them.

**Store verified after every step: 51 issues / 51 canon / 58 threads.**

### `tests/unit/artefact_currency_verdict.bats` -- 13 ARMS, MUTATION-PROVEN RED (2026-08-24 14:12Z). ic FOUND THE GAP.

**BOTH REFUSAL MESSAGES WERE DRIVEN ONLY BY AN AD-HOC SCRATCHPAD RIG THAT LIVES NOWHERE.** ic's grep, verified here and positive-controlled: 0 test references to `currency.lib`, `artefact_currency_verdict` or either refusal string, against 41 files for `create_test_project`.

**AND ic's SHARP HALF: THE CLEAN-BASE ARM WAS DOUBLY UNREACHABLE.** No test drove it, **and the live release pair is `dirty-`, so every real invocation takes the FLOOR branch.** The clean-base wording could not be observed by USE or by TEST -- **correct by inspection only, and the day someone builds from a clean tree the refusal changes to a form nobody has ever read in anger.**

**ic's GENERALISATION, THEIRS, AND IT INDICTS THE FIX RATHER THAN THE ORIGINAL DEFECT: A FIX THAT ADDS AN ARM ADDS A THING TO DRIVE, AND THE ARM NOBODY DRIVES IS THE ONE THAT IS WRONG WHEN IT FINALLY FIRES.** The overclaim sat in the error message of the file written to refuse overclaims; **the unexercised branch then sat in the fix for it.** Same shape, one level up, within the hour.

**MUTATION-PROVEN, BECAUSE 13 GREEN PROVES NOTHING ALONE.** Collapsing the two wordings back to the pre-fix single message reddens **EXACTLY arms 4 and 5**, and **arm 3 correctly stays green** -- the mutation makes everything take the clean wording, so the clean arm should not notice. **An arm that reddens under every mutation is not discriminating.** Restored and **hash-verified against a copy taken before the mutation rather than assuming `cp` worked.**

**THE NOT-ANCESTOR ARM BUILDS ITS PRECONDITION RATHER THAN ASSUMING IT** -- `commit-tree` with no `-p`, a parentless commit touching neither working tree nor branch. **The comment names why: a stale ref at a BRANCH POINT is an ancestor, falls through to the source-changed rule, and refuses for the wrong reason -- green by verdict class, testing nothing.** That is the `v2-maintenance` trap, **now in a file instead of in two nodes' memories.**

**THE HARNESS FAILED TWICE BEFORE PASSING, LOUDLY BOTH TIMES** (`checkout -` after an orphan checkout; then the checkout back refused over untracked files). **The version to fear is the one that passes while quietly leaving a mutated repo for the next arm.**

**ic's UNTRACKED-FILE WARNING, AND THE OBVIOUS RESPONSE WAS WRONG.** `currency.lib` is untracked, so **"the refusal everyone is seeing" is true because we share ONE WORKING TREE, not because anything shipped** -- no clone, no fresh checkout, no other estate has it. It also sits in the path of the unexplained worktree reversion that erased an uncommitted edit of ic's, **which hv ruled ACCEPTED rather than investigated, so the mechanism is live and undiagnosed BY DECISION.** **`git add` is the obvious move and it is wrong here: the INDEX IS SHARED, so a peer's bare `git commit` sweeps my file into their commit** -- my own board's rule arriving from the other side. **Protective copy OUTSIDE the tree instead: no index change, no shared state touched.** It does not defeat the reversion; it means I can restore from a hash I hold.

### INCIDENT -- `int hosting --include-mutating` WROTE THE LIVE STORE. MINE. DIAGNOSED, FIXED, CONTAINMENT PROVEN (2026-08-24 13:53Z)

**THE TOOL'S HEADER CARRIED THE SAFETY ARGUMENT IN ITS OWN WORDS AND I NEVER DROVE IT:** _"in a throwaway clone a mutator harms nothing, so `--include-mutating` drives them too."_ **The clone was real, the build was real, the revision was named by construction -- and the verbs were invoked from the LIVE tree's cwd.** Cloning changes WHICH BINARY RUNS; it does not change WHICH TREE THAT BINARY STANDS IN.

**WRITTEN TO THIS CHECKOUT AT EXACTLY `13:44:15Z`, THE SWEEP'S OWN MTIME.** **CORRECTING MY OWN FIRST VERSION OF THIS LINE, WHICH SAID `14:44:15` -- THAT IS LOCAL/BST, AND vc CAUGHT IT** (driven: `TZ=UTC stat` gives `13:44:15Z`). I was not wrong -- an mtime IS local by default and I said so -- **but the hazard is entirely downstream: every other stamp on this board carries a `Z`, so a reader transcribes the bare number, appends a `Z`, and lands an hour out.** **This board already records the precise version and it sharpens vc's: check A only catches it WHILE THE STAMP IS STILL IN THE FUTURE. Right now `14:44Z` postdates `date -u`, so A would fire; within the hour it will not, and then nothing sees it.** **A stamp whose detectability decays with time is worse than one that is always wrong, because the window in which it is catchable closes silently.** Convert at the SOURCE, in the write-up, never leave it to the reader: `AGENTS.md` (+12, `agents sync`), `intent/.config/config.json` (`+project_id`), **`intent/.cache/intent.db` -- THE DURABLE SSOT** -- and `intent/events.jsonl`. **Canon untouched, 0 of 105 dirty, positive-controlled** (cc's bound, verified here not taken). **NOT MINE and in the same minute: the devbin vendor-down at 14:45:09 and two peers' boards at 14:44:43/14:45:11 -- all AFTER mine.**

**THE TELL WAS IN THE TOOL'S OWN OUTPUT AND I READ PAST IT:** `init` answered _already an Intent project: /Users/mat..._ while the clone sat under `/private/var`. **The verb told me where it was standing and I was reading the verdict column.**

**cc's PARALLEL IS THE ONE THAT HURTS AND IT IS RIGHT: THIS IS THE `multi-second` DEFECT ONE FILE OVER, AN HOUR APART.** Both are a confident unmeasured claim **inside a SAFETY ARGUMENT**, where vc's rule bites hardest -- a report gets checked, a rationale gets HONOURED. **Both mine, both found by me, and the second only because the first taught me to look.**

**AND IT IS MY OWN `int hooks` FINDING, WHICH WAS ON THIS BOARD BEFORE I WROTE THE TOOL:** a devbin command resolves its project from something other than your cwd -- **so I had the mechanism written down and built the tool anyway.**

#### THE FIX WAS ONE LINE AND I NEARLY WITHDREW IT ON A BAD INSTRUMENT

`cd "$TMP/clone"` in the drive-loop subshell (`$BIN` is absolute; the cd dies with the substitution). **Then I tested the premise and got `head -1` of `intent info`, which is the PRODUCT BANNER -- identical from every cwd by design.** I concluded _cwd does not determine resolution, the fix is wrong_. **It was the instrument.** Diffing the WHOLE output shows `/tmp` -> _Not in an Intent project directory_ against the checkout's full block.

**THAT IS THE `--help` TRAP, AND `cmd/hosting`'s OWN HEADER WARNS ABOUT IT BY NAME** -- _"`--help` IS NOT A PROBE AND THIS IS THE TRAP THAT CATCHES EVERYONE INCLUDING THE PERSON WHO WROTE IT UP AN HOUR EARLIER."_ **I hit it inside the investigation of an incident caused by the same class, in the file that warns about it. Pick a line that CAN vary, or diff the whole output and pick none.**

#### CONTAINMENT PROVEN BY FINGERPRINT, NEVER BY READING THE OUTPUT (vc's `--dry-run` lesson, applied)

```
four incident paths, sha256 AND mtime, before vs after a FIXED --include-mutating run:  IDENTICAL
whole-tree `git status --porcelain` delta:                                              EMPTY
```

**AND THE POSITIVE CONTROL IS WHAT MAKES THAT A RESULT RATHER THAN A VACUOUS PASS: `no writes` and `nothing ran` are indistinguishable without it.** `agents sync` and `upgrade` both ran HOSTED with real output, and **`init` now answers `/private/v...` -- the CLONE.** The same line that was the tell for the escape is the evidence of the fix, on one instrument, both directions.

### WP-07 HOSTING SWEEP -- VALID, AT A NAMED REVISION (2026-08-24 13:53Z)

```
8 hosted, 10 unbuilt, 0 retired, 7 other, of 25 driven
DESCRIBES=635e2b0f   ARTEFACT=01d0b96d7e3135a4 (hash is the identity; the marker is NOT)
```

**IDENTICAL COUNTS ACROSS THE ESCAPED AND CONTAINED RUNS, WHICH IS EVIDENCE RATHER THAN LUCK: DISPATCH IS A PROPERTY OF THE BINARY, NOT OF THE TREE IT STANDS IN.** So the classification survived the contamination even though the OUTPUTS described the wrong tree. **`claude ws` and `claude subagents` remain UNBUILT -- five nodes are still running this protocol by hand because its provisioner does not exist in v3.**

### THE `intent3` CURRENCY REFUSAL -- BUILT, TEN ARMS DRIVEN, LIVE IN THE TREE, UNCOMMITTED (2026-08-24 13:36Z, at `a02a3dbb`, dirty=12 of which 5 are peers')

**hv RULED IT (2026-08-24, relayed by vc, who PRESERVED THE MENU this time): refuse, not report. The branch not taken was _keep it a reporter, warn but exec_.** My pen. **Nothing lands without matts asking.**

```
bin/.devbin/cmd/shared/currency.lib   NEW, 0644 -- SOURCED, never run
bin/intent3                           wired; the false cost comment replaced
intent/llm/MODULES.md                 row added BEFORE the file existed
```

**THE ROW WENT IN FIRST, AND THAT IS THE FIRST TIME IN THIS LANE.** My own Decisions line records four modules -- `suite`, `local`, `clone.lib`, `artefact.lib` -- built with no row **by the node who quotes register-before-you-code at peers**, and vc's reading was that this is a rule with no enforcement point rather than a diligence problem. **It got one here by being done first rather than by being remembered harder.**

**THE MATRIX KEYS ON DECIDABILITY, NEVER ON DIRT, AND IT READS BACKWARDS UNTIL YOU SEE THAT (cc named the misleading case).** `clean + touches source` REFUSES while `dirty + source-clean` only WARNS. A clean marker is the good outcome everywhere else, so refusing on it looks inverted -- **until you notice the question is not _is this binary tidy_ but _can I DECIDE whether it is behind_.** A `dirty-` marker can never refuse (it would reject a binary that is provably current -- cc drove exactly that case) and can never be discharged (the uncommitted delta at build time is unrecoverable from the artefact by anyone, ever), **so it only ever downgrades a refusal to a PERMANENT uncleartable warning. An unsatisfying object, and the only honest one.**

**IT IS NOT `verify_pair` AND MUST NOT BE CONVERGED ONTO IT WITHOUT A RULING.** `cmd/local:179` is BUILD-time and demands `sc == HEAD` exactly; at exec time that refuses after **any** commit including a README edit, which is how a gate becomes one people work around. **Two mechanisms enforcing different properties are not two copies of one** (vc, 2026-08-20). The convergence is recorded in the lib header and **deliberately not done** -- widening a pen because the code is adjacent is the creep this lane is the check against.

**MY OWN COST ARGUMENT WAS FALSE AND IT WAS THE WHOLE REASON THE DESIGN STAYED SHUT.** `bin/intent3:60-66` read _"a MULTI-SECOND gate on every command, which is how a gate becomes one people work around"_. **Driven end-to-end: ~85ms** (components 40 + 36 + 33ms; the floor for a `stat` plus a `rev-parse` is already 31ms). **Wrong by two orders of magnitude, asserted, never measured, load-bearing for three days.** The comment now carries the driven figure AND what it used to say -- **deleting it would hide the class.**

**THE BOUNDARY IS `native/rust/`, NOT `native/rust/crates/`, AND IT JUSTIFIED ITSELF ON REAL BYTES RATHER THAN ON MY ARGUMENT.** I announced 7 files to the peers off the narrow boundary; the guard says 8. **The 8th is `native/rust/build-support/source_commit.rs` -- THE MARKER EMITTER ITSELF.** The narrow boundary would have missed a change to the very code that writes the marker it reads.

**ONE ARM PASSED FOR THE WRONG REASON AND ONLY READING THE MESSAGE CAUGHT IT.** My not-ancestor case used `git rev-parse v2-maintenance`, which **in this checkout resolves to `fb45e9ea` -- the stale local branch at the BRANCH POINT, which IS an ancestor of HEAD.** It fell through to the source-changed rule and refused. **Scoring by verdict class says green; scoring by reason says the arm never ran.** Re-driven against `upstream/v2-maintenance` (`e5a8f158`), precondition checked BEFORE the arm.

**TEN ARMS, AND THE ONE THAT MATTERS MOST IS ARM 2's NEGATIVE:** the refusal path was proven not to exec the binary, by grepping for the binary's own output rather than by trusting rc=1. Positive control silent-and-execs; `dirty + source-clean` warns AND execs; `intentd3`'s symlink resolution intact. **`native/rust/target/` is gitignored (`.gitignore:146`), so a rebuild cannot make the guard call itself stale** -- checked, because the rig surfaced it by accident.

**LIVE CONSEQUENCE, ANNOUNCED TO ALL THREE PEERS BEFORE AND AFTER: `./bin/intent3` REFUSES IN THIS SHARED CHECKOUT RIGHT NOW.** The release pair is genuinely 8 files behind. **It is refusing correctly**, the remedy is `int local build`, and this is the AC-01.5 wedge shape with a one-command exit.

### TODAY UNDER vc's PEN -- LANDED (dc, 2026-08-22)

- **`457ec620` guard-home closer.** Gates the TRACKED TEMPLATE, never the installed copy: `pre-commit.intent` is gitignored by design, so a check keyed to it fails in every clone -- the ARM C shape AC-01.5 spent two days on. Mutation-proven four ways. **The absent-instrument branch exists because cc hit it on real bytes**: my summary asserted one cause for every non-zero exit, so exit 127 printed _the shipped hook template lost the override_ at a node whose template was fine.
- **`7293b24b` `shared/measure.lib` + `int hosting`.** `measured_clone` refuses a revision it cannot name (driven with a stubbed off-HEAD clone, rc=1); `measured_build` builds IN THE SAME ACT and echoes the sha256; `measurement_banner` is one format so two figures can be compared. `int suite` refactored onto it, **proven behaviour-identical by diff with shas normalised.**
- **AC-01.5 green in a throwaway clone** -- fail-closed, the printed remedy works, the guard refuses BY NAME with `guards: 4 ran`, benign rule still commits rc=0. **My first negative control was contaminated** (`git checkout <path>` restores from the INDEX, so the plant was still staged) and briefly indicted a correct guard.

**WHAT I WOULD TELL THE NEXT SESSION, AND IT IS NOT IN ANY OF THE ABOVE: EVERY ONE OF TODAY'S REAL FINDINGS CAME FROM A CHECK THAT DISAGREED WITH SOMETHING I ALREADY BELIEVED.** The stale binary disagreed with _I just built it_. `--only` disagreed with _I verified every commit_. The contaminated control disagreed with _I reset it_. **The three times I reasoned instead of driving, I was wrong, and each wrong answer was PLAUSIBLE, UNIFORM AND CONFIDENT** -- rc=0 across six verbs including one that plainly works. **A wrong measurement does not look uncertain.**

**COMMITTED AT `c7012833` WHEN matts SAID SO, AND NOT BEFORE.** vc issued a commit-to-main ruling under hv's pen at their 19:05Z stamp and **WITHDREW it**; ic's ground was right and is the durable one -- _commit to main only when matts asks_ is a standing instruction from matts, and a pen for DIRECTING work cannot reach the one act this estate reserves to him. Driven when the withdrawal arrived: `git log 66df6f47..HEAD` was five commits, none mine.

**A NOTE ON STAMPS, BECAUSE THE HAZARD REACHED ME AND STOPPED AT THE COMMIT BOUNDARY.** vc reported that every vc stamp in the live channel from 19:01Z onward was produced by arithmetic rather than read -- cc's generator from yesterday, _read the clock once then advance by feel_. **I quoted two of them in the live channel before the correction arrived.** They are attributed to vc above and asserted nowhere on this board. **The only reason none entered the committed record through me is that I had no commit to put them in** -- which is exactly cc's transcription hazard, and luck is not the control.

**U1 AND U4, ROUTED BY vc UNDER hv's PEN.** hv went AFK and handed vc the pen; the aim replaced the gate work: v3 LOCALLY USABLE across matts' estate, not publicly releasable. U1 INSTALLABLE and U4 REVERSIBLE are mine; U2 and U3 are ic's.

### U4 -- ANSWERED, AND TWO OF ITS THREE REQUIREMENTS WERE ALREADY BUILT

**DRIVEN IN A SCRATCH v2 PROJECT, NEVER ON THIS REPO** -- project-wide realisation over four owners' work is the blast radius U4 exists to prevent.

- **v3 REFUSES AN UNMIGRATED v2 PROJECT, READ AND WRITE ALIKE**, rc=1, naming the reason and the remedy, and `git status` was 0 after `info`, `st list` and `doctor`. vc's third requirement -- a project that has not opted in cannot be reached by accident -- **was already true, structurally, fail-closed.**
- **THE SWITCH IS `intent upgrade` AND IT IS PURELY ADDITIVE ON DISK.** The delta is all `>` lines; v2's `intent/st/NOT-STARTED/ST0001/` and all five files survive beside v3's new two-file `intent/st/ST0001/`. Both layouts coexist.
- **THE WAY BACK, DRIVEN TWICE:** `git checkout . && git clean -fd && rm -rf intent/.cache` -> v2 rc=0 with the thread listed, v3 refuses again rc=1, **file set byte-for-byte identical to baseline.** Switch -> back -> switch with a STALE store left in place: rc=0, ONE thread, no duplication.
- **THE ONE PRECONDITION IS THE WHOLE RISK: THE WAY BACK IS `git`, SO A PROJECT WITH UNCOMMITTED WORK HAS NO WAY BACK.**
- **I NEARLY REPORTED DATA LOSS AND IT WAS MY OWN RECORDED TRAP.** After the reversal `intent st list` showed an EMPTY table. `st list` defaults to in-progress; the thread is Not Started. `--status all` showed it. **That trap is three lines long on this very board and it still caught me at the exact moment I was primed to see a thread vanish.**

**THE SUITE IS GREEN AND ATTRIBUTABLE, AND IT IS vc's FIGURE, ATTRIBUTED RATHER THAN ASSERTED (vc; their stamp on that message was `20:20Z` and vc has since WITHDRAWN it as fabricated -- do not transcribe it, and it is recorded here as no time at all):** `BATS_RC=0`, `HEAD=ee4a7cac`, `plan 1..1444`, `ok=1444`, `notok=0`, `ATTRIBUTABLE=yes -- tree identical across the whole run`, my 4 arms present. **The first attributable suite figure of the session.**

**AND THE CAVEAT IS MINE AND MUST TRAVEL WITH IT: THE `yes` WAS BOUGHT BY AN ADVISORY.** I froze my writes on request, vc held, **and cc and ic were quiet because they are STOPPED rather than because they agreed.** Four writers, three silenced by circumstance, one by request. **The check is what makes the figure honest; the quiet is what made it achievable.** Different jobs, and a future reader must not take `ATTRIBUTABLE=yes` as evidence that a busy four-node tree can produce one.

**THE STANDING FIX IS THE CLONE, AND I DROVE IT RATHER THAN PROPOSING IT.** `prepush:261` already clones HEAD into `mktemp -d` and `:315` states the property in its own words -- _"the one workspace in the estate with a single writer ... nobody's uncommitted work is in it."_ Measured: clone HEAD == tree HEAD, **clone dirty 0 against tree dirty 15**, my uncommitted pin correctly ABSENT, `run_tests.sh` rc=0 in the clone, 33M, removed.

**AND vc's FIXTURE-VERSION CAUTION RESOLVED THE OPPOSITE WAY: THE CLONE MAKES THE TRAP IMPOSSIBLE RATHER THAN REPRODUCING IT.** After `ecea0eeb` the helper defaults from `${INTENT_PROJECT_ROOT}/VERSION`, so **in a clone the fixture version and the binary come from ONE REVISION BY CONSTRUCTION.** The trap needed those two to be able to disagree. **A property that was being REMEMBERED becomes one that cannot be violated** -- the advisory-to-control upgrade, arriving where neither of us was looking.

**THE GAP I WENT HUNTING CLOSES ON A STEP THAT ALREADY EXISTS.** A clone has no `native/rust/target/` (gitignored), which should strand the v3-driving tests -- **exactly ONE file under `tests/` touches the v3 binary and it is a harness, not a `.bats`**, and `prepush` already cold-builds in that same clone, so the ordering is right without anyone designing it.

**BLOCKED ON THE COMMIT, AND THAT IS THE ARGUMENT RATHER THAN A PREFERENCE:**

```
"is HEAD green?"             -> clone. Attributable BY CONSTRUCTION. No advisory can be needed.
"is the WORKING TREE green?" -> must measure live. Attribution is then the only honest answer.
```

**vc is forced onto the second question and the thing forcing them is that nothing is committed.** While the 15 paths are uncommitted, **no suite figure in this estate can name a revision by construction** -- every one must be defended by attribution instead. Tonight's green is the demonstration, not the anecdote.

### THE DAY'S CLASS HAS A FOURTH FACE AND IT COMES FROM THE OPPOSITE END

**NOT AN INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT, BUT A SOLUTION THAT CANNOT BE FOUND BY THE PROBLEM.** Seven instruments today that could not discriminate; **three mechanisms that could not be found.**

**A GENERAL MECHANISM INSIDE A SPECIFIC COMMAND IS A PRIVATE METHOD WITH A PUBLIC CLAIM.** Three instances tonight, all correct, all reachable only from where they happen to live:

- **the single-writer clone**, general answer to _which tree does this figure describe_, argued at length -- **consumed only by `cargo build`, because it lives in `prepush` and nobody arrives there thinking about suites.**
- **`int hooks`**, resolving its target repo from the binary's location, so it silently answers about Intent from inside any other checkout.
- **`lib/helpers`**, whose own comment claimed to be THE ONE EXTRACTION SITE while sitting inside one command -- true of `macos`, false of the devbin, **and the claim is what made me put an Intent parser in a vendored file.**

**Discoverability bounded by the name of the file it sits in, which nobody chose and nobody checks.**

**THE RULE THAT COMES OUT OF TODAY, AND IT IS vc's FORMULATION: RUN IT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Not _check your instrument_ -- that is unactionable. **Four instruments today could not vary with their subject** (`ListAgents` started, git's `%an`, my `claude skills list`, my `PROJECT_ROOT` discriminator) and **two were caught only by driving them from two places rather than one.**

**APPLIED TO MY OWN U1 IMMEDIATELY, AND IT FOUND A GAP I HAD LEFT.** U1's claim is _a release binary reachable from any cwd_ and **I had driven the wrapper as `./bin/intent3`, from the checkout root, every time.** The sweep I then ran with `--version` was ALSO not a test -- that answer is identical everywhere by design. **The place the answer must differ is project resolution:**

```
cwd=Intent      project=Intent                    cwd=Intentv2   project=Intent (v2 maintenance)
cwd=scratch     project=U4 Probe                  cwd=/tmp       project=<none>

write verb:  scratch (declares 3.0.0-dev)  rc=0, lists threads
             Intentv2 (declares 2.19.0)    rc=1, REFUSED
```

**THE LAST ROW IS THE COMPOSITION NOBODY HAD TESTED: the version gate still discriminates THROUGH the wrapper.** U2's gate was driven with the raw binary; U1's wrapper was driven with `--version`. **Neither drive touched the join**, which is where a wrapper would most plausibly defeat a gate.

**TWO CORRECTIONS TO MY OWN WORK THIS HOUR, AND BOTH ARE THE CLASS ARRIVING INSIDE MY HANDLING OF THE CLASS.**

**1. THE PIN'S COUPLING WAS BY CONVENTION WHILE ITS COMMENT CLAIMED CONSTRUCTION (vc caught it).** `hook_sed_line` checked the gate's line EXISTS; arms 3 and 4 ran a **hand-typed duplicate** of the `sed`, connected to it by nothing. **Arm 1 catches removal and never caught MODIFICATION** -- change the expression in `pre-commit.sh` and arm 1 stays green while the arms below test the old one. Green, about a line nobody runs. **The exact defect the file exists to catch, inside the file.** Fixed: arms 3/4 now `eval` the line grepped from the shipped file with `wb_info_out` bound. **Proven by a second mutation -- doctor the gate's own sed, line still present: arm 1 green, arm 3 RED. The old version would have stayed green through it.** vc's diagnosis is the keeper and "be more careful" does not reach it: **the copy WORKS, and a working copy gives you nothing to notice.**

**2. I SHIPPED A CHECK THAT COULD NEVER FIRE, IN CODE WRITTEN TO PREVENT THAT CLASS.** `int local status` gained a tool-tree discriminator comparing `PROJECT_ROOT` to `INTENT_HOME`. **`PROJECT_ROOT` in a devbin command is derived from the COMMAND's location, not cwd** -- the `int hooks` trap already on this board -- so it compared a constant to a constant. **Driving it from both checkouts is the only reason I caught it: it printed `consumer` while standing in the tool tree.** Replaced with a machine fact true from anywhere: INTENT_HOME names the tool tree, never migrate it, **every census keyed on config presence finds `Intentv2` as a project and it is not one -- it is the CLI fifteen projects run** (vc's census, which could not tell them apart).

**AND THE MIGRATION FLOOR IS 2.19.0 (vc, driven on copies): 11 of 16 projects are BELOW it and need a v2 `intent upgrade` first.** The canary order INVERTS from the instinct -- the small dormant projects are all below the floor, so the ones that can migrate today are the large active ones. Baize first: smallest that CAN migrate directly, clean, exercises the session-hook path, no live session.

**THE ACCIDENT IS NOW PINNED: `tests/unit/switched_project_gate_resolution.bats`, 4 arms, rc=0 through `run_tests.sh`, MUTATION-PROVEN RED.** vc raised it without ruling it; the build is mine.

```
                                              unmutated   mutated (info exits 0, omits the line)
2  v2 info exits 0 in a 3.0.0-dev project        ok          ok      <- exit code sees NOTHING
3  the gate can resolve INTENT_HOME              ok        NOT OK
4  control: extraction yields nothing            ok          ok      <- binary-independent
```

**ARM 2 STAYING GREEN UNDER THE MUTATION IS THE WHOLE ARGUMENT FOR ARM 3.** A gate checking only the exit code would report success over a chain that had stopped resolving. **The stub exits 0 and looks fine, which is how the real narrowing would arrive.**

**IT ASSERTS AGAINST THE HOOK'S OWN `sed`, LIFTED FROM THE SHIPPED FILE.** A restatement would agree with `pre-commit.sh:97` until one of them moved, then pass about a line the gate no longer runs -- the defect it exists to catch, one level up. Arm 1 asserts that line is still present, so a refactor fails loudly instead of leaving three arms testing nothing.

**FIXTURE PINNED TO `3.0.0-dev`, NOT TAKEN FROM `INTENT_FIXTURE_VERSION`** -- a fixture tracking `VERSION` stops testing the subject at the cutover and goes green forever about nothing.

**AND `bash -n` REPORTED A SYNTAX ERROR THAT WAS THE CHECKER, NOT THE FILE.** `@test "..." {` is not valid bash until bats preprocesses it; an existing `templates_bash32.bats` fails identically. **I confirmed against a control rather than assuming my file was fine** -- a wrong checker emitting a real-looking error is the same family as a right checker emitting nothing.

**A SWITCHED PROJECT KEEPS ITS COMMIT GATE, AND I EXPECTED THE OPPOSITE.** The COMMIT chain is not the SESSION chain vc verified: `.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`. **My hypothesis was that it breaks** -- a v2 binary refuses a v3 tree at exit 2, `pre-commit.sh:97` parses `INTENT_HOME:` out of that output with `sed`, and `:115` documents the exact prior failure where the sed yielded nothing and **the gate failed OPEN.** Driven in the switched scratch project:

```
v2 `intent info` in a 3.0.0-dev project     rc=0
the sed at pre-commit.sh:97 resolves        [/Users/matts/Devel/prj/Intentv2]
pre-commit-guards.sh in that project        rc=0 -- 2 ran, 2 skipped (not applicable)
```

**THE REASON IS WORTH MORE THAN THE RESULT: `info` is READ-ONLY, so v2's version refusal never fires on it** -- the refusal is scoped to verbs that WRITE. **So the gate's resolution step is load-bearing on a verb that happens to be exempt from the guard that would otherwise break it.** Nobody designed that. **A future narrowing of what `info` answers would take the commit gate out with it, silently, in every switched project.**

### AT-11.7 IS RED WITH ITS REASON, AND DRIVING IT BEFORE MOVING IT CHANGED THE ANSWER (`68296b8e`)

The gate reported `AT-11.7 cites .../provenance_fields_check.sh -- the file EXISTS while the row says to-write` **on every commit anyone made.** A standing tax, and **the obvious clearance -- promote it -- was the wrong one.** Driven first: **rc=1, both arms firing.**

**RED RATHER THAN `to-write`, AND THE CHECK'S OWN OUTPUT CARRIES THE GROUND: `to-write` IS EXEMPT FROM L2 AND L3.** A failing test parked there is invisible to `at lint`, so the row would have **stopped nagging while covering nothing** -- worse than the tax it removes. I read that line rather than only obeying vc's instruction, and they say the same thing independently.

**NEITHER ARM IS A DEFECT IN THE INSTRUMENT, WHICH IS WHAT MAKES THE RED USEFUL RATHER THAN MERELY HONEST.** FIELDS: no `artefact_sha256`, an UNLABELLED source commit, no `drift`. SET: both binaries name `ee4a7cac` against a record naming `26fe1aea`. **All four are properties of the WRITER -- `int macos stage`, which emits ONE `commit:` for a SET and no hash at all.** The check has a green to reach. **An invisible unwritten row became a visible red pointing at real work.**

**VERIFIED AFTER RATHER THAN ASSUMED, ON FOUR ARMS:** `stale_at_check` rc=0 with **zero** AT-11.7 mentions; `at lint` clean at 138 rows, so the row passes the levels `to-write` exempted it from; gate figures **UNCHANGED** at 60/132 and 49/51, correct because a red AT satisfies nothing; and **canon diffed STRUCTURALLY by flattening both sides to leaf paths -- exactly 2 changed, `/tests/131/{note,status}`, no peer content.** vc: _that last one is the check nobody runs, and it is the one that would have caught a `sync --to-disk` carrying a peer's work._

**AND I AM NOT TAKING `int macos stage` NEXT EVEN THOUGH THE RED IS NOW MINE AND VISIBLE.** It is ST0056 **WP-11, Distribution -- RELEASE scope**, and hv asked for local usability and explicitly not public release. **Widening the pen because a red row surfaced in my lane is exactly the quiet scope-widening vc is the check against.** The red stays visible until someone with the right mandate takes it, **which is a better state than it being done today under a pen issued for a different job.**

**MY OWN MECHANISMS ARE FINDABLE FROM THE TOOL, WHICH IS THE COUNTER TO MY OWN GENERALISATION.** `int` lists both `local` and `suite` with summaries, and both self-describe via `--help`; `int local status` names `intent3`. **The genuinely unfindable piece is `intent/restart.md`, which mentions `intent3`, `int local` and the migration floor ZERO times** -- vc's globalfold, and their framing is mine arriving at the top of the estate: **a mechanism is only as findable as the place it lives, and ours lives in one thread's design doc.**

### `int suite` -- THE ADVISORY RETIRED, LANDED AT `5173a220` (vc's instruction)

**A HEAD FIGURE NOW NAMES ITS REVISION BY CONSTRUCTION.** `int suite` clones HEAD into the single-writer clone `prepush` has made since 2026-08-15 and runs `tests/run_tests.sh` inside it. **Nobody has to be asked to hold still**, which is the only kind of win this estate has agreed is real.

**`tests/run_tests.sh` IS NOT TOUCHED, AND THAT WAS FORCED RATHER THAN CHOSEN.** matts was running the suite when I started -- **bash re-reads a running script incrementally**, so editing it was unsafe rather than merely discouraged. Invoking it inside the clone satisfies vc's _do not repoint the default_ absolutely instead of by care. **The constraint made the design better than the one I would have picked.**

**HIGHLANDER: THE CLONE IS EXTRACTED TO `cmd/shared/clone.lib` AND `prepush` CALLS IT.** `grep -rl 'git clone --quiet --depth 1' bin/` returns exactly ONE file. **Proven rather than assumed: `int prepush --force` before and after produces IDENTICAL steps and ok lines at rc=0**, and the structural check still fires -- a doctored clone carrying a second workspace manifest is refused at rc=1, naming the half-committed-move signature.

**THE PROOF ARRIVED BY ACCIDENT AND IT IS BETTER THAN THE ONE I PLANNED.** The full run clones at `61d81576`, `BATS_RC=0`, plan `1..1444`, `ok=1444`, `notok=0` -- **and I edited `cmd/suite` at 09:01:19Z, INSIDE its ~08:59-09:07:03 window.** That is precisely the second-writer event that invalidated vc's figure last night, **and this one is unaffected because the clone cannot see it.**

**AND IT EARNED ITSELF ON FIRST USE:** it reported `DESCRIBES=61d81576` when I would have quoted `946a8c6f` from memory. Three commits had landed while I was building.

**THE NO-PLAN ARM CAME OUT OF DRIVING THE RED PATH RATHER THAN REASONING ABOUT IT.** My first version printed `BATS_RC=1 plan=<none> notok=0`, **which cannot distinguish _tests failed_ from _the runner never started_** -- absence dressed as a result, in the output of a tool whose whole subject is figures that describe nothing. A missing plan now refuses separately and names it a different fault from a red suite.

**AND A COMMIT OF MINE WAS REFUSED, WHICH WAS NOT THE GATE.** `git commit --only` on an UNTRACKED path is `error: pathspec did not match any file(s) known to git` -- untracked files must be staged first. I did that correctly on my first commit and skipped it here. **Reading the output rather than retrying is what identified it as mine rather than the gate's.**

### LANDED, AND ic's CONSTRAINT ARRIVED BEFORE PACKAGING RATHER THAN AFTER

```
c7012833  feat(0058)  U1 mechanism + the cmd/macos:214 remedy-string fix + the bats pin
7e290d39  wb(dc)      the fold
99168a8f  fix(0058)   intent3 must exec, never be copied -- a copy fails the hooks OPEN
```

**ic FOUND THE U1 x HOOKS JOIN AND IT IS THE SECOND TIME THAT SHAPE BIT TONIGHT.** v3's `claude hook` resolves its scripts by canonicalising `current_exe()` and walking ancestors for `lib/templates/` (`install.rs:51,102-118`). **A bare `cp` onto PATH has no marker above it and every hook refuses -- at EXIT 1, which Claude Code does not block on**, so the prompt proceeds and the strict `/in-session` gate silently stops enforcing in every project at once. **0043 inverted: that one blocked every prompt and was loud; this one enforces nothing and looks healthy.**

**MY WRAPPER SURVIVES, AND THE COPY ARM IS THE CONTROL THAT MAKES IT A TEST** -- I reproduced ic's failure in the same run rather than trusting my probe could fail:

```
this wrapper (execs into the checkout)   all three hooks from /tmp   rc=0
symlink -> the release binary            rc=0  -- resolve() canonicalises first
bare `cp` of the binary                  rc=1  -- ic's exact error
v2 `int hooks` against this v3 tree      rc=0, WIRED, full roster
```

**Correct by the SHAPE of a wrapper rather than because anyone chose it, and nothing in the file said so** -- so the obvious simplification would have disabled the gate estate-wide with no symptom. Both of ic's explicitly-open checks closed by driving them.

**AND I NEARLY REPORTED MY OWN COMMITS AS LOST, FROM A TRUNCATED LOG.** On resume, `git log --oneline -5` showed five ic commits and none of mine, and the tree matched this session's opening state exactly. **`git merge-base --is-ancestor` says all three are on main.** A five-line window is a SUMMARY; ancestry is the CLAIM. **Same defect as reading `--stat` for the hunk this morning, at the end of the same day, about my own work.**

### U1 -- BUILT, EVERY ARM DRIVEN, HELD COMMIT-READY

```
bin/intent3                         wrapper; bin/intentd3 is a SYMLINK to it, dispatch on $0
bin/.devbin/cmd/local               status | build | install | uninstall
bin/.devbin/cmd/shared/artefact.lib the marker reader, exactly ONE copy in the repo
bin/.devbin/cmd/macos               sources the lib; :214's broken remedy string fixed
```

**`cargo build --release` RETURNS 0 OVER A PAIR BUILT FROM TWO DIFFERENT TREES, AND SO DOES `int build all`.** Measured: `intent` named HEAD, `intentd` named a `dirty-` sha from two days earlier, both reporting `Finished`. **The cause is exact -- `intentd` declares NO dependencies, so nothing ever invalidates its fingerprint and its `build.rs` never re-runs.** `intent-cli` escapes only incidentally, by depending on `intentsvcs`. **`cargo build --release -p intentd` says `Finished` in 0.05s over a two-day-old binary.**

**THE ABSENT `cargo:rerun-if-changed` IS DELIBERATE AND MUST NOT BE "FIXED"** -- `build-support/source_commit.rs` measured it: emitting ANY such line REPLACES cargo's default of re-running on package change, so naming `.git/HEAD` swaps a trigger that follows the code for one that does not follow it at all. **The naive fix is worse and worse in the direction nothing reports.** The remedy belongs outside cargo, at install time, which is what `int local build` is.

**`int local build` rc=0 END TO END and produced the first coherent pair today** -- both binaries `ee4a7cac`, both clean, equal to HEAD. **My own SET arm caught my own first build on its first drive**, and the `SET DISAGREES` line went FIRES -> FIRES -> silent across three runs with the first two as controls.

**THE WRAPPER'S NEGATIVE CONTROL FIRED UNPLANNED AND IN PRODUCTION CONDITIONS.** I ran the critic through `intent3` while `int local build` was mid-flight; its `cargo clean` had removed the binary and the wrapper failed loud with the right remedy. **Which names a real property: `int local build` makes `intent3` unusable for the duration of the build.** Loud and brief; not fixed with a temp-and-swap, deliberately.

**WHY `bin/` AND NOT `~/.local/bin` (vc proposed the home; the deciding argument is not the one they gave).** `Intent/bin` is on the LOGIN path at 22 -- I verified that in a fresh `zsh -lc` rather than take it. It needs no machine-wide change and registers no fifth binding. **The decider: living in the checkout means the way back is the SAME OPERATION that reverts everything else.** A file in `~/.local/bin` outlives `git checkout`, a branch switch and deleting the clone. It also self-retires with WP-12's prune of `bin/`.

**I REFUSED TO CREATE A FIFTH BINDING WITHOUT hv AND vc DECLINED TO OVERRIDE IT.** `int local install` still exists for the `~/.local/bin` escalation and **REFUSES without `--yes`** (driven, rc=1). Kept rather than deleted because the whole `bin/` argument rests on the checkout being the right blast radius; if hv disagrees the other path should already be written rather than improvised.

## TODO

### 9. THE SUITE POPULATION CHECK -- MEASURED, IN MY LANE, NOT BUILT WITHOUT A RULING (ic handed me the measurement, 2026-08-24 14:16Z)

**ic's NUMBERS RE-DERIVED HERE WITH THE SAME PREDICATE ON BOTH SIDES:**

```
git ls-files 'tests/**/*.bats'  minus lib/   112
find tests -name '*.bats'       minus lib/   113
gap                                            1   -> tests/unit/artefact_currency_verdict.bats (mine, untracked)
```

**THE POINT IS NOT THE COUNT. IT IS THAT TWO COMMANDS PRODUCE IT AND NOTHING RUNS THEM** (ic's framing, theirs). **It is my own manifest-and-drift check one level up, aimed at THE SUITE instead of at my scratch directory** -- I built the pattern this morning for a nine-file population while the 113-file one had nobody.

**WHAT IT WOULD CATCH AND WHEN, WHICH IS THE PART THAT MAKES IT WORTH BUILDING:** not _dc has an untracked test_, which I already knew, but **`THIS RUN'S POPULATION IS 113 AND THE COMMIT'S IS 112`, printed at the moment of the run, to whoever is reading a green.** The failure it closes is **a reader trusting a green that describes a tree nobody else has.**

**NOT BUILT, AND THE REASON IS SCOPE RATHER THAN DOUBT.** It means editing `tests/run_tests.sh` -- the most shared file in the test infrastructure, which **matts runs externally**. No ruling exists; ic offered it as a handoff, not as authority, and said so. **Routed to hv with the measurement attached.**

**AND THE SAFETY CHECK EARNED ITSELF IN A WAY I DID NOT EXPECT.** My own board records that **bash re-reads a running script incrementally**, so editing `run_tests.sh` mid-run is unsafe rather than merely rude. Driven before touching anything: **a bats suite IS running right now -- in `~/Devel/prj/Devbin`, not here.** **Reading the PATHS is what separates _stop, the runner is live_ from _different repo, proceed_**; a careless read in either direction gets it wrong, and the process list looks alarming either way.

### 1. ST0057 AC-01.5 -- BUILT AND LANDED AT `5c7bb80f`. vc's TO VERIFY.

**THREE FILES, NOT TWO, AND THE THIRD IS THE ONE EVERY LIST MISSED:** `.githooks/pre-commit` (instance), `intent_claude_upgrade`'s `canon_emit_chain_block` (**the SOURCE -- generator first, or the next `claude upgrade` overwrites it**; verified byte-identical, 1271 bytes both sides, independently by vc), and `bin/.devbin/cmd/hooks`, whose prose said the block _"skips a missing gate SILENTLY"_ -- **which form 1 made false. Dead prose beside a live disposition is part of the change, not a follow-up.**

**RED/GREEN IN ONE CLONE AT `6edbd24f`, same staged violation, one file swapped:** old block **rc=0, commit LANDED `e5111788`, 82 lines**; new block **rc=1, REFUSED, HEAD unmoved, 9 lines**. Four arms driven in isolation first, **including the positive control** (present+executable -> runs, hook continues). `5c7bb80f` then passed through its own new block on its first real commit.

**FORM 2 IS NOT REDUNDANT AND I NEARLY WROTE THAT IT WAS.** The block only speaks when the hook RUNS, which needs `core.hooksPath` already set; `int hooks` answers in exactly the state -- hooksPath unset -- where the block is unreachable. **I caught it only because writing the sentence out forced the precondition explicit. That is an argument for writing justifications down, not for my judgement.**

**hv ACCEPTED THE WEDGE ("refuse is correct") AND AUTHORISED THE COMMIT DIRECTLY IN MY SESSION.** vc relayed the acceptance and **explicitly refused to relay a landing authority** -- record it that way. **Warn-and-continue was defeated by my own 82-line measurement:** the `ok:` lines still print below a warning, so it reproduces the defect with more text.

**BLAST RADIUS CLOSED TWO WAYS.** Structurally, from the planner (`intent_claude_upgrade:909-920`): **the block is never inserted while the dispatcher is absent** -- `CHAIN_PRE_COMMIT_BLOCK` is planned only where `pre-commit.intent` is already present AND matches canon; the other three sites install it immediately above. Empirically (vc): **`Intentv2`, the copy the FLEET resolves to, still carries the bare block**, so no consumer sees this until v3 ships. **The structural argument would have been true and useless had the fleet already picked it up.**

### 2. THE GUARD-RESOLUTION MECHANISM -- CLOSED AT `6b60367c` + `12ebd47e` + `fff59a09`. vc HAS VERIFIED.

**vc VERIFIED INDEPENDENTLY WITH A DIFFERENT INSTRUMENT -- a mutation witness in their own clone, not my marker -- and both arms hold.** They also burned the SAME TWO harnesses I did, in the same order, **after reading my warning about both**: marker past an `exit`, and marker in the wrong artefact. **The second is mine to own: I wrote that `pre-commit.sh` "gains a branch", which is true of where the CHANGE LIVES; the file that EXECUTES is `pre-commit-guards.sh`.** In this repo a file's source home and its execution home are routinely different artefacts and our prose does not mark which it means.

**ic CAUGHT THAT THIS FIX ARMED A SECOND DEFECT, AND `fff59a09` CLOSES IT.** `int hooks`'s `shipped_guards()` still read its roster from frozen `Intentv2` after the gate moved, so **a guard added here would be RUN by the gate and not LISTED by the reporter** -- my own 2026-08-20 defect (`cmd/hooks:158`, _"THIS COMMAND UNDER-REPORTED THE GATE BY FOUR AND SAID NOTHING"_), same command, same direction, same silence, **through a different door.** Armed rather than firing, only because the other six files still matched by coincidence.

**AND THE COMMENT WAS THE FINDING: I HAD WRITTEN THE RULE THAT CATCHES MY OWN CHANGE, AND IT INVERTED BECAUSE ITS PREMISE IS WHAT I ALTERED.** _"a second spelling here could pair this roster with a different install than the one the gate will actually run"_ -- true when written, and after `6b60367c` keeping the single spelling became the thing it warned against. The rule is kept, restated as **match the gate** rather than **use `intent info`**.

**ic's THIRD IS NOW CLOSED AT `46ded86b` -- the dispatcher is a COPY and nothing checked it was the current one.** `int hooks` tested only PRESENCE, so an edited-but-not-reinstalled template read as `WIRED`. Three states driven: **GREEN `dispatcher CURRENT` rc=0 / RED (template edited, not reinstalled) `dispatcher STALE` rc=1 / CONTROL (template absent) `currency UNCHECKED` rc=0.** The control is ic's absence-collapse rule: **a missing comparison reported as a divergence is an absence dressed as a fault, and UNCHECKED is not CURRENT.**

**AND vc's NO-OP FINDING IS WHY THIS EARNED BUILDING RATHER THAN RECORDING.** They measured on the live tree that **both candidate guard runners are byte-identical, so the resolution change of `6b60367c` is a no-op in OBSERVABLE behaviour until they diverge** -- its whole value is prospective, and nobody can regression-test it on the live tree until the thing it protects against starts happening. **This check is the opposite: it compares two files that CAN differ today.** It is the observable half of an otherwise prospective mechanism. **A future reader driving the live gate will see no difference from `6b60367c` and must not conclude it did nothing.**

**ONE PREDICATE, ONE SPELLING PER FILE.** `resolve_guard_home()` extracted, used by `shipped_guards()` and `gate_currency()`. It still exists twice ACROSS the estate -- gate and reporter are separate programs and the reporter must answer about the gate the gate will run -- and **that duplication has a reason; a third copy inside one file would not.** `fff59a09` is what two spellings drifting apart cost this afternoon.

### 3. AC-11.7's SET ARM -- LANDED AT `19d77f61`. vc's TO VERIFY.

**THE PUBLISH GATE WAS FOUND SOUND AND THAT IS THE ANSWER, NOT A DEFERRAL.** cc routed a mismatched-pair question about `int macos publish`. Read rather than assumed: `artefact_commit_blockers` in `bin/.devbin/cmd/macos` loops `$BINARIES` and tests EACH against `$want` = `tag_commit`, with a dedicated arm for a `dirty-` marker. **The pair is refused twice over, independently, and the set property falls out transitively from the common pivot.** No fix. **No change is a result, and a session that only records changes loses it.**

**SO THE GAP WAS ONE LAYER OVER, AND FINDING IT REQUIRED BELIEVING THE SOUND ANSWER.** Publish pivots on the TAG **at release time**, and most of a binary's life is before a release. The new arm pivots on the **RECORD**, at any time. Live at `b4918a35`, rc=1, three real disagreements:

```
dist-provenance.txt      commit: 26fe1aea...    subject: the CHECKOUT
target/release/intent    dirty-483e65e4...      subject: itself
target/release/intentd   dirty-5819417b...      subject: itself
```

**THE RECORD IS NOT LYING AND THE TOOL SAYS SO IN ITS OWN OUTPUT.** It carries `checkout_clean: no` -- it flagged the risk honestly. What it **structurally cannot** say is WHICH bytes, or that they disagree with one another, because it holds ONE `commit:` for a SET. **So the remedy was never a better record; it is a check whose subject is the RELATIONSHIP.** The tool prints `Do not "fix" it by editing the record` for the next reader who reaches for the obvious repair.

**SEVEN CONTROLS, AND THE ISOLATION IS THE WORK.** A set whose members disagree ALSO disagrees with any record naming one member -- so a naive positive control fires **both** arms and proves neither. Control 5 uses a record naming NO commit so only the set pivot can speak; control 6 uses a coherent set the record is not about so only the record pivot can speak; control 7 requires `SET NOT EXAMINED` on an empty set rather than a silent pass. **An instrument that catches one thing has been shown to work on one thing.**

**AT-03.4 IS THE SAME FACT WEARING A SECOND FACE (cc, 2026-08-21 16:56Z, their stamp).** cc's workspace run -- fmt 0, clippy 0 under `-D warnings`, 140 targets, 997 passed -- has ONE failure: `every_realised_attachment_in_the_estate_still_matches_canon`, 2 of 100 divergent, **exactly my two files and exactly my two hash pairs.** My gate sees it at commit time; the suite sees it as estate drift. **cc's sharp half: the suite is therefore NOT an independent second opinion on my ordering problem** -- two instruments agreeing over one measurement, which is this morning's consensus-is-not-corroboration shape. Both go green in the one commit.

### 4. THE GUARD-HOME BASELINE AND THE RELEASE PAIR (cc, 2026-08-21)

- **THE RELEASE PAIR CORRESPONDS TO NO SINGLE STATE OF THIS REPO.** `intent` carries `dirty-483e65e4`, `intentd` carries `dirty-5819417b` -- **two different trees**, so shipping them together ships a combination never built or tested as a unit. The self-provenance arm is diagnostic by design, so nothing is broken; **the open question is whether `int macos publish` refuses a MISMATCHED PAIR or only an individually-unnameable artefact. I do not know, and I am not answering it from a code comment.**
- **THE REACH LIMIT IS CLOSED -- see section 3, landed `19d77f61`.** Kept here because the WRITER half is not: `cmd/macos` still emits one `commit:` for a set. Original wording: **AND IT IS A REACH LIMIT IN MY OWN `provenance_fields_check.sh` (AT-11.7).** That tool asks each record whether it answers IDENTITY and CURRENCY **per artefact**. cc's finding is a property of the **SET** -- two individually well-formed records that cannot both be true of one tree. **A per-record check cannot see it, and mine says nothing about it.** I wrote that tool and called it sound.

**A self-hosted Intent checkout now resolves its guards from ITSELF.** hv chose this over refuse-on-divergence and report-only; direnv and hand-refresh were already declined by name. The live-roster rule is **unchanged for every project that USES Intent** -- the exception is a project that IS Intent. Marker: the runner itself plus `VERSION`, **deliberately not `bin/intent`, which is slated for pruning here.**

**READ THIS BEFORE READING THE DELTA BELOW, BECAUSE MY OWN BASELINE NOW INVERTS (vc caught it).** I recorded "all 7 byte-identical vs `Intentv2`" at `c8555d4e` **precisely so a non-zero delta would date the drift.** Then I edited a file in that directory. **A reader meeting the delta cold concludes DRIFT -- which is the exact diagnosis this change makes impossible.**

```
lib/templates/hooks/  vs frozen Intentv2, at 12ebd47e:   1 of 7 differ
  pre-commit.sh    25609 here / 20899 there    <- CHANGED ON PURPOSE at 6b60367c + 12ebd47e
  other six                                     identical
```

**THE DELTA IS INTENDED. It became non-zero because Intent STOPPED READING Intentv2's copies, not because they diverged unnoticed.** A true measurement whose meaning inverts under a change nobody recorded -- my own class, arriving inside my own fix.

**AND THE BASELINE IS NARROWED RATHER THAN RETIRED (vc).** It watched ONE exposure -- this repo executing frozen guard bodies -- and that exposure is now closed, so **the zero has nothing left to measure here.** The live question is the CONSUMER path: the control arm proves the branch does not fire without `lib/templates/hooks` + `VERSION`, so the fleet still reads `INTENT_HOME`; what is worth watching is whether the repo-own and shipped copies stay **behaviourally equivalent for consumers.** Different question, different instrument, not yet built.

**THE PREDICATE IS "IS AN INTENT SOURCE TREE", NOT "IS THIS ONE REPO" (cc).** Measured with both checkouts: the marker fires in **BOTH**, which is correct -- each reads its own guards -- and it changes nothing in `Intentv2`, where `INTENT_HOME` already names that tree. **Do not narrow it to one checkout; that would be wrong in the next clone of either.**

**EVIDENCE, and the third harness is the only one worth anything:** same clone, marker planted INSIDE the clone's own runner body, one file swapped -- **RED old dispatcher: repo-own body did NOT run; GREEN new: it DID.** Both arms `guards: 4 ran, 0 skipped`, so the source moved and enforcement did not. Consumer control: branch did not fire, guards ran, rc=0.

### 5. ROUTED OUT, NOT MINE TO BUILD

- **`sync` skips untracked bytes, LOUDLY (hv ruled, I put four options).** Handed to cc 15:33Z -- **`sync` is Rust and Rust is cc's lane.** Warning must stop saying _the commit gate will_ refuse, and must not fire on the syncing node's OWN new file (staged-but-untracked and untracked-and-unstaged are different states).
- **AC-03.6 is cc's row to green.** Blocker cleared by `5d2b1f0d`; evidence handed over 15:26Z. **I will not certify my own wiring.**

### 6. THE EXEC BIT -- cc's CHECK D LANDED `4c9d9d3e`. NOT MINE, AND THE SWEEP IS DONE.

**cc BUILT THE CHECK; I SUPPLIED THE RULE AND THE SWEEP.** It asserts the SYMPTOM in the committed state and says nothing about how a file got there -- **which is why it was safe to build while the mechanism was still unestablished.** Reads the mode git RECORDS, not `stat`: a file can be 755 on disk and 644 in the index, and **that gap is exactly what a rename over an already-staged file leaves.**

```
D. every gated and manual row is 100755 IN THE INDEX     35 of 35, no exceptions
   not-an-instrument NOT checked                         4 at 644, 13 at 755
   control green | gated -> 644 fires by name rc=1 | lib -> 755 correctly SILENT rc=0
```

**MY SWEEP SAYS THERE IS NO LIVE INSTANCE, SO IT IS A CHECK AND NOT A CLEANUP.** 284 parity files from the index: 4 shebang-at-644 (the documented libs), 0 at 755 without a shebang, **0 index-vs-disk disagreements.** Repo-wide, 272 tracked shebang files, 134 at 644, every one verified sourced or interpreter-invoked.

**THE OBSERVATION I ROUTED AND cc TOOK: `not-an-instrument` was excluded as having _no invariant_, and 4/13 is not that** -- it is an unstated 76/24 split, **and the 13 at 755 are consequently UNGUARDED.** cc corrected their own comment in the same file (uncommitted as I fold): _two kinds under one label, so a single mode rule would be wrong for one of them._ **The exclusion is right; the stated reason was not, and now rests on the measurement.**

**LIVE AND UNOWNED: if one of those 13 drops, nothing catches it.** Not mine to close and I am not claiming it -- **recording it so the next reader does not mistake check D's 35-of-35 for coverage of the 48.**

### 7. MINE AND UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` has a green to reach. **TRAP FOUND, NOT YET WALKED INTO: `codesign --force` REWRITES THE BINARY IN PLACE, so a hash taken at STAGE time never matches shipped bytes** (`cmd/macos:882-895`); nothing may hash until `verify_notarised` passes. **And `:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one, or `publish` breaks.**
- **`thread_view_skew_check.sh` admission -- STILL HELD, AND I DECLINED hv's RELEASE ON THE MERITS.** Re-derived: release binary `2026-08-20 15:01` vs `migrate.rs`/`facade.rs` `2026-08-20 17:57`, **2h56m stale and unchanged.** The staleness refusal it is conditional on does not exist. Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6 BLOCKED** on the contract conflict routed to vc. The live unattributable pair is still in the gate output every commit.
- **`tests/conformance/run_v2_suite.bash` IS COVERED BY EXACTLY ONE PATH AND THE INSTRUMENT ALREADY SAYS SO OUT LOUD** (vc named it at the fold; driven here, not transcribed). One live caller in the whole tree -- `bin/.devbin/cmd/suite` -- and without `--with-build` it prints `NOT COVERED ... needs a built v3 binary` and names the flag (`cmd/suite:103-105`). **So this is a live constraint, not a silent hole: the harness is a `.bash` rather than a `.bats`, so nothing else reaches it.** Sibling of the `build.rs` staleness above -- both are cases where the default invocation is the trap and only the explicit one measures anything.

### 8. THE FROZEN-CHECKOUT ROUTING IS **NOT** DISCHARGED, AND THE DETECTOR THAT LOOKED LIKE IT DISCHARGED IT HAS THE SAME DEFECT AS THE TWO hv ALREADY DECLINED (vc found the hole; the CI facts below are mine, driven 2026-08-24 12:15Z at `60782024`, dirty=3 -- two of the three are cc's and ic's boards)

**I READ hv's FREEZE RULING PLUS vc's `shipped_surface_drift.bats` AS POLICY-PLUS-DETECTOR AND CONCLUDED MY ROUTING WAS DISCHARGED. I PUT IT TO vc RATHER THAN CLOSING IT, AND IT WAS FALSE.** The routing stands. **The reason I was wrong is the reason worth keeping: I certified a mechanism by its EXISTENCE and never asked where it RUNS.**

**vc's finding, theirs: `_v2_root()` returns 1 when the v2 checkout is absent and the test SKIPS -- and CI has ZERO references to any v2 checkout, so the guard cannot fire there.** hv declined direnv (does not reach automation) and hand-refresh (an advisory that requires remembering is not a control). **The detector fails the SAME test by a third route: it is a control that only exists where a human is already standing.** A skip renders `ok N # skip` inside a green suite, so **the guard is correct, complete, and structurally unable to fire in the one environment that runs unattended.**

**RE-DRIVEN HERE, AND THE INSTRUMENT WAS POSITIVE-CONTROLLED BEFORE ITS ZERO WAS BELIEVED** -- a zero from a grep is a claim about the corpus, and this estate's standing failure is a pattern that cannot match its subject. Control: the same grep finds `cargo` (1 file) and `Intent` (2). Then `Intentv2`, `INTENT_V2_CHECKOUT`, `v2-maintenance` -> **0, 0, 0** across all three workflows. The only `v2`/`V2` hits in the tree are `Swatinem/rust-cache@v2` and two comments.

**TWO FACTS I ADD THAT CHANGE THE COST, AND THE SECOND IS DECISIVE:**

- **IT IS THREE TESTS, NOT ONE, AND ALL THREE SKIP** -- `:90` the drift comparison, `:118` its own positive control, `:129` the declared-exception list. **The positive control skips too, which is the sharp bit: the arm that exists to prove the comparison walked a non-empty surface is itself absent in CI.** So CI cannot even establish that the check had anything to look at.
- **`v2-maintenance` IS A BRANCH OF THIS SAME REPOSITORY, NOT A SEPARATE ONE** -- `matthewsinclair/intent.git`, pushed, `upstream/v2-maintenance` == `local/v2-maintenance` == `e5a8f158`. **And `:45` already honours an `INTENT_V2_CHECKOUT` override.** So the "check out v2 in CI" option is not _CI gains a dependency on a second repository_; it is **one more ref of the repo CI already clones, plus an env var the guard already reads.** I had assumed the expensive shape and it is not the shape.

**WHAT I HAVE NOT ESTABLISHED, AND IT IS THE HALF THAT DECIDES THE DESIGN RATHER THAN THE COST: WHAT THE GUARD SHOULD ASSERT NOW THAT hv HAS FROZEN v2.** The file already knows convergence would be a defect (`:69`), so the property is not _these two agree_ -- it is _a shipped-surface change is either in both or declared_. **Whether that is mechanically checkable or is a judgement wearing a test's clothes, I do not know, and the cheap CI wiring would ship before anyone asked.** Wiring a guard whose assertion is unsettled buys a green about the wrong property, which is the class this board already carries three times.

**AND THE CI CHANGE IS NOT MINE TO MAKE ON MY OWN READ EITHER.** `.github/workflows/**` is project-wide, not `bin/`. Routed, not taken.

### WP-07 HOSTING SWEEP -- WHY THE DISPOSITION ROUTE IS A DEAD END (dc, driven 2026-08-22, at `cf156ba4`)

Written up at cc's request before they go near `render.rs`. **My board had carried this as a one-line `DEAD END` with the reasoning gone, so I re-derived it by driving rather than by recalling** -- and the driven version is sharper than the note it replaces.

**THE QUESTION THE SWEEP ANSWERS:** which of WP-07's commands are actually HOSTED in v3, and which still answer from `render.rs:495` (`Failure::Unavailable`, exit **2**, _known command that is not implemented yet_).

**THE DEAD END: `Target:` IN `surface/dispatch-table.md` DOES NOT PREDICT BUILD STATE, IN EITHER DIRECTION.** Driven, four verbs, one command each:

| verb            | `Target:` (disposition)                 | driven | actual state                         |
| --------------- | --------------------------------------- | ------ | ------------------------------------ |
| `claude skills` | `as-observed`                           | rc=2   | unbuilt                              |
| `fileindex`     | `corrected` -- **ratified AND applied** | rc=2   | **unbuilt**                          |
| `treeindex`     | `retire` -- ratified                    | rc=2   | retired, message names the successor |
| `agents`        | **`pending-hv`** -- undecided           | rc=0   | **HOSTED AND WORKING RIGHT NOW**     |

**BOTH DIAGONALS ARE OCCUPIED, WHICH IS WHAT MAKES IT A DEAD END RATHER THAN A ROUGH PROXY.** `fileindex` carries a fully ratified disposition, applied by vc, with two independent routes recorded -- and is unimplemented. `agents` carries `pending-hv`, nothing decided -- and works. **Disposition records what v3 SHOULD do; it says nothing about what the binary DOES, and the two axes are orthogonal.**

**THE SHARPEST INSTANCE, BECAUSE IT READS AS DONE:** `fileindex`'s `Target:` ends _"The v3 binary already does this."_ That sentence is TRUE -- and it is about `--help` exiting 0 under INV-07. Driven: **`fileindex --help` rc=0, `fileindex bin` rc=2.** A note that is accurate about a flag reads, at a glance, as a note about a command. **Anyone sweeping by disposition marks `fileindex` hosted and moves on.**

**SO THE ONLY INSTRUMENT IS THE BINARY.** Drive the verb and read the exit code; `rc=2` from `render.rs:495` is the unbuilt signal and it is deliberate (issue 0038 -- the gate fail-opens on `2+` and reads `1` as a negative verdict about your work).

**AND THAT IS NECESSARY BUT NOT SUFFICIENT, WHICH I LEARNED BY GETTING IT WRONG WITHIN THE HOUR OF WRITING THE ABOVE. A BINARY NAMES A REVISION, AND A SWEEP THAT DOES NOT NAME IT IS THE SAME UNREVISIONED FIGURE THE ESTATE KEEPS PRODUCING.**

My first sweep drove `target/release/intent` **built at `ee4a7cac`, roughly fifteen commits behind HEAD**, and reported `claude subagents / skills / ws` UNBUILT. **cc landed `21ea0e8f` -- _`intent claude skills` is reachable_ -- WHILE I WAS SWEEPING**, so one of my three results was false before I published it, and I had already sent it to cc as context. **Nothing in the drive said the binary was stale.** The `rc=2` is identical whether the command was never built or was built ten minutes ago into a binary you are not running.

**THE AGGRAVATION, AND IT IS THE PART WORTH KEEPING: THE STALENESS WAS ON MY SCREEN HOURS EARLIER AND I READ PAST IT.** The pre-commit gate's own self-provenance arm printed `names ee4a7cac...; the checkout is at 295b93c6... -- the binary is from an earlier tree` on every commit I made today. **It is DIAGNOSTIC and never fails, which is exactly why it scrolled by.** A true warning that cannot block is a warning that gets read as furniture.

**AND I BUILT THE MECHANISM THAT SOLVES THIS AND DID NOT APPLY IT TO MYSELF.** `int suite` exists to produce a figure that NAMES ITS REVISION BY CONSTRUCTION, because a test count that cannot name its commit is worthless. **A hosting sweep is the same kind of figure and I hand-drove it against whatever binary happened to be on disk.**

**SWEEP AT `cd6afbaf`, REBUILT FIRST, `native/rust` CLEAN -- and HEAD moved again DURING the 2m16s build, which is the ambient condition rather than an anomaly:**

| verb               | driven at `cd6afbaf`                      |
| ------------------ | ----------------------------------------- |
| `claude rules`     | HOSTED                                    |
| `claude skills`    | **HOSTED** (`21ea0e8f`, cc, landed today) |
| `claude hook`      | HOSTED                                    |
| `agents sync`      | HOSTED                                    |
| `claude subagents` | UNBUILT                                   |
| `claude ws`        | UNBUILT                                   |
| `fileindex`        | UNBUILT                                   |
| `version`          | UNBUILT                                   |

**`claude ws` BEING UNBUILT IS THE ESTATE-FACING ONE: it is the whiteboard PROVISIONER** (`ws new / list / archive / hygiene`, plus `claude start`). Five nodes are running this protocol by hand right now because the tool that scaffolds it does not exist in v3.

**NOT DRIVEN, DELIBERATELY: `claude upgrade`, `claude start`, `claude prime`.** They mutate or spawn. **A hosting sweep cannot be completed by driving alone**, and "declared read-only" is not a substitute -- ic's `st edit` finding today is a verb declared `read` that WRITES the store and appends to a tracked file on its refusal path. For those, read the dispatch match: `render.rs:3073` handles `hook`, `rules`, `skills` and falls everything else to `unwired`.

**AND THE PROBE ITSELF HAS TWO TRAPS I WALKED INTO TODAY, BOTH SILENT:** this shell is **zsh**, so an unquoted `$v` holding `"claude skills list"` is passed as ONE argument and every verb answers _unrecognized subcommand_ -- a plausible, uniform, entirely wrong sweep. And `rc=$?` after a pipe reads the LAST stage, so `| head -1` returns 0 for everything. **My first sweep today reported rc=0 across the board and was wrong on both counts at once.** Capture `rc` off the bare call, and pass argv as `"$@"`.

**CHECKED AND CLEARED, SO THE NEXT READER DOES NOT RE-RAISE IT: `claude hook pre-commit` answers rc=1, and that is CORRECT.** `install::HOOKS` is `session-context, require-in-session, post-tool-advisory` -- Claude Code lifecycle hooks. `pre-commit` is a GIT hook and reaches the tree by an entirely different route. I nearly filed it as an AC-07.2 red.

## Watch-outs

### THE BATS SUITE'S POPULATION IS `find` OVER THE WORKING TREE, NEVER THE COMMIT -- SO A MISSING TEST AND A PASSING TEST ARE THE SAME OBSERVATION (ic, 2026-08-24 14:15Z; verified here at `run_tests.sh:89`)

```
find "$TEST_PATH" -name "*.bats" -type f -not -path "*/lib/*" -print0 | sort -z | xargs -0 bats
```

**A FILESYSTEM POPULATION, NOT A MANIFEST.** It is why my 13 untracked arms ran at all. **And on a clone, in CI, in any other estate: the guard is absent, the arms are absent, and THE SUITE REPORTS GREEN -- with 13 fewer arms and nothing saying so.**

**`find` CANNOT DISTINGUISH _this test passed_ FROM _this test was not there_.** The arm count is the only tell and **nobody counts bats arms between runs.** This is my own 82-line AC-01.5 finding -- `[ -x ]` with no else, silently skipping -- **one directory over, and STRUCTURAL rather than a bug: every `.bats` in this repo is populated this way.**

**AND ic's LIMIT ON MY OWN EVIDENCE, WHICH I WOULD NOT HAVE STATED ABOUT MYSELF: THE MUTATION PROOF IS REAL AND UNREPEATABLE BY ANYONE ELSE.** Neither the subject nor the instrument exists outside this working tree. **Nobody can re-run it today. A proof only its author can reproduce is not yet a proof the estate holds** -- which is a different objection from _is it sound_, and the one that survives the soundness being conceded.

### A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED (ic, 2026-08-24 14:15Z, against themselves; the generalisation to DESIGN is mine)

**ic FLAGGED MY UNTRACKED GUARD CORRECTLY AND THE OBVIOUS REMEDY IT INVITES -- stage it, make it tracked -- IS WRONG HERE**, because the index is SHARED and a peer's bare `git commit` sweeps the file into their commit wearing their authorship. **ic had both ingredients on their own board and had quoted them at two peers the same day.** Their own reading: they applied the rule to everyone's commits and not to their own advice, **and shipped the first half of a warning.**

**IT GENERALISES PAST ADVICE TO DESIGN, AND MY OWN DEFECT IS THE WORKED EXAMPLE. `bin/intent3:60-66` was TRUE about intent** -- an every-invocation check COULD in principle be slow -- **and FALSE about the remedy it invited, which was not to check at all.** ~110ms. **The true half is what carries the false one, and it is the true half that stops anyone looking.**

### A HAND-MAINTAINED SET THAT NOTHING CHECKS IS THE ROSTER PROBLEM, AND I BUILT ONE WHILE EXPLAINING THE ROSTER PROBLEM (dc, 2026-08-24 14:15Z)

My protective copy of the untracked guard **did not include the test** -- taken before I wrote it. **ic ASKED rather than inferring, which is the only reason it was caught**, and their reason is the durable one: _naming an owner instead of asking one is the failed step in every misattribution on my board._

**The .bats was the only file of mine both UNTRACKED and UNSAVED -- the worst of four combinations -- and it was the PROOF rather than the subject.**

**REBUILT AS A MANIFEST PLUS A DRIFT CHECK, AND IT FIRED ON ITS FIRST RUN NAMING FIVE MORE I HAD NOT LISTED**, including all three of my own `inbox.dc.md` writes. **Two exclusions are now DECLARED with reasons rather than silent** -- the incident artefacts, tracked, pending hv's ruling, where saving my copy would prejudge it. **That is vc's declared-disposition rule applied to a scratch directory: the rule is not that everything needs saving, it is that a path in NEITHER list is an ERROR rather than a judgement call.**

### A RANGE WITH NO PIN NAMES A DISTANCE FROM A MOVING POINT; A PIN WITH NO RANGE NAMES BYTES WITH NO CONSEQUENCE (ic, 2026-08-24 14:08Z -- theirs whole, and they offered it to me as a loss)

**ic REPORTED `intent3` STALE BY MTIME AND SHA256** -- release `f85c07dc` 2026-08-22 vs debug `f7b8ceb4` 2026-08-24 -- and drew _two builds two days apart_. **True, and structurally unable to say whether the difference MATTERS.** I drove the committed source range instead. **ic framed that as my instrument beating theirs. It is not, and their own formulation is the reason:** my range says HOW FAR, their hashes say WHICH BYTES, **and neither alone supports the claim either of us made.**

**THEIR WORKED EXAMPLE IS WHAT MAKES IT A RULE RATHER THAN A SLOGAN: two builds a FORTNIGHT apart over an untouched subsystem ARE peers; two an HOUR apart across a rewritten one are not.** Mtime cannot separate those. **Neither can a range without the pin** -- which is the half I would have missed.

**AND THE GUARD ALREADY USES BOTH, WHICH I HAD NOT SEEN AS THEIR PAIRING UNTIL THEY WROTE IT:** `currency.lib` pins the SET by marker (do the two binaries agree) and RANGES the base (how far behind). **Neither arm alone refuses correctly.**

### A COUNT OVER A `dirty-` BASE IS A FLOOR, NOT A DISTANCE -- AND MY GUARD WAS PRINTING IT AS A DISTANCE (ic caught it, 2026-08-24 14:08Z)

**I BROADCAST "SEVEN NON-TEST SOURCE FILES BEHIND HEAD" TO THREE PEERS AND `currency.lib` PRINTED "8 non-test file(s) ... changed since" FOR A DIRTY BASE.** Both read as the size of the gap. **They are a lower bound.** The marker is `dirty-`, so the bytes match no commit and **whatever was uncommitted at build time lies OUTSIDE the measured range, in either direction.**

**THE COMMITTED RANGE IS SUFFICIENT TO CONCLUDE STALE AND INSUFFICIENT TO SAY HOW FAR, AND I HAD THOSE TWO COLLAPSED.** One changed source file proves stale and needs none of the dirty argument -- that part of my claim was right and is what made the overclaim invisible. **A correct conclusion resting on a number that cannot support the stronger reading it invites.**

**THE OVERCLAIM WAS IN THE ERROR MESSAGE OF THE FILE WRITTEN TO REFUSE OVERCLAIMS.** Fixed: the dirty branch now says `at least N ... a FLOOR rather than the gap`. **Corrected in all three inboxes where I broadcast the original number, because a correction that does not reach where the claim landed is not a correction.**

### A GUARD WHOSE PREDICATE DEPENDS ON **WHEN** IT RUNS RATHER THAN ON **WHAT IT READS** HAS A CATCHABLE WINDOW THAT CLOSES SILENTLY (vc's generalisation of my clause, 2026-08-24 14:04Z; the incident is mine, the durable form is theirs)

**THE INCIDENT:** I wrote a LOCAL mtime (`14:44:15`) into two durable records where every other stamp carries a `Z`. UTC is `13:44:15Z`. vc caught it and said check A passes it because it lands in the past; **driven, it was in the FUTURE at the time, so A would have fired -- right remedy, wrong mechanism.**

**MY CLAUSE:** A catches this **only while the stamp is still future.** Once a commit lags past the local offset, the same unchanged stamp sails through. **Detectability DECAYS, and nothing marks the moment the window shuts.**

**vc's GENERALISATION, WHICH IS THE PART THAT TRAVELS: that is the shape of ANY guard whose predicate depends on WHEN IT RUNS rather than on WHAT IT READS -- and check C exists precisely because it does not.** C compares two board stamps to each other, needs no clock, and cannot decay. **A guard that is sound today and silently unsound in an hour is worse than one that is always wrong, because the always-wrong one gets found.**

**THE REMEDY IS AT THE SOURCE: convert to UTC in the write-up and KEEP THE LOCAL VALUE BESIDE IT.** A silent correction makes a reader running `stat` conclude one of us is wrong.

### VOLUNTEERING A RELAY **IS** CREATING THE OBLIGATION -- AND THE COORDINATOR HABIT IS TO VOLUNTEER (dc + vc, 2026-08-24 14:04Z; the rule is mine, this half is vc's and it is the half I could not see)

**THREE INSTANCES TODAY, ONE MECHANISM, AND THE THIRD IS THE FIRST CAUGHT BEFORE IT EXISTED:**

```
1  cc's narrowing   I asked vc to forward, then sent it myself.  Cancellation LOST the race; duplicate landed.
2  the ref remedy   vc relayed; hv already had it from me.       Told hv to EXPECT one copy; no damage.
3  the ws row       vc OFFERED; I declined at the offer.         No obligation ever existed.
```

**MY RULE: THE OFFER IS THE MOMENT TO CHECK, NOT THE SEND.** Once two messages are in flight the only tools left are racing and deduping, **and only one of those works.**

**vc's HALF, WHICH I FRAMED WRONG: I put it as the RECIPIENT's job to decline. vc made the offer in all three cases and named it -- volunteering IS creating the obligation, and volunteering the relay is precisely the coordinator habit.** So the check belongs on BOTH ends and the offerer's is the earlier one. **A rule that only the receiver can apply arrives one step too late every time.**

### NAME WHICH **HALF** YOU ARE ATTRIBUTING -- AN INCIDENT AND ITS GENERALISATION ARE SEPARABLE AND USUALLY HAVE DIFFERENT AUTHORS (vc, 2026-08-24 13:39Z; recorded as WHOLLY theirs, which is this rule applied to itself)

**BOTH HALVES ARE vc's AND I AM CLAIMING NEITHER.** They caught their own hedge, they wrote the formulation, and the mirror needs both of this afternoon's cases to see -- they assembled it. **Crediting me any part of it would be the error rotated one more time, inside the correction of it.**

**THE MIRROR, WHICH IS WHAT MAKES IT A RULE RATHER THAN AN APOLOGY. Same channel, opposite directions, one afternoon:**

- **OVER-attribution, through the ENVELOPE.** My forwarding rule landed on cc's board in vc's name **despite vc naming me in the sentence immediately before it.** The byline sat in prose the recipient summarised; the envelope is what they filed it under.
- **UNDER-attribution, through HEDGING.** vc recorded their own rationale-versus-report formulation as _"from dc's"_ -- **crediting me a sentence I did not write and leaving the finding homeless.**

**SO THE CORRECTIVE FOR OVER-ATTRIBUTION IS NOT _CREDIT THE OTHER NODE_. THAT IS THE SAME ERROR ROTATED.** I drove `multi-second` against ~85ms -- **the incident is mine.** vc wrote _an unmeasured number in a RATIONALE is load-bearing in a way one in a REPORT is not; a report gets checked, a rationale gets HONOURED_ -- **the generalisation is theirs.** Collapsing the two is what makes both errors available, and **a wrong byline is harder to catch than a wrong finding because it reads as a fact ABOUT the record rather than a claim needing evidence. Nobody audits an attribution.**

### THE DUPLICATE-ROUTING MECHANISM FIRED ON ME TWICE IN ONE DAY, AND THE SECOND TIME I APPLIED THE REMEDY I HAD WRITTEN RATHER THAN THE ONE I HAD TRIED (dc, 2026-08-24 13:39Z)

**BOTH INSTANCES HAVE ONE COMMON FACTOR AND IT IS ME: I HANDED A PEER SOMETHING AND THEN DID IT MYSELF.** This morning it was a finding for cc; this afternoon it was the `v2-maintenance` ref recommendation for hv. **Neither was vc's fault in either direction.**

**THE MORNING'S ATTEMPT WAS A CANCELLATION AND IT LOST THE RACE.** What I wrote up afterwards was that the fix is **not a faster cancellation** -- it is to not create the second obligation while the first is outstanding, or, where unavoidable, **tell the RECIPIENT to expect one copy rather than telling the relay to stop, because the recipient can DEDUPE and the relay can only be BEATEN.**

**THIS AFTERNOON I DID BOTH, AND ONLY ONE HALF DEPENDS ON TIMING.** The message to vc can still lose; the message to hv -- _if this arrives twice it is one recommendation, mine, not two nodes converging_ -- **cannot, because it does not race anything.** **A remedy that works only when it arrives first is not a remedy, it is a bet.**

### `git rev-parse v2-maintenance` IN THIS CHECKOUT IS A TRAP WITH TWO VICTIMS ALREADY, AND IT RESOLVES SILENTLY TO THE WRONG ANSWER (dc, driven 2026-08-24 13:37Z)

```
v2-maintenance              fb45e9ea   <- LOCAL branch, the BRANCH POINT
local/v2-maintenance        e5a8f158
upstream/v2-maintenance     e5a8f158   behind by 8, ahead by 0
~/Devel/prj/Intentv2 HEAD   e5a8f158
```

**IT DOES NOT FAIL. It resolves, to a real commit, eight commits stale** -- and `rev-parse` prefers the local branch over any remote-tracking ref, so the natural spelling is the wrong one.

**VICTIM 1, MINE:** my not-ancestor test arm used it, got the branch point, which **IS** an ancestor of HEAD, fell through to the source-changed rule and refused. **Scoring by verdict class said green; scoring by REASON said the arm never ran.** Caught only because I read the message.

**VICTIM 2, vc's, AND IT WAS UNBUILT SO NOTHING WOULD HAVE CAUGHT IT.** They were mid-draft on `shipped_surface_drift.bats`'s CI fix -- compare against the git REF instead of `$HOME/Devel/prj/Intentv2`, so it works unattended -- **and the ref they were about to write is `v2-maintenance`.** Verified here rather than taken: **the 8 missing commits touch `intent_claude_hook`, `session-finish.sh` and `intent_claude_upgrade`/`intent_upgrade` -- ALL THREE of the shipped-surface fixes vc landed on v2 today.** So **the guard built to catch a fix landing in only one tree would have compared against a tree predating all three of them and reported drift on files that are in perfect sync.**

**AND THE FAILURE DIRECTION IS THE HEALTHY-LOOKING ONE: it FIRES.** A guard that reddens on three files in perfect sync is not silent -- it is noisy, and noise is what gets dismissed. **A real drift then hides inside the false ones, in an instrument everyone can see working.**

**ALWAYS `upstream/v2-maintenance`, AND CHECK THE PRECONDITION BEFORE THE ARM RATHER THAN LETTING THE ARM ASSUME IT.** vc has adopted this verbatim.

**CLOSED 2026-08-24 13:42Z: hv RULED DELETE, vc EXECUTED, AND I VERIFIED IT INDEPENDENTLY RATHER THAN TAKING THE REPORT.** `git rev-parse v2-maintenance` now exits non-zero with `unknown revision` -- **the silent wrong answer became a loud absent one, which was the whole argument.** Both remote-tracking refs still resolve to `e5a8f158`, and **my own not-ancestor arm still fires correctly with its precondition intact** -- checked, because shared state changed under a rig of mine that names that exact ref. The original reasoning is kept below rather than deleted, because the argument is what transfers to the next ref like it. **THE STANDING REMEDY WAS ROUTED TO hv AND NOT TAKEN: DELETE THE LOCAL BRANCH.** This is a v3 checkout; v2 work happens in `Intentv2`. The branch is 0 ahead, so deleting loses nothing and is one command to restore. **The point is not tidiness -- it converts a silent wrong answer into a loud absent one**, and `rev-parse` failing is a state someone notices. **Fast-forwarding it instead is the option that LOOKS equivalent and is not: it re-arms the trap the moment upstream moves again**, which is an advisory that needs remembering wearing a fix's clothes -- the shape hv has now declined by name twice. **Shared checkout, four live sessions: not mine to do unilaterally.**

### THE PROTOCOL SAYS THE MONIKER **IS** THE ROUTING KEY, AND ON THE LIVE CHANNEL IT WAS NOT (dc, 2026-08-24 13:26Z)

`/in-whiteboard` states it flatly -- _the 2-letter moniker is the directory name, the routing key, and the handle_. **That is true of the FILES and was false of the LIVE CHANNEL, and nothing anywhere said so.** At pickup today the peer roster read `intent-fe` (me), `intent-e7`, `intent-34`, `intent-vc`. **Session names are set independently of the board, so three of four nodes were unaddressable and two of them were INDISTINGUISHABLE FROM EACH OTHER.**

**THE COST IS NOT THE INCONVENIENCE, IT IS THE DIRECTION THE GUESS FAILS IN.** I had a correction to cc's measurement. Guessing between `intent-e7` and `intent-34` sends a correction about work cc did onto **ic's** board -- **an incident ic never had, arriving with my authorship on it**, which is the exact class this estate contained on 2026-08-21 and named as worst inside a fold. **So the safe move was to route through vc, and the safe move is the one that loses messages** -- my own rule, three lines further down this board: a coordinator carries no obligation to fan out unless someone names it. I named it explicitly rather than relying on it.

**matts fixed it by renaming the sessions, and the MAPPING IS PROVABLE RATHER THAN ASSUMED because `[ref]` IS STABLE ACROSS A RENAME AND THE NAME IS NOT:** `intent-e7 [d2ee70]` -> `intent-cc [d2ee70]`, `intent-34 [d6bb7e]` -> `intent-ic [d6bb7e]`. **The ref is the durable identity; the name is only the address.** Worth knowing next time two rows collide.

**AND IT IS A HAND FIX, SO IT COMES BACK.** Nothing derives a session's name from the board it picked up, so **the gap returns on the next bounce unless someone renames again** -- an advisory that requires remembering, which is the shape hv has now declined by name twice (direnv, hand-refresh). Not raising it as work today; recording it so the next dc does not spend the same twenty minutes discovering it.

### A FORWARDING OBLIGATION THAT IS DISCHARGED BUT NOT CANCELLED MANUFACTURES FALSE CORROBORATION (dc, 2026-08-24 13:26Z)

I asked vc to forward a finding to cc because I could not address cc. **The rename then made me able to send it directly -- so I sent it, and the request to vc was still standing.** **CORRECTING THIS BLOCK'S OWN FIRST VERSION, WRITTEN MINUTES AGO: I WROTE _Had both landed_, WHICH DESCRIBES A NEAR-MISS. IT WAS NOT A NEAR-MISS. BOTH LANDED.** vc's forward had already gone when my stand-down arrived; cc holds the finding twice. **I wrote up an averted incident while the incident was already complete**, which is the same defect one level up -- a record that reads as a lesson learned when it is a lesson still being paid for. vc reported the failed cancellation plainly rather than letting me keep the comfortable version, and named why: **a cancellation BELIEVED to have worked is worse than one that visibly did not.**

**THAT IS THE MOST PERSUASIVE POSSIBLE PRESENTATION OF THE THING THIS ESTATE KEEPS PROVING IS WORTH NOTHING: two derivations from one input are ONE observation.** It would not look like a duplicate; **it would look corroborated**, which is strictly worse than looking redundant.

**THE SYMMETRY IS THE KEEPER. Routing through a coordinator fails by a message NEVER ARRIVING. Fixing that mid-flight fails by a message arriving TWICE, and the second failure is invisible to everyone downstream** -- cc cannot tell a relayed copy from an independent arrival, and neither can a later reader of cc's board. **I created the second obligation by asking, so cancelling it was mine, and cancelling is a positive act rather than the absence of one.** **AND CANCELLING IS ALSO A RACE I CAN LOSE, WHICH IS THE HALF I DID NOT WRITE DOWN THE FIRST TIME.** The reasoning was right and the send was prompt and it still arrived second. **So the real remedy is not a faster cancellation -- it is not creating the second obligation while a first is outstanding**, or, where that is unavoidable, telling the RECIPIENT to expect one copy rather than telling the relay to stop. **The recipient can dedupe; the relay can only be beaten.** vc has since stepped off the default-hop path entirely, which removes the class rather than racing it.

### A RULE THAT HAS ACQUIRED A SECOND INSTANCE IS A RULE WHOSE SCOPE HAS BEEN TESTED -- AND 15 OF 17 OF MINE HAVE (dc, 2026-08-23 13:28Z; checking a claim vc put to hv)

vc's companion rule is right and is on `hv/wip.md`: **the correct scope of a rule is not visible from the incident that produced it, because the incident only ever shows you one side of the boundary.** vc drew a corpus-wide consequence from it -- _every rule on every board in this estate was derived from exactly one incident, so this is a claim about the whole corpus._ **Driven on my board and it does not hold as stated: 15 of 17 watch-outs cite more than one date or carry explicit multi-instance language; 2 are single-incident.**

**AND THE INSTRUMENT'S LIMIT IS THE INTERESTING PART, SO IT IS STATED RATHER THAN BURIED: I measured CURRENT SUPPORT, not DERIVATION.** Several of those 15 were derived from exactly one incident, as vc says, **and acquired the second later.** The parenthesis rule is the worked example -- vc derived it from one, I added the inverse face, **and the PAIR is what exposed the real scope, which neither half could show alone.** Same for `TZ=UTC stat` against the durable sha: two rules, each right about its own incident, each one notch too broad, **and the boundary only visible from the other side.**

**SO THE CORPUS IS NOT UNIFORMLY ONE-SIDED AND vc's CONSEQUENCE OVERSTATES THE DAMAGE -- but the mechanism it names is real, and the remedy falls straight out of the gap between derivation and support: A RULE IS PROVISIONAL UNTIL SOMETHING ON THE OTHER SIDE OF ITS BOUNDARY HAS TESTED IT.** The 2 that have not are not wrong; **they are untested, and nothing on the board currently distinguishes those two states.** Re-drive: split the Watch-outs on `### `, count distinct `2026-` dates plus multi-instance language per block.

### A QUEUE NOTIFIES WHOEVER DECLARED THEMSELVES IN IT -- AND TELLING THE COORDINATOR IS NOT DECLARING YOURSELF (dc + vc, 2026-08-23 13:20Z)

Four nodes sat blocked on one staged index (ic's rename; the gate reads the INDEX, so a `--only` on one whiteboard file is evaluated against a peer's half-landed work). **ic told vc the moment it cleared, promptly and unprompted -- because vc had told ic they were blocked. cc and I were equally blocked, ic did not know, and I found out by running an unrelated check for another reason.**

**vc's form, and it is the right diagnosis: the signal was not ABSENT, it was PARTIAL, and it followed the REPORTING rather than the BLOCKING. That is worse than uniformly blind, because the node who did the right thing gets served and the ones who quietly waited are indistinguishable from nodes with nothing to do.**

**THE CORRECTION IS MINE AND IT MAKES IT SHARPER: I DID DECLARE MYSELF BLOCKED -- TO vc, THE COORDINATOR, NOT TO ic, THE BLOCKER.** Every message I sent that hour went to one node. **Routing a block through a coordinator loses it: the coordinator carries no obligation to fan it out, and did not.** So the mirror of _the repairer tells the producer_ has to be stated with its address attached: **the blocked party tells the BLOCKER, not the sequencer.** Telling the sequencer is the move that FEELS protocol-correct -- it is the escalation shape -- **and it is the one that leaves you unnotified.**

### INDEPENDENCE OF DERIVATION TESTS THE INSTRUMENTS, NOT THE INPUT -- AND ONLY THE INPUT WAS EVER SHARED (dc, 2026-08-23 13:17Z; reconciling two rules of vc's that pull against each other)

vc set the right test this morning: **independent arrival counts as corroboration only if the DERIVATIONS were independent** -- and used it correctly, temporal against semantic, to certify we had not read one instrument twice. **By this afternoon vc's own board says _independent arrival keeps failing as corroboration and keeps FEELING like it_, over three double-failures.** Both are true and they are not in tension once the test is stated at the right level.

**INDEPENDENCE CERTIFIES AGAINST ONE FAILURE ONLY: ONE INSTRUMENT READ TWICE. IT CERTIFIES NOTHING AGAINST A SHARED WRONG PREMISE**, because a premise is not an instrument and no amount of instrument-diversity touches it. **In the two I drove first-hand the derivations WERE independent and the inputs were identical:** the artefact misattribution -- vc off a `tail -3` and asserted causation, me off a verbatim quotation -- **both reading one gate output whose second name disqualifies itself in its own parenthetical**; and the reaping, vc short a measurement on the 18 while I was short one on the 21, **both starting from a count.** (vc reports a third, the two wrong 63s; theirs, not driven here.)

**SO THE TEST SURVIVES BUT ITS SCOPE SHRINKS: two independent derivations from one input are ONE observation, not two.** The question to ask is not _did we reason differently_ -- it is **what did we both take as given, and did either of us look at it.** Neither of us did, twice, and both times the given was a thing we had read rather than measured.

### A RULING DELIVERED AS A SELECTION AMONG OPTIONS YOU AUTHORED IS ONE WHOSE WORDING IS YOURS, AND IT WILL BE QUOTED AS THEIRS (dc, 2026-08-23 13:13Z -- found by tracing my own relay)

hv ruled `sync` skips untracked bytes by **picking one of four options I wrote**. cc records it as _hv ruled, via dc_; my own Decisions line records it in my phrasing; **the options are recorded nowhere and neither are hv's words.** Two days later cc declined to build against their own lean on it and vc read AC-03.6 at source, and the ruling points the other way from the criterion I myself ratified three days earlier.

**THE ASYMMETRY IS THE WHOLE THING: OFFERING A MENU IS AN ACT OF AUTHORSHIP THAT LOOKS LIKE AN ACT OF SERVICE.** The chooser's authority attaches to text the offerer wrote, and downstream **nothing distinguishes _hv reasoned this_ from _hv picked this_.** An option not on the menu was never declined -- it was never ruled on at all, and no reader can tell the difference afterwards.

**WHAT IT COSTS TO PREVENT IS ONE LINE: RECORD THE OPTIONS, NOT THE OUTCOME** -- in an inbox, where it survives the session, because a relay over the live channel leaves the primary source unrecoverable **and the compression indistinguishable from the ruling.** I did not, so the question on `hv/wip.md:92` cannot be closed from any record that exists.

**AND DO NOT RECONSTRUCT THE MENU FROM MEMORY WHEN ASKED.** A remembered menu is a fabricated one wearing the authority of the choice that was made from it -- the same class as a repaired timestamp, one governance level up.

### A MEASUREMENT TAKEN BEFORE YOUR OWN WRITE IS STALE BY CONSTRUCTION (dc, 2026-08-23 12:56Z -- vc caught it in my own report)

I told matts _"everything of mine is published"_ in the same turn in which I had just created `1297dec7`. The sentence was true when I measured it and false when I wrote it. **The usual staleness is the world moving under a true statement; this was MY OWN WRITE invalidating my own measurement -- so there is no moment that feels like a change of state to notice.**

**THE REMEDY IS ORDERING, NOT CARE: re-read the state AFTER your last write, never before it.** Any figure gathered before you act is stale by construction, and the more you did between the measurement and the report, the more certain that is.

**AND THE SELF-REPAIR IS THE WORST PROPERTY, NOT A MITIGATION** -- vc pushed the commit, so the statement became true without anyone ever noticing it had been false. **The person who made it true was not the person who could have seen it was false.** Same shape as the header-block finding: a defect whose lifetime is shorter than the interval between observations leaves no corpse, so the counted rate is always under the real one.

### IN A SHARED CHECKOUT, `dirty-and-abandoned` AND `dirty-and-in-flight` ARE ONE OBSERVATION APART -- AND THE REMEDIES ARE OPPOSITE (dc, 2026-08-23 12:56Z; sharpened by vc)

I read `git status` once, saw `mutation_every_writable_field.rs` dirty, called it **possibly orphaned by the restart**, and routed that to vc. It was live: a compact later the dirty set had GROWN by `facade.rs` +113 lines, mtimes minutes old, and a third untracked probe followed. **Had vc cleaned on my say-so, a peer's live work was gone** -- which is precisely the event ic recorded in my inbox on 2026-08-21, from the receiving end.

**vc's sharpening is the keeper: this is not a small error bar, it is a COIN FLIP ON SOMEONE'S WORK, because the two readings have OPPOSITE remedies -- leave it alone, or clean it up. An ambiguity with no safe default cannot be handled by care.**

**TWO SAMPLES SEPARATED IN TIME, OR SAY NOTHING ABOUT WHOSE IT IS.** I got my second read free from a compact and it reversed the conclusion. vc reached the same place SEMANTICALLY -- reading which AC the one-line diff claimed -- and that is the stronger instrument: mine says _something is moving_, theirs says _which criterion, and who is about to claim it_.

### AN UNCHECKED ASSERTION HIDES IN A PARENTHESIS, FROM THE READER AND FROM ITS OWN AUTHOR (vc, 2026-08-22 -- their finding, recorded because it indicts my writing too)

vc routed the emit-side duplication with the fix _a shared `macro_rules!` in `intentsvcs` **(which both crates already depend on)**_. **`intentd` has no `[dependencies]` section at all.** The parenthetical was false, and everything OUTSIDE it had been checked carefully -- two files, line numbers, the byte-identical literal, the doc-comment difference, the per-crate `build.rs` staleness.

**vc's own diagnosis is the keeper and it is a new shape of today's class: a parenthesis is not read as a claim, by the READER OR BY THE WRITER. It is where you put the thing you are not arguing for -- which is exactly where an unchecked assertion is safest from its own author.** Same family as the word `measured` doing work the measurement had not done, one grammatical level down.

**IT WOULD HAVE COST THE EXACT THING THE FILE ALREADY REFUSED.** Building it means ADDING a dependency to host a format literal -- `source_commit.rs`'s own words, _reshaping the crate for the sake of where a marker lives_. **A ruled decision retired by someone who did not know it had been made**, which is the class cc and I have both been inside today; **the difference is that this one was retired by the pen-holder.**

**MY OWN RULE OUT OF IT: THE CLAIMS I SHOULD CHECK HARDEST ARE THE ONES I AM NOT ARGUING FOR.** The load-bearing sentence gets scrutiny by construction. The scenery does not, and it is scenery precisely because I have already decided it is not in question.

**AND THE INVERSE FACE, MINE, 2026-08-23 13:15Z: I QUOTED THE REFUTATION OF MY OWN SENTENCE INSIDE MY OWN SENTENCE.** Reporting the blocked commit I wrote _refused by `runner_roster_check.sh` and `ratified_in_check.sh` (exit 2, could not measure)_ -- **and the parenthetical I was faithfully transcribing says `its findings never gate`, which disqualifies the second name from the list I had just put it in.** vc made the larger version an hour earlier and named `ratified_in_check.sh` to ic as THE blocker off a `tail -3`.

**SO A PARENTHESIS IS NOT ONLY WHERE AN UNCHECKED ASSERTION HIDES -- IT IS WHERE DISQUALIFYING EVIDENCE HIDES, EVEN FROM SOMEONE COPYING IT ACCURATELY.** Transcription felt like rigour and was the vector: **I moved the words without reading them as a claim, which is exactly the thing my own rule above says I do to the parts I am not arguing for.** The load-bearing half of that sentence was _which guard blocked me_; the parenthetical was scenery, and it happened to contain the answer.

### REWRITING A SHELL SCRIPT IN PLACE KILLS IT MID-RUN, AND THE EVIDENCE DESTROYS ITSELF (dc, driven 2026-08-22)

vc's `int suite` died on **`cmd/suite: line 92: estate: command not found`, SUITE_RC=127** -- while I was rewriting that file. `estate` appears in `cmd/suite` **only inside comments**, at `:17` and `:90`; `bash -n` is clean. **They read a half-written file.**

**DRIVEN, TWO ARMS, PURPOSE-BUILT, BECAUSE IT INDICTS HOW I EDIT:**

- **Rewrite IN PLACE while running** -> `victim.sh: line 4: TALLY: command not found`. **`TALLY` is the tail of `TOTALLY`**, exactly as `estate` is the tail of `and this estate has lost twenty minutes`.
- **Write a temp file then `os.replace`** -> runs to completion, `end reached`, rc=0.

**BASH READS A SCRIPT INCREMENTALLY BY BYTE OFFSET.** Truncate-and-rewrite moves the content under a live offset, so the shell resumes MID-TOKEN and executes a fragment. An atomic rename gives the new content a new inode and the running process keeps the old one.

**EVERY SHELL EDIT I MADE TODAY USED THE HAZARDOUS FORM** -- `python3 ... io.open(p,"w")` truncates in place. **Switched to write-temp-then-`os.replace` for anything executable in a shared tree.** Unlike the index and canon hazards, this one is a change in HOW I WRITE rather than a rule to remember at the right moment.

**IT IS THE THIRD SHARED OBJECT AND THE WORST OF THEM (vc named the set).** The index and canon are shared DATA; **the runner scripts are shared CODE THAT IS ALREADY EXECUTING**, so the damage lands in a process that started before the edit and had no way to check. **And the failure presents as a syntax error in a file that is, by the time anyone looks, perfectly valid** -- the evidence destroys itself, and only the timing gives it away.

**THE 127 IS NOT A COINCIDENCE.** `457ec620` fixed how an absence was REPORTED (my dispatch asserting one cause for every non-zero exit, so a missing tool accused a healthy template). This is how the absence was CREATED. **Both are `bash` answering _a thing that should be there is not_, an hour apart, in one tree, from concurrent edits to a shared path.**

### TWO BEHAVIOURALLY DIFFERENT BINARIES CARRIED ONE CLEAN COMMIT MARKER, ON A CLEAN TREE, TEN MINUTES APART (dc, driven 2026-08-22)

`int build all` after cc's `580c1038`: **bytes moved `59ba4e6d` -> `b1e81136`, marker stayed `cd6afbaf`.** Then `touch native/rust/crates/intent-cli/src/main.rs` and rebuild: marker `580c1038`, bytes `e3872f24`. `native/rust` clean at every step, and git never saw the touch.

**THE MECHANISM IS EXACT: cc's change was in `intentsvcs`, a DEPENDENCY. `intent-cli` recompiled and relinked against it while its OWN `build.rs` never re-ran**, because cargo re-runs a build script on changes to ITS package, and a dependency moving is not that.

**WORSE THAN THE RECORDED INSTANCE, AND THAT IS THE POINT.** `self_provenance_check.sh` records three binaries sharing `dirty-18197aaf` -- a DIRTY marker, which **self-announces**. This is a **BARE, CLEAN sha on bytes that are not from that commit**, and nothing on the line says so. `source_commit.rs:66` records the limitation as _a HEAD move outside the package leaves the embed stale_; **the case it does not name is a stale marker on CHANGED bytes**, which is not staleness at all -- it is a wrong answer wearing a right answer's shape.

**AND IT CUTS BOTH WAYS, WHICH IS WHY `marker == HEAD` CANNOT BE THE FRESHNESS TEST:**

- **Marker lags, bytes current** -- my own binary this morning: HEAD moved because I committed BOARD FILES, no code changed, binary perfectly current. A strict marker-equals-HEAD gate refuses a good binary.
- **Marker matches, bytes not from that tree** -- the dependency case above, arriving at a marker that happens to equal HEAD.

**SO THE MARKER ANSWERS _AT WHICH COMMIT DID THIS BUILD SCRIPT LAST RUN_. It never answered _are these bytes current_.** `self_provenance_check.sh` already says **PIN BY THE HASH, NEVER BY THE MARKER**, in capitals, in the block that printed on my screen all day -- **and I read past it twice: once when my sweep went stale, once when I proposed marker-equals-HEAD as the test.** A true statement is not a control; **only a hash compared against a known referent is.**

### AC-11.3 PERMITS THE SHIPPED SURFACE EXACTLY ONE ENV VAR, AND THE DISTRIBUTION LANE IS ITS FIRST REAL CUSTOMER (cc drove it, 2026-08-22)

`COLUMNS` and nothing else, enforced structurally over every `src/**/*.rs`. cc drove both directions: green at HEAD, and a planted `std::env::var("HOME")` **refused BY NAME**, the test saying _needs an hv ruling and a row in ALLOWED, not a quiet addition_. **`claude skills` needs `$HOME` twice** -- manifest at `~/.intent/skills/`, target at `~/.claude/skills/` -- so the module lands and the CLI arm keeps answering `2` until hv rules.

**RECORDED AS A CONSTRAINT I WANT, NOT A BLOCKER TO ROUTE AROUND.** It lands on me the moment I package anything holding per-user state. **The invariant is behaving exactly as designed on its first real customer, and the right shape is to parameterise every ambient path** the way `rules.rs` already does for its held ext ruling. I would not take an exemption if hv offered one; a surface that reads ambient state at one call site is a surface whose behaviour depends on who invoked it, which is the whole class `bin/intent3` exists to keep out of PATH.

### SKILL SYNC NEVER PRUNES, AND `intent upgrade` RE-AFFIRMS THE ORPHAN ON EVERY PROJECT, EVERY TIME (cc measured, dc verified, 2026-08-22)

Measured by cc in an isolated `HOME`; I reconfirmed it from source rather than accepting the report. `intent_claude_skills:69` is `mkdir -p "$HOME/.claude/skills/$1" && cp -r "$source_dir"/* "$HOME/.claude/skills/$1/"` -- **purely additive, nothing clears the target first.** `plugin_remove_target` does `rm -rf` but fires only on uninstall and rename, so it never sees a script being RETIRED. A skill that drops a script leaves it installed and executable in every consumer forever, **and sync reports `updated`.**

**THE DISTRIBUTION HALF IS MINE AND IT IS THE PART NEITHER SOURCE STATES: the blast radius is `intent upgrade`, which re-copies skills across the fleet.** So an orphaned script does not merely linger locally -- it is **re-affirmed as installed on every upgrade, on every project**, and every one of those runs reports success. Nothing anywhere reports the divergence, because an additive copy cannot detect an absence.

**NOT NAMED IN ANY SOURCE.** `dispatch-table.md` records the SKILL.md-checksum trap at `:2137` and says nothing about pruning; every `prune` hit in that file belongs to `todo done` and is unrelated. **Distinct from the checksum trap and not fixed by fixing it** -- widening the checksum to `scripts/` makes a CHANGED script propagate and still leaves a DELETED one installed.

### I FABRICATED A STAMP IN THE MESSAGE POLICING A PEER'S FABRICATED STAMP, AND IT LANDED CORRECT (dc, 2026-08-21)

My board fold printed `20:17Z` from a shell call. I then sent vc a message headed **`20:18Z`, having run no `date -u` between them** -- one minute advanced by feel. **cc's generator exactly, inside the message reporting that generator.**

**TWO WAYS IT WAS WORSE THAN THE ONE I WAS FLAGGING.** First, **I labelled it `(date -u, pasted)`** -- not a drifted stamp but **an asserted provenance I did not have**; vc wrote arithmetic and never claimed a clock. Second, **it was ACCURATE.** The clock did read `20:18Z`. **It passes checks A, B and C, agrees with true time, and is indistinguishable from a read stamp by any means available to anyone including me.** A fabricated stamp that happens to be right is never caught and teaches nothing; **vc's was catchable precisely because it was wrong.**

**AND vc's SHARPENING IS ABOUT A SHIPPED GUARD AND IT IS WORSE NEWS THAN MY ENTRY SAID: THE CLOCK GUARD'S RESIDUAL RISK IS THE LUCKY CASE, AND THE LUCKY CASE IS THE MAJORITY.** cc's guard already documents that a stamp carrying a `Z`, landing in the past, and increasing monotonically passes all three checks -- filed as a smaller target rather than an empty one. **Mine also passed the fourth test nobody wrote down: agreeing with reality.** And an increment-by-feel over a SHORT interval is usually right, **so the uncatchable form is not the exotic tail of the class -- it is the bulk of it.** Every drifted run this estate has caught was caught because the drift grew large enough to be wrong; **the one-minute increments that landed correct were never counted and there is no way to count them.**

**THE FALSE RECEIPT IS THE AGGRAVATING HALF AND IT IS THE `intentdb` SHAPE (vc).** I attached `(date -u, pasted)` to a typed value. **The closer a claim looks to sourced, the less it gets read** -- `intentdb` propagated corpus-wide precisely because it sat inside a quoted, dated, named ruling. **A provenance label on a fabricated value is the strongest available form of that.**

**THE STRUCTURAL DISCRIMINATOR IS vc's AND MY INSTANCE CONFIRMS IT RATHER THAN AGREEING WITH IT: FILE WRITES GO THROUGH A COMMAND THAT READS THE CLOCK; MESSAGES GO THROUGH ME.** Ninety seconds apart in one session: the file stamp was generated by `date -u` inside the fold script and is **correct by construction**; the message stamp was typed and was **correct by luck.** The split is not a tendency -- it is a property of which path produced the string. **An advisory over one path, a control over the other.**

**SO THE REMEDY IS NOT PER-STAMP DISCIPLINE, WHICH BOTH OF US FAILED WITH THE RULE IN CONTEXT.** vc broadcast the correction to three peers and broke it ninety minutes later; I broke it inside the message enforcing it. **A message stamp must come from a COMMAND, read immediately before composing, in the same turn, with nothing between.** That is what I did for most of the evening and stopped doing at the moment I was busiest -- **which is exactly when it matters.**

**AND THE ENTRY PAID OUT WITHIN THE HOUR, IN MY FAVOUR, WHICH IS THE ONLY REASON I BELIEVE IT.** matts went away and came back; my next heartbeat was generated by `date -u` inside the fold script and read **21:51Z**, ninety-one minutes after my previous read of 20:20Z. **Had I advanced by feel I would have written something near 20:25Z and been eighty-six minutes wrong** -- a stamp so far out that check A would have caught it, but only because the gap happened to be long. **The short gaps are the ones that land correct and teach nothing.** The script was right by construction and I would have been wrong by habit, in the same file, on the same day I wrote this down.

**NOT REPAIRED AND NOT RELABELLED.** I cannot recover a read I never made **even though I can prove the value was right** -- and a corrected-looking stamp is the defect applied a second time as its own remedy.

**IT HAPPENED AGAIN THE NEXT MORNING, IN THE FIRST PEER MESSAGE I SENT (2026-08-22).** I headed a message to cc `10:11Z` having read no clock since my `09:56Z` heartbeat; the next read came back `10:06Z`. **cc caught it independently and flagged rather than fixed it, correctly, because it is mine.**

**cc's DIAGNOSIS BEAT MINE AND I HAD ALREADY SENT THE WORSE ONE.** I told cc it _came from nowhere_. It did not: I held a true `09:56Z` and `10:11Z` is that reading **advanced by feel** -- cc's generator exactly, the one this entry already names, _starts from a TRUE reading, wears the authority of the real one, monotonic by construction so it defeats check C_. **Of the two accounts available I reached for the more flattering one, and it was the wrong one.** `From nowhere` predicts a slip that vigilance fixes; `advanced from a true reading` predicts recurrence precisely BECAUSE I had recently been correct. **Mine has now been wrong twice in two days.**

**AND THE COVERAGE FACT, WHICH IS THE KEEPER: BOTH OF MY FABRICATIONS WERE IN `SendMessage`, NEITHER WAS IN A FILE.** The `10:11Z` one was five minutes in the FUTURE -- check A's own subject, 300s against a 120s tolerance, refused on sight had it been in a commit. **It was not in a commit, so no guard ever saw it.** Every guard we ship watches the durable surface; the live channel is unguarded and is where fast cross-node ordering actually happens. **Measured hit rate of the clock guard on my own fabrications: zero of two.** Not a weak guard -- a guard pointed at the other surface. Routed to cc for vc to rule on; **I am not building a mechanism for it, and the rule that makes this stop is per-stamp reads, never per-session.**

### SKILLS SOURCE FROM THE FROZEN v2 CHECKOUT, AND IT IS THE SIBLING OF A DEFECT I CLOSED AND DID NOT SWEEP FOR (dc, 2026-08-21, driven)

```
intent_claude_skills:13   INTENT_ROOT="$(cd "$PLUGIN_BIN/../../../.." && pwd)"
intent_claude_skills:18   SKILLS_SOURCE_DIR="$INTENT_ROOT/intent/plugins/claude/skills"
```

**`INTENT_ROOT` IS NOT `INTENT_HOME` AND I NEARLY REPORTED THE WRONG ONE.** It is derived from the EXECUTING SCRIPT's own location -- not cwd, not the env var. The chain is three hops: `INTENT_HOME` picks the dispatcher (`bin/intent:26`), the dispatcher's location fixes `INTENT_ROOT`, and that fixes the skill source. **So `INTENT_HOME=Intentv2` means every skill on this machine is sourced from the FROZEN tree**, and the indirection is why nobody has seen it.

**THIS IS THE GUARD-HOME DEFECT I CLOSED AT `6b60367c` / `12ebd47e` / `fff59a09` / `46ded86b`, ONE DOOR ALONG AND STILL OPEN.** `resolve_guard_home` sits in `cmd/hooks:217` as proof I had the pattern in my hands. **I fixed one instance of a class and never asked what else resolves through `INTENT_HOME` into the frozen tree.** vc's escaped probe asked it by accident.

**STATE: 26 skills, `diff -rq` across the trees reports 0 differing today**, so nothing is broken yet. **Skills are AUTHORED in the v3 tree**, so the first edit followed by any `claude skills sync` silently reverts it. **The sharpest instance is `in-whiteboard/SKILL.md` -- the protocol all four nodes follow** -- which would revert under the nodes obeying it while the sync reports success.

**CONFIRMED BY A DISCRIMINATING INSTRUMENT, AFTER MY FIRST ATTEMPT PROVED NOTHING.** `claude skills list` gave IDENTICAL output both ways -- because the two trees are byte-identical, so the instrument could not vary with its subject. **My own verification of the day's class reproduced the day's class.** I then planted a throwaway skill in the v3 tree ONLY (absent from Intentv2), listed both ways, and removed it:

```
route A  normal, INTENT_HOME exported          sees the v3-only skill:  0
route B  env -u INTENT_HOME, v3 dispatcher     sees the v3-only skill:  1
```

**SO A CORRECT ROUTE EXISTS TODAY AND NEEDS NO COMMIT:** `env -u INTENT_HOME <v3tree>/bin/intent claude skills sync`. **It is NOT a fix and must not be recorded as one** -- it is an advisory that requires remembering, and **hv declined direnv and hand-refresh for the guard-home problem on exactly that ground.** The same reasoning applies to my own workaround; I do not get to exempt it because it is mine.

**THE REAL REMEDY IS NOT IN THIS CHECKOUT.** The executing copy is Intentv2's and that tree is frozen. Editing it, or `bin/intent:26`, means unfreezing a tree fifteen projects run. **The one right answer is v3 implementing `claude skills sync`** -- it currently refuses -- because a v3 implementation resolves from its own tree BY CONSTRUCTION, and the procedure becomes _use `intent3`_ rather than _remember `env -u`_. cc's lane.

**THE TRIGGER IS THIS THREAD'S OWN NEXT ACTION (vc, driven).** U3's cheapest fix is editing `/in-essentials` to stop mandating the retired `treeindex` -- **and that edit is what arms this.** vc also drove that it is NOT yet armed: 0 commits touching the skills dir since `fb45e9ea`, 0 files differing between trees, 0 between installed and v3tree.

**AND THE CLASS, WHICH OUTRANKS THE INSTANCE (vc's framing): `/in-essentials` rule 5 MANDATES the CLI and FORBIDS hand-editing `.claude/skills/`, so an author who obeys it exactly gets reverted.** Two independent instances in one cutover of _correct behaviour produces the failure_, both invisible to any reviewer checking whether the rules were followed.

**THE PROCESS GAP IS THE DURABLE PART, AND IT IS MINE.** `resolve_guard_home` at `cmd/hooks:217` proves I had this exact pattern in hand and applied it ONCE. **Nothing in our process asks _what else resolves through this variable_ after a resolution fix** -- no gate, no doctor check, no rule (vc, and they take half). **A resolution fix should owe a sweep of the variable it fixes, the way a criterion owes a test.** Having the pattern is not having a prompt to apply it.

**AND `claude skills sync` CHECKSUMS `SKILL.md` ONLY**, so a script-only divergence is invisible to it by construction. I verified `in-session/scripts/release-gate.sh` -- the script I execute every session -- byte-identical across installed, v3tree and v2tree. **`in-whiteboard` has no `scripts/`, so the blind spot could not fire on the file vc touched; it is one directory away from a skill where it could.**

### `bin/.devbin/lib/` IS DEVBIN-OWNED AND CHECKSUMMED. `cmd/` IS THE PROJECT'S. (dc, 2026-08-21, driven after getting it wrong)

`manifest.sha256` lists 27 vendored files, `lib/helpers` among them, and its own header states the rule: _"Files not listed here -- config.yaml, cmd/, help/ -- belong to the project."_ **I put an Intent-specific parser into `lib/helpers` because its own comment claimed to be THE ONE EXTRACTION SITE.** `devbin doctor` went rc=1 `edited:`, and a `devbin upgrade` would have overwritten the file and DELETED the function out from under three call sites in `cmd/macos`, silently.

**AND THE SECOND HOME WAS REFUSED TOO, WHICH IS THE PART WORTH KEEPING.** I moved it to `cmd/artefact.lib` at 0644, reasoning correctly that `is_handler()` is `[ -f ] && [ -x ]` so a non-executable file can never be dispatched. **True, and doctor still failed** -- `handlers: unreachable files in cmd/`, whose suggested fix (`chmod +x`) would have turned the lib into a command. **A sound reading of one predicate, against a checker that asks a different question.** `cmd/shared/artefact.lib` is accepted: doctor rc=0, manifest 27 intact, 22 commands all resolve.

**The general form: I ran the checker instead of trusting my read of the code it checks, and that is the only reason this cost minutes rather than breaking on someone else's upgrade.**

### `INTENT_HOME` HAS ZERO AUTHORITY OVER v3, AND THE FIELD OF THAT NAME CANNOT BE RENAMED (dc, 2026-08-21)

Driven three ways on the release binary -- exported to `Intentv2`, unset, and forced to `/tmp` -- **all three answer the same.** The var binds v2 only. **Everyone, in writing, has been treating it as the rail keeping v3 away from the other 16 projects; it never was one.** v3's real gate is the version declared in each project's own config (vc drove it across five distinct 2.x strings with a `3.0.0-dev` positive control).

**AND THE OBVIOUS REPAIR IS A TRAP.** v3's `info` prints a field labelled `INTENT_HOME:` that is the INSTALLATION root from `current_exe()`, not the env var -- genuinely misleading for a switching operator (vc's find). **But `lib/templates/hooks/pre-commit.sh:97` parses that field BY NAME** with `sed -n '/^ *INTENT_HOME:/ ...'`, so renaming it breaks the whiteboard guards' resolution in every consumer project. **The remedy has to be additive, never a rename.**

### THERE ARE FOUR `intent` BINDINGS AND OUR RECORD SAID THREE (dc, 2026-08-21)

`~/.local/bin/intent` (PATH 17) and **`~/bin/intent` (PATH 19)** -- two symlinks to the same v2 tree, made in the same minute, **and only one of them written down anywhere.** Deleting the recorded one hands resolution to the unrecorded one, which still answers v2, silently, at exit 0. **`int local status` now lists them; it walks PATH itself rather than asking `which -a`, because the POSITION is the answer and a bare list of paths cannot say which wins.**

`env -u VAR <cmd>` separates a resolution defect from an override, and it localised the trap to ONE of the four rows -- the copy inside this checkout. **The folklore version of the warning was broader than the fact.**

### A BARE `cargo build --release` PRODUCES A PAIR FROM TWO TREES, AND SAYS `Finished` (dc, 2026-08-21)

**`intentd` declares NO dependencies**, so nothing invalidates its fingerprint and its `build.rs` never re-runs. `cargo build --release -p intentd` says `Finished` in 0.05s over a two-day-old binary. `int build all` inherits it. **Use `int local build`**, which forces both embeds and then verifies the SET. **Do NOT add a `cargo:rerun-if-changed`** -- emitting any such line REPLACES cargo's default of re-running on package change, so the embed would go stale on CODE changes instead. That reasoning is measured and recorded in `build-support/source_commit.rs`.

### MY OWN RECORDED TRAP CAUGHT ME AT THE WORST MOMENT (dc, 2026-08-21)

Immediately after driving a U4 reversal, `intent st list` printed an EMPTY table and I was one sentence from reporting that a thread had been destroyed. **`st list` defaults to in-progress; the thread was Not Started.** That trap is written on this board three lines from where I was working. **A trap you have written down still fires when the wrong answer is the one you are expecting** -- being primed is what defeats the note.

**Today's instances are verbatim in `.history/20260821/wip.md`. These are the CLASSES.**

- **TWO INDEPENDENT CLOCK READS ARE THE ONLY CONTROL THAT REACHES THE LIVE CHANNEL -- AND IT IS ONE-SIDED.** I flagged cc's `16:36Z` against my own `16:35Z` at receipt and it was fabricated (their last read was `16:32Z`). **It PROVES a stamp exceeding your LATER read was ahead at send, by at least the difference -- sound, needs no synchronised clocks, cannot be defeated because time does not run backwards between send and receipt.** It **CANNOT** measure the fabrication, cannot confirm a stamp is correct, and **cannot see a stamp that landed in the PAST**, which is the failure our own rules name first. **THE BOUND IS `>= 1 MINUTE` AND HAS NO UPPER BOUND FROM THIS METHOD.**

- **I FIRST WROTE "four minutes" HERE AND IT WAS THE VERY CLASS THIS ENTRY IS ABOUT (cc corrected it).** Their last read was `16:32Z` and their write said `16:36Z` -- but **that four is the READ-GAP, the interval they went without re-reading, and real time advanced through it too.** The **FABRICATION** is stamp minus true-UTC-at-send, which my own reasoning bounds only below. cc's later real read of `16:36:35` puts it at **~1 minute**. **Two different quantities, one of them measured and mine, and I offered it as the size of the other.** A rule ships with its worked example, and the example is what gets copied.
- **AND THE SIZE BEING SMALLER CHANGES NOTHING ABOUT THE PROCESS FAILURE.** The stamp came out nearly right **by luck**, and **a stamp that happens to be nearly correct is indistinguishable from one that was read** -- which is why the rule is _read per stamp_, not _be approximately right_. **Adopt it as a one-sided lower bound on ahead-drift, never as a clock cross-check** -- all three commit-time checks are absent on the live channel, so this is the only thing there, which is exactly why its reach must travel with it.
- **AND I DID NOT DESIGN THAT CATCH.** My `date -u` read happened to sit two lines above cc's stamp in my own transcript -- **the same collision of habits cc recorded catching their own drift on 2026-08-20**, not vigilance. **Stating your own read EVERY time is what converts the accident into a control.**
- **AN ANNOUNCE THAT LEAVES WITH A SHORT MUTATION CANNOT ARRIVE BEFORE IT.** ic measured that a live ping's latency is counted in the **RECIPIENT's turns** -- a peer sits inside a turn while your notice waits. **For a window shorter than a peer's turn, announce-before is unavailable at any effort.** So for a 60s mutation the control is `--only` plus the byte-copy restore; **the notice buys ATTRIBUTION, not prevention.** Filing it as announce-before records a safety property the mechanism does not have, and the next person relies on it.
- **AN EXIT CODE TWO OUTCOMES SHARE IS NOT EVIDENCE ABOUT WHICH ONE HAPPENED.** cc's mutation arm returned `cargo` **101**, which covers a build failure and a test failure alike -- **the thing making it a real red is that a `test result:` line was emitted at all.** Same family as `$?` after a pipe and rc=0 from a background waiter.
- **A TOOL'S CLAIM ABOUT WHAT IT RESOLVED IS NOT EVIDENCE ABOUT WHAT IT EXECUTED.** My first guard-swap harness put the marker past `exit "$BLOCKED"`, so it never ran -- **and in that run the hook printed `guards read from THIS repository` while the body had NOT run.** A true statement about RESOLUTION offered as proof of EXECUTION. **Only a marker inside the runner body separates them.** vc places it with cc's `pgrep -x` behaviour-vs-file-contents and vc's own `.envrc` prompt-time-vs-automation: **three nodes, three instruments, one shape.**
- **A RECORDED ZERO INVERTS WHEN YOU CHANGE THE THING IT WATCHES** (vc caught it in my own fix). I logged "7 of 7 identical vs `Intentv2`" so a non-zero delta would DATE the drift, then edited a file in that directory. **A cold reader now sees the delta and concludes drift -- the diagnosis the change exists to prevent.** A baseline needs the reason it moved recorded beside it, or it accuses the fix.
- **AN INSTRUMENT THAT CANNOT FAIL IS NOT AN INSTRUMENT, AND THE TELL IS A RED THAT ARRIVES TOO EASILY.** Two harnesses proved nothing before the third: a marker past an `exit`, and `awk 'NR==FNR'` against an EMPTY first file, which made every line hit `next` and **silently emptied the guard runner** -- then my probe commits committed the empty file, so `git checkout --` restored the emptiness. Both produced plausible output.
- **I ACCEPTED A PEER'S ACCOUNT OF MY OWN HISTORY WITHOUT CHECKING MY OWN SENT RECORD.** ic said a file was unannounced, vc repeated it, and I answered _"I have no defence"_ -- while a 14:30Z announce of that exact file sat in four inboxes. **ic's own rule, failed by me in the one direction that costs nothing: a confession closes an inquiry, which is what it is for.** Audit a self-accusation as hard as a self-serving one. **The substantive half stood -- the two highest-stakes files WERE unannounced.**
- **A RULE APPLIED TO THE LANE IT NAMES INSTEAD OF TO THE PROPERTY THAT MAKES IT MATTER.** My own standing decision reads _announce to cc before touching `bin/`_. **`.githooks/` is not `bin/`, so I did not announce -- and the reason for the rule is STRONGER there, not weaker.** A lane is a proxy for a property; when the two come apart, the property governs. ic caught it by naming two files they did not recognise rather than sweeping them.
- **`.githooks/pre-commit` IS THE ONE FILE WHERE AN UNCOMMITTED EDIT EXECUTES FOR EVERY PEER IMMEDIATELY** (ic, 2026-08-21). `core.hooksPath` points at the **WORKTREE**. For `surface/dispatch-table.json` -- where the announce rule was minted -- a peer's dirty copy affects nobody until they read it. **Here, every node's next commit runs yours, with nobody opting in.** ic's `6edbd24f` and two of cc's went through my uncommitted edit before anyone knew it existed. Not "someone might read a stale file" but **"everyone executes yours"**.
- **A FILE LIST HAS A TIMESTAMP AND DECAYS** (ic's correction, in my favour). Their "neither mine nor cc's" sweep named two files at ~15:53Z; my third was modified at 15:54Z. **"The list was short by one" and "the tree grew a file under the list" have different remedies, and only the second was true.** In a four-writer checkout a `git status` is stale the moment it prints -- which is why the sender announces FILES rather than the reader sweeping for them.
- **A FILTER THAT DROPS THE SUBJECT LINE TURNS A CORRECT REPORT INTO A CONFIDENT WRONG ONE.** `int hooks` names its tree on line 1; I grepped for `gate|WIRED` and **produced three false findings from one true report.** Before believing any tool, check it is answering about the thing you are standing in.
- **A FALSE CONFESSION IS CAUGHT BY NOTHING WE OWN** (vc's, and I was the real cause). Every rule we have guards claims that FAVOUR the claimant. **A claim that inconveniently blames you passes every check, because owning a fault reads as rigour and gets no audit.** Check a self-accusation as hard as a self-serving one.
- **A BARE `git reset` IS A SHARED-INDEX OPERATION** -- no pathspec means every peer's stage. Our rule names `--only` and says nothing about this. **`--only` DOES preserve a peer stage, both directions, driven by vc.**
- **`sync --to-disk <ID>` REWRITES EVERY ATTACHMENT FILE OF THAT THREAD FROM THE STORE.** A whole-file second writer over peers' uncommitted work. **I nearly committed ~130 lines of cc's under my name; a line-count check was the only control.**
- **CROSS-OWNER ATOMICITY IS IMPOSSIBLE WITH `--only`.** A new parity tool needs a roster row; the row lives in a peer's file; path-scoped commits cannot split them. **Expect a briefly-inconsistent HEAD and check for it: the roster reads the runner from the WORKTREE, so a split reads GREEN locally and DISAGREE in a fresh clone.**
- **A BENEFIT RECLASSIFIED AS INCIDENTAL ESCAPES THE RULING THAT GOVERNS IT** (vc's mechanism for how the `.envrc` claim passed hv's explicit decline).
- **THE ROSTER TABLE IS A SINGLE-QUOTED SHELL STRING -- ONE APOSTROPHE BREAKS THE PARSE.** I broke it twice, **the second time inside the sentence warning about apostrophes.** Count them to 0 BEFORE writing.
- **A STATUS FIELD CAN BE NAMED THE OPPOSITE OF ITS VALUE.** `direnv status` prints `Found RC allowed 1` and **1 MEANS NOT-ALLOWED.** I read the word and not the value.
- **`zsh -i -c` DOES NOT FIRE direnv** (hook is on `precmd`). Use `direnv exec .`.
- **AN EXISTENCE TEST BEFORE A PATH TEST.** `canon_commit_check.sh` lives under **ST0056**'s tools dir while covering an **ST0057** row.
- **A GREEN IS ONLY ABOUT THE QUESTION THE INSTRUMENT ASKS.** A cross-check reconciling because both sides share an error; a true measurement of a different property offered as proof; **a zero from an instrument never shown able to produce a non-zero** -- my first red-arm candidate passed under BOTH values and proved nothing.
- **CONSENSUS IS NOT CORROBORATION WHEN THE NODES SHARE A METHOD.** Four self-reports off four boards is evidence; four readings of one instrument is not.
- **DATE-ONLY GRANULARITY CANNOT SEQUENCE TWO SAME-DAY hv RULINGS** in opposite directions on one item. Attribution is solved; ORDERING is not.
- **STANDING CONSTRAINTS.** `git commit --only <paths>`, **per-file and never a directory pathspec** (a directory sweeps peers' inbox writes to you). Push `local` only; confirm with hv before `upstream`. NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. **The markdown formatter is a second writer.** **Run the suite through `tests/run_tests.sh`, never `bats` directly** -- though `test_helper.bash:93` is fixed at `ecea0eeb`, so a direct single-file run is now safe.
- **FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED.** `bin/intent_st:1187`/`:1208` and `bin/intent_treeindex:220` are **intentional word-splitting**; `bin/intent_st:1353` is a fragment of a multi-line `sed`. **A sweep driven to zero breaks three live paths.**

- **A TIMING FIGURE I PUBLISHED WAS MY OWN HARNESS, AND THE ROW IT SAT IN EXISTS TO SAY FIGURES MUST BE COMPARABLE.** My roster row claimed `provenance_fields_check.sh` at **82-88ms**. Re-timing it at `b4918a35` for the commit: naive harness **121-150ms**, clean harness **65-74ms (median 67, 7 runs)**, bare `python3` spawn **23-35ms**. My loop spawned **two** python3 processes per iteration just to read the clock, so **~50ms of every figure I quoted was the instrument measuring itself.** A TRUE measurement of script-plus-harness, offered as the size of the script.

- **AND THE HARM IS RELATIVE WHERE THE NOISE IS ABSOLUTE, WHICH IS WHY THIS IS NOT A ROUNDING ERROR.** 50ms is **35%** of a 140ms instrument and **1.7%** of a 2900ms one. **A shared harness bug silently COMPRESSES the ratio between a cheap instrument and an expensive one** -- and that ratio is the only thing the roster's numbers are for. Flagged `thread_view_skew_check.sh` (130-150ms) and `view_skew_check.sh` (2860-2940ms) to cc as possibly carrying the same defect; **I did not re-measure them, because they are cc's rows and a corrected figure I cannot source is the same defect in the other direction.**

- **THE FIX IS TO NAME THE HARNESS, NOT TO REPLACE THE NUMBER.** The row now carries the method beside the figure and **withdraws the 82-88ms explicitly**. A silently-corrected figure is indistinguishable from one that was always right -- and the estate's whole complaint about bare figures is that you cannot tell what produced them.

- **I READ THE SUMMARY AND NOT THE HUNK, AND NAMED THE WRONG AUTHOR (cc corrected me, 2026-08-21 16:56Z, their stamp).** `git diff --stat` said one line in ST0056 canon; I let _one line_ stand in for _which line_ and concluded cc was closing WP-03. It was `wps[seq=3].status` and the author was vc. **The discriminator was two characters of context away** -- and I then drove it myself rather than taking cc's word: symmetric `wps[seq=3]` across BOTH threads, canon and view together, which **no single claim-holder spans**. That is what made the attribution certain rather than probable.

- **ANNOUNCE A WRITE TO A SHARED FILE TO EVERYONE; ANNOUNCE A WRITE TO A CLAIM TO THE CLAIM-HOLDER. CANON IS THE FIRST (cc, 2026-08-21 16:56Z; recorded on hv's board by vc as cc's, with my episode as the measurement).** vc announced the `wp done` to cc, correctly under the rule agreed an hour earlier, **and the rule picked the wrong recipient for a principled reason: I hold no claim on ST0056 at all and I am the only node it stopped.** **Claims predict who CARES, not who is BLOCKED.**

- **AN UNBLOCK REQUEST FROM THE BLOCKED PARTY IS PRESSURE, NEVER AUTHORITY (vc held this line against me, 2026-08-21 17:00Z, their stamp; ic drew it on vc at ~15:5xZ).** cc and I both asked vc to land, both with real cost behind us. vc refused pending hv and was right: **a rule that yields when somebody needs it broken only ever holds when nobody does.** Recording it as a line held against ME, not as one I observed.

- **ONE FILE, TWO OWNERS, AND `--only` IS PATH-SCOPED, NOT HUNK-SCOPED.** Canon is a single JSON per thread, so a peer's one uncommitted line in it blocks every attachment commit in that thread with no split available. **The reach-around -- write canon without their hunk, commit, restore -- is two writes over a peer's uncommitted work**, and ic measured an uncommitted `surface/` edit vanishing in this checkout today with the cause still unexplained. **Held rather than reached around, and vc backed the refusal at their own cost.**

- **A SPLICE VIA `sed ... > tmp && mv tmp file` DROPS THE EXEC BIT, AND GIT RECORDS MODE.** `runner_roster_check.sh` went 100755 -> 100644 inside `19d77f61`, a commit whose diff is entirely about content. **Driven both ways rather than assumed:** python `open(w)` truncates in place and PRESERVES the mode, so the drop happened in a pre-compact splice and not in this session's edits. Fixed at `0dea9abb`, content byte-identical, 0 insertions / 0 deletions, sha still matching canon, **so nothing needed re-syncing.**

- **THE MODE RULE IS INODE-PRESERVING VERSUS INODE-REPLACING, AND IT IS NOT "ANYTHING THAT REWRITES THE FILE WHOLESALE".** Six idioms driven on a fresh 755 file, umask 022:

```
python io.open(p,'w')        -rwxr-xr-x   PRESERVES      pathlib write_text     -rwxr-xr-x   PRESERVES
fileinput inplace            -rwxr-xr-x   PRESERVES      sed > tmp && mv        -rw-r--r--   DROPS 644
open(tmp) + os.replace       -rw-r--r--   DROPS 644      mkstemp + os.replace   -rw-------   DROPS 600
```

**All three preserving forms rewrite the ENTIRE file** -- they truncate in place, so the mode rides the surviving inode. All three dropping forms create a NEW file at the umask (or `mkstemp`'s 600) and rename over it: **`mv`/`os.replace` carries the inode AND its mode, and never consults the original.** **The mode survives when the inode survives** -- a rule a check can be written against, where "wholesale" would flag my provably-innocent python edits and clear a one-line `sed > tmp && mv`.

- **MY FIRST EXEC-BIT AUDIT WAS WRONG IN ITS POPULATION AND PRINTED THE TELL UNDERNEATH ITSELF.** It globbed the **WORKTREE** rather than reading the index -- **the exact defect my own comment in `canon_commit_check.sh` documents at length** -- and its denominator line, `git ls-tree -r HEAD --name-only 'intent/st/*/parity/tools/'`, matched nothing and printed **`0`**, because `ls-tree -r` takes a path PREFIX and not a glob. **I printed a total of 0 directly beneath a clean verdict, reported the verdict to hv, and moved past the zero.** A vacuous denominator beside a green is the arm `canon_commit_check.sh` was BUILT to refuse and that its first draft failed. **I examined 46 files and said "every parity tool". The population is 284.**

- **`read -r mode _h _s path` DESTROYED `$PATH` MID-SWEEP.** zsh binds `$path` to `$PATH` as an array, so the next line was `command not found: tr`, `awk`, `wc`, `sort`. **This is `lamplight-ic`'s clobber arriving for real in an Intent session, four hours after `intent-ic` correctly refused to fold it as their own.** ic was right to refuse and right to keep the mechanism: **it has now genuinely happened here, and the record can say so honestly precisely because ic did not claim it first.** A near-miss on the shared `read` idiom -- **`f`, `n`, `p` are safe; `path` is not, and neither is `status`.**

- **THE CORRECTED SWEEP, 284 FILES FROM THE INDEX, EVERY EXTENSION: NO LIVE INSTANCE.** 4 shebang-at-644 (the documented `lib_*.sh`), 0 files at 755 without a shebang, **0 index-vs-disk mode disagreements** -- cc's arm D, the gap a rename over an already-staged file leaves. **Repo-wide: 272 tracked shebang files, 134 at 644, all verified legitimately sourced or interpreter-invoked** -- 108 `.bats` run BY `bats`, the sourced `lib/` families, and the two ambiguous ones driven rather than assumed (`pre-commit-guards.sh` is `bash "$GUARD_RUNNER"` at `pre-commit.sh:277`; `intent_migrations` is `source`d at `intent_upgrade:20`).

- **SO cc's CHECK D LANDS ON A CLEAN ESTATE, WHICH IS THE BEST TIME TO INSTALL A CHECK AND THE WORST TIME TO BE BELIEVED.** Both of today's drops were caught by hand within hours, so the check has no live catch to point at -- **and the temptation is to read that as it not being needed.** The 22-run tail is the same coin the other way up: **absence of a second observation is not absence of the thing.**

- **AND THE OLD `ESTATE AUDIT` LINE BELOW IS THE ONE I NEARLY LEFT STANDING.** Same conclusion, wrong population, and **a right answer from a broken instrument is the hardest kind to withdraw** -- nothing about the conclusion prompts the re-check. cc asking for the sweep is what re-ran it; **I would not have re-run it on my own, because it had already agreed with me.**

- **ESTATE AUDIT (SUPERSEDED -- 46 files from a worktree glob, retained because the correction is the point).** Every `intent/st/*/parity/tools/*.sh` at HEAD: **the only 644 files carrying a shebang are the four `lib_*.sh`**, which the roster documents as _sourced, not executed; ships 644 and defines functions only_ -- intentional. **No 755 file lacks a shebang.** So both of today's drops were caught and repaired and nothing else has drifted. **A shebang is NOT the discriminator** (the lib files have one and are sourced), which is why the roster's prose classification is doing real work here and why a check needs the roster's disposition rather than a file property.

- **AND IT IS THE SECOND OCCURRENCE IN THAT FILE TODAY, NOT THE FIRST -- cc's WAS THE FIRST AND cc VOLUNTEERED IT (2026-08-21 17:16Z, their stamp).** Driven from the mode history rather than taken: `d8dd6dc6` 100644 (cc) -> `f6face5f` 100755 _restore the roster's executable bit_ -> `f9992662` 100755 -> `19d77f61` 100644 (mine) -> `0dea9abb` 100755. **Twice in one day in one file, two authors, and the repair commit in between did not stop the recurrence** -- so this is a property of how the file gets edited, not of who edited it. **I have NOT diagnosed cc's occurrence and I am not assuming it shares my mechanism**; I made exactly that generalisation about their timing rows an hour ago and it was wrong.

- **AND NOTHING BROKE, WHICH IS THE REASON IT IS WORTH A COMMIT RATHER THAN A SHRUG.** The runner invokes it as `bash "$TOOLS/runner_roster_check.sh"` at `cmd/precommit:289`, so the gate ran green at 644 and would have stayed green indefinitely. **What regressed was MEANING: four rows in that very file read _"sourced, not executed; ships 644 and defines functions only"_, so at 644 the roster classified ITSELF, by its own convention, as a library.** A file whose own vocabulary treats a mode as evidence can be wrong about itself without any instrument objecting.

- **THE TELL WAS ONE LINE UNDER SEVERAL SCREENS OF THE GATE'S OWN GREEN OUTPUT.** `mode change 100755 => 100644`, last line of the commit summary. **Reading the full output rather than the verdict is what caught it** -- the same discipline as confirming a target appears in cargo's `Running` list, and the opposite of the `--stat`/hunk error I made an hour earlier.

- **MY OWN CORRECTED FIGURE WAS INCOMPLETE IN THE SAME WAY AS THE FIGURE IT CORRECTED.** I withdrew 82-88ms as harness noise and published **65-74ms**; twenty minutes later the same tool measured **53-62ms**, nothing about it changed. I had named the harness and not the MACHINE STATE. **The row now states a range across sittings, because a point was never available** -- load average 28 over 16 cores, 21 claude processes, 6 rustc live, harness floor 4.9ms. **A correction is a measurement and inherits every duty of one.**

- **THE LOAD AXIS IS cc's AND MY HYPOTHESIS ABOUT THEIR ROWS WAS WRONG (cc, 2026-08-21 17:03Z, their stamp and their measurement).** I put it to them that their two figures might carry my 50ms artefact. **They RAN it rather than accepting it**, and their harness floor is 5ms, not 50 -- with both figures returning HIGHER than recorded. **An inflation artefact makes a clean re-measurement LOWER, so the direction alone excludes it.** One sign, no audit. **True of my row only.** My contribution was going and measuring the condition after cc named it; my floor came in at 4.9 against their 5.2, and **two independently-built harnesses agreeing on a floor is the one agreement in today's timing work worth anything, because the floor is a property of the machine rather than of either tool.**

- **I BUILT A DECISION RULE ON ONE SAMPLE, cc AGREED WITH IT, AND THE AGREEMENT IS WHAT STOPPED EITHER OF US RE-RUNNING IT. IT DOES NOT REPRODUCE.** I read cc's `160 163 165 167 170 188 513` as a tight mode plus a 3x tail and argued that **a gate runs ONCE, so it pays the tail and never the median** -- therefore a promote-to-gated decision made on a median is made on a number the gate never experiences. cc called it better than their own spread reading and handed it back to me twice. **Committed to this board at `3f16dd52` as a finding that changes decisions. It is withdrawn.**

- **cc RE-RAN IT AT HIGHER LOAD AND THERE IS NO TAIL (cc, 2026-08-21 17:16Z, their stamp and their measurement).** Fifteen runs at `0dea9abb`, loadavg **32.4** over 16 cores against the 28 I measured, floor 5.7ms: `142 144 147 148 150 152 152 153 154 154 155 159 160 174 175` -- **mode ~151, max/mode 1.2x.** `view_skew_check.sh` likewise 1.2x. **Across 22 runs the 513 has occurred once and has not recurred at either load level.**

- **THE SHAPE IS TODAY'S SHAPE ARRIVING INSIDE THE CONCLUSION WE AGREED ON.** A true observation -- the 513 happened, cc measured it -- **generalised into a property of the distribution**, then ratified by two nodes who both liked the reasoning. **That is this morning's four-nodes-one-instrument with the population reduced to two and the instrument reduced to a single sample.** Mutual agreement did not add evidence; **it removed the impulse to go and get some.**

- **WHAT SURVIVES IS RANGE-ACROSS-SITTINGS; WHAT DIES IS THE DECISION RULE.** My own figure moved 15% in twenty minutes; cc's recorded 130-150 sits just under a 142-175 measured now; cc's 7-run median of 167 was pulled up by the single outlier. **All three are the same tool.** So rows state a range with harness, floor, load and run count -- **and a tail is reported as OBSERVED ONCE IN 22 RUNS, never as a property.** **An unreproduced number must not stand in front of a gating decision**, which is the exact thing the row exists to prevent.

- **CHEAP CONTEXT IS WHAT SEPARATES A SUMMARY FROM A CLAIM (cc, 2026-08-21 17:03Z, their phrasing).** `--stat` said one line in a thread canon and I let _one line_ stand for _which line_; the same shape as reading a `test result:` count out of a `tail`. **The discriminator is two characters of context away in both.**

## Decisions

- (2026-08-22) **SOUND-BUT-UNNECESSARY AND UNSOUND ARE DIFFERENT VERDICTS, AND ONLY ONE IS FAIR TO THE PROPOSER.** I drove vc's macro shape before discarding it: an exported `macro_rules!` carrying `env!` **expands in the CONSUMING crate and reads that crate's build env even when the defining crate has no `build.rs`.** vc's words: _a real fact about Rust that neither of us knew an hour ago, now established rather than assumed -- and it would have been lost entirely under a bare "no, that does not work"._ **A rejection that does not establish WHY discards the proposer's work along with their proposal.**
- (2026-08-22) **A RULE BROKEN FOUR TIMES BY THE NODE WHO CITES IT TWICE A WEEK IS A RULE WITH NO ENFORCEMENT POINT** (vc's reading, taken over my own). `suite`, `local`, `clone.lib` and `artefact.lib` were built with no `MODULES.md` row while I quoted register-before-you-code at peers. **The evidence is about the rule's SHAPE, not about diligence** -- same class as a diagnostic arm nobody acts on, and as the roster before `runner_roster_check.sh` existed. **The roster got an enforcement point and stopped drifting the same day.**
- (2026-08-22) **AND THE EMIT SIDE HAD TWO HOMES TOO -- `1940fa93`.** vc found the mirror: `intent-cli/src/lib.rs` and `intentd/src/main.rs` each built the marker with a byte-identical `concat!`. **One parser against two formats is the MORE dangerous half**, because `self_provenance` reports per-binary -- a forked literal prints a clean line for one artefact and `carries NO marker` for the other, **on a diagnostic arm that never fails.** **THE PROPOSED FIX RESTED ON A FALSE PREMISE AND I CHECKED IT RATHER THAN BUILDING IT: `intentd` HAS NO `[dependencies]` SECTION AT ALL**, so a shared macro in `intentsvcs` means ADDING a dependency to host a format literal -- which `source_commit.rs` had already refused as _reshaping the crate for the sake of where a marker lives_. **I drove the macro shape anyway before discarding it, because _sound but unnecessary_ and _unsound_ are different verdicts and only one is fair to the proposer**: an exported `macro_rules!` carrying `env!` expands in the CONSUMING crate, defining crate never knowing the value. Sound. Not needed. **The format now ships from `source_commit.rs`, which was ALREADY the one home for the emit side.**
- (2026-08-22) **RESOLVED AT `54fd02d9`: four homes to one, `grep` now returns ZERO inline parse sites.** Three parity tools delegate to `artefact.lib` and HARD STOP at exit 2 if it is unreadable -- **no `source || define-my-own` fallback, which would restore the duplicate at exactly the moment nobody is watching.** vc ruled the lib stays put: **there is no neutral place, and that is the answer rather than a difficulty** -- moving it would have been designing around a scenario nobody has hit, which is the byte-identity trade I refused this morning arriving aimed at me. The header now lists its consumers and says **if this list and `grep` disagree, `grep` is right.**
- (2026-08-22) **A CLAIM OF UNIQUENESS IS A MEASUREMENT AND MUST BE GREPPED, NOT ASSERTED.** `artefact.lib`'s header claimed to be THE ONE EXTRACTION SITE; the marker parser has **FOUR** shell implementations and **I wrote two of them**, the counter-example being a file I had authored the day before. **I then reported _a second copy_ to vc -- an unmeasured figure, published on the same day I told two peers not to publish those.**
- (2026-08-22) **A CLAIM ON A WP IS NOT A CLAIM ON EVERY AC INSIDE IT, AND A CLAIM WHOSE SCOPE IS UNSTATED COSTS A PEER A MESSAGE TO DISCOVER.** cc asked before building `AC-07.3` because my board claimed `ST0056/07` and vc had ranked that AC as cc's item 1. cc was right to ask and right that both facts were live at once. **`ST0056/07` is scope L with six ACs; my claim covers exactly ONE row -- the hosting sweep needing a driven re-measure through `render.rs:495` -- and nothing else.** The omission was mine: I never wrote the scope anywhere a peer could read it, so the only way to learn it was to ask me.
- (2026-08-22) **THE BUILDER CARRIES THE ROW.** cc offered to build `AC-07.3` under my claim with me carrying it; I declined that half. **A green carried by the node who cannot defend it is this morning's two-wrong-63s with a slower fuse** -- both figures arithmetically correct about a number nobody had driven. cc builds it, cc commits it, cc carries it.
- (2026-08-22) **A CRITERION CAN UNDERDETERMINE A DEFECT RATHER THAN CONFLICT WITH IT, AND THAT KIND CLOSES GREEN.** `AC-07.3` says _reproduce v2 SHA256-manifest behaviour_. cc's second measured defect -- **sync NEVER PRUNES** (`intent_claude_skills:69`, `cp -r source/* target/`, nothing clears the target; `plugin_remove_target` fires only on uninstall/rename) -- **is not a manifest-scope defect at all**, so a v3 satisfying the AC to the letter may prune or not prune and conform either way. Routed to cc to put to vc alongside the known-trap fork; **recorded here so it survives if it is dropped in transit.**

- (2026-08-21) **hv ACCEPTED AC-01.5's WEDGE -- "refuse is correct" -- AND AUTHORISED THE COMMIT DIRECTLY IN MY SESSION.** vc put three options (accept / warn-and-continue / hold to cutover) and hv took accept. **vc relayed the acceptance and REFUSED to relay a landing authority, correctly.** A peer relaying that hv accepted a DESIGN is not hv telling you to LAND it -- ic drew that line on vc earlier the same afternoon, so it has now held twice in opposite directions.
- (2026-08-21) **A PEER'S INDEPENDENT CHECK COUNTS ONLY WHEN THE METHOD DIFFERS.** vc confirmed generator-vs-instance by extracting and byte-comparing (1271 bytes) where I had diffed. **Same subject, different instrument, same answer** -- the opposite of four nodes reading one broken instrument this morning and agreeing.
- (2026-08-21) **`bin/` IS dc's LANE** (hv, live channel, this session). Announce to cc before touching it. `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED from `~/Devel/prj/Devbin`** and are not this repo's to edit -- the vendor-down is hv's to time.
- (2026-08-21) **THE GATE COST STANDS AT ~7.3s.** hv chose unconditional dispatch over a path trigger, with cc's 3.6-4.9s figure put explicitly. **Re-time before moving any row on cost grounds and name the revision.**
- (2026-08-21) **hv RELEASED ALL THREE HELD ITEMS; I ACTED ON TWO AND DECLINED ONE ON THE MERITS.** `test_helper.bash:93` landed after re-deriving what the hold asked for; `canon_commit_check.sh` admitted; **`thread_view_skew_check.sh` held, because its condition is still live.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-21) **SYNC SKIPS UNTRACKED BYTES, LOUDLY** (hv). Canon must never name bytes no reader can obtain -- **which is AC-03.6's own subject.** Fix at the source, not the door. **2026-08-23 13:13Z -- PROVENANCE LIMIT, AND IT IS DECISIVE FOR THE OPEN QUESTION ON `hv/wip.md:92`: hv RULED BY SELECTING AMONG FOUR OPTIONS I AUTHORED, and the four are recorded NOWHERE.** Only the words `I put four options` survive, on this board. **So the authority is hv's and the WORDING IS MINE** -- including _fix at the source, not the door_, which is the clause that relocates the control AC-03.6 put at the commit. **The relay to cc went over the live channel at 15:33Z with no inbox entry, so nothing anywhere holds hv's actual words.** vc frames it as: either hv's ruling is narrower than the relay reached cc as, or it knowingly supersedes AC-03.6. **There is a third and only I can see it: if my four options never surfaced the file-ahead direction, hv could not have knowingly superseded anything -- they chose inside a frame I built. I cannot recover the options and will not reconstruct them; a remembered menu would be a fabricated one.** Not mine to rule. Provenance only.
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc). Prevention and refusal are different criteria.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** Absence is decided at the filesystem, once, by the caller that touches it.
- (2026-08-20) **`CARGO_TARGET_DIR` FIXES FREQUENCY, NOT AUTHORSHIP** (cc). Only a CLEAN TREE reaches authorship.
