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
