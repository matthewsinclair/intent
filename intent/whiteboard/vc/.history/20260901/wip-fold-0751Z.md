---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-31 21:59Z
status: active
focus: "FOLDED 2026-08-31 18:59Z, pre-fold sha-verified in .history/20260831/. STATUS STAYS active -- a compact does not end a session. THE NEXT RELEASE IS NOT A PATCH (13 commands, 30 flags at 0f5ee514) and the NUMBER IS hv's -- AND THE RELEASE IS ITSELF THE BLOCKER ON ST0068 AC-02.1, which I had filed as unbuilt work owned by dc when it had been built for four days. I understated my own figures to hv THREE times today, all in the direction nobody checks. RE-RUN EVERY FIGURE; READ NONE OFF THIS BOARD."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS. This section has drifted back into narrative three times now -- cut it again when it does.**

## DOING

### THE CUT -- RUN THE VERBS, NEVER READ A FIGURE OFF THIS BOARD

`intent ac gate <ST>`. **`intent --version` names the commit the binary was built from, which is NOT HEAD -- and that difference ALONE IS NOT STALENESS.** Currency is decided by whether any non-test file under `native/rust` moved since the commit the pair names; the gate's `self-provenance` currency arm prints the verdict on every commit. **Pin by the sha256 the gate prints, never by the marker** -- three distinct binaries carried one marker in a day.

**AND TWO ARTEFACTS BOTH ANSWER `3.0.0` WITH 1516 COMMITS BETWEEN THEM** (cc). So _does X ship in 3.0.0_ is UNANSWERABLE from the version string. That reaches any AC or release note citing a version.

### THE NEXT RELEASE IS NOT A PATCH, AND THE NUMBER IS hv's

`bash intent/st/ST0056/parity/tools/gen_cut_surface.sh`. **+13 commands, +28 flags over the tag.** dc drove behaviour, not declarations: the `daemon` family went from _known command not implemented yet_ to answering on a socket. **Every removal is hv-authorised -- I chased all three.** Recorded in hv's `Questions PUT to hv`, recommendation `v3.1.0`. **A release decision is hv's and a delegated pen does not transfer it.**

### `AC-00.16` -- SIDE C LANDED, RED FOR A NEW REASON

The per-property union exists. **What keeps it red is that the criterion's first conjunct is NOT MACHINE-DECIDABLE.** The fix is a structural declaration, which is an amendment, which is hv's -- **and a delegated pen does not cure it, because holding the pen does not change who benefits from my own red row going green.**

## TODO

1. **`AT-00.11` -- THE CONTROL IS FOUND, AND IT IS NOT A REVISION.** `of_n_closes_over_examined.sh` still does not exist; what changed 2026-08-31 20:35Z is that its blocking prerequisite is now answered by measurement.

   **`AC-00.11` CITES `c51f10d5` AS THE DEFECT LIVE. IT IS THE FIX COMMIT.** Its tree already carries the `EXAMINED 2 of 1` comment and the `comm -12` intersection that closes the hole. Its parent `4ba598f1` IS pre-fix -- and driven against its OWN tree in a worktree it printed `EXAMINED 2 of 278`, arithmetically clean. **So the row's own cited control does not reproduce, and neither does its parent.**

   **NO MAIN-LINE REVISION REPRODUCES IT.** The fix landed FOUR MINUTES after the defect commit (23:04 -> 23:08 on 2026-08-18) and both predate the live layout move (`16048f82`, 08:31 the next morning). The window in which the flat layout met the unfixed instrument **never existed on main** -- the historical `2 of 1` came from a fixture or a pre-move experiment, which is why reading the log was never going to find it. **The row told me to find the revision BY RUNNING IT; running it says there is none.**

   **THE CONTROL IS A CONSTRUCTED FIXTURE, AND IT IS DRIVEN: `EXAMINED 5 of 3 ... the other -2`.** Recipe, so nobody re-derives it: take `4ba598f1`'s `canon_commit_check.sh`; give it a HEAD whose RECORDED-attachment total is small (half-migrate -- delete every `thread.json` but one, and commit it); then COMMIT more files under the canon roots than that total. Pre-fix, `scoped` counts FILTER KEYS -- every changed file under the roots, attachment or not -- so it runs past `total` and the remainder goes negative.

   **TWO TRAPS THAT COST ME A PASS EACH, BOTH WORTH KEEPING.** `total` is read from the REVISION, not the worktree, so deleting files on disk moves nothing. And the narrowing reads `git diff-tree` on the **COMMIT**, not the index -- staging the files printed `EXAMINED 0 of 3` and looked like a refutation. **Both failures presented as the defect being absent.**

   **SO THE BUILD IS NOW SHAPED**: the instrument takes two trees, and the positive control is a fixture it constructs rather than a revision it checks out. `AC-00.11`'s `c51f10d5` citation should be corrected when someone next has the pen on that row -- it is load-bearing wording that sent me at a revision that cannot fail.

2. **`AT-00.2`, `AC-00.8`, `AC-00.10`** -- behind cc's WP-10, behind WP-06 and WP-07. Read off WP-10's cover by cc; sequence around it.
3. ~~The missing absence class in `capability_ship_check.sh`~~ **DONE 48533319.** It was a FOURTH, not a third, and the gap LIED ABOUT THE SUBSYSTEM: a present-executable-broken binary printed `NO DAEMON is answering` with remedy `intent daemon start`, measured by planting `#!/bin/sh; exit 1` on PATH. **Worse than the conflation cc named, because the other three tell the truth about the world they name.** Discriminator is `--version` -- a read needing no daemon, so a failure is a fact about the ARTEFACT. Controlled both ways; byte-identical on a working binary.
4. **WP-15's four criteria have NO OWNER** (hv's to assign); **`0136`** (hv ruled: after the tag); **`intent/wip.md`** before the tag.

5. **`parity/register.md` -- BLOCKED ON THE RECLASSIFICATION, NOT ON ME, AND REGENERATING IT NOW WOULD LAUNDER ITS STALENESS.** Measured 2026-08-31: `gen_register.sh` reads the COMMITTED `burn-baseline.tsv`, not a live burn. So regenerating today reproduces the SAME 2026-08-15 classifications (`c60cdbd`, 16 days old) while refreshing the _Measured at_ line -- **a no-op on substance that makes a stale measurement read as current.** That is the copied-figure-without-its-revision class, manufactured by the act of tidying.

   **The order is: reclassify the 8 `keep` rows (my `AC-06.1` ruling, cc executing) -> UNSTABLE drops -> a fresh baseline becomes installable -> THEN regenerate.** Regenerating at any earlier point is worse than leaving it visibly stale. `gen_register.sh:41` already says `burn.tsv` carries no revision and cannot know where it came from -- **the script documents this hazard and nothing enforces it.**

6. **CONDITIONAL, FIRES ONLY IF hv RULES `3.1.0`: `ST0064/info.md` carries hv's dated ruling record naming `3.0.1` three times.** It gets a SUPERSEDING NOTE -- hv's original verbatim with its date, then the new decision with ITS date -- and **NOTHING IS DELETED OR REWORDED.** ic caught that my own advice, applied as I phrased it, would have edited hv's words to keep them true; **a stale number is visibly stale and a smoothed record is not.** ic's own prose goes property-based, which is theirs. **This is the pen's constraint one thread over and I failed to apply it: the pen writes hv's word with attribution, it never revises hv's word.**
7. **The re-pin rule in `estate_corpus.sh` has EXPIRED for EVERY MEMBER IT NAMES -- four, not the three I found** (cc's correction, in my favour). It names `canary`, `lamplight`, `utilz`, `baize` as repositories whose migrations have NOT run; all four now declare 3.0.0, **canary included, because canary IS this repo and it is self-hosted.** Re-pinning any to HEAD would point at a tree with no v2 source. Harness is ic's.
8. **`design.md:88`** still says `rmcp (official SDK): stdio ... now`, which my ruling reversed. Design prose is hv's hand -- a flag, not an edit.

9. **OWED, DELIBERATELY NOT FILED TONIGHT: the INDEX layer of the three-layer property has neither a row nor a name.** The store layer has `0206` with a harness; the git layer has a discipline every node now knows; **the index layer has ONE observed instance and no measurement.** Filing it now would give it a row before anyone has driven it -- **the exact shape I spent this evening ruling against in other people's work.** It earns its row on evidence if it bites again before the tag. (dc's framing: contention is routable with a private index, COHERENCE is not, because the gate reads the index regardless of what a node names in its own commit.)

## Holds

**Each carries the CONDITION that releases it. A hold with no condition is an abandonment.**

- **`ST0068` AC-02.1 -- and the thread prefix is NOT decoration: `ST0056` ALSO has an `AC-02.1`** (a satisfied CI row about fmt + clippy). I wrote it bare on this board and then read my own note onto the wrong thread at pickup. **This is the version-string defect in miniature -- one name, two artefacts, so the identifier cannot answer which.** Prefix every criterion here.
- **CONDITION CORRECTED 2026-08-31 20:17Z: `--note` DID NOT NEED TO LAND. It landed 2026-08-27 at `6fa22a79`, four days before I called it a release blocker**, on all four `at` verbs, and it is in the delivered binary. `docs/getting-started.md:124,126,135` tell the reader to run it. **What is red is that a NEW USER INSTALLS THE KEG, and the keg is v3.0.0, which has no `--note`.** The row demands the journey be DRIVEN on a machine that has never seen this repo, following only published pages. **So the condition is a RELEASE, not a build, and it is hv's -- see hv item 1.**
- **`AT-07.5`'s behavioural arm is not re-verifiable while a daemon runs**, and one always is. Green with the caveat recorded; the tool REFUSES rather than fails, which is why it is not red. **Condition: hv authorises a daemon-down window.** A node must not take one.

## hv items

**Re-run `gen_cut_surface.sh`; do not read figures off this board.**

1. **CUT A RELEASE, AND ITS NUMBER. THIS IS ONE ITEM, NOT TWO -- it absorbs what I filed separately as "the `--note` blocker" and "the version number".** They were never two: `--note` is BUILT (`6fa22a79`, 2026-08-27), so nothing is waiting on dc, and the only thing standing between `ST0068` AC-02.1 and green is that **the published getting-started page instructs a command the installable artefact does not have.** Measured at `0f5ee514`: **+13 commands (2 removed), +30 flags (10 removed)** over the tag. A release decision is hv's and a delegated pen does not transfer it; recommendation stays `v3.1.0`.
2. **SCOPE, NOT A BLOCKER -- `--title` (+ `--severity`) on `issues edit`, a verb that does not ship at all.** dc building, cc green-lit. **They must never be sequenced as one** (dc's split, adopted).
3. **`AC-00.16`'s AMENDMENT.** Mine, and mine is why hv rules it.
4. _(folded into item 1 -- the version number and the release are the same decision.)_
5. **cc's MCP EXPOSURE question** on `issues edit --title` vs `--severity` -- dc's authored-text-versus-enumerated-field split is the shape, not the answer.
6. **Is a BEHAVIOUR-level register check in this cut?** New surface in a tag window; falls out of `AC-06.3`'s reword. Nothing checks an `as-observed` claim against behaviour, which is why that deviation was silent.
7. **Should the migrator COMMIT?** cc's `AC-10.6` exercise found it does not, so `migration.md`'s documented `git revert <migration-commit>` has no subject and one visible commit is an operator convention rather than a guarantee.

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**

## Watch-outs

**MECHANISMS ONLY. Instances are in `.history/`.**

1. **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED AND ITS OUTPUT LOOKS LIKE AN ANSWER.** **CONVERGENCE IS NOT CORROBORATION WHEN BOTH METHODS SHARE A DEFINITION -- OR AN INSTRUMENT** (cc): we both drove the tree binary, so agreement was one measurement reported twice. **A PROBE THAT CANNOT EXHIBIT THE FAILURE RETURNS THE NUMBER THAT MEANS SUCCESS.** **A ZERO WITH NO PLANTED POSITIVE IS A SILENCE.** **A RECORDED PROOF IS PROSE UNTIL SOMEONE TYPES IT, and its correct steady state is silence, so nothing reports the rot** -- a MANUAL instrument's green rots exactly this way; sweep them rather than trusting them. **TWO GREPS WITH DIFFERENT PATTERNS DISAGREE AND THE SECOND READS AS EXPLAINING THE FIRST.**

2. **AN ABSENCE NEVER LOOKS LIKE A BUG IN THE QUERY.** A partial read that finds SOMETHING self-corrects; one that finds NOTHING is unfalsifiable without redoing it. **Output to a FILE, then count.** **NEVER SUPPRESS A COMMIT'S ERROR OUTPUT** -- `>/dev/null 2>&1` in a retry loop turns every distinct failure into one silence.

3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING WATCHES THE JOIN**; the basis can also POSTDATE the subject. **A GREEN ROW WHOSE TEXT OVERCLAIMS IS WORSE THAN A RED.** **The repair splits on WHY: rewording to match an unfinished build is the defect; rewording because a RULING moved, or because the criterion was UNSATISFIABLE BY CONSTRUCTION, is the correct repair.** **AN AC REWORD THAT LEAVES ITS AT BEHIND WEAKENS THE CRITERION.** **`closed` DOES NOT MEAN FIXED.** **EXIT 2 SPANS COULD-NOT-RUN AND PROPERTY-IS-FALSE, and a refusal is not a failure** -- I counted my own usage error as a stale green until I read the message.

   **3g. STALE IN THE PESSIMISTIC DIRECTION IS THE ONE NOBODY GOES LOOKING FOR, AND IT IS MINE, NOT hv's.** Five instances in a day, three on my own board: the currency claim, _two flags on shipping verbs_ for a 13-command release, and `--note` on two verbs when it is four. **Structural: a board that OVERSTATES gets caught by the first person who relies on it; one that UNDERSTATES never prompts anyone to check.** **Cure is not vigilance -- a figure addressed to hv carries the VERB that regenerates it.**

4. **THE SHARED TREE MAKES ORDINARY OPERATIONS MEAN SOMETHING ELSE.** `git commit --only <explicit paths>`, **PATH-scoped not hunk-scoped**; post-verify against `git show --name-only` EQUALLING your intended set. **A guard whose population is the INDEX makes every node's commit depend on every other node's uncommitted state.** **AND AN UNCOMMITTED CHANGE OF YOURS LANDS IN A PEER'S COMMIT** -- my `AT-00.12` red went in under cc's message. **NEVER REMOVE AN INDEX LOCK; RETRY.**

   **4b. AN ATTACHMENT IS AUTHORED AND NO SYNC DIRECTION REWRITES IT** -- `intent st attach`, then commit file AND extract together. **THREE INSTANCES IN ONE SESSION, THE LAST TWO WITHIN AN HOUR OF WRITING THIS DOWN. KNOWING A RULE DOES NOT FIRE IT; ONLY A GUARD FIRES.** The canon guard catches it at COMMIT; it cannot see the window between edit and commit, where a peer's `sync --to-disk` renders the store over your disk and answers ok.

5. **A CRITERION IS OWNED BY WHOEVER CAN SATISFY IT AND MUST BE ABLE TO FAIL -- AND MUST ALSO BE ABLE TO PASS.** A universal negative over an open future can only ever be falsified: that is a trap, not a bar. **A PARTITION DERIVED FROM ITS PARTS CLOSES BY CONSTRUCTION.** **A MEMBER THE PROPERTY CANNOT BE EVALUATED OVER IS EXCLUDED WITH ITS REASON NAMED AND THE DENOMINATOR MOVING VISIBLY.** **THE AUTHOR OF A CHECK IS NOT A SAFE SOURCE FOR ITS OWN DENOMINATOR** -- but where a criterion says _named revisions_, the names are an ARTEFACT to be read, not a choice to be made.

   **5f. A CRITERION DERIVED FROM A REGISTER ANYONE CAN ADD TO RECEDES AT THE SPEED YOU WORK** (dc). **AC-02.3's population is LIVE, not pinned at the cut -- I checked.** Filing enlarges it. **Cure is procedural: file and disposition in ONE act** -- and telling a peer to file does the same to them, which I did to dc without saying so.

6. **MAKE THE BAD STATE UNREPRESENTABLE; WHERE YOU CANNOT, WITNESS THE MECHANISM.** **A predicate is sound only relative to what is DONE with the answer.** **WIREDNESS MUST BE DERIVED OR DRIVEN, NEVER A HAND-MAINTAINED FLAG.**

7. **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK AND A GREEN RUN NEVER EXERCISES IT.** **THE UNIVERSAL SAFE PROBE CAN HAVE A SIDE EFFECT**, sharpest as **A DEFECT WHOSE VERIFICATION REPRODUCES THE HARM** -- `intentd --help` starts a real daemon. **SCREEN AN INSTRUMENT FOR SIDE EFFECTS BEFORE DRIVING IT, and positive-control the screen.**

8. **vc's OWN.**

   **8a. SHELL QUOTING EATS CONTENT AND THE COMMAND STILL SUCCEEDS.** zsh does not word-split; an unmatched glob aborts the whole call; an apostrophe in a single-quoted program runs nothing; `$?` after a pipe reads the last stage. **`set -- $var` IN A LOOP: `$1` takes the whole string and `$2` is EMPTY.** **THE CONTROL IS THE LESSON: mine PASSED because it exercised the variable that could not be wrong.** A control must drive the variable that CAN be wrong, through the identical path, with a value that MUST fail.

   **8b. I RULE AGAINST STATES THAT HAVE ALREADY MOVED.** **RE-MEASURE BEFORE RULING. When a premise dies, SAY SO AND REVERSE IT.**

   **8c. A ROW MOVE CAN BE IRREVERSIBLE AND THERE IS NO UNDO VERB.** **The repair is the ARTEFACT the row was owed, never a label.**

   **8d. REPORTING IS NOT ROUTING**, and never perform an action a peer's session was denied.

   **8e. A RULING IN TRANSIT IS A RULING NOBODY HAS.** **Execution status is the discriminator, never the date.** **A finding living only in a peer message is discoverable by nobody. BEFORE PUTTING A QUESTION TO hv, CHECK WHETHER hv HAS ALREADY ANSWERED IT** -- and before reporting a finding, CHECK THE REGISTER: I nearly re-filed `0142`, my own, already fixed.

   **8f. I RELAY CLAIMS I HAVE NOT MEASURED, AND IT IS MY MOST PERSISTENT DEFECT.** **Reporting a peer's finding across a boundary is right; repeating it as FACT is the defect, and REASONING ONWARD from it is worse**, because the conclusion outlives the correction. **Its quieter form: relaying a figure STRIPPED OF THE LIMIT ITS OWN ARTEFACT PRINTS** -- I passed dc the 13 without the bolded _this is a DECLARATION, not a behaviour claim_ beside it.

   **8g. EVERY NODE COMMITS AS THE SAME PERSON AND PROVENANCE IS CARRIED BY PATH, NEVER BY AUTHOR.**

   **8h. A REVERT LANDS IN THE TREE AND TAKES EFFECT IN THE ARTEFACT, AND A BUILD NOBODY SCHEDULES SITS BETWEEN THEM.**

   **8i. A CLOCK VALUE GOES IN ONLY WHEN `date -u` IS IN THIS TURN'S TOOL OUTPUT, VERBATIM.** Any tool printing local (`ls`, `stat`, `git log`) makes a trailing `Z` an ASSERTION, not a format.

   **8j. I MOVED FROM A TRUE CONCLUSION TO A FALSE ONE BY REASONING BETTER.** **RE-MEASURE WITH AN INSTRUMENT THAT CAN ANSWER.**

   **8k. I ADDRESSED THREE OF FOUR MESSAGES TO THE WRONG NODE.** Body composed from a correct mapping, address typed from intention. **Pin the mapping to a FILE and read the address off the file.** **Peer names COLLIDE across projects** -- `laksa-cc` is not `intent-cc`.

   **8l. A MENTION IS NOT AN INSTANCE.** **Before counting matches, ask what ELSE produces this token, and WHICH SECTION it sits in** -- a heading can invert its contents. **Where one such section exists, sweep it rather than fixing the instance.**

## Decisions

**Standing rulings. Every entry here was EXECUTED before it was archived; an UNEXECUTED ruling never leaves this board.**

- **`AC-06.1`'s 8 KEEP ROWS RECLASSIFY, AND FILING IS NOT SHIPPING** (2026-08-31 20:40Z, on cc's establishment of all 27 UNSTABLE to four causes). Two rulings, one message.

  **THE RECLASSIFICATION IS MINE AND NOT hv's BECAUSE IT IS NOT A NEW DECISION** -- it is the bookkeeping of one already taken. `AC-12.1`'s prune was authorised and fail-forward is this project's law. A `keep` row ASSERTS v3 should have the command it exercises; for these eight that assertion is now FALSE, made so deliberately. **Leaving them `keep` is worse than the UNSTABLE they show, because it is silent and it reads as intent.** I refused this on 2026-08-30 with 2 causes known and 6 unknown; what changed is the unknowns, not the principle.

  **FILE B/C/D NOW, DO NOT HOLD THEM FOR THE RELEASE NUMBER. Filing is RECORDING: it changes no surface and cannot expand a release scope -- FIXING can, and that is a separate call.** A filing deferred to an unrelated decision is a second home for the record whose arrival is contingent on something it has nothing to do with. **That is the recorded-proof-is-prose-until-someone-types-it class, and deferring it inside a ruling ABOUT silence would be authoring a fresh instance while adjudicating the old one.**

  **VERIFIED BEFORE RULING, because a ruling on a borrowed measurement is where I was wrong twice today:** `125f601d` deleted 7 non-test files and ZERO tests; cause B is genuinely TWO HOMES, byte-identical, at `bin/intent_helpers:372` and `intent/plugins/claude/lib/rules_lib.sh:56`. **`config.bats` t7 is correct and finding a real Highlander violation -- not flaky, not over-reaching.** Both homes sit in the v2 tree the prune is eating, so the remedy may be _the prune completes_ rather than a code change.

  **AND A COMMENT WRITTEN TO PREEMPT A HUMAN REVIEWER CANNOT DISCHARGE AN OBLIGATION TO A MECHANICAL CHECK** (cc's, and it generalises). The hunk's comment predicted the finding and named the wrong finder: a TEST found it, and a test cannot read the comment written to answer it. Same shape as a REACH paragraph answering an objection no grep will ever raise.

- **v2 MESSAGE STRINGS ARE OUTSIDE THE NARROWED PARITY CONTRACT** (2026-08-31, on dc's re-drive of AT-05.2 finding 6 of 10 core `keep` files red under v3). **The six rows are MISCLASSIFIED and want `deviate`; they are not a v3 defect.**

  **`parity.md`'s in-scope list names VOICE, NOT WORDING** -- _stderr voice (`ok:`/`error:` lowercase, 0023)_ -- and the list is CLOSED by construction (_except where this contract says so, IN ADVANCE_ / _decided here, never discovered in triage_). Neither _stdout shape_ nor _behavioural semantics_ reaches a literal: shape is not content, and the behaviour holds.

  **THE SETTLING GROUND IS THAT 0023 IS CITED IN THE SCOPE LINE ITSELF.** That is the ruling which RETIRED the capitalised voice across 26 sites, hv-ruled on the batching principle. **A contract naming the wording-change decision as its definition of in-scope voice cannot also make the pre-change literals binding -- it would make v3 non-compliant FOR COMPLYING WITH 0023.** Driven, not quoted: v3 emits `error: no steel thread ST9999 in this project` plus a `remedy:` line -- correct voice, strictly MORE informative than the v2 string asserted. **The tests are the stale artefact, not the binary.**

  **THE CONDITION IS BINDING: `deviate` means RETARGET, NEVER DELETE.** Assertions removed rather than repointed convert a live regression net into silence -- the AC-06.3 defect reached through a reclassification instead of through neglect.

  **AND THE DURABLE HALF IS ABOUT THE REGISTER'S AXIS, NOT SIX FILES (dc's): BURN MEASURES WHETHER A TEST REACHES THE CLI, NOT WHETHER IT PASSES.** So `keep` has never meant _runs unmodified_, whatever the class name says. Known since cc's 8-of-31 on 2026-08-14; **what is new is that it reaches the CORE families, the one corpus `AC-05.2` claims.**

  **WP-05 STAYS `wip` -- dc's call and it is right.** AT-05.2's green does not become honest by re-declaring it; it becomes honest when the six carry retargeted assertions.

- **CORRECTION 2026-08-31, SUPERSEDING MY OWN `AC-06.1` RULING BELOW: ZERO ROWS RETIRE, NOT EIGHT.** The reclassification stands; **its TARGET CLASS was wrong and so was the reason I gave for it.**

  **`deviate`, not `retire`, for all 21 rows** (19 wired + 2 mixed + the 3 unwired). **The tests are not aimed at commands v3 removed -- they are aimed at a DEAD ENTRY POINT for verbs v3 KEEPS.** `125f601d` deleted the v2 PLUGIN SCRIPTS (`intent/plugins/claude/bin/intent_claude_*`), not the commands; v3 carries its own Rust `rules`, `skills` and `upgrade` and they answer.

  **AND `UNWIRED` IS NOT RETIREMENT.** `claude prime` and `claude subagents` return _a known command that is not implemented_ -- **the exact state the whole daemon family was in at the tag, and that family is now 4 of the 13 commands in the release delta.** Pending implementation, not departure.

  **THE SSOT SETTLES IT AND MY OWN EARLIER RULING MAKES IT BINDING**: `surface/dispatch-table.json` disposes EVERY `claude` verb `keep`, prime and subagents included -- and the register's vocabulary is _shared VERBATIM with the dispatch table_ (vc, 2026-08-14). **A register `retire` against a table `keep` is two artefacts contradicting each other in the one place I ruled they must agree, and NOTHING CROSS-CHECKS THEM** (filed).

  **MY ERROR IS DISTINCT FROM cc's AND WORSE IN ONE WAY. I verified the cheap half of their report -- `125f601d` deleted zero tests, true -- and treated that as verifying the INFERENCE built on it.** Verifying a premise is not verifying its conclusion, and **a ruling built that way FEELS driven**, which is why nothing in my own process caught it. cc caught it, then narrowed to 2; the answer was 0. **Each `deviate` row needs a D-number or carries `UNRATIFIED` loudly -- never an invented one.**

- **AN ARTEFACT NAMED IN A RULING MUST BE VERIFIED TO EXIST AT THE MOMENT OF WRITING** (2026-08-31). **TWICE TODAY A FORMULATION OF MINE SENT A NODE AT SOMETHING THAT IS NOT THERE**, and both were found by a peer DRIVING rather than reading:

  - **`AC-00.11` cites `c51f10d5` as _the defect live_. It is the FIX commit.** No main-line revision reproduces the defect at all.
  - **My app model said _the registry's root_. There is no registry.** `~/.intent/config.json` holds `intent_version`, `author`, `intent_dir` -- nothing registers project roots.

  **A PHRASE OF MINE BECOMES A SPEC THE MOMENT SOMEONE BUILDS AGAINST IT.** Both were loose talk that read as design, and neither cost anything only because ic and I checked before building. **This is the same failure as the premise-versus-conclusion error above, one step earlier: there I verified the wrong thing, here I verified nothing and it still read as authoritative.**

- **ST0064 PROJECT ROOT: RULED (a) -- THE APP STORES A CONFIGURED ROOT AND SETS THE CHILD'S CWD** (2026-08-31, ic's question, ic's recommendation, and the measurement inverted the argument I expected to make). **CORRECTED WITHIN THE HOUR: `D07` RATIFIES A REGISTRY** -- _One intentd per machine, N projects, per-project DBs, REGISTRY_. I measured `~/.intent/config.json`, found none, and ruled that none EXISTS -- reading today's filesystem as the design. **(a) still stands but its strongest ground is void**: (b) would not INVENT a global fact, it would build a ratified-but-unbuilt D07 component. What survives is SEQUENCING, not novelty -- building D07's registry is release scope and hv's.

  **AND (b) IS THE HIGHLANDER VIOLATION, NOT (a)**: a machine-level registry query introduces a SECOND resolution mechanism alongside CWD walk-up, which the two would then have to agree on forever with nothing checking -- **a fresh instance of `0204`, which I filed an hour earlier.** (b) also asserts the machine has _a_ project, false the moment a second exists, and this fleet is fourteen. **CONDITION, NOW TWO CLAUSES: (i) the configured root is VALIDATED as an Intent project when set, and the app refuses LOUDLY if not; (ii) the store is marked IN THE CODE as an INTERIM standing in for `D07`'s registry, naming D07, so that when the registry lands the app READS it rather than keeping a parallel home. A short-lived second home that knows it is second is fine; one that forgets is `0204`.** -- otherwise children spawn where walk-up finds a different project and a correctly-working CLI emits confusing errors.

- **THREE TIMES TODAY I MEASURED SOMETHING TRUE AND RULED ON A BROADER QUESTION THAN THE MEASUREMENT ANSWERED.** This is the session's own defect and it is not the same as being careless -- **every one of the three measurements was correct, which is what made each ruling feel driven.**

  - **`AC-06.1`:** verified `125f601d` deleted zero tests (TRUE) -> ruled the commands were gone (FALSE; it deleted the plugin SCRIPTS).
  - **`AT-00.11`:** read `c51f10d5` cited as the defect live -> it is the FIX commit.
  - **`ST0064`:** measured `~/.intent/config.json` holds no registry (TRUE) -> ruled no registry EXISTS (FALSE; `D07` ratifies one, unbuilt).

  **THE COMMON FACTOR IS THAT THE MEASUREMENT ANSWERED A NARROWER QUESTION THAN THE RULING.** A spot check confirms the narrow fact and licenses nothing. **All three were caught by a PEER driving, never by me re-reading -- so the corrective is not more care, it is stating what the measurement does NOT establish before ruling on it.**

- **`D10` RATIFIES THE PLUGIN-SCRIPT PRUNE; the 24 `deviate` rows cite it and drop `UNRATIFIED`** (2026-08-31). **TRACED, NOT PICKED, and that distinction is the whole ruling**: `D10` reads _shell pruned at the cut_; `AC-12.1` reads _the shell implementation is pruned at the cut_ -- **the operative clause WORD FOR WORD**; and `125f601d`'s own title is _AC-12.1's prune lands_. Three links, each checkable. **`lib_classify.sh` forbids supplying a PLAUSIBLE reference -- resemblance is not evidence, a shared operative clause is. Had D10 merely MENTIONED shell, this would have gone to hv to mint a new one.** Minting is a design act and hv's hand; ruling that an existing ratified decision covers a case is adjudication and mine.

  **AND THE CITATION IS WHAT MAKES TRIMMING THE NOTES SAFE:** once every row cites `D10` the shared argument has a named home, so a per-row note repeating it is a second copy of a ratified decision. Each note keeps only what is true of ITS row. **A D-number is a citation with a referent, not the absence-as-meaning this estate rejects.**

  **cc's COUNT IS 24 AND MINE SAID 21** -- my own clauses read 19 + 2 + 3 and I totalled them wrong in the same message. Three rows would have stayed silently `keep`.

- **THIS ESTATE DOCUMENTS ITS OWN MECHANISMS CORRECTLY AND THEN APPLIES THE FIX ONE FIELD SHORT** (2026-08-31, **THREE** independent instances in one day, **all three routed from Laksa, from unrelated directions**). **No one of them would have suggested a class.**

  - **`whiteboard-clock-guard.sh`** explains at length that BSD fills an unspecified time field from the clock, fixes it for the decision-DATE path (`<date> 00:00`), and **leaves `%S` unspecified on the HH:MM path the guard spends all its time in** -- refusing honest stamps on macOS. Fixed `e0cdc389`.
  - **`apply_envelopes`** carries the guard added after Lamplight lost 4934 bytes of issue prose, whose comment states the principle exactly -- _against the DISK rather than against `self.canon`, because `self.canon` is what this write is derived from, so comparing to it can only ever agree with itself_ -- **scoped to one authored body instead of to the record.** The reasoning generalises verbatim and nobody generalised it. Filed `0206`.

  - **`0209` -- `st start` realises from canon WITHOUT ASKING THE DISK.** On an unhydrated thread it wrote 9493 bytes where the tree held 14789, at the canonical path, while the complete original stayed tracked elsewhere. **`apply_envelopes` had already derived the rule one scope down** -- compare against the DISK, because `self.canon` is what the write is derived from and can only agree with itself.

  **THE COMMON SHAPE: THE PROSE IS RIGHT, THE SCOPE OF THE REMEDY IS THE AUTHOR'S CURRENT BUG.** A comment this good reads as coverage -- **nobody re-derives a mechanism that is already written down beside them**, so the narrow fix inherits the wide explanation's credibility. **Detection is not "read more carefully"; it is asking of any documented mechanism WHICH CALL SITES THE REMEDY ACTUALLY REACHES.**

- **`0206` -- CANON VERBS ARE A READ-MODIFY-WRITE WITH NO COMPARE-AND-SWAP** (HIGH, filed `fdbf22e7`, routed by laksa-vc from a real loss on Laksa ST0111). `facade.rs:5215`: `self.canon.clone()` -> mutate one field -> apply the whole record, over a snapshot loaded at `Facade` construction. **THE UNIT OF THE RACE IS THE THREAD** -- `apply_envelopes` diffs and writes only threads that differ, so cross-thread work is safe and **`ST0056` is the exposed one, written by four nodes today.**

  **WORSE THAN A HAND-EDIT RACE BECAUSE THE CLI IS WHAT PEOPLE ARE TOLD TO USE INSTEAD** -- hand-editing is documented unsafe, the CLI carries the identical race and feels safe, and this project SHIPS the concurrency pattern that triggers it. **NOT CLAIMED: no lost write is demonstrated in THIS repo -- the mechanism is read here, the loss was observed on Laksa.** A harness planting two concurrent edits on one thread is the missing evidence and must exist before any fix is banked. **Interim discipline: announce before a canon verb on a shared thread, and commit in the same breath.** No code fix tonight -- a concurrency change to the write path in a release window is hv's.

- **ONE PROPERTY WITH THREE HOMES, AND NONE OF THE THREE REPORTS ITSELF** (dc's generalisation, 2026-08-31, and it is the best one anyone made tonight):

  | layer     | instance                                      | evidence                                                |
  | --------- | --------------------------------------------- | ------------------------------------------------------- |
  | **store** | canon writes racing                           | `0206` -- 19 of 20 on the SHIPPED binary                |
  | **git**   | a canon commit silently carrying peers' rows  | any canon commit; declared only if the committer looks  |
  | **index** | a staged instrument racing its own roster row | the gate judges what the INDEX holds, not what you name |

  **EACH IS TWO HALVES OF ONE WRITE AT A DIFFERENT LAYER; EACH LEAVES A STATE THAT IS INTERNALLY VALID; AND IN ALL THREE THE ESTATE THAT GETS JUDGED IS ONE NO NODE INTENDED TO EXIST.** Not three incidents -- one property.

  **CONTENTION CAN BE ROUTED AROUND WITH A PRIVATE INDEX; COHERENCE CANNOT** (dc), because the gate reads the index regardless of what any node names in its own commit. **The remedy is the same sentence at all three layers -- announce, commit in the same breath -- which is either elegant or damning. vc reads it as damning: a discipline re-derived at every layer is one the TOOLING should carry.**

- **THE ESTATE HAS THREE DELIMITER SHAPES FOR AUTHORED PROSE IN SHELL DATA BLOCKS, AND `0205` NAMED TWO** (cc, found while landing into `runner_roster_check.sh`): a **quoted heredoc** is safe; a **single-quoted** block is safe against `"` and vulnerable to an APOSTROPHE; a **plain double-quoted** block is the filed hazard. **The roster is single-quoted, so cc wrote their row with zero apostrophes deliberately.** Same class, opposite delimiter -- and an apostrophe inside a single-quoted shell string is a trap this estate already knows from the `perl -e` case.

- **A DIFF OF ONE ARTEFACT ACROSS A VERB CANNOT TELL A REGRESSION FROM A MIGRATION -- BOTH LOOK LIKE CONTENT LEAVING. THE DISCRIMINATING COMPARISON IS AGAINST THE POPULATION.** (laksa-cc, 2026-08-31, correcting `0209` after I had already corrected it once.)

  The 58 `acceptance.md` lines read as loss because the instrument was a before/after on ONE file. Measured across every thread, they are **pre-v3 boilerplate the v3 renderer deliberately does not emit** -- every v3-active thread lacks them and one lost the same blocks at a deliberate migration with nothing lost. **A format change and a regression are indistinguishable from inside a single artefact's history.**

  **AND IT IS THE SAME RULE `apply_envelopes` ALREADY STATES AT A DIFFERENT SCALE:** _`self.canon` is what this write is derived from, so comparing to it can only ever agree with itself._ **Compare against something INDEPENDENT of the thing under test** -- the disk for a write, the population for a diff. **That is the general form of every error I made today**: I compared a thing to itself or to its own narrow measurement and concluded about a class.

- **A RECLASSIFICATION IS A CLAIM ABOUT WHY, AND A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL** (2026-08-31). **Three rulings today turned out to be this one rule in different clothes** -- worth stating once rather than three times.

  - **dc's `deviate` (AT-05.2's six core files): RETARGET, never DELETE.** Assertions removed rather than repointed convert a live regression net into silence.
  - **cc's `retire` (AC-06.1's eight `keep` rows): CARRY THE BASIS, not just the class.**
  - **`AC-06.3` GOES GREEN on the reclassification plus cc's three filings -- CONDITIONAL on WHERE the reasons live: the `OVERRIDES` table at `lib_classify.sh:208`**, format `<basename>|<class>|<basis>|<note>`. **Not a parity doc and not a commit message, and the reason is in the criterion's own evidence clause: _register diff history shows land-time recording_.** A deviation recorded anywhere the register is not GENERATED FROM cannot be checked against register diff history -- the row would be uncollectable by construction, which `lib_classify.sh`'s ratification block records having already happened once.

  **THE PRECEDENT EXISTS AND NOBODY HAS TO INVENT THE SHAPE:** `treeindex_commands.bats|retire|hv ruling 2026-08-15|...` and `organize_commands.bats|retire|hv ruling 2026-08-14|...`, both reading _Classified by ruling, not by burn_, and treeindex's note draws cc's exact distinction -- **this file does not die with the shell, it dies with the COMMAND.**

- **ANY CANON COMMIT ON THIS ESTATE IS SILENTLY A MULTI-NODE COMMIT** (cc's, and it supersedes my _deadlock_ framing). Canon regenerates wholesale from the store and cannot be split, so `git commit` takes every peer row that has entered since the last canon commit. **Whether that gets DECLARED depends entirely on whether the committer happens to look.** **The deadlock is the VISIBLE case and therefore the benign one** -- the real failure is the commit that sails through with nobody noticing. Remedy remains a sentence: say which option you are taking.

- **THE MCP RULESET.** MCP tools call the FACADE, never the CLI dispatch arm. A parent row is a namespace, not a verb. The test for exposure is NEED, not provenance. **A vocabulary reserved for a population that measures EMPTY is STRUCK.** `exposed_on_mcp` refuses on absence. **A tool that can only mutate inverts the safety gradient.** SERVED implies a CLI read to agree with.
- **`close --note` IS NOT BUILT.** `issues edit --from` then `issues close` IS that act. **I originally wrote that it ALREADY SHIPS and that was false** -- `issues edit` does not exist on the keg; I drove it on the tree. **The ruling stands on a weaker warrant than I gave: contingent on the next release, not independent of it.**
- **AN ISSUE'S AUTHOR DISPOSITIONS IT.**
- **`AC-06.3` IS REWORDED, NOT WITHDRAWN, AND THE NEW FORM IS HARDER.** _None discovered after_ is a universal negative over an open future. Ruled form: **every KNOWN deviation is recorded -- a `keep`/`as-observed` row found to differ is a recorded deviation or a filed defect, never silence.** **Deliberately not shaped to what the current state satisfies.**
- **A CLONE AT A PINNED REVISION IS `FOR REAL`.** `for real` opposes SIMULATED, not CLONED. What a clone omits is the live tree with peers on it, which is not evidence -- it is what makes the exercise destructive.
- **A CRITERION THAT COULD FORCE SCOPE IS WRITTEN AS AGREEMENT, NOT COVERAGE.**
- **A DELETE HAS THREE POPULATIONS AND EVERYONE ASKS ONLY THE FIRST:** what EXECUTES this; what CITES this as evidence -- **population is the STORE, not the tree**; what CHECKS this.
- **AN IDENTIFIER IS ONLY UNIQUE WITHIN ITS SCOPE, AND EVERY BOARD WRITES IT BARE.**
- **A SECOND HOME IS NEVER ACCEPTABLE AT A TAG.** The escape is not _accept two homes_, it is _need less machinery_.
- **A CITATION'S AUTHORITY COMES FROM ITS MEMBERSHIP RULE, NEVER FROM ITS NAME.**
- **DERIVED CENSUSES MULTIPLY FREELY; AUTHORITATIVE COPIES DO NOT.**
- **A TEST GOING RED BECAUSE A FIX LANDED IS THE NOTIFICATION WORKING.**
- **THE DAEMON'S PUBLISHED PORT SERVES BOTH PROTOCOLS, DISAMBIGUATED AT BYTE 0.**
- **THE MANIFEST IS A SHARED MEASUREMENT SURFACE, NOT A SINGLE-WRITER FILE** (dc's, adopted). Rows come from whoever drove the fact; **the door is `st attach` because the disk file is a VIEW.**

**STATUS STAYS `active` THROUGH THIS COMPACT.**
