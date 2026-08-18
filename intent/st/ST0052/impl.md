# Implementation - ST0052: Author project-type pack

As-built notes. Scope + decisions live in `design.md`; the AC/AT contract in `acceptance.md`. This file records what was built (WP01-WP05) and the WP06 dogfood.

## As-built (WP01-WP05)

| WP   | Built                                                                                                                                                                                                          | Commits          |
| ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| WP01 | `AU` language code across the ID/validation sites (rule-schema enum, id-scheme codes+regex, index-generator regex, `intent_claude_rules` validator) and the canon enumerator `LANG_SUBDIRS` + guard            | 14fe6c3, e930cad |
| WP02 | Nine `IN-AU-*` rules -- 5 `style` (mechanical, greppable) + 4 `craft` (judgment, critic-as-reader); global `rules/index.json` regenerated; `rule_pack_author.bats`                                             | ea54dca, 6c5bd19 |
| WP03 | `critic-author` subagent -- read-only, two-tier (`review` / `craft-check`), two-form detrope (STYLE-005 mechanical default + CRAFT-003 `/in-detrope` handoff); registered in `.manifest`; `critic_author.bats` | 2fdd923          |
| WP04 | `templates/author/{RULES.md,ARCHITECTURE.md}` -- `intent lang init author` installs them + writes config `languages`; `intent_lang.bats` author cases                                                          | 7d48a1b          |
| WP05 | `/in-author-essentials` skill + `author -> critic-author` wired into `/in-review` (D7 exclusion note) and `/in-session` (fan-out row + `chains_to`); ATs in their Highlander homes                             | 447e5cc          |

The five build WPs closed through the acceptance gate. `bin/intent_critic` (the headless prose gate) was left untouched -- deferred as D4.

## WP06 dogfood (mechanical tier)

`critic-author` and `/in-author-essentials` are canon but not live in the building session (the agent + skill registries are read at session start), so the dogfood applied the mechanical `style` tier by hand -- exactly as the STYLE-005 Detection specifies -- against the prose authored for the pack itself (`templates/author/*`, the skill, the critic `agent.md`).

Result: no real defects. Every raw hit was a house-style false positive, and that is the finding:

- The STYLE-005 trope pass extracts the `detection: automated` regexes from the single `trope-catalog.md` (the D5 Highlander path) and applies them. It fired heavily -- but on Intent's own house style: the mandated `--` (em-dash substitute) trips the catalogue's dash-overuse regex; bold-bullet lists trip the over-structuring regex; "the author" (the pack name) trips the fourth-wall "the author" regex; the unicode-symbol class matched ASCII `->` arrows. A targeted check confirmed the files carry no real Unicode.
- The `eg`-not-`e.g.` hits were backtick-quoted meta-references in docs that are _about_ the rule.

This validates the design:

1. The mechanical tier runs end to end off the single catalogue (D5).
2. Every candidate here required the judgment / context-confirmation step to dismiss -- direct evidence for the D3 two-tier split (mechanical produces candidates; judgment confirms).
3. Intent's `--` house style directly trips the dash-overuse trope, so a _headless_ prose gate (D4) would be swamped without a confirmation / house-style-suppression layer. This reinforces the D4 deferral.

The live path -- `/in-session` loading `/in-author-essentials` from a `languages: [author]` project, and `/in-review` dispatching only `critic-author` -- is verified by the WP05 acceptance tests (`in_session_skill.bats`, `critic_dispatch.bats`) and available for a live `Task(critic-author)` run in any session that picks up the new registries.

## Close

Shipped as minor v2.15.0 (first non-code project-type; new command surface) on hv's directive (2026-07-03). ST-level AC-00.1 was signed off on the manual-dogfood approach plus the wiring verified by the WP05 acceptance tests.
