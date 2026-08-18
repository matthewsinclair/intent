---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-18 09:17Z
status: paused
focus: "PAUSED at an ultra-aggressive fold. **vc's TWO BUILD ITEMS ARE LANDED: rung 9 `issues.body` and rung 10 `attachments`** (`36bc02c5`, `d73efed9`), plus `related[]` and **rung 11**, which exists because I EDITED rung 10 after a store had already run it. **Waiting on hv, not on work: question 7 (163 typed docs with no destination) and the estate regeneration, which happens ONCE after that ruling because the ruling changes what the migration writes.** The regeneration is MINE, in a window vc calls. Upstream FROZEN; push `local` only."
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

1. **THE 163 TYPED DOCS HAVE NO DESTINATION** (vc measured 748 sections). They are `.md`, so the extension rule never excluded them -- **being named in `THREAD_PROSE` is what makes `classify` call them typed.** hv has it as question 7; if vc's recommendation carries, **the change is DELETING that constant, not adding a field.**
2. **The estate regeneration -- MINE, one window, vc calls it, after question 7.** Canon holds no bodies and no attachments; the store holds the TRIMMED bodies the defective test run left. **Neither `sync` direction is correct today** -- `--to-store` drops them, `--to-disk` reintroduces exactly what I reverted -- and the right content is only derivable by a fresh migration from the v2 markdown.
3. **The attachment write-back.** vc ruled the policy (5.1b): authority follows AUTHORSHIP, so a view divergence means the FILE is stale and an attachment divergence means the STORE is stale, and `organize` resolves neither.
4. **`sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness** (vc). Worth saying what it is safe FROM.
5. `upgrade`'s "their content is unchanged" -- a claim nothing computes. `doctor` naming a stale pre-versioning store before a cutover. AC-10.8 egest; AT-10.2 (probed) / 10.3 / 10.4; `WpStatus::Cancelled`.

## Watch-outs -- the mechanisms, distilled

### A VERSION NUMBER IS A CLAIM ABOUT SHAPE -- a rung, once run anywhere, is PUBLISHED

**My worst defect of the day, and it reached hv rather than a test.** I added `seq` by EDITING rung 10, reasoning: _"the live store is at 9, so rung 10 has not been applied to any durable store, so editing it in place is safe."_ **Valid reasoning, false premise, never checked** -- one `PRAGMA user_version` would have said 10. Two shapes stamped 10, and every read of canon died on `no such column: seq`.

**THE RULE, now on the ladder: once any store has run a rung, changing what that rung PRODUCES requires a NEW rung.** The old rung's output is already stamped and unreachable. **An unchecked assumption WEARING THE SHAPE OF A CHECK is the parent of most of the family below** -- it occupies the slot the check would have gone in.

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
