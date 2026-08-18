---
node: cc
name: Control Claude
role: control
session_id: 58ada566-7779-4209-a426-8622a8b8e323
heartbeat_at: 2026-08-18 20:14Z
status: active
focus: "**HOLDING FOR hv DIRECTLY. vc RELAYS THAT hv SEQUENCED ME item 1 THEN WP-01 -- I DO NOT START ON A RELAY**, quoting vc's own 12:31Z _do not start on my relay ... I am not relaying it again_. Ready in minutes on one word. **Everything blocking vc's row is DONE and read-only: the verb set is 27 MUTATING VERBS THROUGH 9 `apply` SITES, +4 non-apply writers = 31 VERBS / 6 COMMIT SITES** -- so _enumerate the nine_ would print an INTERNAL denominator into a row whose point is the observable boundary. **`ac gate` UNVERIFIED, so 27 is a FLOOR.** Upstream FROZEN."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, **and the ingest corpus excludes ignored paths.**

**hv's DISK MODEL: no status directories, disk becomes OPTIONAL, index plus render-on-demand.** ST0057 is IN the 3.0.0 gate (hv). `realisation.md` 5.1 is the dehydration gate; 5.1b is the attachment asymmetry.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory. **`one_clock.rs` enforces it structurally and it caught ME today**, in a test about measurement discipline.

## THE CHURN FIX: A DEAD GUARD, A BARE ID, AND NO CRITERION AT ALL

**EVERY ACCEPTANCE ID IS THREAD-QUALIFIED FROM NOW ON -- `ST00NN AC-XX.Y`, NEVER BARE** (vc's convention, adopted, and I asked them for the same back). **I had been citing a bare `AC-04.4` and BOTH THREADS CARRY ONE.** Verified myself rather than taken on report:

| id                 | what it actually says                                                      | state                                                                                 |
| ------------------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| **ST0056 AC-04.4** | every facade error is typed and renders a remedy with its full cause chain | **satisfied: yes**, `error_remedies.rs` GREEN -- **unrelated to any of this**         |
| **ST0057 AC-04.4** | **`organize`** run twice changes nothing, including mtimes                 | **`organize` IS RETIRED IN THIS BUILD** -- verified verbatim -- and ST0057 is 0 of 43 |

**AND I AM CLAIMED ON ST0056/10, SO THE NATURAL RESOLUTION OF THE BARE ID LANDS ON THE GREEN ROW.** An id that resolves to a satisfied criterion is the worst possible ambiguity: it reads as _already handled_.

**THE FIX SERVES NO CRITERION THAT EXISTS. I SEARCHED BOTH CONTRACTS INDEPENDENTLY AND REPRODUCED vc's RESULT: the ONLY mtime criterion in either thread is ST0057 AC-04.4, about the retired command.** ST0056 AC-03.3 is the READ side -- detecting a same-size same-mtime rewrite by hash. Nothing anywhere says a no-op sync writes zero files. **vc is minting the row and told me to build rather than wait; its subject is the WRITER, so ~364 is the denominator and 266 never was.**

**WHY NOTHING EVER TRIPPED, AND IT IS STRUCTURAL RATHER THAN AN OVERSIGHT: ST0056 AC-03.2 IS CONTENT DETERMINISM -- _same model, same bytes, twice_ -- AND THE CHURN SATISFIES IT PERFECTLY.** The estate HAD an idempotence criterion; the defect is invisible to it by construction. **A criterion can be green, correct and complete about its own subject while the thing next to it rots.**

**MY SELF-CORRECTION REACHED RATIFIED CANON, NOT JUST A DOC COMMENT.** ST0057 AC-04.4's own text justifies measuring mtime over a content diff because the defect _"corrupts `file_index`'s clean/changed state"_. **`FileState` is sha256-only, so that reason is false -- and vc wrote it.** The MEASUREMENT stands; the RATIONALE does not. vc is amending to the three real costs.

## THE CHURN IS THE HOT PATH, AND IT BREAKS THE CRITERION WE WERE BOTH ABOUT TO WRITE

**READ FROM SOURCE, NOT OBSERVED AS AN MTIME COUNT** -- I will not drive a mutating verb on the live estate while peers are measuring, and the distinction is the point. `apply` (`facade.rs:2664`) calls `self.projection(&next, &changed_threads, &changed_issues)`: **`next` is the FULL canon, the other two are SUBSETS.** Inside `projection` (`:1259`) the subsets drive canon JSON -- correctly narrow -- while **`views::render_all(.., canon /* FULL */, ..)` at `:1272` adds ALL 266 VIEWS, EVERY TIME.**

**SO EVERY MUTATING VERB REWRITES 266 VIEWS TO CHANGE ABOUT TWO. NINE `apply` CALL SITES, SO THAT IS MOST OF THE MUTATING SURFACE.** The defect is not a rare no-op sync; **`intent st start` touches 266 files.**

**AND IT FALSIFIES THE FORM BOTH vc AND I HAD.** For a mutating verb _zero mtimes move_ is FALSE and SHOULD be -- the mutated thread's `info.md`, `steel_threads.md` and `todo.md` genuinely change. **A row saying "run twice writes zero files" is INAPPLICABLE to the nine verbs where the churn is worst, so they sit outside the denominator while looking covered.** Sent to vc before they minted it. **Proposed: A WRITE MOVES MTIME ON EXACTLY THE FILES WHOSE BYTES CHANGED, AND NO OTHERS** -- measured at every user-visible verb that writes the estate, verb set enumerated and printed. It grades correctly everywhere rather than only on no-ops, and its denominator is _files in the write set_, always well defined, instead of _files that should not have changed_, which needs a per-verb judgement. **"Run twice writes zero" demotes to a COROLLARY: it was the easiest instance to measure, never the property.**

**SIX COMMIT SITES, NOT SEVEN, AND THE MISCOUNT IS THE GOOD NEWS:** `:768` (`upgrade`), `:1023` (`sync_to_disk`), `:1084` (`sync_from_disk`), `:1192` (`todo_update`), `:2676` (`apply`), `migrate.rs:338`. **`:1259` is `projection` -- a BUILDER, shared by three of the six, which is exactly why ONE skip in `WriteSet::commit` covers them all.**

## THE VERB SET IS 27, NOT 9 -- AND 9 IS THE INTERNAL NUMBER

vc asked me to enumerate "the nine `apply` sites" into the new row. **NINE IS THE COUNT OF FACADE METHODS. FOUR OF THE NINE HAVE ZERO CLI CALLERS BY NAME** -- they are setters behind wrappers, measured:

| `apply` site        | user-visible verbs it serves                                                                  |
| ------------------- | --------------------------------------------------------------------------------------------- |
| `st_new`            | `st new`                                                                                      |
| `set_thread_status` | `st start` `done` `cancel` `triage` `hold` `resume` `reopen` `reinstate` -- **8**             |
| `wp_new`            | `wp new`                                                                                      |
| `wp_rescope`        | `wp rescope`                                                                                  |
| `set_wp_status`     | `wp start` `done` `reopen` `unstart` -- **4**                                                 |
| `set_ac_state`      | `ac satisfy` `unsatisfy` `descope` `rescope` `withdraw` `reinstate` -- **6**                  |
| `at_set`            | `at green` `red` `na` -- **3** (ONE CLI site, `render.rs:1033`, status passed as a parameter) |
| `issue_add`         | `issues add`                                                                                  |
| `set_issue_status`  | `issues close` `open` -- **2**                                                                |

**27 USER-VISIBLE MUTATING VERBS THROUGH 9 `apply` SITES.** Plus the four non-`apply` writers -- `sync --to-disk`, `sync --to-store`, `todo update`, `upgrade` -- for **31 VERBS ACROSS 6 COMMIT SITES.**

**`ac gate` IS UNVERIFIED and I flagged it rather than let vc discover it:** a declared `ac` subcommand, absent from the six `set_ac_state` wrappers, so it either reaches `apply` by a route I did not trace or does not write. **27 IS A FLOOR, NOT A TOTAL.** A number with a named hole beats a tidy one.

**AND THE SYMMETRY IS THE FINDING: vc CAUGHT MY DENOMINATOR REACHING INTO `WriteSet`, AND ASKED FOR AN ENUMERATION OF AN INTERNAL NUMBER, IN THE SAME MESSAGE.** We each reached for the internal figure once, in opposite directions, inside one exchange. **So POPULATION and SUBJECT DEPTH are ONE LIMB SEEN FROM TWO ENDS -- the denominator IS a subject, and it has a depth.** Proposed to vc that the kit collapse them.

**vc's DENOMINATOR REFINEMENT TAKEN IN FULL, and it is my own argument used on me correctly:** I ruled the subject must sit at the observable boundary, then set the denominator to _files in the write set_. **`WriteSet` is internal.** The measurement is **the FILE ESTATE before and after** -- `moved == changed` as SETS -- which needs no internal type and additionally catches a write that never joins a set.

## HOLDING ON A RELAY, ON MY OWN PRECEDENT THAT vc AFFIRMED

**vc relays that hv sequenced me item 1 then WP-01. I DO NOT START ON A RELAY.** vc's own 12:31Z entry, still live in my inbox: _"**DO NOT START ON MY RELAY.** hv is in my session and you need the go from hv directly. Your precedent on this is right and I am not relaying it again."_ **hv's last word to me DIRECTLY was stop and wait.**

**This is not doubt that hv ruled -- I expect they did. The precedent is that a relay and a mistake are INDISTINGUISHABLE from where I sit**, which is why it cannot be case-by-case. **Nothing is lost by the wait: everything that was blocking vc's row is above, done, and read-only.**

## A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT

vc's rule, four limbs, **one live example of each from three nodes inside one day** -- and it is one rule rather than four because the limbs are found by asking the same question:

| limb              | the freedom left open                                                                      | today's instance                                                        |
| ----------------- | ------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| **INSTRUMENT**    | git and mtime give OPPOSITE answers                                                        | my AC-01.4 question -- and the fix was in the criterion, not my reading |
| **SUBJECT DEPTH** | `WriteSet` vs `intent sync`; an internal subject lets the test reach PAST the thing tested | `view_determinism.rs` drives `write_all`, passes, estate churns         |
| **POPULATION**    | 20/20 was views only; the writer's denominator is ~364                                     | vc's, and my 266 was wrong too                                          |
| **PIN**           | measure at a NAMED COMMIT, never at `HEAD`, or the subject moves under the instrument      | the marker/artefact split                                               |

**THIS SECTION IS THE SUBJECT-DEPTH LIMB EATING ITS OWN TAIL.** I argued for the verb boundary over the type; vc caught that ONE verb under-covers; mapping the verbs found the PROPERTY ITSELF was wrong. **Three passes, three nodes, and each was only possible because the previous one was written down.**

## THE GUARD ITSELF: MY FIX LANDED ON A PATH NOTHING CALLS

**vc pre-registered the prediction before rebuilding and it FAILED: sync 1 moved 20 of 20, sync 2 moved 20 of 20.** Two churny syncs in a row is my own stated form of the finding. **I verified the load-bearing half independently: `views::write_all` has NO production caller -- every caller is in `tests/`.** So the guard I committed at `843a69ce` is real, correct, and reaches nothing.

**vc's CONCLUSION is right and vc's LOCATED CAUSE is wrong, and the difference would have cost a second dead guard.** They named `facade.rs:125` and `:150` as the live path; those are `converge_gitignore` and `stamp_version`, which write `.gitignore` and `config.json` and have never written a view. **Measured live path: `Facade::projection` (facade.rs:1253) -> `views::render_all` (:1272) -> `WriteSet` -> `write_set.rs:254` `write_atomically`, which is temp-file-plus-rename.** A rename swaps in a new inode, so **that path CANNOT be mtime-idempotent without an explicit skip.** Three production entry points, all through `WriteSet`: `projection` (both sync directions + `apply`), `todo_update` (:1188), `migrate::assemble` (:338).

**THE DENOMINATOR IS NOT 266.** `projection` adds 57 `thread.json` + 40 issue JSON to the same unconditional set, plus the event log: **~364 paths churn per no-op sync, CANON INCLUDED.** vc's 20-of-20 was a subset and my 266 was too.

**WHY IT WAS INVISIBLE: `projection`'s own doc comment says "THE ONE PLACE THE db -> disk DIRECTION IS EXPRESSED", and `write_all` is a second expression of exactly that.** A Highlander violation is what let a correct guard be unreachable. `view_determinism.rs` is GREEN on a property of a function nothing calls -- **the file that could not see the defect now also cannot see the fix.**

**THE COST I WROTE INTO THE DOC COMMENT IS FALSE, AND IT IS MINE.** `write_all`'s comment claims the mtime move matters because "`file_index` reads it to decide clean from changed, so a sync that changed nothing marked the whole estate changed". **`sync.rs:317-326` decides `FileState` from SHA-256 ALONE**, and the module doc says stat is never a gate -- "size and mtime are carried as reporting metadata" -- on vc's own 2026-08-14 ruling that the contract governs over `design.md`. **The churn does not touch the index.** The real costs: the criterion is ratified and unmet; it defeats every external mtime instrument INCLUDING the one vc measured with; and touching 364 files per no-op sync is the opposite of what ST0057 is about. **SIXTH instance of the class in two days and THE FIRST NO PEER CAUGHT -- found only because I went to re-measure my own sentence instead of citing it.**

**PROPOSED, NOT STARTED, HOLDING FOR hv: put the skip in `WriteSet::commit`, not in views.** `record()` has ALREADY read the prior content, so **the comparison costs no I/O at all** -- unlike the views guard, which added a read. `Prior.written: false` is the EXISTING semantics for "nothing to undo here", so rollback stays correct with no new state and no new field. **Then make `write_all` build a `WriteSet` and commit it rather than deleting it**: its six test files keep working unchanged and start exercising the mechanism the estate runs. **Red-first must be driven through `Facade`, or it proves nothing a second time.** **vc CONFIRMS the location and the fix, BOTH DERIVED INDEPENDENTLY this morning before my message landed** -- two nodes reaching one location separately is worth more than either reaching it once -- and their board's wrong pointer is struck.

**vc's CAUTION ON THE RED, AND IT IS THE STEP THAT DECIDES WHETHER ANY OF THIS IS REAL: DRIVE THE WHOLE-ESTATE DIRECTION, NEVER A SINGLE-THREAD PROJECTION.** A per-thread `projection` writes a SUBSET, and **a subset that happens to be entirely changed goes GREEN against an unguarded writer** -- the discriminating case is simply absent from the population. **THAT IS THE THIRD INSTANCE OF ONE SHAPE IN TWO DAYS AND IT DESERVES ITS OWN NAME: A PROBE WHOSE POPULATION CANNOT CONTAIN THE FAILURE IT TESTS FOR.** `sync` printing _the store and the extract agree_ over **0 == 0**; `git status` confirming an **mtime-only** prediction it is structurally blind to; and now an all-changed subset vouching for a skip that never fired. **Every one returns the right answer for the wrong population, and every one reads as a pass.**

## NEXT: ST0057 WP-01 -- STARTED, REVERTED, RESUMABLE IN MINUTES

**The code worked and the tree is green because I took it back out.** vc cleared me to move the live estate; hv called a compact first. Leaving 68 fixture-caused failures under four peers who are actively measuring is a cost nobody should pay for my convenience -- **ic asked explicitly not to be measured inside someone else's window.**

**PATCH: `<session-scratchpad>/wp01-canon-relocation.patch`, 159 lines, `export.rs` + `project.rs`.** If it is gone, it is 30 minutes of re-typing and every decision is below.

**WHAT WAS BUILT:** `canon_dir()` = `intent/.canon/`; `canon_st_dir()`; `thread_json` -> `.canon/st/<ID>.json`; `issue_json` -> `.canon/issues/<nnnn>.json`; `issues_dir()` repointed (**the whole directory moves -- it held nothing but `<nnnn>.json`; an issue has no realised markdown to leave behind**); `thread_ids()` reads `.canon/st/` by file stem instead of walking `st/` for a nested file; **`classify`'s `thread.json` arm KEPT with a note saying why it is not dead** -- a v2 tree has one, and so does a tree caught mid-move, and it is what keeps a stale canon file out of the attachment carry.

**THE HIGHLANDER FIX THAT FELL OUT, AND ITS OWN COMMENT PREDICTED IT:** the exporter spelled the canon path independently while every reader resolved `Project::thread_json`. Its neighbouring comment records the issue arm having ALREADY shipped that bug -- `issues/46.json` written where readers opened `issues/0046.json`, "two ends had to agree by convention and did not". **Relocating would have re-created it at the thread arm.** Both now call one `canon_thread_rel` / `canon_issue_rel`. Comment tense corrected too: it described the fixed bug in the present, which is history reading as state.

**WHAT IS LEFT, in order:**

1. **Repoint ~15 test fixtures that spell `intent/st/<ID>/thread.json` independently** -- the same Highlander problem one level down. `canon_thread_rel` is `pub`, so tests can resolve through it and never spell it again. Failing binaries measured: `acceptance_surface`, `cli_end_to_end`, `declared_values_are_enforced`, `issues_surface`, `literal_stdout_parity`, `search_surface`, `self_loop_voice`, `upgrade_command`, `export_round_trip`, `facade_acceptance`, plus lib unittests. **Every failure is "no steel thread ST0001 in this project" -- the fixture writes canon where the tool no longer looks. Not a design problem.**
2. **ic MEASURED the parity-harness exposure: 3 targeted edits, NOT a path migration.** `realise_plan.sh:44` (ic's, breaks), `canon_commit_check.sh:82,93,198,199,203` (dc's, breaks), `gen_register.sh:256` (**a GENERATED doc cell -- fails in the QUIET direction: nothing goes red and the register just becomes wrong about where canon lives**). **HAZARD ic measured so I would not hit it: a grep for `intent/st/` returns 17 of 41 tools, and most MUST NOT be touched** -- they are comments correctly describing the v2 layout, and a mechanical `s|intent/st/|intent/.canon/|` would rewrite true statements about v2 into false ones, silently, in the comments that explain why the migration works. Another 17 are the harness's own directory, which does not move.
3. **The live move: 57 thread + 40 issue canon files. vc CLEARED it (`25605e6b`) and will not touch ST canon until I say it landed. Ping vc after.** **REQUIRED STEP, NOT A HABIT (ic): after every WP-01 rebuild, RUN `st list` AND READ THE ROW COUNT.** It must be 57. **No shape check can substitute** -- the wiping binary passed one perfectly -- and a row count is the ONLY cheap instrument that sees which canon path the binary actually resolved.
4. **The four ATs**, all to-write: `canon_relocation.rs`, `canon_clone_completeness.sh`, `canon_relocation_roundtrip.rs`, `canon_concurrent_diff.sh`.
5. **AC-01.5, minted by vc at `da3a52aa` on my report** -- see the hazard below.

**AC-01.2 IS CHECKED BY CLONING, NEVER BY READING `.gitignore`** -- the question is what git DOES, not what a rule appears to say. AC-01.3 is AC-02.6 applied to the move, denominator printed. AC-01.4 rejects the consolidated `threads.jsonl` (D57-1 option B) by editing two threads and observing the changed-path set.

## THE `.canon/` HAZARD -- structural, and AC-01.2 only catches it at check time

```
intent/.treeindex/   ignored
intent/.cache/       ignored
intent/.backup/      ignored
intent/.canon/       MUST BE COMMITTED
```

**Every existing `intent/.<x>/` is gitignored, so the convention reads "a dot directory under intent/ is local and never travels".** `.canon/` is the single deliberate exception. **A future tidy-up adding `intent/.*/` to `.gitignore` is natural, tidy-looking and correct-seeming, and would silently un-commit the entire estate** -- D29's failure with the whole model behind it, and the same shape as the 434KB of issue bodies that lived only in a gitignored store today. **AC-01.2 checks the STATE by cloning; AC-01.5 refuses the EDIT. The gap between those two moments is where this class lives.** The reasoning is in `canon_dir()`'s doc comment so it is re-derivable at the source even if the guard is never built.

## ORDERING: RESOLVED AGAINST MY OWN RECOMMENDATION

**AC-01.4 IS MEASURED BY GIT, CONTENT-BASED, so the churn CANNOT pollute it and WP-01 IS NOT ORDERED BEHIND THE `WriteSet` SKIP** (vc, three independent legs: the criterion's word is _diffs_ and mtime has no diff; its PURPOSE is rejecting consolidated `threads.jsonl`, a version-control GRANULARITY property mtime cannot express at all; and a sync moved 10/10 view mtimes while `git status` reported 0). **My own correction gives it a fourth leg I did not claim: `entry_for` decides `FileState` from sha256 alone, so Intent's OWN change detection is immune too. Nothing in the estate that matters is mtime-gated.**

**SO I WITHDRAW THE ORDERING ARGUMENT.** It had one load-bearing leg -- _if AC-01.4 reads mtime this is a hard block_ -- and the condition resolved against it. What is left (S lands green, L takes the suite red, removing a false green before a 97-file move) is PREFERENCE, not blocking, and **the sequence is hv's free choice.** Re-litigating a conditional argument after its condition fails is how a preference gets dressed as a constraint.

**AND THE QUESTION ITSELF WAS THE FINDING (vc): a criterion that does not name its instrument is defective when two plausible instruments disagree.** AC-01.4 will name `git diff --name-only`; vc owes a sweep across 118 + 43 rows. **I could not read the criterion correctly and the fix is in the criterion, not in the reading.**

## OPEN -- what is actually mine

1. **WP-01, above.**
2. **The cold-warm collection gap `doctor` has no arm for -- PARKED BEHIND WP-01, on vc's ruling and my reasoning.** The arm reports "a file on disk that canon does not know about", and **that sentence CHANGES MEANING under sparseness**: an absent file stops being an anomaly and a present-but-unknown one becomes the interesting case. Building it now would encode today's dense-disk assumption into the one check whose job is to police the sparse one. **I have already eaten that exact error once, with the replacing write-back.**
3. **`critic rust` and `critic shell` arm ZERO rules** (0 of 6, 0 of 7). dc's Half A relit the gate; a green from either still means "nothing asked a question", and I move the most `.rs`. Half B is dc's.
4. **THE BINARY MARKER -- (a) WITHDRAWN, (b) NOT MINE TO CLOSE.** **(a) STALENESS: withdrawn, and the withdrawal is the finding.** vc refused the reversal and asked what expired the reason. Nothing had -- **and measuring it found HALF MY OWN RECORDED REASONING WAS FALSE.** The comment claimed "HEAD moves when ANYONE commits ANYTHING"; **`.git/HEAD` is rewritten on a BRANCH SWITCH, not a commit** -- measured, six months stale against `refs/heads/main` moving seconds earlier. **The refusal stands on its other limb, sufficient alone: emitting ANY `rerun-if-changed` REPLACES cargo's package-file default**, so naming `.git/HEAD` would leave the embed stale on CODE changes, permanently and silently. **The naive fix is strictly worse than the gap, and the wrong reason was the one that made it look obviously correct.** Corrected in place at `a466f90f`. **(b) NON-IDENTITY (dc's, the one that actually bit):** `dirty-<HEAD>` gives two behaviourally different dirty builds one value. It names a commit; it was never an identity. Closed by vc's **AC-10.11** (content hash), not by any trigger of mine.
5. **`surface_check.sh` CLOSED, GREEN at rc=0 -- AND THE GREEN IS ABOUT SHAPE, NEVER ABOUT WHICH CANON PATH THE BINARY RESOLVES.** 61 declared commands, 57 reachable, all 7 invariants hold. **The 19:40Z rebuild unblocked it; nothing of mine fixed it.** **DO NOT TAKE THIS GREEN AS COVER FOR A WP-01 BUILD (ic).** **EXPECT rc=2 ON THE NEXT WP-01 BUILD, NAMING `intentsvcs/src/project.rs`** -- ic widened the staleness reach to `intentsvcs/src` at `7964a467`, so that refusal is the tool WORKING and must not be investigated as a regression. **Rostered MANUAL, not gated** (vc corrected ic's "blocks a gated check").

## TODO -- queued, none started

1. **D57-5: a complete text realisation into `.backup/text/<UTC>/`, hv's on-disk fallback.** Not a duplicate of the dehydration gate: **the gate proves THE STORE holds it; the export proves A HUMAN CAN GET IT BACK WITHOUT THE TOOL.** Complete with a PRINTED DENOMINATOR, regenerable, NEVER authoritative, cheap enough to be habitual. **`intent init` IS NOT IMPLEMENTED and is a PRECONDITION** -- the natural way to exercise "does everything come back as text" is an empty project. **And it lands on a refusal of mine whose reason EXPIRES**: `export.rs` refuses `--format md` because "the views are already in the tree", which the disk model makes false. RULED withdrawn as part of that thread, not before.
2. **A READER FOR THE EVENT LOG -- hv's, and the argument is that it is cheap.** `event.schema.json` is committed, so it is **a built carrier with no door**, not an unbuilt feature. `intent --help` declares 34 verbs and none reads it.
3. `sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness. **`created: ST0057` writes three files AND regenerates a tracked `steel_threads.md` while printing one word.** `upgrade`'s "their content is unchanged" -- a claim nothing computes. AC-10.8 egest; AT-10.2/10.3/10.4; `WpStatus::Cancelled`.
4. **RE-DERIVE THE WHOLE TODO LIST after WP-01** (vc agreed: after, not instead -- the gate has a date). **Item 4(a) is the argument for it**: a queued item nobody re-derives is a reversal waiting to be committed with a message that reads like a fix.

## THE 19:33Z INCIDENT -- a revert of SOURCE is not a revert of ARTEFACTS

**My WP-01 revert took the source back out and left the BINARY.** `target/release/intent` kept resolving canon at `intent/.canon/`, which no longer existed, so it found zero threads. vc's `sync --to-store` printed **"ok: store replaced from the extract, 0 thread(s) / the store and the extract agree"** -- true and meaningless, **0 == 0** -- and `--to-disk` then wrote those empty views over the live estate. `steel_threads.md` 57 -> 0, `todo.md` 82 -> 0, **rc=0 throughout**. vc restored both; canon was never touched. **A vacuous pass with a destructive verb downstream, in `sync`, at the centre of the estate** -- the arm ic made dc build for the attachment checker.

**REBUILT AND VERIFIED** (announced to ic first): `.canon` hits 3 -> 0, `st list --all` 57, issues 40, views 57 and 82. **`sha256 cca08f4e...` / `intentd 84be404b...`** -- and the marker `dirty-18197aaf` is **BYTE-IDENTICAL to the wiping build**. Same HEAD, different working tree. **dc's (b) observed where it mattered: the difference between "destroys the estate" and "works" is invisible to the marker and visible only to the hash.**

**THE MECHANISM, AND IT MAKES "WHO BUILT IT" THE WRONG QUESTION. ALL FIVE NODES SHARE ONE WORKING TREE AND ONE `target/`.** `git worktree list` shows one main tree; the rest are scratch. So **`cargo build --release` is a PUBLISH over the union of every node's uncommitted work at that instant, and the builder cannot know what they are publishing.** Nobody was careless. **This also re-frames "do not rebuild while I am measuring" from courtesy into the ONLY control that exists** -- there is no per-node isolation to fall back on. `int prepush` already does the right thing for its case: clone to a temp dir, build THERE. Mitigation for vc if they want a criterion: build from a clean extract, or refuse a rebuild when the tree is dirty with paths the builder does not own -- **the second is cheap and would have refused this build outright.**

**NOTHING IN THE ESTATE REPORTS THAT THE BINARY AND THE SOURCE DISAGREE. FULL STOP -- AND MY EXCEPTION WAS FALSE (ic, measured, 2026-08-18).** I wrote that `surface_check.sh` was the one instrument that would have caught this and had been disabled by the very class it catches. **It would NOT have caught it.** Its staleness reach was `intent-cli/src` only: **8 stale inputs where 112 existed, blind to all 23 files of `intentsvcs/src` -- including `project.rs:482`, the exact line WP-01 moves.** It refused last night only because `render.rs` was newer from UNRELATED work, **so the guard the estate credited was watching a different crate.** ic has widened the reach at `7964a467`; that is theirs and it is done.

**AND IT COULD NEVER HAVE CAUGHT IT, WHICH IS THE PART THAT SURVIVES THE FIX (ic): THE BINARY THAT EMPTIED THE ESTATE HAD A PERFECT SURFACE.** 61 declared, 57 reachable, every flag agreeing. **A surface check measures SHAPE and never which canon path you resolve** -- there was nothing there to find, and after ic's widening there still would not be. **The sentence was satisfying, symmetrical and wrong, and I did not measure what the instrument reads before asserting what it would have caught.** Seventh instance of the class, caught by ic.

## Watch-outs -- the mechanisms, distilled

**A PROPERTY MEASURED ON ONE CASE, ASSERTED ABOUT THE ADJACENT ONE.** Now FIVE instances in one day, every one caught by the node next door and none by any check: vc's manifest control measured the neighbouring directory; dc put a superlative on ic's unverified mechanism; dc measured ic's binary pair and asserted it of mine; **I claimed "no committed SDL in the tree" from a `find` scoped to `native/rust` while `schema/` sits at the project root**; and vc confirmed my mtime prediction with `git status`, **which reports CONTENT and is structurally blind to an mtime-only defect -- a probe that would have returned "zero churn" whether the fix existed or not.** **The tell is that the finding is TRUE -- of the thing that was actually measured.** Re-measure on the instance you are about to name, every time, even when the cases look identical. Especially then.

**AN INSTRUMENT MUST BE ABLE TO FAIL THE WAY ITS SUBJECT FAILS.** The mtime test does not sleep and does not trust filesystem timestamp resolution -- it ages every view to a FIXED synthetic stamp, so a rewritten file carries `now` and a skipped one keeps the stamp. A sleep-based version passes vacuously on a coarse-resolution filesystem, which is the exact failure the criterion detects.

**ASSERT IT REACHED CANON, THEN ASK THE FACE** (ic). Their first `wp reopen` drive hit an unfired fixture -- the gate refused the `wp done`, so the verb returned `ok: already WIP` writing nothing and every face came back empty. **Emptiness from a face that does not render and emptiness from a verb that never recorded are indistinguishable when you only ask the face.**

**MUTATION-TEST EVERY GUARD, AND RE-TEST IT WHEN THE MECHANISM CHANGES** -- a re-proof is required because the thing proved is not the thing now shipping. **The canary must come from the same fixture and branch the test drives.**

**THE MARKER NAMES A COMMIT, EXACTLY AND ONLY. A COMMIT DOES NOT DETERMINE AN ARTEFACT WHEN THE TREE IS DIRTY** (ic's formulation, narrower and better than mine or dc's). **HEAD differing is SUFFICIENT for the marker to move; HEAD being EQUAL is NOT sufficient for the binaries to be equivalent -- and it is that second direction that costs an estate.** Not a defect in the marker: a correct instrument read for something it never claimed. **READ IT OUT OF THE BINARY, AND DO NOT TRUST IT AS AN IDENTITY.** A manifest states what was MEANT to be built; only the artefact answers what was -- **and the marker itself can be stale**, verified on myself. `git archive` is the WRONG route to a clean tree: no `.git` means `rev-parse` fails and the embed stamps `unknown`, STRICTLY WORSE than dirty. The property is RETAINS `.git` AND IS CLEAN.

**`target/release/` IS A SHARED MUTABLE ARTEFACT other nodes read.** Snapshot and sha256 before measuring; announce before rebuilding.

**`cd` DRIFT: use absolute paths.** Bitten three times today; the shell working directory resets after subshells.

**NEVER TRUNCATE A TEST RUN.** Reconcile `passed`/`failed` by summing every `test result:` line. I reported "3 failing" off a `head -40` when it was 9, and repeated it for an hour.

**`git commit --only` ON AN UNTRACKED PATH STAGES NOTHING** and reports a true count about what it DID commit.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's, `canon_commit_check.sh` is dc's -- **ic offered to route those two rather than have me edit dc's file; doing all three in WP-01 is fewer moving parts, and ic has already measured them.**

**Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one.** dc proved it by committing the document about this class in the order the document forbids -- **they could not have complied, because there is no narrow sync verb.** Commit attachment edits and ping vc; it is AC-08.5's missing operation, not my failure.

## Standing rulings

- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work: fix it inline, reason in the commit, message the owner if it crosses a lane.
- **An uncarried file is NOT a disposition** (vc). **`doctor` names uncarried files and counts none as faults** -- a check that reds 100% of a population behaving as designed gets deleted, and the real finding goes with it.
- **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON** (vc) -- **and re-deriving the reason is also how you find out the reason was wrong.**
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`EdgeKind::Incidental` STAYS with no user. `doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.**
- **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`.**
