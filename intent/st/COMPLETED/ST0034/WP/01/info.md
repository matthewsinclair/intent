---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-01
title: "Architecture and rule schema"
scope: Medium
status: Done
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
- [ ] `archetype/strong-assertions/good_test.exs` exits 0 under `elixir <path>`
- [ ] `archetype/strong-assertions/bad_test.exs` exits 0 under `elixir <path>` (exit-code contract: both examples pass; Critic — not ExUnit — is the enforcer)
- [ ] `_attribution/elixir-test-critic.md` exists with verbatim MIT notice and pinned commit `1d9aa40700dab7370b4abd338ce11b922e914b14`
- [ ] `rules/index.json.template` exists with the planned JSON shape
- [ ] `rules/_schema/CI-LIMITATIONS.md` documents the non-Elixir runnable-example exclusion
- [ ] `intent/llm/MODULES.md` lists every planned module for ST0034 (rule library, rule validator, critic family, ext discovery, all rule packs)
- [ ] No CLI code, no subagent files, no migration code in this WP — design only
- [ ] `./tests/run_tests.sh` exits 0 (pristine invariant — WP01 adds no runtime changes, so baseline holds)
- [ ] User confirms schema before WP02 begins

### Tests to add

None. WP01 is design-only; all test additions land in downstream WPs per `design.md` §Testing Strategy.

### Tests to update

None. The existing 469-test baseline stays green.

## Dependencies

None. This WP is the foundation for everything else.

## Implementation Notes

- **Upstream reference commit** pinned: `1d9aa40700dab7370b4abd338ce11b922e914b14` (captured 2026-04-22 at WP01 start). Recorded in `_attribution/elixir-test-critic.md` and `attribution-policy.md`. WP05 uses this same hash for rule porting.
- **Schema extension fields** are strictly optional: upstream tools must still be able to consume Intent rules, which means unknown fields are ignored. Keep `upstream_id`, `references`, `aliases` as scalars or simple arrays (no nested maps).
- **Archetype choice**: IN-EX-TEST-001 (strong assertions) is Intent-original (no upstream counterpart — upstream's `test-shape-not-values` is telemetry-scoped, not general). The archetype demonstrates the full frontmatter and the nine-section body shape. A second exemplar exercising `upstream_id:` (e.g. IN-EX-TEST-002 no-process-sleep) is deferred to WP05; the schema document itself cites real upstream slugs where applicable.
- **Upstream convention for section headings**: `## When This Applies` / `## When This Does Not Apply` (verified against multiple upstream RULE.md files). Intent adopts verbatim for compatibility. Frontmatter keys (`applies_when:`, `does_not_apply_when:`) keep Intent's internal names.
- **Runnable-example contract**: both `good_test.exs` and `bad_test.exs` exit 0 when run (upstream convention). Canonical invocation is `elixir <path>` standalone; `mix test <path>` works inside a Mix project. Enforcement of the antipattern is the Critic's job, not ExUnit's.
- **Critic contract sample report** includes all four severity levels with at least one finding each to make the format unambiguous for WP07.
- **Bash 3.x constraint** for index.json consumers: the generator must handle flat fields with `jq`. Documentation says "all top-level frontmatter fields must be scalars or flat arrays" to prevent future schema drift.
- **MODULES.md discipline**: register every planned module path, even those not yet existing. This prevents Highlander violations during WP02-WP11 when new files are created.

## Risks and Edge Cases

- **Schema over-engineering**: risk of adding speculative fields. Every field in the schema must have a named consumer (Claude, bash, a validator, an LLM). If a field has no consumer in v2.9.0, it goes in a "future considerations" appendix, not the schema.
- **Upstream schema drift during WP01**: unlikely at one week's duration, but if upstream pushes a schema change during this WP, re-pin the commit hash and note the deviation.
- **Archetype ambiguity**: if `good.exs` / `bad.exs` don't clearly map to the rule's Detection heuristic, the rule is underspecified. Rewrite Detection before freezing archetype.
- **MODULES.md registration of not-yet-existing modules** can feel like cheating, but it's the correct Highlander discipline: plan the namespace before populating it.

## Testing Approach

- No BATS additions in this WP (no code to test); pristine invariant holds trivially.
- Archetype validation: run `elixir <path>/good_test.exs` and `elixir <path>/bad_test.exs` standalone; both must exit 0.
- Schema validation: hand-check archetype frontmatter against `rule-schema.md` field by field.
- Cross-reference with upstream: fetch `rules/core/no-process-sleep/RULE.md` at the pinned commit and verify its frontmatter parses correctly against Intent's schema (schema compatibility smoke test).
- Baseline run of `./tests/run_tests.sh` before WP01 commit, and again at WP01 close — both must report 469 passing (or 469 + any pre-existing additions).

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: upstream schema research + draft of `rule-schema.md`, `id-scheme.md`, `attribution-policy.md`. Archetype RULE.md.
- **Session 2**: archetype `good.exs` / `bad.exs` validated; Critic contract spec; CI-LIMITATIONS; MODULES.md registrations.
- **Session 3** (if needed): user review iteration, schema refinements.

## Exit Checklist

Before closing WP01:

- [x] All acceptance criteria met
- [x] User-approved schema (explicit confirmation)
- [x] No TODOs in `_schema/` files
- [x] `_attribution/elixir-test-critic.md` committed with pinned-commit SHA verified against upstream
- [x] MODULES.md committed with all planned registrations
- [x] Both archetype `.exs` files exit 0 under `elixir <path>` (exit-code contract)
- [x] `./tests/run_tests.sh` exits 0 with ≥469 passing
- [x] Upstream commit hash recorded in `attribution-policy.md`

## As-Built

Closed at commit `3625b18` (ST0034:01 Done).

### Shipped artefacts

| Path                                                                            | Purpose                                                                           |
| ------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `intent/plugins/claude/rules/_schema/rule-schema.md`                            | Frontmatter reference + 9 mandatory sections + 2-space indentation invariant      |
| `intent/plugins/claude/rules/_schema/id-scheme.md`                              | `IN-<LANG>-<CAT>-<NNN>` with real upstream-slug cross-reference table             |
| `intent/plugins/claude/rules/_schema/attribution-policy.md`                     | Tier 1 / 2 / 3 attribution rules + slug-existence verification snippet            |
| `intent/plugins/claude/rules/_schema/index-generator.md`                        | Full pipeline spec for `intent claude rules index` (deterministic, skip-tolerant) |
| `intent/plugins/claude/rules/_schema/archetype/strong-assertions/RULE.md`       | Exemplar rule (Intent-original IN-EX-TEST-001, no `upstream_id`)                  |
| `intent/plugins/claude/rules/_schema/archetype/strong-assertions/good_test.exs` | Pattern-match assertion; exits 0 under `elixir <path>`                            |
| `intent/plugins/claude/rules/_schema/archetype/strong-assertions/bad_test.exs`  | Shape-only antipattern; exits 0 (Critic is enforcer, not ExUnit)                  |
| `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`                | MIT text + pinned commit `1d9aa40700dab7370b4abd338ce11b922e914b14` (2026-04-22)  |
| `intent/plugins/claude/rules/index.json.template`                               | Target shape with 3 sample entries; validated by jq                               |
| `intent/st/ST0034/design.md`                                                    | Testing Strategy section with 469-test pristine invariant + per-WP test surfaces  |

### Design shifts vs original plan

- **`bad_test.exs` exit contract reversed**: original plan said it should fail; upstream convention is both files exit 0 with Critic as enforcer. Acceptance criterion updated.
- **Archetype made Intent-original**: originally I tried to cite upstream's `test-shape-not-values`, but that slug is telemetry-scoped. IN-EX-TEST-001 ships without `upstream_id` — the cleanest Tier-2 exemplar for WP05 will be `no-process-sleep` instead.
- **Upstream heading wording**: rule schema now mandates `## When This Applies` / `## When This Does Not Apply` verbatim (upstream form, was `## When It Applies`).
- **2-space indentation everywhere**: explicit Intent-wide rule recorded in `rule-schema.md` Formatting Invariants section; overrides Rust/Swift/Lua/Python language defaults for content under `intent/`.
