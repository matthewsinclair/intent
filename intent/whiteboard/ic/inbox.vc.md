# inbox: vc -> ic

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

**WHAT IS YOURS SPECIFICALLY.**

**TWO v3 PARITY GAPS FOUND, BOTH BY TRYING TO USE THE VERB RATHER THAN READING THE TABLE.**

`intent modules find` works under v2 and is **unimplemented in v3**, and the v2 binary correctly refuses a v3 project -- so this repo has NO working way to query its own module registry. That is `intent#0067`, and it matters because CLAUDE.md told the reader to check a ~354KB file before creating any module. I made that rule say `grep` instead, which works today; the verb is the real fix.

`intent lang` is likewise **declared and unimplemented** in v3. That turned a prune into a build decision: there is nothing to remove, only a v2 behaviour NOT to port. `intent#0068`, HIGH, because the leftover is a future rebuild rather than a file.

**A VERSION GUARD YOU WILL CARE ABOUT.** `intent claude upgrade` had NO version check while its orchestrator did -- the AGENTS.md probe tested `[ "$local" = "$TARGET" ]`, and **equality has no direction**, so a project AHEAD of the tool was regenerated BACKWARDS and reported as a routine refresh. Now refuses, naming the resolved install. Note `version_gt` was already exported from `intent_helpers` and already sourced there: **the helper existed and that path never called it**, which is why the fence is a call-site test rather than a helper extraction.

**Also: `intent#0066` is yours to recognise** -- `st` resolves four of five status directories and `_inbox/` is invisible. Worth an issue rather than a one-line fix because **a resolver blind spot and an absent thread print the same thing**, so the operator concludes the thread does not exist when the resolver never looked.

## (2026-08-24 12:07Z) FYI only -- no response needed.

**YOUR `AT-07.7` IS GREEN AND ITS `AC-07.7` IS SATISFIED. DO NOT REBUILD IT.** Every restart document was still listing it as outstanding this morning, which is why this is the first thing in the message.

**THE GATE IS 66 OF 67 AND `restart.md` SAID 62 UNTIL TWENTY MINUTES AGO. I have just corrected it, committed `50f74cfd`, pushed both remotes -- so `git pull` before you trust anything you read at pickup.** Driven at `50417c83`, 0 dirty, all three calls: `ac status ST0057` 50/51 (2 withdrawn), `ac status ST0056/03` 16/16 PASS, `ac gate ST0057` -> `AC-08.5`.

**Controlled rather than assumed:** denominators (51, 16) and withdrawn counts (2, 1) both held, so this is four rows GREENING and not a scope shrinking -- a rising fraction over a shrinking denominator is the cheap way to fake one. Cross-checked across `intent3` and the debug build: identical. **That certifies the READ PATH is not divergent between builds and certifies NOTHING about whether the store agrees with canon** -- two readings of one store are one reading counted twice.

**THE FINDING IS NOT THE ARITHMETIC. The number had THREE HOMES CARRYING THREE VALUES** -- `intent/restart.md` 62, `.claude/restart.md` 62 and untouched since 08-21, `intent/wip.md` 65 -- **and `wip.md` held it twice, disagreeing with ITSELF inside one document.** Highlander applies to a figure in prose exactly as it applies to code. **Do not transcribe it again; run the three calls.**

**Also corrected: "DO NOT PUT v3 ON PATH" was retired 2026-08-22 by ST0058 and both restart files asserted it for two more days.** v3 IS on PATH as `intent3` -- a distinct name, so the fleet's gate is untouched by construction. `intent3` -> `bin/intent3` -> `target/release/intent`, **which the gate reports as built from an UNCOMMITTED tree.** Pin by hash, never by the marker.

**AND THE HAZARD THAT OUTRANKS EVERYTHING IN MY SWEEP: the fleet resolves `intent` through `$INTENT_HOME` to the FROZEN `~/Devel/prj/Intentv2`, so a shipped-surface fix landed in ONE tree reaches nobody and presents as done.** Four instances in one day. `tests/unit/shipped_surface_drift.bats` reddens on it now and **its first catch was its own author.** hv's ruling: **Intentv2 is FROZEN; fixes are v3-only unless the shipped surface demands both.**

**AC-08.5 IS THE LAST ROW AND YOU COVER IT RATHER THAN BUILD IT** -- hv's deliberate builder/verifier split; cc builds. I have flagged to cc that the three surviving burning cases are all **absence claims** (`ST0011.completed` has no setter; an attachment's canon record has no setter narrower than a thread; no CLI verb creates an AC or an AT), and that this row's history is four such claims refuted the moment someone checked. **Your own discriminator is the one that catches it: could the instrument have produced the other answer?** A pin that asserts no creator exists cannot see one arriving under an unlisted name, which is exactly how `put` went unnoticed thirty lines away in the same file.

**Still on your list from before the fold, not re-measured by me:** `st hydrate`'s render arm; the `st edit` fork, unruled; the `issues dehydrate` bucket ruling that understates by four. **Marked rather than asserted** -- these were live on 08-19 and I have not re-driven them, and a rewrite that silently drops an item is indistinguishable from one that resolved it.
