# AGENTS.md

This is the primary tool-agnostic config file for AI coding agents working on this project. Every major agentic CLI (Claude Code, Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI) reads AGENTS.md as the canonical project contract. Spec: https://agents.md.

## Project Overview

Intent -- an Intent project. See `CLAUDE.md` for Claude-specific overlay.

## Development Environment

### Prerequisites

- Bats testing framework
- Bash 4.0+, POSIX-compliant shell

### Setup

```bash
intent doctor     # verify Intent configuration
```

## Build and Test Commands

### Testing

```bash
bats -r tests/
```

### Building

_No build step required._

### Validation

```bash
intent doctor       # check Intent configuration
intent st list      # list steel threads
```

## Code Style

- Shell scripts: 2-space indentation, POSIX-compliant where practical.
- Markdown: no manual line wrapping; verblock frontmatter on persistent docs.
- See `usage-rules.md` for the terse "DO / NEVER" contract.
- See `intent/docs/working-with-llms.md` at the Intent install for the canon tech note on the LLM-facing layout and how hooks + critics + skills compose.

## Steel Thread + Work Package Process

Work is organised into numbered **steel threads** (`ST####`), each containing **work packages** (`WP`). Every steel thread has its own directory under `intent/st/<ID>/` with `info.md` as the entry point.

```bash
intent st list               # list steel threads
intent st new "Title"        # create new
intent st show <id>          # inspect
intent wp list <STID>        # list work packages
intent wp new <STID> "..."   # create work package
intent wp start <STID/NN>    # mark WIP
intent wp done <STID/NN>     # mark done
```

Never create steel thread or work package directories by hand -- always use the CLI.

## Installed Skills

- **in-ash-ecto-essentials** -- Ash Framework database-access rules: domain code interfaces, actor on query/changeset
- **in-autopsy** -- Session forensics: analyzes Claude Code sessions against memory rules, finds gaps and enforcement failures
- **in-cost-analysis** -- Cost analysis: estimates development cost of reproducing a codebase from scratch
- **in-debug** -- Systematic debugging: 4-phase process with 3-strike architectural review
- **in-detrope** -- Detrope: diagnoses LLM writing tropes and stylometric tells, assesses in project context, shows remediation plan
- **in-elixir-essentials** -- Elixir coding rules: pattern matching, tagged tuples, with-railways, @impl true, Highlander
- **in-elixir-testing** -- Elixir testing rules: strong assertions, no control flow in tests, async by default, real code over mocks, Highlander for tests
- **in-essentials** -- Core Intent workflow rules for steel threads, agents, treeindex, and session management
- **in-finish** -- Session finish: update ST docs, wip.md, restart.md, commit cleanly
- **in-handoff** -- Session handoff: generate handoff doc for future sessions
- **in-next** -- Next step: review state, identify smallest coherent work unit, wait for go
- **in-phoenix-liveview** -- Phoenix LiveView rules: two-phase mount, streams for lists, thin LiveViews, @impl true on callbacks
- **in-plan** -- Planning kickoff: show workplan, invoke coding skills, enforce rules before coding
- **in-review** -- Two-stage code review: spec compliance then rule-library compliance via critic-<lang>
- **in-session** -- Session bootstrap: auto-load coding skills for the detected project language after context reset or compact
- **in-standards** -- Coding standards: agnostic principles (Highlander, PFIC, Thin Coordinator, No Silent Errors) + project rules
- **in-start** -- Session start: read restart files, review STs, orientation overview before coding
- **in-tca-audit** -- TCA audit: execute component audits with sub-agents, track progress, manage context
- **in-tca-finish** -- TCA finish: final verification, ST doc updates, feedback report generation, and session wrap-up
- **in-tca-init** -- TCA init: provision steel thread, rule set, component map, and work packages for a Total Codebase Audit
- **in-tca-remediate** -- TCA remediate: execute prioritized fix batches in main conversation with compile/test gates
- **in-tca-synthesize** -- TCA synthesize: cross-component deduplication, priority classification, and fix batch ordering
- **in-verify** -- Verification gate: require fresh evidence before any completion claim
- **in-whiteboard** -- Multi-session coordination via intent/whiteboard/<node>/: per-node boards + single-writer inboxes, claim ST scopes, broadcast, heartbeat, release

## Installed Subagents

- **elixir** -- Elixir code doctor specializing in functional programming, Usage Rules, and framework best practices
- **intent** -- Helps manage Intent projects using steel threads methodology
- **critic-elixir** -- Elixir rule-library critic. Reads rules/elixir/ (code, test, ash, lv, phoenix) and the agnostic pack, applies each rule's Detection heuristic to target `.ex`/`.exs` files, and emits a machine-parseable report grouped by severity.
- **critic-lua** -- Lua rule-library critic. Reads rules/lua/ (code, test) and the agnostic pack, applies each rule's Detection heuristic to target `.lua` files, and emits a machine-parseable report grouped by severity.
- **critic-rust** -- Rust rule-library critic. Reads rules/rust/ (code, test) and the agnostic pack, applies each rule's Detection heuristic to target `.rs` files, and emits a machine-parseable report grouped by severity.
- **critic-shell** -- Shell (bash + zsh) rule-library critic. Reads rules/shell/ and the relevant agnostic rules, applies each rule's Detection heuristic to the target shell files, and emits a machine-parseable report grouped by severity.
- **critic-swift** -- Swift rule-library critic. Reads rules/swift/ (code, test) and the agnostic pack, applies each rule's Detection heuristic to target `.swift` files, and emits a machine-parseable report grouped by severity.
- **diogenes** -- Elixir Test Architect - Socratic dialog that produces test specifications and validates test quality
- **socrates** -- CTO Review Mode - Facilitates Socratic dialog between CTO and Tech Lead for technical decision-making

## Critic Family

Per-language rule enforcement via thin subagents that read the rule library at invocation time. Available: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`. Invoke:

```
Task(subagent_type="critic-<lang>", prompt="review <targets>")
```

The installed Intent tool's headless runner (`intent critic <lang>`, Greppable-proxy rules only; no LLM required) powers the pre-commit gate. Contract: `intent/docs/critics.md` at the Intent install. Exit codes: `0` clean, `1` findings, `2` error.

## Rule Library

The coding-rule library is served by the installed Intent tool, not vendored into this project. Access rules through the CLI below -- each rule carries YAML frontmatter, a Detection heuristic, and bad/good examples. Skills cite rule IDs; critics enforce them.

```bash
intent claude rules list        # enumerate
intent claude rules show <id>   # inspect
intent claude rules validate    # schema check
```

## Extensions

User extensions live at `~/.intent/ext/<name>/` and contribute subagents, skills, or rule packs without modifying canon. Reference extension: `worker-bee`.

```bash
intent ext list
intent ext show <name>
intent ext new <name>
```

Authoring guide: `intent/docs/writing-extensions.md` at the Intent install.

## Session Hooks

`.claude/settings.json` wires three Claude Code lifecycle hooks: **SessionStart** (inject context + `/in-session` reminder), **UserPromptSubmit** (strict gate -- block first prompt until `/in-session` runs), **Stop** (remind `/in-finish` at wrap-up). Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` at the Intent install.

## Socrates vs Diogenes FAQ

Two distinct subagents for two distinct concerns:

- **Socrates** -- CTO Review Mode. Architectural and strategic technical decision-making via Socratic dialog.
- **Diogenes** -- Elixir Test Architect. Test-specification generation and gap analysis.

They have never been the same agent. FAQ + forensic detail: `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq` at the Intent install.

## Testing Instructions

1. Run the test suite before making changes -- establish a baseline.
2. Make changes.
3. Run the suite again; confirm no regressions.
4. Add new tests for new functionality.
5. Before commit, the pre-commit critic gate checks staged files for `CRITICAL` + `WARNING` findings. See `intent/docs/pre-commit-hook.md` at the Intent install.

## Commit Guidelines

- Descriptive commit messages; reference steel thread or work package IDs where applicable.
- No AI attribution in commit messages (no `Co-Authored-By`, no tool signatures).
- Small, coherent commits preferred over large batches.

## Security Considerations

- Never commit secrets or credentials.
- Review external inputs for injection-class bugs.
- Follow the project's security policy if one exists.

## Additional Resources

At the Intent install (not this project):

- `intent/docs/working-with-llms.md` -- canon tech note (narrative).
- `intent/docs/critics.md` -- critic contract.
- `intent/docs/rules.md` -- rule library authoring guide.
- `intent/docs/writing-extensions.md` -- extension author guide.
- `intent/docs/pre-commit-hook.md` -- pre-commit critic gate install + configure.
- `intent/docs/migration-v2.10.0.md` -- v2.9.0 -> v2.10.0 migration guide (directory move + recovery).

In this project:

- `intent/llm/MODULES.md` -- module registry (Highlander enforcement).
- `CLAUDE.md` -- Claude-specific overlay.
- `usage-rules.md` -- DO / NEVER contract for this project.

---

_Generated by Intent v2.15.1 on 2026-07-07_
