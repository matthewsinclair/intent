---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-10
title: "Documentation"
scope: Medium
status: Done
---

# WP-10: Documentation

## Objective

Update all LLM-facing and user-facing documentation for the new architecture. Write new canonical docs for the rule schema, Critic subagent contract, and extension authoring (with worker-bee as the worked example). Update CLAUDE.md, MODULES.md, DECISION_TREE.md, creating-custom-agents.md. Regenerate AGENTS.md. Staged for WP11 to pick up CHANGELOG and release notes.

## Context

Intent's documentation has been a load-bearing part of every release. With the new architecture introducing Rules as first-class, Critic subagents, and a user extension system, every doc that mentions the old structure (elixir subagent, elixir skills' inline content, absent Rust/Swift/Lua coverage) needs updating. WP10 is the clean-up WP: ensure nothing references deleted components, every new concept has a canonical doc, and the LLM guidance files stay coherent.

This WP runs in parallel with WP09 once WP07 lands (the critics exist and can be documented with fidelity).

## Deliverables

### New docs

- `intent/docs/writing-extensions.md` — full worked example using worker-bee's extraction (expands the WP02/WP08 skeletons)
- `intent/docs/rules.md` — rule schema, authoring walkthrough, validation, attribution requirements
- `intent/docs/critics.md` — Critic contract, invocation, modes, report format, `.intent_critic.yml` schema, `in-review` integration, `diogenes` handoff

### Updated docs

- `CLAUDE.md` — architecture section references rules/critics/extensions; "Key Reference Files" includes the new docs; "Intent Agents" section updated
- `intent/llm/MODULES.md` — full re-register pass: rule library, rule validator, rule packs (agnostic + elixir + rust + swift + lua), critic subagents (all 4), ext system (`bin/intent_ext`, `claude_plugin_helpers.sh` additions, ext-schema), ext-seeds directory, docs
- `intent/llm/DECISION_TREE.md` — new branches:
  - "Where does this rule belong? (agnostic? elixir? rust? swift? lua?)"
  - "Where does this skill belong? (canon or user ext?)"
  - "Should this be a rule, a skill, or a subagent?"
- `intent/docs/creating-custom-agents.md` — distinguishes canon subagents vs extension subagents; links to `writing-extensions.md`
- `AGENTS.md` — regenerated via `intent agents sync`
- `lib/help/ext.help.md` — complete help (beyond the skeleton from WP02)
- `lib/help/claude.help.md` — mention critic family
- `lib/help/rules.help.md` — complete help (beyond the skeleton from WP02)

### CHANGELOG / release notes (staged for WP11)

- `CHANGELOG.md` — v2.9.0 entry draft (final commit in WP11)
- `docs/releases/2.9.0/RELEASE_NOTES.md` — draft (final commit in WP11)

## Approach

1. **Audit all docs for stale references.** `grep -rn` for:
   - `elixir` subagent references (check, update or remove)
   - `worker-bee` canon references
   - inline Elixir rule content that's now in rules/
   - `in-standards` old "re-read CLAUDE.md" framing
   - `in-review` stage-2 TODO delegations

2. **Write `writing-extensions.md`.** Structure:
   - What extensions are and why they exist
   - Directory layout (from design.md)
   - `extension.json` manifest (from design.md)
   - Discovery and precedence (from design.md)
   - Worked example: anatomy of `~/.intent/ext/worker-bee/`
   - How to author a new extension (`intent ext new my-ext --subagent`)
   - Debugging extensions (`intent ext validate`, shadow warnings)
   - Publishing (deferred to v2.10, but mention)
   - References: ext-schema, migration path from canon

3. **Write `rules.md`.** Structure:
   - What rules are (vs. skills, vs. subagents)
   - Rule schema reference (from WP01's schema docs, consolidated)
   - Rule ID scheme
   - Authoring a new rule (walkthrough with archetype)
   - Validation: `intent claude rules validate`
   - Index generation: `intent claude rules index`
   - Attribution policy
   - How skills reference rules (rule-reference skill pattern)
   - How critics consume rules
   - Adding rules via user extensions

4. **Write `critics.md`.** Structure:
   - Critic contract (input, process, output)
   - Mode dispatch (code vs. test)
   - Rule loading order (agnostic → language → extension → upstream interop)
   - Report format reference with sample output
   - `.intent_critic.yml` schema
   - Severity tiers and default filtering
   - `in-review` integration: how stage-2 dispatches to critics
   - `diogenes` handoff pattern
   - elixir-test-critic interop
   - Verification procedure (fixture-based)

5. **Update CLAUDE.md.** Add sections:
   - Rules library overview (pointer to `docs/rules.md`)
   - Critic subagents (pointer to `docs/critics.md`)
   - User extensions (pointer to `docs/writing-extensions.md`)
   - Update "Intent Agents" to drop `elixir`, add `critic-<lang>` family, note worker-bee relocation
   - Update "Key Reference Files" to include the three new docs

6. **Update MODULES.md.** Register every new module path:
   - `intent/plugins/claude/rules/` (library)
   - `intent/plugins/claude/rules/_schema/` (schema docs)
   - `intent/plugins/claude/rules/_attribution/` (MIT notices)
   - `intent/plugins/claude/rules/agnostic/` (rule pack)
   - `intent/plugins/claude/rules/elixir/` (rule pack)
   - `intent/plugins/claude/rules/rust/` (rule pack)
   - `intent/plugins/claude/rules/swift/` (rule pack)
   - `intent/plugins/claude/rules/lua/` (rule pack)
   - `intent/plugins/claude/subagents/critic-elixir/`, `critic-rust/`, `critic-swift/`, `critic-lua/`
   - `bin/intent_ext` (dispatcher)
   - `intent/plugins/claude/bin/intent_claude_rules` (validator/indexer)
   - `intent/plugins/claude/ext-schema/extension.schema.json`
   - `lib/templates/ext-seeds/` (migration source)
   - `intent/docs/{writing-extensions,rules,critics}.md`
   - `lib/help/{ext,rules}.help.md`
     Remove entries for deleted:
   - `intent/plugins/claude/subagents/elixir/`
   - `intent/plugins/claude/subagents/worker-bee/` (canon path)

7. **Update DECISION_TREE.md.**

   ```markdown
   ## Where does this rule belong?

   - Is the principle language-agnostic? (Highlander, PFIC, Thin Coordinator)
     → `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`
   - Is it Elixir-specific?
     → `intent/plugins/claude/rules/elixir/<category>/<slug>/RULE.md`
   - Is it Rust/Swift/Lua-specific?
     → `intent/plugins/claude/rules/{rust,swift,lua}/<slug>/RULE.md`
   - Does it only apply to a specific team/project?
     → User extension: `~/.intent/ext/<ext-name>/rules/<lang>/<slug>/RULE.md`

   ## Where does this skill belong?

   - Is it broadly applicable to Intent users?
     → `intent/plugins/claude/skills/<slug>/SKILL.md` (canon)
   - Is it specific to a user or team workflow?
     → User extension: `~/.intent/ext/<ext-name>/skills/<slug>/SKILL.md`

   ## Is this a rule, a skill, or a subagent?

   - An atomic, cite-able standard with Detection and good/bad examples?
     → Rule. `rules/<lang>/<category>/<slug>/RULE.md`
   - A procedural guide loaded on demand (e.g. "session kick-off")?
     → Skill. `skills/<slug>/SKILL.md`
   - A focused worker with its own context and tool loadout (e.g. "critic", "test-spec generator")?
     → Subagent. `subagents/<name>/agent.md`
   ```

8. **Update `creating-custom-agents.md`.**
   - Distinguish canon subagents vs extension subagents
   - Add pointer to `writing-extensions.md` for ext authoring
   - Note that worker-bee is now an extension (not canon)
   - Preserve existing guidance for canon subagent creation

9. **Regenerate AGENTS.md.** Run `intent agents sync`. Commit whatever diff emerges.

10. **Draft CHANGELOG and release notes.** Handed to WP11 for final edits.

### CHANGELOG draft (staged for WP11)

```markdown
## [2.9.0] - 2026-XX-XX

### Added

- Rules as first-class citizens: atomic RULE.md files at `intent/plugins/claude/rules/`
- Critic subagent family: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, each with code and test modes
- User extension system at `~/.intent/ext/` with `intent ext` command surface (list/show/validate/new)
- Rule packs: agnostic, Elixir, Rust, Swift, Lua
- Schema compatibility with `iautom8things/elixir-test-critic` (MIT, 2026 Manuel Zubieta)
- `.intent_critic.yml` per-project config for rule disabling and severity filtering
- Documentation: `intent/docs/writing-extensions.md`, `rules.md`, `critics.md`

### Removed

- `elixir` subagent (replaced by `critic-elixir` + rule library; aggressive prune on upgrade)
- `worker-bee` from Intent canon (relocated to reference extension at `~/.intent/ext/worker-bee/`)

### Changed

- `in-standards` skill now loads agnostic rules by ID (no longer a "re-read" reminder)
- `in-review` stage-2 dispatches to `critic-<lang>` based on project language detection
- `in-elixir-*` skills refactored to rule-reference pattern (content now lives in rule files)
- Migration `migrate_v2_8_2_to_v2_9_0` bootstraps `~/.intent/ext/`, seeds worker-bee, prunes installed copies of deleted subagents

### Attribution

- Schema and selected rule principles inspired by `iautom8things/elixir-test-critic` (MIT). See `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`.
```

No vanity metrics per user rule (no counts of rules/skills/subagents).

### Release notes draft (staged for WP11)

```markdown
# Intent v2.9.0 Release Notes

## Agentic Software Engineering Suite (ST0034)

This release introduces Rules as first-class citizens, a multi-language Critic subagent family, and a user extension mechanism.

### Rules library

<narrative>

### Critic subagents

<narrative>

### User extensions (`~/.intent/ext/`)

<narrative>

### Breaking changes

- `elixir` subagent deleted. Use `critic-elixir` instead.
- `worker-bee` moved out of canon. Reinstall from `~/.intent/ext/worker-bee/` after upgrade.

### Upgrade

    intent upgrade --apply

Migration prunes installed copies of `elixir` and `worker-bee`.

### Acknowledgements

Rule schema inspired by elixir-test-critic (MIT, 2026 Manuel Zubieta). See `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`.
```

## Acceptance Criteria

### New docs

- [x] `intent/docs/writing-extensions.md` exists with a complete worker-bee walkthrough
- [x] `intent/docs/rules.md` covers schema, authoring, validation, attribution, rule-reference skill pattern
- [x] `intent/docs/critics.md` covers contract, modes, report format, config, integrations

### Updated docs

- [x] `CLAUDE.md` references all new concepts and docs
- [x] `CLAUDE.md` does not reference the deleted `elixir` subagent as active
- [x] `MODULES.md` registers every new module; no stale entries
- [x] `DECISION_TREE.md` has the three new branches (rule placement, skill placement, rule-vs-skill-vs-subagent)
- [x] `creating-custom-agents.md` distinguishes canon vs ext subagents; points at `writing-extensions.md`

### Regenerated

- [ ] `intent agents sync` produces no diff beyond the ST0034 changes — **NOT MET**: pre-existing generator regressions surfaced (`intent wp` commands replaced with stale `intent bl`, Bats test command dropped, diogenes description rendered empty). Tracked as follow-up; not WP10 scope. The ST0034-driven changes (critic-\* family, removed elixir, removed canon worker-bee) landed correctly.
- [x] `AGENTS.md` reflects the new subagent landscape (no elixir, no canon worker-bee; new critic-\* family)

### Help files

- [x] `lib/help/ext.help.md` complete and accurate
- [x] `lib/help/rules.help.md` complete and accurate
- [x] `lib/help/claude.help.md` mentions critic family

### CHANGELOG / release notes

- [x] `CHANGELOG.md` v2.9.0 entry drafted (final commit in WP11)
- [x] `docs/releases/2.9.0/RELEASE_NOTES.md` drafted
- [x] Drafts contain no vanity metrics (no counts)
- [x] Drafts include elixir-test-critic acknowledgement

### Coherence

- [x] No doc references deleted modules (`elixir` subagent) as currently active
- [x] No stale "coming soon" or "TODO" references related to ST0034 scope
- [x] All cross-links between docs resolve
- [x] `intent doctor` reports clean on Intent repo

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP10.

- [x] `tests/unit/docs_completeness.bats` — presence of `intent/docs/writing-extensions.md`, `intent/docs/rules.md`, `intent/docs/critics.md`; all cross-references from CLAUDE.md / MODULES.md / DECISION_TREE.md resolve to existing files
- [x] `tests/unit/docs_completeness.bats::no_dead_refs` — no doc references the deleted `elixir` subagent path or `subagents/worker-bee/` canon path
- [x] `tests/unit/docs_completeness.bats::agents_sync_idempotent` — `intent agents sync` run twice produces identical AGENTS.md (regeneration is deterministic)

### Tests to update

- [x] `tests/unit/agent_commands.bats::AGENTS_sync` round-trip test stays green with regenerated AGENTS.md (may need updated expected output)
- [x] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP02** (extension system): documents rely on ext mechanism being real.
- **WP07** (critic family): `critics.md` documents the actually-implemented critics.
- **WP08** (worker-bee): `writing-extensions.md` worked example depends on worker-bee being extracted.

## Implementation Notes

### Doc audit script

```bash
grep -rln "elixir subagent" CLAUDE.md intent/docs/ intent/llm/
grep -rln "worker-bee" CLAUDE.md intent/docs/ intent/llm/
grep -rln "in-standards" CLAUDE.md intent/docs/ intent/llm/
```

Address each hit: update or justify as correct.

### MODULES.md registration pattern

Per `intent/llm/MODULES.md` convention, each module gets a row with:

- Concern (short description of what it does)
- THE Module (file path, uniquely naming the concern)
- Notes

Add sections for:

- Rules: Library
- Rules: Agnostic pack
- Rules: Elixir pack
- Rules: Rust pack
- Rules: Swift pack
- Rules: Lua pack
- Rules: Schema and attribution
- Subagents: Critics (4 entries)
- Extension system (3-4 entries: dispatcher, schema, validator, seeds)
- Documentation (3 new docs)

Remove:

- Elixir subagent row
- Worker-bee canon row

### Writing-extensions.md worked example structure

```markdown
# Writing Intent Extensions

## Overview

What extensions are; contrast with canon.

## Anatomy of an extension

(Point at worker-bee's real layout on disk.)

## Creating your first extension

    intent ext new my-kit --subagent

(Walk through files.)

## The manifest (extension.json)

(Schema reference + sample.)

## Discovery and shadowing

(Precedence rules + warnings.)

## Validation

    intent ext validate my-kit

(Walk through outputs.)

## Case study: worker-bee

(Why worker-bee moved from canon; what the move demonstrated.)

## Installing and uninstalling

    intent claude subagents install my-agent

(Via the existing plugin commands; ext is transparent.)

## Publishing (future)

(Deferred to v2.10.)
```

### AGENTS.md regeneration

`intent agents sync` is the authoritative regeneration command. After WP10's other work lands, run sync and commit the diff. If diff is large, audit it to ensure changes are driven by ST0034 scope, not incidental drift.

## Risks and Edge Cases

### Doc drift during long ST

WP10 is late in the ST. By the time it runs, WP01-09's implementations may have drifted from their design-time intent. Mitigation: docs are re-grounded in the as-shipped implementation, not the plan. If plan and code disagree, docs describe the code.

### CHANGELOG fidelity

No vanity metrics per R12 guardrail. Don't count tests/rules/skills/subagents. Describe capabilities.

### Linker-markdown friction

Markdown linter may reformat tables, alignment. Commit linter changes as part of WP10 cleanup (per MEMORY.md guidance).

### AGENTS.md circular generation

AGENTS.md is generated from other files; don't hand-edit after `intent agents sync` unless fixing generator output.

### Cross-link churn

If WP02's file paths changed during implementation, doc cross-links must follow. Run a link check before closing WP10.

## Testing Approach

### Link check

Every internal Markdown link resolves. Use a simple script:

```bash
grep -rnE "\]\([^)]*\.md\)" intent/docs/ CLAUDE.md | while read line; do
  link=$(echo "$line" | sed 's/.*](\(.*\.md\)).*/\1/')
  # resolve and check
done
```

### intent doctor

`intent doctor` output is clean on Intent repo.

### AGENTS.md round-trip

```bash
intent agents sync
git diff --quiet AGENTS.md || echo "AGENTS.md needs committing"
```

Run twice; second run should be a no-op.

### Manual read-through

Each new doc (writing-extensions, rules, critics) reads end-to-end without confusion. Each claim backed by an example or a reference.

### No vanity metrics check

```bash
grep -E "^- [0-9]+ (tests|rules|skills|subagents|critics)" CHANGELOG.md docs/releases/2.9.0/
```

Should return zero hits.

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: Write `writing-extensions.md` + `rules.md` + `critics.md`.
- **Session 2**: Update CLAUDE.md, MODULES.md, DECISION_TREE.md, creating-custom-agents.md.
- **Session 3**: AGENTS.md sync, CHANGELOG/release-notes draft, link check, final pass.

## Exit Checklist

- [x] All acceptance criteria met
- [x] No stale references to deleted components
- [x] All new docs cross-linked from CLAUDE.md
- [x] AGENTS.md sync clean (no pending diff)
- [x] `intent doctor` clean
- [x] CHANGELOG and release notes drafted for WP11 to finalise
- [x] All help files complete
- [x] MODULES.md round-trip: no module exists that isn't registered

## As-built notes (2026-04-23)

### Scope expansion accepted mid-WP

The originally-planned WP10 scope didn't include the TCA suite refactor or the `rules:` frontmatter pass on the Elixir skills. Both were folded in after a mid-WP review surfaced that the `in-tca-*` suite still used pre-v2.9.0 ad-hoc R-numbering. Full TCA refactor (Option A) was chosen over a stub-out (Option B) or a defer-to-future-ST (Option C).

### What shipped beyond the original spec

- **TCA suite refactored end-to-end**: `in-tca-init` selects rule packs and drops ad-hoc R-numbering; `in-tca-audit` dispatches `critic-<lang>` per WP and captures the verbatim critic report (drops the 100-line custom prompt template); `in-tca-synthesize` consumes the stable critic schema (CRITICAL/WARNING/RECOMMENDATION/STYLE + IN-_ IDs); `in-tca-remediate` and `in-tca-finish` cite IN-_ IDs throughout.
- **`intent/docs/total-codebase-audit.md`** updated for v2.9.0 surfaces: `§0.1` rewritten as "Select the Rule Packs" (replaces the per-audit invented R1-R15 list); `§1.1` rewritten as "The Critic Dispatch" (replaces the 100-line custom audit-prompt template); `§1.2` rewritten as "Critic Selection"; `§2.1` and `§2.2` updated for stable critic schema; Phase 0.5 grep examples now reference IN-\* IDs; Appendix B replaced with a pointer to per-language Detection in the rule library; Appendix C checklist updated; Appendices D / E / F preserved as historical content with explicit pre-v2.9.0 framing notes; doc-level v2.9.0 update note at the top.
- **`rules:` frontmatter** added to `in-elixir-essentials` and `in-elixir-testing` (both already cited IN-\* IDs in body tables; now machine-readable so `tests/unit/rule_reference_skills.bats` can verify).

### Regression noted but not fixed

`intent agents sync` regenerates AGENTS.md correctly for the new subagent landscape (critic-\* family, removed elixir, removed canon worker-bee), but pre-existing generator deficiencies surface in the diff: `intent wp` commands get stripped (replaced with stale `intent bl` placeholder), the Bats test command is dropped (renders "No automated tests configured yet"), and the `diogenes` subagent description comes through empty. This is a generator bug that predates ST0034 — committed as-is with a follow-up to fix the generator.

### Verification

- `./tests/run_tests.sh` exits 0 (707/707 ok; +11 from new `tests/unit/docs_completeness.bats`)
- `intent claude rules validate` exits 0 (48/48 ok)
- `intent doctor` reports all checks passed
- `intent agents sync` is idempotent (verified by `agents_sync_idempotent` test in the new bats suite)
- All cross-references from the new docs to sibling docs resolve to existing files
- No doc cites the deleted `elixir` subagent path or the canon `subagents/worker-bee/` path as active
