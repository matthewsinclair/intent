---
verblock: "09 Jul 2026:v0.1: matts - Initial version"
st_id: ST0054
title: "Update Intent for latest usage-rules.md format -- acceptance contract"
---

# ST0054 Update Intent for latest usage-rules.md format -- Acceptance

> Canonical acceptance contract for ST0054. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
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

none -- WP-distributed. This is a documentation-and-template ST; every AC is non-test (doc / eyeball).

### WP-01 -- Rewrite the usage_rules interop doc (status: pending)

- AC-01.1 (non-test) the `working-with-llms.md` interop section describes the v1.x config-driven model (the `:usage_rules` mix.exs key; the three delivery modes inline / link / skills) with NO pre-v1.0 claims (no zero-config auto-include, no arg-driven `usage_rules.sync <file> --link-to-folder`). -- evidence: working-with-llms.md 'usage_rules interop' section: v1.x config-driven model, no pre-v1.0 claims -- satisfied: yes
- AC-01.2 (non-test) the section distinguishes the two `usage-rules.md` artifacts (Intent project-contract vs library per-dep files) explicitly. -- evidence: working-with-llms.md interop section names the two usage-rules.md artifacts (Intent project-contract vs library per-dep) -- satisfied: yes
- AC-01.3 (non-test) the section documents the `.claude/skills` collision and states Intent's official coexistence policy (Intent projects stay Intent-native; library skill-gen left off). -- evidence: working-with-llms.md 'Coexistence policy' subsection: .claude/skills collision + Intent-native policy -- satisfied: yes

### WP-02 -- Topical sub-rule folders in /in-standards (status: pending)

- AC-02.1 (non-test) `/in-standards` Step 3 references topical `deps/*/usage-rules/*.md` sub-rule folders, not only the single `deps/*/usage-rules.md`. -- evidence: in-standards/SKILL.md Step 3 (+ peers in-elixir-essentials, in-ash-ecto-essentials) now reference topical deps/_/usage-rules/_.md sub-rule folders -- satisfied: yes

### WP-03 -- Template freshness + name-collision note (status: pending)

- AC-03.1 (non-test) `_usage-rules.md` no longer carries a staleable hard-coded Intent version, or the version is unambiguously refresh-on-upgrade. -- evidence: _usage-rules.md: staleable 'v[[INTENT_VERSION]]' removed; now 'This project uses Intent. Run intent doctor for the installed version' -- satisfied: yes
- AC-03.2 (non-test) `_usage-rules.md` notes the distinction between Intent's project-contract file and the library's per-dep `usage-rules.md`. -- evidence: _usage-rules.md new 2nd paragraph distinguishes this project-contract file from the library's per-dep usage-rules.md -- satisfied: yes

## Acceptance Tests

### WP-01

- AT-01.1 (doc) read the rewritten interop section against the v1.x model in design.md -- covers AC-01.1, AC-01.2, AC-01.3 -- status: n/a (doc / eyeball)
- Coverage: all WP-01 ACs are non-test (doc); evidence on each AC line.

### WP-02

- AT-02.1 (doc) read `/in-standards` Step 3 -- covers AC-02.1 -- status: n/a (doc / eyeball)
- Coverage: AC-02.1 non-test (doc).

### WP-03

- AT-03.1 (doc) read the `_usage-rules.md` template -- covers AC-03.1, AC-03.2 -- status: n/a (doc / eyeball)
- Coverage: all WP-03 ACs non-test (doc).
