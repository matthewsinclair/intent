---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
intent_version: 2.4.0
status: WIP
slug: steel-thread-zero
created: 20260304
completed:
---

# ST0026: Steel Thread Zero

## Objective

Define and implement a "Steel Thread Zero" (ST0000) concept: a foundational steel thread that every new Intent-managed project runs FIRST, before any feature work begins. ST0000 primes the project with every countermeasure we know to prevent code quality violations from accumulating.

The goal: make it structurally impossible (or at least very hard) for the violations found in audits like ST0058 (408 violations in laksa-web) to ever happen again.

## Context

### The Problem

In laksa-web (ST0058), a total codebase audit found 408 violations:

- 7 P0 bugs/crash risks (bare `=` on fallible calls, `String.to_atom` on user input, etc.)
- 14 P1 Highlander duplications (same logic in 2-12 places)
- ~50 P2 thick coordinators (business logic in controllers/LiveViews/CLI commands)
- ~337 P3 mechanical style violations (boolean operators, missing @impl, etc.)

These accumulated over months of development. Each violation was individually small, but collectively they represented significant technical debt. The remediation (ST0058 Phase 4 + ST0060) took substantial effort.

### The Root Cause

The violations weren't caused by ignorance — the rules existed in `RULES.md`. They accumulated because:

1. **Rules weren't present from commit one** — they were added after patterns were already established
2. **No canonical module registry** — developers (human and AI) didn't know which module already owned a concern, so they created duplicates
3. **No archetype templates** — new controllers/LiveViews started blank, so business logic crept in
4. **No automated enforcement** — mechanical violations (boolean operators, @impl) had no tooling to catch them
5. **No "where does this go?" guidance** — the decision of where to put logic was left to ad-hoc judgement
6. **Session amnesia** — Claude Code lost learned patterns on every context reset, re-learning the same mistakes
7. **No periodic health checks** — violations accumulated silently until the audit

### The Solution: Prevention Over Remediation

ST0000 addresses each root cause with a specific countermeasure, delivered as Intent infrastructure that applies to every new project.

## Related Steel Threads

- **laksa-web ST0058**: Total Codebase Audit (the audit that revealed the need)
- **laksa-web ST0060**: P2 Thick Coordinator Extractions (the remediation)
- **TN004**: Total Codebase Audit process tech note

## Deliverables

### D1: `intent init --with-st0000` (or `intent st zero`)

A new Intent command that bootstraps a project with all ST0000 countermeasures. This is the primary entry point. Running it on a new (or existing) project should:

1. Create/update `CLAUDE.md` with the full rule set and architectural guidance
2. Create `intent/llm/RULES.md` from a language-appropriate template
3. Create `intent/llm/MODULES.md` (canonical module registry)
4. Create `intent/llm/DECISION_TREE.md` (where does this go?)
5. Create `intent/llm/ARCHETYPES.md` (module templates)
6. Prime Claude Code memory with operational knowledge
7. Optionally scaffold custom Credo checks (Elixir projects)

### D2: CLAUDE.md Template with Rules Baked In

A comprehensive CLAUDE.md template that includes:

- The complete rule set (not just a reference to RULES.md)
- The canonical module registry (inline or by reference)
- The "where does this go?" decision tree
- Session start checklist (what to read on every context reset)
- Project-specific patterns and conventions

**Key principle**: CLAUDE.md is read on EVERY conversation. If the rules are there from commit one, every line of code is written against them.

### D3: Canonical Module Registry (`MODULES.md`)

A living document that declares ownership of concerns:

```markdown
## Auth & Authorization

| Concern            | THE Module                  |
| ------------------ | --------------------------- |
| Auth checks        | `MyApp.Authorization`       |
| Session management | `MyApp.Auth.SessionService` |

## Content

| Concern         | THE Module                    |
| --------------- | ----------------------------- |
| Content hashing | `MyApp.Content.ContentHasher` |
| Status parsing  | `MyApp.Content.StatusParser`  |
```

**Enforcement mechanism**: An Intent command or skill that:

- Validates new modules are registered before creation
- Warns when a module is created that overlaps an existing registry entry
- Can be queried: `intent modules find "subscription status"` → shows the canonical module

### D4: Archetype Templates

Starter templates for common module types that are ALREADY correct:

| Archetype          | What it enforces                                        |
| ------------------ | ------------------------------------------------------- |
| Ash Domain         | Domain module with `resources` block, code interfaces   |
| Ash Resource       | Resource with actions, policies, attributes skeleton    |
| Phoenix Controller | Thin coordinator: parse params → call service → render  |
| LiveView           | Two-phase mount, handle_event → service → assign        |
| Service Module     | Pure business logic, tagged tuple returns, @spec        |
| CLI Command        | Arca.Cli BaseCommand, config macro, delegate to service |
| GenServer/Worker   | Callbacks with @impl, init/handle\_\* skeleton          |
| Oban Worker        | @impl perform/1, tagged tuple returns                   |
| Test Module        | ExUnit skeleton with describe blocks, setup             |

Each template includes:

- Comments explaining what goes here vs what goes elsewhere
- Anti-patterns as comments: `# DO NOT put business logic here`
- Links to the canonical module for the concern

**Delivery**: `intent generate <archetype> <module_name>` or integrated into existing `mix` generators.

### D5: Automated Rule Enforcement

#### D5a: Custom Credo Checks (Elixir)

Programmatic enforcement of mechanical rules that can be checked statically:

| Check                   | Rule | What it catches                                    |
| ----------------------- | ---- | -------------------------------------------------- | --- | ------------------------------ |
| `BooleanOperators`      | R8   | `&&`/`                                             |     | ` on known boolean expressions |
| `MissingImplAnnotation` | R11  | Behaviour callbacks without `@impl true`           |
| `DebugArtifacts`        | R15  | `IO.inspect`, `IO.puts`, `dbg()` in lib/           |
| `MapGetOnStruct`        | R7   | `Map.get(known_struct, :field)`                    |
| `ThickCoordinator`      | R2   | Controllers/LiveViews exceeding line threshold     |
| `HighlanderSuspect`     | R6   | Functions with identical names in multiple modules |

These would live in the project's `lib/mix/checks/` or as a shared Credo plugin.

**Important**: Not all rules are statically enforceable. R6 (Highlander) in particular requires semantic understanding. The `HighlanderSuspect` check flags _potential_ duplicates by name; actual Highlander enforcement needs periodic audits (D7).

#### D5b: `intent audit quick` Command

A lightweight audit command that runs the enforceable checks without a full Socratic sub-agent audit:

```bash
intent audit quick              # Run all automated checks
intent audit quick --rule R8    # Run specific rule check
intent audit quick --fix        # Auto-fix where possible
```

For Elixir: wraps `mix credo` with custom checks.
For other languages: wraps the ecosystem's linter with custom rules.

### D6: Decision Tree ("Where Does This Go?")

A flowchart document that answers the single most common question:

```
Is it a database query or data transformation?
  → Ash resource action or domain module

Is it business logic (validation, orchestration, calculation)?
  → Dedicated service module in lib/myapp/domain/

Is it HTTP request/response handling?
  → Controller (thin: parse params → service → render)

Is it WebSocket/LiveView state management?
  → LiveView (thin: handle_event → service → assign)

Is it a CLI command?
  → Command module (thin: parse args → service → format output)

Is it formatting for display?
  → View helper or component

Does something similar ALREADY EXIST?
  → STOP. Check MODULES.md. Use the existing module.
  → If it doesn't exist yet, register it in MODULES.md FIRST, then create it.
```

### D7: Lightweight Periodic Audit

A command that runs a quick health check, suitable for end-of-day use:

```bash
intent audit health             # ~2 min, covers mechanical rules
intent audit health --report    # Generates a markdown report
intent audit health --diff      # Only check files changed since last audit
```

This is NOT a full Socratic audit (that's TN004). This is:

1. Run custom Credo checks (D5a)
2. Check for new modules not in MODULES.md
3. Flag any controller/LiveView that grew past the line threshold
4. Report any functions with identical names in multiple modules (Highlander suspects)

Output: a short markdown report that can be reviewed in 5 minutes next morning.

### D8: Memory Injection (`intent claude prime`)

**This is the killer feature.**

A command that pre-loads Claude Code's project memory with everything it needs to know:

```bash
intent claude prime                    # Prime current project
intent claude prime --refresh          # Re-prime after rule changes
intent claude prime --from <project>   # Import learnings from another project
```

What it generates/updates in `.claude/projects/.../memory/MEMORY.md`:

1. **Operational knowledge**: How to use Intent commands (`intent st`, `intent wp`, `intent treeindex`, etc.) — the stuff that currently takes 35 corrections per session
2. **Project rules**: Condensed from `intent/llm/RULES.md` — the key rules with examples
3. **Canonical modules**: From `MODULES.md` — so the LLM knows what exists before creating duplicates
4. **Known footguns**: Project-specific anti-patterns learned from previous audits
5. **Decision tree**: Condensed "where does this go?" guidance
6. **Session checklist**: What to read on startup and after context resets

**Memory injection sources** (in priority order):

1. `intent/llm/RULES.md` — project coding rules
2. `intent/llm/MODULES.md` — canonical module registry
3. `intent/llm/DECISION_TREE.md` — architectural guidance
4. `intent/llm/ARCHETYPES.md` — module templates reference
5. `.intent/learnings.md` — accumulated project-specific learnings (new file)
6. Global Intent operational knowledge (bundled with Intent itself)

**Post-context-refresh**: The memory file persists across conversations, so it's automatically available after context resets. But we should also add a reminder in CLAUDE.md:

```markdown
## After Context Reset

Re-read these files before continuing work:

- intent/llm/RULES.md
- intent/llm/MODULES.md
- This file (CLAUDE.md)
```

### D9: New Module Checklist (enforced via skill or hook)

Before creating ANY new module, the workflow should be:

1. Check `MODULES.md` — does a module for this concern already exist?
2. If yes → extend it (or delegate to it)
3. If no → register the new module in `MODULES.md` FIRST
4. Choose the correct archetype template
5. Place it in the correct directory per the decision tree

**Enforcement options**:

- A Claude Code hook that fires on `Write` tool calls to `lib/**/*.ex` and checks if the module is registered
- An Intent skill that wraps module creation: `intent generate service MyApp.Foo.BarService`
- A CLAUDE.md instruction that says "ALWAYS check MODULES.md before creating a new module"

### D10: Learnings Accumulator

A project-local file (`.intent/learnings.md`) that accumulates hard-won knowledge:

```markdown
## Footguns

- `published_at` has a time component; use date granularity for visibility checks
- Bare `{:ok, x} =` on Ash calls will crash on NotFound — always use case/with

## Patterns That Worked

- Service extraction: controller → service → controller delegates
- Single dispatch point: ServingMode.resolve/1 for all content loading

## Patterns That Failed

- Putting parse_status/1 in each module "because it's small" → 4 incompatible copies
```

Updated manually or via `intent learn "description"` command. Consumed by `intent claude prime` (D8).

## Architecture

### How the pieces fit together

```
intent init --with-st0000
    │
    ├── Creates CLAUDE.md (D2) ─── read by Claude on every conversation
    ├── Creates RULES.md (existing) ─── coding standards
    ├── Creates MODULES.md (D3) ─── canonical module ownership
    ├── Creates DECISION_TREE.md (D6) ─── where does this go?
    ├── Creates ARCHETYPES.md (D4) ─── module templates
    ├── Scaffolds Credo checks (D5a) ─── automated enforcement
    └── Runs `intent claude prime` (D8) ─── memory injection
            │
            └── Generates MEMORY.md ─── persists across sessions
                    │
                    ├── Intent operational knowledge
                    ├── Project rules (condensed)
                    ├── Canonical modules
                    ├── Known footguns
                    └── Session checklist

During development:
    intent generate <archetype> ─── D4 templates
    intent modules check ─── D3 registry validation
    intent audit quick ─── D5b automated checks
    intent audit health ─── D7 end-of-day health check
    intent learn "..." ─── D10 accumulate learnings
    intent claude prime --refresh ─── D8 re-prime after changes
```

### What prevents each violation category

| Category              | Prevention Mechanism                                       | When it fires           |
| --------------------- | ---------------------------------------------------------- | ----------------------- |
| P0 bugs               | RULES.md in CLAUDE.md + memory                             | Every conversation      |
| P1 Highlander         | MODULES.md registry + `intent modules check`               | Module creation + audit |
| P2 thick coordinators | Archetype templates + decision tree + line threshold check | Module creation + audit |
| P3 mechanical         | Custom Credo checks                                        | Every compile / CI      |

## Non-Goals

- This is NOT a full audit framework (that's TN004)
- This is NOT a CI/CD integration (that's a follow-on)
- This does NOT replace human judgement — it augments it with guardrails

## Success Criteria

1. A new Intent project bootstrapped with `intent init --with-st0000` should have all countermeasures in place before the first feature commit
2. Running `intent audit quick` on a freshly bootstrapped project should produce zero violations
3. A Claude Code session on a primed project should never need to be told "check MODULES.md first" — it already knows
4. After 3 months of development, running a full TN004 audit should find <50 violations (vs 408 in laksa-web)

## Implementation Priority

| Deliverable                     | Impact     | Effort | Priority            |
| ------------------------------- | ---------- | ------ | ------------------- |
| D8: Memory injection            | Highest    | Medium | 1st                 |
| D2: CLAUDE.md template          | Highest    | Low    | 1st                 |
| D3: MODULES.md                  | High       | Low    | 1st                 |
| D6: Decision tree               | High       | Low    | 1st                 |
| D4: Archetype templates         | High       | Medium | 2nd                 |
| D5b: `intent audit quick`       | High       | Medium | 2nd                 |
| D7: `intent audit health`       | Medium     | Medium | 3rd                 |
| D5a: Custom Credo checks        | Medium     | High   | 3rd                 |
| D10: Learnings accumulator      | Medium     | Low    | 3rd                 |
| D9: New module checklist        | Medium     | Medium | 4th                 |
| D1: `intent init --with-st0000` | Integrator | Medium | Last (wraps D2-D10) |
