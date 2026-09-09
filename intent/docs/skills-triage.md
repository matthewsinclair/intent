---
title: Skills catalogue triage
verblock: "2026-09-09: v1.0 - vc - one row per skill, KEEP / UPDATE / RETIRE, denominator enumerated at close"
---

# Skills catalogue triage

`ST0056/WP-15`. hv's framing, 2026-08-17: _"There are a lot of skills that I can see that don't make sense anymore, and it's been forever since the skills catalog (for Claude) had anyone look at it."_

## The denominator, and why it is not written in this sentence

**Count it: `ls -1d intent/plugins/claude/skills/*/ | wc -l`.** The figure is deliberately absent from this prose, because `WP-15`'s own objective records it going stale **four times** in one document -- right at authoring, wrong when `in-handoff` retired, corrected in the view and silently reverted by a regeneration, corrected in canon in a field that carried it twice so one copy was repaired and the other left contradicting it, and stale a third time when `in-next` and `in-start` retired. **A denominator in prose goes stale by construction.** `AT-15.1` asserts this table's rows against the catalogue **enumerated at close**, as a set equality rather than a count, so a skill added or removed after this document was written reddens a test rather than quietly falsifying a sentence.

**AN ABSENT ROW AND AN UNEXAMINED SKILL ARE THE SAME ABSENCE.** Every skill the enumeration returns has a row below, including every one whose verdict is KEEP with no change. A triage that lists only what it changed has an unmeasured arm, and the unmeasured arm is precisely the skills nobody looked at.

## The table

| Skill                    | Verdict    | Reason                                                                                                                                                                                                                            |
| ------------------------ | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `in-ash-ecto-essentials` | KEEP       | Ash/Ecto access rules, loaded by `/in-session` on the elixir + `:ash` dependency fan-out. Names one verb, `claude rules`, which ships.                                                                                            |
| `in-author-essentials`   | KEEP       | The `author` pack's pipeline; `author` is a declared language of this project. Chains to `in-detrope`. Names `claude rules` and `lang init`; both ship.                                                                           |
| `in-autopsy`             | KEEP       | Session forensics. **The one data divergence on record is GONE**: `banned-words.txt` no longer carries `overall`, which `CLAUDE.md` dropped 2026-08-24, and its `Absolutely` agrees with canon. Re-driven.                        |
| `in-content-essentials`  | KEEP       | The `content` pack's pipeline; `content` is a declared language. Same shape as the author pack.                                                                                                                                   |
| `in-cost-analysis`       | KEEP       | ST0027's deliverable; ships a 479-line script and a rates table. No stale path or verb. **Its `reference-rates.md` is a dated external fact and is this catalogue's clearest future-staleness risk.**                             |
| `in-debug`               | KEEP       | Four-phase debugging, chains to `in-verify`. Language-agnostic; names no verb, so nothing in it can rot against the surface.                                                                                                      |
| `in-detrope`             | KEEP       | The single home of the trope catalogue (2,819 lines), read by `IN-PR-STYLE-004` and by both prose packs. Most-referenced asset in the catalogue.                                                                                  |
| `in-elixir-essentials`   | KEEP       | Elixir production rules, `IN-EX-CODE-001..006`. Loaded by `/in-session` on the declared `elixir` language.                                                                                                                        |
| `in-elixir-testing`      | KEEP       | Elixir test rules, `IN-EX-TEST-001..007`. Same loader.                                                                                                                                                                            |
| `in-essentials`          | KEEP       | Corrected 2026-09-08: step 3 no longer orders state into `.claude/restart.md`, which contradicted `/in-finish` about the same file. Its `intent/llm/AGENTS.md` mention RETIRES that path, not instructs it.                       |
| `in-finish`              | KEEP       | Corrected 2026-08-31: the localfold-versus-`release` contradiction. Names `ac descope`/`rescope`/`withdraw`, `at lint`, `st done`, `wp done` -- all ship.                                                                         |
| `in-phoenix-liveview`    | KEEP       | LiveView rules, loaded on the `:phoenix_live_view` dependency fan-out.                                                                                                                                                            |
| `in-plan`                | **UPDATE** | **`chains_to: []` IS FALSE.** Its own description says _invoke coding skills_ and its body invokes eight. The field is semantic rather than parsed, so it misleads a reader and breaks nothing -- which is why nothing caught it. |
| `in-review`              | KEEP       | Two-stage review dispatching `critic-<lang>`. 38 non-catalogue references, the most-wired skill after `in-session`.                                                                                                               |
| `in-session`             | KEEP       | The bootstrap every session runs. `chains_to` is a complete ten-entry list -- **checked rather than assumed, because `in-plan`'s was not.**                                                                                       |
| `in-standards`           | KEEP       | Corrected 2026-09-03: `MODULES.md` and `DECISION_TREE.md` now carry their opposite dispositions instead of one sentence collapsing both.                                                                                          |
| `in-tca-audit`           | KEEP       | TCA step 2. Names `claude subagents`, which ships. Reached from `in-tca-init`.                                                                                                                                                    |
| `in-tca-finish`          | KEEP       | TCA step 5, chains to `in-finish`.                                                                                                                                                                                                |
| `in-tca-init`            | KEEP       | TCA entry point, documented at `intent/docs/total-codebase-audit.md`. **Its `~/.intent/ext/` mention RETIRES that layout in the same sentence** -- it is not a live instruction.                                                  |
| `in-tca-remediate`       | KEEP       | TCA step 4. Fewest external references in the catalogue (3), all of them its own family and the registries -- **which is its workflow, not orphanhood.**                                                                          |
| `in-tca-synthesize`      | KEEP       | TCA step 3.                                                                                                                                                                                                                       |
| `in-verify`              | KEEP       | The verification gate. Names `ac gate`/`satisfy`/`status`, `at lint`, `at red` -- all five ship, driven.                                                                                                                          |
| `in-whiteboard`          | KEEP       | Protocol 3.0. `chains_to: []` is CORRECT here -- `in-session` and `in-finish` chain **to** it, not from it.                                                                                                                       |

## ZERO RETIRE IS A RESULT, AND THE GROUNDS I LOOKED FOR ARE NAMED SO THE ZERO CAN BE ARGUED WITH

A triage returning no retirements against hv's premise that _a lot of skills don't make sense anymore_ owes its reasoning, not a shrug. **Three retirements have ALREADY happened and I verified all three are gone**: `in-handoff`, `in-next` and `in-start`. The catalogue went 26 -> 23, so the premise was acted on before this table existed.

**Four grounds would each have produced a RETIRE, and each was driven:**

- **Names a verb the surface does not carry.** Every `intent <verb>` spelling in every skill was extracted and invoked. **All ship**, including the four I expected to fail: `at lint`, `at red`, `fc`, `claude subagents`.
- **Names a v2-era path.** Two hits in 23 skills: `intent/llm/AGENTS.md` in `in-essentials` and `~/.intent/ext/` in `in-tca-init`. **BOTH RETIRE THE THING THEY NAME IN THE SAME SENTENCE.** The naive stale-reference test scores **0 for 2** on this corpus, which is the discriminator constraint `AC-15.2` records and this is its measurement.
- **Orphaned -- no caller.** Every skill has non-catalogue references, from 3 to 50. **The first pass of this measurement was WRONG and inflated every count**, because `usage-rules.md` and `CHANGELOG.md` list every skill by construction; a reference count including them measures being CATALOGUED, not being CALLED. Re-driven with catalogue documents excluded.
- **Superseded by a mechanism that replaced it** -- the rule library, `critic-<lang>`, the AC/AT contract. No skill was found duplicating one; the language packs point AT the rule library rather than restating it.

**WHAT THIS TRIAGE DOES NOT ESTABLISH.** Whether Intent should still ship a Total Codebase Audit workflow at all is a product-scope call and is hv's, not the pen's. The five TCA skills are internally coherent, reachable from a documented entry point, and name nothing that has moved -- so **no measurement available to me supports retiring them**, and retiring five skills on an impression is what this table exists to prevent.
