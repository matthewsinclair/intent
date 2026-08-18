# Design - ST0026: Steel Thread Zero

## Approach

ST0026 delivers the "Steel Thread Zero" concept: a foundational steel thread that every new Intent-managed project runs FIRST, before any feature work. It primes the project with every countermeasure we know to prevent code quality violations from accumulating.

The approach is: **prevention over remediation**. Rather than audit and fix after the fact (797 violations across laksa-web + Lamplight), we bake the rules, registries, and enforcement into the project from commit one.

### Delivery Strategy

Work is organized into 10 work packages, sequenced by dependency and priority:

**Wave 1 -- Foundation (WP-01, WP-02)**:
Rename skills to `in-*` prefix, create 5 new workflow skills. This is the ergonomic foundation that makes everything else usable.

**Wave 2 -- Templates (WP-03, WP-05)**:
Create the LLM guidance templates (CLAUDE.md, MODULES.md, DECISION_TREE.md, ARCHETYPES.md). These are the documents that every project gets from day one.

**Wave 3 -- Memory (WP-04)**:
`intent claude prime` -- the killer feature. Reads templates and synthesizes them into MEMORY.md. Solves session amnesia.

**Wave 4 -- Enforcement (WP-06, WP-07, WP-08)**:
Custom Credo checks, `intent audit quick`, `intent audit health`, module checklist, dependency graph enforcement. These catch violations automatically.

**Wave 5 -- Integration (WP-09, WP-10)**:
Retrofit installation for brownfield projects, integrator command for greenfield. The final assembly.

### Dependency Graph

```
WP-01 (skill rename)
  └── WP-02 (workflow skills)

WP-03 (templates) ──────┐
WP-05 (archetypes) ─────┤
                         ├── WP-04 (memory injection) ── WP-10 (integrator)
WP-06 (enforcement) ────┤
  └── WP-07 (health)    │
WP-08 (guardrails) ─────┤
                         └── WP-09 (retrofit) ────────── WP-10
```

## Design Decisions

### DD1: Skill Rename -- `in-*` over `intent-*`

**Decision**: Rename all skills from `intent-*` to `in-*` prefix.

**Rationale**: `/intent-elixir-essentials` is 27 characters. `/in-elixir-essentials` is 22. Over hundreds of invocations per week, this adds up. The `in-` prefix is still clearly namespaced as Intent skills while being significantly faster to type.

**Impact**: All 6 existing skills renamed. All references updated. Old installed skills uninstalled, new ones installed.

### DD2: Workflow Skills -- Procedural vs Enforcement

**Decision**: New workflow skills (`/in-start`, `/in-plan`, `/in-standards`, `/in-next`, `/in-finish`) are **procedural** skills (step-by-step checklists) rather than enforcement skills (always-on rules).

**Rationale**: The existing skills (essentials, elixir, ash, etc.) are enforcement rules -- they're always active and shape code generation. The workflow skills are different: they're invoked at specific moments (session start, planning, session end) and execute a procedure.

The SKILL.md format supports both: enforcement skills have `## Rules` sections, procedural skills have `## Procedure` sections (like `in-autopsy`).

### DD3: Memory Injection -- Condensation, Not Verbatim Copy

**Decision**: `intent claude prime` condenses source files into a structured MEMORY.md rather than copying them verbatim.

**Rationale**: MEMORY.md has a 200-line effective limit (lines after 200 are truncated in Claude Code's context window). Source files (RULES.md, MODULES.md, DECISION_TREE.md) may be 50-100 lines each. Verbatim concatenation would exceed the limit. Prime must synthesize: extract key points, omit examples, use terse formatting.

### DD4: MODULES.md -- Flat Table, Not Nested Hierarchy

**Decision**: MODULES.md uses flat tables grouped by domain, not a nested tree structure.

**Rationale**: The primary use case is: "I need to do X, which module owns that?" A flat table with `| Concern | THE Module |` answers this instantly. A nested hierarchy adds cognitive load and makes grepping harder.

### DD5: Templates -- Language-Scoped

**Decision**: Archetype templates, Credo checks, and decision trees are scoped by language (e.g., `lib/templates/archetypes/elixir/`).

**Rationale**: Intent supports projects in Elixir, Rust, Swift, Lua (and potentially others). Templates must be language-appropriate. Starting with Elixir (most mature), with the directory structure ready for expansion.

### DD6: Retrofit -- Non-Destructive, Proposal-Based

**Decision**: `intent st zero install` generates proposals for human review rather than auto-applying changes.

**Rationale**: Brownfield projects have months of accumulated CLAUDE.md customization, established patterns, and project-specific conventions. Auto-applying ST0000 countermeasures could overwrite critical project-specific instructions. Every change must be individually reviewable and confirmable.

### DD7: Three-Tier Module Enforcement (D9)

**Decision**: Module registration enforcement uses three escalating tiers: CLAUDE.md instruction (soft), Claude Code hook (advisory), `intent modules check` command (explicit).

**Rationale**: Hard blocking on unregistered modules would be too disruptive -- rapid prototyping sometimes requires creating modules before registering them. The advisory approach warns without blocking, while the command provides explicit verification on demand.

## Architecture

### Deliverable-to-WP Mapping

| Deliverable | WP    | Description                           |
| ----------- | ----- | ------------------------------------- |
| D13         | WP-01 | Skill rename (intent-_ to in-_)       |
| D14         | WP-02 | Workflow skills (5 new /in-\* skills) |
| D2          | WP-03 | CLAUDE.md template                    |
| D3          | WP-03 | MODULES.md template                   |
| D6          | WP-03 | DECISION_TREE.md template             |
| D8          | WP-04 | Memory injection (prime)              |
| D4          | WP-05 | Archetype templates                   |
| D5a         | WP-06 | Custom Credo checks                   |
| D5b         | WP-06 | `intent audit quick`                  |
| D7          | WP-07 | `intent audit health`                 |
| D10         | WP-07 | Learnings accumulator                 |
| D9          | WP-08 | New module checklist                  |
| D11         | WP-08 | Dependency graph enforcement          |
| D12         | WP-09 | Retrofit installation                 |
| D1          | WP-10 | Integrator (`init --with-st0000`)     |

### New Files Created

```
intent/plugins/claude/skills/
  in-essentials/SKILL.md          (renamed from intent-essentials)
  in-elixir-essentials/SKILL.md   (renamed)
  in-ash-ecto-essentials/SKILL.md (renamed)
  in-phoenix-liveview/SKILL.md    (renamed)
  in-elixir-testing/SKILL.md      (renamed)
  in-autopsy/SKILL.md + scripts/  (renamed)
  in-start/SKILL.md               (NEW)
  in-plan/SKILL.md                (NEW)
  in-standards/SKILL.md           (NEW)
  in-next/SKILL.md                (NEW)
  in-finish/SKILL.md              (NEW)

intent/plugins/claude/bin/
  intent_claude_prime              (NEW -- D8)

lib/templates/
  llm/_CLAUDE.md                  (UPDATED -- D2)
  llm/_MODULES.md                 (NEW -- D3)
  llm/_DECISION_TREE.md           (NEW -- D6)
  llm/_ARCHETYPES.md              (NEW -- D4)
  archetypes/elixir/*.ex.eex      (NEW -- D4, 9 templates)
  credo_checks/elixir/*.ex        (NEW -- D5a, 6 checks)
  prime/operational-knowledge.md  (NEW -- D8)

bin/
  intent_audit                    (NEW -- D5b, D7)
  intent_learn                    (NEW -- D10)
  intent_modules                  (NEW -- D9)
  intent_st_zero                  (NEW -- D1)

lib/help/
  audit.help.md                   (NEW)
```

### What Prevents Each Violation Category

| Category              | Prevention                                          | When                    |
| --------------------- | --------------------------------------------------- | ----------------------- |
| P0 bugs               | RULES.md in CLAUDE.md + MEMORY.md + /in-standards   | Every conversation      |
| P1 Highlander         | MODULES.md + `intent modules check` + /in-standards | Module creation + audit |
| P2 thick coordinators | Archetypes + decision tree + line threshold check   | Module creation + audit |
| P3 mechanical         | Custom Credo checks + `intent audit quick`          | Every compile / CI      |
| Dependency violations | Dependency graph in `intent audit quick`            | Audit                   |

## As-Built Deviations

### Execution order differed from plan

The plan had WP-01 -> WP-02 -> WP-03+05 -> WP-04 -> WP-11. Actual execution reordered to: WP-01 -> WP-03+05 -> WP-02 -> WP-04 -> WP-11. This unblocked WP-04 (memory injection) one session earlier by doing templates before workflow skills.

### WP-03 and WP-05 combined in one commit

Templates and archetypes were delivered together as `ef46eea` since they share the same template directory structure and have no dependency conflict.

### Rename migration added to plugin sync

Not in original design. During WP-01, recognized that users with old `intent-*` skills installed would need automatic migration. Added rename detection logic to `plugin_sync()` in the shared plugin library.

### Highlander fix added to WP-03 scope

Original WP-03 spec didn't mention consolidating the three CLAUDE.md heredocs. This was identified during implementation as a Highlander violation and fixed as part of WP-03 -- all three copies now use the single template.

### WP-08 scope expanded with output rationalization

WP-08 originally scoped to D9 (module checklist) and D11 (dependency graph). During implementation, user requested rationalization of all CLI output to Rust-style conventions (lowercase prefixes, no separator bars). This touched 14+ source scripts and 14 test files beyond the original WP-08 scope.

### TN004 filename simplified

Original spec placed the tech note at `intent/docs/tn004-total-codebase-audit.md`. Renamed to `intent/docs/total-codebase-audit.md` since the `tn004` prefix is a deliverable identifier, not part of the filename convention.

## Alternatives Considered

### Alt 1: Single Monolithic Skill Instead of Templates

Could have put all rules, modules, decision tree into one giant skill. Rejected because: skills are always-on context (consuming tokens on every conversation), while templates are read-on-demand. Templates scale better.

### Alt 2: Git Hooks Instead of Credo Checks

Could have used git pre-commit hooks for enforcement. Rejected because: hooks are per-developer and easy to bypass (`--no-verify`). Credo checks run in CI and are project-level.

### Alt 3: Auto-Apply Retrofit

Could have made `intent st zero install` auto-apply all changes. Rejected because: brownfield projects have accumulated customization that must not be destroyed. Proposal-based approach is safer.
