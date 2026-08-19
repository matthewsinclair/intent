---
st_id: ST0052
title: Author project-type pack
---

# ST0052: Author project-type pack -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) The author pack works end to end: a project declaring `languages: [author]` loads `/in-author-essentials` on `/in-session` and `/in-review` dispatches only `critic-author` (not the code critics); `critic-author` runs the mechanical tier by default and recommends `/in-detrope` for full diagnosis under direct instruction. -- evidence: impl.md dogfood (mechanical tier; D3/D4 evidence) + WP05 wiring tests (in_session_skill/critic_dispatch) + hv sign-off 'ship as minor v2.15.0' (2026-07-03); live Task(critic-author) available post-restart -- satisfied: yes

### WP-01 -- AU language-code schema bump (status: Done)

- AC-01.1 A well-formed author rule id (eg `IN-AU-STYLE-001`) passes the rule-id validator, and a malformed id (bad code, missing zero-padding) still fails. -- satisfied: yes (computed)
- AC-01.2 (non-test) All four ID/validation sites carry `author`/`AU` consistently. -- evidence: grep of `rule-schema.md` enum + `id-scheme.md` codes/regex + `index-generator.md` regex + `intent_claude_rules` regex -- satisfied: yes
- AC-01.3 (non-test) The widening is scoped to the ID/validation layer -- `bin/intent_critic` and the config/template layer are untouched by WP-01 (D4). -- evidence: `git diff` for WP-01 -- satisfied: yes

### WP-02 -- Author rule library seed (status: Done)

- AC-02.1 Every author rule (style + craft) is schema-valid: `intent claude rules validate` passes it (frontmatter + all nine sections + a well-formed `IN-AU-*` id). -- satisfied: yes (computed)
- AC-02.2 (non-test) The `style` tier carries greppable Detection (mechanical); the `craft` tier is judgment / critic-as-reader; each rule's severity + category are correct. -- evidence: style tier greppable (warning/reco); craft tier judgment (recommendation); category+severity split guarded by rule_pack_author.bats -- satisfied: yes
- AC-02.3 (non-test) The mechanical trope pass references `in-detrope/data/trope-catalog.md` (the single trope home), not a duplicated indicator set. -- evidence: mechanical-trope-pass Detection cites in-detrope/data/trope-catalog.md; no vendored indicators (guarded) -- satisfied: yes
- AC-02.4 (non-test) `rules/author/index.json` is regenerated and lists the author rules. -- evidence: rules/index.json regenerated; lists all 9 IN-AU-* rules -- satisfied: yes

### WP-03 -- critic-author subagent (status: Done)

- AC-03.1 The `critic-author` subagent exists (`agent.md` + `metadata.json`), declares tools `Read, Grep, Glob, Bash` and neither `Write` nor `Edit`, and is registered in `.manifest/global-agents.json`. -- satisfied: yes (computed)
- AC-03.2 (non-test) `agent.md` honours the two-tier contract (D3): a mechanical `style` pass by default (`review`) and a judgment `craft` pass on instruction (`craft-check`); it reports only and never writes, edits, or runs external fixers. -- evidence: agent.md: review (style/mechanical, default) + craft-check (craft/judgment, on instruction); read-only, no autofix; guarded by critic_author.bats -- satisfied: yes
- AC-03.3 (non-test) The two-form detrope (D5) is wired: the mechanical trope pass (`IN-AU-STYLE-005`) runs by default; the full `/in-detrope` diagnosis (`IN-AU-CRAFT-003`) is emitted as a handoff recommendation, never invoked by the critic. -- evidence: agent.md Two-form detrope: STYLE-005 mechanical pass by default; CRAFT-003 /in-detrope emitted as handoff, never invoked; guarded -- satisfied: yes
- AC-03.4 (non-test) Scope is on-demand only -- `bin/intent_critic` (the headless gate) is untouched (deferred D4); the widening is the subagent + its manifest row. -- evidence: git diff: subagents/critic-author + .manifest + acceptance.md + tests only; bin/intent_critic untouched (D4) -- satisfied: yes

### WP-04 -- intent lang init author canon (status: Done)

- AC-04.1 `intent lang init author` in a fresh project installs `intent/llm/RULES-author.md` and `intent/llm/ARCHITECTURE-author.md`, appends the `author` Language Packs entry to the agnostic `RULES.md`, and adds `author` to `config.json` `languages`. -- satisfied: yes (computed)
- AC-04.2 `intent lang list` enumerates `author`. -- satisfied: yes (computed)
- AC-04.3 (non-test) The author canon templates at `templates/author/{RULES.md,ARCHITECTURE.md}` carry the two-tier framing and the book/course IA (parts/chapters/modules/objectives), citing `IN-AU-*` ids. -- evidence: templates/author/RULES.md (two-tier framing + NEVER-DO citing IN-AU-*) + ARCHITECTURE.md (book/course IA: work layout, unit structure, objectives, pipeline, review) -- satisfied: yes

### WP-05 -- Skill and dispatch wiring (status: Done)

- AC-05.1 The `/in-author-essentials` skill exists (`SKILL.md`, valid frontmatter) and carries the authoring pipeline (outline -> draft -> mechanical detrope -> revise -> structural check), references the nine `IN-AU-*` rule ids, and is renderer-safe (no em dashes, no `$N` positional tokens). -- satisfied: yes (computed)
- AC-05.2 `/in-review` dispatches `author -> critic-author` (a `subagent_type="critic-author"` Task example) and documents the D7 exclusion (author-only runs no code critic; a mixed project runs both on their subtrees). -- satisfied: yes (computed)
- AC-05.3 `/in-session` lists an `author` fan-out row invoking `/in-author-essentials` and includes `in-author-essentials` in `chains_to`. -- satisfied: yes (computed)

### WP-06 -- Dogfood, docs, and close (status: Done)

- AC-06.1 (non-test) The pack is dogfooded: the mechanical `style` tier ran against real prose, the transcript is captured in-repo (`impl.md`), and there are no unresolved defects. -- evidence: impl.md WP06 dogfood: mechanical style tier run by hand against the pack's own prose; all hits house-style false positives; no unresolved defects -- satisfied: yes
- AC-06.2 (non-test) Docs shipped: a CHANGELOG v2.15.0 entry, `intent/history/v2.15.0.md`, `docs/releases/2.15.0/RELEASE_NOTES.md`, and `usage-rules.md` author-pack entries (Skills, Subagents, Rule Library, Critics). -- evidence: CHANGELOG [2.15.0] + intent/history/v2.15.0.md + docs/releases/2.15.0/RELEASE_NOTES.md + usage-rules author entries -- satisfied: yes
- AC-06.3 (non-test) Release framing ratified: minor v2.15.0 (new non-code project-type surface; opt-in). -- evidence: hv directive 2026-07-03: ship as minor v2.15.0 -- satisfied: yes

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- AU language-code schema bump (status: Done)

- AT-01.1 `tests/unit/au_language_code_guard.bats` -- covers AC-01.1 -- status: green

### WP-02 -- Author rule library seed (status: Done)

- AT-02.1 `tests/unit/rule_pack_author.bats` -- covers AC-02.1 -- status: green

### WP-03 -- critic-author subagent (status: Done)

- AT-03.1 `tests/unit/critic_prose.bats` -- covers AC-03.1 -- status: green -- deck renamed critic_author -> critic_prose in ST0053 WP03

### WP-04 -- intent lang init author canon (status: Done)

- AT-04.1 `tests/unit/intent_lang.bats` -- covers AC-04.1, AC-04.2 -- status: green

### WP-05 -- Skill and dispatch wiring (status: Done)

- AT-05.1 `tests/unit/in_author_essentials_skill.bats` -- covers AC-05.1 -- status: green
- AT-05.2 `tests/unit/critic_dispatch.bats` -- covers AC-05.2 -- status: green
- AT-05.3 `tests/unit/in_session_skill.bats` -- covers AC-05.3 -- status: green

### WP-06 -- Dogfood, docs, and close (status: Done)

_(no tests in this group)_

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
