---
node: vc
name: Validation Claude
role: validation
session_id: d6fb337d-3328-4360-865e-04ff4ba194e7
commit_session_id: 012urACYMDQ4oZhtofSEJzxg -- the Claude-Session trailer id on commits authored from THIS session; POINT-IN-TIME, one session per line, and the mapping accumulates in .history/ rather than growing here (0208)
heartbeat_at: 2026-09-03 15:16Z
status: active
focus: "BOUNCED 2026-09-03 15:16Z after a >1h backend outage; hv back and said kick off work again. RELAYED VERBATIM to cc/dc/ic with the LIMIT STATED FIRST -- a general restart is NOT a window grant, and the daemon-down, rebuild and flip-then-burn windows each still need hv naming them specifically. STATUS STAYS active. THE PROGRAMME IS STILL THE LLM-CONFIG RATIONALISATION, still blocked on the same three decisions. RUN THE VERBS; no figure here is evidence."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS and UNEXECUTED WORK.**

**AND THE RULE THAT GOVERNS THIS BOARD: PRINTING THE COMMAND IS NOT RUNNING IT.** A figure with its regenerating command beside it is AUDITABLE, not CURRENT, and no reader can tell which they are holding -- **proved by TN001's crate table going stale UNDER a caption boasting it was cargo-verified.** **A LIVE FIGURE APPEARS HERE AS THE COMMAND ALONE, WITH THE NUMBER DELETED.** **A HISTORICAL FIGURE IS DATED**, and stops being a live claim.

## DOING

**ROUTING AND ADJUDICATING; nothing of vc's uncommitted.** hv delegated the open set (_"go with your recs"_), so **a vc ruling under hv's pen declares `authority: vc`, NEVER `authority: hv`** -- and see `9c`, which is vc getting that exactly wrong in the message that explained it.

**THE LLM-CONFIG RATIONALISATION IS THE LIVE PROGRAMME AND IT IS UNBUILT.** Target APPROVED by hv 2026-09-03 and published: **https://claude.ai/code/artifact/d3da2edf-23fd-42b1-bc92-05c33bf1a3a8** -- twelve invariants, three mechanisms (DERIVED / TESTED / POINTED), the artefact table, and the `intent llm` verb. **THE ARTEFACT IS THE NARRATIVE'S ONE HOME; this board carries only what is UNEXECUTED.** Gap analysis and the five-phase transformation plan are both delivered to hv in chat and are NOT on disk anywhere -- **if hv wants them durable they must be re-stated, and that is a gap in this fold, named rather than hidden.**

**THE PROGRAMME'S LOAD-BEARING FIGURE, RE-DRIVEN 2026-09-03 AGAINST `3f037021`: the generator emits FOUR `IN-AG-*` ids and the library holds SIX.** Do not transcribe that; the commands are `intent agents generate | grep -oE 'IN-AG-[A-Z-]+-001' | sort -u` and `intent claude rules list --lang agnostic`. **The ordering constraint is the whole plan: DERIVE THE INDEX BEFORE SHIPPING ANY SYNC**, or the sync distributes the wrong index to ten estates faithfully and reports green.

**vc's OWN WORK, UNSTARTED: `WP-15` skills-catalogue triage** (`ST0065`'s criteria are the bar); **the estate-tree question**, unsourced and born in a fold; **`hv/inbox.vc.md`'s lifecycle**; and **lamplight-vc's four tool defects are still UNFILED pending hv's word on issue numbers** -- `acceptance: exempt` is the one with a live cost, blocking three Lamplight WPs.

## Holds

**Each carries the CONDITION that releases it. A hold with no condition is an abandonment.**

- **`ST0068` AC-02.1 -- and the thread prefix is NOT decoration: `ST0056` ALSO has an `AC-02.1`.** vc read a bare note onto the wrong thread. **CONDITION: A RELEASE, NOT A BUILD.** `--note` landed `6fa22a79`; `docs/getting-started.md:124,126,135` instruct it; the keg has none.
- **`AT-07.5`'s behavioural arm is not re-verifiable while a daemon runs**, and one always is. Green with the caveat; the tool REFUSES rather than fails. **Condition: hv authorises a daemon-down window. A node must not take one.**

## hv items

**FOLDED 2026-09-02 23:17Z; pre-fold verbatim at `.history/20260902/wip-prefold-2312Z.md`. Everything discharged this evening is archived there with its reasoning.**

### THE LLM-CONFIG PROGRAMME -- THREE DECISIONS OWED, ALL UNEXECUTED

**hv APPROVED THE TARGET 2026-09-03** (_"Yep, love it. Lets do it."_). Published: **https://claude.ai/code/artifact/d3da2edf-23fd-42b1-bc92-05c33bf1a3a8**. Gap analysis and the five-phase plan were delivered IN CHAT ONLY.

- **(a) A STEEL THREAD.** vc proposed rather than created it -- `st new` is a write and the scope is hv's. **Nothing is buildable until this exists.**
- **(b) THE VERB NAME.** `intent llm` recommended; `intent canon` reads better and collides with `intent/.canon/`. hv's call.
- **(c) DOES PHASE 0 RIDE v3.0.1?** `in-standards/SKILL.md:18` states a writer v3 retired -- **false in canon AND in every installed copy, in the skill every session loads at boot**, and it has already caused a peer estate to design the wrong fleet-wide fix. **XS, one line.** The rest of the programme is a NEW THREAD AFTER THE TAG -- hv's 3.0.1 scope is ST0056 + feeders and this is wider.
- **AND A GAP IN THIS FOLD, NAMED RATHER THAN HIDDEN: the gap analysis and the transformation plan are on NO DISK.** They survive only in the pre-compact transcript. **If hv wants them durable, vc must re-state them into the thread once (a) lands.**

### hv's HANDS -- four

1. **`flip` THEN `burn`, ONE SITTING, IN THAT ORDER.** `flip` = rebind the default `INTENT_BIN` off `bin/intent`, the v2 SHELL SCRIPT (`tests/lib/test_helper.bash:21`). `burn` = re-run `burn.sh`, every `.bats` twice, classifying by the delta; hung 3.5h once. **A burn before the flip produces a baseline the flip invalidates. NEITHER NEEDS A BUILD.** Also `AC-06.1`'s remaining coverage limb.
2. **`ST0057 AT-07.5` NEEDS A STOP-THE-DAEMON WINDOW.** Arm A REFUSES rather than fails while an intentd is up, **so the green rests on a moment nobody can reproduce on demand.** **NO LONGER PAIRED WITH `0216`'s ARM** -- that arm wants the daemon RUNNING, so this is the only remaining window ask.
3. **`ST0064` 01.7 SIGNING NEEDS hv's ADC.**
4. **THE DOC GATE -- RE-RULE ON CORRECTED GROUND.** vc's _flip now, fix forward_ is WITHDRAWN: cc re-drove the class against the SITES rather than the NAMES and **five of six supposed absentees are present under another name.** **The class is ONE-TOKEN PATH REPAIRS, not missing mechanisms**, so _a gate waiting for zero on a growing class waits forever_ is false. **vc recommends the config's OWN condition: clear the class, then flip.** Held with dc; not built.

### For hv's morning, narrowed rather than guessed

5. **`intent claude skills sync` REPORTS `4 need a decision` ON EVERY RUN, AND NOBODY KNOWS WHAT THE FOUR ARE.** `list` shows all 23 `installed canon`. **vc narrowed it and did NOT resolve it: it is NOT extra installed skills and NOT orphans** -- `~/.claude/skills/` holds exactly 23 and the set difference against canon is empty both ways. **So it is a different population from the one `list` renders.** Seeing the message needs a `sync`, which is a WRITE. **ic flagged it and refused to guess; vc removed two candidates and did not improve on that.** **A standing count with no visible subject is the shape this whole day was spent catching.**
6. **`0218` -- `uninstall` LEAVES AN EMPTY DIRECTORY**, so a directory count disagrees with a skill count. hv killed the two residues; the tool is unchanged. ic filed at LOW.

### Open, lower rank

7. **`0216`'s PROVOCATION ARM (cc)** -- see `9a`. **It needs nothing hv has to grant**, but it must be FENCED: scratch entity, peers quiet and told, repair loop armed. **Provoking an ingest is firing the loss mechanism on purpose on a live tree.**
8. **A SECOND CENSUS, OFFERED BY dc AND NOT COMMISSIONED: 57 of 138 `v2:` entries declare NO `evidence_class`.** Larger gap than the collision class. **dc wants a positive control before believing any number it produces, and that condition is ratified in advance.**
9. **CANDIDATE, NOT FILED: a stale-heartbeat guard** comparing `heartbeat_at` against the board file's own last-commit time -- **about the ORDER of the two, never a GAP** (dc), or it fires on everybody mid-session.
10. **THE DAEMON-LOCK RACE STILL HAS NO ISSUE AND WANTS A RECORD** (cc's).
11. **`AC-10.5`'s RE-DISPOSITION IS RATIFIED AND UNSTARTED (cc's).** _Modelled in ST0069, not in this release._ **Second in cc's order, ahead of the `0216` arm** -- it should not queue behind work that waits on a window.
12. **`intent_agents.bats` STAYS `pending`** -- its failures are v2 WORDING, which the narrowed contract excludes. **If it is to go green the path is `wp_commands`': assert the CONTRACT, not the string.** Separate work; does not ride `0203`.

### Standing, and not vc's to unblock

13. **`AT-00.2`, `AC-00.8` AND `AC-00.10` SIT BEHIND cc's `WP-10`, WHICH IS BEHIND `WP-06` (cc's) AND `WP-07` (dc's).** Read off the cover; sequence around it.
14. **`AC-07.7` IS dc's GENUINE HOLD AND NOBODY HERE CAN UNBLOCK IT** -- it needs a keg built from fixed code, which needs a published tag.
15. **`WP-08` STAYS HELD.** 12/12 green with `wp done` an `XS` away, **and closing it marks the daemon DONE over zero conformance coverage.** cc's call, endorsed, waiting on a WORD rather than on work.
16. **`AC-02.3`'s fixture decline SURVIVES the scarcity strike**, **dc's `bin/` prune survives** on irreversibility plus the CI failure mode, and **`WP-13`/`WP-16` STAY IN ST0069 on hv's own dated sequencing sentence.**
17. **THE ACCEPTANCE GATE STILL CANNOT GO RED FROM CODE CHANGING.** baize-vc broke the test `AT-02.1` covers with a prediction written first and the gate returned all-pass; **`status: green` is a STORED STRING and the `at` family has no verify verb.** **`0207` now REFUSES a `--note` that would drop the existing one, so the free half is a GUARD rather than a discipline.** The verify half is unbuilt.
18. **`WP-14` RULING WITHDRAWN.** All 12 ACs were **descoped to ST0069 by hv 2026-08-30, `by: "hv via vc"` -- vc ruled against a descope vc relayed.** `AC-09.5`'s wip/boards half goes with it.

**Re-run the verbs; do not read figures off this board.**

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **And PFIC means _Pure Function, Impure Coordination_ -- deterministic core, I/O at the boundary. NOT the idiom gloss six documents carried until 2026-09-02.**
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs

**FOLDED 2026-09-03 15:03Z. Pre-fold verbatim + cmp-verified at `.history/20260903/wip-prefold-1503Z.md` (sha `8d4fdc47`, 308 lines / 52214 bytes).** The 9-series below is compressed to CLAIM + DETECTION; every incident narrative is in that snapshot. Census across the fold: **19 nine-series ids and 37 eight-series mechanisms in, same out**, with a planted positive (`9m` found) and a planted negative (`9zz` absent) proving the census discriminates.

- **9a. `0216` IS AN ACTIVE LOSS CONDITION.** A canon write reports `ok`, LANDS, and intentd's disk ingest REVERTS it ~1s later. **The mitigation is a LOOP ON AN OBSERVABLE, never a wait on a duration** -- read back, compare, re-write, repeat. Not `verify past the ingest` (smuggles a constant nobody has); not `one verb at a time` (the debouncer sees WRITES, not AUTHORS). Mechanism UNKNOWN, four readings killed. **Reachable by an ordinary shell loop, so the fix cannot be discipline -- it is daemon-side.**
- **9b. THE BASE RATE IS NOT THE CONDITIONAL.** _How often does the risky shape occur_ is not _given it occurs, what happens_. **DETECTION: the base rate looks like the safety question and answers a different one.**
- **9c. AN AUTHORITY FIELD RECORDS WHERE THE WORD CAME FROM, NOT WHETHER IT IS TRUE.** A true relay is still a relay. vc walked into it while citing it; ic refused and got hv first-hand.
- **9d. A DISPLAY FILTER IS A CLAIM ABOUT THE POPULATION, AND IT IS THE ONE NOBODY REVIEWS.** Two filters encoded the same hidden assumption -- that a loss gets NOTICED -- which was the variable under study. **CURE: make the summary RECONSTRUCT from two directions; a printed total hides a miscount, a total shown CLOSING cannot.**
- **9e. A CLAIM THAT SOUNDS LIKE PHYSICS IS EXEMPT FROM THE STOPWATCH.** _A process spawn costs >250ms_ survived four assertions and died to a four-second measurement (23-26ms). **A CONSTRUCTED VARIABLE STOPS BEING LABELLED CONSTRUCTED THE MOMENT IT IS QUOTED, INCLUDING BY ITS AUTHOR.**
- **9f. A SURVIVAL IS NOT EVIDENCE THE DISCIPLINE WORKS WHEN THE MECHANISM WAS NOT ARMED** (ic). **And its inverse: a mechanism correction is not a refutation of the conclusion it sits under.**
- **9g. A GUESSED FIELD NAME RETURNS `absent` FOR BOTH _NOT THERE_ AND _NOT WHERE I LOOKED_** (ic). **CURE: walk the objects AND NAME THE FIELD; before ruling on any row, query its `state`.**
- **9h. A REFUSAL-AND-RETRY IS WHERE A DISCIPLINE GETS DROPPED.** The retry FEELS like a continuation and is a FRESH COMMAND. **RE-ISSUE THE SAME COMMAND, never recompose it.** Kept twice today: the format-refusal on this board, and cc waiting out an index.lock rather than removing it.
- **9i. ON A SHARED TREE A WRITE'S OUTCOME IS NOT DETERMINED BY THE WRITER.** A plain `git commit` takes the index AS IT STANDS; a shared gate reads a PEER's live working tree.
- **9j. THE STORE'S GRAMMAR REFUSES SCHEMES FASTER THAN A PARAGRAPH DEFENDS THEM.** **DETECTION: when a ruling invents a HOUSING, drive the write BEFORE writing the reasoning.** And the sharper half: a structural check aimed at the WRONG FIELD returns a true answer nobody was blocked on and reads exactly like due diligence.
- **9k. A PREMISE EXPIRES BETWEEN BEING FORMED AND BEING ACTED ON.** Six times 2026-09-02; **three more on 2026-09-03, every one caught by driving rather than reasoning** -- `AC-17.1`'s reads-only menu (cc built `Op::Set`), the built-pair lag (rebuilt), `AT-07.5`'s window (an observable, not a consent). **A RULING WHOSE PREDICATE DISSOLVES WHILE ITS PRINCIPLE HOLDS MUST BE RECORDED AS SUCH, or it reads as a reversal.**
- **9l. AN ASSERTION THAT LIVES IN A COMMENT CANNOT BE CHECKED** (dc).
- **9m. A MENU PUT TO hv IS A CLAIM ABOUT THE OPTION SET, AND THE OPTION SET IS THE PART THAT EXPIRES.** The recommendation and grounds get re-read because they are argued; the option set reads as the statement of the problem. **DETECTION: drive the EXHAUSTIVENESS claim -- grep the enum, not the prose about the enum.** Second instance in two days (cc's `sync` discharge was the first).
- **9n. A RELEASE NOTE IS AN INSTRUMENT WHOSE POPULATION IS THE TREE AT THE MOMENT OF WRITING** (dc). `RELEASE_NOTES.md:51` went false when the schema moved 13 -> 17. **The dangerous half was the clause that was TRUE: a reader who verifies the stated direction confirms it and walks through a one-way door.** Fixed `c9ec5dd7`.
- **9o. FOUR HOMES FOR _WHAT IS A BUILD INPUT_, AND THE REACH IS STATED IN A DIFFERENT FILE FROM THE IMPLEMENTATION.** `source_commit.rs:174` and `sharedtarget.lib:124` carry 3 paths; `currency.lib:108` diffs 1; `self_provenance_check.sh:388` hardcodes the scope in the PROSE a reader sees. **A COMMITTED `surface/` edit passes every guard green; an UNCOMMITTED one is caught -- the direction is inverted.** dc filing; not started.
- **9p. FOUR TOOL DEFECTS FOUND FROM OUTSIDE THE ESTATE** (lamplight-vc), three confirmed here: `acceptance: exempt` has no setter and a TEST pins the dead-end refusal; a green gate over rows the enumerator cannot read; **505 criteria, ONE key-set `[id,kind,state,text]`, `on`/`control`/`red-first`/`evidence` ZERO each**; and the shell critic scoping on `bin/*` with no extension constraint across **FIVE** canon rules, two critical.
- **9q. A PROBE RUN FROM THE WRONG ROOT DOES NOT ERROR -- IT AGREES WITH YOU.** A `cd` persisted between calls and voided two probes; both returned clean zeros that read as CONFIRMING the reporter. **A wrong-cwd probe and a true negative are the same bytes.** Caught only by a planted positive. Five such probes across two estates in one day.
- **9r. A GATE VERDICT READ FROM THE WORKING TREE IS NOT A PROPERTY OF A REVISION.** `at lint` reads the working tree, so the contract arm can go green on bytes in nobody's git, and nothing in the output says which tree it read. **THE SIGN FLIP IS THE DANGER: `8ab+8ae` says one node's in-progress state is another's OUTAGE, and an outage gets INVESTIGATED; this is another node's GREEN, and a green gets RECORDED.** **RULE: before a gate figure goes anywhere durable, confirm the paths it read are COMMITTED.**
- **9s. `intent/llm/` IS WRITTEN ONCE AT INIT AND OWNED BY NO REFRESH VERB, IN EVERY ESTATE.** Not in the canon set; `lang init` installs nothing (`rules.rs:67`). **ORPHAN-NESS IS A PROPERTY OF THE CODE, NEVER OF A FILE'S HISTORY -- the test is _does a verb write this_.** Two instruments named and REJECTED: marker-scanning finds only files that already have one; date-cohorting finds only estates where nobody edits (**driven: our four span five months and all four are orphaned**). **The estate's own `in-standards/SKILL.md:18` names a writer v3 retired, and it misled a peer estate into designing the wrong fleet-wide fix.**

### The 8-series -- claim and detection only; every incident narrative is in `.history/`

**MEASUREMENT AND INSTRUMENTS**

- **8t+8ac. A PROCEDURE I AUTHOR IS AN INSTRUMENT, AND AN INSTRUMENT GETS A POSITIVE CONTROL.** Applies to the probe as readily as to the subject.
- **8x+8af. A CONTROL THAT CAN ONLY PASS IS DECORATION, AND THE THING IT CATCHES MAY BE ITS OWN AUTHOR.** A control that would pass under the broken instrument too establishes nothing. **`AC-00.11`'s own founding defect (`EXAMINED 86 of 278`) has N < M and a closing remainder, so NEITHER mechanical arm fires on it** -- true of the ARMS and false of the CRITERION.
- **8o. A CONTROL THAT MOVES A VARIABLE THE INSTRUMENT IGNORES IS GUARANTEED TO REPRODUCE**, and proves nothing about the variable that matters.
- **8u. A FOLD CONTROL THAT COUNTS IDS DETECTS ONLY DELETIONS.** A fold that moves PROSE needs a MECHANISM census beside the id census.
- **8at. AND THE MECHANISM CENSUS HAS ITS OWN BLIND SPOT.** Its first regex required a capital start, so id-prefixed and marker-prefixed headers were invisible and it reported rewordings while whole entries vanished. **A CENSUS NEEDS A PLANTED POSITIVE: delete a known entry, confirm the census names it, restore.**
- **8k. THE PROXY THAT IS WRONG IN THE DIRECTION THAT PRODUCES WORK IS THE ONE NOBODY AUDITS.**
- **8i. A `head -10` TRUNCATION READ AS A COMPLETE POPULATION** almost put a false gap in the record.
- **8y. THE SEAM THAT MAKES A THING PROVABLE ALSO PUTS ONE CLASS OUT OF REACH.**
- **8am. THE ESTATE ITSELF HAS A REACH AND NOTHING OWNS IT** -- `AC-00.16`'s amendment, landed `54ea0cc6`. **Reach is bounded by what the ENVIRONMENT can exhibit, not only by what the instrument checks** -- worked case: in a self-hosted checkout the install root and the project root are one directory, so no drive here can exhibit their difference (`0215`).
- **8ah-bis. A REPORT-ONLY GATE WHOSE OUTPUT NOBODY DIFFS DECAYS INTO DECORATION.**
- **AN INSTRUMENT STATES ITS POPULATION BESIDE ITS VERDICT** -- POPULATION (derived, with the derivation named), FORMS (per-form counts, never a single total), REACH (`COVERS` / `DOES NOT` / `UNOWNED`). **THE CORPUS IS THE CLAIM; THE VERDICT IS NOT.**
- **COMPARE AGAINST SOMETHING INDEPENDENT OF THE THING UNDER TEST.** The disk for a write; the POPULATION for a diff. **A diff of one artefact across a verb cannot tell a REGRESSION from a MIGRATION.**

**PREMISES, STALENESS AND PROVENANCE**

- **8ai+8an+8ap. A PREMISE IS NEVER RE-DRIVEN UNTIL ITS REMEDY, AND IT CARRIES A VERSION.** A claim true of v2 re-drives CLEAN against the v2 line it cites, **so only a re-drive that checks the SUBJECT rather than the CITATION can find it.**
- **8b. I RULE AGAINST STATES THAT HAVE ALREADY MOVED.** Re-measure before ruling; when a premise dissolves, say which.
- **8g+8g-bis. A CLAIM ABOUT A FILE IS A CLAIM ABOUT A REVISION** and goes stale exactly like a claim about a tool. **The corrective is not more care at the moment of writing.**
- **8s+8w+8z+8ah. THE PROVENANCE OF A CLAIM IS NOT RECOVERABLE FROM THE CLAIM.**
- **8f. I STAMPED A PEN NOTE `driven` FOR A CLAIM I HAD NOT DRIVEN, AND THE MARKER IS THE WHOLE VALUE.**
- **8q. A RULING RECORDED IN A FOLD CARRIES A CRITERION'S AUTHORITY AND GETS NONE OF ITS REVIEW.**
- **8p. A STALE FIGURE MISINFORMS; A STALE TODO STOPS WORK, AND NOBODY RE-READS A TASK THEY HAVE NOT STARTED.** An untouched TODO earns a re-drive precisely BECAUSE it is untouched.
- **8e. AN OPEN ESCALATION AGES INTO AN ASSERTION.**
- **8h. A DOCUMENT APPLIES ITS OWN RULE TO CLAIMS ABOUT OTHERS AND NOT TO CLAIMS ABOUT ITSELF.**
- **8j. A TRUE FACT FROM AN ADJACENT CONVERSATION, HANDED TO A DOCUMENT THAT MAKES NO SUCH CLAIM**, arrives as evidence and is not.
- **8n. A WRONG MECHANISM PRODUCING THE RIGHT SYMPTOM HAS NO NATURAL CORRECTIVE.**
- **8d. A PHRASE OF MINE BECOMES A SPEC THE MOMENT SOMEONE BUILDS AGAINST IT.**
- **8ao. A STALE HEARTBEAT IS INVISIBLE TO EVERY GUARD, BECAUSE A HEADER NOBODY EDITS ADDS NOTHING.** vc's was ELEVEN HOURS stale while two peers read `status: active` at face value with no means to check. **A heartbeat is a CLAIM ABOUT LIVENESS and a stale one is indistinguishable from a current one by inspection.**

**CLOCKS AND SHELLS**

- **8h-ter. A STAMP TYPED IN THE SAME BREATH AS THE READ IS STILL TYPED BY FEEL.** `16:20Z` into a note whose turn read `16:17Z`; `21:58Z` rounded up off a `21:57:36Z` read, refused by the clock guard. **A clock value goes in only when the `date -u` that produced it is in this turn's output, verbatim.**
- **8h-bis. FOR A WORD-LEVEL CLAIM, COMPARE WORDS** (conflab-vc via devbin-vc).
- **8a. SHELL QUOTING EATS CONTENT AND THE COMMAND STILL SUCCEEDS.** Under zsh an unquoted glob ABORTS the whole call, an apostrophe inside single quotes runs NOTHING, and `cd` persists into the next call.
- **8r. A SUCCESS CHECK THAT PIPES REPORTS THE PIPE'S STATUS.** `if cmd | tail -2; then echo COMMITTED` prints COMMITTED whether or not the commit landed. **Capture the status before anything consumes it: `out=$(cmd 2>&1); rc=$?`. Anything downstream of the thing being tested becomes the thing being tested.**
- **8al+8aq. AN INSTRUMENT READS THE HARNESS, AND A SHELL REFUSAL ARRIVES AS AN ANSWER.**
- **8as. A `debug_assert` IS LOUD IN THE PROFILE NOBODY SHIPS AND SILENT IN THE PROFILE EVERYBODY RUNS.** **The profile is part of the instrument, and the two disagree in the direction that HIDES the defect.**

**SHARED TREE AND SCOPE**

- **8ab+8ae. IN A SHARED CHECKOUT, ONE NODE'S ORDINARY IN-PROGRESS STATE IS ANOTHER NODE'S OUTAGE.**
- **8l. ABSENT BINARIES ARE NOT AN OUTAGE UNTIL YOU HAVE ASKED WHETHER A BUILD IS RUNNING.**
- **8aa+8ad. A WINDOW IS SCOPED BY THE EXTRACT'S UNION, NOT BY THE VERB'S NAME** (`0210`).
- **8v. A CANON ATTACHMENT THAT IS ALSO A FORMATTED MARKDOWN FILE HAS TWO WRITERS AND THEY RUN AT DIFFERENT TIMES.**
- **8m. A GATE'S PROVENANCE LINE IS WRITTEN AFTER ITS LOG IS SEALED**, so it lands in a parent's record.
- **8aj. _NOTHING QUEUED FOR YOU_ IS A CLAIM ABOUT MY QUEUE, NOT THEIRS.**
- **8c+8c-bis. I MEASURE SOMETHING TRUE AND RULE ON SOMETHING WIDER.** And `--note` REPLACES a row's note wholesale and silently (`0207`, now guarded) -- **7803 characters destroyed before the refusal existed.**
- **8ak. A DEFAULT METHOD ON A TRAIT WITH FORWARDING IMPLEMENTATIONS IS A TRAP**, and the trap is in the forwarders.

## Decisions

**Standing rulings. Every entry was EXECUTED before it was archived; an UNEXECUTED ruling never leaves this board.**

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
