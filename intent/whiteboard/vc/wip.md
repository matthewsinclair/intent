---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-09-01 07:53Z
status: active
focus: "FOLDED 2026-09-01 07:53Z, pre-fold verbatim + sha-verified at .history/20260901/wip-fold-0751Z.md (82bbcb52, 40649 bytes). RE-DRIVE EVERY FIGURE; READ NONE OFF THIS BOARD. hv's desk is TEN. Yesterday's own defect: five times I measured something TRUE and ruled on something WIDER -- every one caught by a peer driving, never by me re-reading."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS and UNEXECUTED WORK.**

## DOING

### THE CUT -- RUN THE VERBS, NEVER READ A FIGURE OFF THIS BOARD

`intent ac gate <ST>`; `bash intent/st/ST0056/parity/tools/gen_cut_surface.sh`. **`intent --version` names the commit the binary was built from, which is NOT HEAD -- and that difference ALONE IS NOT STALENESS.** Currency is whether any non-test file under `native/rust` moved since that commit; the gate's self-provenance arm prints the verdict. **Pin by the sha256 the gate prints, never by the marker.**

**AND TWO ARTEFACTS BOTH ANSWER `3.0.0` WITH 1516 COMMITS BETWEEN THEM** (cc), so _does X ship in 3.0.0_ is unanswerable from the version string. That reaches any AC or release note citing a version.

### THE NEXT RELEASE IS NOT A PATCH, AND THE NUMBER IS hv's

**+13 commands (2 removed), +30 flags (10 removed)** over the tag at `0f5ee514`. The `daemon` family went from _known command not implemented_ to answering on a socket. **Every removal is hv-authorised -- I chased all three.** Recommendation `v3.1.0`. **A release decision is hv's and a delegated pen does not transfer it.**

### `AC-00.16` -- SIDE C LANDED, RED FOR A NEW REASON

The per-property union exists. **What keeps it red is that the criterion's first conjunct is NOT MACHINE-DECIDABLE.** The fix is a structural declaration, which is an amendment, which is hv's -- **and a delegated pen does not cure it, because holding the pen does not change who benefits from my own red row going green.**

## TODO

1. **`AT-00.11`'s instrument `of_n_closes_over_examined.sh` DOES NOT EXIST -- it is a BUILD, and its blocking prerequisite is now ANSWERED.** `AC-00.11` cites `c51f10d5` as _the defect live_; **it is the FIX commit**, and its parent `4ba598f1` drives arithmetically clean (`EXAMINED 2 of 278`). **NO MAIN-LINE REVISION REPRODUCES IT** -- the fix landed FOUR MINUTES after the defect commit, both predating the layout move. **The control is a CONSTRUCTED FIXTURE and it is driven: `EXAMINED 5 of 3 ... the other -2`.** Recipe: `4ba598f1`'s `canon_commit_check.sh`; a HEAD whose RECORDED-attachment total is small (half-migrate, one `thread.json`, committed); then COMMIT more files under the canon roots than that total. **Two traps, both presenting as the defect being ABSENT: `total` reads the REVISION not the worktree, and the narrowing reads `git diff-tree` on the COMMIT not the index.** `AC-00.11`'s citation wants correcting by whoever next holds the pen on that row.
2. **`AT-00.2`, `AC-00.8`, `AC-00.10`** -- behind cc's WP-10, behind WP-06 and WP-07. Sequence around it.
3. **WP-15's four criteria have NO OWNER** (hv's); **`0136`** (hv ruled: after the tag); **`intent/wip.md`** before the tag.
4. **`parity/register.md` -- BLOCKED ON THE RECLASSIFICATION, AND REGENERATING IT NOW WOULD LAUNDER ITS STALENESS.** `gen_register.sh` reads the COMMITTED `burn-baseline.tsv`, not a live burn, so regenerating reproduces the SAME 2026-08-15 classifications while refreshing the _Measured at_ line. **Order: reclassify -> UNSTABLE drops -> fresh baseline installable -> THEN regenerate.**
5. **CONDITIONAL, FIRES ONLY IF hv RULES `3.1.0`: `ST0064/info.md` carries hv's dated ruling naming `3.0.1` three times.** It gets a SUPERSEDING NOTE -- hv's original verbatim with its date, then the new decision with ITS date. **NOTHING DELETED OR REWORDED.** ic caught that my own advice would have edited hv's words to keep them true.
6. **The re-pin rule in `estate_corpus.sh` has EXPIRED for all FOUR members it names** -- `canary`, `lamplight`, `utilz`, `baize` all declare 3.0.0 now, canary included because canary IS this repo. Re-pinning any to HEAD points at a tree with no v2 source. Harness is ic's.
7. **`design.md:88`** still says `rmcp (official SDK): stdio ... now`, which my ruling reversed. Design prose is hv's hand -- **a flag, not an edit.**
8. **OWED, DELIBERATELY NOT FILED: the INDEX layer of the three-layer property has neither a row nor a name.** One observed instance, no measurement. **Filing it now would give it a row before anyone drove it** -- the shape I spent yesterday ruling against in others' work. It earns a row on evidence.

## Holds

**Each carries the CONDITION that releases it. A hold with no condition is an abandonment.**

- **`ST0068` AC-02.1 -- and the thread prefix is NOT decoration: `ST0056` ALSO has an `AC-02.1`** (a satisfied CI row). I read my own bare note onto the wrong thread. **CONDITION: A RELEASE, NOT A BUILD.** `--note` landed `6fa22a79` on 2026-08-27, four days before I called it a blocker; `docs/getting-started.md:124,126,135` instruct it; the keg is v3.0.0 and has none. **hv item 1.**
- **`AT-07.5`'s behavioural arm is not re-verifiable while a daemon runs**, and one always is. Green with the caveat; the tool REFUSES rather than fails. **Condition: hv authorises a daemon-down window. A node must not take one.**

## hv items -- TEN

**Re-run the verbs; do not read figures off this board.**

1. **CUT A RELEASE, AND ITS NUMBER.** Absorbs the old _`--note` blocker_ and _version number_ items -- they were never two. **+13 commands, +30 flags.** Recommendation `v3.1.0`.
2. **`0206` -- DOES A DATA-LOSS RACE BLOCK THE TAG? 19 of 20 on the SHIPPED binary, 19 call sites.** Two concurrent canon verbs on one thread lose a write silently; `git status` reports nothing. **vc recommends SHIP + name it + no concurrency fix in the tag window.**
3. **THE `daemon` FAMILY SHIPS WITH ZERO BURNING CONFORMANCE COVERAGE** -- 4 of the 13 new commands. Not a v2-parity gap, because v2 had no daemon. **Caveat: the BATS estate does not reach it; that is NOT _untested_.**
4. **`config`'s HOLE HAS AN EXPIRY.** `AC-06.1` requires its conformance test BEFORE the behaviour is designed -- an ORDERING constraint, so writing it later FAILS the clause.
5. **WHO OWNS `WP-14`?** `AC-10.5` chains behind it; Not Started, `L`, claimed by nobody. `WP-13` (XL) and `WP-15` (L) also unowned.
6. **`AC-00.16`'s AMENDMENT.** Mine, and mine is why hv rules it.
7. **SCOPE: `--title`/`--severity` on `issues edit`, a verb that does not ship.** **BUILT at `f6d37b18`** -- so the question is no longer whether to build it but whether it belongs in this cut. Part of item 1.
8. **cc's MCP EXPOSURE question** on `issues edit --title` vs `--severity` -- dc's authored-text-versus-enumerated-field split is the shape, not the answer.
9. **Is a BEHAVIOUR-level register check in this cut?** Falls out of `AC-06.3`'s reword; nothing checks an `as-observed` claim against behaviour.
10. **Should the migrator COMMIT?** `AC-10.6` found it does not, so `migration.md`'s `git revert <migration-commit>` has no subject.

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**

## Watch-outs

**MECHANISMS ONLY. Instances are in `.history/`.**

1. **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED AND ITS OUTPUT LOOKS LIKE AN ANSWER.** **CONVERGENCE IS NOT CORROBORATION WHEN BOTH METHODS SHARE AN INSTRUMENT.** **A PROBE THAT CANNOT EXHIBIT THE FAILURE RETURNS THE NUMBER THAT MEANS SUCCESS.** **A ZERO WITH NO PLANTED POSITIVE IS A SILENCE.** **A RECORDED PROOF IS PROSE UNTIL SOMEONE TYPES IT** -- a MANUAL instrument's green rots exactly this way.
2. **AN ABSENCE NEVER LOOKS LIKE A BUG IN THE QUERY.** Output to a FILE, then count. **NEVER SUPPRESS A COMMIT'S ERROR OUTPUT.**
3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING WATCHES THE JOIN.** **A GREEN ROW WHOSE TEXT OVERCLAIMS IS WORSE THAN A RED.** **`closed` DOES NOT MEAN FIXED.** **EXIT 2 SPANS COULD-NOT-RUN AND PROPERTY-IS-FALSE.**

   **3g. STALE IN THE PESSIMISTIC DIRECTION IS THE ONE NOBODY GOES LOOKING FOR, AND IT IS MINE.** Six instances now, latest: I put `0206` on hv's board at 6-in-10 when it is 19-in-20. **A board that OVERSTATES gets caught by the first person who relies on it; one that UNDERSTATES never prompts anyone to check.** **Cure: a figure addressed to hv carries the VERB that regenerates it.**

4. **THE SHARED TREE MAKES ORDINARY OPERATIONS MEAN SOMETHING ELSE.** `git commit --only <explicit paths>`, **PATH-scoped not hunk-scoped**. **AN UNCOMMITTED CHANGE OF YOURS LANDS IN A PEER'S COMMIT.** **NEVER REMOVE AN INDEX LOCK; RETRY.**

   **4b. AN ATTACHMENT IS AUTHORED AND NO SYNC DIRECTION REWRITES IT** -- `intent st attach`, then commit file AND extract together. **KNOWING A RULE DOES NOT FIRE IT; ONLY A GUARD FIRES.**

   **4c. ONE PROPERTY, THREE HOMES, AND NONE REPORTS ITSELF** (dc): canon writes racing at the STORE (`0206`), a canon commit carrying peers' rows at GIT, a staged instrument racing its roster row at the INDEX. **Contention is routable with a private index; COHERENCE is not.** Remedy at all three: **announce, and commit in the same breath.**

5. **A CRITERION MUST BE ABLE TO FAIL -- AND ALSO TO PASS.** A universal negative over an open future is a trap, not a bar. **A MEMBER THE PROPERTY CANNOT BE EVALUATED OVER IS EXCLUDED WITH ITS REASON NAMED AND THE DENOMINATOR MOVING VISIBLY.**
6. **MAKE THE BAD STATE UNREPRESENTABLE; WHERE YOU CANNOT, WITNESS THE MECHANISM.** **WIREDNESS MUST BE DERIVED OR DRIVEN, NEVER A HAND-MAINTAINED FLAG.**
7. **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK AND A GREEN RUN NEVER EXERCISES IT.** **SCREEN AN INSTRUMENT FOR SIDE EFFECTS BEFORE DRIVING IT** -- `intentd --help` starts a real daemon.
8. **vc's OWN.**

   **8a. SHELL QUOTING EATS CONTENT AND THE COMMAND STILL SUCCEEDS.** zsh does not word-split; an unmatched glob aborts the call; an apostrophe in a single-quoted program runs nothing; `$?` after a pipe reads the last stage. **THE CONTROL IS THE LESSON: mine PASSED because it exercised the variable that could not be wrong.** **AND THE ESTATE HAS THREE DELIMITER SHAPES FOR AUTHORED PROSE IN SHELL DATA BLOCKS** (cc): a quoted HEREDOC is safe; a SINGLE-quoted block is safe against `"` and vulnerable to an APOSTROPHE; a plain DOUBLE-quoted block is the filed hazard (`0205`, population FOUR -- the fourth is an EXTENSIONLESS executable an `--include='*.sh'` sweep structurally cannot see). **`bash -n` PASSES on a block whose MEANING changed, because the result is still valid shell.**

   **8b. I RULE AGAINST STATES THAT HAVE ALREADY MOVED. RE-MEASURE BEFORE RULING; when a premise dies, SAY SO AND REVERSE IT.**

   **8c. I MEASURE SOMETHING TRUE AND RULE ON SOMETHING WIDER. FIVE TIMES ON 2026-08-31, EVERY ONE CAUGHT BY A PEER DRIVING RATHER THAN BY ME RE-READING.** `AC-06.1` (verified the prune deleted no tests -> ruled the commands gone), `AT-00.11` (cited revision is the FIX), `ST0064` (no registry ON DISK -> ruled none EXISTS; `D07` ratifies one), my own `AC-06.3` text (cited `--help` as proof a command ANSWERS), `0209` (a size delta -> a size PREDICATE, which is a dead check). **The measurement was TRUE every time, which is what made each ruling feel driven.** **Corrective: state what the measurement does NOT establish, before ruling on it. It fired once -- I nearly told cc their harness was on the wrong path and checked one level deeper first.**

   **8c-bis. `--note` REPLACES A ROW'S NOTE WHOLESALE AND SILENTLY (`0207`), AND I DESTROYED 7803 BYTES OF `AT-00.12`'s PROVENANCE THAT WAY WITHOUT NOTICING.** Absent KEEPS, present OVERWRITES, totally -- on `at red`/`green`/`na`. **The verb that records WHY A ROW MOVED deletes WHY IT EXISTS, at the moment you are most likely to type it.** Live exposure: `AT-11.5` 16977 bytes, `AT-04.6` 13007, `AT-03.15` 8831. **Until it is fixed: read the note first and APPEND.** `ac satisfy` is NOT in the population -- the state machine forbids a second write, so that side is safe BY CONSTRUCTION rather than by care.

   **8d. A PHRASE OF MINE BECOMES A SPEC THE MOMENT SOMEONE BUILDS AGAINST IT.** _the registry's root_ named a thing that does not exist. **An artefact named in a ruling must be verified to exist at the moment of writing.**

## Decisions

**Standing rulings. Every entry was EXECUTED before it was archived; an UNEXECUTED ruling never leaves this board. Full reasoning for anything dated 2026-08-31 is in `.history/20260901/wip-fold-0751Z.md`.**

- **COMPARE AGAINST SOMETHING INDEPENDENT OF THE THING UNDER TEST.** The disk for a write; the POPULATION for a diff. **A diff of one artefact across a verb cannot tell a REGRESSION from a MIGRATION -- both look like content leaving** (laksa-cc). Same rule `apply_envelopes` states: _`self.canon` is what this write is derived from, so comparing to it can only ever agree with itself._ **This is the general form of every 8c error.**
- **THE ESTATE DOCUMENTS ITS MECHANISMS CORRECTLY AND APPLIES THE FIX ONE FIELD SHORT.** Three instances, all routed from Laksa: the clock guard (BSD's unspecified field, fixed for DATES, left on HH:MM), `0206` (compare-against-disk, scoped to an authored BODY not the record), `0209` (realise without asking the disk). **A comment that good reads as coverage. Detection: ask which CALL SITES the remedy reaches.**
- **A RECLASSIFICATION IS A CLAIM ABOUT WHY; A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.** `deviate` RETARGETS never deletes; `retire` CARRIES ITS BASIS. **Reasons live where the register is GENERATED FROM (`OVERRIDES`, `lib_classify.sh:208`), because `AC-06.3`'s evidence clause is _register diff history_.**
- **ZERO ROWS RETIRE, NOT EIGHT** (superseding my own ruling). `125f601d` deleted the v2 PLUGIN SCRIPTS, not the commands. **UNWIRED IS NOT RETIREMENT** -- `prime`/`subagents` return _a known command that is not implemented_, the daemon family's state at the tag. **`surface/dispatch-table.json` disposes every `claude` verb `keep`, and the register shares that vocabulary VERBATIM** (vc 2026-08-14), so a register `retire` against a table `keep` is two artefacts contradicting each other where I ruled they must agree. **Nothing cross-checks them -- filed `0204`.** Class is `deviate`, 24 rows.
- **`D10` RATIFIES THE PLUGIN-SCRIPT PRUNE. TRACED, NOT PICKED:** `D10` reads _shell pruned at the cut_, `AC-12.1` reads _the shell implementation is pruned at the cut_ -- **the operative clause word for word** -- and `125f601d`'s title is _AC-12.1's prune lands_. **Resemblance is not evidence; a shared operative clause is.** Minting a NEW D-number is a design act and hv's; ruling that an existing one covers a case is adjudication and mine.
- **v2 MESSAGE STRINGS ARE OUTSIDE THE NARROWED PARITY CONTRACT.** `parity.md`'s in-scope list names stderr VOICE, not wording, **and cites `0023` -- the ruling that RETIRED the capitalised voice.** A contract naming the wording-change decision as its definition of voice cannot also bind the pre-change literals; that would make v3 non-compliant FOR COMPLYING. **`deviate` means RETARGET, never delete.**
- **ST0064 PROJECT ROOT: (a) -- the app stores a configured root and sets the child's CWD.** **`D07` RATIFIES A REGISTRY** and it is unbuilt, so the ground is SEQUENCING not novelty. (b) would add a SECOND resolution path beside CWD walk-up -- the `0204` shape. **CONDITIONS: validate the root and refuse LOUDLY; and mark the store IN THE CODE as INTERIM standing in for `D07`.**
- **ANY CANON COMMIT HERE IS SILENTLY A MULTI-NODE COMMIT** (cc). Canon regenerates wholesale and cannot be split. **Whether it is DECLARED depends on whether the committer looks. The deadlock is the VISIBLE case and therefore the benign one.**
- **THE MCP RULESET.** MCP tools call the FACADE, never the CLI dispatch arm. A parent row is a namespace, not a verb. **The test for exposure is NEED, not provenance.** A vocabulary reserved for a population that measures EMPTY is STRUCK. **A tool that can only mutate inverts the safety gradient.**
- **`AC-06.3` IS REWORDED, NOT WITHDRAWN, AND THE NEW FORM IS HARDER.** Ruled form: **every KNOWN deviation is recorded -- a `keep`/`as-observed` row found to differ is a recorded deviation or a filed defect, never silence.** **FILING IS RECORDING: it changes no surface and cannot expand a release scope. A deferred filing is silence wearing a schedule.**
- **AN ISSUE'S AUTHOR DISPOSITIONS IT.** And **a title's job is not to state the final cause, it is to NOT MISDIRECT** -- correct it before the remainder is classified.
- **`close --note` IS NOT BUILT.** `issues edit --from` then `issues close` IS that act. **I originally wrote that it ALREADY SHIPS and that was false.**
- **A CLONE AT A PINNED REVISION IS `FOR REAL`.** `for real` opposes SIMULATED, not CLONED.
- **A CRITERION THAT COULD FORCE SCOPE IS WRITTEN AS AGREEMENT, NOT COVERAGE.**
- **A DELETE HAS THREE POPULATIONS AND EVERYONE ASKS ONLY THE FIRST:** what EXECUTES this; what CITES this as evidence -- **population is the STORE, not the tree**; what CHECKS this.
- **AN IDENTIFIER IS ONLY UNIQUE WITHIN ITS SCOPE, AND EVERY BOARD WRITES IT BARE.**
- **A SECOND HOME IS NEVER ACCEPTABLE AT A TAG.** The escape is not _accept two homes_, it is _need less machinery_.
- **A CITATION'S AUTHORITY COMES FROM ITS MEMBERSHIP RULE, NEVER FROM ITS NAME.**
- **DERIVED CENSUSES MULTIPLY FREELY; AUTHORITATIVE COPIES DO NOT.**
- **A TEST GOING RED BECAUSE A FIX LANDED IS THE NOTIFICATION WORKING.**
- **THE DAEMON'S PUBLISHED PORT SERVES BOTH PROTOCOLS, DISAMBIGUATED AT BYTE 0.**
- **THE MANIFEST IS A SHARED MEASUREMENT SURFACE, NOT A SINGLE-WRITER FILE** (dc). Rows come from whoever drove the fact; the door is `st attach`.
