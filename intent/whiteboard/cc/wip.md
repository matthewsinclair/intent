---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-18 10:26Z
status: paused
focus: "PAUSED, holding for the window to be called. **vc's ST0057 design lands question 7 RULED: `THREAD_PROSE` is DELETED, design/impl/tasks become ATTACHMENTS, canon relocates to `intent/.canon`.** Queue in vc's order: the `Triage -> Wip` row, the `THREAD_PROSE` deletion, `st new --start` walking vs setting, `has_end_date()` over `is_terminal()`, then `--format md` when the disk model lands. **Found while holding: every `st cancel` produces a thread `doctor` calls model-inconsistent -- live on ST0010 and ST0015 today.** Upstream FROZEN; push `local` only."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, **and the ingest corpus excludes ignored paths.**

**hv's DISK MODEL: no status directories, disk becomes OPTIONAL, index plus render-on-demand.** The moment disk is optional, anything the store does not hold is destroyed by the first render. **`realisation.md` 5.1 is the dehydration gate; 5.1b is the attachment asymmetry.**

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. A timestamp not read off a clock is fabricated data, not an approximation. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory.

## DOING -- landed; the rest waits on hv

**Committed and pushed:** rung 9 `issues.body` (40 v2 bodies, 443,643 bytes, no model field at all, VERBATIM), rung 10 `attachments` (110 carried, ONE constructor so `bytes`/`sha256` cannot drift, ONE classifier so ingest/migrator/doctor cannot disagree), `doctor` NAMING all 198 uncarried by path and counting NONE as faults.

**Also landed (`387cab7d`):** `related[]` -- 116 links from 123 bullets, ids from the LEADING region only so a note mentioning a thread is not a link -- and **rung 11**. The live store was already at 11, so this had to land or the tree and the store disagreed.

**Both prose fields are VERBATIM.** `Issue.body` was trimmed until vc scheduled its renderer: **a normalisation that requires a future component to compensate is a scheduled defect**, so the trim went rather than the precondition.

**`.sh` IS CARRIED** on vc's principle -- _no tool can make this again, versus a tool made this and can again._ **A mode bit does not survive; `+x` at hydration is the DECIDED answer**, recorded at the constant for whoever builds the write-back.

## TODO

0. **`Triage -> Wip` VIA `st start` -- the declared transition row is MINE** (hv amended their own 2026-08-15 ruling; doc side `53bc2c10`, published, so it will not move under the implementation). Rider: **`st new --start` can now span ONE transition instead of two, so check whether the flag WALKS the machine or SETS the state** -- those differ now, and it retires ic's EXP-04 rather than documenting it. **The naming question is RULED (vc): the message keeps its wording, the predicate renames.** "Terminal" means ABSORBING, and `Completed` has `st reopen` while `Cancelled` has `st reinstate` -- so _"the machine has no terminal states"_ is the CORRECT use of the word and `is_terminal()` would have been the wrong one. Name it for what it is FOR: **`has_end_date()`**. `is_closed()` is out; `IssueStatus::Closed` owns it.
1. **THAT PREDICATE IS A DEFECT FIX, NOT A TIDY -- the two call sites DISAGREE, and it fires on this estate TODAY.** `facade.rs:1500` writes `completed` for `Completed | Cancelled`; `doctor.rs:612` flags a completion date on any status `!= Completed`. **So every `st cancel` produces a thread `doctor` immediately calls model-inconsistent** -- driven in a sacrificial project, not read off the source: ST0010 and ST0015 are reported on the live estate right now. **And its remedy is unactionable** -- "correct the artefact named above", when hand-editing is the thing the design forbids and the next cancel writes it straight back. Both arms read `has_end_date()`, which also gains the dateless-cancelled arm that was never there. ST0011's _"Completed with no completion date"_ is the OTHER direction and is a REAL finding that survives the fix. **Nothing covers cancel-then-doctor**, which is how 85 legs stayed green over it.
2. **QUESTION 7 IS RULED, and it is the answer this board predicted: `THREAD_PROSE` is DELETED and `design.md` / `impl.md` / `tasks.md` become ATTACHMENTS** (hv, landed by vc at `442ef27d`). Subtractive -- three strings out of a constant -- and it closes the 165 that exist and the 166th together. **The rest of ST0057 moves my ground: canon relocates to `intent/.canon`, ONE FILE PER ARTEFACT** (a consolidated file is a merge-conflict generator with four writers), **the manifest governs VIEWS ONLY, and canon is never sparse** -- otherwise an unrealised artefact would live only inside a gitignored database and be absent from a fresh clone.
3. **D57-5 IS NEW AND IT IS MINE: a complete text realisation of the whole project into `.backup/text/<UTC>/`, as hv's on-disk fallback.** Not a duplicate of the dehydration gate and vc was precise about why: **the gate proves THE STORE holds it; the export proves A HUMAN CAN GET IT BACK WITHOUT THE TOOL.** Two assurances, one covered. Four requirements: **complete with a PRINTED DENOMINATOR** (a partial export reading as complete is worse than none), `.backup/text/<UTC>/` as a third mechanism in the established never-commit namespace beside `upgrade/` and `db/`, **regenerable and NEVER authoritative** (no import path, `classify` never sees it), and cheap enough to be habitual. **And it lands on a refusal of mine whose reason EXPIRES**: `export.rs:192`/`export.rs:208` refuse `--format md` because _"the views are already in the tree"_ -- true today, FALSE the moment the disk model lands, since that is the entire point of it. **RULED: withdrawn as PART of this thread and not before.** Same class as `Issue::body`'s trim -- a claim true when written, expiring on a change already scheduled -- except caught before it shipped rather than after.
4. **The estate regeneration -- MINE, one window, vc calls it.** Canon holds no bodies and no attachments; the store holds the TRIMMED bodies the defective test run left. **Neither `sync` direction is correct today** -- `--to-store` drops them, `--to-disk` reintroduces exactly what I reverted -- and the right content is only derivable by a fresh migration from the v2 markdown. **It now has to land on `intent/.canon` as well.**
5. **The attachment write-back.** vc ruled the policy (5.1b): authority follows AUTHORSHIP, so a view divergence means the FILE is stale and an attachment divergence means the STORE is stale, and `organize` resolves neither.
6. **`sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness** (vc). Worth saying what it is safe FROM.
7. **`created: ST0057` writes three files AND regenerates a tracked `steel_threads.md` while printing one word** (hv's dogfood). A command that modifies a tracked file the user did not name must say so, or they find it in `git status` and have to work out who did it.
8. `upgrade`'s "their content is unchanged" -- a claim nothing computes. `doctor` naming a stale pre-versioning store before a cutover. **`views::info` adds a blank line after a deprecation blockquote** -- visible only on ST0010/ST0015, which are the only two threads that carry one. AC-10.8 egest; AT-10.2 (probed) / 10.3 / 10.4; `WpStatus::Cancelled`.

## Watch-outs -- the mechanisms, distilled

### A VERSION NUMBER IS A CLAIM ABOUT SHAPE -- a rung, once run anywhere, is PUBLISHED

**My worst defect of the day, and it reached hv rather than a test.** I added `seq` by EDITING rung 10, reasoning: _"the live store is at 9, so rung 10 has not been applied to any durable store, so editing it in place is safe."_ **Valid reasoning, false premise, never checked** -- one `PRAGMA user_version` would have said 10. Two shapes stamped 10, and every read of canon died on `no such column: seq`.

**THE RULE, now on the ladder: once any store has run a rung, changing what that rung PRODUCES requires a NEW rung.** The old rung's output is already stamped and unreachable. **An unchecked assumption WEARING THE SHAPE OF A CHECK is the parent of most of the family below** -- it occupies the slot the check would have gone in.

**vc hit the same class the same day, in a document rather than a store**: they asserted a defect in `data-model.md` from a premise they had not checked, and what saved it was going looking for a rationale so hv would not rule against a straw man -- the ratification was sitting at `data-model.md:428` the whole time. **The save came from checking for a DIFFERENT reason, which is luck rather than method.**

### A HUMAN TYPING THE SEQUENCE EXERCISES WHAT NO TEST EXPRESSES -- 3 for 3 in one day

hv's dogfood beat the suite three times: the schema-10 shape collision (two commands, where 85 legs could not express it), `st new` -> `st triage` -> `st start` where v2 was two steps (_"this is STOOPID"_), and `created:` printing one word while rewriting a tracked index. **The project had even PREDICTED the adjacent drift** -- ic's EXP-04 recorded that `st new -s` now spans two transitions -- **and recorded the SEMANTIC cost while missing the ERGONOMIC one.** Nothing short of a person doing the ordinary thing surfaces "a user creates a thread and starts working on it".

### A SUITE THAT ALWAYS STARTS FRESH CANNOT SEE A MIGRATION DEFECT

Every fixture builds from the current `DDL`, so every test gets the current shape and passes. The remedy is a test whose fixture is **a store this binary did not create**. Same shape as: **a subject that cannot exhibit the defect cannot clear it** -- the gitignored store no clone could hold, and the attachment ordering the path-sorting migrator makes unobservable on the real tree. **In both cases the thing that found it differed from the subject BY ACCIDENT.**

### FIVE WAYS AN INSTRUMENT REPORTS GREEN WITHOUT MEASURING

1. **It never ran** -- 6 compile errors, 0 legs, sentinel `0` reading as a clean estate.
2. **It ran where the property held FOR ANOTHER REASON** -- my fixture's attachments happened to be in path order; vc's count agreed only because the live tree IS a git repo, so `require_git` was satisfied.
3. **It returned the RIGHT answer without running** -- `grep -F` eating a leading dash.
4. **Its alarm is the RECORD OF THE FIX** -- grepping `legacy.rs` for `trim` to ask whether the trim is gone, and hitting the comment saying it is. **The better the comment, the louder the false positive.**
5. **Sampling** -- five legs green individually while the full run had 41 failures, off a sample chosen before the answer was known.

**And the one that beats all of them: a harness that applies the transformation it is testing for cannot fail.** I compared `canon.body` to `source.strip()` and reported 40 of 40 byte-exact; vc hashed the raw remainder and got 40 of 40 off by one byte.

### THE COUNTER-PATTERN, AND IT COSTS ONE LINE

`conservation_check.sh:793` names its own blind spot. **`sync`'s refusal is that shipped**: it declines to guess, names both directions, marks which is destructive, and enumerates all 40 issues it would overwrite before doing nothing. **Never suppress a harness's stderr, never truncate its output, never read an exit code through a pipe. When a result would be BIG NEWS, test the harness before sending the news.**

### A TEST THAT ASSERTS ITS OWN PREMISE RETIRES ITSELF

My `related[]` parser falsified _"ingest does not populate `related`"_ and the test **failed on the PREMISE line** rather than quietly measuring something else. The counter-example keeps passing and stops meaning anything.

### GIT, AND THE FOUR-NODE TREE

**The push exit code carries no information** -- `ls-remote` plus `merge-base --is-ancestor` is the only witness. **`git commit --amend` rewrites whoever committed LAST.** Always `git commit --only <paths>`. **`cd` resets after a subshell** -- it drifted nine times in two sessions, once manufacturing a finding out of a failed `stat`. **We share one working tree, so a half-finished contract is on everyone's PATH the moment it compiles** -- and `cargo` rebuilding `target/debug/intent` can replace a binary a peer is running.

### PROSE THAT IS A BUILD INPUT

**A `///` doc comment is SHIPPED OUTPUT (D37)** -- schemars lifts it into JSON Schema, async-graphql into the SDL. Plain `//` for reasoning. Intent's own ST/WP/AC ids never reach output. **And a `/*` anywhere in shipped source trips the pm-state scanner by design.**

## Lane boundary

`dc` owns dev-x, build, CI, release, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour.** `surface/dispatch-table.json` is ic's; `acceptance.md` / `design.md` / `data-model.md` / `realisation.md` are vc's. **`bin/intent*` is cc's and FROZEN -- and that does NOT generalise: `lib/templates/` is not.**

## Standing rulings

- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work: fix it inline, reason in the commit, message the owner if it crosses a lane.
- **An uncarried file is NOT a disposition** (vc). `dropped` licenses `conservation_check.sh` to stop looking; "still on disk" is the one thing that record exists to distinguish.
- **`doctor` names uncarried files and counts none as faults.** A check that reds 100% of a population behaving as designed is a check that gets deleted, and the real finding goes with it.
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`EdgeKind::Incidental` STAYS with no user. `owner_wp` stays carried and unread. `doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.**
- **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`.**
