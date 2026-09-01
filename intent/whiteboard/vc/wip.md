---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-09-01 13:48Z
status: active
focus: "REBUILT AND CURRENT -- binaries 13:27Z from 361eff99, daemon up 13:29Z, store user_version 17, gate 110/135. I escalated the OPPOSITE to hv at 13:40Z and it was already false by seven minutes; cc/dc/ic boards still carry it. RE-DRIVE EVERY FIGURE; READ NONE OFF THIS BOARD. TN001 is dc file until they hand back a sha."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS and UNEXECUTED WORK.**

## DOING

### THE REBUILD WINDOW WAS TAKEN AT 13:27Z AND I ESCALATED ITS ABSENCE AT 13:40Z

**hv took it seven minutes before my fold. I wrote _nobody has taken it_ at 13:34Z and put it at the TOP of hv's board at 13:40Z, as the first thing hv reads on restart.** Re-measured on the bounce: both release binaries and `target/debug/intent` rebuilt **13:27Z**, daemon up 13:29Z, store `pragma user_version` = **17**, `revision` column live in the store AND in `schema/ddl.sql` (regenerated at `aa81ee58`, ancestor of the built commit). **CURRENT, not merely present**: built from `361eff99`, and the only commits since are the five fold commits -- **zero files outside `intent/whiteboard/` moved**, so the currency rule this board states is satisfied by derivation, not by the version string. `intent ac gate ST0056` drives: **110/135 satisfied, 25 descoped, 2 withdrawn**.

**THIS IS 3g AND 8b FIRING ON THE ITEM I RANKED FIRST.** Stale in the PESSIMISTIC direction, so it prompted nobody to check -- an overstatement gets caught by whoever relies on it, an understatement recruits the whole board into standing still. **cc, dc and ic all still carry _the tool is broken_ and none of their holds ever depended on it.** **A DOING ENTRY ASSERTING A LIVE BREAKAGE IS RE-MEASURED AT PICKUP BEFORE IT IS REPEATED, AND ALWAYS BEFORE IT IS ESCALATED** -- the measurement cost four commands.

### THE CUT -- RUN THE VERBS, NEVER READ A FIGURE OFF THIS BOARD

`intent ac gate <ST>`; `bash intent/st/ST0056/parity/tools/gen_cut_surface.sh`. **`intent --version` names the commit the binary was built from, which is NOT HEAD -- and that difference ALONE IS NOT STALENESS.** Currency is whether any non-test file under `native/rust` moved since that commit. **Pin by the sha256, never by the marker** -- three distinct binaries have carried one marker in a day, and **two artefacts both answer `3.0.0` with 1516 commits between them**, so _does X ship in 3.0.0_ is unanswerable from the version string.

### THE RELEASE IS `v3.0.1`, FEATURE COMPLETE, AND THE NUMBER QUESTION IS CLOSED

**hv, first-hand, 2026-09-01 08:32Z: everything outstanding goes into `3.0.1`, cost is not a constraint, there is no external consumer. NOT TO BE PUT AGAIN, IN ANY FORM.** I asked three times across three days by re-deriving it from the command delta -- **which is not evidence about the NUMBER and never was.** **THE SCARCITY REGISTER IS RETIRED AS A CLASS**: _ship it red_, _not in this cut_, _after the tag_, _new machinery in a tag window_ all protected a date that does not exist. **Any recommendation of mine resting on _there is not time_ is withdrawn unless re-derived on its merits.** **After striking a class, GREP THE ESTATE FOR THE CLASS'S OWN PHRASES** -- I struck it, carried it to three nodes, and left its clearest live instance on hv's desk two minutes downstream of the ruling.

### `AC-00.16` -- SIDE C LANDED, RED FOR A NEW REASON

The per-property union exists. **What keeps it red is that the criterion's first conjunct is NOT MACHINE-DECIDABLE.** The fix is an amendment and hv's -- **a delegated pen does not cure it, because holding the pen does not change who benefits from my own red row going green.** **AND MINTING THE POPULATION/FORMS/REACH SHAPE MAY MAKE IT DECIDABLE:** the row is red because the set of ways to state a limit is open, so any keyword probe undercounts and **its false positives are precisely the instruments that stated their limit WELL**. Conformance to a MINTED form is greppable where good practice is not. `THE FORM MUST BE MINTED BEFORE THE POPULATION CAN BE` (ic) is the prerequisite that row was missing.

## TODO

1. **`AT-00.11`'s instrument `of_n_closes_over_examined.sh` DOES NOT EXIST -- it is a BUILD and its prerequisite is ANSWERED.** `AC-00.11` cites `c51f10d5` as _the defect live_; **it is the FIX commit**, parent `4ba598f1` drives clean. **NO MAIN-LINE REVISION REPRODUCES IT** -- the fix landed four minutes after the defect, both predating the layout move. **The control is a CONSTRUCTED FIXTURE, already driven to `EXAMINED 5 of 3 ... the other -2`.** Recipe: `4ba598f1`'s `canon_commit_check.sh`; a HEAD whose RECORDED-attachment total is small; then commit more files under the canon roots than that total. **Two traps, both presenting as the defect being ABSENT: `total` reads the REVISION not the worktree, and the narrowing reads `git diff-tree` on the COMMIT not the index.** `AC-00.11`'s citation wants correcting.
2. **`AT-00.2`, `AC-00.8`, `AC-00.10`** -- behind cc's WP-10, behind WP-06 and WP-07. Sequence around it.
3. **`0136`** (hv ruled: after the tag); **`intent/wip.md`** before the tag.
4. **`parity/register.md` -- BLOCKED ON THE RECLASSIFICATION; REGENERATING NOW WOULD LAUNDER ITS STALENESS.** `gen_register.sh` reads the COMMITTED `burn-baseline.tsv`, so regenerating reproduces the SAME 2026-08-15 classifications while refreshing the _Measured at_ line. **Order: reclassify -> UNSTABLE drops -> fresh baseline -> THEN regenerate.** The baseline is also 99 rows against 114 real `.bats`, so `coverage_map.sh` currently REFUSES to publish -- correctly.
5. **`design.md:88`** still says `rmcp (official SDK): stdio ... now`, which my ruling reversed. Design prose is hv's hand -- **a flag, not an edit.**
6. **OWED, DELIBERATELY NOT FILED: the INDEX layer of the three-layer property has neither a row nor a name.** One observed instance, no measurement. **Filing it now would give it a row before anyone drove it.** It earns a row on evidence.
7. **`evidence`-ONLY-ON-SATISFIED: THE OPEN QUESTION IS NOW ANSWERED AND THE FILING IS OWED.** Measured: **100 satisfied WITH evidence, 0 satisfied without, 0 UNSATISFIED with evidence, 33 without** -- the unsatisfied state shape has **no evidence slot at all**, and `ac satisfy --evidence` discharges in the same call, **so a row structurally cannot name WHAT WOULD DISCHARGE IT until it IS discharged.** I held it as a DESIGN question because I could not establish a defect. **baize-vc established it: the same coupling destroyed 31 contract fragments on their port, 28 unrestorable (ST0024 22, ST0010 6), plus three AT rows truncated inside the test name.** That is the cost, measured, on a real estate. **File it.**
8. **RE-DRIVE baize-vc's FINDING A AGAINST OUR HEAD -- IT IS BIGGER THAN ANYTHING ELSE ON THIS BOARD.** Claim: **the acceptance gate CANNOT FAIL.** Falsified in Baize 2026-08-27 with a positive control and a prediction written first -- broke the test `AT-02.1` covers, proved the mutation real through a calibrated seal, and the gate returned `15/15 PASS` with the covered test RED. **Mechanism corroborated HERE from data already in hand: the `at` family is `list/lint/green/red/na/new/edit` -- no verify, no run mode -- and `status: green` is a STORED STRING that stands until a human types `intent at red`.** **NOT re-driven here; their figure is five days old and Intent has moved.** Records at Baize `intent/whiteboard/vc/.history/20260827/session-20.md` and `session-21.md`. **If it holds, every gate figure I have quoted is a claim about what someone typed.**

## Holds

**Each carries the CONDITION that releases it. A hold with no condition is an abandonment.**

- **TN001 IS dc's FILE, NOT MINE.** hv gave them the task; I raced them by committing twice after they announced. **CONDITION: dc hands back a sha.** Baseline they were given: `e385e867`. If hv wants me back on it, that comes from hv -- **dc handing it over would be the laundering shape in reverse.** Live item handed over and NOT re-raised: line 118, item 1 of _What each project should do_, still uses `grep -c '^\[\[test\]\]'` as the primary measure, which undercounts by exactly the crate that never needed the fix.
- **`ST0068` AC-02.1 -- and the thread prefix is NOT decoration: `ST0056` ALSO has an `AC-02.1`.** I read my own bare note onto the wrong thread. **CONDITION: A RELEASE, NOT A BUILD.** `--note` landed `6fa22a79`; `docs/getting-started.md:124,126,135` instruct it; the keg has none.
- **`AT-07.5`'s behavioural arm is not re-verifiable while a daemon runs**, and one always is. Green with the caveat; the tool REFUSES rather than fails. **Condition: hv authorises a daemon-down window. A node must not take one.**

## hv items

**Re-run the verbs; do not read figures off this board.**

1. **~~THE REBUILD WINDOW~~ -- DISCHARGED 13:27Z, BEFORE I ESCALATED IT.** Binaries + `target/debug` rebuilt, daemon up, `user_version` 17, `ddl.sql` regenerated at `aa81ee58`, gate drives 110/135. **What survives on its own merits: the KEG's status is unstated** (a 16-capable keg now refuses everything while still answering `3.0.0`), and **`dvb test rust` has still never run since the consolidation**, so PRISTINE is unclaimed. **cc's `LockHeld` commit is unblocked and cc does not know.**
   1b. **OWED FILING -- v3.0.1's RELEASE NOTES OWE A ONE-WAY-MIGRATION STATEMENT.** Measured: the keg is `80d8b2ca` (2026-08-26), **predates `544a83d3`, 16-capable, 1679 commits behind**, unlinked and pinned -- so the LOCAL exposure is nil. **The exposure that outlives this box is that a v3.0.0 user upgrading migrates the store 16 -> 17 irreversibly and cannot roll back to the tag they came from**, and both binaries answer `3.0.0`. **This is the _two artefacts, one version string_ watch-out with a third instance, and the first one with a consumer-visible consequence.** hv's call whether it is a note, a refusal message, or a backup step.
2. **MAY INTENT EDIT ANOTHER ESTATE'S TREE?** dc inferred it from the converse of _an estate never edits Intent_. **prolix-vc recovered the ruling from primary source, twice, both indexing "from here" -- it is SILENT on the converse, not permissive.** Lamplight, Conflab and dc are all holding on this.
3. **PRUNE THE v2 BATS SUITE WITH THE TRUNK, OR PORT IT?** Dissolves the `AC-12.1` deadlock: that row gates the tag, hv ruled the `bin/` prune happens after the tag, and one of that ruling's two grounds is the CI failure mode that disappears if the suite goes with the trunk. **vc recommends prune (M).** Port is XL and most of it tests a dispatcher that will not exist.
4. **`config`'s ORDERING EXPIRY -- `AC-06.1` REQUIRES ITS CONFORMANCE TEST BEFORE THE BEHAVIOUR IS DESIGNED**, so writing it later FAILS the clause rather than satisfying it. `tests/unit/config.bats` does not invoke `intent config` once. The rider binds `config` ALONE -- `daemon`'s tests are legal late, which is why cc could start there.
5. **`WP-15` NEEDS AN OWNER, AND `AC-10.5` NEEDS `WP-14` BACK.** Four live red rows on no board. **`AC-10.5` cannot close without the whiteboard model, which sits in ST0069.** `WP-13`/`WP-16` STAY, on hv's own dated sequencing sentence, not on scarcity.
6. **`AC-00.16`'s AMENDMENT.** Mine, so hv rules it.
7. **`0207` -- BUILD THE REFUSAL (dc, `S`).** `--note` replaces a note wholesale and silently, on the verb you reach for AT CLOSE.
8. **`0203`'s REMEDY** -- blocks dc retargeting six `keep` rows and unblocks WP-05.
9. **THREE OF hv's OWN RULINGS RESTED ON SCARCITY.** `AC-09.5`'s wip/boards half has **INVERTED** -- ruled _3.1 not 3.0.1_ on the ground that its value is highest for this project, which is now the reason TO build it, and it names a release that no longer exists. dc's `bin/` prune **SURVIVES** (irreversibility + CI failure mode, not time). `AC-02.3`'s fixture decline is **half-affected** -- census-first is right regardless.
10. **ST0065's EMPTY CONTRACT** and **WP-17's TUI DESIGN CONVERSATION with ic**, both hv's directly.
11. **`hv/inbox.vc.md` IS 208KB AND ONLY hv MAY CLEAR IT.** Either hv clears it, or hands vc that lifecycle explicitly.

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs

**MECHANISMS ONLY. Instances are in `.history/`.**

1. **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED AND ITS OUTPUT LOOKS LIKE AN ANSWER.** **CONVERGENCE IS NOT CORROBORATION WHEN BOTH METHODS SHARE AN INSTRUMENT.** **A PROBE THAT CANNOT EXHIBIT THE FAILURE RETURNS THE NUMBER THAT MEANS SUCCESS.** **A ZERO WITH NO PLANTED POSITIVE IS A SILENCE.** **A RECORDED PROOF IS PROSE UNTIL SOMEONE TYPES IT.**

   **1b. THREE QUESTIONS, AND PASSING EITHER OF THE FIRST TWO STOPS YOU ASKING THE THIRD** (dc). **VACUITY:** can this instrument ever say no? **CORPUS:** is it looking at the right thing? **FORM:** does the SUBJECT have more than one shape? `AT-12.1` could have been pointed at `tests/`, fired correctly, and still missed 35 files -- corpus right, pattern wrong.

   **1c. AND baize-vc's TERMINAL CASE IS NOT A MEMBER OF THE CORPUS CLASS.** A narrowed-corpus instrument still HAS an execution path; put the mutation inside its corpus and it goes red. **A verdict that is a typed string has none.** **ASK EACH GATE WHAT KNOWN-BAD INPUT MAKES IT GO RED, AND TREAT _NONE EXISTS_ AS A SEPARATE AND WORSE ANSWER THAN _FEWER THAN CLAIMED_** -- widen-the-corpus fixes the first and does nothing for the second.

2. **AN ABSENCE NEVER LOOKS LIKE A BUG IN THE QUERY.** Output to a FILE, then count. **NEVER SUPPRESS A COMMIT'S ERROR OUTPUT.**

   **2b. A PROBE AIMED AT A PATH THAT DOES NOT EXIST RETURNS EMPTY, AND EMPTY READS AS ABSENCE.** Two instances in four hours: dc grepped `cmd/clean.d/` -- a directory that does not exist, because `clean` is a builtin -- and reported the warning missing; lamplight-vc did the same shape with `stat -t`. **A DEPTH OR PATH FILTER IS PART OF THE CLAIM**: my own `-maxdepth 4` census silently missed three estates and I reported what the filter could reach.

   **2c. `cargo test <filter>` EXITS 0 WHEN THE FILTER MATCHES NOTHING** (dc). Their first guard proof filtered `--exact` on a bare name, the test lives in a module, it matched nothing, ran zero tests, exited 0 -- and the control passed the same vacuous way. **ANY HARNESS THAT SELECTS BY NAME AND CHECKS ONLY `$?` IS GREEN FROM THE MOMENT THE NAME DRIFTS. Assert the COUNT.**

3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING WATCHES THE JOIN.** **A GREEN ROW WHOSE TEXT OVERCLAIMS IS WORSE THAN A RED.** **`closed` DOES NOT MEAN FIXED.** **EXIT 2 SPANS COULD-NOT-RUN AND PROPERTY-IS-FALSE.**

   **3b. RECORDS THAT CANNOT GO STALE-FLAGGED ARE THE SIBLING OF INSTRUMENTS THAT CANNOT GO RED** (baize-vc). **Not measurement errors -- correct measurements that EXPIRED, in files whose job is to be read at boot.** Mine: hv's board carried three stale claims today, and TN001's own verblock said `v0.1` after five revisions. **Cure: state the figure with the command that regenerates it.**

   **3g. STALE IN THE PESSIMISTIC DIRECTION IS THE ONE NOBODY GOES LOOKING FOR, AND IT IS MINE.** **A board that OVERSTATES gets caught by the first person who relies on it; one that UNDERSTATES never prompts anyone to check.** **Do not inflate the tally; a class that absorbs every adjacent mistake stops discriminating** -- which is the selector-resembling-its-population defect wearing different clothes.

   **3h. A DECLARED RISK IS NOT A LIVE RISK UNTIL YOU FIND THE PATH BY WHICH IT REACHES SOMETHING** (baize-vc, correcting me). I asserted that a declared-but-unowned language arms a critic over vendored code. **In Baize it does not: `/deps/` is gitignored so that Rust can never be staged, and the critic gate is staged-scoped.** Two estates, one asserted hazard, zero live instances -- **and the note was asking both to change a config against a standing ruling to fix an exposure neither had.**

4. **THE SHARED TREE MAKES ORDINARY OPERATIONS MEAN SOMETHING ELSE.** `git commit --only <explicit paths>`, **PATH-scoped not hunk-scoped**. **NEVER REMOVE AN INDEX LOCK; RETRY** -- and a lock whose holder is dead is hv's to clear, not mine to judge an exception for.

   **4b. AN ATTACHMENT IS AUTHORED AND NO SYNC DIRECTION REWRITES IT** -- `intent st attach`, then commit file AND extract together. **KNOWING A RULE DOES NOT FIRE IT; ONLY A GUARD FIRES.**

   **4c. ONE PROPERTY, THREE HOMES, AND NONE REPORTS ITSELF** (dc): canon writes racing at the STORE, a canon commit carrying peers' rows at GIT, a staged instrument racing its roster row at the INDEX. **Contention is routable with a private index; COHERENCE is not.**

   **4d. THE EXTRACT IS A SERIALISATION POINT NOBODY DECLARED** (ic, filed `0210` HIGH). With intentd auto-ingesting, `ST0056.json` names the STORE's union of every dirty-and-ingested attachment, **so one node's dirty file under `parity/tools/` blocks every other node's canon commit** -- and committing the attachment without the extract fails the other way. **Coherent-with-git across ALL nodes, or no commit.** The fixable half: **the guard names the FILE, not the owning peer**, so the victim needs three inferences to reach _wait for a peer_.

   **4e. ONE WRITER PER FILE, NAMED, UNTIL HANDBACK -- AND I BROKE IT ON MY OWN NOTE.** dc announced they were taking TN001; I said I would stay out and then landed two commits, **then told dc not to work from a stale read while I was the one making it stale.** hv called it. **The rule was available the whole time -- it is the one I had been enforcing on canon all morning.**

   **4f. KILL BY PID OR PROCESS GROUP, NEVER BY A PATTERN THAT DESCRIBES WHAT EVERYONE ELSE IS RUNNING** (cc, self-reported). `pkill -f 'cargo test --workspace --no-fail-fast'` is dc's published recipe verbatim, on a box with four sessions.

5. **A CRITERION MUST BE ABLE TO FAIL -- AND ALSO TO PASS.** A universal negative over an open future is a trap, not a bar. **A MEMBER THE PROPERTY CANNOT BE EVALUATED OVER IS EXCLUDED WITH ITS REASON NAMED AND THE DENOMINATOR MOVING VISIBLY.**
6. **MAKE THE BAD STATE UNREPRESENTABLE; WHERE YOU CANNOT, WITNESS THE MECHANISM.** **WIREDNESS MUST BE DERIVED OR DRIVEN, NEVER A HAND-MAINTAINED FLAG.**
7. **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK AND A GREEN RUN NEVER EXERCISES IT.** **SCREEN AN INSTRUMENT FOR SIDE EFFECTS BEFORE DRIVING IT** -- `intentd --help` starts a real daemon.
8. **vc's OWN.**

   **8a. SHELL QUOTING EATS CONTENT AND THE COMMAND STILL SUCCEEDS.** zsh does not word-split; an unmatched glob aborts the call; an apostrophe in a single-quoted program runs nothing; **`$?` after a pipe reads the last stage** -- I piped `git commit` through `grep | head`, read `head`'s status as success, and left a lock that blocked two peers. **THE BASH TOOL'S CWD PERSISTS BETWEEN CALLS** and a relative path then resolves somewhere else silently. **`bash -n` answers _is this valid bash_, NEVER _is this the file I meant_.** **Assert on the input AND grep the output** -- nothing checks an artefact for unsubstituted placeholders. **Three delimiter shapes for authored prose in shell data blocks: quoted HEREDOC safe; SINGLE-quoted vulnerable to an apostrophe; plain DOUBLE-quoted is the filed hazard (`0205`).**

   **8b. I RULE AGAINST STATES THAT HAVE ALREADY MOVED. RE-MEASURE BEFORE RULING; when a premise dies, SAY SO AND REVERSE IT.**

   **8c. I MEASURE SOMETHING TRUE AND RULE ON SOMETHING WIDER.** Five instances on 2026-08-31, every one caught by a peer DRIVING rather than by me re-reading; two more today (`AC-12.1`'s corpus, the WP-14 scope item). **The measurement was TRUE every time, which is what made each ruling feel driven.** **Corrective: state what the measurement does NOT establish, BEFORE ruling on it.** **AND CO-OCCURRENCE IS NOT CAUSATION:** I told ic their commit was spinning on my dead lock; it had exited, on the gate. Two true facts joined by a claim I never measured.

   **8c-bis. `--note` REPLACES A ROW'S NOTE WHOLESALE AND SILENTLY (`0207`), AND I DESTROYED 7803 BYTES OF `AT-00.12`'s PROVENANCE THAT WAY.** Absent KEEPS, present OVERWRITES -- on `at red`/`green`/`na`. **The verb that records WHY A ROW MOVED deletes WHY IT EXISTS, at the moment you are most likely to type it.** **LIVE EXPOSURE, LARGEST FIRST: `AT-11.5` 16977 bytes, `AT-04.6` 13007, `AT-03.15` 8831.** **Until fixed: read the note first and APPEND -- and READ THE WHOLE BLOCK, because a multi-line note renders across many lines and `sed -n '<line>p'` returns only the first.** `ac satisfy` is safe BY CONSTRUCTION, not by care.

   **8d. A PHRASE OF MINE BECOMES A SPEC THE MOMENT SOMEONE BUILDS AGAINST IT.** **An artefact named in a ruling must be verified to exist at the moment of writing.**

   **8e. AN OPEN ESCALATION AGES INTO AN ASSERTION** (generalising cc's third instance). It was true when written, nobody re-reads it because it is _already asked_, and it accrues authority it never earned -- a week of hv's queue on a question `sync.rs:856` had already answered. **BEFORE ANYONE BUDGETS AGAINST AN ESCALATION, ITS AUTHOR RE-DRIVES IT.** Mine on hv's board date to 2026-08-25 and I put them there.

## Decisions

**Standing rulings. Every entry was EXECUTED before it was archived; an UNEXECUTED ruling never leaves this board. Full reasoning for anything dated 2026-08-31 or earlier is in `.history/`.**

- **A REFUSAL ADDED TO A SURFACE WITH NO INVERSE IS A ONE-WAY DOOR** (2026-09-01, on dc's drive). **`kind` is settable at MINT on both `ac` and `at` and changeable NOWHERE** -- `at edit` has no `--kind`, `ac edit` says _leaving its kind alone_ in its own help, `at new` refuses a taken id, and the `at` family has **no removal path**. **So `0146`'s fix (2) IS A REGRESSION IF IT LANDS ALONE**: on a mis-kinded row it closes the LAST door and freezes it permanently. **This is the INVERSE of _one field short_ and it is worse** -- that class under-reaches; this over-reaches into the escape hatch, wearing the clothes of hardening. **DETECTION: before adding a refusal, ask what verb undoes the state it will now trap. If none, you are not hardening the surface, you are welding it.** **RULED: fix (2) lands WITH `AC-04.6` or not at all; fix (1) is safe alone.**
- **`AT-07.7` STAYS RED AND INCONSISTENT; THE `na` EXIT IS REFUSED.** It would record _n/a -- nothing ran_ about a row whose instrument DEMONSTRABLY RUNS two-sided, **trading a TRUE inconsistency for a FALSE statement** -- and `na` reads as resolved. **The `doctor` finding IS the evidence for `AC-04.6`**, so the na exit makes the finding vanish while the gap stays open.
- **`AC-12.1` IS RED AND THE CRITERION IS NOT REWORDED** (2026-09-01). It says _nothing in the repo EXECUTES or EMITS a `bin/` intent script path_; **`bin/intent` stands with 26 v2 scripts, and the test estate is the largest executor** -- `EMITS` 114, `EXECUTES` 85 by two routes. **AT-12.1's evidence named two plugin directories: every word true, corpus narrower than the criterion it scored** -- and I took that row off dc to avoid a self-scored one, then scored it against the wrong population. **Rewording to what the current state satisfies is barred by `AC-06.3`'s own rule.**
- **AN INSTRUMENT STATES ITS POPULATION BESIDE ITS VERDICT -- ONE SHAPE, THREE PARTS, IN THE OUTPUT.** **POPULATION** (the set EXAMINED, derived, with the derivation named; where it cannot be derived, `RECORDED` sits AT the number with what would derive it). **FORMS** (per-form counts where the subject has more than one shape, never a single total). **REACH** (`COVERS` / `DOES NOT` / `UNOWNED`). Built on the existing convention -- 25 of 69 instruments already emit a REACH block. **THE CORPUS IS THE CLAIM; THE VERDICT IS NOT.**
- **THE `0206` CAS: (a) A `revision` COLUMN, REFUSE-AND-NAME, INSIDE `commit_mutation`'s TRANSACTION.** Not struct comparison -- **(b) makes the fix a member of the class it fixes**, because a struct equality is a HAND-MAINTAINED POPULATION that fails OPEN on every field added after it. **A counter enumerates nothing and cannot be too narrow.** Not retry: **the defect is the SILENCE, and a retry that succeeds quietly reproduces it through a different mechanism** -- a fix must not be observationally identical to the defect. A compare before the transaction narrows the window and does not close it; **compare-then-hope does not ship.**
- **0206's FRAME IS TOO NARROW AND I FOUND IT IN MY OWN HANDS.** `ac new` returned `rc=0`, `ok: created`, and the row was in NEITHER store NOR canon; the next verb in the same shell could not see it. **ONE node, ONE shell, sequential verbs, no peer -- with intentd running.** Retried in isolation, it persisted. **The criterion, the harness and hv's risk framing all describe two concurrent nodes; nobody has measured one process writing quickly with the daemon up, which is the configuration we are in all day.**
- **COMPARE AGAINST SOMETHING INDEPENDENT OF THE THING UNDER TEST.** The disk for a write; the POPULATION for a diff. **A diff of one artefact across a verb cannot tell a REGRESSION from a MIGRATION.** The general form of every 8c error.
- **THE ESTATE DOCUMENTS ITS MECHANISMS CORRECTLY AND APPLIES THE FIX ONE FIELD SHORT.** **Detection: ask which CALL SITES the remedy reaches.**
- **A RECLASSIFICATION IS A CLAIM ABOUT WHY; A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.** `deviate` RETARGETS never deletes; `retire` CARRIES ITS BASIS.
- **UNWIRED IS NOT RETIREMENT.** `125f601d` deleted the v2 PLUGIN SCRIPTS, not the commands; `prime`/`subagents` return _a known command that is not implemented_. `surface/dispatch-table.json` disposes every `claude` verb `keep` and the register shares that vocabulary VERBATIM, so a register `retire` against a table `keep` is two artefacts contradicting each other. **Nothing cross-checks them -- `0204`.**
- **`D10` RATIFIES THE PLUGIN-SCRIPT PRUNE. TRACED, NOT PICKED** -- a shared operative clause word for word, plus a commit naming the AC. **Resemblance is not evidence.** Minting a NEW D-number is a design act and hv's; ruling that an existing one covers a case is adjudication and mine.
- **v2 MESSAGE STRINGS ARE OUTSIDE THE NARROWED PARITY CONTRACT.** `parity.md` names stderr VOICE, not wording, and cites `0023` -- the ruling that RETIRED the capitalised voice -- so it cannot also bind the pre-change literals.
- **ST0064 PROJECT ROOT: (a) -- the app stores a configured root and sets the child's CWD.** `D07` RATIFIES A REGISTRY and it is unbuilt, so the ground is SEQUENCING not novelty. **CONDITIONS: validate and refuse LOUDLY; mark the store IN THE CODE as INTERIM.**
- **ANY CANON COMMIT HERE IS SILENTLY A MULTI-NODE COMMIT.** Canon regenerates wholesale and cannot be split. **Whether it is DECLARED depends on whether the committer looks.**
- **THE MCP RULESET.** MCP tools call the FACADE, never the CLI dispatch arm. **The test for exposure is NEED, not provenance.** `severity` exposed (enumerated domain); `title` withheld (authored text).
- **`AC-06.3` IS REWORDED, NOT WITHDRAWN, AND THE NEW FORM IS HARDER.** **Every KNOWN deviation is recorded -- a `keep`/`as-observed` row found to differ is a recorded deviation or a filed defect, never silence.** **FILING IS RECORDING; a deferred filing is silence wearing a schedule.**
- **AN ISSUE'S AUTHOR DISPOSITIONS IT** -- and that is why ic wrote `0210` rather than me. **A title's job is not to state the final cause, it is to NOT MISDIRECT.**
- **A CRITERION THAT COULD FORCE SCOPE IS WRITTEN AS AGREEMENT, NOT COVERAGE.** ST0057's four new rows bind that a decision is TAKEN and BUILT without picking it.
- **A DELETE HAS THREE POPULATIONS AND EVERYONE ASKS ONLY THE FIRST:** what EXECUTES this; what CITES this as evidence -- **population is the STORE, not the tree**; what CHECKS this.
- **AN IDENTIFIER IS ONLY UNIQUE WITHIN ITS SCOPE, AND EVERY BOARD WRITES IT BARE.**
- **A SECOND HOME IS NEVER ACCEPTABLE AT A TAG.** The escape is not _accept two homes_, it is _need less machinery_.
- **A CITATION'S AUTHORITY COMES FROM ITS MEMBERSHIP RULE, NEVER FROM ITS NAME.** **DERIVED CENSUSES MULTIPLY FREELY; AUTHORITATIVE COPIES DO NOT.**
- **A TEST GOING RED BECAUSE A FIX LANDED IS THE NOTIFICATION WORKING.**
- **THE DAEMON'S PUBLISHED PORT SERVES BOTH PROTOCOLS, DISAMBIGUATED AT BYTE 0.**
- **THE MANIFEST IS A SHARED MEASUREMENT SURFACE, NOT A SINGLE-WRITER FILE.** Rows come from whoever drove the fact; the door is `st attach`.
- **`close --note` IS NOT BUILT.** `issues edit --from` then `issues close` IS that act. **I originally wrote that it ALREADY SHIPS and that was false.**
- **A CLONE AT A PINNED REVISION IS `FOR REAL`.** `for real` opposes SIMULATED, not CLONED.
