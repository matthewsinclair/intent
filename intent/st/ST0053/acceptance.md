---
verblock: "07 Jul 2026:v0.1: matts - Initial version"
st_id: ST0053
title: "Content (web-content) project-type pack -- acceptance contract"
---

# ST0053 Content (web-content) project-type pack -- Acceptance

> Canonical acceptance contract for ST0053. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
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

- AC-00.1 (non-test) The content pack ships whole: the `IN-PR-*` prose base, the `IN-CO-*` content pack, the renamed `critic-prose`, `intent lang init content` canon, the `/in-content-essentials` skill, and `content -> critic-prose` dispatch -- strictly opt-in, with zero behaviour change until a project runs `intent lang init content`; hv accepts the dogfood (WP-06). -- evidence: WP-06 dogfood + hv sign-off -- satisfied: no

### WP-01 -- IN-PR-* prose base + author refactor (status: DONE)

- AC-01.1 The `PR` (prose) language code is admitted by the rule-id validator (a well-formed `IN-PR-*` id validates; a malformed one still fails) and `prose` is registered in the canon-enumeration `LANG_SUBDIRS`.
- AC-01.2 The four shared mechanical rules live once, in the prose base (`rules/prose/style/`): each is schema-valid, declares `language: prose`, is enumerated under `--lang prose`, and carries its `IN-AU-STYLE-*` migration alias.
- AC-01.3 The author pack retains only its discipline-specific rules -- one style rule (front-matter/objectives) plus the four craft rules (5 total) -- each still schema-valid and enumerated under `--lang author`; no moved rule remains under `rules/author/`.
- AC-01.4 (non-test) The prose base carries no upward dependency on an author-specific rule (no base->discipline `related_rules`), and every downstream reference (author craft cross-links, the `/in-author-essentials` skill, the author canon templates) re-points to the `IN-PR-*` ids. -- evidence: heading-hygiene + mechanical-trope-pass `related_rules` emptied; author full-trope-diagnosis links `IN-PR-STYLE-004`; skill + `templates/author/{RULES,ARCHITECTURE}.md` cite `IN-PR-*`; full-library `rules validate` 58/58 green -- satisfied: yes

### WP-02 -- content (CO) rule tiers (status: DONE)

- AC-02.1 The `CO` (content) language code is admitted by the validator and `content` is registered in `LANG_SUBDIRS`.
- AC-02.2 The content pack owns six web-distinct rules -- `style` (mechanical): page meta present (title/description/canonical), image alt-text, descriptive link text; `craft` (judgment): scannability, primary CTA, reading-level target -- each schema-valid and enumerated under `--lang content`. The generic prose hygiene (filler, vanity metrics, headings, trope pass) is the shared `IN-PR-*` base, not re-authored here.
- AC-02.3 (non-test) No `IN-CO-*` rule duplicates an `IN-PR-*` mechanical rule (Highlander). -- evidence: rule_pack_content.bats asserts no content dir reuses a prose-base slug and no content rule declares an `IN-PR-*` id; the content style tier is web-specific (meta/alt/links), disjoint from the base -- satisfied: yes

### WP-03 -- critic-prose (rename critic-author) (status: DONE)

- AC-03.1 `critic-author` is renamed to `critic-prose`: the subagent directory, `agent.md`, metadata, and the agent manifest all use `critic-prose`; no `critic-author` remains (dir gone, no manifest entry).
- AC-03.2 `critic-prose` loads the `IN-PR-*` base plus the declared discipline's rules, parameterised by language (author or content, resolved from config `languages`); its `agent.md` cites the `IN-PR-STYLE-004` mechanical pass and preserves the two-form detrope (mechanical default + `IN-AU-CRAFT-003` `/in-detrope` handoff, never invoked).

### WP-04 -- content lang canon (status: DONE)

- AC-04.1 `intent lang init content` installs `intent/llm/RULES-content.md` + `ARCHITECTURE-content.md`, appends the Language Packs entry, and writes `content` to config `languages`; `intent lang list` enumerates `content`. (Allowlist-free: `templates/content/{RULES,ARCHITECTURE}.md` is all that was needed -- `lang init`/`lang list` are dir-driven.)

### WP-05 -- in-content-essentials skill + dispatch (status: DONE)

- AC-05.1 The `/in-content-essentials` skill exists, is renderer-safe (no em dash, no `$N`), carries the content pipeline, and references the prose base + content rule ids.
- AC-05.2 `content -> critic-prose` is wired into `/in-review` (dispatch map + the generalised "Prose projects" note + Task example) and `/in-session` (fan-out table row + `chains_to`); a prose-only project (`[author]` or `[content]`) runs no code critic, a mixed project runs each critic on its own subtree, and a project declaring both applies both discipline packs.

### WP-06 -- dogfood + docs + close (status: not started)

- AC-06.1 (non-test) A real content target is dogfooded end-to-end (the pack applied via `intent lang init content` + a `critic-prose` pass) and hv signs off. -- evidence: [dogfood note + hv sign-off] -- satisfied: no
- AC-06.2 (non-test) The 2.16.0 release docs are updated: CHANGELOG, `intent/history/v2.16.0.md`, `docs/releases/2.16.0/`. -- evidence: [release docs] -- satisfied: no

## Acceptance Tests

### WP-01

- AT-01.1 tests/unit/pr_language_code_guard.bats -- covers AC-01.1 -- status: green
- AT-01.2 tests/unit/rule_pack_prose.bats -- covers AC-01.2 -- status: green
- AT-01.3 tests/unit/rule_pack_author.bats -- covers AC-01.3 -- status: green
- AT-01.4 tests/unit/in_author_essentials_skill.bats "references the prose base + author rules by ID" -- covers AC-01.4 -- status: green
- Coverage: AC-01.1..01.3 test-backed (green); AC-01.4 is non-test (evidence satisfied) and additionally backstopped by AT-01.4.

### WP-02

- AT-02.1 tests/unit/co_language_code_guard.bats -- covers AC-02.1 -- status: green
- AT-02.2 tests/unit/rule_pack_content.bats -- covers AC-02.2, AC-02.3 -- status: green
- Coverage: AC-02.1, AC-02.2 test-backed (green); AC-02.3 non-test (evidence satisfied), backstopped by the Highlander tests in AT-02.2.

### WP-03

- AT-03.1 tests/unit/critic_prose.bats -- covers AC-03.1, AC-03.2 -- status: green
- Coverage: AC-03.1, AC-03.2 test-backed (green).

### WP-04

- AT-04.1 tests/unit/intent_lang.bats (content cases: list enumerates content; init installs both files, appends the pack entry, writes config languages) -- covers AC-04.1 -- status: green
- Coverage: AC-04.1 test-backed (green).

### WP-05

- AT-05.1 tests/unit/in_content_essentials_skill.bats -- covers AC-05.1 -- status: green
- AT-05.2 tests/unit/critic_dispatch.bats + tests/unit/in_session_skill.bats (content cases) -- covers AC-05.2 -- status: green
- Coverage: AC-05.1, AC-05.2 test-backed (green).

### WP-06

- Coverage: AC-06.1, AC-06.2 non-test (dogfood evidence + release docs); AC-00.1 ST-level, gated on WP-06.
