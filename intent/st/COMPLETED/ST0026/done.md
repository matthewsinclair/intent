# Done - ST0026: Steel Thread Zero

## WP-01: Skill Rename (intent-_ to in-_)

- [x] Rename 6 skill source directories from `intent-*` to `in-*`
- [x] Update self-references inside SKILL.md files (autopsy script path, essentials install example)
- [x] Update `lib/help/claude.help.md` examples
- [x] Update project `CLAUDE.md` skill references
- [x] Update `lib/templates/llm/_CLAUDE.md` template
- [x] Update MEMORY.md skill names
- [x] Update any test file references
- [x] Uninstall old skills, install new via `intent claude skills sync`
- [x] Add rename migration logic to `plugin_sync()` in `claude_plugin_helpers.sh`
- [x] Add test for rename migration
- [x] Run full test suite

## WP-02: Workflow Skills

- [x] Create `in-start/SKILL.md` (session start procedure)
- [x] Create `in-plan/SKILL.md` (planning kickoff procedure)
- [x] Create `in-standards/SKILL.md` (coding standards primer)
- [x] Create `in-next/SKILL.md` (next step identification)
- [x] Create `in-finish/SKILL.md` (session finish procedure)
- [x] Install all 5 new skills

## WP-03: LLM Templates (D2, D3, D6)

- [x] Rewrite `lib/templates/llm/_CLAUDE.md` (enhanced template with full rule set)
- [x] Consolidate 3 CLAUDE.md heredocs into single template (Highlander fix)
- [x] Update `create_claude_md()` in `bin/intent_helpers` to use template
- [x] Update `bin/intent_init` to use template + create MODULES.md and DECISION_TREE.md
- [x] Create `lib/templates/llm/_MODULES.md`
- [x] Create `lib/templates/llm/_DECISION_TREE.md`
- [x] Create Intent's own `intent/llm/MODULES.md` (26 modules)
- [x] Create Intent's own `intent/llm/DECISION_TREE.md`
- [x] Update Intent's `CLAUDE.md` with Rules section
- [x] Test `intent init` creates all 3 files correctly
- [x] Verify variable substitution works

## WP-04: Memory Injection (D8)

- [x] Create `lib/templates/prime/operational-knowledge.md` (bundled knowledge)
- [x] Create `intent/plugins/claude/bin/intent_claude_prime` script
- [x] Implement memory path computation (project path -> ~/.claude/projects/HASH/memory/)
- [x] Implement source file reading with graceful missing-file handling
- [x] Implement MEMORY.md synthesis (condensation, not verbatim)
- [x] Implement `--refresh` flag (full overwrite)
- [x] Implement `--dry-run` flag (show output, don't write)
- [x] Implement `--from <project>` flag (import learnings)
- [x] Add to `intent/plugins/claude/plugin.json`
- [x] Add prime section to `lib/help/claude.help.md`
- [x] Wire dispatch in `bin/intent`
- [x] Handle PROJECT_ROOT detection for plugin commands
- [x] Verify output stays under 200 lines
- [x] Update plugin_commands.bats for new command count

## WP-05: Archetype Templates (D4)

- [x] Create `lib/templates/archetypes/elixir/` directory
- [x] Create `ash_domain.ex.eex`
- [x] Create `ash_resource.ex.eex`
- [x] Create `phoenix_controller.ex.eex`
- [x] Create `live_view.ex.eex`
- [x] Create `service.ex.eex`
- [x] Create `cli_command.ex.eex`
- [x] Create `genserver.ex.eex`
- [x] Create `oban_worker.ex.eex`
- [x] Create `test.ex.eex`
- [x] Create `lib/templates/llm/_ARCHETYPES.md` reference document

## WP-11: TN004 Tech Note (Total Codebase Audit)

- [x] Copy from laksa-web source
- [x] Replace project-specific references (laksa-web, Lamplight, ST0058, ST0098) with generic examples
- [x] Generalize stats and specific module names
- [x] Add Prerequisites section
- [x] Add cross-references to ST0026 deliverables
- [x] Clean up Appendix D, E, F artifacts
- [x] Place at `intent/docs/total-codebase-audit.md`
