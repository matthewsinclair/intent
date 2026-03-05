# Implementation - ST0026: Steel Thread Zero

## Implementation Status

Phase 1 (WP-01 through WP-05 + WP-11) complete. Phase 2 in progress: WP-06 done.

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
| 8     | WP-07 | Health Check & Learnings | D7, D10      | Not Started |
| 9     | WP-08 | Guardrails               | D9, D11      | Not Started |
| 10    | WP-09 | Retrofit Installation    | D12          | Not Started |
| 11    | WP-10 | Integrator Command       | D1           | Not Started |

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

## Test Status

All 382 tests passing across 19 BATS test files.
