---
title: "Pre-loading Claude with Intent context"
date: "2026-04-28"
author: "Matthew Sinclair"
draft: true
word_count: 1700
---

> **Editor's note.** This post supersedes an earlier v2.1.0 piece that described the original sub-agent installation system. Backlog.md integration was removed in v2.5.0. The `intent agents` namespace was repurposed in v2.10.0 to manage the project's `AGENTS.md` file at the root. Claude Code sub-agent installation moved to `intent claude subagents install`. The v2.10 line also added session hooks, a rule library, a pre-commit critic gate, and the three-file canonical LLM config that this post covers. v2.11.0 (ST0037) replaced filesystem-marker language detection with an explicit `languages` config field. What follows is the architecture as of v2.11.0.

# Pre-loading Claude with Intent context

Intent is a CLI tool for managing project intent — steel threads, work packages, the rule libraries and architectural notes that go alongside them. Intent doesn't write code itself; it produces a body of context that LLM-based tools can read so a session arrives knowing the project. This post is about how that context gets in front of Claude Code.

## The context problem

Every Claude session starts the same way. You ask for help, Claude asks what your project structure is, you explain Intent, Claude asks what steel threads are, you explain those, and the conversation reaches the actual change you wanted to make somewhere around minute eight:

```
You: "Help me add a caching layer"
Claude: "I'll help you add caching. What's your project structure?"
You: "I use Intent with steel threads for organisation..."
Claude: "Can you explain what steel threads are?"
[5 minutes of explanation later]
Claude: "Should I create a new file for the cache?"
You: "No, first run 'intent st new'..."
```

Working with a smart colleague who has never seen the project, every time. The fix is to get the project context in front of Claude before the session starts; once you commit to that, it's a config problem.

## The three-file canon

The base layer of an Intent project's LLM config is three plain markdown files at the project root, each one read by a different audience.

`AGENTS.md` is the tool-agnostic contract. Every major agentic CLI — Claude Code, Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI — reads `AGENTS.md` as the canonical project overview. It covers the project structure, the development environment, how to run the test suite, commit guidelines, security conventions, and pointers to deeper documentation: the kind of orientation a new engineer would need on day one. `intent agents sync` regenerates it from the project state.

`CLAUDE.md` is a Claude-specific overlay on top of `AGENTS.md`. It carries the directives that make sense only for Claude Code: which slash commands to run at session start, how the session hooks are wired, how the post-compact reload procedure works, plus anything else that goes beyond the tool-agnostic contract.

`usage-rules.md` is a terse DO / NEVER contract. It's short enough to read in thirty seconds and specific enough to gate decisions inside a session — the sort of file an LLM can re-read at any point in a conversation when it needs to remember what's allowed.

`intent claude upgrade` keeps the three files in sync with canon. The command checks each one against the canonical templates and either copies a fresh version or refreshes an existing user-customised one while preserving the user-edited section.

## Session hooks

A static config file is only useful if Claude reads it. Intent v2.10 wires three Claude Code lifecycle hooks under `.claude/settings.json` so the three-file canon ends up in the model's context.

The `SessionStart` hook runs `.claude/scripts/session-context.sh`, which prints the project name, current branch, in-progress steel threads, and a reminder to run `/in-session`. The script's output goes into the model's context at the start of every new or resumed Claude Code session.

The `UserPromptSubmit` hook runs `.claude/scripts/require-in-session.sh` as a strict gate. The first prompt of a session is blocked until `/in-session` has been invoked; the gate looks for a sentinel file under `/tmp/intent/` keyed per-project, so multiple Intent projects can run concurrent Claude Code sessions without contaminating each other.

The `Stop` hook prints a reminder to run `/in-finish` at session wrap-up so the WIP and restart files get updated before the session closes. All three hook scripts ship as templates under `lib/templates/.claude/scripts/` and get installed by `intent claude upgrade --apply`.

The strict gate is the load-bearing one. Without it, a user can submit a substantive first prompt before the methodology has been loaded, and the session ends up where it would have been without any Intent integration. The gate forces orientation to land before substantive work begins.

## Skills

A Claude Code skill is a procedural guide — a markdown file under `~/.claude/skills/<name>/SKILL.md` (or `intent/plugins/claude/skills/<name>/SKILL.md` for canonical Intent skills), invoked by the user via the slash command `/<name>`. Intent ships a family of `in-*` skills.

Three of them are universal. `/in-session` is the bootstrap. The skill loads the project state (WIP, restart files, steel-thread inventory) together with the universal coding-discipline skills `/in-essentials` and `/in-standards`. It then reads the `languages` array from `intent/.config/config.json` to decide which language-specific essentials skills to load. The last step writes the per-session sentinel that releases the `UserPromptSubmit` gate.

`/in-essentials` carries the Intent workflow rules — using the CLI for steel-thread management, leaving generated files alone, and similar process-level constraints. It also includes the discipline of checking `intent/llm/MODULES.md` before creating any new module. `/in-standards` carries the agnostic coding discipline: Highlander, PFIC (Pure-Functional-Idiomatic-Coordination), Thin Coordinator, No Silent Errors.

Languages-in-use is a per-project configuration decision, not a filesystem detection. The user declares the set with `intent lang init <lang>` (and removes with `intent lang remove <lang>`); the canonical names are `elixir`, `rust`, `swift`, `lua`, `shell`. Array order is the explicit declaration; the first entry is the primary where a primary is needed (the pre-commit gate fires every entry equally). For each language listed, the bootstrap invokes any matching essentials skill — currently only Elixir has one (`/in-elixir-essentials` plus `/in-elixir-testing`); the other four ship their coding rules through the rule library at `intent/plugins/claude/rules/<lang>/` and the `critic-<lang>` subagent applied on demand. File presence is unreliable evidence of intent — a vendored example or a one-off script can flip the wrong switch — so the configuration is the single source of truth.

Workflow skills get invoked on demand at specific phases of work. `/in-plan` formalises a kickoff before substantial implementation. The systematic-debugging skill `/in-debug` walks a four-phase process with a three-strike architectural review. `/in-verify` is a verification gate that requires fresh evidence before a task can be marked complete. `/in-review` runs a two-stage code review — specification compliance first, then rule-library compliance via the language-specific critic. The end-of-session skills are `/in-handoff` for handoff documents and `/in-finish` for wrap-up.

The diagnostic skills include `/in-detrope` (which produced this post), `/in-autopsy` (session forensics that compares what happened in a session against the project's memory rules), and `/in-cost-analysis` (which estimates the cost of reproducing a codebase from scratch).

Skills are managed via `intent claude skills`. The subcommands are `install`, `sync`, `uninstall`, and `list`. Each install carries an SHA256 manifest entry, so drift between the canonical version and the installed version is detectable.

## Subagents and the critic family

A Claude Code sub-agent is a focused worker with its own context window and tool loadout. Intent ships several.

The critic family — `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell` — is one critic per supported language. Each one reads the rule library at `intent/plugins/claude/rules/<lang>/` and applies each rule's Detection section to a target file. Findings come back grouped by severity in a stable report format. Critics never modify code and never call external linters. A critic gets invoked with `Task(subagent_type="critic-<lang>", prompt="review <paths>")`; the `/in-review` skill auto-detects the project language and dispatches.

`diogenes` is an Elixir test architect — Socratic dialog about test design that produces test specifications and validates test quality. `socrates` is a CTO review mode that facilitates Socratic dialog about architectural decisions; it's distinct from `diogenes` and the two have always been separate agents.

Subagents are managed via `intent claude subagents` (`install`, `sync`, `uninstall`, `show`, `status`). The same SHA256 manifest pattern as skills.

## The rule library and the pre-commit gate

The rule library at `intent/plugins/claude/rules/` is the canonical source for coding rules. Each rule is a markdown file with YAML frontmatter and a Detection heuristic, plus good/bad examples for the pattern. Rules cite each other through frontmatter fields like `concretised_by:` and `upstream_id:`. Skills cite rule IDs in their `rules:` frontmatter; the critic subagents enforce the rules at invocation time.

The pre-commit critic gate runs the rule library against staged files at commit time. `intent claude upgrade` installs the gate as `.git/hooks/pre-commit`. The hook invokes `bin/intent_critic --staged` and blocks commits with `CRITICAL` or `WARNING` findings. Per-project carve-outs live in `.intent_critic.yml` at the project root.

The gate closes the loop between rule authoring and rule enforcement. Rule packs decay against the codebases they're meant to govern unless something runs them periodically; the gate runs them on every commit that touches a relevant file, so the rule pack and the codebase stay in agreement without anybody having to allocate time for an audit.

## What changes for the user

A first session in an Intent project, after `intent init` and `intent claude upgrade --apply`, looks like this. Claude Code starts and `SessionStart` fires immediately; the project context lands in the model's context window. The model's first instruction (from the `SessionStart` output) is to invoke `/in-session`. If the user types something else first, the `UserPromptSubmit` gate blocks it. The user types `/in-session`, the bootstrap skill runs through its procedure, and the gate releases. The user's next prompt runs against a Claude session that already has the methodology, the project state, the steel-thread inventory, and the language-specific rules in context. The first exchange is at the architectural level.

For ongoing work, every commit touching shell, Elixir, Rust, Swift, or Lua passes through the pre-commit critic gate. The gate-firing rate is meant to settle near zero — most of what fires it at first becomes a small per-script fix, and the rule pack stays in step with the codebase as both evolve. v2.10.1 was the second release through the gate; the rule pack has stayed in agreement with the codebase across both cuts.

For onboarding, a new contributor clones the repo and runs `intent claude upgrade --apply`. After that, they inherit the same hooks, skills, subagents, and rule library the existing team has. The senior-developer time that would otherwise go to explaining the project's conventions can be redirected.

## What's left

The current architecture covers the loading and validation phases of an LLM session. Three areas are still open.

Cross-tool fidelity is the most prominent. AGENTS.md is read by every major agentic CLI, but the depth of integration varies. Claude Code's hooks and skills are the most developed; other tools rely on the AGENTS.md text alone. A future release could ship adapters that emit equivalents of Claude Code's strict-gate behaviour in whatever form Cursor, Aider, or Codex understand.

Session-state portability is the second open area. The strict gate's per-project sentinel lives under `/tmp/intent/`, which works for a single user on a single machine. Containerised or remote sessions, and multi-user sessions on a shared host, would need a different state-file strategy.

Rule provenance is the third. The current `intent claude rules` API can list and validate rules. A longer-term goal is a per-rule audit trail — the authoring date, the source corpus, and the most recent time the rule either triggered against real code or got updated against findings. The data exists in commit history but isn't exposed as a first-class query yet.

[Back to Intent Blog Index](./README.md)
