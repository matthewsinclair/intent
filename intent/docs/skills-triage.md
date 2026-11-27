---
title: Skills catalogue triage
verblock: "2026-11-27:v1.3: Matthew Sinclair - synced to the code"
---

# Skills catalogue triage

This table gives each skill in the catalogue a verdict: KEEP, UPDATE or RETIRE (`ST0056/WP-15`). hv asked for it on 2026-08-17: _"There are a lot of skills that I can see that don't make sense anymore, and it's been forever since the skills catalog (for Claude) had anyone look at it."_

`native/rust/crates/intent-cli/tests/skills_triage_covers_the_catalogue.rs` holds the table to the catalogue as a set, in both directions. Every skill under `intent/plugins/claude/skills/` has a row, no row names a skill that is not there, and each verdict is one of the three. So this page gives no count. `ls -1d intent/plugins/claude/skills/*/` lists the catalogue in the source tree, and `intent claude skills list` lists the catalogue that the installed Intent carries.

## The table

| Skill                    | Verdict    | Reason                                                                                                                                                                                                                   |
| ------------------------ | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `in-ash-ecto-essentials` | KEEP       | Ash and Ecto access rules. `/in-session` loads it for the `elixir` language when the project depends on `:ash`.                                                                                                          |
| `in-author-essentials`   | KEEP       | The `author` pack's pipeline; `author` is a declared language of this project. Chains to `in-detrope`.                                                                                                                   |
| `in-autopsy`             | KEEP       | Session forensics. Its script reads who wrote each entry and what each verb printed. The skill edits no rule, so the house style has one home.                                                                           |
| `in-content-essentials`  | KEEP       | The `content` pack's pipeline; `content` is a declared language of this project. Chains to `in-detrope`.                                                                                                                 |
| `in-detrope`             | KEEP       | The one home of the trope catalogue (`data/trope-catalog.md`), which `IN-PR-STYLE-004` and both prose packs read.                                                                                                        |
| `in-elixir-essentials`   | KEEP       | Elixir production rules, listed in its `rules:` frontmatter. `/in-session` loads it for the declared `elixir` language.                                                                                                  |
| `in-elixir-testing`      | KEEP       | Elixir test rules, listed in its `rules:` frontmatter. The same loader.                                                                                                                                                  |
| `in-essentials`          | KEEP       | Intent's workflow rules and formatting standards. It took the formatting standards of `in-standards` when that skill was removed on 2026-10-25 (ST0088 WP-02). It names `intent/llm/AGENTS.md` only to retire that path. |
| `in-finish`              | KEEP       | The localfold and the globalfold. Chains to `in-whiteboard`.                                                                                                                                                             |
| `in-phoenix-liveview`    | KEEP       | LiveView rules. `/in-session` loads it when the project depends on `:phoenix_live_view`.                                                                                                                                 |
| `in-plan`                | **UPDATE** | Its `chains_to` must agree with its body. Since 2026-10-25 (ST0088 WP-02) the body invokes no coding skill, because `/in-session` loads them, so `chains_to` is `[]`.                                                    |
| `in-review`              | KEEP       | The two-stage review that dispatches `critic-<lang>`, as `CLAUDE.md` and `intent/docs/critics.md` say. Chains to `in-verify`.                                                                                            |
| `in-session`             | KEEP       | The bootstrap every session runs. `chains_to` names every skill its body loads.                                                                                                                                          |
| `in-verify`              | KEEP       | The verification gate, through `ac gate`, `ac satisfy`, `ac status` and `at green`.                                                                                                                                      |
| `in-whiteboard`          | KEEP       | The whiteboard protocol. `chains_to: []` is correct: `in-session` and `in-finish` chain to it.                                                                                                                           |

Two tests hold what every row relies on. `no_skill_names_an_unshipped_verb.rs` holds that each `intent <verb>` a skill names ships. `every_skill_has_a_live_caller.rs` holds that each `chains_to` target exists and that each skill is reached.

## No RETIRE verdict

The triage found no skill to retire. `in-handoff`, `in-next` and `in-start` were retired before it. Each of these grounds would have given a RETIRE, and each was checked:

- **It names a verb that does not ship.** Every `intent <verb>` that a skill names ships, and `no_skill_names_an_unshipped_verb.rs` holds this.
- **It names a v2-era path.** The one such path, `intent/llm/AGENTS.md` in `in-essentials`, is named only to retire it.
- **It has no caller.** Every skill is chained to, or invoked by name in, `AGENTS.md`, `CLAUDE.md`, `usage-rules.md` or a `SKILL.md`, and `every_skill_has_a_live_caller.rs` holds this. A count of references must leave out the catalogue documents: `usage-rules.md` lists every skill, so a count that includes it measures being listed, not being called.
- **A newer mechanism replaced it**, such as the rule library, `critic-<lang>` or the AC and AT contract. No skill restates one, and the language packs point at the rule library.

hv ruled the one open question on 2026-10-25 (ST0088 WP-01). The five `in-tca-*` skills, `in-cost-analysis` and `in-debug` left the catalogue with their scripts. The Total Codebase Audit's written procedure is at `git show 13754d86f:intent/docs/total-codebase-audit.md`.
