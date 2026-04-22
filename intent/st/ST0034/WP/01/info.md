---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-01
title: "Architecture and rule schema"
scope: Medium
status: Not Started
---

# WP-01: Architecture and rule schema

## Objective

Lock the foundational contracts for ST0034 before any downstream work begins: the rule file schema (frontmatter + sections), the rule ID scheme, the rules directory layout, the attribution policy, and the Critic subagent contract. Produce a reference archetype rule that downstream WPs can use as a template.

## Context

Every subsequent WP depends on the schema and layout decided here. WP02's validator, WP04-06's rule packs, WP07's Critic subagents, and WP10's docs all consume these contracts. This WP is pure design-in-files: no CLI code, no critic subagents, no migrations. It ends with enough on disk that WP02 can start implementing.

elixir-test-critic's schema is the compatibility target. Intent adopts upstream's frontmatter shape and mandatory Markdown sections verbatim, then adds three optional fields (`upstream_id`, `references`, `aliases`). This preserves future merge paths and lets users drop elixir-test-critic rules into Intent without reformatting.

## Deliverables

### Schema documentation (`intent/plugins/claude/rules/_schema/`)

- `rule-schema.md` — frontmatter field reference, severity tiers, required sections
- `id-scheme.md` — `IN-<LANG>-<CAT>-<NNN>` format, renumbering prohibition, alias rules
- `attribution-policy.md` — when to set `upstream_id:`, when to add MIT notices, provenance discipline
- `critic-contract.md` — Critic subagent input/output contract (moves to `intent/docs/critics.md` when WP10 lands; draft here first)
- `CI-LIMITATIONS.md` — documents that Rust/Swift/Lua examples are textual-only in v2.9.0

### Reference archetype (`intent/plugins/claude/rules/_schema/archetype/`)

- `strong-assertions/RULE.md` — fully populated exemplar rule (Elixir test category)
- `strong-assertions/good.exs` — runnable: `mix test` passes
- `strong-assertions/bad.exs` — runnable: `mix test` fails with the expected rule violation

### Index schema

- `intent/plugins/claude/rules/index.json.template` — machine-readable catalogue schema. This is NOT the generated index (that comes from `intent claude rules index` in WP02); this is the shape-definition only.

### Index generator design (spec only; implementation in WP02)

- `rules/_schema/index-generator.md` — documents the shell + jq pipeline that will produce `index.json` from RULE.md frontmatter

### MODULES.md registrations (Register-before-you-code)

Entries added to `intent/llm/MODULES.md` for:

- Rule library (directory: `intent/plugins/claude/rules/`)
- Rule validator (planned module: `intent/plugins/claude/bin/intent_claude_rules`, delivered in WP02)
- Rule-pack directories (agnostic + elixir + rust + swift + lua, delivered in WP04-06)
- Critic subagent family (planned subagents, delivered in WP07)
- Extension discovery (planned module: `bin/intent_ext`, delivered in WP02)

## Approach

1. **Fetch the upstream schema.** Read 3-5 representative elixir-test-critic rules (different categories: core, test, mock) and extract the exact frontmatter and section structure.
2. **Draft Intent's extended schema.** Compare upstream to what Intent needs. Add `upstream_id`, `references`, `aliases`. Write `rule-schema.md` as the authoritative reference.
3. **Write the archetype.** Pick IN-EX-TEST-001 (strong assertions) as the exemplar. Populate all sections. Author runnable `good.exs` and `bad.exs`.
4. **Verify the archetype runs.** `cd` into a throwaway Elixir sandbox; run `mix test good.exs` (must pass) and `mix test bad.exs` (must fail with the rule's expected violation).
5. **Write the Critic contract.** Spell out invocation format, rule-loading order, mode dispatch, report format. Include a sample report output so WP07 has a template.
6. **Write the attribution policy.** When MIT notice is required; how `upstream_id` interacts with attribution; one canonical example.
7. **Document CI limitations.** Rust/Swift/Lua examples are textual-only; critic validates them against real code, not runnable snippets.
8. **Register in MODULES.md.** Add every new module concern even though the code lands in later WPs (Intent's discipline).
9. **User review gate.** Before WP02 starts, confirm schema + archetype with user. This is the contract; changing it later is expensive.

## Acceptance Criteria

- [ ] `rules/_schema/rule-schema.md` documents every frontmatter field with type and usage
- [ ] `rules/_schema/id-scheme.md` defines the format and states the renumbering prohibition
- [ ] `rules/_schema/attribution-policy.md` specifies when `upstream_id:` must be used and the MIT notice structure
- [ ] `rules/_schema/critic-contract.md` specifies Critic input, rule-loading order, mode semantics, and report format with at least one sample report
- [ ] `rules/_schema/archetype/strong-assertions/RULE.md` exists and conforms to the documented schema
- [ ] `archetype/strong-assertions/good.exs` passes `mix test`
- [ ] `archetype/strong-assertions/bad.exs` fails `mix test` with the documented violation
- [ ] `rules/index.json.template` exists with the planned JSON shape
- [ ] `rules/_schema/CI-LIMITATIONS.md` documents the non-Elixir runnable-example exclusion
- [ ] `intent/llm/MODULES.md` lists every planned module for ST0034 (rule library, rule validator, critic family, ext discovery, all rule packs)
- [ ] No CLI code, no subagent files, no migration code in this WP — design only
- [ ] User confirms schema before WP02 begins

## Dependencies

None. This WP is the foundation for everything else.

## Implementation Notes

- **Upstream reference commit**: capture the elixir-test-critic HEAD commit hash at WP01 start and record it in `attribution-policy.md`. WP05 uses this same hash for rule porting. This pins Intent to a known upstream state.
- **Schema extension fields** are strictly optional: upstream tools must still be able to consume Intent rules, which means unknown fields are ignored. Keep `upstream_id`, `references`, `aliases` as scalars or simple arrays (no nested maps).
- **Archetype choice**: IN-EX-TEST-001 (strong assertions) is a good archetype because it maps cleanly to an upstream rule (`test-critic-strong-assertions`), exercises the `upstream_id` field, has a simple runnable example, and applies to a common Elixir testing antipattern.
- **Critic contract sample report** should include all severity levels with at least one finding each to make the format unambiguous for WP07.
- **Bash 3.x constraint** for index.json consumers: the generator must handle flat fields with `jq`. Documentation should say "all top-level frontmatter fields must be scalars or flat arrays" to prevent future schema drift.
- **MODULES.md discipline**: register every planned module path, even those not yet existing. This prevents Highlander violations during WP02-WP11 when new files are created.

## Risks and Edge Cases

- **Schema over-engineering**: risk of adding speculative fields. Every field in the schema must have a named consumer (Claude, bash, a validator, an LLM). If a field has no consumer in v2.9.0, it goes in a "future considerations" appendix, not the schema.
- **Upstream schema drift during WP01**: unlikely at one week's duration, but if upstream pushes a schema change during this WP, re-pin the commit hash and note the deviation.
- **Archetype ambiguity**: if `good.exs` / `bad.exs` don't clearly map to the rule's Detection heuristic, the rule is underspecified. Rewrite Detection before freezing archetype.
- **MODULES.md registration of not-yet-existing modules** can feel like cheating, but it's the correct Highlander discipline: plan the namespace before populating it.

## Testing Approach

- No BATS tests in this WP (no code to test).
- Archetype validation: manually run `mix test` on `good.exs` and `bad.exs` in a local Elixir project.
- Schema validation: hand-check archetype frontmatter against `rule-schema.md` field by field.
- Cross-reference with upstream: pick one real elixir-test-critic rule and verify its frontmatter would parse correctly against Intent's schema (schema compatibility smoke test).

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: upstream schema research + draft of `rule-schema.md`, `id-scheme.md`, `attribution-policy.md`. Archetype RULE.md.
- **Session 2**: archetype `good.exs` / `bad.exs` validated; Critic contract spec; CI-LIMITATIONS; MODULES.md registrations.
- **Session 3** (if needed): user review iteration, schema refinements.

## Exit Checklist

Before closing WP01:

- [ ] All acceptance criteria met
- [ ] User-approved schema (explicit confirmation)
- [ ] No TODOs in `_schema/` files
- [ ] MODULES.md committed with all planned registrations
- [ ] Archetype runnable-examples pass/fail as documented
- [ ] Upstream commit hash recorded in `attribution-policy.md`
