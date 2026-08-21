---
node: dc
name: DevX Claude
role: worker
session_id: d2fad1a7-ad92-47bc-befb-0f130c964137
heartbeat_at: 2026-08-21 17:22Z
status: paused
focus: "**FOLDED AND PAUSED. Both of my items LANDED and both are vc to verify: AC-01.5 form 1 at `5c7bb80f`, the AC-11.7 SET arm at `19d77f61`. AT-03.4 green, driven.** **`int macos publish` was found SOUND, no fix** -- the gap was one layer over, it pivots on the TAG at release time and the new arm pivots on the RECORD at any time. **FIVE CORRECTIONS OF MINE TODAY AND EVERY ONE IS A SUMMARY STANDING IN FOR THE CLAIM IT SUMMARISES**: --stat not the hunk (wrong author); 82-88ms was my own harness; the corrected 65-74ms was itself incomplete (machine state, not just harness); one outlier read as a distribution, withdrawn 15 minutes after I committed it; and **a clean exec-bit audit that examined 46 files, said 284, and printed its own vacuous denominator as `0` right underneath the green.** **A right answer from a broken instrument prompts no re-check -- cc asking is what re-ran it.** Repo-wide sweep: no live instance, so check D is a check and not a cleanup. **UNOWNED AND LIVE: the 13 not-an-instrument files at 755 are unguarded.**"
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

**LANDED. `19d77f61` (the SET arm + its roster row + canon, together) and `0dea9abb` (an exec bit a splice dropped).** Nothing in flight. No `target/dc` exists, so prune-at-fold is vacuous for me a third day.

**THE BLOCK WAS ONE FILE WITH TWO OWNERS, AND HOLDING WAS RIGHT.** `canon_commit_check.sh` -- the guard I admitted at `5d2b1f0d` -- refused correctly: HEAD canon named my files' OLD bytes, so the index would have named bytes it does not contain, which is AC-03.6's subject. Its remedy is sync-canon-then-commit-together, and `intent/.canon/st/ST0056.json` also carried vc's uncommitted `wp done`. **`--only` is path-scoped, not hunk-scoped, and canon is one JSON per thread, so no split existed.** vc landed at `e5df40f8` on hv's direct word -- _"I really don't care how. I just want this fixed."_ -- **not on my ask and not on cc's.**

**THE REACH-AROUND WAS AVAILABLE THE WHOLE TIME AND COST ME REAL DELAY TO DECLINE.** Write my canon over the worktree without vc's hunk, commit, restore theirs: two writes over a peer's uncommitted work, and **ic measured an uncommitted `surface/` edit vanishing in this checkout today, cause still unexplained.** vc backed the refusal at their own cost and said it should not have cost me the time. **Recording that it did, because a refusal that turns out to be free teaches nothing about the next one.**

**I VERIFIED THE CANON DIFF STRUCTURALLY AND NOT BY EYE.** Flattened both sides to leaf paths and diffed the sets: **exactly 6 changed leaves, all mine** -- `attachments[74]` and `attachments[88]`, `bytes`/`sha256`/`text` each. A 6-line `git diff` would have looked identical and proved less; **line-count agreement is not field identity**, which is precisely the mistake I had made an hour earlier reading a `--stat`.

**AT-03.4 GREEN, DRIVEN AND NOT INFERRED FROM THE COMMIT LANDING.** `cargo test -p intentsvcs --test attachment_drift_detected --no-fail-fast`, rc=0 read from a redirected file and never through a pipe, 6 passed / 0 failed, binary confirmed in the `Running` list. **First attempt named `-p intent-cli` and cargo refused at rc=101** -- worth holding, because a `--no-fail-fast` run that finds no such target reads a lot like one that found nothing wrong.

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
- **WP-07 hosting sweep** needs a driven re-measure through `render.rs:495`; the disposition route is a DEAD END.

## Watch-outs

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
