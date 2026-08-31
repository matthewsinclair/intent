---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-31 20:45Z
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
4. **WP-15's four criteria have NO OWNER** (hv's to assign); **`parity/register.md`'s 39 stale rows** (GENERATED -- the template is the fix); **`0136`** (hv ruled: after the tag); **`intent/wip.md`** before the tag.
5. **The re-pin rule in `estate_corpus.sh` has EXPIRED for EVERY MEMBER IT NAMES -- four, not the three I found** (cc's correction, in my favour). It names `canary`, `lamplight`, `utilz`, `baize` as repositories whose migrations have NOT run; all four now declare 3.0.0, **canary included, because canary IS this repo and it is self-hosted.** Re-pinning any to HEAD would point at a tree with no v2 source. Harness is ic's.
6. **`design.md:88`** still says `rmcp (official SDK): stdio ... now`, which my ruling reversed. Design prose is hv's hand -- a flag, not an edit.

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
