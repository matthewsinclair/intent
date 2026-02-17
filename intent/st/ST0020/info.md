---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
intent_version: 2.3.4
status: WIP
created: 20260217
completed:
---

# ST0020: Modernizing Intent's Elixir Support for Agentic Coding

## Objective

Modernize Intent's Elixir support from a subagent-only model to a layered skills + subagent architecture that shapes code as it is generated, adds missing coverage (Ash/Ecto, LiveView, testing, project structure), and rationalizes the LLM guidance file system (AGENTS.md + RULES.md + ARCHITECTURE.md) for target projects.

## Problem

Intent v2.3.4 ships an Elixir subagent with strong architectural principles but three structural weaknesses:

1. **Reactive only** — subagents review after the fact; no enforcement during code generation
2. **Overlapping rules** — 23 core rules with significant duplication (3 say "pattern match", 5 say "use pipes")
3. **Missing coverage** — no Ash database patterns, no LiveView operational detail, no testing reference, no project structure guidance

The core symptom: Claude defaults to imperative nested `if/case/cond` instead of idiomatic multi-clause pattern matching, even with the subagent loaded, because rules are buried, overlapping, and lack concrete examples.

## Solution

A layered approach:

- **Skills** (always-on, proactive) — `elixir-essentials`, `ash-ecto-essentials`, `phoenix-liveview` shape code as it is generated
- **Subagent** (on-demand, reactive) — refactored agent.md with ~12 non-overlapping rules, plus new reference docs for deep review
- **Templates** — `intent agents init --template elixir` creates pre-populated AGENTS.md, RULES.md, ARCHITECTURE.md
- **Infrastructure** — `intent claude skills install/sync/uninstall` manages skill lifecycle like subagents

## Work Packages

- **WP-01**: Distill core Elixir rules (23 → ~12) — Foundation
- **WP-02**: Create `elixir-essentials` skill — Highest-impact change
- **WP-03**: Create Ash/Ecto reference doc + `ash-ecto-essentials` skill
- **WP-04**: Create LiveView reference doc + `phoenix-liveview` skill
- **WP-05**: Expand testing reference doc
- **WP-06**: Skill installation infrastructure (`intent claude skills`)
- **WP-07**: RULES.md / ARCHITECTURE.md templates and tooling
- **WP-08**: Project structure reference doc
- **WP-09**: Elixir upgrade skill for existing projects
- **WP-10**: Intent usage-rules.md

## Sequencing

```
Phase 0 (documentation): ST0020 docs — NO CODE until committed
Phase 1 (foundation):    WP-01 + WP-08 in parallel
Phase 2 (skills):        WP-02 + WP-03 + WP-04 in parallel
Phase 3 (testing):       WP-05
Phase 4 (infrastructure): WP-06 + WP-07
Phase 5 (upgrade+docs):  WP-09 + WP-10 in parallel
```

## Related Steel Threads

- ST0019: Treeindex (directory summaries — used by subagent for navigation)

## Key Decisions

- Skills are installable artifacts with lifecycle management (not manual copy)
- Ash-first, never raw Ecto — all target projects use Ash
- Three-file LLM guidance: AGENTS.md (factual), RULES.md (prescriptive), ARCHITECTURE.md (descriptive)
- Do NOT replace subagents with skills — they are complementary
- Do NOT implement git hooks — Intent is not a hooks manager
