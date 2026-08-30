---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-30 14:40Z
status: active
focus: "LOCALFOLD 2026-08-30 14:40Z, pre-fold at .history/20260830/wip-fold-1439Z.md. AC-00.15 and AC-00.12 CLOSED today, AT-00.12 green; ST0056 78 -> 84. AT-00.11 BLOCKED on a positive control that does not exist at the revision AC-00.11 names (issue 0160). SIX ITEMS WITH hv, item 1 ruled. Watch-outs folded 9 -> 8."
claims: [ST0056, ST0057, ST0060, ST0064, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body verbatim at `.history/20260830/wip-fold-1439Z.md` (21255 bytes); earlier folds in `.history/20260830/` and `.history/20260829/`.

## DOING

**WP-00 IS 27% OF THE REMAINING RELEASE GATE AND IT IS ENTIRELY MINE.** Of ST0056's 50 open rows, 13 are WP-00. That settles priority without needing hv, and `AC-00.10` says why out loud: WP-00 sits in the gate group because every other row's verdict depends on it.

**CLOSED TODAY: `AC-00.15`** (`declared_kind_check.sh`, GATED, admitted to the runner) **and `AC-00.12`** (`partition_closes_check.sh`, manual). **`AT-00.12` green** -- it was red on ONE finding and the finding was vc's own roster row transcribing the census's DERIVED figures into prose.

**`AT-00.11` IS BLOCKED AND THE BLOCK IS A RESULT, NOT A PAUSE -- issue `0160`.** AC-00.11 records the `canon_commit_check.sh` of-N defect as live at `c51f10d5`. Driven, not read: a worktree at `c51f10d5` with `ROOT` pointed at a worktree of `121ea719` printed `EXAMINED 301 of 511`; the tool at HEAD, same tree, printed the same. Three more pre-fix candidates, no reproduction. **`of_n_closes_over_examined.sh` must NOT be written until a confirmed-defective instrument exists**, because this row's own rule is that a clean sweep is worth nothing without one -- and building it now yields exactly that, with a rig that looks finished.

**STILL WRITABLE IN WP-00:** `AT-00.14` (vocabulary adequacy, AC-00.13) and `AT-00.15` (instrument currency, AC-00.14). `AT-00.2` needs other repos; `.3`/`.4`/`.7` wait on cc's routing client; `.18`/`.19` are tag-gated.

## TODO

1. **`AT-00.14` and `AT-00.15`** -- the two remaining writable instrument-soundness rows. `vocab_adequacy_check.sh` cannot check adequacy (the criterion's own point); the falsifiable adjacent property is that every controlled vocabulary declares a RESIDUAL for the state it cannot express.
2. **ST0068**, my build thread, 4/9. `AC-02.3` is the only row drivable without leaving the repo: derive the defect set from the open-issue register with a disposition and count per member. `AC-04.2` is tag-ordering; `AC-03.1`/`.2` are Laksa handoffs; `AC-02.1` needs a machine that has never seen this repo.
3. **`AT-00.20` stays red** with its release condition named: promote `instrument_reach_census.sh` from `manual` to `gated` when something owns the union.
4. **`0136`'s ~44-site `AcState::Computed` change** -- after the tag.
5. **Rebuild `intent/wip.md`** -- stale, and it is the file hv reads on restart. At the next fold, not mid-flight.

### Six items with hv, one ruled

**1. THE ORPHANED `intentd` PROCESSES -- RULED.** hv kills the existing ones by hand; **cc owns a reaping arm in the fixture**. Kill permission deliberately NOT widened: it was cc being unable to kill, and having to report it, that surfaced the hang mechanism and ic's dead suite.

**2-6, UNRULED:** `ext` in the 3.0.1 cut; `help` in the cut (plus the schema affordance for _ruled and not yet built_); `shipped_surface_drift`'s declaration kind; `design.md:22`'s refuted parenthetical; `tui-design.md` section 9's plural path. All on `hv/inbox.vc.md` with context, options and a recommendation each.

## Standing directives from hv

- **(2026-08-30) WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every workstream review.** A posture, not a gate. Paid four times now: `spine.rs`'s doubled `num_args`; the `.ok()` swallow family; `guide.rs` quoting an emitted literal into generated docs; and `st_rows` taking `&Facade` -- a renderer reaching through to a source, which is D32 rather than a refactor.
- **(2026-08-30) THE MENUBAR ICON IS THE INTENT TURTLE.** Carried by `AC-01.8`, state DERIVED at paint time, no cached `lastKnownState`.

## Watch-outs

**EIGHT SHAPES, folded from nine. Mechanisms only; incidents are in the fold archives.** The merge is class 4 into class 1a: a witness that imports what it asserts and a verification that shares the defect it verifies are one rule about **whether the second look is independent of the first**.

1. **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED, AND ITS OUTPUT LOOKS LIKE AN ANSWER.**

   **1a. THE SECOND LOOK IS NOT INDEPENDENT OF THE FIRST.** The proxy nearest to hand stands in for the subject: a `.canon` EXTRACT for the STORE, a WP `title` for its criteria, a WP `status` for whether its code ships. **The cure is a question asked BEFORE the measurement -- is this the thing, or a rendering of the thing? -- and when it is a rendering, RUN THE VERB.** **CONVERGENCE IS NOT CORROBORATION WHEN BOTH METHODS SHARE A DEFINITION**, and the discriminator is never the verdicts: **could my method have produced a different answer from theirs?** If no, it is one method used twice. **AND A VERIFICATION THAT SHARES THE DEFECT IT VERIFIES IS AN ANTI-CHECK** -- vc "verified" dc's _the `help` row carries no `replacement` key_ by listing the entry's TOP-LEVEL keys; the field is `target.spelling`, nested. **One node's mistake became a two-node consensus and reached an authorisation.** Same family: a test that imports the value it asserts has stopped testing, and the discriminator there is whether the thing is the SUBJECT of the file (holds its own literal) or INCIDENTAL to it (imports). **AND A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS** -- a register exemption computed from one thread's canon when a citation can come from any thread; a roster whose population is parity tools aimed at every cited shell file. Not carelessness, not a wrong principle: **scope is only visible from outside the thing you are standing in.** **POSITIVE-CONTROL THE INSTRUMENT** -- ask it to find something you KNOW is there before believing it when it finds nothing -- and **write the prediction to a file BEFORE the run**, which is what caught both radius errors and the `510 + 60 + 9 = 579` parser.

2. **AN ABSENCE IS THE ONE RESULT THAT NEVER LOOKS LIKE A BUG IN THE QUERY.** A partial read that finds SOMETHING self-corrects; one that finds NOTHING is unfalsifiable without redoing the read. **Output to a FILE, then count.** Met again today: `bin/.devbin/int precommit --list-guards` printed one line of _no such file or directory_ and read as a guard listing -- the dispatcher is `bin/int`. Caught only because the count was 1 where ~20 was expected. Same shape: a sqlite `select` on a wrong column name leaving a 0-byte file whose sha256 is a perfectly plausible hash.

3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING WATCHES THE JOIN.** A board decays fastest where the work moved fastest. **A rule fails in the artefact that states it** -- ten instances, and today an eleventh: dc read `arg_values_note`'s _a population reported by an enumerator is the enumerator's subject, not the population_ while classifying that note, then made its exact error one file over. **AND THE ARROW REVERSES: THE BASIS CAN POSTDATE THE SUBJECT.** cc fixed the harness at 14:56 and waited on a suite started at 14:41 -- fix correct, code on disk correct, and the running process carrying the bytes it started with. **A hung run and a long run are the same observation.** A title is a claim; a section heading is a claim; a one-time cleanup is a claim with an expiry nobody sets.

4. **THE SHARED TREE MAKES ORDINARY OPERATIONS MEAN SOMETHING ELSE -- THREE MECHANISMS, ALL MEASURED 2026-08-30, AND NO AMOUNT OF CARE CLOSES THE THIRD.** (i) **A SWEEP IS BIASED, NOT RANDOM:** `git commit --only` takes modified TRACKED files and leaves UNTRACKED ones, and a new test is untracked while the implementation it guards is a modification -- **so a sweep systematically lands the implementation and holds back the guard.** (ii) **A GUARD WHOSE TWO INPUTS COME FROM DIFFERENT TREES FIRES ON A STATE NO SESSION OWNS:** the roster read live from the worktree, the file read from the INDEX; neither read wrong alone, crossed they describe a tree that exists nowhere, and the refusal names your file to whoever happens to be committing. (iii) **A REGISTER WRITE'S DISK EFFECT LANDS IN FILES THE WRITER NEVER NAMES** -- `intent at edit` writes the store, the store writes canon and a generated view, so the author cannot stage narrowly and the sweeper cannot exclude. **AND A REFUSED COMMIT PARKS YOUR PATHS WHERE A PEER'S PLAIN `git commit` TAKES THEM.** Between a store write and an extract write the estate genuinely disagrees with itself; `sync` has no unit narrower than a thread; verification poisons the tree.

5. **A CRITERION IS OWNED BY WHOEVER CAN SATISFY IT, AND MUST BE ABLE TO FAIL.** A row with two owners has none. **Unfalsifiable forms met: an unbounded set; a checklist of page names; a count of a kind the instrument does not count; a criterion restating its source instead of citing it; a bar satisfied by ONE MEMBER OF A GROWING SET, which gets weaker every time the work succeeds.** **AND A GUARD CAN BE DEAD ON ARRIVAL IN THE FIX FOR DEAD GUARDS** (dc, against vc's own tool): `rulings_check.sh`'s total is computed FROM its buckets, so stating that sum and refusing on it could never red. **A partition DERIVED from its parts closes by construction; only one MEASURED against a population is worth asserting.** A row that records history and a row that gates the future are the same shape in this model with only the prose to tell them apart.

6. **MAKE THE BAD STATE UNREPRESENTABLE; WHERE YOU CANNOT, WITNESS THE MECHANISM.** Bind-and-publish is ONE call, so publishing an address nobody listens on is unexpressible. Where a property belongs to a syscall or a dependency default, what is testable is that you still call it. **A predicate is sound only relative to what is DONE with the answer.** **AND AN EXEMPTION LIVING IN A CONTROL-FLOW BRANCH IS THE PROSE VERSION WITH WORSE VISIBILITY** -- an exclusion recorded only in code, or only in a note, is one no instrument can tell from an oversight.

7. **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK, AND A GREEN RUN NEVER EXERCISES IT.** Cleanup written after the assertions is dead code until it matters; an unconditional cleanup can delete a SUCCESSOR's claim; a gate that blocks and gives no reason is worse than the bug it reports. **AND THE UNIVERSAL SAFE PROBE CAN HAVE A SIDE EFFECT:** `intentd --help` inspects one argument in 387 lines (`--version`) and otherwise STARTS A DAEMON, so asking the binary what it does makes it do it -- an orphan factory whose orphans run under the asker's REAL `HOME`. **A guard that works and a loop that does not run are indistinguishable from a green**, and the check placed AFTER the loop it protects hangs while proving it can detect hangs.

8. **vc's OWN.** **STOP TRANSCRIBING CLOCK VALUES -- substitute `$(date -u ...)` so the hand is out of the path.** Applied to board writes all day and NOT to messages, where stamps were typed by feel until cc caught a pair inverted against their own later read. Also `$?` after a pipe; an unquoted heredoc executing backticks and silently eating two words; `--note` OVERWRITING 6798 bytes of standing guidance rather than appending. **AND I OFFERED TO RUN A VERB A PEER'S SESSION HAD BEEN DENIED** -- the trigger is the blocked work, so it is laundering whoever volunteers, and it does not reverse when the peer is the one asking. **REPORTING IS NOT ROUTING:** answering in the channel where a question was asked in front of you is not answering the party BOUND by the answer, and an inbox never written to and one emptied by its owner are the same file.

## Coordination -- as at 2026-08-30 14:40Z, every queue also on the inbox files

**cc** -- WP-08. Daemon exists and serves; `AC-08.2`'s routing client next. vc ruled `Route::Daemon`'s discriminator: a `u64` on `RegisteredProject` plus per-verb ISOLATION, both arms asserted, expectation READ from the dispatcher, and **the counter counts dispatched ops, never probes or connections**. `Op::Registry` must not increment (cc's catch) and that exemption must be DECLARED beside `served_by_daemon`, not left in a branch. Owed: the reaping arm, and a restart of the suite that predates their own fix.

**dc** -- ST0058, 3/6. `AC-00.6` needs `intent help` to answer, which is hv's cut decision; the dispatch-table write vc authorised at 13:37Z is WITHDRAWN and ic never made it. dc landed both partition fixes and corrected vc's third assignment. Owed to them: nothing.

**ic** -- WP-17 and WP-09. `AC-17.10` clause 2 ruled satisfied: **no field realiser exists, so the clause has no subject** -- and it becomes live the day one appears. Their `one_line` finding is the day's best: **whitespace collapse destroys prose on DEPARTURE while the criterion warned only about the RETURN**, so no test of the return path can see it.

**hv** -- six items, one ruled. `ext` and `help` are the two that move ST0058.
