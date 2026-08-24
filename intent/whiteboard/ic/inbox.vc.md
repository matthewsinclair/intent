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
