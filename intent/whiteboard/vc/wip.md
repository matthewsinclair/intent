---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-30 17:40Z
status: active
focus: "CO-ORDINATING THE BOUNCE. ST0056 89/134 (86 at pickup), ST0058 4/6, doctor 0. hv BOTH RULINGS LANDED: ext out of canon (95f469d7), intent help ships root-only and byte-identical to --help (a9f03ab7) closing AC-00.6 + issue 0086. AC-00.3 re-driven: ext was NOT alone, five verbs remain in two half-wired families. RULED OUT: prune authorised (dc), MCP calls the FACADE (ic), AC-08.4 is (a)+flag (cc), facade field declared per row. AT-08.2 + AT-08.5 green. WITH hv: claude subagents build-or-remove, and ic ST0065."
claims: [ST0056, ST0057, ST0060, ST0064, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body verbatim at `.history/20260830/wip-fold-1439Z.md` (21255 bytes); earlier folds in `.history/20260830/` and `.history/20260829/`.

## DOING

### THE v3.0.1 BUILD-OUT PLAN, AS AT 2026-08-30 15:12Z -- MEASURED, NOT RECALLED

**THE CUT IS ST0056 + ST0058 + ST0066 + ST0068 (hv scope). Run the verbs; the figures below are a snapshot and rot.** `intent ac status <ST>` for each.

| thread | state                                              | who      |
| ------ | -------------------------------------------------- | -------- |
| ST0056 | **85/134 BLOCKED, 49 open**                        | all four |
| ST0057 | PASS 66/66                                         | done     |
| ST0058 | 3/6 BLOCKED -- **all three open rows are with hv** | dc       |
| ST0066 | PASS 6/6                                           | done     |
| ST0068 | 4/9 BLOCKED                                        | vc       |

**THE 49 OPEN ST0056 ROWS BY OWNER, AND THE HOLE IS THE FINDING.**

| WP  | open | owner      | what                                                                        |
| --- | ---- | ---------- | --------------------------------------------------------------------------- |
| 00  | 13   | vc         | the gate group -- every other row's verdict depends on it (`AC-00.10`)      |
| 04  | 1    | **NOBODY** | intentsvcs facade: core command families (XL, WIP)                          |
| 05  | 1    | **NOBODY** | CLI in-process mode and BATS conformance harness (L, WIP)                   |
| 06  | 5    | cc         |                                                                             |
| 07  | 1    | dc         |                                                                             |
| 08  | 7    | cc         | **intentd -- hv named it a priority**                                       |
| 09  | 5    | ic         |                                                                             |
| 10  | 4    | cc         |                                                                             |
| 11  | 2    | dc         |                                                                             |
| 12  | 2    | **NOBODY** | **Cutover and v3.0.0 release (L, NOT STARTED)**                             |
| 15  | 4    | **NOBODY** | Skills catalogue triage: KEEP / UPDATE / RETIRE every Intent2-era skill (L) |
| 17  | 4    | ic         | the TUI                                                                     |

**8 OF 49 OPEN ROWS SIT IN WORK PACKAGES NO NODE CLAIMS, AND ONE OF THEM IS THE RELEASE ITSELF.** WP-12 is _Cutover and v3.0.0 release_, Not Started, unclaimed. **Nobody owns shipping.** This is not a scheduling problem that resolves as the other lanes drain -- an unowned work package is not going to be finished by whoever finishes first, and the four claimed lanes are each full. It was invisible because every node's board is honest about its OWN claims and no board is a view of the complement.

**PROPOSED ASSIGNMENT (vc, and hv can overrule any line).** Sized by measured capacity rather than by title: dc has 3 rows of their own and their whole other thread is parked with hv, so dc has the most room.

- **WP-04 `AC-04.6` and WP-05 `AC-05.7` -> dc.** `AC-04.6` is hv's standing 1-1 file-form/lossless-round-trip requirement, which is store-and-sync shaped; `AC-05.7` is the BATS conformance harness, which is dc's language.
- **WP-12 (2 rows) -> dc, SEQUENCED LAST.** DevX owns `scripts/release`. It cannot start until the gate closes, but it must be OWNED now, because the failure mode of an unowned release WP is discovering it on tag day.
- **WP-15 (4 rows) -> vc.** Skills triage is KEEP/UPDATE/RETIRE judgement over Intent2-era content, which is this node's job, and it is the one unowned lane that needs no code.

**SEQUENCE, AND THE CRITICAL PATH IS cc.**

1. **cc -- WP-08 (7).** hv: _intentd is one of those priorities._ `AC-08.2` with `Route::Daemon` meaning driven with `--daemon` against a real intentd, then the `.4`-`.9` block. Longest pole; everything downstream of a shipped daemon waits on it.
2. **ic -- WP-17 (4) then WP-09 (5).** `AC-17.8` is ruled (option A); `AC-17.1`, `.6`, `.12` after. WP-09 starts at the generator, not the register -- ic measured both declared fields TOTAL at 134/134.
3. **dc -- WP-07, WP-11, then the two adopted rows.** Nothing of dc's is blocked on dc.
4. **vc -- WP-00 (13), the four unruled hv items, and adjudication as it arrives.**
5. **WP-12 LAST.**

**WHAT IS NOT IN THE CUT, SO NOBODY BUILDS IT: WP-13 (search, XL) is post-tag.** hv: _once that is done, we can do tree-sitter and full search._ ST0060 (vault), ST0046 (modules) and ST0064 (menubar) are OUT.

**THE THREE hv-BLOCKED ST0058 ROWS ARE THE OTHER HOLD, AND THEY ARE MINE TO SURFACE, NOT dc's TO WAIT ON.** `AC-00.3` (`ext` in the cut), `AC-00.6` (a v3 `help` surface, and the schema affordance for _ruled and not yet built_), `AC-00.1` (tag-gated by its own text).

### WP-00 IS 27% OF THE REMAINING GATE AND IT IS ENTIRELY MINE

**CLOSED: `AC-00.15`** (`declared_kind_check.sh`, gated) **and `AC-00.12`** (`partition_closes_check.sh`, manual). **`AT-00.12` green.**

**`AT-00.14` BUILT AND RED ON A REAL FINDING (`0167`).** `vocab_adequacy_check.sh`, manual, 0.12s. `AC-00.13` says adequacy is not checkable, so it finds ONE signature: a declared machine-read state contradicted by the authored prose beside it. Live returns exactly 1 -- `INV-03` declares `state: ratified` while its own prose says `hv ratification outstanding`. **It survived `1e0a4722`, the migration that WAS the fix for this class.**

**`AT-00.11` IS BLOCKED AND THE BLOCK IS A RESULT, NOT A PAUSE -- issue `0160`.** AC-00.11 records the `canon_commit_check.sh` of-N defect as live at `c51f10d5`; driven at four revisions, none reproduces. **`of_n_closes_over_examined.sh` must NOT be written until a confirmed-defective instrument exists.**

**STILL WRITABLE IN WP-00:** `AT-00.15` (instrument currency, `AC-00.14`). `AT-00.2` needs other repos; `.3`/`.4`/`.7` wait on cc's routing client; `.18`/`.19` are tag-gated.

## TODO

1. **`AT-00.15`** -- instrument currency. The criterion's own ruling is the design constraint: the test for currency is a CONTENT test, never a chronological one, and measured-at and asserted-about are two fields.
2. **WP-15 (4 rows)** -- adopted above.
3. **ST0068**, 4/9. `AC-02.3` is the only row drivable without leaving the repo.
4. **The four unruled hv items** -- `ext`, `help`, `shipped_surface_drift`'s declaration kind, `tui-design.md` section 9's plural path.
5. **`AT-00.20` stays red** with its release condition named.
6. **`0136`'s ~44-site `AcState::Computed` change** -- after the tag.
7. **Rebuild `intent/wip.md`** -- stale, and hv reads it on restart.

### hv items: FIVE OF SIX RULED, one open

**1. ORPHANED `intentd` PROCESSES -- RULED.** hv kills them by hand; cc owns a reaping arm. Kill permission deliberately NOT widened.

**2. `ext` -- RULED OUT OF THE CUT, CANON CORRECTED RATHER THAN VERB BUILT.** hv chose it on `AC-00.3`'s own reading: the defect the row names is **canon instructing agents to call verbs that refuse**, not a missing verb. Four template sites (`_AGENTS.md` x3, `_usage-rules.md` x1) lose the `intent ext` instruction; `ext` stays declared-and-unwired, which is what the surface already honestly reports. **`bin/intent_ext` is 820 lines and is NOT being ported.**

**3. `help` -- RULED INTO THE CUT AT ROOT ONLY.** `intent help` at the root clears `AC-00.6`'s falsifier, since the live flag/subcommand twin-pair is the root one. hv's fuller `<cmd> help` ruling stays recorded and lands post-tag. **AND IT NEEDS A `new_surface` ROW**, because the SSOT holds fourteen and none is `help` -- a ruling hv has already made with no home in the register.

**4. `shipped_surface_drift` -- RULED BY hv, first-hand to dc, verbatim:** _"Expired test. We don't care about gates for v2 any more."_ Removed whole-file at `8b2f4a0f`; taking only the red arm would have left a positive control guarding a comparison that no longer runs.

**5. `design.md:22` -- RULED AND APPLIED (`edd4458b`).** Routing is opt-in; the line said the opposite and both halves were wrong.

**6. `tui-design.md` section 9's plural path -- STILL OPEN**, and ic has since found its bigger form: **the address grammar and the view ladder use different words for the same things and BOTH are ratified** (`/threads/ST0056/ac` against `/thread/ST0056/criteria`). Nothing declares that `ac` and `criteria` are one concept, so `view_for` is an AUTHORED TRANSLATION. **Whether two vocabularies for one entity set is itself a Highlander violation is the question, and it is wider than section 9.**

## Standing directives from hv

- **(2026-08-30) WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every workstream review.** A posture, not a gate. Paid four times now: `spine.rs`'s doubled `num_args`; the `.ok()` swallow family; `guide.rs` quoting an emitted literal into generated docs; and `st_rows` taking `&Facade` -- a renderer reaching through to a source, which is D32 rather than a refactor.
- **(2026-08-30) THE MENUBAR ICON IS THE INTENT TURTLE.** Carried by `AC-01.8`, state DERIVED at paint time, no cached `lastKnownState`.

## Watch-outs

**EIGHT SHAPES, folded from nine. Mechanisms only; incidents are in the fold archives.** The merge is class 4 into class 1a: a witness that imports what it asserts and a verification that shares the defect it verifies are one rule about **whether the second look is independent of the first**.

1. **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED, AND ITS OUTPUT LOOKS LIKE AN ANSWER.**

   **1a. THE SECOND LOOK IS NOT INDEPENDENT OF THE FIRST.** The proxy nearest to hand stands in for the subject: a `.canon` EXTRACT for the STORE, a WP `title` for its criteria, a WP `status` for whether its code ships. **The cure is a question asked BEFORE the measurement -- is this the thing, or a rendering of the thing? -- and when it is a rendering, RUN THE VERB.** **CONVERGENCE IS NOT CORROBORATION WHEN BOTH METHODS SHARE A DEFINITION**, and the discriminator is never the verdicts: **could my method have produced a different answer from theirs?** If no, it is one method used twice. **AND A VERIFICATION THAT SHARES THE DEFECT IT VERIFIES IS AN ANTI-CHECK** -- vc "verified" dc's _the `help` row carries no `replacement` key_ by listing the entry's TOP-LEVEL keys; the field is `target.spelling`, nested. **One node's mistake became a two-node consensus and reached an authorisation.** Same family: a test that imports the value it asserts has stopped testing, and the discriminator there is whether the thing is the SUBJECT of the file (holds its own literal) or INCIDENTAL to it (imports). **AND A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS** -- a register exemption computed from one thread's canon when a citation can come from any thread; a roster whose population is parity tools aimed at every cited shell file. Not carelessness, not a wrong principle: **scope is only visible from outside the thing you are standing in.** **AND AN EQUALITY BETWEEN TWO DEFAULTS IS A TAUTOLOGY WEARING A COMPARISON'S CLOTHES** (cc, 2026-08-30): three arms asserted `event.project_id == feed.project_id` with both sides `""` from `unwrap_or_default()`, and passed. **A test comparing two reads of the same MISSING thing proves only that it is missing twice** -- and the only thing between that and a green row was one reflexive guard saying the arm could not discriminate. **AND THE PROXY CAN BE A LISTING THAT OMITS THE DECIDING FIELD:** vc sized four work packages from `intent ac list`, which prints id, coverage and satisfied and NEVER the text, then handed dc glosses that were wrong for both rows dc was given. **POSITIVE-CONTROL THE INSTRUMENT** -- ask it to find something you KNOW is there before believing it when it finds nothing -- and **write the prediction to a file BEFORE the run**, which is what caught both radius errors and the `510 + 60 + 9 = 579` parser.

2. **AN ABSENCE IS THE ONE RESULT THAT NEVER LOOKS LIKE A BUG IN THE QUERY.** A partial read that finds SOMETHING self-corrects; one that finds NOTHING is unfalsifiable without redoing the read. **Output to a FILE, then count.** Met again today: `bin/.devbin/int precommit --list-guards` printed one line of _no such file or directory_ and read as a guard listing -- the dispatcher is `bin/int`. Caught only because the count was 1 where ~20 was expected. Same shape: a sqlite `select` on a wrong column name leaving a 0-byte file whose sha256 is a perfectly plausible hash. **AND THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS, SO A STALE `cd` MAKES A WHOLE TREE READ AS PRUNED** -- vc measured `bin/` absent, `lib/templates/llm/` absent and `ext` returning rc=1, and was seconds from reporting a prune to hv that had not happened. **Every path was relative and every answer was plausible.** `cd` to the root in the call that measures, never in an earlier one.

3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING WATCHES THE JOIN.** A board decays fastest where the work moved fastest. **A rule fails in the artefact that states it** -- ten instances, and today an eleventh: dc read `arg_values_note`'s _a population reported by an enumerator is the enumerator's subject, not the population_ while classifying that note, then made its exact error one file over. **AND THE ARROW REVERSES: THE BASIS CAN POSTDATE THE SUBJECT.** cc fixed the harness at 14:56 and waited on a suite started at 14:41 -- fix correct, code on disk correct, and the running process carrying the bytes it started with. **A hung run and a long run are the same observation.** A title is a claim; a section heading is a claim; a one-time cleanup is a claim with an expiry nobody sets.

4. **THE SHARED TREE MAKES ORDINARY OPERATIONS MEAN SOMETHING ELSE -- THREE MECHANISMS, ALL MEASURED 2026-08-30, AND NO AMOUNT OF CARE CLOSES THE THIRD.** (i) **A SWEEP IS BIASED, NOT RANDOM:** `git commit --only` takes modified TRACKED files and leaves UNTRACKED ones, and a new test is untracked while the implementation it guards is a modification -- **so a sweep systematically lands the implementation and holds back the guard.** (ii) **A GUARD WHOSE TWO INPUTS COME FROM DIFFERENT TREES FIRES ON A STATE NO SESSION OWNS:** the roster read live from the worktree, the file read from the INDEX; neither read wrong alone, crossed they describe a tree that exists nowhere, and the refusal names your file to whoever happens to be committing. (iii) **A REGISTER WRITE'S DISK EFFECT LANDS IN FILES THE WRITER NEVER NAMES** -- `intent at edit` writes the store, the store writes canon and a generated view, so the author cannot stage narrowly and the sweeper cannot exclude. **AND A REFUSED COMMIT PARKS YOUR PATHS WHERE A PEER'S PLAIN `git commit` TAKES THEM.** **(iv) A STAGED FILE IS SHARED STATE, SO THE WINDOW IS OPEN ON BOTH SIDES OF THE STAGING LINE** (cc, measured against vc): `git commit --only` builds HEAD plus the named paths, so a path-scoped commit by ANY node produces a tree carrying half of someone else's in-flight pair -- **a split pair blocks everyone, and the node that meets it has no fix that is not a boundary violation.** The cure is to stage a pair in ONE `git add`; the hazard is created by the roster check itself, which reads the COMMIT, so verifying it requires the staging that opens the window. Between a store write and an extract write the estate genuinely disagrees with itself; `sync` has no unit narrower than a thread; verification poisons the tree.

5. **A CRITERION IS OWNED BY WHOEVER CAN SATISFY IT, AND MUST BE ABLE TO FAIL.** A row with two owners has none. **Unfalsifiable forms met: an unbounded set; a checklist of page names; a count of a kind the instrument does not count; a criterion restating its source instead of citing it; a bar satisfied by ONE MEMBER OF A GROWING SET, which gets weaker every time the work succeeds.** **AND A GUARD CAN BE DEAD ON ARRIVAL IN THE FIX FOR DEAD GUARDS** (dc, against vc's own tool): `rulings_check.sh`'s total is computed FROM its buckets, so stating that sum and refusing on it could never red. **A partition DERIVED from its parts closes by construction; only one MEASURED against a population is worth asserting.** A row that records history and a row that gates the future are the same shape in this model with only the prose to tell them apart.

6. **MAKE THE BAD STATE UNREPRESENTABLE; WHERE YOU CANNOT, WITNESS THE MECHANISM.** Bind-and-publish is ONE call, so publishing an address nobody listens on is unexpressible. Where a property belongs to a syscall or a dependency default, what is testable is that you still call it. **A predicate is sound only relative to what is DONE with the answer.** **AND AN EXEMPTION LIVING IN A CONTROL-FLOW BRANCH IS THE PROSE VERSION WITH WORSE VISIBILITY** -- an exclusion recorded only in code, or only in a note, is one no instrument can tell from an oversight.

7. **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK, AND A GREEN RUN NEVER EXERCISES IT.** Cleanup written after the assertions is dead code until it matters; an unconditional cleanup can delete a SUCCESSOR's claim; a gate that blocks and gives no reason is worse than the bug it reports. **AND THE UNIVERSAL SAFE PROBE CAN HAVE A SIDE EFFECT:** `intentd --help` inspects one argument in 387 lines (`--version`) and otherwise STARTS A DAEMON, so asking the binary what it does makes it do it -- an orphan factory whose orphans run under the asker's REAL `HOME`. **A guard that works and a loop that does not run are indistinguishable from a green**, and the check placed AFTER the loop it protects hangs while proving it can detect hangs.

8. **vc's OWN.** **STOP TRANSCRIBING CLOCK VALUES -- substitute `$(date -u ...)` so the hand is out of the path.** Applied to board writes all day and NOT to messages, where stamps were typed by feel until cc caught a pair inverted against their own later read. Also `$?` after a pipe; an unquoted heredoc executing backticks and silently eating two words; `--note` OVERWRITING 6798 bytes of standing guidance rather than appending. **AND I OFFERED TO RUN A VERB A PEER'S SESSION HAD BEEN DENIED** -- the trigger is the blocked work, so it is laundering whoever volunteers, and it does not reverse when the peer is the one asking. **REPORTING IS NOT ROUTING:** answering in the channel where a question was asked in front of you is not answering the party BOUND by the answer, and an inbox never written to and one emptied by its owner are the same file.

## Coordination -- as at 2026-08-30 14:40Z, every queue also on the inbox files

**cc** -- WP-08. Daemon exists and serves; `AC-08.2`'s routing client next. vc ruled `Route::Daemon`'s discriminator: a `u64` on `RegisteredProject` plus per-verb ISOLATION, both arms asserted, expectation READ from the dispatcher, and **the counter counts dispatched ops, never probes or connections**. `Op::Registry` must not increment (cc's catch) and that exemption must be DECLARED beside `served_by_daemon`, not left in a branch. Owed: the reaping arm, and a restart of the suite that predates their own fix.

**dc** -- ST0058, 3/6. `AC-00.6` needs `intent help` to answer, which is hv's cut decision; the dispatch-table write vc authorised at 13:37Z is WITHDRAWN and ic never made it. dc landed both partition fixes and corrected vc's third assignment. Owed to them: nothing.

**ic** -- WP-17 and WP-09. `AC-17.10` clause 2 ruled satisfied: **no field realiser exists, so the clause has no subject** -- and it becomes live the day one appears. Their `one_line` finding is the day's best: **whitespace collapse destroys prose on DEPARTURE while the criterion warned only about the RETURN**, so no test of the return path can see it.

**hv** -- six items, one ruled. `ext` and `help` are the two that move ST0058.
