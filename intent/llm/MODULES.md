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
| Bootstrap            | `bin/intent_bootstrap` | First-run setup, PATH instructions                  |

## Plugin: Claude

| Concern               | THE Module                                           | Notes                                  |
| --------------------- | ---------------------------------------------------- | -------------------------------------- |
| Subagent lifecycle    | `intent/plugins/claude/bin/intent_claude_subagents`  | install, sync, uninstall, show, status |
| Skill lifecycle       | `intent/plugins/claude/bin/intent_claude_skills`     | install, sync, uninstall, show, list   |
| Project upgrade       | `intent/plugins/claude/bin/intent_claude_upgrade`    | LLM guidance file upgrades             |
| Plugin shared library | `intent/plugins/claude/lib/claude_plugin_helpers.sh` | install/sync/uninstall/manifest logic  |

## Plugin: Agents

| Concern             | THE Module                                | Notes                          |
| ------------------- | ----------------------------------------- | ------------------------------ |
| AGENTS.md lifecycle | `intent/plugins/agents/bin/intent_agents` | init, generate, sync, validate |

## Templates

| Concern              | THE Module                            | Notes                                   |
| -------------------- | ------------------------------------- | --------------------------------------- |
| CLAUDE.md template   | `lib/templates/llm/_CLAUDE.md`        | Single source -- used by init + upgrade |
| Module registry      | `lib/templates/llm/_MODULES.md`       | Highlander registry template            |
| Decision tree        | `lib/templates/llm/_DECISION_TREE.md` | Code placement guide template           |
| Archetypes reference | `lib/templates/llm/_ARCHETYPES.md`    | Archetype listing template              |
| Elixir archetypes    | `lib/templates/archetypes/elixir/`    | 9 .ex.eex code templates                |
| ST info template     | `lib/templates/prj/st/ST####/info.md` | Steel thread frontmatter                |
| WP info template     | `lib/templates/prj/st/WP/info.md`     | Work package frontmatter                |
| WIP template         | `lib/templates/prj/_wip.md`           | Work-in-progress file                   |

## Help Files

| Concern      | THE Module                   | Notes                 |
| ------------ | ---------------------------- | --------------------- |
| General help | `lib/help/general.help.md`   | Top-level help output |
| Claude help  | `lib/help/claude.help.md`    | Claude plugin help    |
| Agents help  | `lib/help/agents.help.md`    | Agents plugin help    |
| Plugin help  | `lib/help/plugin.help.md`    | Plugin discovery help |
| WP help      | `lib/help/wp.help.md`        | Work package help     |
| Fileindex    | `lib/help/fileindex.help.md` | Fileindex help        |

## Tests

| Concern              | THE Module                   | Notes                       |
| -------------------- | ---------------------------- | --------------------------- |
| Test helper/fixtures | `tests/lib/test_helper.bash` | Shared setup, assertions    |
| Test runner          | `tests/run_tests.sh`         | Discovers and runs all BATS |
