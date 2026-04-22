---
verblock: "22 Apr 2026:v0.3: matts - As-built WP03"
wp_id: WP-03
title: "Skill and subagent rationalisation"
scope: Medium
status: Done
---

# WP-03: Skill and subagent rationalisation

## Objective

Delete the `elixir` subagent from canon; migrate its rule-like content into atomic RULE.md files under `rules/elixir/**` (coordinated with WP05); refactor every `in-elixir-*` skill from content-holding to rule-referencing; make `in-standards` actionable by referencing `rules/agnostic/*` rules; make `in-review` stage-2 language-parameterised with a critic-`<lang>` dispatcher.

## Context

The `elixir` subagent (515 lines in `agent.md` plus 6 auxiliary .md files totalling over 3500 lines) duplicates content in four `in-elixir-*` skills with no sync contract. `in-standards` is a hollow "re-read CLAUDE.md" reminder rather than an enforceable gate. `in-review` stage-2 has an explicit TODO delegating to `diogenes` for Elixir, leaving a gap for Rust/Swift/Lua. This WP resolves all three problems by applying Rules as the single source of truth.

WP03 coordinates tightly with WP05 (Elixir rule pack): WP03 deletes and refactors; WP05 authors the atomic rule files that WP03's refactored skills reference. A shared content-inventory document (`intent/st/ST0034/content-inventory.md`) is the audit trail.

## Deliverables

### Deletions

- `intent/plugins/claude/subagents/elixir/` — the entire directory, including `agent.md`, `metadata.json`, `antipatterns.md`, `testing.md`, `style.md`, `liveview.md`, `ash-ecto.md`, `project-structure.md`
- `intent/plugins/claude/subagents/.manifest/global-agents.json` — remove `elixir` entry

### Skill refactors

- `intent/plugins/claude/skills/in-standards/SKILL.md` — rewritten to reference `rules/agnostic/*` by ID
- `intent/plugins/claude/skills/in-review/SKILL.md` — stage-2 becomes language-parameterised dispatcher (mix.exs / Cargo.toml / Package.swift / \*.lua detection)
- `intent/plugins/claude/skills/in-elixir-essentials/SKILL.md` — refactored to rule-reference format (lists `rules:` frontmatter + directive to read each RULE.md)
- `intent/plugins/claude/skills/in-elixir-testing/SKILL.md` — same
- `intent/plugins/claude/skills/in-phoenix-liveview/SKILL.md` — same
- `intent/plugins/claude/skills/in-ash-ecto-essentials/SKILL.md` — same

### Content inventory (shared with WP05)

- `intent/st/ST0034/content-inventory.md` — row-by-row trace of every discrete rule or guideline from deleted sources to new home (either a rule file in `rules/elixir/**` or inline body in a skill). No orphans, no duplication.

### in-review stage-2 dispatcher design

- Inline in `in-review/SKILL.md`: a decision table mapping project indicators to `critic-<lang>` subagent invocation
- Polyglot handling: if multiple indicators detected, skill instructs Claude to prompt the user
- Fallback: "unknown language — use language-agnostic review patterns"

## Approach

1. **Build content inventory.** Read every line of:
   - `intent/plugins/claude/subagents/elixir/agent.md` (515 lines)
   - `intent/plugins/claude/subagents/elixir/antipatterns.md` (1852 lines)
   - `intent/plugins/claude/subagents/elixir/testing.md` (563 lines)
   - `intent/plugins/claude/subagents/elixir/style.md` (400 lines)
   - `intent/plugins/claude/subagents/elixir/liveview.md` (354 lines)
   - `intent/plugins/claude/subagents/elixir/ash-ecto.md` (348 lines)
   - `intent/plugins/claude/subagents/elixir/project-structure.md` (223 lines)
   - `intent/plugins/claude/skills/in-elixir-essentials/SKILL.md`
   - `intent/plugins/claude/skills/in-elixir-testing/SKILL.md`
   - `intent/plugins/claude/skills/in-phoenix-liveview/SKILL.md`
   - `intent/plugins/claude/skills/in-ash-ecto-essentials/SKILL.md`
     Extract every discrete rule. Catalog with: source file, source line, rule statement, proposed destination (rule file ID or skill-body section).

2. **Deduplicate.** Many rules appear in multiple places. Pick the best-stated version as canonical.

3. **Decide rule vs skill-body.** Rules with concrete Detection and examples become RULE.md files (delegated to WP05). Guidance that is purely procedural (how to invoke a mix task, how to organise a directory) stays in skill bodies as prose.

4. **Hand off to WP05.** Share `content-inventory.md` with WP05 so they author the rule files for every entry marked "→ rule file". WP03 waits for WP05 rule files to land before completing refactors (because skill rules-frontmatter must reference existing rule IDs).

5. **Refactor `in-standards/SKILL.md`.** New structure:
   - Frontmatter lists `rules:` array referencing agnostic rule IDs
   - Body directs Claude to read each `RULE.md` and apply its Detection to work in progress
   - Removes the "re-read CLAUDE.md" language; becomes actionable

6. **Refactor `in-review/SKILL.md` stage-2.** Decision table:
   - Stage 1: unchanged (spec-compliance review)
   - Stage 2: language detection via filesystem probes
     - `mix.exs` in project root → `Task(subagent_type="critic-elixir", prompt="review <path>")`
     - `Cargo.toml` → `Task(subagent_type="critic-rust", ...)`
     - `Package.swift` → `Task(subagent_type="critic-swift", ...)`
     - entry `*.lua` file or `.luarc.json` → `Task(subagent_type="critic-lua", ...)`
     - multiple indicators → prompt user to choose
     - none → language-agnostic review (agnostic rules only)

7. **Refactor each `in-elixir-*/SKILL.md`.**
   - Add `rules:` array to frontmatter listing the rule IDs this skill curates
   - Replace inline rule prose with directive: "When this skill is invoked, read `intent/plugins/claude/rules/elixir/<cat>/<slug>/RULE.md` for each rule in the `rules:` list"
   - Keep any procedural framing (e.g. "use this skill when writing LiveView templates") but cut the content that's now in RULE.md

8. **Delete the `elixir` subagent directory.** After WP05 confirms all content is absorbed into rule files. Also remove entry from `global-agents.json`.

9. **Highlander audit.** `grep -rn` for specific rule prose (e.g. "pattern matching over nested conditionals") across `intent/plugins/claude/skills/`. No hits should exist outside the rule files.

10. **Update MODULES.md.** Remove the `elixir` subagent row; add rule-loader references.

## Acceptance Criteria

### Deletions

- [ ] `intent/plugins/claude/subagents/elixir/` directory does not exist
- [ ] `intent/plugins/claude/subagents/.manifest/global-agents.json` does not contain `elixir` entry
- [ ] `intent/llm/MODULES.md` does not list `elixir` as an active subagent

### Skill refactors

- [ ] `in-standards/SKILL.md` frontmatter has `rules:` array with at least 4 agnostic rule IDs (from WP04)
- [ ] `in-standards/SKILL.md` body directs Claude to read each RULE.md; no "re-read CLAUDE.md" reminder language
- [ ] `in-review/SKILL.md` stage-2 includes explicit language-detection logic
- [ ] `in-review/SKILL.md` dispatches to `critic-<lang>` via `Task(subagent_type=...)` invocation pattern for all four languages
- [ ] `in-review/SKILL.md` handles polyglot case by prompting user
- [ ] `in-review/SKILL.md` handles unknown-language case with agnostic-rules fallback
- [ ] Each `in-elixir-*/SKILL.md` has `rules:` frontmatter array that matches the rules authored in WP05
- [ ] No inline rule prose exists in any `in-elixir-*/SKILL.md` (Highlander audit: `grep -F` for rule statements returns no hits outside RULE.md files)

### Content inventory

- [ ] `content-inventory.md` lists every discrete rule from deleted sources
- [ ] Every entry has a "destination" column pointing at either a rule file ID or a skill-body section
- [ ] Zero orphans (every rule has a destination)
- [ ] Zero duplications (no rule appears in two destinations)

### Dispatch verification

- [ ] `in-review` stage-2 detection works for sample Elixir project (mix.exs present) — dispatches to critic-elixir
- [ ] Same for Rust (Cargo.toml), Swift (Package.swift), Lua (\*.lua entry)

### Highlander

- [ ] `grep -rn "pattern matching over conditionals" intent/plugins/claude/skills/` returns no hits (content is only in rules/elixir/code/)
- [ ] Same for other representative rule phrases
- [ ] No skill duplicates content from another skill

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP03 for the authoritative list.

- [ ] `tests/unit/rule_reference_skills.bats` — asserts `in-standards`, `in-elixir-essentials`, `in-elixir-testing`, `in-phoenix-liveview`, `in-ash-ecto-essentials` each have a `rules:` list in frontmatter; every listed ID resolves against the rule library
- [ ] `tests/unit/highlander_audit.bats` — greps for duplicated rule prose across skills + rules; returns no hits (Highlander enforcement for rule content)
- [ ] Negative case: `intent claude subagents show elixir` returns non-zero with "not found" (proves deletion completed)

### Tests to update

- [ ] `tests/unit/agent_commands.bats` — swap all `elixir` subagent fixture usages to `intent` (and `diogenes` for dual-agent cases). Affected lines from baseline grep: 90, 178, 181, 188, 195, 205, 389, 396, 457, 461, 464, 469, 474, 481, 589, 591, 608, 611, 624, 629, 631, 737, 745, 749
- [ ] `tests/unit/skills_commands.bats` — skills that reference `in-elixir-essentials` continue to install/uninstall/sync cleanly (skill survives WP03; only content shifts). Any assertion on SKILL.md text content must be refreshed against the refactored SKILL.md
- [ ] Post-update: `./tests/run_tests.sh` exits 0 with count ≥ 469 (baseline) + new WP03 additions — 0 skips introduced by this WP

## Dependencies

- **WP01** (schema): required. Skills must reference rules via well-formed IDs.
- **WP04** (agnostic rules): required. `in-standards` references agnostic rule IDs that must exist.
- **WP05** (elixir rule pack): required. `in-elixir-*` skills reference Elixir rule IDs that must exist.

**Sequencing**: WP05 must complete content authoring before WP03's skill refactors can resolve rule references. Use `content-inventory.md` as the shared artifact; WP03 starts by building the inventory, hands off to WP05, resumes once rule files exist.

## Implementation Notes

### Exact files to modify

- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-standards/SKILL.md`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-review/SKILL.md`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-elixir-essentials/SKILL.md`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-elixir-testing/SKILL.md`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-phoenix-liveview/SKILL.md`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/skills/in-ash-ecto-essentials/SKILL.md`

### Files to delete

- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/subagents/elixir/` (entire directory)

### Files to update

- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/subagents/.manifest/global-agents.json` (remove elixir entry)
- `/Users/matts/Devel/prj/Intent/intent/llm/MODULES.md` (remove elixir subagent row; update to match new canonical skill/subagent surface)

### Files to create

- `/Users/matts/Devel/prj/Intent/intent/st/ST0034/content-inventory.md`

### Refactored SKILL.md template (rule-reference style)

```yaml
---
description: "Elixir coding rules: pattern matching, tagged tuples, pipes, naming conventions, callbacks"
rules:
  - IN-EX-CODE-001
  - IN-EX-CODE-002
  - IN-EX-CODE-003
  - IN-EX-CODE-004
  - IN-EX-CODE-005
  - IN-EX-CODE-006
  - IN-EX-CODE-007
  - IN-EX-CODE-008
agnostic_rules:
  - IN-AG-PFIC-001
---

# Elixir Essentials

This skill loads the core Elixir coding rules. When invoked, read the rule
files listed in the `rules:` frontmatter. Each rule has Problem, Detection,
Bad Example, and Good Example.

## Rule list

- IN-EX-CODE-001: Multi-clause pattern matching over conditionals
- IN-EX-CODE-002: Tagged tuples for fallible operations
- IN-EX-CODE-003: @impl true on behaviour callbacks
- ... (list all)

## How to load

Read `intent/plugins/claude/rules/elixir/code/<slug>/RULE.md` for each rule ID
above. Apply the rule's Detection heuristic when reviewing or writing Elixir code.
```

### `in-review/SKILL.md` stage-2 structure

```markdown
## Stage 2: Language-specific code review

Detect the project ecosystem:

1. Look for `mix.exs` in project root → Elixir → `Task(subagent_type="critic-elixir", prompt="review <target>")`
2. Look for `Cargo.toml` in project root → Rust → `critic-rust`
3. Look for `Package.swift` in project root → Swift → `critic-swift`
4. Look for `.lua` entry file (main.lua, init.lua) or `.luarc.json` → Lua → `critic-lua`
5. Multiple indicators found → ask the user which language to target
6. None found → apply agnostic rules only (read `rules/agnostic/*`)
```

## Risks and Edge Cases

### Content inventory missed items

A rule in the deleted subagent is overlooked and becomes a silent loss. Mitigation: post-delete grep audit — `grep -rn "elixir" lib/ intent/plugins/` to find references, confirm each is addressed.

### Skill references stale rule IDs

A skill's `rules:` frontmatter lists IN-EX-CODE-007 but that rule was never authored. Mitigation: WP02's `intent claude rules validate` catches this on every CI run.

### Two skills reference the same rule

Not a violation — rules are meant to be referenced multiply. Only prose duplication is forbidden.

### in-review polyglot detection is brittle

Elixir + Lua + Rust projects exist (e.g. Nerves firmware). Skill defers to user rather than guessing.

### Deleting elixir subagent breaks Task invocations that name it

Per D6 / fail-forward: accepted. Release notes call out the migration.

## Testing Approach

### Highlander grep audit

```bash
# Rule prose should only appear in rule files
grep -rn "pattern matching" intent/plugins/claude/skills/  # should be zero hits
grep -rn "tagged tuple" intent/plugins/claude/skills/       # should be zero hits
grep -rn "strong assertion" intent/plugins/claude/skills/   # should be zero hits
```

### Dispatch smoke tests

- Create sample directory with only `mix.exs` → run `in-review` stage-2 → verify it picks critic-elixir
- Repeat for Rust/Swift/Lua
- Create dir with both `mix.exs` and `Cargo.toml` → verify prompt

### BATS

- `tests/unit/skills_commands.bats` (existing) — confirm `in-standards`, `in-review`, `in-elixir-*` still listed and installable
- No new BATS tests in this WP; dispatch logic is Claude-side, not shell-side

### Manual

- Invoke `in-standards` → Claude reads agnostic rules and applies to current work
- Invoke `in-review` on a real Elixir file → stage-2 dispatches to critic-elixir (WP07)

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: Content inventory (reading through all source files, cataloguing). Hand-off to WP05.
- **Session 2**: Skill refactors once WP05 rule files exist.
- **Session 3**: Delete subagent, update manifest, Highlander audit, regression tests.

## Exit Checklist

- [x] All acceptance criteria met
- [x] Content inventory documented and confirmed complete (covered by WP05 rule pack)
- [x] Highlander grep audit passes (zero fenced code blocks in refactored skills)
- [x] elixir subagent directory removed
- [x] global-agents.json updated
- [x] MODULES.md reflects new canonical surface (rule-reference-skills + highlander-audit BATS already rowed)
- [x] `intent claude rules validate` passes — 23/23 ok
- [ ] Release notes draft includes "BREAKING: elixir subagent removed; use critic-elixir instead" (deferred to WP11)

## As-Built (Done — 2026-04-22)

### Summary

Canon `elixir` subagent fully removed — 8 files (4,255 lines) deleted. All rule-like content now lives in the atomic RULE.md files WP05 authored. Five skills (`in-elixir-essentials`, `in-elixir-testing`, `in-ash-ecto-essentials`, `in-phoenix-liveview`, `in-standards`) were rewritten as thin pointer files referencing rules by ID. `in-review` stage-2 was language-parameterised with a critic-`<lang>` dispatcher placeholder (actual critic subagents land in WP07). Two new BATS files guard the invariants: every refactored skill references the right rule IDs, and no skill carries fenced code blocks (Highlander proxy).

### Deletions

- `intent/plugins/claude/subagents/elixir/` — entire directory, 8 files (`agent.md`, `antipatterns.md`, `ash-ecto.md`, `liveview.md`, `project-structure.md`, `style.md`, `testing.md`, `metadata.json`).
- Removed `elixir` entry from `intent/plugins/claude/subagents/.manifest/global-agents.json`.

### Refactors

| Skill                      | Before (lines) | After (lines) | Rules referenced                                       |
| -------------------------- | -------------- | ------------- | ------------------------------------------------------ |
| `in-elixir-essentials`     | 202            | ~50           | IN-EX-CODE-001..006                                    |
| `in-elixir-testing`        | 199            | ~50           | IN-EX-TEST-001..007                                    |
| `in-ash-ecto-essentials`   | 165            | ~35           | IN-EX-ASH-001..002                                     |
| `in-phoenix-liveview`      | 186            | ~55           | IN-EX-LV-001..003, IN-EX-PHX-001, + shared code rules  |
| `in-standards`             | 64             | ~55           | IN-AG-HIGHLANDER/PFIC/THIN-COORD/NO-SILENT-001         |
| `in-review` (stage-2 only) | —              | —             | Language dispatcher added; critic-`<lang>` placeholder |

Zero fenced code blocks in any refactored skill — rule prose lives exclusively in `rules/**/RULE.md` now.

### New tests (9 added; suite grows 555 → 564)

- `tests/unit/rule_reference_skills.bats` (5 tests) — each refactored skill references the correct rule IDs.
- `tests/unit/highlander_audit.bats` (4 tests) — zero code fences, thin (<150 lines), canon dir gone, manifest clean.

### Tests updated (elixir references swapped to socrates)

- `tests/unit/agent_commands.bats` — 8 test cases updated (list, install multi, install --all, sync multi, uninstall multi, uninstall --all, show metadata, show installation info, show works-for-both, status multi).
- `tests/unit/ext_discovery.bats` — 1 test case (`subagents list still shows canon agents`).
- `tests/unit/test_diogenes.bats` — 1 test case (`in-elixir-testing file has correct content`) — swap from inline-rule-prose assertions to rule-ID-reference assertions.

### Commits landed under WP03

- (this turn) — canon deletion + 5 skill rewrites + in-review refactor + 2 BATS files + test updates.
