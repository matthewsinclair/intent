# ARCHITECTURE.md

System architecture for Intent v2.10.0. Pair this with `intent/llm/MODULES.md` (the Highlander registry) and `intent/llm/DECISION_TREE.md` (code-placement flow chart).

## System overview

Intent is a Bash CLI for managing software projects through "steel threads" -- coherent, finite slices of work bracketed by an `info.md` spec, optional `design.md` / `impl.md` companions, and one or more `WP/<NN>/info.md` work packages. The CLI ships with a plugin system (`intent claude *`, `intent agents *`) and a canon of LLM-collaboration artefacts (`AGENTS.md`, `CLAUDE.md`, `usage-rules.md`, `.claude/settings.json`, pre-commit critic gate).

The whole system is bash. There is no compile step, no language runtime to install, no service to run. `intent` is a single dispatcher script that sources shared helpers and routes to subcommand binaries.

## Core layout

| Directory                | Purpose                                                                                                                                             |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bin/`                   | Top-level CLI commands (`intent`, `intent_helpers`, `intent_st`, `intent_wp`, `intent_treeindex`, `intent_critic`, `intent_doctor`, ...).           |
| `bin/intent_helpers`     | Shared bash library: `error()`, `info()`, version detection, project-root resolution, layout-aware migrations.                                      |
| `intent/plugins/claude/` | Claude Code plugin: subagents, skills, rule library (`rules/agnostic/`, `rules/<lang>/`), canon installer (`bin/intent_claude_upgrade`), templates. |
| `intent/plugins/agents/` | AGENTS.md generator + per-language `_default` and language-specific templates (`templates/_default/`, `templates/elixir/`, ...).                    |
| `lib/templates/`         | Single source for all generated content -- AGENTS / CLAUDE / usage-rules templates, hook scripts, settings.json, .treeindexignore.                  |
| `intent/st/`             | Steel threads. Active under `intent/st/STxxxx/`; closed threads move to `intent/st/COMPLETED/STxxxx/` via `intent st done`.                         |
| `intent/.config/`        | Project configuration. `config.json` carries `intent_version`, `project_name`, `author`. `cache/` and `backup/` are tooling-local.                  |
| `intent/.treeindex/`     | Tree summaries for fast LLM exploration. Generated per-subdirectory by `intent treeindex <dir>`. `.treeindexignore` excludes cache/backup.          |
| `intent/llm/`            | LLM-facing project documents: `MODULES.md`, `DECISION_TREE.md`, `RULES.md`, `ARCHITECTURE.md`. Read by every LLM session.                           |
| `tests/`                 | BATS test suite. `tests/lib/test_helper.bash` provides shared fixtures; `tests/run_tests.sh` is the runner.                                         |

## Key patterns

### Thin coordinator dispatch

`bin/intent` is a small dispatcher. It parses one argument (the subcommand), delegates to `bin/intent_<subcommand>`, and renders nothing of its own. Every subcommand binary follows the same shape: parse args, source helpers, call into modules under `bin/` or `intent/plugins/`, return.

This is `IN-AG-THIN-COORD-001` (Thin Coordinator) concretised for shell. The dispatcher itself never grew business logic, even when it would have been "easier" to inline.

### Plugin callback architecture

Each plugin (`claude`, `agents`) lives under `intent/plugins/<name>/` with a stable interface: 4 config vars (`PLUGIN_NAME`, `PLUGIN_BIN_DIR`, `PLUGIN_TEMPLATES_DIR`, `PLUGIN_RULES_DIR`) and 8 callbacks (`plugin_install_*`, `plugin_sync_*`, `plugin_verify_*`, ...). The shared sourcing layer in `bin/intent_helpers` discovers and dispatches plugins uniformly.

### Single template source

All generated content -- AGENTS.md, CLAUDE.md, settings.json, hook scripts, `_treeindexignore`, ext seeds -- comes from `lib/templates/` via `sed` substitution. There are no parallel inline heredocs duplicating template content. This is `IN-AG-HIGHLANDER-001` (Highlander) concretised: one source, many use sites.

### Layout-keyed idempotence

Migrations key on the **layout state** (which directories exist), not the version stamp alone. A project stamped 2.10.0 but still at the `.intent/` layout (a state that arose during ST0036 mid-development) is detected and migrated. See `bin/intent_helpers:needs_v2_10_0_upgrade` and the dispatcher early-exit in `bin/intent_upgrade`. Stamp-only checks miss this class of bug; layout-keyed checks catch it.

### Canon-apply via the canon-installer

`intent claude upgrade [--apply]` is a three-phase installer:

1. **Diagnose** -- probe LLM guidance files, deprecated artefacts, deprecated skill names, installed subagents, installed skills, canon artefacts. Each probe enqueues an action when drift is detected.
2. **Plan** -- print the action set + manual-review warnings.
3. **Apply** -- execute each action. Phase 3 is a no-op when no actions are enqueued (idempotence by construction).

Auto-insert mechanisms (such as the pre-commit chain block) use marker pairs (`intent-chain-block:start` / `:end`) so re-application detects "already done" and skips.

### Steel thread lifecycle

`intent st new "Title"` creates `intent/st/STxxxx/info.md` with frontmatter (`status: Not Started`, dates, author). The CLI is the single mutator -- direct file edits to `status:` are forbidden by `IN-AG-HIGHLANDER-001`. Lifecycle: `Not Started -> WIP -> Done -> Completed`. Closed STs move to `intent/st/COMPLETED/STxxxx/`; cancelled STs use the existing `Cancelled` status with an inline deprecation note. Work packages (`WP/NN/info.md`) follow the same pattern via `intent wp new|start|done`.

## Hook architecture (Claude Code session lifecycle)

`.claude/settings.json` wires three lifecycle hooks shipped with the canon:

- **SessionStart** (`session-context.sh`) -- prints project context, current ST, git head, WIP. Writes a per-project state file at `/tmp/intent-claude-session-current-id-<project-key>` that the gate uses to scope sentinels.
- **UserPromptSubmit** (`require-in-session.sh`) -- strict gate: blocks the first prompt until `/in-session` has run. Released cooperatively when the skill writes `/tmp/intent/in-session-<UUID>.sentinel`.
- **Stop** (`echo`) -- prints the `/in-finish` reminder at session wrap.

This pattern means every coding session starts with `/in-session`, which loads the right language skills and Highlander/PFIC/Thin-Coordinator/No-Silent-Errors discipline before any code is written.

## Critic dispatch

`bin/intent critic <lang> <paths>` dispatches to a per-language critic subagent (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`). Each critic reads its rule library at invocation, applies Detection heuristics, and emits a machine-parseable report.

`bin/intent_critic` is the headless variant -- same dispatch, no LLM, intended for the pre-commit gate. The gate is wired via `.git/hooks/pre-commit` (canon hook) or via the chain block in an existing pre-commit. Severity threshold reads from `.intent_critic.yml`.

## Migration history

| Version | Notable change                                                                                                                                                                                                     |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| v2.0.0  | STP renamed to Intent. Plugin system introduced.                                                                                                                                                                   |
| v2.8.0  | Steel thread CLI consolidated; per-WP work packages.                                                                                                                                                               |
| v2.9.0  | Critics-as-subagents; rule-library schema; pre-commit critic gate; `~/.intent/ext/` extension surface.                                                                                                             |
| v2.10.0 | Canonical LLM config (AGENTS.md / CLAUDE.md overlay / usage-rules.md); session hooks; pre-commit chain auto-insert; `.intent/` relocated to `intent/.config/` (preserves rename history under `git log --follow`). |

Per-version detail is in `CHANGELOG.md`. Steel threads under `intent/st/COMPLETED/` carry the as-built record.

## Where to read next

- New module? `intent/llm/MODULES.md` first; `intent/llm/DECISION_TREE.md` for placement.
- New rule? `intent/docs/rules.md` for the schema and Detection contract.
- New plugin or extension? `intent/docs/writing-extensions.md`.
- New ST or WP? Use the CLI (`intent st new`, `intent wp new`); do not hand-roll directories.
