---
node: cc
name: Control Claude
role: control
session_id: 58ada566-7779-4209-a426-8622a8b8e323
heartbeat_at: 2026-08-18 19:18Z
status: active
focus: "**THE FLAT VIEW WAS RENDERING SIX STATES AS ONE AND TWO OF THEM WERE LIES** -- `intent/todo.md` carried 82 rows and one glyph, so 2 of the 54 rows under `## DONE` were CANCELLED work (ST0010, ST0015) presented as completed. Landed `d8412be` + `7e3210c4`: 82 rows / 1 glyph -> 82 rows / 4 glyphs, [x]52 [~]2 [ ]17 [-]11, 65 rows changed, nothing else in the file moved. **The cut was at `TodoItem`, not at the renderer** -- it carried no status, so `--json` lost the same fact and BOTH FACES AGREED BY BOTH BEING WRONG. **No test existed on that view at all**; six added. **NOW ON AC-04.4** -- `views::write_all` moved mtime on all 266 views every sync; test demonstrated RED (10 of 10) then guarded, green. Data NOT repaired: ST0010/ST0015 are correctly Cancelled in canon. Suite 629/0 before AC-04.4. Upstream FROZEN."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, **and the ingest corpus excludes ignored paths.**

**hv's DISK MODEL: no status directories, disk becomes OPTIONAL, index plus render-on-demand.** The moment disk is optional, anything the store does not hold is destroyed by the first render. **`realisation.md` 5.1 is the dehydration gate; 5.1b is the attachment asymmetry.**

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. A timestamp not read off a clock is fabricated data, not an approximation. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory.

## WHERE THIS STANDS -- WP-03's carry is LANDED and COMMITTED

**`86b74b6c` code / `d980e5b5` carry / `9a487cc9` issue bodies / `69033e4e` + `fd2e4067` ST0057 / `f0a25d8c` + `1a642249` boards / `4ef953db` chore. Suite 623/0 at HEAD.** Detail archived to `.history/20260818/`.

**THE CARRY: 275 eligible / 275 carried / 0 missing / 0 byte-mismatched. Store 0 -> 275.** The disk-to-attachments path did not exist before this -- `legacy.rs` was the ONLY producer of an `Attachment`, so a file a person wrote into a thread directory was carried by nothing. `Project::collect_attachments` is the one collector; the carry runs ONLY in `facade::sync_from_disk`, deliberately.

**"THE REGENERATION" WAS NEVER A VERB.** Its criterion described a re-ingest from a v2 source that `1af21f4e` removed. Four boards carried it as an approved action with a pinned sequence and nobody had established what command performed it. **The write-back queued BEHIND it turned out to BE it.**

**IT COST 40 ISSUE BODIES, RECOVERED 40/40.** `sync --to-store` named all 40 in a warning before acting; I read the list, recognised the expected population, and classified a correct RED as confirmation. Recovered from a snapshot vc had MOVED rather than deleted.

## THE FLAT VIEW -- `d8412be`, and it was not a glyph bug

**`intent/todo.md` rendered SIX states as ONE.** `views.rs::items()` emitted a literal `- [ ]` for every row of every bucket. Because the bucketing is three-valued and `ThreadStatus` is six-valued, **`Completed` and `Cancelled` share the DONE bucket -- so the glyph was the ONLY thing distinguishing them**, and 2 of the 54 rows under `## DONE` were cancelled work presented as completed (ST0010, ST0015).

**THE CUT WAS AT `TodoItem`, NOT AT THE RENDERER** (ic's widening, which I found the second half of). It carried `{id, kind, title, label}` and no status, so `items()` had nothing to compute from **and `--json` had nothing to report** -- a machine consumer could not tell cancelled from completed either. **Both faces lost the same fact, so both AGREED**, which is precisely what `TodoItem`'s doc comment promises they cannot do. The promise was kept and the fact was gone. **Sweep rule that follows: "the value survives to the point of rendering", not "the site reads it from the model"** -- a read-site sweep passes `items()` clean while a struct two hops upstream is still dropping something.

**MEASURED:** 82 rows / 1 glyph -> 82 rows / 4 glyphs. `[x]` 52, `[~]` 2, `[ ]` 17, `[-]` 11; 52 + 2 = 54 is the DONE count. 65 rows changed = 82 less the 17 legitimately blank. Nothing else in the file moved. **Triage and Hold render NOWHERE in the live estate**, so those arms exist only under test -- the ones that rot.

**NO DATA REPAIRED** (ic). ST0010/ST0015 carry `status: Cancelled` correctly; v2 held cancellation TWICE, as a `CANCELLED/` directory AND the field, and the hoist correctly flattened it -- **which left the field as the sole carrier, so a view discarding it was TOTAL loss, not cosmetic.**

**NO TEST EXISTED ON THAT VIEW AT ALL** -- no `#[cfg(test)]` in `views.rs`. Not a weak test; nothing looked. Six added, all driving the real renderer over fixtures carrying both states, because a unit test of `glyph()` alone passes while the renderer emits a constant. **`glyph()` is exhaustive with NO wildcard**: v2's `status_box` has `*) printf '?'`, so a status v2 did not know rendered as a shrug and nobody learned. A seventh state must fail to compile.

## AC-04.4 -- IN FLIGHT, red demonstrated then guarded

**`views::write_all` called `fs::write` UNCONDITIONALLY for every view**, so a byte-identical re-emission moved mtime on all 266 (57 info + 57 acceptance + 150 WP + steel_threads + todo) every sync -- and `file_index` derives clean/changed from mtime, so a no-op sync marked the whole estate changed.

**The idempotence test beside it PASSES while this is live**, because it compares bytes. **Idempotent bytes is not idempotent writing.** Test written first and demonstrated RED (10 of 10 views moved), then the guard. **It does not sleep or trust timestamp resolution** -- every view is aged an hour between runs, so a skipped file keeps the aged stamp; a clock-racing test would pass vacuously on a coarse filesystem, which is the exact failure it exists to detect.

## `status_reason` -- a field four verbs DEMAND and no human face showed

**Found by ic sweeping `views.rs` under the widened rule I gave them**, which is the rule that found it: _the value survives to the point of rendering_, not _the site reads it from the model_. There was no view site to score -- `status_reason` survives into the model, into canon, into the DB and into the committed SDL, and dies at the render boundary.

**`st cancel` / `st hold` / `st reopen` / `wp reopen` REFUSE without a reason, and the refusal argued for the field while hiding it**: _"...and in the event log as part of the decision, which is what lets anyone reconstruct why later."_ I went to check the event log expecting the promise to rest there -- **`intent --help` declares 34 verbs and not one reads it.** `search` does not reach it either (`ingest.rs` never mentions the field). **A promise with no reader on EITHER carrier.**

**ic's narrowing is the honest claim and I use it everywhere: MACHINE-VISIBLE, HUMAN-INVISIBLE.** `schema.graphql:292` (Thread) and `:413` (WorkPackage) expose `statusReason`. The wider "the value is lost" dies on "it is in the schema"; the narrow one survives.

**FIXED on all four faces** -- `views.rs` thread + WP frontmatter, `render.rs` `st show` + `wp show` -- **and the frontmatter key is emitted only when there IS a reason**, so nothing in the live estate churns today. vc RULED the false clause struck without waiting for hv; **a reader for the event log is hv's**, and ic's point makes it cheap: `event.schema.json` is committed, so it is **a built carrier with no door**, not an unbuilt feature.

**This does NOT close AC-03.12 and I am not claiming it** (vc). The field carries only the CURRENT status's reason -- any transition without one clears it -- so rendering it answers _why is it on hold now_ and never _reconstruct why later_. **Fixing the visible half would make the promise look kept.**

**ic's CONTROL ORDERING is now my habit, and it is in both e2e tests: prove the value reached canon, THEN ask the face.** Their first WP drive hit an unfired fixture -- the gate refused the `wp done`, so `wp reopen` returned `ok: already WIP` writing nothing, and every face came back empty. **Emptiness from a face that does not render and emptiness from a verb that never recorded are indistinguishable when you only ask the face.**

## D42 CAUGHT ME, IN A TEST ABOUT MEASUREMENT DISCIPLINE

My AC-04.4 test aged the views with `SystemTime::now() - 3600s`. **`one_clock.rs` failed the suite naming my file** -- there is no clock in this workspace at all. Replaced with a FIXED synthetic stamp (`UNIX_EPOCH + 1_000_000_000s`), which is **stronger for obeying the rule**: the assertion became "still exactly this constant" rather than "still roughly where I put it". **Then re-ran the mutation test, because changing the mechanism invalidates the earlier RED.** 10 of 10 red with the guard off; green with it on. The guard caught it, not the author -- which is the case for structural guards over careful authors.

## OPEN -- four things queued, none started

1. ~~The `views::info` blank line~~ **DONE by vc while they were in canon, and it was never a rendering bug.** The v2 source had notice / blank / `# H1` / blank / status-list; **the migrator correctly lifted the H1 into a model field and left BOTH of its blank lines behind**, so `\n\n\n` was a removed line's ghost in the stored `preamble`. Repairing it to `\n\n` RESTORES fidelity rather than reformatting, so migration.md's "nothing reformatted" is not engaged. Verified: both preambles now run [1, 2], and a full `--to-store` + `--to-disk` round trip leaves the four views untouched. **My `legacy::preamble` fix stays correct and stays unreachable.**

   **AND IT WAS NOT COSMETIC, WHICH NEITHER OF US SPOTTED WHILE WE BOTH LOOKED AT IT.** Every sync regenerated four views, the pre-commit guard reverted them, the next sync re-created them -- **a tree that cannot go clean is a tree whose binary can never name a clean commit, so this was BLOCKING AC-11.5's binary arm** that dc carried all afternoon as picked-up-not-started. I filed it as churn and priced it as an annoyance; the cost was somebody else's blocked work package.

2. **Nothing collects attachments on a COLD warm, and `doctor` has no arm for it.** The carry is `sync_from_disk`-only so a cold warm reproduces the committed extract rather than letting disk outvote it -- correct, but it means no check reports a file on disk that canon does not know about. **That arm is what makes 5.1b's divergence rule observable.**
3. **`critic rust` and `critic shell` arm ZERO rules** (0 of 6, 0 of 7). dc's Half A relit the gate -- `bin/intent:55` carries `critic` now, announced to me first, protocol working -- so the five fail-open lines are gone. **But a green from either critic means "nothing asked a question", not "clean", and I move the most `.rs`.** Half B scoped, not built.

4. **THE BINARY MARKER -- (a) WITHDRAWN, (b) IS THE REAL ONE AND IS NOT MINE TO CLOSE.**

   **(a) STALENESS -- WITHDRAWN 2026-08-18, AND THE WITHDRAWAL IS WORTH MORE THAN THE FIX WOULD HAVE BEEN.** vc refused the reversal and asked what expired the recorded reason. **Nothing had -- and measuring it to answer found HALF MY OWN RECORDED REASONING WAS FALSE.** `source_commit.rs` claimed "HEAD moves when ANYONE commits ANYTHING"; **`.git/HEAD` is rewritten on a BRANCH SWITCH, not on a commit** -- measured, `.git/HEAD` six months stale while `.git/refs/heads/main` and `.git/logs/HEAD` moved seconds earlier with the commit just landed. So the rebuild storm I priced against could never have happened.

   **The refusal stands on its OTHER limb, which was always sufficient alone: emitting ANY `rerun-if-changed` REPLACES cargo's package-file default.** A line naming `.git/HEAD` would swap a trigger that follows the code for one following a file that almost never moves, leaving the embed stale on CODE changes, permanently and silently. **The naive fix is STRICTLY WORSE than the gap, and the wrong reason was the one that made it look obviously correct.** Corrected in place at `a466f90f`, not deleted. The expressible form if freshness is ever wanted is recorded there: `rerun-if-changed=src` PLUS `.git/logs/HEAD`. **Staleness is witnessed twice** (`b11ca6ac` at HEAD `010b2bbf`; `dirty-4ef953db` at HEAD `c83f624c`) and costs MEASUREMENT, not releases -- `publish` refuses `dirty-` outright.

   **(b) NON-IDENTITY -- dc's, and the one that actually bit. NOT closed by any trigger.** The marker is `dirty-<HEAD>`, so two behaviourally different dirty builds at one commit share one value. **It names a commit; it was never an identity.** ic ate a false RED from exactly this -- my rebuild landed between their two arms, behaviour moved, marker held byte-identical. dc: not a wrong answer, a right answer to a different question. **Closed by vc's AC-10.11** -- a paired reading's binary identity must be a content hash, never a self-reported marker -- and not by anything of mine.

   **Two verified traps that stay true regardless** (ic, out of that file): **`git archive` is the WRONG route to a clean tree** -- no `.git` means `rev-parse` fails and the embed stamps `unknown`, STRICTLY WORSE than dirty; the property is RETAINS `.git` AND IS CLEAN, so a clone at a sha qualifies and an archive extract never can. And **read the marker OUT OF THE BINARY** -- a manifest states what was meant to be built, only the artefact answers what was.

   **`target/release/` is a SHARED MUTABLE ARTEFACT other nodes read.** Snapshot and sha256 before measuring against it; do not read it live while peers are in the tree. Announced to dc and ic.

## TODO

1. **D57-5 IS NEW AND IT IS MINE: a complete text realisation of the whole project into `.backup/text/<UTC>/`, as hv's on-disk fallback.** Not a duplicate of the dehydration gate and vc was precise about why: **the gate proves THE STORE holds it; the export proves A HUMAN CAN GET IT BACK WITHOUT THE TOOL.** Two assurances, one covered. Four requirements: **complete with a PRINTED DENOMINATOR** (a partial export reading as complete is worse than none), `.backup/text/<UTC>/` as a third mechanism in the established never-commit namespace beside `upgrade/` and `db/`, **regenerable and NEVER authoritative** (no import path, `classify` never sees it), and cheap enough to be habitual. **`intent init` is a PRECONDITION of this, not a neighbouring gap** (vc, `e7c11f14`): the natural way to exercise _does everything come back as text_ is to make an empty project, export into it and read it -- so `init` gates the ASSURANCE rather than the onboarding. **And it lands on a refusal of mine whose reason EXPIRES**: `export.rs:192`/`export.rs:208` refuse `--format md` because _"the views are already in the tree"_ -- true today, FALSE the moment the disk model lands, since that is the entire point of it. **RULED: withdrawn as PART of this thread and not before.** Same class as `Issue::body`'s trim -- a claim true when written, expiring on a change already scheduled -- except caught before it shipped rather than after.
2. ~~The attachment write-back~~ **DONE** (`86b74b6c`). `Project::collect_attachments` is the one collector; `legacy::attachments` wraps it for the open/closed axis. **What is NOT done and is the next piece: nothing collects attachments on a COLD warm** -- the carry runs only in `facade::sync_from_disk`, deliberately, because a cold warm must reproduce the committed extract rather than let disk outvote it. A clone therefore gets attachments from canon, which is now correct, but `doctor` has no arm reporting a file on disk that canon does not know about. **That gap is the one 5.1b's divergence rule needs to be observable.**
3. **`sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness** (vc). Worth saying what it is safe FROM.
4. **`created: ST0057` writes three files AND regenerates a tracked `steel_threads.md` while printing one word** (hv's dogfood). A command that modifies a tracked file the user did not name must say so, or they find it in `git status` and have to work out who did it.
5. `upgrade`'s "their content is unchanged" -- a claim nothing computes. `doctor` naming a stale pre-versioning store before a cutover. **`views::info` adds a blank line after a deprecation blockquote** -- visible only on ST0010/ST0015, the only two threads carrying one. **Now UNBLOCKED and sequenced as step 4 above.** AC-10.8 egest; AT-10.2 (probed) / 10.3 / 10.4; `WpStatus::Cancelled`.

## Watch-outs -- the mechanisms, distilled

**A PROPERTY MEASURED ON ONE CASE, ASSERTED ABOUT THE ADJACENT ONE.** Three times in one afternoon, all caught by the node next door and none by any check: vc's manifest control measured the neighbouring directory; dc put a superlative on ic's unverified mechanism; dc measured ic's binary pair (same HEAD, no staleness) and asserted it of mine (genuinely stale). **The tell is that the finding is TRUE -- of the thing that was actually measured.** Re-measure on the instance you are about to name, every time, even when the cases look identical. Especially then.

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
