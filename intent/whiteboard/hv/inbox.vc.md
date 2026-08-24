# inbox: vc -> hv

## (2026-08-21 10:28Z)

**Four for your ruling. All surfaced live as well as written here -- the write is the durable half, not the delivery.**

**1. THE GATE HAS NO COMPOSING VERB, AND THAT IS WHY IT DRIFTED.** Corrected to 62 of 67 at `14298e6b`. `ac status` computes each half; **nothing composes them**, so every fold hand-adds 47+15 and the last one drifted inside 24 hours. The machinery already exists: ST0057 AC-00.1 carries `<<PRECONDITIONS AC-00.2 ... AC-07.6 PRECONDITIONS>>` and the ship gate reads that list. **The 3.0.0 release gate is the same pattern one level up and currently has none.** Not mine to build -- scope call and owner are yours.

**2. THE hv BOARD IS THE MIRROR OF THE FOUR-DAY FAILURE, AND NOBODY HAS NAMED IT.** The README fixed peer -> hv by naming vc the obliged reader. **hv -> peer has the identical hole.** `hv/wip.md` reads `heartbeat_at: 2026-06-25T12:01Z`, `focus: "(new workstream -- not yet started)"`, **no standing directives** -- eight weeks stale, on the one board every peer is told to read for directives at pickup. The real directives (hold the three gated items, the upstream freeze, do not put v3 on PATH) live scattered across peers' boards and quoted speech in `README.md`. **The escalation surface got a named reader; the directive surface still has no named writer.** Also: your stamp uses a `T` separator where the protocol specifies a space -- the one board that would not round-trip the format its own guard enforces. Yours to edit, not mine.

**3. ~71G OF BUILD ARTEFACTS, AND CORRECT COMPLIANCE IS WHAT PRODUCED IT.** `native/rust/target` is **66G across 1,336,417 files** against 1,481 tracked files -- `debug` 33G, `cc` 18G, `ic` 15G -- plus **4.7G stranded in two orphaned worktrees** from yesterday's dead sessions (mine 4.3G clean, dc's 399M holding only an untracked `tgt/`). Our own rule says _isolate the target dir, keep it inside the checkout_, written after a drifted build put 1.2G where gitignore hid it. **cc and ic are following it exactly, and that is what produced 33G of duplication.** The rule says how to create these and is silent on disposal. **A gitignored artefact is invisible to every instrument we use, so nothing here was ever going to report it.** My removal of my own orphaned worktree was refused by the session's permission classifier -- correctly, it is a 4.3G delete -- so it is yours to run.

**4. THE FIVE GATE ROWS ARE CITATION-CLEAN.** Swept before cc and ic spend context on them. All four cited files exist and carry thread-qualified ids; `AT-07.7` cites nothing under `11762943`'s ruling, which is the declared handoff and not an omission. **No builder will hit a stale citation on the gate.** One stale row OUTSIDE the gate, surfaced free by the pre-commit gate: **`AT-00.6` is `to-write` while `migrate_v2_project.rs` exists.** cc's, filed not fixed.

## (2026-08-22 10:11Z)

**ONE RULING OWED, AND IT BLOCKS A WHOLE COMMAND: may the shipped v3 surface read `$HOME`?**

AC-11.3 permits the shipped surface exactly ONE environment variable, `COLUMNS`, enforced structurally over every `src/**/*.rs` by `no_intent_home::the_shipped_surface_reads_exactly_one_environment_variable`. **`intent claude skills` needs `$HOME` twice** -- the manifest at `~/.intent/skills/installed-skills.json` and the target at `~/.claude/skills/`. Without a ruling the command cannot exist.

**cc drove the invariant in BOTH directions rather than reading it**: green at HEAD (surface reads exactly `{COLUMNS}`), and a planted `std::env::var("HOME")` was refused BY NAME with a message saying it needs an hv ruling and a row in `ALLOWED`, not a quiet addition. Probe removed, tree clean. cc is shipping the module with every ambient path as a PARAMETER and leaving the CLI arm unwired, so `intent claude skills` answers `2` and says why -- rather than smuggling a second variable in behind a feature. That posture is right and I have ratified it.

**MY RECOMMENDATION: GRANT IT, BUT ONLY AFTER THE MANIFEST PATH SPLITS -- and the sequencing is the whole recommendation, not a caveat on it.**

cc's argument for granting is that `$HOME` is unlike `$INTENT_HOME` because there is no versioned v2/v3 asset skew -- it is the user's own home. **That is true only once the paths separate, and today it is false.** I verified against the live file: v2 writes `$HOME/.intent/skills/installed-skills.json`, and cc's proposed v3 manifest was the **same path, same `version: "1.0.0"`, same four fields** (`checksum`, `installed_at`, `name`, `source_path`) -- identical to the 26-entry file on this machine. After my AC-07.3 ruling, `checksum` denotes a different function. **v2 is shipped and cannot be taught to read a scope field**, so each binary would read the other's manifest, find a mismatch it must treat as an update, and overwrite -- **forever, with both printing `updated` and neither having anything to report.**

**And the condition that makes it live is the local cutover I drove this morning: both binaries on PATH together.** `intent` at 19, `intent3` alongside. That is not a future risk; it is the state every project on this machine is in right now.

I have ruled the repair (v3 owns a separate manifest path, never reads or writes v2's). **With that in place `$HOME` really is just the user's own home and I would argue for granting it. Before it, the shared mutable per-user store IS the skew AC-11.3 exists to prevent, and I would not ask you to wave it through.**

**Nothing is blocked on you this minute** -- cc is building the module, which is pure and fully driven against tempdirs. Only the CLI edge waits.

**Also still open and separately yours: PUSH.** `upstream/main` at `ee4a7cac`, local **23 ahead**. Your "commit away" answered a question that bundled commit and push; only the commit half was ever settled.

## (2026-08-22 10:21Z)

**A SECOND RULING NOW OWED ON THE SAME COMMAND, AND I AM RAISING IT BECAUSE ic PARKED A ROW ON A RULING NOBODY HAD ASKED YOU FOR.**

I ruled AC-07.3 myself as contract steward: v2's `claude skills sync` checksums SKILL.md only, so a change confined to a skill's `scripts/` never propagates, and I ruled that a DEFECT rather than a behaviour -- v3 fixes it. ic implemented the table edit but marked it **PROVISIONAL pending hv**, which is the more careful call and I have ratified it rather than overridden it.

**But a row marked "pending hv" against a question hv was never asked is a write surface with no reader -- the exact failure the whiteboard protocol names.** So here is the question, plainly:

**Does v3 carry v2's SKILL.md-only checksum scope, or fix it?**

**My ruling, for you to confirm or reverse:** FIX IT. The criterion says _reproduce v2 SHA256-manifest behaviour_, which names a MECHANISM (a SHA256 manifest, versus timestamps or version fields) and no scope. **A rewrite that reproduces the defects of the thing it rewrites is not a rewrite.** The blind spot has no beneficiary -- no workflow depends on a change failing to propagate. It was surfaced 2026-05-21 during the v2.11.8 gate-deadlock fix, on a script-only edit to **`release-gate.sh`, the in-session gate's own releaser -- the highest-blast-radius script in the estate** -- with the remedy direction recorded at the time. And it is on the v2 backlog to fix, which makes carrying it forward a preservation this project forbids.

**The precedent cuts both ways and you should see it: AC-06.9 (`doctor --fix`) was the same SHAPE -- whether v3 carries a v2 behaviour -- and you ruled that one, not me.** That is why ic's caution is right and why I am not treating my own ruling as final.

**Both questions are about the same command, so rule them together if it is easier:** this one, and the `$HOME`/AC-11.3 question in my previous entry. Nothing is blocked on either this minute -- cc's module is built and drivable against tempdirs; only the CLI edge waits.

## (2026-08-24 11:13Z) FYI only -- no response needed.

**DO NOT RE-DERIVE ANY OF THIS. It is hv's instruction that you take it from this entry and start fresh.**

**WHAT HAPPENED: a five-estate Claude Code config sweep**, coordinated by `lamplight/vc` across Intent, Lamplight, Laksa, Baize and Conflab, plus `devbin/vc`. Intent's role was UPSTREAM -- most findings were template- or tool-borne, so they were one fix here rather than five downstream patches. Intent's row on that program ledger is CLOSED.

**LANDED IN INTENT (main, pushed, `rust` CI green):**

| sha                   | what                                                                             |
| --------------------- | -------------------------------------------------------------------------------- |
| `55fc4a50`            | hook-script prune + downgrade guard + `--dry-run` + Stop routing + autopsy D1-D5 |
| `c3b95502`            | Intent's own `.claude/` layer: 3 inert scripts and a shadow agent removed        |
| `3d838eca`            | 14 fence tests, the honest dry-run preview, whole-directory skill checksum       |
| `cc7352de`+`b046a5c9` | the v2/v3 shipped-surface drift guard                                            |
| `dc04df56`            | **MODULES.md stops being seeded in v3**                                          |
| `ebb94e92`            | Intent's ten verbatim per-language canon files deleted                           |
| `5eb2a857`            | the agnostic RULES/ARCHITECTURE pair restored to v3, REWRITTEN                   |
| `243d126c`+`607306dd` | the elixir template stops asserting project facts it cannot know                 |
| `628b74ad`+`eb4fe67c` | the two CI failures                                                              |

**THE ONE HAZARD BEHIND MOST OF IT:** the fleet runs the FROZEN `Intentv2` via `$INTENT_HOME`, so **a fix landed in one tree reaches nobody and presents as done.** Four instances in a day: the Claude Code hook door, the commit guards, the `upgrade` verb, and v3 having silently DROPPED the agnostic templates. **Land shipped-surface changes in BOTH checkouts.** `tests/unit/shipped_surface_drift.bats` now reddens if you forget -- its first catch was me.

**OPEN, WITH NUMBERS:** `intent#0065` doctor acknowledgement, `intent#0066` `st` does not resolve `_inbox/`, `intent#0067` `modules find` v3 parity gap, `intent#0068` do NOT rebuild the per-language doc fan-out (HIGH).

**NEW CONVENTION, in `usage-rules.md` (`7eb0efe6`):** cross-estate references are qualified -- `devbin#0017`, `lamplight/ic`. Unqualified is legal same-repo only.

**STATUS AT THIS STAMP.**

**Intent and Intentv2 are both clean and pushed, all four refs level.** `rust` CI is GREEN on the fix push; `Intent Tests` was still running when this was written -- **check it rather than assume it.**

**Both CI failures are understood and different in kind.** The `rust` one was a genuine Linux flake, confirmed by re-running the identical commit unchanged and watching it pass; fixed anyway, because a test that reddens intermittently is a defect. The `Intent Tests` one was MINE and the test was right to fail -- it encoded the contract `55fc4a50` deliberately changed, and its replacement asserts more than the original did.

**DECISIONS YOU TOOK TODAY THAT ARE NOW STRUCTURAL, recorded so they are not re-litigated:** prune the dross now rather than gating on WP-13; v3 only, Intentv2 frozen except for content that ships a measured falsehood; the agnostic pair is restored while the ten per-language files are not; and the elixir template stops asserting project facts.

**ONE JUDGEMENT I MADE THAT WAS NOT IN THE INSTRUCTION,** flagged because it should be yours to overturn: restoring the agnostic templates literally would have re-seeded v2's bodies, and those ARE the spec-shaped scaffolding this review was called to remove -- four `<!-- Replace with ... -->` prompts, a placeholder tree, and a paragraph advertising the fan-out `0068` retires. I rewrote them instead. Both now say what belongs in the file and that they ship EMPTY ON PURPOSE.

## (2026-08-24 12:07Z) FYI only -- no response needed.

**GLOBALFOLD DONE, AND IT FOUND SOMETHING WORTH YOUR ATTENTION MORE THAN THE NUMBER DID.**

**THE GATE IS 66 OF 67 AND `restart.md` SAID 62 UNTIL TWENTY MINUTES AGO. I have just corrected it, committed `50f74cfd`, pushed both remotes -- so `git pull` before you trust anything you read at pickup.** Driven at `50417c83`, 0 dirty, all three calls: `ac status ST0057` 50/51 (2 withdrawn), `ac status ST0056/03` 16/16 PASS, `ac gate ST0057` -> `AC-08.5`.

**Controlled rather than assumed:** denominators (51, 16) and withdrawn counts (2, 1) both held, so this is four rows GREENING and not a scope shrinking -- a rising fraction over a shrinking denominator is the cheap way to fake one. Cross-checked across `intent3` and the debug build: identical. **That certifies the READ PATH is not divergent between builds and certifies NOTHING about whether the store agrees with canon** -- two readings of one store are one reading counted twice.

**THE FINDING IS NOT THE ARITHMETIC. The number had THREE HOMES CARRYING THREE VALUES** -- `intent/restart.md` 62, `.claude/restart.md` 62 and untouched since 08-21, `intent/wip.md` 65 -- **and `wip.md` held it twice, disagreeing with ITSELF inside one document.** Highlander applies to a figure in prose exactly as it applies to code. **Do not transcribe it again; run the three calls.**

**Also corrected: "DO NOT PUT v3 ON PATH" was retired 2026-08-22 by ST0058 and both restart files asserted it for two more days.** v3 IS on PATH as `intent3` -- a distinct name, so the fleet's gate is untouched by construction. `intent3` -> `bin/intent3` -> `target/release/intent`, **which the gate reports as built from an UNCOMMITTED tree.** Pin by hash, never by the marker.

**AND THE HAZARD THAT OUTRANKS EVERYTHING IN MY SWEEP: the fleet resolves `intent` through `$INTENT_HOME` to the FROZEN `~/Devel/prj/Intentv2`, so a shipped-surface fix landed in ONE tree reaches nobody and presents as done.** Four instances in one day. `tests/unit/shipped_surface_drift.bats` reddens on it now and **its first catch was its own author.** hv's ruling: **Intentv2 is FROZEN; fixes are v3-only unless the shipped surface demands both.**

**THE GOVERNANCE POINT: nothing in this repo compares the three copies of that figure, and nothing can.** `at lint` checks rows against files, `doctor` checks views against canon, the clock guard checks stamps -- **a number transcribed into prose in three documents is checked by a reader noticing.** It went wrong in all three homes independently, in different directions, and the two that agreed (62 and 62) agreed only because one was a copy of the other. **Agreement between a document and its own copy is not corroboration.**

**I have not proposed a mechanism, deliberately.** The cheap one -- a guard grepping for `\d+ of 67` -- hard-codes the denominator it is meant to protect, and would have passed every day this week. **The honest fix is that the figure should not be in prose at all**, and that is a call about how these documents are written rather than a check to add. Routed to you rather than decided by me.

**STATE:** one row left in the gate (ST0057 `AC-08.5`, cc builds / ic covers), three peers up and picking up, both trees clean, all refs level, both CI workflows green. Four sweep issues open with numbers: `0065`, `0066`, `0067`, `0068` (HIGH).

## (2026-08-24 12:16Z)

**TWO RULINGS NEEDED AND ONE ADMISSION. Neither ruling is mine or dc's to take, and the admission is that I was one command from taking the first one badly.**

**RULING 1 -- WHAT SHOULD `shipped_surface_drift.bats` ASSERT NOW THAT YOU HAVE FROZEN Intentv2? This is the question, and the cheap fix would bury it.**

The guard I built yesterday compares the shipped surface across the two checkouts and fails on divergence. **It cannot fire in CI**: it resolves the v2 tree from a filesystem path, `.github/workflows/` contains zero references to `Intentv2`, `INTENT_V2_CHECKOUT` or `v2-maintenance` (driven, positive-controlled by dc -- the same grep finds `cargo` and `Intent`), so **all THREE of its tests skip, INCLUDING ITS OWN POSITIVE CONTROL.** CI cannot even establish the comparison had anything to look at. A skip renders as `ok N # skip` inside a green suite.

**THE FIX IS CHEAP AND THAT IS THE TRAP.** `v2-maintenance` is a branch of THIS repository -- `upstream/v2-maintenance == local/v2-maintenance == e5a8f158`, and the Intentv2 working tree sits exactly there, 0 dirty -- so comparing against the REF instead of a checkout works in CI, needs no second repository, and removes the dependency on someone's working copy being clean. **I had resolved the refs and was writing that change when dc's message arrived.**

**dc STOPPED IT ON THE RIGHT GROUND: THE PROPERTY IS UNSETTLED, AND A GUARD WIRED BEFORE ITS ASSERTION IS SETTLED BUYS A GREEN ABOUT THE WRONG THING.** The file ALREADY knows convergence would be a defect -- `_is_v3_only_by_ruling` exists precisely because you ruled the agnostic pair v3-only -- so the property is not _these two trees agree_. It is something closer to _a shipped-surface change is either in both or declared_, **and I do not know whether that is mechanically checkable or a judgement wearing a test's clothes.**

**WHAT ACTUALLY NEEDS YOUR WORD IS THE SCOPE OF THE FREEZE, because the assertion follows from it and nothing else does.** Your ruling on the MODULES.md round was _if we fix, and what gets fixed, is all to be Intentv3 only; Intentv2 is frozen_. **But on the same day I landed the `intent claude upgrade` downgrade guard, the hook door and `session-finish.sh` in BOTH trees, and that was right** -- the fleet EXECUTES v2, so a defect in what consumers run has nowhere else to go. So the operative rule looks like **frozen for features, live for defects in the shipped surface** -- which nobody has stated, which I have been acting on all day, and which the guard would have to encode.

**RULING 2 -- dc's, routed as they asked: may `intent3` REFUSE an incoherent binary?** Their framing is the right one: a reporter fails open, an actor refuses, and `intent3` is now an ACTOR on PATH. Live instance from this morning's gate: both release binaries report `dirty-69f672d3` and their bytes match no commit; cc measured `intent3` as **73 commits behind HEAD, 8 of them under `native/rust/crates`** including `facade.rs`. dc will not make something on PATH start refusing on their own read, and **I declined to clear it for them** -- a guard cleared by a peer saying the ruling happened is not a guard, and the manner of a first clearing sets what clearing it costs afterwards.

**THE ADMISSION, AND IT IS THE PART I WOULD KEEP IF YOU KEEP ONE THING.** You had already declined two controls for this exact problem, BY NAME: direnv, because it does not reach automation; and hand-refresh, because an advisory that requires remembering is not a control. **My detector fails the same test -- it is a control that only exists where a human is already standing** -- and neither dc nor I applied your stated criterion to the third candidate. dc's own words: _I certified the detector by its EXISTENCE and never asked WHERE IT RUNS._ **You supplied the discriminator; we both stopped using it one item later.**

Nothing is being built on either question. dc is holding and will draft either once ruled.

## (2026-08-24 15:07Z)

**THREE FINDINGS THAT ARE dc's, CARRIED BY ME BECAUSE I WAS ALREADY WRITING. dc IS FILING ALL THREE THEMSELVES -- so this is ONE copy, correctly attributed, not a second node converging.** dc asked for exactly this shape: the thing that worked on the ref remedy and failed on cc's narrowing.

**1. dc's, HIGH: `intent upgrade` DESTROYS EVERY ISSUE IN AN ALREADY-MIGRATED v3 PROJECT, AND PRINTS `0 issue(s)` WHILE DOING IT.** Reproduced by dc in a throwaway on both v3 binaries, 5 issues -> 0, scripted so anyone can re-run. **It hit this repo for real: the store lost all 47 issues and canon kept them.** **THE DIAGNOSIS IS THE ASYMMETRY, and it is dc's: v2's upgrade SHORT-CIRCUITS when already at target; v3's does NOT -- it re-runs the migration on an already-migrated project, and the re-run is where the issues die.** cc's lane to fix, and **v3-ONLY under your freeze scope** -- dc drove the v2 arm and v2 does not share it.
**2. dc's, and it is the FLEET-FACING one: v2 `intent upgrade` BLOCKS ON AN INTERACTIVE READ, WITH NO `--yes` FLAG.** dc's first real-migration attempt hung and was killed at two minutes; the identical run with stdin CLOSED finished in seconds at rc=0. **11 of 16 fleet projects are below the 2.19.0 floor and need exactly this upgrade** -- anything driving it without a TTY (a hook, a script, a scheduled sweep, one of us) **hangs silently rather than failing.** Under your freeze scope this IS a shipped-surface defect, so **it lands in BOTH trees**, unlike (1).
**3. dc's: `intent/.backup/db/` IS EMPTY, and today is its evidence.** `doctor` has been flagging backup-stale residue for a while; **the first time anyone needed a pre-incident snapshot of the store, there was none.** dc's forensics had to reconstruct from freed pages instead.

**AND THE METHOD POINT IS dc's TOO, BECAUSE THEIR FIRST v2 ARM WAS A FALSE CLEAN THEY CAUGHT RATHER THAN SHIPPED.** It reported issues 5 -> 5 -- **but upgrade had printed "already at intent v2.19.0" and short-circuited, so they had measured that A NO-OP DESTROYS NOTHING.** They backdated the declared version to 2.10.0 to build a fixture that COULD fail before believing any clean result.

**MINE, FILED AS `intent#0069`, MEDIUM: `intent sync`'s OUTPUT MISDESCRIBES ITS OWN SCOPE, TWICE.** A thread-scoped `--to-store` warns `OVERWRITES: ST0057` then says **"store replaced from the extract, 1 thread(s)"** -- two scopes in one message, **and that wording is the entire reason my write looked like a plausible culprit for an hour.** And the unscoped restore printed **"the store and the extract agree; this restore overwrites nothing"** while the store held 0 issues and the extract 47 -- **then wrote anyway (db sha changed).** The agreement check evidently considers threads only. **A message that is backwards about the one thing that mattered.**

**RECOVERY DONE ON YOUR RULING AND VERIFIED BY FINGERPRINT RATHER THAN BY THE SUCCESS MESSAGE: store 47 == canon 47** (6 open, 41 closed), threads 58 intact, gate unmoved at 66 of 67, `sync --to-disk` back to rc=0. **The broken db is snapshotted in the scratchpad** -- it cost nothing and keeps the freed-page state independent of dc's script.

**MY OWN SLIP IN THE VERIFICATION, RECORDED BECAUSE IT IS THE THIRD TODAY: I checked the restore with `issues list --status all`. The flag is `--kind`.** It exited rc=1, my grep saw an empty stdout, and I reported **"store 0 vs canon 47"** and nearly called a successful restore a failure. **A wrong flag and a true zero are the same characters on screen.**

## (2026-08-24 16:35Z)

**YOUR RULING 1 IS IMPLEMENTED AND MEASURED IN CI ITSELF. `a38e884b`, pushed, all three refs level, both workflows green.**

**THE PROPERTY IS THE ONE YOU NAMED, VERBATIM: not _these two trees agree_ but _a shipped-surface change is either in both or declared_.** That is now the guard's stated contract, and it decides which of the two exception kinds a new entry gets.

**WHAT WAS ACTUALLY BROKEN, DEMONSTRATED BEFORE IT WAS FIXED.** `shipped_surface_drift.bats` resolved the v2 tree from a filesystem path only. On a CI runner no such directory exists, so **all three tests skipped -- INCLUDING THE POSITIVE CONTROL whose entire job is to prove the comparison looked at something** -- and because bats reports a skip as `ok`, the suite printed `All tests passed!` and exited 0. **It reported green over nothing for its whole first day, and the skip was in the original design rather than in a regression.** You had already declined direnv and hand-refresh BY NAME as controls that only exist where a human is already standing; **my detector failed the same test and nobody applied your criterion to it.** dc named it.

**THE FIX: two routes, one comparison.** The live v2 checkout is ground truth, because it is the tree `$INTENT_HOME` resolves to and therefore the only source that can answer _did this fix reach anybody_. `git archive` of the pushed `v2-maintenance` ref is the CI-reachable proxy. **Remote-tracking refs only, never a bare local `v2-maintenance`** -- your deleted-branch ruling is now structural inside the guard rather than remembered. Absence is **skip locally, FAIL in CI**: a guard that skips when its input is missing cannot tell _not applicable_ from _broken_.

**AND THE PROXY IS VALIDATED RATHER THAN ASSUMED, by a third arm.** CI compares against the REF; the fleet executes the CHECKOUT. **Measured: the checkout sits 2 commits ahead of the pushed ref and nothing pushes that branch.** The guard's answer survives today only because both commits are confined to `bin/.devbin/`, which the walk excludes anyway -- **luck, not a property.** Without that arm, a v2 shipped-surface fix that is committed but unpushed makes CI redden **naming DRIFT while blaming the tree that is correct.**

**CI IS THE EVIDENCE, NOT MY SIMULATION.** The run before mine was also green and contained three skips, **so a green alone settles nothing.** Read out of the log, both platforms:

```
ok 1146 transitional: the shipped surface has not diverged ...           RAN
ok 1147 positive control: ... non-empty surface on both sides            RAN
ok 1148 the pushed v2-maintenance ref still stands in ...  # skip        correct -- no checkout on a runner
ok 1149 the declared-exception list stays small ...                      RAN
```

**ONE THING FOR YOU TO RULE, AND I AM DELIBERATELY NOT CLEARING IT MYSELF.** dc asked whether the **frozen-`$INTENT_HOME` mechanism routing** is discharged. I answered NOT DISCHARGED, and my stated reason was this hole in the detector. **The hole is now closed, so my reason is gone** -- but a reason expiring is not the same as a routing being discharged, and **a guard cleared by a peer saying the ruling happened is not a guard.** Yours or dc's, not mine.

**TWO FINDINGS WORTH MORE THAN THE FIX.**

**MY FIRST DRAFT CARRIED THE SAME DEFECT THROUGH A DIFFERENT DOOR, AND ONLY THE NEGATIVE CONTROL FOUND IT.** Factoring the skip into a helper called as `v2="$(_helper)"` puts bats's `skip` and `fail` **inside a command substitution**, where they unwind the SUBSHELL and let the test continue with an empty path. Instead of skipping, it compared the whole v3 surface against `""` and reported **all 247 files as drifted** -- maximum noise, in CI, unattended, which is precisely the outcome dc warned me to avoid. **The old code was right and I "improved" it. Nothing about the new form looks wrong.** It was found by RUNNING the failing configuration, never by reading it -- **cc's principle paying out on my own work: a refusal survives what a reading does not.**

**THE EXCEPTION CAPS WERE A LATENT LANDMINE UNDER YOUR FREEZE RULING.** A single `count <= 6` treated both kinds as one overflow risk, but PENDING is a debt that must reach zero while **V3-ONLY growth is your ruling working as intended.** A shared cap fires on legitimate v3 divergence -- **and now that the guard runs unattended it would fire there first, which is how a guard trains people to ignore it.** Split. The v3-only cap is a CHECKPOINT rather than a limit, is derived from nothing, and says so; raising it is yours.

**Driven at `797ea1b7`:** 247 files in the shipped surface, 243 byte-identical, 2 differ, 2 absent -- and the declared lists account for exactly those four with nothing left over. The only v3/v2 delta is the RULES/ARCHITECTURE pair you already ruled v3-only.

## (2026-08-24 16:38Z)

**A DECISION THAT IS YOURS HAS BEEN SITTING ON TWO BOARDS AND REACHED YOUR INBOX FROM NEITHER, AND I AM THE NODE WHOSE JOB THAT IS.** Found by grepping this file for it and getting zero, with a positive control to prove the search worked. **dc's commit message says "the revert is hv's ruling"; my board says "Decision still hv's"; the roster names vc as the node obliged to surface hv-channel content TO you.** Both of us recorded it correctly and neither of us delivered it. **Nothing failed -- every write returned 0 -- which is precisely the shape the protocol warns about: a write surface with no delivery is a queue, and nothing observable distinguishes the two.**

**THE DECISION: `intent upgrade` ESCAPED AND MUTATED TWO TRACKED FILES IN THIS WORKING TREE AT 13:44:15Z. Do we revert them?**

**dc's answer, and mine, is KEEP BOTH -- but the REASON changed under us, which is the part worth your attention.**

- **`AGENTS.md`** is a GENERATED VIEW. The escape ran `agents sync` early, so the output is **correct and current**; reverting would restore a STALE view. A legitimate regeneration that arrived by an illegitimate route.
- **`intent/.config/config.json`** -- `project_id` is **LIVE IN THE STORE**. dc drove `.dump`, which emits live rows only and never freed pages, with both controls. Reverting diverges disk from store.

**THE ORIGINAL ARGUMENT WAS DIFFERENT AND IS NOW DEAD: _do not revert because `git checkout` cannot reach the db_.** That was an argument from an INCONSISTENT store, and **that premise died when the store was restored (47 == 47).** The verdict did not move while the reasoning underneath it was replaced entirely. **A right answer resting on a dead reason is indistinguishable from a live one until someone re-derives it** -- I asked dc to re-put it rather than carrying the earlier answer forward, and that is the only reason we know the current one stands on its own.

**AND MY EARLIER LIST OF THE AFFECTED FILES WAS WRONG IN A WAY THAT WOULD HAVE COST YOU WORK.** I reported THREE files; the incident touched **TWO**. Driven by mtime against the incident stamp:

```
AGENTS.md                  13:44:15Z   <- incident
intent/.config/config.json 13:44:15Z   <- incident
intent/llm/MODULES.md      13:30:54Z   <- THIRTEEN MINUTES EARLIER. NOT the incident.
bin/intent3                13:34:12Z   <- also dc's own work
```

**`MODULES.md`'s entire diff is ONE LINE: dc's `currency.lib` registration row** -- the row they registered before creating the file. **A revert of the list as I gave it destroys dc's work.** ic had the correct partition earlier and it drifted between there and my fold; **I assembled the set from what was DIRTY IN THE SAME WINDOW rather than from what the incident's own timestamp reaches.** One `stat` settles it and nobody ran it until dc did.

**WHAT I AM NOT DOING: deciding it.** Both files are tracked, both are yours, and I would rather you rule on a live reason than inherit a verdict whose argument expired.

**THE ROOT CAUSE IS ALREADY FILED AND IS NOT THIS DECISION:** `intent#0070`, dc's, HIGH -- `intent upgrade` re-runs the migration on an already-migrated v3 project and destroys every issue while printing `0 issue(s)`. v3-only; v2 short-circuits.
