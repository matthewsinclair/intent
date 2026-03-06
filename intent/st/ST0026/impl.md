# Implementation - ST0026: Steel Thread Zero

## Implementation Status

Phase 1 (WP-01 through WP-05 + WP-11) complete. Phase 2 in progress: WP-06 through WP-09 done.

## Execution Order

| Order | WP    | Title                    | Deliverables | Status      |
| ----- | ----- | ------------------------ | ------------ | ----------- |
| 1     | WP-01 | Skill Rename             | D13          | Done        |
| 2     | WP-03 | LLM Templates            | D2, D3, D6   | Done        |
| 3     | WP-05 | Archetype Templates      | D4           | Done        |
| 4     | WP-02 | Workflow Skills          | D14          | Done        |
| 5     | WP-04 | Memory Injection         | D8           | Done        |
| 6     | WP-11 | TN004 Tech Note          | --           | Done        |
| 7     | WP-06 | Automated Enforcement    | D5a, D5b     | Done        |
| 8     | WP-07 | Health Check & Learnings | D7, D10      | Done        |
| 9     | WP-08 | Guardrails               | D9, D11      | Done        |
| 10    | WP-09 | Retrofit Installation    | D12          | Done        |
| 11    | WP-10 | Integrator Command       | D1           | Done        |

## As-Built Notes

### WP-01: Skill Rename (Done)

Renamed 6 skill directories from `intent-*` to `in-*` prefix. Updated ~248 references across 14 files (SKILL.md internals, help files, docs, tests). Added rename migration logic to `claude_plugin_helpers.sh` so `intent claude skills sync` automatically detects old `intent-*` installs and migrates them to `in-*`.

**Key files changed:**

- `intent/plugins/claude/skills/` -- 6 directories renamed
- `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- rename migration in `plugin_sync()`
- `tests/unit/skills_commands.bats` -- ~86 occurrences updated + new migration test
- `tests/unit/test_autopsy.bats` -- ~61 occurrences updated
- `tests/unit/test_diogenes.bats` -- ~41 occurrences updated
- `bin/intent_helpers`, `bin/intent_doctor`, `lib/help/claude.help.md`

**Commit:** `3962972 ST0026/WP-01: rename skills from intent-* to in-* prefix`

### WP-03: LLM Templates (Done)

Rewrote `lib/templates/llm/_CLAUDE.md` from basic ~100 lines to comprehensive template with full rule set, key references, session workflow. Created `lib/templates/llm/_MODULES.md` (Highlander registry template) and `lib/templates/llm/_DECISION_TREE.md` (code placement guide).

Fixed a Highlander violation: three copies of CLAUDE.md content (template, `create_claude_md()` in `bin/intent_helpers`, inline heredoc in `bin/intent_init`) consolidated to single template with sed substitution.

Updated `bin/intent_init` to also create MODULES.md and DECISION_TREE.md from templates during project init.

Created Intent's own `intent/llm/MODULES.md` (26 modules across 6 sections) and `intent/llm/DECISION_TREE.md` (Bash CLI decision tree).

**Key files changed:**

- `lib/templates/llm/_CLAUDE.md` -- complete rewrite
- `lib/templates/llm/_MODULES.md` -- new template
- `lib/templates/llm/_DECISION_TREE.md` -- new template
- `bin/intent_helpers` -- `create_claude_md()` uses template
- `bin/intent_init` -- uses template, creates MODULES.md + DECISION_TREE.md
- `intent/llm/MODULES.md` -- new, Intent's own registry
- `intent/llm/DECISION_TREE.md` -- new, Intent's own decision tree
- `CLAUDE.md` -- updated with Rules section

**Commits:** `ef46eea ST0026/WP-03+05`, `e86b0ba ST0026/WP-03: add MODULES.md and DECISION_TREE.md for Intent itself`

### WP-05: Archetype Templates (Done)

Created 9 Elixir archetype templates in `lib/templates/archetypes/elixir/`:
`ash_domain.ex.eex`, `ash_resource.ex.eex`, `phoenix_controller.ex.eex`, `live_view.ex.eex`, `service.ex.eex`, `cli_command.ex.eex`, `genserver.ex.eex`, `oban_worker.ex.eex`, `test.ex.eex`.

Created `lib/templates/llm/_ARCHETYPES.md` reference doc listing all archetypes with usage guidance.

Each template includes `@impl true` annotations, thin coordinator comments, MODULES.md reminders.

**Commit:** `ef46eea ST0026/WP-03+05` (combined with WP-03)

### WP-02: Workflow Skills (Done)

Created 5 new procedural skills: `in-start`, `in-plan`, `in-standards`, `in-next`, `in-finish`. Each with SKILL.md following established pattern (frontmatter with description, `## Procedure` section with numbered steps).

Total skills now: 11 (6 renamed + 5 new).

**Commit:** `df2aec5 ST0026/WP-02: add 5 workflow skills`

### WP-04: Memory Injection (Done)

Created `intent claude prime` command (195 lines) at `intent/plugins/claude/bin/intent_claude_prime`. Supports `--refresh`, `--dry-run`, `--from <project>` flags. Synthesizes operational knowledge, project rules, MODULES.md, DECISION_TREE.md, ARCHETYPES.md, learnings into structured MEMORY.md.

Created `lib/templates/prime/operational-knowledge.md` (bundled Intent operational knowledge).

Key design decision: plugin commands detect PROJECT_ROOT locally since `bin/intent` exec's them without loading `intent_config`.

Enforces 200-line limit with truncation warning.

**Key files changed:**

- `intent/plugins/claude/bin/intent_claude_prime` -- new command
- `lib/templates/prime/operational-knowledge.md` -- new bundled knowledge
- `bin/intent` -- dispatch for `prime` subcommand
- `intent/plugins/claude/plugin.json` -- added prime command
- `lib/help/claude.help.md` -- added prime section
- `tests/unit/plugin_commands.bats` -- updated command count

**Commit:** `50b8484 ST0026/WP-04: add intent claude prime for memory injection`

### WP-11: TN004 Tech Note (Done)

Ported total codebase audit tech note from laksa-web. Generalized all project-specific references (ST0058, ST0098, laksa-web, Lamplight) to generic examples (Example A, Example B). Added Prerequisites section and ST0026 cross-references. Placed at `intent/docs/total-codebase-audit.md` (917 lines).

**Commit:** `770b7db ST0026/WP-11: add total codebase audit tech note`

## Challenges and Solutions

### Rename migration for installed skills

Users with old `intent-*` skills installed at `~/.claude/skills/` needed automatic migration. Added detection logic to `plugin_sync()` that recognizes the `intent-* -> in-*` rename pattern and performs uninstall/reinstall in place.

### Highlander violation in CLAUDE.md generation

Three copies of CLAUDE.md content existed: template file, `create_claude_md()` inline heredoc, `bin/intent_init` inline heredoc. Consolidated all three to use the single template with sed variable substitution.

### Cross-platform checksum in tests

The rename migration test used `md5 -q` which is macOS-only. CI on Ubuntu failed. Fixed by using a fake checksum string since the exact value doesn't matter for the migration test.

### Plugin command PROJECT_ROOT detection

`bin/intent` exec's plugin commands WITHOUT loading `intent_config`, so PROJECT_ROOT isn't set. `intent_claude_prime` detects it locally by checking for `.intent/config.json` in the current directory.

### Brittle count assertions in tests

Adding the prime command changed "Commands (3):" to "Commands (4):" in `plugin show claude` output. This is the kind of brittle assertion that `/in-elixir-testing` warns against -- testing exact counts instead of behavior. Happened again with WP-06 (4->5).

## Phase 2 As-Built Notes

### WP-06: Automated Enforcement (Done)

Created 6 custom Credo check templates in `lib/templates/credo_checks/elixir/` and `bin/intent_audit` command (~250 lines). The audit command auto-copies check templates into the target project's `lib/mix/checks/` on first run, then delegates to `mix credo`.

Bash 3.x compatibility: replaced `declare -A` (associative arrays, bash 4+ only) with a `case` statement for rule-to-template mapping. macOS ships bash 3.x.

Added v2.5.0->v2.6.0 upgrade path (version-stamp-only, no directory creation).

**Files created (9):**

- `lib/templates/credo_checks/elixir/boolean_operators.ex` -- R8
- `lib/templates/credo_checks/elixir/missing_impl_annotation.ex` -- R11
- `lib/templates/credo_checks/elixir/debug_artifacts.ex` -- R15
- `lib/templates/credo_checks/elixir/map_get_on_struct.ex` -- R7
- `lib/templates/credo_checks/elixir/thick_coordinator.ex` -- R2
- `lib/templates/credo_checks/elixir/highlander_suspect.ex` -- R6
- `bin/intent_audit` -- main audit command
- `lib/help/audit.help.md` -- help file
- `tests/unit/audit_commands.bats` -- 17 tests

**Files modified (7):**

- `bin/intent_helpers` -- `needs_v2_6_0_upgrade()`, `migrate_v2_5_0_to_v2_6_0()`
- `bin/intent_upgrade` -- wired v2.6.0 into all upgrade paths
- `bin/intent` -- audit dispatch case
- `bin/intent_help` -- audit in help display + skip list
- `intent/plugins/claude/plugin.json` -- audit command entry
- `intent/llm/MODULES.md` -- registered 4 new entries
- `tests/unit/plugin_commands.bats` -- command count 4->5

**Commit:** `3aa8aa6 ST0026/WP-06: audit command and Credo check templates`

### WP-07: Health Check & Learnings (Done)

Created `bin/intent_learn` (~170 lines) for capturing project learnings with 3 categories (footgun/worked/failed). Learnings stored in `.intent/learnings.md` with date-prefixed entries under section headers. Already integrated with `intent claude prime` (reads `.intent/learnings.md` at line 134).

Added `cmd_health()` to `bin/intent_audit` (~200 lines added, total ~500 lines). Health check runs 4 checks: MODULES.md coverage, thick coordinators, Highlander suspects, and Credo status. Supports `--report` (save markdown to `intent/audit/YYYYMMDD-health.md`), `--diff` (git-based changed files only), and timestamp tracking in `.intent/last-health-check`.

**Files created (3):**

- `bin/intent_learn` -- learn command (footgun/worked/failed categories)
- `lib/help/learn.help.md` -- help file
- `tests/unit/learn_commands.bats` -- 18 tests

**Files modified (6):**

- `bin/intent_audit` -- added `cmd_health()` + 4 health check functions
- `lib/help/audit.help.md` -- added health subcommand docs
- `bin/intent` -- learn dispatch case
- `bin/intent_help` -- learn in help display + skip list
- `intent/llm/MODULES.md` -- registered learn command + tests
- `tests/unit/audit_commands.bats` -- 7 additional health tests (17->24)

**Commit:** `39dda74 ST0026/WP-07: audit health + learn commands`

### WP-08: Guardrails (Done)

Created `bin/intent_modules` (~230 lines) for module registry guardrails. The `check` subcommand compares MODULES.md registry entries against the filesystem, reporting unregistered files (`+ path`) and stale entries (`- path`). The `find` subcommand searches the registry by keyword. Scanner intelligently skips individual files under directory-registered entries (e.g., won't flag `dependency_graph.ex` when `lib/templates/credo_checks/elixir/` is registered).

Created dependency graph Credo check template (`lib/templates/credo_checks/elixir/dependency_graph.ex`, ~160 lines). Uses `run_on_all: true` pattern (like `highlander_suspect.ex`). Reads rules from `intent/llm/DEPENDENCY_GRAPH.md`, walks AST for alias/import/use, infers target app via `Macro.underscore`, reports forbidden cross-app dependencies. Integrated as D11 rule in `intent audit quick`.

Created advisory Claude Code hook template (`lib/templates/hooks/module_check_hook.json`) for projects to install -- not installed in Intent's own settings.

Also rationalized CLI output across all 14+ source scripts and plugin helpers to Rust-style conventions: lowercase status prefixes (`ok:`, `error:`, `warning:`, `hint:`), action prefixes (`created:`, `updated:`, `removed:`), no separator bars or banners. Added `--help`/`-h` flag support to `intent st`. Registered 8 previously-missing bin scripts and removed 1 stale entry in MODULES.md.

**Files created (5):**

- `bin/intent_modules` -- modules command (check, find, help)
- `lib/templates/hooks/module_check_hook.json` -- advisory hook template
- `lib/templates/llm/_DEPENDENCY_GRAPH.md` -- dependency rules template
- `lib/templates/credo_checks/elixir/dependency_graph.ex` -- D11 Credo check
- `lib/help/modules.help.md` -- help file
- `tests/unit/modules_commands.bats` -- 19 tests

**Files modified (14+ for output rationalization):**

- `bin/intent` -- modules dispatch + lowercase output
- `bin/intent_help` -- modules in Core list + skip list
- `bin/intent_audit` -- D11 rule + output rationalization
- `lib/help/audit.help.md` -- D11 rule documentation
- `intent/plugins/claude/plugin.json` -- modules command entry
- `intent/llm/MODULES.md` -- registered new modules + 8 missing scripts, removed stale entry
- `bin/intent_st` -- `--help` flag + output rationalization
- `bin/intent_wp`, `bin/intent_learn`, `bin/intent_doctor`, `bin/intent_bootstrap` -- output rationalization
- `bin/intent_organise`, `bin/intent_upgrade`, `bin/intent_llm`, `bin/intent_fileindex`, `bin/intent_treeindex` -- output rationalization
- `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- output rationalization
- `tests/README.md` -- modules_commands.bats entry
- 14 test files updated for new output assertions

**Commits:**

- `f03e75a` -- guardrails + rationalized CLI output (41 files)
- `23a516c` -- docs updated to as-built state (11 files)
- `92e41f1` -- fix CI: configure git identity in audit health test
- `1411655` -- fix: lowercase health tags `[skip]`/`[warn]`/`[ok]`, Credo count `grep -c` bug, escaped pipes in doc tables

### WP-09: Retrofit Installation (Done)

Created `bin/intent_st_zero` (~430 lines) for brownfield project retrofitting. 4-phase process: audit each of 9 ST0000 deliverables (D2-D11), display gap analysis with `[present]`/`[missing]`/`[partial]` status, generate proposals, apply with per-deliverable output. Dispatched from `bin/intent_st` via `zero` subcommand.

Module auto-discovery (D3) scans `lib/` for `.ex` files, extracts `defmodule` declarations, classifies by path (controller, LiveView, component, service, schema, Ash resource/domain, worker, mix task). Umbrella-aware: detects `apps/` directory and generates per-app sections in MODULES.md.

Also made `audit health` umbrella-aware: `get_lib_dirs()` helper returns `apps/*/lib/` directories for umbrella projects. All 3 health checks (modules coverage, thick coordinators, Highlander suspects) now scan umbrella apps. Added `lib/mix/checks/` exclusion from unregistered modules check (Intent-managed Credo templates). Added `run` to common callback names. Reformatted Highlander suspects output to multi-line format. Fixed `--checks-only` to force-overwrite existing templates.

Fixed 3 compiler warnings in Credo check templates: `@default_params` interpolation before definition (hardcoded value), `_arity` underscore mismatch, unused `@debug_calls` attribute.

Tested against 3 real projects: Laksa (380 modules), Conflab (150 modules), Lamplight umbrella (751 modules across 5 apps).

**Files created (3):**

- `bin/intent_st_zero` -- st zero command (audit, install, help)
- `lib/help/stzero.help.md` -- help file
- `tests/unit/st_zero_commands.bats` -- 31 tests

**Files modified (8):**

- `bin/intent_st` -- zero dispatch case
- `bin/intent_help` -- st_zero in skip list
- `bin/intent_audit` -- umbrella support, Highlander format, Credo exclusion, --checks-only force-copy
- `lib/templates/credo_checks/elixir/thick_coordinator.ex` -- fix @default_params warning
- `lib/templates/credo_checks/elixir/highlander_suspect.ex` -- fix _arity warning
- `lib/templates/credo_checks/elixir/debug_artifacts.ex` -- remove unused @debug_calls
- `intent/llm/MODULES.md` -- registered 3 new entries
- `tests/README.md` -- st_zero_commands.bats entry

**Other:** Moved `tests/core_functionality.bats` to `tests/integration/`

**Commit:** `e50da0a ST0026/WP-09: st zero retrofit + umbrella-aware audit health`

## Test Status

### WP-10: Integrator Command (Done)

Added `--with-st0000` flag to `bin/intent_init`. After standard project initialization, runs `intent st zero install` to bootstrap all ST0000 deliverables. Reuses all WP-09 logic -- no duplication.

**Files modified (2):**

- `bin/intent_init` -- `--with-st0000` flag parsing, st zero install call
- `tests/unit/init_commands.bats` -- 4 new tests (8->12)

**Commit:** TBD

## Test Status

All 462 tests passing across 22 BATS test files.
