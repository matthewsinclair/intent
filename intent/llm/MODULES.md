# Module Registry - Intent

> **The Highlander Rule**: There can be only one module per concern.
> ALWAYS check this file before creating a new script or helper function.
> If a module already owns that concern, use it.
> When you must create a new module, register it here FIRST, then create the file.

## Core CLI

| Concern                 | THE Module             | Notes                                               |
| ----------------------- | ---------------------- | --------------------------------------------------- |
| Main entry point        | `bin/intent`           | Command dispatch, global commands, INTENT_HOME      |
| Shared helpers          | `bin/intent_helpers`   | error(), checksum, terminal width, get_config_field |
| Project init            | `bin/intent_init`      | Creates new Intent projects from templates          |
| Steel thread mgmt       | `bin/intent_st`        | new, list, show, start, done, cancel, organize      |
| Work package mgmt       | `bin/intent_wp`        | new, list, show, start, done                        |
| Treeindex generation    | `bin/intent_treeindex` | Shadow directory index generation with Claude       |
| File indexing           | `bin/intent_fileindex` | File listing, checkbox state tracking               |
| Health checks           | `bin/intent_doctor`    | Dependency checks, config validation                |
| Plugin discovery        | `bin/intent_plugin`    | list, show, help for installed plugins              |
| Audit command           | `bin/intent_audit`     | quick, health, help; Elixir credo integration       |
| Learn command           | `bin/intent_learn`     | Capture project learnings for future sessions       |
| Module registry         | `bin/intent_modules`   | check, find; MODULES.md enforcement                 |
| STZero retrofit         | `bin/intent_st_zero`   | Brownfield project retrofit installation            |
| Help display            | `bin/intent_help`      | General and command-specific help display           |
| Config loading          | `bin/intent_config`    | PROJECT_ROOT detection, load_intent_config          |
| Project info            | `bin/intent_info`      | Status display, no-args default                     |
| LLM guidance            | `bin/intent_llm`       | Display LLM-specific guidance files                 |
| Organise                | `bin/intent_organise`  | Organize steel threads by status                    |
| Upgrade                 | `bin/intent_upgrade`   | STP to Intent migration                             |
| Minimal bootstrap       | `bin/intent_minimal`   | Minimal no-dependency bootstrap                     |
| Main (legacy)           | `bin/intent_main`      | Legacy entry point                                  |
| Bootstrap               | `bin/intent_bootstrap` | First-run setup, PATH instructions                  |
| Extension mgmt (v2.9.0) | `bin/intent_ext`       | list, show, validate, new (WP02)                    |

## Plugin: Claude

| Concern                   | THE Module                                           | Notes                                                                                                  |
| ------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Subagent lifecycle        | `intent/plugins/claude/bin/intent_claude_subagents`  | install, sync, uninstall, show, status                                                                 |
| Skill lifecycle           | `intent/plugins/claude/bin/intent_claude_skills`     | install, sync, uninstall, show, list                                                                   |
| Rule library CLI (v2.9.0) | `intent/plugins/claude/bin/intent_claude_rules`      | list, show, validate, index (WP02)                                                                     |
| Project upgrade           | `intent/plugins/claude/bin/intent_claude_upgrade`    | LLM guidance file upgrades                                                                             |
| Memory injection          | `intent/plugins/claude/bin/intent_claude_prime`      | prime MEMORY.md with project knowledge                                                                 |
| Plugin shared library     | `intent/plugins/claude/lib/claude_plugin_helpers.sh` | install/sync/uninstall/manifest logic; `plugin_get_source_roots` + `plugin_resolve_source_file` (WP02) |

## Plugin: Agents

| Concern             | THE Module                                | Notes                          |
| ------------------- | ----------------------------------------- | ------------------------------ |
| AGENTS.md lifecycle | `intent/plugins/agents/bin/intent_agents` | init, generate, sync, validate |

## Templates

| Concern                   | THE Module                                              | Notes                                    |
| ------------------------- | ------------------------------------------------------- | ---------------------------------------- |
| CLAUDE.md template        | `lib/templates/llm/_CLAUDE.md`                          | Single source -- used by init + upgrade  |
| Module registry           | `lib/templates/llm/_MODULES.md`                         | Highlander registry template             |
| Decision tree             | `lib/templates/llm/_DECISION_TREE.md`                   | Code placement guide template            |
| Archetypes reference      | `lib/templates/llm/_ARCHETYPES.md`                      | Archetype listing template               |
| Usage rules template      | `lib/templates/llm/_usage-rules.md`                     | Downstream project DO / NEVER scaffold   |
| Elixir archetypes         | `lib/templates/archetypes/elixir/`                      | 9 .ex.eex code templates                 |
| ST info template          | `lib/templates/prj/st/ST####/info.md`                   | Steel thread frontmatter                 |
| WP info template          | `lib/templates/prj/st/WP/info.md`                       | Work package frontmatter                 |
| WIP template              | `lib/templates/prj/_wip.md`                             | Work-in-progress file                    |
| Dependency graph template | `lib/templates/llm/_DEPENDENCY_GRAPH.md`                | Dependency rules template                |
| D11 Credo check           | `lib/templates/credo_checks/elixir/dependency_graph.ex` | Cross-app dependency enforcement         |
| Hook template             | `lib/templates/hooks/module_check_hook.json`            | Advisory write hook for Claude Code      |
| Credo check templates     | `lib/templates/credo_checks/elixir/`                    | 6 checks: R2, R6, R7, R11, R15, R16      |
| Credo config script       | `lib/scripts/configure_credo.exs`                       | Configures .credo.exs in target projects |

## Help Files

| Concern             | THE Module                   | Notes                               |
| ------------------- | ---------------------------- | ----------------------------------- |
| Claude help         | `lib/help/claude.help.md`    | Claude plugin help                  |
| Agents help         | `lib/help/agents.help.md`    | Agents plugin help                  |
| Plugin help         | `lib/help/plugin.help.md`    | Plugin discovery help               |
| WP help             | `lib/help/wp.help.md`        | Work package help                   |
| Fileindex           | `lib/help/fileindex.help.md` | Fileindex help                      |
| Audit help          | `lib/help/audit.help.md`     | Audit command help                  |
| Learn help          | `lib/help/learn.help.md`     | Learn command help                  |
| Modules help        | `lib/help/modules.help.md`   | Modules command help                |
| STZero help         | `lib/help/stzero.help.md`    | STZero command help                 |
| Ext help (v2.9.0)   | `lib/help/ext.help.md`       | `intent ext` dispatcher help (WP02) |
| Rules help (v2.9.0) | `lib/help/rules.help.md`     | `intent claude rules` help (WP02)   |

## Skills: TCA Suite

| Concern             | THE Module                                        | Notes                      |
| ------------------- | ------------------------------------------------- | -------------------------- |
| TCA provisioning    | `intent/plugins/claude/skills/in-tca-init/`       | SKILL.md + tca-init.sh     |
| TCA audit execution | `intent/plugins/claude/skills/in-tca-audit/`      | SKILL.md + tca-progress.sh |
| TCA synthesis       | `intent/plugins/claude/skills/in-tca-synthesize/` | SKILL.md only              |
| TCA remediation     | `intent/plugins/claude/skills/in-tca-remediate/`  | SKILL.md only              |
| TCA wrap-up         | `intent/plugins/claude/skills/in-tca-finish/`     | SKILL.md + tca-report.sh   |

## Skills: Superpowers Cherry-Picks

| Concern              | THE Module                                | Notes         |
| -------------------- | ----------------------------------------- | ------------- |
| Verification gate    | `intent/plugins/claude/skills/in-verify/` | SKILL.md only |
| Systematic debugging | `intent/plugins/claude/skills/in-debug/`  | SKILL.md only |
| Two-stage review     | `intent/plugins/claude/skills/in-review/` | SKILL.md only |

## Skills: Handoff

| Concern         | THE Module                                 | Notes                      |
| --------------- | ------------------------------------------ | -------------------------- |
| Session handoff | `intent/plugins/claude/skills/in-handoff/` | SKILL.md + handoff-prep.sh |

## Skills: Detrope

| Concern        | THE Module                                 | Notes                    |
| -------------- | ------------------------------------------ | ------------------------ |
| Trope analysis | `intent/plugins/claude/skills/in-detrope/` | SKILL.md + trope-catalog |

## Rule Library (v2.9.0 / ST0034)

Single source of truth for rule content. Skills reference by ID; Critic subagents enforce.

| Concern                   | THE Module                                                       | Notes                                                                    |
| ------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Schema reference          | `intent/plugins/claude/rules/_schema/rule-schema.md`             | Frontmatter fields, body sections, runnable-example contract             |
| ID scheme                 | `intent/plugins/claude/rules/_schema/id-scheme.md`               | `IN-<LANG>-<CAT>-<NNN>`, rename discipline                               |
| Attribution policy        | `intent/plugins/claude/rules/_schema/attribution-policy.md`      | Three tiers, `upstream_id:`, MIT notice structure                        |
| CI limitations            | `intent/plugins/claude/rules/_schema/CI-LIMITATIONS.md`          | Runnable (Elixir) vs textual (Rust/Swift/Lua)                            |
| Critic contract           | `intent/plugins/claude/rules/_schema/critic-contract.md`         | Input/output/rule-loading for `critic-*`                                 |
| Index generator           | `intent/plugins/claude/rules/_schema/index-generator.md`         | Shell+jq pipeline spec; `intent claude rules index`                      |
| Index template            | `intent/plugins/claude/rules/index.json.template`                | Target JSON shape (generated `index.json` lands alongside, WP02)         |
| Exemplar rule             | `intent/plugins/claude/rules/elixir/test/strong-assertions/`     | Copy-from template for new rules: RULE.md + good_test.exs + bad_test.exs |
| Attribution notice        | `intent/plugins/claude/rules/_attribution/elixir-test-critic.md` | MIT notice + pinned commit + derived-rule table                          |
| Agnostic rule pack (WP04) | `intent/plugins/claude/rules/agnostic/`                          | Highlander, PFIC, Thin Coordinator, No Silent Errors                     |
| Elixir rule pack (WP05)   | `intent/plugins/claude/rules/elixir/`                            | ≥15 rules across code, test, ash, phoenix, lv categories                 |
| Rust rule pack (WP06)     | `intent/plugins/claude/rules/rust/`                              | ≥5 rules; textual-only per CI-LIMITATIONS                                |
| Swift rule pack (WP06)    | `intent/plugins/claude/rules/swift/`                             | ≥5 rules; textual-only                                                   |
| Lua rule pack (WP06)      | `intent/plugins/claude/rules/lua/`                               | ≥5 rules; textual-only                                                   |
| Shell rule pack (WP12)    | `intent/plugins/claude/rules/shell/`                             | ≥5 rules (bash + zsh); dogfooded against `bin/intent*`                   |

## Subagents: Critic Family (v2.9.0 / WP07)

Thin orchestrators: read rules, apply Detection to target files, report findings. Modes: `code`, `test`.

| Concern             | THE Module                                                     | Notes                                            |
| ------------------- | -------------------------------------------------------------- | ------------------------------------------------ |
| Elixir critic       | `intent/plugins/claude/subagents/critic-elixir/`               | agent.md + metadata.json; code/test modes        |
| Rust critic         | `intent/plugins/claude/subagents/critic-rust/`                 | code/test modes                                  |
| Swift critic        | `intent/plugins/claude/subagents/critic-swift/`                | code/test modes                                  |
| Lua critic          | `intent/plugins/claude/subagents/critic-lua/`                  | code/test modes                                  |
| Shell critic (WP12) | `intent/plugins/claude/subagents/critic-shell/`                | bash + zsh detection via shebang; code mode only |
| Per-project config  | `intent/plugins/claude/rules/_schema/sample-intent-critic.yml` | Template for `.intent_critic.yml`                |

## Extension System (v2.9.0 / WP02 + WP08)

User-local extensions at `~/.intent/ext/<name>/`. Discovered alongside canon; shadow warnings emitted on collisions.

| Concern                  | THE Module                                               | Notes                                                                   |
| ------------------------ | -------------------------------------------------------- | ----------------------------------------------------------------------- |
| Ext dispatcher           | `bin/intent_ext`                                         | list, show, validate, new                                               |
| Manifest schema          | `intent/plugins/claude/ext-schema/extension.schema.json` | JSON Schema for `extension.json`                                        |
| Multi-root discovery     | `intent/plugins/claude/lib/claude_plugin_helpers.sh`     | `plugin_get_source_roots` + `plugin_resolve_source_file` callbacks      |
| Ext-seed template source | `lib/templates/ext-seeds/`                               | Seed directory root — copied by migrations                              |
| Worker-bee ext seed      | `lib/templates/ext-seeds/worker-bee/`                    | Reference extension; source for `migrate_v2_8_2_to_v2_9_0` seeding      |
| User-ext root (runtime)  | `~/.intent/ext/` (outside repo)                          | Created by migration; README stub included                              |
| Migration function       | `bin/intent_helpers::migrate_v2_8_2_to_v2_9_0`           | Creates ext root, seeds worker-bee, prunes elixir + worker-bee installs |
| Upgrade predicate        | `bin/intent_helpers::needs_v2_9_0_upgrade`               | Returns true when project's `.intent/config.json` is < 2.9.0            |
| Ext README emitter       | `bin/intent_helpers::generate_ext_readme`                | Writes ~/.intent/ext/README.md on first bootstrap                       |

## Tests

| Concern                         | THE Module                                  | Notes                                                 |
| ------------------------------- | ------------------------------------------- | ----------------------------------------------------- |
| Test helper/fixtures            | `tests/lib/test_helper.bash`                | Shared setup, assertions                              |
| Test runner                     | `tests/run_tests.sh`                        | Discovers and runs all BATS                           |
| Audit tests                     | `tests/unit/audit_commands.bats`            | BATS tests for audit command                          |
| Learn tests                     | `tests/unit/learn_commands.bats`            | BATS tests for learn command                          |
| Modules tests                   | `tests/unit/modules_commands.bats`          | BATS tests for modules command                        |
| STZero tests                    | `tests/unit/st_zero_commands.bats`          | BATS tests for st zero command                        |
| Ext commands (v2.9.0)           | `tests/unit/ext_commands.bats`              | `intent ext list/show/validate/new` (WP02)            |
| Ext discovery (v2.9.0)          | `tests/unit/ext_discovery.bats`             | Precedence, shadowing, env-var overrides (WP02)       |
| Ext migration (v2.9.0)          | `tests/unit/ext_migration.bats`             | Seed copy, idempotency, prune (WP08 + WP09)           |
| Ext seed validity (v2.9.0)      | `tests/unit/ext_seed_validity.bats`         | Worker-bee seed passes `intent ext validate` (WP08)   |
| Rule validator (v2.9.0)         | `tests/unit/rule_validator.bats`            | `intent claude rules validate` (WP02)                 |
| Rule index (v2.9.0)             | `tests/unit/rule_index.bats`                | `intent claude rules index` determinism (WP02)        |
| Rule pack — agnostic (v2.9.0)   | `tests/unit/rule_pack_agnostic.bats`        | Presence + `concretised_by:` ≥ 2 invariant (WP04)     |
| Rule pack — elixir (v2.9.0)     | `tests/unit/rule_pack_elixir.bats`          | Frontmatter, sections, attribution cross-check (WP05) |
| Rule pack — elixir runnable     | `tests/unit/rule_pack_elixir_runnable.bats` | Each `.exs` exits 0 under `elixir <path>` (WP05)      |
| Rule pack — rust (v2.9.0)       | `tests/unit/rule_pack_rust.bats`            | Textual-only; `rust` fence tag (WP06)                 |
| Rule pack — swift (v2.9.0)      | `tests/unit/rule_pack_swift.bats`           | Textual-only; `swift` fence tag (WP06)                |
| Rule pack — lua (v2.9.0)        | `tests/unit/rule_pack_lua.bats`             | Textual-only; `lua` fence tag (WP06)                  |
| Rule pack — shell (v2.9.0)      | `tests/unit/rule_pack_shell.bats`           | bash + zsh; `bash -n` / `zsh -n` syntax gate (WP12)   |
| Attribution compliance (v2.9.0) | `tests/unit/attribution_compliance.bats`    | `upstream_id:` slugs resolve at pinned commit (WP05)  |
| Rule-reference skills (v2.9.0)  | `tests/unit/rule_reference_skills.bats`     | `rules:` frontmatter resolves in each skill (WP03)    |
| Highlander audit (v2.9.0)       | `tests/unit/highlander_audit.bats`          | No duplicated rule prose across skills + rules (WP03) |
| Critic dispatch (v2.9.0)        | `tests/unit/critic_dispatch.bats`           | `in-review` stage-2 language detection (WP07)         |
| Critic report format (v2.9.0)   | `tests/unit/critic_report_format.bats`      | Stable report shape across four Critics (WP07)        |
| Critic config (v2.9.0)          | `tests/unit/critic_config.bats`             | `.intent_critic.yml` honoured (WP07)                  |
| Docs completeness (v2.9.0)      | `tests/unit/docs_completeness.bats`         | New-doc presence + cross-ref resolution (WP10)        |

## Docs (v2.9.0 / WP10)

| Concern            | THE Module                          | Notes                                                               |
| ------------------ | ----------------------------------- | ------------------------------------------------------------------- |
| Writing extensions | `intent/docs/writing-extensions.md` | Walkthrough with worker-bee as the worked example                   |
| Rules guide        | `intent/docs/rules.md`              | Schema, authoring, validation, attribution, skill-reference pattern |
| Critics guide      | `intent/docs/critics.md`            | Contract, modes, report format, `.intent_critic.yml` schema         |

## Fixture Roots (v2.9.0 / tests)

Test inputs checked into the repo. Generators do not create these at runtime.

| Concern            | THE Module                       | Notes                                                      |
| ------------------ | -------------------------------- | ---------------------------------------------------------- |
| Extension fixtures | `tests/fixtures/extensions/`     | valid-ext, malformed-ext, shadow-ext, traversal-ext (WP02) |
| Rule fixtures      | `tests/fixtures/rules/`          | valid + 5 error-path variants (WP02)                       |
| Critic fixtures    | `tests/fixtures/critics/<lang>/` | known-violating + known-clean + manifest.txt (WP07)        |
| Upgrade fixtures   | `tests/fixtures/upgrade/`        | Simulated stale-version project trees (WP09)               |
