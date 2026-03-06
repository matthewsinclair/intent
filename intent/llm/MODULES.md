# Module Registry - Intent

> **The Highlander Rule**: There can be only one module per concern.
> ALWAYS check this file before creating a new script or helper function.
> If a module already owns that concern, use it.
> When you must create a new module, register it here FIRST, then create the file.

## Core CLI

| Concern              | THE Module             | Notes                                               |
| -------------------- | ---------------------- | --------------------------------------------------- |
| Main entry point     | `bin/intent`           | Command dispatch, global commands, INTENT_HOME      |
| Shared helpers       | `bin/intent_helpers`   | error(), checksum, terminal width, get_config_field |
| Project init         | `bin/intent_init`      | Creates new Intent projects from templates          |
| Steel thread mgmt    | `bin/intent_st`        | new, list, show, start, done, cancel, organize      |
| Work package mgmt    | `bin/intent_wp`        | new, list, show, start, done                        |
| Treeindex generation | `bin/intent_treeindex` | Shadow directory index generation with Claude       |
| File indexing        | `bin/intent_fileindex` | File listing, checkbox state tracking               |
| Health checks        | `bin/intent_doctor`    | Dependency checks, config validation                |
| Plugin discovery     | `bin/intent_plugin`    | list, show, help for installed plugins              |
| Audit command        | `bin/intent_audit`     | quick, health, help; Elixir credo integration       |
| Learn command        | `bin/intent_learn`     | Capture project learnings for future sessions       |
| Module registry      | `bin/intent_modules`   | check, find; MODULES.md enforcement                 |
| STZero retrofit      | `bin/intent_st_zero`   | Brownfield project retrofit installation            |
| Help display         | `bin/intent_help`      | General and command-specific help display           |
| Config loading       | `bin/intent_config`    | PROJECT_ROOT detection, load_intent_config          |
| Project info         | `bin/intent_info`      | Status display, no-args default                     |
| LLM guidance         | `bin/intent_llm`       | Display LLM-specific guidance files                 |
| Organise             | `bin/intent_organise`  | Organize steel threads by status                    |
| Upgrade              | `bin/intent_upgrade`   | STP to Intent migration                             |
| Minimal bootstrap    | `bin/intent_minimal`   | Minimal no-dependency bootstrap                     |
| Main (legacy)        | `bin/intent_main`      | Legacy entry point                                  |
| Bootstrap            | `bin/intent_bootstrap` | First-run setup, PATH instructions                  |

## Plugin: Claude

| Concern               | THE Module                                           | Notes                                  |
| --------------------- | ---------------------------------------------------- | -------------------------------------- |
| Subagent lifecycle    | `intent/plugins/claude/bin/intent_claude_subagents`  | install, sync, uninstall, show, status |
| Skill lifecycle       | `intent/plugins/claude/bin/intent_claude_skills`     | install, sync, uninstall, show, list   |
| Project upgrade       | `intent/plugins/claude/bin/intent_claude_upgrade`    | LLM guidance file upgrades             |
| Memory injection      | `intent/plugins/claude/bin/intent_claude_prime`      | prime MEMORY.md with project knowledge |
| Plugin shared library | `intent/plugins/claude/lib/claude_plugin_helpers.sh` | install/sync/uninstall/manifest logic  |

## Plugin: Agents

| Concern             | THE Module                                | Notes                          |
| ------------------- | ----------------------------------------- | ------------------------------ |
| AGENTS.md lifecycle | `intent/plugins/agents/bin/intent_agents` | init, generate, sync, validate |

## Templates

| Concern                   | THE Module                                              | Notes                                   |
| ------------------------- | ------------------------------------------------------- | --------------------------------------- |
| CLAUDE.md template        | `lib/templates/llm/_CLAUDE.md`                          | Single source -- used by init + upgrade |
| Module registry           | `lib/templates/llm/_MODULES.md`                         | Highlander registry template            |
| Decision tree             | `lib/templates/llm/_DECISION_TREE.md`                   | Code placement guide template           |
| Archetypes reference      | `lib/templates/llm/_ARCHETYPES.md`                      | Archetype listing template              |
| Elixir archetypes         | `lib/templates/archetypes/elixir/`                      | 9 .ex.eex code templates                |
| ST info template          | `lib/templates/prj/st/ST####/info.md`                   | Steel thread frontmatter                |
| WP info template          | `lib/templates/prj/st/WP/info.md`                       | Work package frontmatter                |
| WIP template              | `lib/templates/prj/_wip.md`                             | Work-in-progress file                   |
| Dependency graph template | `lib/templates/llm/_DEPENDENCY_GRAPH.md`                | Dependency rules template               |
| D11 Credo check           | `lib/templates/credo_checks/elixir/dependency_graph.ex` | Cross-app dependency enforcement        |
| Hook template             | `lib/templates/hooks/module_check_hook.json`            | Advisory write hook for Claude Code     |
| Credo check templates     | `lib/templates/credo_checks/elixir/`                    | 7 checks: R2, R6, R7, R8, R11, R15, D11 |

## Help Files

| Concern      | THE Module                   | Notes                 |
| ------------ | ---------------------------- | --------------------- |
| Claude help  | `lib/help/claude.help.md`    | Claude plugin help    |
| Agents help  | `lib/help/agents.help.md`    | Agents plugin help    |
| Plugin help  | `lib/help/plugin.help.md`    | Plugin discovery help |
| WP help      | `lib/help/wp.help.md`        | Work package help     |
| Fileindex    | `lib/help/fileindex.help.md` | Fileindex help        |
| Audit help   | `lib/help/audit.help.md`     | Audit command help    |
| Learn help   | `lib/help/learn.help.md`     | Learn command help    |
| Modules help | `lib/help/modules.help.md`   | Modules command help  |
| STZero help  | `lib/help/stzero.help.md`    | STZero command help   |

## Tests

| Concern              | THE Module                         | Notes                          |
| -------------------- | ---------------------------------- | ------------------------------ |
| Test helper/fixtures | `tests/lib/test_helper.bash`       | Shared setup, assertions       |
| Test runner          | `tests/run_tests.sh`               | Discovers and runs all BATS    |
| Audit tests          | `tests/unit/audit_commands.bats`   | BATS tests for audit command   |
| Learn tests          | `tests/unit/learn_commands.bats`   | BATS tests for learn command   |
| Modules tests        | `tests/unit/modules_commands.bats` | BATS tests for modules command |
| STZero tests         | `tests/unit/st_zero_commands.bats` | BATS tests for st zero command |
