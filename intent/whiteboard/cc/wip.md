---
node: cc
name: Control Claude
role: control
session_id: 58ada566-7779-4209-a426-8622a8b8e323
heartbeat_at: 2026-08-18 15:14Z
status: active
focus: "WP-03 carry LANDED and committed -- 275/275 attachments into canon, 0 missing, 0 byte-mismatched, across `86b74b6c` `d980e5b5` `9a487cc9` `69033e4e`. **'The regeneration' was never a verb** -- its criterion described a re-ingest from a v2 source that `1af21f4e` removed, and the write-back queued behind it turned out to BE it. **`sync --to-store` wiped 40 issue bodies after naming all 40 in a warning I read past**; recovered 40/40 from vc's snapshot, skew now closed. Step 4a correct but unreachable -- it lives in the migrator. Suite 623/0."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, **and the ingest corpus excludes ignored paths.**

**hv's DISK MODEL: no status directories, disk becomes OPTIONAL, index plus render-on-demand.** The moment disk is optional, anything the store does not hold is destroyed by the first render. **`realisation.md` 5.1 is the dehydration gate; 5.1b is the attachment asymmetry.**

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. A timestamp not read off a clock is fabricated data, not an approximation. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory.

## DONE 2026-08-18 15:14Z -- THE CARRY LANDED, AND "THE REGENERATION" NEVER EXISTED

**Commits: `86b74b6c` (code), `d980e5b5` (carry), `9a487cc9` (issue bodies), `69033e4e` (ST0057).** Suite 623/0 at the code commit. Binary that ran the carry, verbatim: `dirty-ce532a979fb735aab2da599f4a2b12efe4491e5c`.

**THE HEADLINE: hv's step 4 was not a verb.** "Regenerate the estate" had an acceptance criterion describing a re-ingest from v2 SOURCE. The v2 status buckets were collapsed at `1af21f4e`, so no such source is in the live tree. **Four boards carried it as an approved action with a pinned sequence, and nobody had established what command performed it.** vc found it after I refused to guess at the irreversible step. **The write-back I had queued BEHIND the regeneration turned out to BE it.**

**THE CARRY: 275 eligible / 275 carried / 0 missing / 0 byte-mismatched. Store 0 -> 275.** 56 of 57 threads (the 57th has no eligible file). 106 paths moved of 1330; nothing moved that should not have -- zero `acceptance.md`, zero `todo.md`, zero `steel_threads.md`.

**THE COST: `sync --to-store` wiped 40 issue bodies, and the alarm had already fired.** It named all 40 under _"replacing the store from the extract OVERWRITES"_ before doing anything. **I read the list, recognised it as the expected 40, and classified a correct RED as the expected direction.** AC-03.9 worked perfectly and the operator pattern-matched past it. Recovered 40/40 from a snapshot vc had MOVED rather than deleted; `data-model.md` and the schema had disagreed about where a body lives, both went unimplemented, and 434KB sat in a gitignored store, never canon (D29), never travelling (D34), absent from every clone since the hoist. **The run destroyed the last live copy of data that was already not travelling.**

**STEP 4a IS CORRECT AND UNREACHABLE.** `legacy::preamble` no longer leaves the blank line where it cut the title out, mutation-proved against the estate's own bytes -- **and it is in the MIGRATOR, which this estate will never run again.** ST0010/ST0015 still render the extra line. A fix pointed at a door nobody walks through.

**SKEW CLOSED AFTER THE COMMITS.** `sync --to-store` re-run: store now 40/40 bodies (434,357 bytes) and 275 attachments; canon unchanged; `git status` clean for issues. **While canon was the richer side, `--to-disk` -- the SILENT direction with no warning banner -- was the destructive one.** That inversion is worth remembering: the posture of the two commands is not fixed, it depends on which side is richer.

## TODO

1. **D57-5 IS NEW AND IT IS MINE: a complete text realisation of the whole project into `.backup/text/<UTC>/`, as hv's on-disk fallback.** Not a duplicate of the dehydration gate and vc was precise about why: **the gate proves THE STORE holds it; the export proves A HUMAN CAN GET IT BACK WITHOUT THE TOOL.** Two assurances, one covered. Four requirements: **complete with a PRINTED DENOMINATOR** (a partial export reading as complete is worse than none), `.backup/text/<UTC>/` as a third mechanism in the established never-commit namespace beside `upgrade/` and `db/`, **regenerable and NEVER authoritative** (no import path, `classify` never sees it), and cheap enough to be habitual. **`intent init` is a PRECONDITION of this, not a neighbouring gap** (vc, `e7c11f14`): the natural way to exercise _does everything come back as text_ is to make an empty project, export into it and read it -- so `init` gates the ASSURANCE rather than the onboarding. **And it lands on a refusal of mine whose reason EXPIRES**: `export.rs:192`/`export.rs:208` refuse `--format md` because _"the views are already in the tree"_ -- true today, FALSE the moment the disk model lands, since that is the entire point of it. **RULED: withdrawn as PART of this thread and not before.** Same class as `Issue::body`'s trim -- a claim true when written, expiring on a change already scheduled -- except caught before it shipped rather than after.
2. ~~The attachment write-back~~ **DONE** (`86b74b6c`). `Project::collect_attachments` is the one collector; `legacy::attachments` wraps it for the open/closed axis. **What is NOT done and is the next piece: nothing collects attachments on a COLD warm** -- the carry runs only in `facade::sync_from_disk`, deliberately, because a cold warm must reproduce the committed extract rather than let disk outvote it. A clone therefore gets attachments from canon, which is now correct, but `doctor` has no arm reporting a file on disk that canon does not know about. **That gap is the one 5.1b's divergence rule needs to be observable.**
3. **`sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness** (vc). Worth saying what it is safe FROM.
4. **`created: ST0057` writes three files AND regenerates a tracked `steel_threads.md` while printing one word** (hv's dogfood). A command that modifies a tracked file the user did not name must say so, or they find it in `git status` and have to work out who did it.
5. `upgrade`'s "their content is unchanged" -- a claim nothing computes. `doctor` naming a stale pre-versioning store before a cutover. **`views::info` adds a blank line after a deprecation blockquote** -- visible only on ST0010/ST0015, the only two threads carrying one. **Now UNBLOCKED and sequenced as step 4 above.** AC-10.8 egest; AT-10.2 (probed) / 10.3 / 10.4; `WpStatus::Cancelled`.

## Watch-outs -- the mechanisms, distilled

### A VERSION NUMBER IS A CLAIM ABOUT SHAPE -- a rung, once run anywhere, is PUBLISHED

**My worst defect of the day, and it reached hv rather than a test.** I added `seq` by EDITING rung 10, reasoning: _"the live store is at 9, so rung 10 has not been applied to any durable store, so editing it in place is safe."_ **Valid reasoning, false premise, never checked** -- one `PRAGMA user_version` would have said 10. Two shapes stamped 10, and every read of canon died on `no such column: seq`.

**THE RULE, now on the ladder: once any store has run a rung, changing what that rung PRODUCES requires a NEW rung.** The old rung's output is already stamped and unreachable. **An unchecked assumption WEARING THE SHAPE OF A CHECK is the parent of most of the family below** -- it occupies the slot the check would have gone in.

**vc hit the same class the same day, in a document rather than a store**: they asserted a defect in `data-model.md` from a premise they had not checked, and what saved it was going looking for a rationale so hv would not rule against a straw man -- the ratification was sitting at `data-model.md:428` the whole time. **The save came from checking for a DIFFERENT reason, which is luck rather than method.** **THE CONCLUSION, and the narrowing is the whole of it.** _"Check your premises"_ is uncosted advice: it would not have found any of these and it prices out on contact. The rule that survives is **WHEN SOMEONE STATES A CLAIM AS THE LOAD-BEARING REASON FOR A DECISION, THAT SPECIFIC CLAIM IS THE ONE TO CHECK** -- bounded, and one grep each time. Both of today's aimed saves fit it exactly: vc went looking for the Triage rationale because hv was about to rule on it, and I greped `is_terminal` because vc had made _"terminal is the wrong word"_ the reason for a ruling. **And it is SOCIAL rather than solitary (vc's addition, and it is the load-bearing half): the person who states the reason is not usually the person who can see it is unchecked.** That is an argument for the peer structure, not for individual vigilance -- vc found `data-model.md:428` themselves only because they went looking for something else, since nobody had stated it back to them.

### A HUMAN TYPING THE SEQUENCE EXERCISES WHAT NO TEST EXPRESSES -- 3 for 3 in one day

hv's dogfood beat the suite three times: the schema-10 shape collision (two commands, where 85 legs could not express it), `st new` -> `st triage` -> `st start` where v2 was two steps (_"this is STOOPID"_), and `created:` printing one word while rewriting a tracked index. **The project had even PREDICTED the adjacent drift** -- ic's EXP-04 recorded that `st new -s` now spans two transitions -- **and recorded the SEMANTIC cost while missing the ERGONOMIC one.** Nothing short of a person doing the ordinary thing surfaces "a user creates a thread and starts working on it".

### A SUITE THAT ALWAYS STARTS FRESH CANNOT SEE A MIGRATION DEFECT

Every fixture builds from the current `DDL`, so every test gets the current shape and passes. The remedy is a test whose fixture is **a store this binary did not create**. Same shape as: **a subject that cannot exhibit the defect cannot clear it** -- the gitignored store no clone could hold, and the attachment ordering the path-sorting migrator makes unobservable on the real tree. **In both cases the thing that found it differed from the subject BY ACCIDENT.**

**AND ITS SIBLING, WHICH IS NOT THE SAME SENTENCE (vc): a suite that CONSTRUCTS a state directly never asks whether the VERB THAT REACHES IT produces a state the checker accepts.** Rung 10 was a fixture that could not HOLD the defect; this is a fixture that holds the STATE and skips the TRANSITION -- so **both arms of a contradiction test green while the path between them is broken.** `st cancel` -> `doctor` is the live instance: every doctor test builds its fixture, none drives a verb and then asks the instrument.

### AND THE INVERSE, WHICH COST 434KB: A CORRECT RED, READ PAST BY A READER WHO ALREADY HAD AN EXPLANATION

`sync --to-store` named all 40 issues under **"replacing the store from the extract OVERWRITES"** before it did anything -- AC-03.9's whole design, the warning delivered one moment EARLIER rather than as a receipt afterwards. **I read the list, recognised it as vc's expected 40, and classified the alarm as confirmation that the run was on track.** The instrument was accurate, specific, and correctly timed. It was defeated by my having a story ready that the list fitted.

**This is not the five-greens family, it is its mirror, and it is worse**: a false green needs the instrument to fail, and this needed only a reader. **The tell was available -- "differs on disk" is not "is about to be improved"** -- and I never asked which direction the difference ran.

**The general form: when an alarm matches something you were already expecting, that is the moment to check the DIRECTION rather than the identity.** Recognising the population is not reading the warning.

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
