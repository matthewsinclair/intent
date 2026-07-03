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

### WP-01 -- AU language-code schema bump (status: Not Started)

- AC-01.1 A well-formed author rule id (eg `IN-AU-STYLE-001`) passes the rule-id validator, and a malformed id (bad code, missing zero-padding) still fails.
- AC-01.2 (non-test) All four ID/validation sites carry `author`/`AU` consistently. -- evidence: grep of `rule-schema.md` enum + `id-scheme.md` codes/regex + `index-generator.md` regex + `intent_claude_rules` regex -- satisfied: no
- AC-01.3 (non-test) The widening is scoped to the ID/validation layer -- `bin/intent_critic` and the config/template layer are untouched by WP-01 (D4). -- evidence: `git diff` for WP-01 -- satisfied: no

### WP-02..WP-06

[ACs defined when each WP starts (five-step, red-first). Placeholder -- not yet ratified.]

## Acceptance Tests

### WP-01

- AT-01.1 tests/unit/au_language_code_guard.bats -- covers AC-01.1 -- status: to-write (red-first)
- Coverage: AC-01.1 by AT-01.1; AC-01.2 and AC-01.3 are non-test (evidence on the AC line).

### WP-02..WP-06

[ATs defined with their WP's ACs.]
