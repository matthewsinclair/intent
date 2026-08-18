---
node: cc
name: Control Claude
role: control
session_id: 58ada566-7779-4209-a426-8622a8b8e323
heartbeat_at: 2026-08-18 19:35Z
status: paused
focus: "FOLDED AND PAUSED on hv's instruction for a compact. **Landed today: the todo view's six-states-as-one glyph defect, AC-04.4's mtime guard (a no-op sync was rewriting all 266 views), `status_reason` rendered on four faces, and item 4(a) WITHDRAWN after measuring found my own recorded reason was half false.** Suite 634/0 at my last full run. **ST0057 WP-01 (canon relocation to `intent/.canon/`) IS STARTED AND DELIBERATELY REVERTED -- patch saved, tree left GREEN for peers.** Nothing of mine outstanding. Upstream FROZEN."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, **and the ingest corpus excludes ignored paths.**

**hv's DISK MODEL: no status directories, disk becomes OPTIONAL, index plus render-on-demand.** ST0057 is IN the 3.0.0 gate (hv). `realisation.md` 5.1 is the dehydration gate; 5.1b is the attachment asymmetry.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory. **`one_clock.rs` enforces it structurally and it caught ME today**, in a test about measurement discipline.

## NEXT: ST0057 WP-01 -- STARTED, REVERTED, RESUMABLE IN MINUTES

**The code worked and the tree is green because I took it back out.** vc cleared me to move the live estate; hv called a compact first. Leaving 68 fixture-caused failures under four peers who are actively measuring is a cost nobody should pay for my convenience -- **ic asked explicitly not to be measured inside someone else's window.**

**PATCH: `<session-scratchpad>/wp01-canon-relocation.patch`, 159 lines, `export.rs` + `project.rs`.** If it is gone, it is 30 minutes of re-typing and every decision is below.

**WHAT WAS BUILT:** `canon_dir()` = `intent/.canon/`; `canon_st_dir()`; `thread_json` -> `.canon/st/<ID>.json`; `issue_json` -> `.canon/issues/<nnnn>.json`; `issues_dir()` repointed (**the whole directory moves -- it held nothing but `<nnnn>.json`; an issue has no realised markdown to leave behind**); `thread_ids()` reads `.canon/st/` by file stem instead of walking `st/` for a nested file; **`classify`'s `thread.json` arm KEPT with a note saying why it is not dead** -- a v2 tree has one, and so does a tree caught mid-move, and it is what keeps a stale canon file out of the attachment carry.

**THE HIGHLANDER FIX THAT FELL OUT, AND ITS OWN COMMENT PREDICTED IT:** the exporter spelled the canon path independently while every reader resolved `Project::thread_json`. Its neighbouring comment records the issue arm having ALREADY shipped that bug -- `issues/46.json` written where readers opened `issues/0046.json`, "two ends had to agree by convention and did not". **Relocating would have re-created it at the thread arm.** Both now call one `canon_thread_rel` / `canon_issue_rel`. Comment tense corrected too: it described the fixed bug in the present, which is history reading as state.

**WHAT IS LEFT, in order:**

1. **Repoint ~15 test fixtures that spell `intent/st/<ID>/thread.json` independently** -- the same Highlander problem one level down. `canon_thread_rel` is `pub`, so tests can resolve through it and never spell it again. Failing binaries measured: `acceptance_surface`, `cli_end_to_end`, `declared_values_are_enforced`, `issues_surface`, `literal_stdout_parity`, `search_surface`, `self_loop_voice`, `upgrade_command`, `export_round_trip`, `facade_acceptance`, plus lib unittests. **Every failure is "no steel thread ST0001 in this project" -- the fixture writes canon where the tool no longer looks. Not a design problem.**
2. **ic MEASURED the parity-harness exposure: 3 targeted edits, NOT a path migration.** `realise_plan.sh:44` (ic's, breaks), `canon_commit_check.sh:82,93,198,199,203` (dc's, breaks), `gen_register.sh:256` (**a GENERATED doc cell -- fails in the QUIET direction: nothing goes red and the register just becomes wrong about where canon lives**). **HAZARD ic measured so I would not hit it: a grep for `intent/st/` returns 17 of 41 tools, and most MUST NOT be touched** -- they are comments correctly describing the v2 layout, and a mechanical `s|intent/st/|intent/.canon/|` would rewrite true statements about v2 into false ones, silently, in the comments that explain why the migration works. Another 17 are the harness's own directory, which does not move.
3. **The live move: 57 thread + 40 issue canon files. vc CLEARED it (`25605e6b`) and will not touch ST canon until I say it landed. Ping vc after.**
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

## OPEN -- what is actually mine

1. **WP-01, above.**
2. **The cold-warm collection gap `doctor` has no arm for -- PARKED BEHIND WP-01, on vc's ruling and my reasoning.** The arm reports "a file on disk that canon does not know about", and **that sentence CHANGES MEANING under sparseness**: an absent file stops being an anomaly and a present-but-unknown one becomes the interesting case. Building it now would encode today's dense-disk assumption into the one check whose job is to police the sparse one. **I have already eaten that exact error once, with the replacing write-back.**
3. **`critic rust` and `critic shell` arm ZERO rules** (0 of 6, 0 of 7). dc's Half A relit the gate; a green from either still means "nothing asked a question", and I move the most `.rs`. Half B is dc's.
4. **THE BINARY MARKER -- (a) WITHDRAWN, (b) NOT MINE TO CLOSE.** **(a) STALENESS: withdrawn, and the withdrawal is the finding.** vc refused the reversal and asked what expired the reason. Nothing had -- **and measuring it found HALF MY OWN RECORDED REASONING WAS FALSE.** The comment claimed "HEAD moves when ANYONE commits ANYTHING"; **`.git/HEAD` is rewritten on a BRANCH SWITCH, not a commit** -- measured, six months stale against `refs/heads/main` moving seconds earlier. **The refusal stands on its other limb, sufficient alone: emitting ANY `rerun-if-changed` REPLACES cargo's package-file default**, so naming `.git/HEAD` would leave the embed stale on CODE changes, permanently and silently. **The naive fix is strictly worse than the gap, and the wrong reason was the one that made it look obviously correct.** Corrected in place at `a466f90f`. **(b) NON-IDENTITY (dc's, the one that actually bit):** `dirty-<HEAD>` gives two behaviourally different dirty builds one value. It names a commit; it was never an identity. Closed by vc's **AC-10.11** (content hash), not by any trigger of mine.
5. **`surface_check.sh` refuses at rc=2** because the release binary is older than `surface/dispatch-table.json` and my `render.rs`. **Rostered MANUAL, not gated** (vc corrected ic's "blocks a gated check"). Mine to take in my next build window -- **and ANNOUNCE the rebuild first**, because `target/release/` is shared and a rebuild under a peer mid-measurement is what invalidated ic's run today.

## TODO -- queued, none started

1. **D57-5: a complete text realisation into `.backup/text/<UTC>/`, hv's on-disk fallback.** Not a duplicate of the dehydration gate: **the gate proves THE STORE holds it; the export proves A HUMAN CAN GET IT BACK WITHOUT THE TOOL.** Complete with a PRINTED DENOMINATOR, regenerable, NEVER authoritative, cheap enough to be habitual. **`intent init` IS NOT IMPLEMENTED and is a PRECONDITION** -- the natural way to exercise "does everything come back as text" is an empty project. **And it lands on a refusal of mine whose reason EXPIRES**: `export.rs` refuses `--format md` because "the views are already in the tree", which the disk model makes false. RULED withdrawn as part of that thread, not before.
2. **A READER FOR THE EVENT LOG -- hv's, and the argument is that it is cheap.** `event.schema.json` is committed, so it is **a built carrier with no door**, not an unbuilt feature. `intent --help` declares 34 verbs and none reads it.
3. `sync`'s "Safe: the files are re-creatable" is a claim about RECOVERABILITY, not correctness. **`created: ST0057` writes three files AND regenerates a tracked `steel_threads.md` while printing one word.** `upgrade`'s "their content is unchanged" -- a claim nothing computes. AC-10.8 egest; AT-10.2/10.3/10.4; `WpStatus::Cancelled`.
4. **RE-DERIVE THE WHOLE TODO LIST after WP-01** (vc agreed: after, not instead -- the gate has a date). **Item 4(a) is the argument for it**: a queued item nobody re-derives is a reversal waiting to be committed with a message that reads like a fix.

## THE 19:33Z INCIDENT -- a revert of SOURCE is not a revert of ARTEFACTS

**My WP-01 revert took the source back out and left the BINARY.** `target/release/intent` kept resolving canon at `intent/.canon/`, which no longer existed, so it found zero threads. vc's `sync --to-store` printed **"ok: store replaced from the extract, 0 thread(s) / the store and the extract agree"** -- true and meaningless, **0 == 0** -- and `--to-disk` then wrote those empty views over the live estate. `steel_threads.md` 57 -> 0, `todo.md` 82 -> 0, **rc=0 throughout**. vc restored both; canon was never touched. **A vacuous pass with a destructive verb downstream, in `sync`, at the centre of the estate** -- the arm ic made dc build for the attachment checker.

**REBUILT AND VERIFIED** (announced to ic first): `.canon` hits 3 -> 0, `st list --all` 57, issues 40, views 57 and 82. **`sha256 cca08f4e...` / `intentd 84be404b...`** -- and the marker `dirty-18197aaf` is **BYTE-IDENTICAL to the wiping build**. Same HEAD, different working tree. **dc's (b) observed where it mattered: the difference between "destroys the estate" and "works" is invisible to the marker and visible only to the hash.**

**THE MECHANISM, AND IT MAKES "WHO BUILT IT" THE WRONG QUESTION. ALL FIVE NODES SHARE ONE WORKING TREE AND ONE `target/`.** `git worktree list` shows one main tree; the rest are scratch. So **`cargo build --release` is a PUBLISH over the union of every node's uncommitted work at that instant, and the builder cannot know what they are publishing.** Nobody was careless. **This also re-frames "do not rebuild while I am measuring" from courtesy into the ONLY control that exists** -- there is no per-node isolation to fall back on. `int prepush` already does the right thing for its case: clone to a temp dir, build THERE. Mitigation for vc if they want a criterion: build from a clean extract, or refuse a rebuild when the tree is dirty with paths the builder does not own -- **the second is cheap and would have refused this build outright.**

**NOTHING IN THE ESTATE REPORTS THAT THE BINARY AND THE SOURCE DISAGREE** -- except `surface_check.sh`, which I had already flagged as unable to run, for the same underlying reason. **The check that catches this class was disabled by the class.**

## Watch-outs -- the mechanisms, distilled

**A PROPERTY MEASURED ON ONE CASE, ASSERTED ABOUT THE ADJACENT ONE.** Now FIVE instances in one day, every one caught by the node next door and none by any check: vc's manifest control measured the neighbouring directory; dc put a superlative on ic's unverified mechanism; dc measured ic's binary pair and asserted it of mine; **I claimed "no committed SDL in the tree" from a `find` scoped to `native/rust` while `schema/` sits at the project root**; and vc confirmed my mtime prediction with `git status`, **which reports CONTENT and is structurally blind to an mtime-only defect -- a probe that would have returned "zero churn" whether the fix existed or not.** **The tell is that the finding is TRUE -- of the thing that was actually measured.** Re-measure on the instance you are about to name, every time, even when the cases look identical. Especially then.

**AN INSTRUMENT MUST BE ABLE TO FAIL THE WAY ITS SUBJECT FAILS.** AC-04.4's test does not sleep and does not trust filesystem timestamp resolution -- it ages every view to a FIXED synthetic stamp, so a rewritten file carries `now` and a skipped one keeps the stamp. A sleep-based version passes vacuously on a coarse-resolution filesystem, which is the exact failure the criterion detects.

**ASSERT IT REACHED CANON, THEN ASK THE FACE** (ic). Their first `wp reopen` drive hit an unfired fixture -- the gate refused the `wp done`, so the verb returned `ok: already WIP` writing nothing and every face came back empty. **Emptiness from a face that does not render and emptiness from a verb that never recorded are indistinguishable when you only ask the face.**

**MUTATION-TEST EVERY GUARD, AND RE-TEST IT WHEN THE MECHANISM CHANGES** -- a re-proof is required because the thing proved is not the thing now shipping. **The canary must come from the same fixture and branch the test drives.**

**READ THE MARKER OUT OF THE BINARY, AND DO NOT TRUST IT AS AN IDENTITY.** A manifest states what was MEANT to be built; only the artefact answers what was -- **and the marker itself can be stale**, verified on myself. `git archive` is the WRONG route to a clean tree: no `.git` means `rev-parse` fails and the embed stamps `unknown`, STRICTLY WORSE than dirty. The property is RETAINS `.git` AND IS CLEAN.

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
