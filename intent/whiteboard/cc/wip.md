---
node: cc
name: Control Claude
role: control
session_id: ef9e17d5-a705-4547-b749-807a30ba16b5
heartbeat_at: 2026-08-21 13:39Z
status: paused
focus: "**FOLDED AND HELD, EOD 2026-08-21. Built nothing today** -- pickup, measurement, a plan tabled to matts, then hv held everyone. **THE ONE THING A COLD SESSION MUST NOT MISREAD: AC-03.14 test file is GREEN and the row is CORRECTLY RED**, because the green is a `UNPROVEN.len() <= 32` ratchet and not a closure; the fix it prescribes is already in at `write_set.rs:154-166`. **Gate 62 of 67 in THREE calls -- `ac status ST0056/03` is a WP-scoped form nothing had written down -- and it is ST0057 CLOSURE, not the 3.0.0 release.** **TWO CLAIMS OF MINE EXPIRED WITHIN THE DAY AND BOTH ARE CORRECTED BELOW: my INTENT_HOME staleness (this session is FINE) and three-of-four-bounced (ZERO bounced -- ListAgents started is SOCKET age).** Three hv rulings are TODO 1-3, unstarted."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**2026-08-20 is in `.history/20260820/`. Today's pre-fold board and the six vc inbox entries are in `.history/20260821/`. This file is the cold-session minimum.**

## The model -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** **`intentdb` IS RETIRED -- IT NAMES NO COMPONENT** (hv, 2026-08-21, corpus-wide at `513642e7`). `intent-cli` and `intentd` are BOTH clients of `intentsvcs`, which solely owns the SQLite db; the word implied a daemon-owned store. Diagram at `design.md:12-17`. **The SUBSTANCE of D01 is unchanged -- only the term was wrong**, and it was adopted from hv's own phrasing inside two quoted 2026-08-15 rulings, which is why nobody challenged it for six days. **D34: the committed extract is the interchange -- it TRAVELS, the db never does.** D29: a gitignored path is never canon.

**D42 -- TIME.** `date -u +'%Y-%m-%d %H:%MZ'`, own step, trailing `Z`. **The mechanism, since care is not one: the stamp enters the file from the same shell command that reads it.** **THE GENERATOR IS ARITHMETIC, NOT MEMORY** -- a drifted stamp starts from a TRUE reading and is advanced by feel, so it wears the authority of the real one, **and increments-by-feel are monotonic BY CONSTRUCTION, which is why they defeat check C.** A second clock read is the only thing that catches it: **the read is per-stamp, never per-session.**

## Mine -- state at the hold

**I BUILT NOTHING TODAY.** Pickup, measurement, a plan tabled to matts; hv's hold landed before step 1. **No canon written, no test touched, no production code touched.** Everything below is measurement.

**PROVENANCE OF THE NUMBERS: driven at `706db8ee`, clean tree. HEAD IS NOW `5fb091cb` AND NOTHING BELOW IS RE-VERIFIED THERE** -- six peer commits landed while I planned.

### The gate is 62 of 67, and the verb form that yields it was undocumented

**Three calls, no arithmetic:** `ac status ST0057` -> 47/51, `ac status ST0056/03` -> **15/16**, and 47+15 = 62 of 51+16 = 67. **`ac status <STID>/<WP>` IS ACCEPTED BY THE VERB AND NOTHING IN THIS ESTATE HAD WRITTEN IT DOWN** (vc, 10:26Z; driven here at rc=0). **I reached 62 before reading that, by grepping `ac list ST0056` and counting the group-03 rows -- the right answer by exactly the hand-tally the instruction forbids.** The instruction named `ac status ST0056`, which answers 59/132 for the WHOLE THREAD and has no path to 67, **so the guard against hand-tallying was the vector for it.**

**AND THE SCOPE IS NOT WHAT I REPORTED TO matts.** 62 of 67 is **ST0057's CLOSURE gate**, not the 3.0.0 release. The release is **ST0056 WP-12**, dependent on all prior WPs; ST0056 is **59/132 with seven WPs Not Started** (08, 09, 12, 13, 14, 15, 16). **Read as release progress it says 93% where ST0056 is at 45%.** I told matts "five rows from v3.0.0" in this morning's plan and that was wrong.

### AC-03.14 -- THE PRODUCTION FIX IS IN AND THE ROW IS CORRECTLY RED

**READ THIS BEFORE TOUCHING THE ROW.** `write_set.rs:154-166` already carries the skip -- `record()` has read the file, so the comparison costs no extra I/O, and `written: false` is the existing nothing-to-undo semantics. **The fix the row's own text prescribes is BUILT.**

**And the test file is GREEN while the row is RED, and both are correct.** Driven at `706db8ee`, `CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc`, target present in the `Running` list, `test result: ok. 3 passed; 0 failed`, recorded `rc=0` read from the file and not from a pipe. **The green is the `UNPROVEN.len() <= 32` RATCHET -- it stops the debt growing and says nothing about closure.** A cold session reading "test green" and moving this row would be wrong.

**What is actually left is the SECOND INSTRUMENT**: a CLI-level driver that runs the binary and snapshots the estate `(path -> bytes, mtime)` around each verb, for the ten thread-estate verbs still in `UNPROVEN`. 31 of the 32 have no facade method, so this file cannot reach them. **Constraints already ruled, do not re-litigate:** `ingest` is cleared by vc's clean-room run; `at lint`'s mutate lives in `--fix` and the bare form proves nothing; `todo list` is a table defect, not a discharged verb; **store materialisation cannot be the observable, because read verbs create `.cache/intent.db` too**; and per ic, `cargo test` returns 101 for a build failure and a test failure alike, so **zero `test result:` lines must read UNMEASURED, never green.**

### AC-10.8 -- step 1 is half driven, and the remaining half decides whether a test gets written at all

**DRIVEN:** `grep -i 'out-of-model' data-model.md` returns ZERO. The enumerating clause is **`## What is deliberately not modelled` at `data-model.md:464`**, and it names three things: prose (FTS-indexed, so IN the store), shipped rules/skills/templates (embedded in the binary), and `wip.md`/`restart.md`.

**DRIVEN:** 876 tracked files under `intent/`; **240 of them under `intent/st/ST0056/parity/`** as `.tap`, `.sh` and `.exs`. ST0056 canon records 88 attachments and every one is `.md`.

**INFERRED, NOT DRIVEN -- THIS IS THE STEP THAT IS LEFT:** that those 240 are absent from the store. **The hypothesis is that the clause UNDER-DESCRIBES the out-of-model set** -- a `.tap` output is not prose, not shipped content, and not `wip.md` -- which lands on hv's standing question about 250 files under `intent/` not being in the store at all. **Next move is a store-side query, not a test.** If the hypothesis holds, **the deliverable is a canon amendment routed to vc and there is no test today**, because a test written around a clause known to be wrong is a vacuous green over a bad population.

### THE DEAD END, recorded so the next session does not re-walk it

**My first grep for `out-of-model` returned zero and I was one step from filing an EXPIRED CITATION against AC-10.8.** The clause exists, under different words. **A name search returns a fact about the search** -- dc's class, fresh instance, in the EXCLUDE direction. The cheap habit that saved it: when a subject grep comes back empty on a document that ought to contain the subject, **read the headings before concluding the subject is absent.**

### AC-01.5's remedy is ruled dc's and sits on NOBODY's board -- AND I DID NOT ROUTE IT

`AT-01.5`'s note reads **"THE REMEDY IS dc's, IT IS SMALL"** -- the bare `[ -x ]` with no else in `.githooks/pre-commit`, or `int hooks --install` refusing to report a wired clone whose dispatcher is absent. **Driven: `grep -cE 'canon-ignore|pre-commit\.intent|AC-01\.5' intent/whiteboard/dc/wip.md` -> 0.** It was step 1 of this morning's plan and the hold arrived first, **so it is still unrouted and that is mine to carry, not dc's to have missed.** A ruling with no owner's TODO row is this estate's own class: the write succeeded and the delivery never happened.

### Two restart-assigned items of mine were already green

ST0056 **AC-10.4** and **AC-10.2** both read `satisfied: yes`. `intent/restart.md` carried them as work. WP-10 is 11/15; the four outstanding are AC-10.5 (matts' call), AC-10.6 (`n-a`, wants a ruling not a build), AC-10.8, AC-10.12 (**held on the unexplained trim asymmetry -- building before it is explained encodes a guess inside a green**).

## TODO -- three hv rulings landed today, in queue order

**All three arrived via vc's inbox, attributed to hv's live channel and written as standing directives on `hv/wip.md`. READ THEM THERE, not from vc, and not from this board.** vc verifies on close. **None of them displaces AC-01.5, AC-03.6 or AC-03.14 unless hv resequences.**

1. **ROUTE AC-01.5 TO dc.** Carried over from this morning, unstarted. XS, and it blocks a gate row.
2. **DECLARE THE GATE'S ROW SET IN CANON AND HAVE A VERB READ IT** (hv ~11:35Z). **Not a new mechanism -- ST0057 AC-00.1 already carries `<<PRECONDITIONS AC-00.2 ... AC-07.6 PRECONDITIONS>>`, 14 ids on one line, and the dehydration ship gate reads that list rather than reimplementing satisfaction. The release gate is that shape one level up, over two threads.** Three checks vc wants to be able to make: **the denominator comes from the declaration and never a hand-typed constant**; **a withdrawn row leaves the denominator by the same rule in BOTH halves** (vc's 57-of-67 was wrong precisely because ST0057 excluded withdrawn and ST0056 counted one); and `ac status ST0056` is the WRONG denominator -- the WP-scoped form is what yields 16. **THE DECLARATION MUST NAME WHICH GATE IT DECLARES** (vc's 12:16Z correction): there are at least two real ones -- ST0057 closure and the WP-12 release -- and a person could not keep them apart in prose for one morning.
3. **WIDEN `runner_roster_check.sh`'s POPULATION TO EVERY PARITY INSTRUMENT** (hv ~11:52Z). Population becomes every instrument under `intent/st/*/parity/tools/` regardless of filename; each declares `gated` or `manual` with a required non-empty reason. **The guard is `gated`, runs every commit, returns clean, and its population is bounded twice** -- `cmd/precommit:116` pins `TOOLS` to ST0056's directory and `runner_roster_check.sh:180` greps `[A-Za-z0-9_]*_check\.sh`. **ST0057's entire toolset is outside BOTH bounds** (wrong directory; five of seven not named `*_check.sh`), plus ST0056's `rig_selftest.sh` and `of_n_labels_its_derivation.sh`. **These are not `manual` -- they are UNDECLARED, invisible to the instrument that adjudicates the question.** **A filename convention doing a population's job is the whole defect: an instrument is in scope for what it IS, not what it is called.** `no_daemon_required.sh` is a genuine `manual` with a real reason -- `pgrep -f 'intentd'` matches `intentdb` in every node's system prompt, so gating it would ship a permanently-refusing gate; **retiring the word does NOT fix the needle, and letting the first look like it closed the second is the trap.**

**Also mine, filed not fixed:**

- **`AT-00.6` IS STALE: `to-write` while `native/rust/crates/intentsvcs/tests/migrate_v2_project.rs` EXISTS** (11138 bytes, verified present). **A built instrument recorded as unwritten understates the estate in the one direction nobody audits.**
- **Two `intentdb` doc-comment sites are mine:** `intentsvcs/src/lib.rs:11` and `intentsvcs/src/project.rs:786`. **`project.rs:786` is the load-bearing one** -- it documents the field a reader goes to in order to learn what the db is. **Fold into the next edit in those files; do not make a trip.**

## What changed under the tree today -- you will wake up inside this

- **THE v2 CLI HAS LEFT THIS CHECKOUT.** `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` = main HEAD and **NOT the `v2.19.0` tag** -- the old symlink resolved into the working tree, so the fleet had never run the tag, and branching there would have reverted 2027 commits across every project while presenting as a symlink move.
- **`intent` ON PATH IS v2.19.0 AND ANSWERS FOR THE FLEET, NOT FOR THIS TREE.** Drive v3 by explicit path: `./native/rust/target/debug/intent`. **`bin/` is no longer load-bearing for anyone else**, so v2 shell can be pruned here without breaking fifteen projects.
- **THE `INTENT_HOME` STALENESS I RECORDED AT 12:57Z EXPIRED AT 13:36 AND THE CORRECTED STATE IS: THIS SESSION IS FINE.** Driven at 13:2xZ on vc's restart probe, same session: `INTENT_HOME=/Users/matts/Devel/prj/Intentv2`, `intent` -> `Intentv2/bin/intent` (v2.19.0), siblings `intent_st` and `int` -> `Intent/bin` (v3). **That is the correct split and no measurement in this session is suspect.** What I wrote at 12:57Z -- that the shell carried the old value and every PATH-`intent` reading described the old binding -- **was true when written and false within forty minutes**: hv's symlink is stamped 13:36, AFTER my 12:58Z fold, so the binding change completed while I was already paused. **THE LESSON IS NOT ABOUT THIS VARIABLE. A CLAIM WHOSE SUBJECT IS STILL BEING CHANGED BY SOMEONE ELSE HAS A SHELF LIFE, AND A FOLD IS EXACTLY WHERE ONE GETS FROZEN AND READ LATER AS CURRENT.** Stamp the reading, name who else can move the subject, and re-drive before relying on it.
- **BOTH TREES ARE ON PATH AND v3 IS FIRST; THE SYMLINK ONLY EVER PICKED THE ENTRYPOINT** (vc, measured before the switch). `~/.local/bin` at 17 gives v2 for `intent` ALONE; `Intent/bin` at 22 beats `Intentv2/bin` at 23, so **`intent_st`, `intent_critic`, `int` and `devbin` all resolve to the v3 tree.** Harmless today only because the 26 executables are byte-identical and `bin/intent:26` sources every handler out of `INTENT_HOME` regardless -- **the env var picks the CODE, the symlink picks the ENTRYPOINT.** It arms itself the moment v3's `main` diverges. Fix is WP-12's _bin/ (shell) pruned at the cut_, mine, later. **vc tested one binary and concluded about the tree; Lamplight's ic caught it.**
- **THIS REPO'S COMMIT GUARDS NOW RESOLVE OUT OF THE FROZEN v2 CHECKOUT** (`.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`). Identical today; **drifting from the next guard change.** dc holds it as a mechanism -- hv declined direnv and hand-refresh by name.

## The practice -- four shapes of a row promising more than it delivers

1. **UNCITED COVERAGE** -- already satisfied by a test nothing links to it. **PARTIAL uncited coverage is worse: a subject grep that hits the file says nothing about WHICH limbs.**
2. **EXPIRED CITATION** -- the row names a file that cannot cover it. `to-write` is exempt from L2/L3, correctly, **so a citation is unvalidated until someone tries to satisfy it.** `AT-00.6` above is the inverse: built, recorded unwritten.
3. **VACUOUS GREEN** -- true by construction. **The falsifiable arm is over the DECISION, never the OUTCOME.**
4. **TITLE BROADER THAN BODY** -- **and this one leaves no trace at all.** The row is internally consistent: lint passes, the citation is right, the test is green, and it reads as covered to anyone who does not open it. **vc's own 12:16Z gate mislabel is a fresh instance in prose rather than in a row.**

**PRACTICE:** subject-grep FIRST because it is cheap, **then DRIVE THE VERB when it comes back empty.** **And vc's discriminator -- what does satisfying this row COMPLETELY still leave broken? -- is asked against the BODY, never the TITLE**; against a title it returns _nothing_ every time, for exactly the rows where it matters most.

## The class -- A RECORDED REASON RETIRED BY AN UNRELATED CHANGE, WITH NOTHING WATCHING THE JOIN

**SIX INSTANCES IN ONE WEEK, WHICH IS WHAT MAKES IT A CLASS RATHER THAN AN ANECDOTE.** A reason is written down, it is true, it is cited; then a change ELSEWHERE makes it false. **The practice it justified usually stays correct, so nothing looks wrong** -- and the reason keeps being read as current.

1. **AT-03.6's roster reason** -- _no narrow attachment-sync verb ... revisit after ST0057 WP-08_ -- **died at `212b0075`** when `sync --to-store <ID>` landed. Nothing pointed at that row when the verb shipped and nothing could have.
2. **AT-03.6's second blocker** -- _what it needs is a `--staged` MODE, not a call site_ -- **died at `19268867`.** Same row, same week, second expiry.
3. **AT-01.5's two recorded reasons** -- _unmeetable by any edit to the guard, to the roster, or to the template_, and _`pre-commit.intent` here is an install-time COPY_ -- **struck in every clause** by dc's Shape 3 plus `core.hooksPath=.githooks`. **A reader taking them at face value goes at exactly the three places that are now right.**
4. **MINE, 2026-08-21** -- the `INTENT_HOME` staleness above: true at 12:57Z, false by 13:36, frozen into a fold in between.
5. **vc's, same week** -- a rationale that expired while the practice it justified stayed correct, so the correctness of the practice concealed the deadness of the reason.
6. **vc's, and THE WORST SHAPE: `intentdb`.** The term was wrong, it sat inside two quoted hv rulings of 2026-08-15 in `design.md`, and it propagated to all five nodes for six days. **ATTRIBUTION IS WHAT STOPPED ANYONE CHECKING** -- a verbatim quote reads as settled, so the one thing that would have caught it is the one thing nobody does to a quotation.

**THE FINDING IS THE JOIN, NOT ANY INSTANCE: NOTHING IN THIS ESTATE WATCHES IT.** `at lint` exempting `to-write` is CORRECT, so it cannot see this; a citation is unvalidated until someone tries to satisfy it. **Every one of the six surfaced the same way -- a builder picked the reason up in order to USE it.** That is not an instrument, it is a coincidence of scheduling, and it means the undiscovered ones are exactly those nobody has needed yet.

**PRACTICE UNTIL SOMETHING WATCHES IT: RE-DERIVE A RECORDED REASON BEFORE YOU RELY ON IT, AND RE-DERIVE AN ATTRIBUTED ONE HARDEST.** Re-deriving is also how you find the reason was wrong when written rather than merely expired. **A reason carries a DATE and a SUBJECT-OWNER; if someone else can move the subject, the reason has a shelf life and the citation must say so.**

## Watch-outs -- evidence

- **THE PROVENANCE RULE, THREE LIMBS**: neither the INSTANCE, nor the CONTROL, nor the PREDICATE may come from the thing under test. **Derive the expectation from the fixture's own bytes.**
- **A GREEN MEANS NOTHING UNTIL EACH TEST HAS DIED FOR ITS OWN REASON** -- distinct kill-sets. **Assert the mutation APPLIED.** **Table the matrix as a PREDICTION before driving it**; written after, it is a transcript.
- **"THE TREE DID NOT CHANGE" IS ALSO WHAT A BROKEN VERB PRODUCES.** Every atomicity claim needs the control that the same verb on a passing input DOES change the tree -- **asserted as a change, never `is_ok()`.**
- **A POPULATION IS A CLAIM, AND ITS DEFINING CLAUSE IS WHERE THE ANSWER HIDES.** **EMPTY:** ask an empty directory _was this ever non-empty_, answerable from the RECORD not the listing. **EXHAUSTIVE AND STILL WRONG:** _zero of 110 `.bats` files set the override_ was true, complete, and carried a 302-failure finding -- the only thing that ever set it was `run_tests.sh`, **which is not a `.bats` file.** **Read your own qualifier as the hypothesis it is.** One hides behind a NAME, the other behind a CATEGORY, and no spelling sweep reaches the second. **AC-10.8 above is a live third instance.**
- **FIVE LIMBS WANT FIVE TESTS, NOT FIVE ASSERTIONS** -- the first failure masks the rest.
- **WHEN YOU GREEN A ROW, WATCH THE CRITERION COUNT.** If it does not move, the row was not the last one. **A green row and a closed criterion look identical from where the builder stands.**
- **A COMMENT CAN BE NEVER TRUE RATHER THAN STALE, AND IT CAN ALSO BE ENTIRELY CORRECT AND UNREAD.** Both failures are the same act -- not reading the prose beside the code -- **so the remedy is not distrust comments, it is READ THEM AND THEN CHECK.**

## Watch-outs -- instruments

- **A DENOMINATOR CAN BE CORRECT AND STATED AND THE CONCLUSION STILL WIDER THAN IT.** **Stating your denominator does not stop you generalising past it.**
- **`cargo test` HALTS AT THE FIRST FAILING TARGET** -- 46/366 against the real 141/985/2. **The stopped run's denominator looks exactly like a denominator.** `--no-fail-fast`.
- **A GITIGNORED SSOT IS INVISIBLE TO `git status`.** Walk the filesystem when the claim is _nothing was written_.
- **A GREEN COMMIT GATE IS NOT A GREEN TREE.**
- **BEFORE CALLING A RED A PEER'S, RE-RUN IT WITH YOUR OWN FILES REMOVED.** `common/mod.rs` compiles into every test in the crate.
- **`grep -c` EXITS 1 ON ZERO.** **A ZERO FROM A DATA COMMAND IS SILENT; FROM A MISSING FILE, LOUD; FROM A NAME SEARCH, A FACT ABOUT THE SEARCH.** `cargo --manifest-path <abs>` beats `cd`. **The Bash tool's shell is zsh; hooks run bash.**
- **A REVISION NAMES SOURCE, NOT THE BINARY THAT ANSWERED** -- `shasum -a 256` and quote the hash WITH the number. **NAME REVISION, CLOCK AND DIRTY COUNT ON EVERY MEASUREMENT.**
- **MARK PROVENANCE PER CLAIM: DRIVEN, READ, OR INFERRED.** **The cost lands on the READER, which is why the writer never feels it.** **VERIFY THE RETRACTION, NOT JUST THE CLAIM.**
- **A BACKGROUND WAITER'S EXIT CODE IS ITS OWN, NEVER THE WATCHED PROCESS'S VERDICT.** Redirect the run's own rc to a file and read it there.
- **AN UNQUOTED HEREDOC IS A SHELL, AND IT ATE THE ONE PART OF A MESSAGE THAT MADE IT CHECKABLE** (mine, 2026-08-21). I used `<<EOF` rather than `<<'EOF'` to interpolate a clock stamp; the body held a fenced block of `a -> b` mappings, so zsh ran `vc`/`ic`/`dc` as commands and turned three `->` into REDIRECTIONS -- creating three empty files named after session ids, in the repo root. **The prose all landed and only the EVIDENCE TABLE vanished, so the entry read as complete and merely unsupported.** The commit succeeded. **When a heredoc must interpolate, interpolate ONE variable and keep the body quoted -- or write the body with a quoted heredoc and substitute afterwards.** `git status` in the repo root is what surfaced it, not the transcript.
- **`ListAgents` "started" IS SOCKET AGE, NOT SESSION AGE, AND FOUR NODES GOT THE SAME WRONG ANSWER FROM IT ON 2026-08-21.** I read three peers as _started ~5 minutes ago_ and concluded **three of four bounced**. **Zero bounced.** When the topology changed every peer re-registered, so **every node saw the other three as fresh and itself as resumed** -- four correct self-reports and one unanimous wrong inference about the population. **UNANIMITY ACROSS INDEPENDENT NODES IS NOT CORROBORATION WHEN ALL FOUR READ THE SAME INSTRUMENT**; it is one reading counted four times, and it feels like the strongest evidence available. **A self-report is first-hand; a peer's state read off an instrument is not, and the two must never be summed.** I put the wrong figure in vc's inbox at `461ef8e6` before vc corrected it.

## Watch-outs -- four nodes, one checkout

- **PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR**; the working tree is nobody's tree. A peer's red conceals mine and reads as an all-clear.
- **A PEER CAN SWEEP A FILE THEY DO NOT WRITE, AND THE SINGLE-WRITER RULE DOES NOT STOP IT -- IT ONLY SAYS WHO WAS WRONG** (measured on my own fold, 2026-08-21). I wrote `dc/inbox.cc.md` at 12:59Z; **dc's fold commit `ad37745f` carried it**, so my message is in the record wearing dc's authorship and `git status` reported the file clean while I was still holding it. **I am that file's sole WRITER; dc is its sole READER, and dc committed it.** Benign here -- the path encodes the routing and the content is intact -- but the shape is the transcription-laundering class arriving through a COMMIT rather than through a quoted stamp. **The tell is a file you just wrote reporting no diff.** When a write of yours goes quiet, check `git log -- <path>` before re-writing it; re-writing would have produced a duplicate entry attributed to two commits.
- **THE WORKTREE AT A NAMED REVISION IS THE ONLY REMEDY FOR THE `--only` CLASS.** `--only` is path-scoped, not hunk-scoped, **and silently skips untracked files inside a named directory** -- `git add -N` first. **Check the checkout SUCCEEDED: `git checkout --detach` ABORTS on an untracked file in the way.**
- **A LIVE CHANNEL DELIVERS AND LEAVES NO RECORD, AND THE GUARDS RUN AT COMMIT, SO THE LIVE CHANNEL IS UNGUARDED.** **The hazard is TRANSCRIPTION: quote a peer's live stamp into a file and it enters the record laundered through you, past a guard watching the wrong door.** **Attribute it, never assert it.** Every hv stamp on this board is vc's attribution, marked as such, and not a time I read.
- **AN INBOX WRITE IS A RECORD, NEVER A DELIVERY** (vc, twice in one day). **A write surface with no named reader is a queue.** Six entries sat in my inbox from 10:26Z and I read them at 12:5x only because vc sent a live message.
- **`prettier` RUNS INSIDE THE COMMIT WINDOW** and re-stages, **after `sync --to-store` hashed the worktree.** Order is **FORMAT, then SYNC, then COMMIT** -- never "sync last", which reads as sync-after-commit and leaves that commit permanently divergent. **It is one formatter with two triggers, not two formatters.**
- **A TABLE EDIT IS A TWO-FILE COMMIT WHOSE SECOND FILE IS ONE YOU NEVER EDITED** -- after editing any SOURCE, ask what renders FROM it before staging.

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE.** A bare `AC-03.6` is GREEN in ST0056 (FTS prose bodies) and RED in ST0057.
- **HIGHLANDER FORBIDS TWO ANSWERS TO ONE QUESTION; IT DOES NOT REQUIRE ONE ANSWER TO TWO.** **Two mechanisms enforcing different properties are not two copies of one** -- prevention and refusal are different criteria.
- **TWO ROWS RATHER THAN ONE WIDENED ROW** keep two assertions separately falsifiable.
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION.** Everything found building v3 is work.
- **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON** -- and re-deriving the reason is how you find the reason was wrong.
- **A CONTROL THAT DEPENDS ON THE AUTHOR REMEMBERING IS NOT A CONTROL, IT IS A HOPE WITH A FILENAME.**
- **`treeindex` and handover RETIRE**; a retired command is PRESENT AND REFUSING. **`doctor --fix` is WITHDRAWN.**
- **DO NOT PUT v3 ON PATH.** **`config.json` DOES NOT MOVE WITH `intent_dir`.**

## Lane and build recipe

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks`, `canon_commit_check.sh`'s admission = dc. **Canon writes route through vc.**

**`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker dir) and manufactures phantom failures; relative under a drifted cwd builds where gitignore hides it, once at 1.2G. **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.** **Drive v3 as `./native/rust/target/debug/intent`; `intent` on PATH is v2 and answers for the fleet.** **Run the shell suite through `tests/run_tests.sh`, NEVER `bats` directly** -- the runner exports `INTENT_FIXTURE_VERSION` from `VERSION`; a direct run builds a v3 fixture against the v2 binary and dies on the version guard, 300 refusals reading as 300 failures.
