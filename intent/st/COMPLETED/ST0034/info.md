---
verblock: "22 Apr 2026:v0.2: matts - Populated after Phase 1/2 planning"
intent_version: 2.9.0
status: Completed
slug: agentic-software-engineering-suite
created: 20260422
completed: 20260423
---

# ST0034: Agentic Software Engineering Suite

## Objective

Rationalise Intent's software-engineering skills and subagents into a coherent suite with Rules as first-class citizens. Add Critic subagents for Elixir, Rust, Swift, and Lua. Introduce a `~/.intent/ext/` extension mechanism that lets users contribute skills, subagents, and rule packs without forking Intent. Extract `worker-bee` from canon into the new extension system as the reference example. Ship as Intent v2.9.0 and upgrade the fleet.

## Context

Intent v2.8.2 has a useful but patchy set of software-engineering skills and subagents. The `elixir` subagent duplicates content in `in-elixir-*` skills with no sync contract. `in-standards` is a hollow "re-read CLAUDE.md" reminder rather than an enforcement surface. `in-review` stage-2 delegates to `diogenes` for Elixir tests but has no path for other languages. There is zero rule coverage for Rust, Swift, or Lua, despite TCA's multi-ecosystem claims. Users who want to add a skill, subagent, or rule must fork Intent; there is no user-local extension surface.

Meanwhile, `iautom8things/elixir-test-critic` (MIT, 2026 Manuel Zubieta, actively maintained) is a production-quality rule library with an elegant, LLM-friendly schema and 81 test-focused rules. Adopting its schema opens a path to full compatibility with upstream and other downstream consumers.

ST0034 unifies three initiatives into a single v2.9.0 release:

1. **Rules as first-class**. A rule library at `intent/plugins/claude/rules/` becomes the single source of truth; skills reference rules by ID; Critic subagents enforce them.
2. **Critic subagent family**. `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua` each with `code` and `test` modes. The existing `elixir` subagent is deleted; its content atomises into rules.
3. **Extension system**. `~/.intent/ext/<name>/` with self-contained extensions (subagents + skills + rule packs) and a declarative `extension.json`. Discovery is multi-root with user-ext shadowing canon (with warnings). Worker-bee moves out of canon as the reference example.

Fail-forward design: no deprecation stubs, no preservation of installed copies for deleted components. Migration actively prunes `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md` and rebuilds to fresh state.

elixir-test-critic integration is reference-and-recommend with schema adoption: Intent ships a curated subset of ~15-20 Elixir rules in Intent's voice, adopts the upstream schema verbatim, and points users at the upstream plugin for the full 81 rules. Full MIT attribution in `rules/_attribution/elixir-test-critic.md`.

## Scope

### In scope (v2.9.0)

- Rule schema as declarative frontmatter + Markdown (elixir-test-critic compatible)
- Rules as inert documents consumed by Critic subagents; no execution engine
- Elixir, Rust, Swift, Lua rule packs + agnostic pack (Highlander, PFIC, Thin Coordinator, No Silent Errors)
- `~/.intent/ext/` extension system with local-path discovery
- `worker-bee` removed from canon, relocated to ext as reference example
- 4 new Critic subagents (one per language) with code/test modes
- 4 new `intent ext` commands: `list`, `show`, `validate`, `new`
- `elixir` subagent deleted (replaced by critic-elixir + rules)
- Migration prunes installed copies of deleted subagents
- All 16 fleet projects upgraded to v2.9.0

### Out of scope

- Auto-fix in Critic subagents (they report; they do not rewrite)
- Languages beyond Elixir/Rust/Swift/Lua (Go, Python, TypeScript, Ruby)
- `intent ext install <url>` (deferred to v2.10 with its own security design)
- `intent ext uninstall`, `intent ext enable/disable`
- Rule-pack versioning or dependency resolution
- Central rule registry or cross-project rule sharing
- CI validation of Rust/Swift/Lua runnable examples (textual only)
- Plugin manifest format change (callback extension only)
- Merging `diogenes` into `critic-elixir` (complementary roles)
- Backwards-compatibility shims for deleted subagents
- Per-project extensions at `<project>/intent/ext/` (user-global only)

## Related Steel Threads

- ST0025: Highlander audit (callback pattern foundation for plugin discovery)
- ST0026: Skill rename `intent-*` → `in-*` (pattern for handling renames in the upgrade chain)
- ST0028: TCA v3.0 (multi-ecosystem validated rules for Rust/Swift — precedent for this ST's non-Elixir coverage)
- ST0030: Superpowers cherry-picks (`chains_to:` frontmatter, Red Flags table — patterns applicable to new Critic subagent prompts)
- ST0032: Credo check wiring (precedent for language-specific check integration)
- ST0033: Cwd-resilient dispatch (INTENT_ORIG_CWD pattern applicable to ext discovery)

## Work Packages

| WP   | Title                              | Deps                   | Size | Risk |
| ---- | ---------------------------------- | ---------------------- | ---- | ---- |
| WP01 | Architecture and rule schema       | —                      | M    | Med  |
| WP02 | Extension system foundation        | WP01                   | L    | High |
| WP03 | Skill and subagent rationalisation | WP01                   | M    | Med  |
| WP04 | Agnostic rule pack                 | WP01                   | S    | Low  |
| WP05 | Elixir rule pack                   | WP01, WP04             | M    | Med  |
| WP06 | Rust/Swift/Lua rule packs          | WP01, WP04             | L    | High |
| WP07 | Critic subagent family             | WP03, WP04, WP05, WP06 | L    | High |
| WP08 | Worker-bee extraction              | WP02                   | S    | Low  |
| WP09 | Migration and upgrade chain        | WP02, WP08             | M    | Med  |
| WP10 | Documentation                      | WP02, WP07, WP08       | M    | Low  |
| WP11 | Release and fleet upgrade          | WP01-WP10              | M    | Med  |

See `design.md` for architectural decisions (D1-D10), full acceptance criteria per WP, and the risk register. Each WP directory under `WP/NN/` has its own `info.md` with objective and acceptance criteria.

## Context for LLM

This document is the Objective and Context summary for steel thread ST0034. For architectural decisions, rule schema, naming conventions, risk register, and implementation detail, see `design.md` in this directory. For per-WP scope, see `WP/NN/info.md` for each work package. When starting a new session on this ST, begin by reading both this file and `design.md`, then check `intent/wip.md` for active WP.

### How to update this document

1. Update the status as work progresses (WIP → Done when WP11 ships)
2. Update the WP table if work packages are added, merged, or renumbered (rare — keep stable)
3. Update `design.md` for any architectural change; update this file's Objective/Context only if the goal changes
4. Mark the completion date when v2.9.0 ships and the fleet is upgraded
