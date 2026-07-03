---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
st_id: ST0052
title: "Author project-type pack -- acceptance contract"
---

# ST0052 Author project-type pack -- Acceptance

> Canonical acceptance contract for ST0052. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> Exemption (ST0048): the close-gate is fail-by-default -- a unit with an empty or missing contract is refused. A unit that is deliberately AC-free (eg a pure content / authorial task) declares `acceptance: exempt` in the frontmatter above; the gate then passes and announces the exemption. Omit it (the default) and the contract is enforced. Never inferred from emptiness; always declared.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) The author pack works end to end: a project declaring `languages: [author]` loads `/in-author-essentials` on `/in-session` and `/in-review` dispatches only `critic-author` (not the code critics); `critic-author` runs the mechanical tier by default and recommends `/in-detrope` for full diagnosis under direct instruction. -- evidence: WP-06 dogfood transcript + hv sign-off -- satisfied: no

### WP-01 -- AU language-code schema bump (status: Done)

- AC-01.1 A well-formed author rule id (eg `IN-AU-STYLE-001`) passes the rule-id validator, and a malformed id (bad code, missing zero-padding) still fails.
- AC-01.2 (non-test) All four ID/validation sites carry `author`/`AU` consistently. -- evidence: grep of `rule-schema.md` enum + `id-scheme.md` codes/regex + `index-generator.md` regex + `intent_claude_rules` regex -- satisfied: yes
- AC-01.3 (non-test) The widening is scoped to the ID/validation layer -- `bin/intent_critic` and the config/template layer are untouched by WP-01 (D4). -- evidence: `git diff` for WP-01 -- satisfied: yes

### WP-02 -- Author rule library seed (status: Done)

- AC-02.1 Every author rule (style + craft) is schema-valid: `intent claude rules validate` passes it (frontmatter + all nine sections + a well-formed `IN-AU-*` id).
- AC-02.2 (non-test) The `style` tier carries greppable Detection (mechanical); the `craft` tier is judgment / critic-as-reader; each rule's severity + category are correct. -- evidence: style tier greppable (warning/reco); craft tier judgment (recommendation); category+severity split guarded by rule_pack_author.bats -- satisfied: yes
- AC-02.3 (non-test) The mechanical trope pass references `in-detrope/data/trope-catalog.md` (the single trope home), not a duplicated indicator set. -- evidence: mechanical-trope-pass Detection cites in-detrope/data/trope-catalog.md; no vendored indicators (guarded) -- satisfied: yes
- AC-02.4 (non-test) `rules/author/index.json` is regenerated and lists the author rules. -- evidence: rules/index.json regenerated; lists all 9 IN-AU-* rules -- satisfied: yes

### WP-03 -- critic-author subagent (status: Done)

- AC-03.1 The `critic-author` subagent exists (`agent.md` + `metadata.json`), declares tools `Read, Grep, Glob, Bash` and neither `Write` nor `Edit`, and is registered in `.manifest/global-agents.json`.
- AC-03.2 (non-test) `agent.md` honours the two-tier contract (D3): a mechanical `style` pass by default (`review`) and a judgment `craft` pass on instruction (`craft-check`); it reports only and never writes, edits, or runs external fixers. -- evidence: agent.md: review (style/mechanical, default) + craft-check (craft/judgment, on instruction); read-only, no autofix; guarded by critic_author.bats -- satisfied: yes
- AC-03.3 (non-test) The two-form detrope (D5) is wired: the mechanical trope pass (`IN-AU-STYLE-005`) runs by default; the full `/in-detrope` diagnosis (`IN-AU-CRAFT-003`) is emitted as a handoff recommendation, never invoked by the critic. -- evidence: agent.md Two-form detrope: STYLE-005 mechanical pass by default; CRAFT-003 /in-detrope emitted as handoff, never invoked; guarded -- satisfied: yes
- AC-03.4 (non-test) Scope is on-demand only -- `bin/intent_critic` (the headless gate) is untouched (deferred D4); the widening is the subagent + its manifest row. -- evidence: git diff: subagents/critic-author + .manifest + acceptance.md + tests only; bin/intent_critic untouched (D4) -- satisfied: yes

### WP-04 -- intent lang init author canon (status: Done)

- AC-04.1 `intent lang init author` in a fresh project installs `intent/llm/RULES-author.md` and `intent/llm/ARCHITECTURE-author.md`, appends the `author` Language Packs entry to the agnostic `RULES.md`, and adds `author` to `config.json` `languages`.
- AC-04.2 `intent lang list` enumerates `author`.
- AC-04.3 (non-test) The author canon templates at `templates/author/{RULES.md,ARCHITECTURE.md}` carry the two-tier framing and the book/course IA (parts/chapters/modules/objectives), citing `IN-AU-*` ids. -- evidence: templates/author/RULES.md (two-tier framing + NEVER-DO citing IN-AU-*) + ARCHITECTURE.md (book/course IA: work layout, unit structure, objectives, pipeline, review) -- satisfied: yes

### WP-05 -- Skill and dispatch wiring (status: Done)

- AC-05.1 The `/in-author-essentials` skill exists (`SKILL.md`, valid frontmatter) and carries the authoring pipeline (outline -> draft -> mechanical detrope -> revise -> structural check), references the nine `IN-AU-*` rule ids, and is renderer-safe (no em dashes, no `$N` positional tokens).
- AC-05.2 `/in-review` dispatches `author -> critic-author` (a `subagent_type="critic-author"` Task example) and documents the D7 exclusion (author-only runs no code critic; a mixed project runs both on their subtrees).
- AC-05.3 `/in-session` lists an `author` fan-out row invoking `/in-author-essentials` and includes `in-author-essentials` in `chains_to`.

### WP-06

[ACs defined when each WP starts (five-step, red-first). Placeholder -- not yet ratified.]

## Acceptance Tests

### WP-01

- AT-01.1 tests/unit/au_language_code_guard.bats -- covers AC-01.1 -- status: green
- Coverage: AC-01.1 by AT-01.1; AC-01.2 and AC-01.3 are non-test (evidence on the AC line).

### WP-02

- AT-02.1 tests/unit/rule_pack_author.bats -- covers AC-02.1 -- status: green
- Coverage: AC-02.1 by AT-02.1; AC-02.2, AC-02.3, AC-02.4 are non-test (evidence on the AC line).

### WP-03

- AT-03.1 tests/unit/critic_author.bats -- covers AC-03.1 -- status: green
- Coverage: AC-03.1 by AT-03.1; AC-03.2, AC-03.3, AC-03.4 are non-test (evidence on the AC line).

### WP-04

- AT-04.1 tests/unit/intent_lang.bats -- covers AC-04.1, AC-04.2 -- status: green
- Coverage: AC-04.1 and AC-04.2 by AT-04.1 (author cases); AC-04.3 is non-test (evidence on the AC line).

### WP-05

- AT-05.1 tests/unit/in_author_essentials_skill.bats -- covers AC-05.1 -- status: green
- AT-05.2 tests/unit/critic_dispatch.bats -- covers AC-05.2 -- status: green
- AT-05.3 tests/unit/in_session_skill.bats -- covers AC-05.3 -- status: green
- Coverage: AC-05.1 by AT-05.1; AC-05.2 by AT-05.2; AC-05.3 by AT-05.3.

### WP-06

[ATs defined with their WP's ACs.]
