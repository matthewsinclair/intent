---
node: dc
name: DevX Claude
role: worker
session_id: d2fad1a7-ad92-47bc-befb-0f130c964137
heartbeat_at: 2026-08-21 16:34Z
status: active
focus: "**BOTH OF MY OPEN ITEMS ARE BUILT AND LANDED, AND BOTH ARE vc's TO VERIFY -- I CERTIFY NEITHER.** **AC-01.5 form 1 at `5c7bb80f`**: the chain block refuses a gate it cannot find; red/green in one clone, old block rc=0 and the commit LANDED, new block rc=1 refused, and **the 82-vs-9 line count is the finding** -- the red arm's 82 lines were the parity roster printing `ok:` while four shipped guards were not present at all. **AC-01.5 STAYS RED on arm 1** (fresh clone, hooksPath unset, forbidden commit lands at rc=0) and that is correct. **GUARD RESOLUTION at `6b60367c` + `12ebd47e` + `fff59a09`**: a self-hosted Intent checkout reads its guards from ITSELF; consumers unchanged, proven by control; **vc verified independently with a mutation witness rather than my marker.** **MY OWN DATED ZERO NOW INVERTS AND vc CAUGHT IT** -- `lib/templates/hooks/` is 1-of-7 different ON PURPOSE. **ic CAUGHT THAT MY FIX ARMED `int hooks` TO UNDER-REPORT THE GATE, and the comment justifying the old spelling stated the premise I had just invalidated.**"
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260821/wip.md` -- the afternoon session is appended under its own heading. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **v2 LIVES IN `~/Devel/prj/Intentv2`** (`v2-maintenance` at `fb45e9ea`). **Its gate was ARM C LIVE and I repaired it** -- dispatcher was absent, `gate ABSENT` -> `WIRED`, planted violation now `rc=1 / guards: 4 ran, 0 skipped`.
- **`intent` ON PATH IS v2.19.0. Drive v3 explicitly: `./native/rust/target/debug/intent`.**
- **`.envrc` EXISTS AND IS BLOCKED -- it does NOTHING until hv runs `direnv allow`.** Even then it fires only at an interactive prompt: **every node commits through non-interactive tool calls, so it is inert for us.**
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** It now warns when they differ (`1005ab88`). To ask about another clone, run ITS `bin/int`.

## DOING

**Nothing in flight. AC-01.5 form 1 landed at `5c7bb80f`; the row is vc's to verify.** Working tree carries only peers' work. No `target/dc` exists, so prune-at-fold is vacuous for me a third day.

**NOT PUBLISHING A GATE FIGURE.** cc's clean-room ingest at HEAD is the number all four of us are holding for. Both `63`s published today were arithmetic in opposite directions from the same figure, and my own `64` was a store read over four dirty worktrees.

## TODO

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

### 3. NEW AND MINE, BOTH UNSTARTED (cc, 2026-08-21)

- **THE RELEASE PAIR CORRESPONDS TO NO SINGLE STATE OF THIS REPO.** `intent` carries `dirty-483e65e4`, `intentd` carries `dirty-5819417b` -- **two different trees**, so shipping them together ships a combination never built or tested as a unit. The self-provenance arm is diagnostic by design, so nothing is broken; **the open question is whether `int macos publish` refuses a MISMATCHED PAIR or only an individually-unnameable artefact. I do not know, and I am not answering it from a code comment.**
- **AND IT IS A REACH LIMIT IN MY OWN `provenance_fields_check.sh` (AT-11.7).** That tool asks each record whether it answers IDENTITY and CURRENCY **per artefact**. cc's finding is a property of the **SET** -- two individually well-formed records that cannot both be true of one tree. **A per-record check cannot see it, and mine says nothing about it.** I wrote that tool and called it sound.

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

### 3. ROUTED OUT, NOT MINE TO BUILD

- **`sync` skips untracked bytes, LOUDLY (hv ruled, I put four options).** Handed to cc 15:33Z -- **`sync` is Rust and Rust is cc's lane.** Warning must stop saying _the commit gate will_ refuse, and must not fire on the syncing node's OWN new file (staged-but-untracked and untracked-and-unstaged are different states).
- **AC-03.6 is cc's row to green.** Blocker cleared by `5d2b1f0d`; evidence handed over 15:26Z. **I will not certify my own wiring.**

### 4. MINE AND UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` has a green to reach. **TRAP FOUND, NOT YET WALKED INTO: `codesign --force` REWRITES THE BINARY IN PLACE, so a hash taken at STAGE time never matches shipped bytes** (`cmd/macos:882-895`); nothing may hash until `verify_notarised` passes. **And `:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one, or `publish` breaks.**
- **`thread_view_skew_check.sh` admission -- STILL HELD, AND I DECLINED hv's RELEASE ON THE MERITS.** Re-derived: release binary `2026-08-20 15:01` vs `migrate.rs`/`facade.rs` `2026-08-20 17:57`, **2h56m stale and unchanged.** The staleness refusal it is conditional on does not exist. Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6 BLOCKED** on the contract conflict routed to vc. The live unattributable pair is still in the gate output every commit.
- **WP-07 hosting sweep** needs a driven re-measure through `render.rs:495`; the disposition route is a DEAD END.

## Watch-outs

**Today's instances are verbatim in `.history/20260821/wip.md`. These are the CLASSES.**

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

## Decisions

- (2026-08-21) **hv ACCEPTED AC-01.5's WEDGE -- "refuse is correct" -- AND AUTHORISED THE COMMIT DIRECTLY IN MY SESSION.** vc put three options (accept / warn-and-continue / hold to cutover) and hv took accept. **vc relayed the acceptance and REFUSED to relay a landing authority, correctly.** A peer relaying that hv accepted a DESIGN is not hv telling you to LAND it -- ic drew that line on vc earlier the same afternoon, so it has now held twice in opposite directions.
- (2026-08-21) **A PEER'S INDEPENDENT CHECK COUNTS ONLY WHEN THE METHOD DIFFERS.** vc confirmed generator-vs-instance by extracting and byte-comparing (1271 bytes) where I had diffed. **Same subject, different instrument, same answer** -- the opposite of four nodes reading one broken instrument this morning and agreeing.
- (2026-08-21) **`bin/` IS dc's LANE** (hv, live channel, this session). Announce to cc before touching it. `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED from `~/Devel/prj/Devbin`** and are not this repo's to edit -- the vendor-down is hv's to time.
- (2026-08-21) **THE GATE COST STANDS AT ~7.3s.** hv chose unconditional dispatch over a path trigger, with cc's 3.6-4.9s figure put explicitly. **Re-time before moving any row on cost grounds and name the revision.**
- (2026-08-21) **hv RELEASED ALL THREE HELD ITEMS; I ACTED ON TWO AND DECLINED ONE ON THE MERITS.** `test_helper.bash:93` landed after re-deriving what the hold asked for; `canon_commit_check.sh` admitted; **`thread_view_skew_check.sh` held, because its condition is still live.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-21) **SYNC SKIPS UNTRACKED BYTES, LOUDLY** (hv). Canon must never name bytes no reader can obtain -- **which is AC-03.6's own subject.** Fix at the source, not the door.
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc). Prevention and refusal are different criteria.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** Absence is decided at the filesystem, once, by the caller that touches it.
- (2026-08-20) **`CARGO_TARGET_DIR` FIXES FREQUENCY, NOT AUTHORSHIP** (cc). Only a CLEAN TREE reaches authorship.
